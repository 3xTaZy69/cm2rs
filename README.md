### cm2rs - roblox game cm2 serializer and deserializer
## what can cm2rs do?

cm2rs can export savestrings in different formats:
  1. json
  2. original
  3. verilog

cm2rs can only import savestrings in original cm2 format.

allows using abstractions such as: adders, multiplexers, shifters, etc.

importing, creating, modifying and exporting game`s hardest mechanics: buildings such as memory, text console...

simulating savestrings(
  planning gpu simd calculations and buildings support later.
)

merging savestrings using tiny interpret language which can be learned in 10 mins.

example:

```
# always define lower - its main variable
let lower = "1,0,0,0,0,?";

# second savestring to merge
# +[0,1.5,0] is displacement AFTER it will be merged with lower
let +[0,1.5,0]another = "0,0,0,0,0,?";


# no semicol
merge lower another

```

output:
```
1,,,,,;0,,,1.5,,??
```

## why cm2rs?

You could write your own library for instancing blocks, connections, but why?
