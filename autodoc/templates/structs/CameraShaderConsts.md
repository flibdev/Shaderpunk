# CameraShaderConsts

- Stored in DXIL as 848 byte buffer (0x350) with name in metadata.
- Stored in SPIR-V as a nameless struct with types.
- Register 0 in most Vertex Shaders

## HLSL
```hlsl
cbuffer CameraShaderConsts : register(b0) {
    float4x4 mat_000;
    float4x4 mat_040;
    float4x4 mat_080;
    float4x4 mat_0C0;
    float4x4 mat_100;
    float4x4 mat_140;
    float4x4 mat_180;
    float4x4 mat_1C0;
    float4x4 mat_200;
    
    float4   vec_240;
    float4   vec_250;
    int4     vec_260;
    float4   vec_270;
    float4   vec_280;
    float4   vec_290;
    float4   vec_2A0;
    float4   vec_2B0;
    float4   vec_2C0;
    float4   vec_2D0;
    float4   vec_2E0;
    float4   vec_2F0;
    float4   vec_300;
    float4   vec_310;
    float4   vec_320;

    float2   unk_330;
    uint     unk_338;
    uint     unk_33C;
    uint2    unk_340;
    float    unk_348;
    float    unk_34C;
}
```
