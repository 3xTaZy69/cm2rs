use eframe::egui;
use cm2rs::*;
use cm2rs::sms::*;
use std::fs;

struct BlueprintViewer {
    save: Save,
    pan: egui::Vec2,
    zoom: f32,
    show_buildings: bool,
}

impl BlueprintViewer {
    fn new(save: Save) -> Self {
        Self {
            save,
            pan: egui::vec2(0.0, 0.0),
            zoom: 20.0,
            show_buildings: true,
        }
    }
}

// ASCII символ из кода Text-блока
fn text_symbol(symbol: u8) -> char {
    if symbol.is_ascii_graphic() || symbol == b' ' {
        symbol as char
    } else {
        '?'
    }
}

// Цвет блока
fn block_color(blocktype: &BlockType) -> egui::Color32 {
    match blocktype {
        BlockType::Nor          => egui::Color32::from_rgb(255,   9,   0),
        BlockType::And          => egui::Color32::from_rgb(  0, 121, 255),
        BlockType::Or           => egui::Color32::from_rgb(  0, 241,  29),
        BlockType::Xor          => egui::Color32::from_rgb(168,   0, 255),
        BlockType::Button       => egui::Color32::from_rgb(255, 127,   0),
        BlockType::FlipFlop     => egui::Color32::from_rgb( 30,  30,  30),
        BlockType::Led { .. }   => egui::Color32::from_rgb(175, 175, 175),
        BlockType::Sound { .. } => egui::Color32::from_rgb(175, 131,  76),
        BlockType::Conductor    => egui::Color32::from_rgb( 73, 185, 255),
        BlockType::Nand         => egui::Color32::from_rgb(  0,  42,  89),
        BlockType::Xnor         => egui::Color32::from_rgb(213,   0, 103),
        BlockType::Random { .. }=> egui::Color32::from_rgb( 84,  54,  35),
        BlockType::Text { .. }  => egui::Color32::from_rgb( 25,  71,  84),
        BlockType::Tile { .. }  => egui::Color32::from_rgb( 75,  75,  75),
        BlockType::Node         => egui::Color32::from_rgb(165, 177, 200),
        BlockType::Delay { .. } => egui::Color32::from_rgb( 98,  24, 148),
        BlockType::Antenna { .. }   => egui::Color32::from_rgb(235, 233, 183),
        BlockType::ConductorV2      => egui::Color32::from_rgb( 52, 132, 182),
        BlockType::Ledmixer { .. }  => egui::Color32::from_rgb(  0,   0,   0),
    }
}

// Цвет building (серая гамма с лёгким оттенком)
fn building_color(buildtype: &BuildingType) -> egui::Color32 {
    match buildtype {
        BuildingType::Assembler         => egui::Color32::from_rgb(200, 160,  60),
        BuildingType::Multiplier
        | BuildingType::Multiplier32Bit => egui::Color32::from_rgb(100, 200, 100),
        BuildingType::Divider
        | BuildingType::Divider32Bit    => egui::Color32::from_rgb(200, 100, 100),
        BuildingType::PixelDisplay
        | BuildingType::LargeRGBDisplay
        | BuildingType::RGBDisplay      => egui::Color32::from_rgb( 80, 180, 220),
        BuildingType::MassMemory
        | BuildingType::MassiveMemory
        | BuildingType::HugeMemory
        | BuildingType::DualMemory      => egui::Color32::from_rgb(180,  80, 220),
        BuildingType::TextConsole
        | BuildingType::Sign            => egui::Color32::from_rgb(220, 220, 120),
        BuildingType::IntegratedCircuit => egui::Color32::from_rgb( 60, 220, 180),
        BuildingType::NTransistor       => egui::Color32::from_rgb(220, 140,  60),
        BuildingType::PTransistor       => egui::Color32::from_rgb(140, 220,  60),
        BuildingType::Door              => egui::Color32::from_rgb(160, 120,  80),
        _                               => egui::Color32::from_rgb(140, 140, 140),
    }
}

impl eframe::App for BlueprintViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Панель управления
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Show:");
                ui.checkbox(&mut self.show_buildings, "Buildings");
                ui.separator();
                ui.label(format!("Zoom: {:.1}", self.zoom));
                if ui.button("Reset view").clicked() {
                    self.pan = egui::Vec2::ZERO;
                    self.zoom = 20.0;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

            let rect   = response.rect;
            let center = rect.center();

            // Перетаскивание
            if response.dragged() {
                self.pan += response.drag_delta();
            }

            // Масштаб с фиксацией под курсором
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                let mouse = response.hover_pos().unwrap_or(center);

                let world_before = egui::pos2(
                    (mouse.x - center.x - self.pan.x) / self.zoom,
                    (mouse.y - center.y - self.pan.y) / self.zoom,
                );

                let factor = if scroll > 0.0 { 1.05 } else { 0.85 };
                self.zoom = (self.zoom * factor).clamp(0.1, 500.0);

                let world_after = egui::pos2(
                    (mouse.x - center.x - self.pan.x) / self.zoom,
                    (mouse.y - center.y - self.pan.y) / self.zoom,
                );

                self.pan += egui::vec2(
                    (world_after.x - world_before.x) * self.zoom,
                    (world_after.y - world_before.y) * self.zoom,
                );
            }

            let to_screen = |wx: f32, wy: f32| -> egui::Pos2 {
                egui::pos2(
                    center.x + wx * self.zoom + self.pan.x,
                    center.y + wy * self.zoom + self.pan.y,
                )
            };

            let clip = ui.clip_rect();
            let mut hovered_block: Option<&Block>    = None;
            let mut hovered_building: Option<&Building> = None;

            // ── Блоки ──────────────────────────────────────────────
            for block in &self.save.blocks {
                let screen = to_screen(block.pos[0] as f32, block.pos[2] as f32);
                let size   = egui::vec2(self.zoom, self.zoom);
                let r      = egui::Rect::from_center_size(screen, size);

                if !clip.intersects(r) { continue; }

                let color = block_color(&block.blocktype);
                painter.rect_filled(r, 2.0, color);
                painter.rect_stroke(r, 0.0,
                    egui::Stroke::new(1.0, egui::Color32::BLACK),
                    egui::StrokeKind::Inside,
                );

                // Символ на Text-блоке
                if let BlockType::Text { symbol } = block.blocktype {
                    if self.zoom >= 10.0 {
                        let ch = text_symbol(symbol);
                        let font_size = (self.zoom * 0.6).clamp(8.0, 48.0);
                        painter.text(
                            screen,
                            egui::Align2::CENTER_CENTER,
                            ch.to_string(),
                            egui::FontId::monospace(font_size),
                            egui::Color32::WHITE,
                        );
                    }
                }

                if response.hover_pos().is_some_and(|p| r.contains(p)) {
                    hovered_block = Some(block);
                }
            }

            // ── Buildings ──────────────────────────────────────────
            if self.show_buildings {
                for building in &self.save.buildings {
                    let screen = to_screen(building.x as f32, building.z as f32);
                    // Buildings отображаем чуть крупнее — 1.5 клетки
                    let size = egui::vec2(self.zoom * 1.5, self.zoom * 1.5);
                    let r    = egui::Rect::from_center_size(screen, size);

                    if !clip.intersects(r) { continue; }

                    let color = building_color(&building.buildtype);
                    painter.rect_filled(r, 4.0, color);
                    painter.rect_stroke(r, 0.0,
                        egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 220, 80)),
                        egui::StrokeKind::Inside,
                    );

                    // Метка типа при достаточном масштабе
                    if self.zoom >= 14.0 {
                        let label = format!("{:?}", building.buildtype)
                            .chars().take(4).collect::<String>();
                        painter.text(
                            screen,
                            egui::Align2::CENTER_CENTER,
                            label,
                            egui::FontId::monospace((self.zoom * 0.35).clamp(6.0, 14.0)),
                            egui::Color32::BLACK,
                        );
                    }

                    if response.hover_pos().is_some_and(|p| r.contains(p)) {
                        hovered_building = Some(building);
                    }
                }
            }

            // ── Тултип ────────────────────────────────────────────
            let tooltip_pos = response.hover_pos().map(|p| p + egui::vec2(12.0, 12.0));

            if let (Some(block), Some(pos)) = (hovered_block, tooltip_pos) {
                painter.text(
                    pos,
                    egui::Align2::LEFT_TOP,
                    format!("Block #{}\n{:?}\npos: {:?}", block.id, block.blocktype, block.pos),
                    egui::FontId::default(),
                    egui::Color32::WHITE,
                );
            } else if let (Some(building), Some(pos)) = (hovered_building, tooltip_pos) {
                painter.text(
                    pos,
                    egui::Align2::LEFT_TOP,
                    format!("Building\n{:?}\npos: ({}, {}, {})",
                        building.buildtype, building.x, building.y, building.z),
                    egui::FontId::default(),
                    egui::Color32::from_rgb(255, 220, 80),
                );
            }
        });
    }
}

pub fn call_bview(file: String) -> eframe::Result<()> {
    
    let code = fs::read_to_string(&file).expect("cannot read file");
    let mut ev = execute_string(code);
    let save = ev.get_save("lower");
    println!("{}", save.as_string());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Blueprint Viewer",
        options,
        Box::new(|_| Ok(Box::new(BlueprintViewer::new(save)))),
    )
    
    
}