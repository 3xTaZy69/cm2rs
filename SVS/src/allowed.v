`define DEFINED 4'b0001

module allowed #(
    parameter N = 7 // parameters
) (
    input wire clk,
    input wire [N+1:0] a, b, // expressions inside bitwidth
    output reg [N:0] c
    // no inout
    // no systemverilog`s logic type
);

    wire a_n; // declaration of wire
    assign a_n = !a; // wires are only for assign
    // for optimization measures, wires can be defined as new owner of an exression
    // semantic analyzer must check on one drive for wires

    always @(posedge clk) begin
        `ifdef DEFINED
            c <= c + {12'b0, DEFINED}; // concatenation and use of defined variables
        `else
            c <= c + 16'b0;
        `endif
        // always_ff statements
    end

    reg is_b; // register declaration
    initial is_b = 0; // initial
    // 0 is default because no Z/X in the logic simulator

    always @* begin
        if (b) // if statement
            is_b = 1'h1; // hexadecimal
        else
            is_b = 1'b0; // binary

        // always_comb    
    end

    reg [N:0] state;
    initial begin
        state = 1;
    end

    // CASE showCASE
    always @(negedge clk or posedge clk) begin
        case (a)
            1'b0: begin
                state <= N-1;
            end
            1'b1: begin
                state <= N-2;
            end
            default: // default values go here
                state <= N;
        endcase
    end

    wire data_o;
    // modules in use
    secondary sec (
        .data (a[0]),
        .clk (clk),
        .data_o (data_o)
    );

    reg [N:0] statex;
    initial statex = N-1;

    always @* begin
        casex (b) // CASEx showCASEx
            4'b1??0:
                statex = 4'b0001;
            4'b0??1:
                statex = 4'b0010;
            default:
                statex = 4'b0000;
        endcase
    end

    wire [2*N:0] ternary_replicated = clk ? {{N{b[N]}}, b} : {(2*N+1){1'b0}}; // muxing on replication

endmodule // \n at the end of file required

module secondary (
    input wire data, clk,
    output reg data_o
);

    always @(posedge clk)
        data_o <= data;

endmodule