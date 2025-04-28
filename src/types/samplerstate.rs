// Names in this file are direct from RTTI
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use enum_try_from::impl_enum_try_from;
use thiserror::Error;

use crate::decode::{Decode, DecodeExt};

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


pub struct SampleStateInfo {
    pub filteringMin: ETextureFilteringMin,
    pub filteringMag: ETextureFilteringMag,
    pub filteringMip: ETextureFilteringMip,
    pub addressU: ETextureAddressing,
    pub addressV: ETextureAddressing,
    pub addressW: ETextureAddressing,
    pub comparisonFunc: ETextureComparisonFunction,
    pub register: u8
}

impl Decode for SampleStateInfo {
    fn decode<I: std::io::Read>(input: &mut I) -> std::io::Result<Self> {
        let filteringMin: u8 = input.decode()?;
        let filteringMag: u8 = input.decode()?;
        let filteringMip: u8 = input.decode()?;
        let addressU: u8 = input.decode()?;
        let addressV: u8 = input.decode()?;
        let addressW: u8 = input.decode()?;
        let comparisonFunc: u8 = input.decode()?;
        let register: u8 = input.decode()?;

        Ok(SampleStateInfo {
            filteringMin: ETextureFilteringMin::try_from(filteringMin).unwrap(),
            filteringMag: ETextureFilteringMag::try_from(filteringMag).unwrap(),
            filteringMip: ETextureFilteringMip::try_from(filteringMip).unwrap(),
            addressU: ETextureAddressing::try_from(addressU).unwrap(),
            addressV: ETextureAddressing::try_from(addressV).unwrap(),
            addressW: ETextureAddressing::try_from(addressW).unwrap(),
            comparisonFunc: ETextureComparisonFunction::try_from(comparisonFunc).unwrap(),
            register
        })
    }
}
