//! ProverResult
use ergotree_ir::serialization::sigma_byte_reader::SigmaByteRead;
use ergotree_ir::serialization::sigma_byte_writer::SigmaByteWrite;
use ergotree_ir::serialization::SerializationError;
use ergotree_ir::serialization::SigmaSerializable;

use super::ContextExtension;
use std::convert::TryFrom;
use std::io;

/// Serialized proof generated by ['Prover']
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum ProofBytes {
    /// Empty proof
    Empty,
    /// Non-empty proof
    Some(Vec<u8>),
}

impl Into<Vec<u8>> for ProofBytes {
    fn into(self) -> Vec<u8> {
        match self {
            ProofBytes::Empty => Vec::new(),
            ProofBytes::Some(bytes) => bytes,
        }
    }
}

impl From<Vec<u8>> for ProofBytes {
    fn from(bytes: Vec<u8>) -> Self {
        if bytes.is_empty() {
            ProofBytes::Empty
        } else {
            ProofBytes::Some(bytes)
        }
    }
}

// for JSON encoding in ergo-lib as Base16-encoded string
impl Into<String> for ProofBytes {
    fn into(self) -> String {
        match self {
            ProofBytes::Empty => "".to_string(),
            ProofBytes::Some(bytes) => base16::encode_lower(&bytes),
        }
    }
}

// for JSON encoding in ergo-lib
impl TryFrom<String> for ProofBytes {
    type Error = base16::DecodeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        base16::decode(&value).map(|bytes| bytes.into())
    }
}

impl SigmaSerializable for ProofBytes {
    fn sigma_serialize<W: SigmaByteWrite>(&self, w: &mut W) -> Result<(), io::Error> {
        match self {
            ProofBytes::Empty => w.put_u16(0)?,
            ProofBytes::Some(bytes) => {
                w.put_u16(bytes.len() as u16)?;
                w.write_all(&bytes)?;
            }
        }
        Ok(())
    }

    fn sigma_parse<R: SigmaByteRead>(r: &mut R) -> Result<Self, SerializationError> {
        let proof_len = r.get_u16()?;
        Ok(if proof_len == 0 {
            ProofBytes::Empty
        } else {
            let mut bytes = vec![0; proof_len as usize];
            r.read_exact(&mut bytes)?;
            ProofBytes::Some(bytes)
        })
    }
}

/// Proof of correctness of tx spending
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProverResult {
    /// proof that satisfies final sigma proposition
    pub proof: ProofBytes,
    /// user-defined variables to be put into context
    pub extension: ContextExtension,
}

#[cfg(feature = "arbitrary")]
pub mod arbitrary {
    use super::*;
    use proptest::{collection::vec, prelude::*};

    impl Arbitrary for ProofBytes {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(ProofBytes::Empty),
                (vec(any::<u8>(), 32..100))
                    .prop_map(ProofBytes::Some)
                    .boxed()
            ]
            .boxed()
        }
    }
}