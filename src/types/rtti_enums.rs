// Names in this file are direct from RTTI
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use enum_try_from::impl_enum_try_from;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum EnumError {
    #[error("Invalid Value")]
    InvalidValue,
}


impl_enum_try_from!(
    #[repr(u8)]
    #[derive(PartialEq, Eq, Debug)]
    pub enum ETextureFilteringMin {
        TFMin_Point          = 0,
        TFMin_Linear         = 1,
        TFMin_Anisotropic    = 2,
        TFMin_AnisotropicLow = 3,
    },
    u8,
    EnumError,
    EnumError::InvalidValue
);


impl_enum_try_from!(
    #[repr(u8)]
    #[derive(PartialEq, Eq, Debug)]
    pub enum ETextureFilteringMag {
        TFMag_Point   = 0,
        TFMag_Linear  = 1,
    },
    u8,
    EnumError,
    EnumError::InvalidValue
);


impl_enum_try_from!(
    #[repr(u8)]
    #[derive(PartialEq, Eq, Debug)]
    pub enum ETextureFilteringMip {
        TFMip_None    = 0,
        TFMip_Point   = 1,
        TFMip_Linear  = 2,
    },
    u8,
    EnumError,
    EnumError::InvalidValue
);


impl_enum_try_from!(
    #[repr(u8)]
    #[derive(PartialEq, Eq, Debug)]
    pub enum ETextureAddressing {
        TA_Wrap       = 0,
        TA_Mirror     = 1,
        TA_Clamp      = 2,
        TA_MirrorOnce = 3,
        TA_Border     = 4,
    },
    u8,
    EnumError,
    EnumError::InvalidValue
);


impl_enum_try_from!(
    #[repr(u8)]
    #[derive(PartialEq, Eq, Debug)]
    pub enum ETextureComparisonFunction {
        TCF_None         = 0,
        TCF_Less         = 1,
        TCF_Equal        = 2,
        TCF_LessEqual    = 3,
        TCF_Greater      = 4,
        TCF_NotEqual     = 5,
        TCF_GreaterEqual = 6,
        TCF_Always       = 7,
    },
    u8,
    EnumError,
    EnumError::InvalidValue
);


impl_enum_try_from!(
    #[repr(u8)]
    #[derive(PartialEq, Eq, Debug)]
    pub enum EMaterialModifier {
        EMATMOD_HitProxy                = 0,
        EMATMOD_WindData                = 1,
        EMATMOD_ParticleParams          = 2,
        EMATMOD_RemoteCamera            = 3,
        EMATMOD_Mirror                  = 4,
        EMATMOD_CustomStructBuffer      = 5,
        EMATMOD_EffectParams            = 6,
        EMATMOD_MotionMatrix            = 7,
        EMATMOD_ColorAndTexture         = 8,
        EMATMOD_MaterialParams          = 9,
        EMATMOD_Eye                     = 10,
        EMATMOD_Skin                    = 11,
        EMATMOD_VehicleParams           = 12,
        EMATMOD_Dismemberment           = 13,
        EMATMOD_Garments                = 14,
        EMATMOD_ShadowsDebugParams      = 15,
        EMATMOD_MultilayeredDebug       = 16,
        EMATMOD_ParallaxParams          = 17,
        EMATMOD_HighlightsParams        = 18,
        EMATMOD_DebugColoring           = 19,
        EMATMOD_DrawBufferMask          = 20,
        EMATMOD_AutoSpawnData           = 21,
        EMATMOD_DestructionRegions      = 22,
        EMATMOD_FloatTracks             = 23,
        EMATMOD_AutoHideDistance        = 24,
        EMATMOD_Rain                    = 25,
        EMATMOD_PlanarReflections       = 26,
        EMATMOD_WaterSim                = 27,
        EMATMOD_TransparencyClipParams  = 28,
        EMATMOD_FlatTireParams          = 29,
        EMATMOD_SecondMultilayerParams  = 30,
        EMATMOD_CrystalCoat             = 31,
        EMATMOD_MAX                     = 32,
    },
    u8,
    EnumError,
    EnumError::InvalidValue
);
