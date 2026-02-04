# FrequentVertexConsts

- Stored in DXIL as 112 byte buffer (0x70) with name in metadata.
- Stored in SPIR-V as a nameless struct with types.
- Register 1 in most Vertex Shaders

## HLSL
```hlsl
cbuffer CameraShaderConsts : register(b1) {
    float4x4 mat_000; // ?Local 2 World
    float4   vec_040; // ?Scale
    float4   vec_050; // ?Offset
    float4   vec_060; // ?Skinning
}
```
