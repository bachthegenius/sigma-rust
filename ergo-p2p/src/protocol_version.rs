//! Ergo P2P network version
use sigma_ser::{ScorexSerializable, ScorexSerializeResult};

/// P2P network protocol version
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct ProtocolVersion(pub u8, pub u8, pub u8);

impl ProtocolVersion {
    /// Create new ProtocolVersion instance
    pub const fn new(first_digit: u8, second_digit: u8, third_digit: u8) -> ProtocolVersion {
        ProtocolVersion {
            0: first_digit,
            1: second_digit,
            2: third_digit,
        }
    }

    /// Initial protocol version
    pub const INITIAL: Self = ProtocolVersion::new(0, 0, 1);
}

impl ScorexSerializable for ProtocolVersion {
    fn scorex_serialize<W: sigma_ser::vlq_encode::WriteSigmaVlqExt>(
        &self,
        w: &mut W,
    ) -> ScorexSerializeResult {
        w.put_u8(self.0)?;
        w.put_u8(self.1)?;
        w.put_u8(self.2)?;

        Ok(())
    }

    fn scorex_parse<R: sigma_ser::vlq_encode::ReadSigmaVlqExt>(
        r: &mut R,
    ) -> Result<Self, sigma_ser::ScorexParsingError> {
        Ok(ProtocolVersion::new(r.get_u8()?, r.get_u8()?, r.get_u8()?))
    }
}

#[allow(clippy::panic)]
#[cfg(test)]
mod tests {
    use super::*;
    use sigma_ser::scorex_serialize_roundtrip;

    #[test]
    fn ser_roundtrip() {
        let ver = ProtocolVersion::new(1, 14, 1);
        assert_eq![scorex_serialize_roundtrip(&ver), ver]
    }
}
