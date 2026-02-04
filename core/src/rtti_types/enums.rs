// Names in this file are direct from RTTI
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use enum_try_from::impl_enum_try_from;
use serde::Serialize;
use strum_macros::{Display, EnumString};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Serialize)]
pub enum EnumError {
    #[error("Invalid Value")]
    InvalidValue,
}


impl_enum_try_from!(
    #[repr(u8)]
    #[derive(PartialEq, Eq, Debug, Copy, Clone, Serialize)]
    pub enum ETextureFilteringMin {
        Point          = 0,
        Linear         = 1,
        Anisotropic    = 2,
        AnisotropicLow = 3,
    },
    u8,
    EnumError,
    EnumError::InvalidValue
);


impl_enum_try_from!(
    #[repr(u8)]
    #[derive(PartialEq, Eq, Debug, Copy, Clone, Serialize)]
    pub enum ETextureFilteringMag {
        Point   = 0,
        Linear  = 1,
    },
    u8,
    EnumError,
    EnumError::InvalidValue
);


impl_enum_try_from!(
    #[repr(u8)]
    #[derive(PartialEq, Eq, Debug, Copy, Clone, Serialize)]
    pub enum ETextureFilteringMip {
        None    = 0,
        Point   = 1,
        Linear  = 2,
    },
    u8,
    EnumError,
    EnumError::InvalidValue
);


impl_enum_try_from!(
    #[repr(u8)]
    #[derive(PartialEq, Eq, Debug, Copy, Clone, Serialize)]
    pub enum ETextureAddressing {
        Wrap       = 0,
        Mirror     = 1,
        Clamp      = 2,
        MirrorOnce = 3,
        Border     = 4,
    },
    u8,
    EnumError,
    EnumError::InvalidValue
);


impl_enum_try_from!(
    #[repr(u8)]
    #[derive(PartialEq, Eq, Debug, Copy, Clone, Serialize)]
    pub enum ETextureComparisonFunction {
        None         = 0,
        Less         = 1,
        Equal        = 2,
        LessEqual    = 3,
        Greater      = 4,
        NotEqual     = 5,
        GreaterEqual = 6,
        Always       = 7,
    },
    u8,
    EnumError,
    EnumError::InvalidValue
);


impl_enum_try_from!(
    #[repr(u8)]
    #[derive(PartialEq, Eq, Debug, Copy, Clone, Serialize)]
    pub enum EMaterialModifier {
        HitProxy                = 0,
        WindData                = 1,
        ParticleParams          = 2,
        RemoteCamera            = 3,
        Mirror                  = 4,
        CustomStructBuffer      = 5,
        EffectParams            = 6,
        MotionMatrix            = 7,
        ColorAndTexture         = 8,
        MaterialParams          = 9,
        Eye                     = 10,
        Skin                    = 11,
        VehicleParams           = 12,
        Dismemberment           = 13,
        Garments                = 14,
        ShadowsDebugParams      = 15,
        MultilayeredDebug       = 16,
        ParallaxParams          = 17,
        HighlightsParams        = 18,
        DebugColoring           = 19,
        DrawBufferMask          = 20,
        AutoSpawnData           = 21,
        DestructionRegions      = 22,
        FloatTracks             = 23,
        AutoHideDistance        = 24,
        Rain                    = 25,
        PlanarReflections       = 26,
        WaterSim                = 27,
        TransparencyClipParams  = 28,
        FlatTireParams          = 29,
        SecondMultilayerParams  = 30,
        CrystalCoat             = 31,
        MAX                     = 32,
    },
    u8,
    EnumError,
    EnumError::InvalidValue
);


impl_enum_try_from!(
    #[repr(u8)]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Display, Serialize, EnumString, Hash)]
    pub enum EMaterialVertexFactory
    {
        Invalid                             = 0,
        Terrain                             = 1,
        MeshStatic                          = 2,
        MeshSkinned                         = 3,
        MeshExtSkinned                      = 4,
        GarmentMeshSkinned                  = 5,
        GarmentMeshExtSkinned               = 6,
        MeshSpeedTree                       = 7,
        ParticleBilboard                    = 8,
        ParticleParallel                    = 9,
        ParticleMotionBlur                  = 10,
        ParticleSphereAligned               = 11,
        ParticleVerticalFixed               = 12,
        ParticleTrail                       = 13,
        ParticleFacingTrail                 = 14,
        ParticleScreen                      = 15,
        ParticleBeam                        = 16,
        ParticleFacingBeam                  = 17,
        Decal                               = 18,
        Debug                               = 19,
        DrawBuffer                          = 20,
        Fullscreen                          = 21,
        MeshSkinnedVehicle                  = 22,
        MeshStaticVehicle                   = 23,
        MeshProcedural                      = 24,
        MeshDestructible                    = 25,
        MeshDestructibleSkinned             = 26,
        MeshSkinnedLightBlockers            = 27,
        MeshExtSkinnedLightBlockers         = 28,
        GarmentMeshSkinnedLightBlockers     = 29,
        GarmentMeshExtSkinnedLightBlockers  = 30,
        MeshSkinnedSingleBone               = 31,
        MeshProxy                           = 32,
        MeshWindowProxy                     = 33,
    },
    u8,
    EnumError,
    EnumError::InvalidValue
);

impl Default for EMaterialVertexFactory {
    fn default() -> Self {
        EMaterialVertexFactory::Invalid
    }
}


impl_enum_try_from!(
    #[repr(u8)]
    #[derive(PartialEq, Eq, Debug, Copy, Clone, Serialize)]
    pub enum EFeatureFlag
	{
		Default                     =  0,
		Shadows                     =  1,
		HitProxies                  =  2,
		Selection                   =  3,
		Wireframe                   =  4,
		VelocityBuffer              =  5,
		DebugDraw_BlendOff          =  6,
		DebugDraw_BlendOn           =  7,
		DynamicDecals               =  8,
		Highlights                  =  9,
        Overdraw                    = 10,
		IndirectInstancedGrass      = 11,
		DecalsOnStaticObjects       = 12,
		DecalsOnDynamicObjects      = 13,
		MaskParticlesInsideCar      = 14,
		MaskParticlesInsideInterior = 15,
		MaskTXAA                    = 16,
		DistantShadows              = 17,
		FloatTracks                 = 18,
		Rain                        = 19,
		NumLights                   = 20,
		DepthPrepass                = 21,
		DecalsOnAllObjects          = 22,
	},
    u8,
    EnumError,
    EnumError::InvalidValue
);
