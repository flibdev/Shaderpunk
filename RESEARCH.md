# Research Outcomes

## shader_final.cache

## Format

Decoded except for one hash value (+0x0C in the footer block) that appears to be unused.


### Material 

### Hash
Composite hash composed of the material name and technique params
```rust
struct Technique {
    // From the EMaterialVertexFactory enum
    vertex_factory: u32;
    flag_dismembered: bool;
    flag_discarded: bool;    
    flag_preskinned: bool;
    tech_index: u32;
    tech_name: String;
    pass_index: u8;
};

tech_id: u32 = vertex_factory << 3;
// Flags are stored in the lower 3 bits
if flag_dismembered { tech_id |= 4; }
if flag_discarded   { tech_id |= 2; }
if flag_preskinned  { tech_id |= 1; }

// Each step uses the previous hash as a basis
tech: u32 = FNV1a32(tech_id);
tech = FNV1a32(tech_index, tech);
tech = FNV1a32(tech_name, tech);
tech = FNV1a32(pass_index, tech);

// Same method as the CName HashMapHash
name64: u64 = FNV1a64(material_name)
name: u32 = UPPER32(name64) ^ LOWER32(name64)

// Final hash is a simple composite of the two
hash: u64 = name << 32 | tech
```
