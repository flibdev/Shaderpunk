// Names in this file are direct from RTTI
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


use crate::decode::{Decode, DecodeExt};
use crate::types::rtti_enums::*;

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
