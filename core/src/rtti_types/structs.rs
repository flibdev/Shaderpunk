// Names in this file are direct from RTTI
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::Serialize;

use crate::bundle::decode::{Decode, DecodeExt};
use crate::bundle::encode::{Encode, EncodeExt};
use crate::rtti_types::enums::*;

#[derive(Clone, Copy, Serialize)]
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

impl Encode for SampleStateInfo {
    fn encode<O: std::io::Write>(&self, output: &mut O) -> std::io::Result<()> {
        output.encode(&(self.filteringMin as u8))?;
        output.encode(&(self.filteringMag as u8))?;
        output.encode(&(self.filteringMip as u8))?;
        output.encode(&(self.addressU as u8))?;
        output.encode(&(self.addressV as u8))?;
        output.encode(&(self.addressW as u8))?;
        output.encode(&(self.comparisonFunc as u8))?;
        output.encode(&self.register)?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn SSI_decode() {
        let bytes = [ 0x02, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00 ];
        let mut reader = Cursor::new(bytes);
        let state: SampleStateInfo = reader.decode().unwrap();

        assert_eq!(state.filteringMin, ETextureFilteringMin::Anisotropic);
        assert_eq!(state.filteringMag, ETextureFilteringMag::Linear);
        assert_eq!(state.filteringMip, ETextureFilteringMip::Linear);
        assert_eq!(state.addressU, ETextureAddressing::Wrap);
        assert_eq!(state.addressV, ETextureAddressing::Wrap);
        assert_eq!(state.addressW, ETextureAddressing::Wrap);
        assert_eq!(state.comparisonFunc, ETextureComparisonFunction::None);
        assert_eq!(state.register, 0);
    }

    #[test]
    fn SSI_encode() {
        let buffer: Vec<u8> = Vec::new();
        let mut writer = Cursor::new(buffer);

        let state: SampleStateInfo = SampleStateInfo {
            filteringMin: ETextureFilteringMin::AnisotropicLow,
            filteringMag: ETextureFilteringMag::Linear,
            filteringMip: ETextureFilteringMip::None,
            addressU: ETextureAddressing::Border,
            addressV: ETextureAddressing::Clamp,
            addressW: ETextureAddressing::Mirror,
            comparisonFunc: ETextureComparisonFunction::GreaterEqual,
            register: 42
        };

        writer.encode(&state).unwrap();

        assert_eq!(writer.get_ref().len(), 8);
        assert_eq!(&writer.get_ref()[0..8], &[
            0x03, 0x01, 0x00,
            0x04, 0x02, 0x01,
            0x06, 0x2A
        ]);
    }

}