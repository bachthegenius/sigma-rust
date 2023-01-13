use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use crate::chain::json::context_extension::ContextExtensionSerde;
use ergotree_interpreter::sigma_protocol::prover::ContextExtension;
use serde::de;
use serde::de::MapAccess;
use serde::de::Visitor;
use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

use super::ProverResult;

impl Serialize for ProverResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("ProverResult", 2)?;
        s.serialize_field("proofBytes", &String::from(self.proof.clone()))?;
        s.serialize_field(
            "extension",
            &ContextExtensionSerde::from(self.extension.clone()),
        )?;
        s.end()
    }
}

impl FromStr for ProverResult {
    type Err = base16::DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let proof_bytes: Vec<u8> = base16::decode(s)?;
        Ok(ProverResult {
            proof: proof_bytes.into(),
            extension: ContextExtension::empty(),
        })
    }
}

// via https://serde.rs/string-or-struct.html
pub fn proof_as_string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = base16::DecodeError>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = base16::DecodeError>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            FromStr::from_str(value).map_err(|e| {
                de::Error::custom(format!(
                    "error: {}, while parsing proof bytes from string: {:?}",
                    e, value
                ))
            })
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use crate::chain::transaction::Input;

    #[test]
    fn parse_proof_explorer_api() {
        // https://github.com/ergoplatform/sigma-rust/issues/670
        let json_str = r#"{
      "id": "61a9e57e6635d02196fadc4ddf40a902c043c692bc6c7b452e02fa467e6e1fd7",
      "value": 6700000,
      "index": 0,
      "spendingProof": "736d882bbfa1767cef64e718eb74f76b96891fb8ad4e8fdd987ec4550b39d7cda0c285f8346e8f842acbae0b3d8e2b52d039cf1dacefc51c",
      "transactionId": "06b02c29a8c1a528c18bab4b6c92d447dc5ff0d99a591ddce2878631c555c97b",
      "outputTransactionId": "88291edf57563b34cb4e4cfae78efb1ad814ef6bff79969b64973a7ec89b59a7",
      "outputIndex": 2,
      "address": "3Wy3BaCjGDWE3bjjZkNo3aWaMz3cYrePMFhchcKovY9uG9vhpAuW"
        }"#;
        let input: Input = serde_json::from_str(json_str).unwrap();
        assert_eq!(input.spending_proof.proof.to_bytes().len(), 56);
    }
}
