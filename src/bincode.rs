//! Bincode encoding/decoding support

use std::marker::PhantomData;

use ::bincode::{
    de::{BorrowDecoder, Decoder},
    enc::Encoder,
    error::{DecodeError, EncodeError},
};

use crate::DecimalU64;

impl<S> ::bincode::Encode for DecimalU64<S> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.0.encode(encoder)
    }
}

impl<S, Context> ::bincode::Decode<Context> for DecimalU64<S> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let value = u64::decode(decoder)?;
        Ok(DecimalU64(value, PhantomData))
    }
}

impl<'de, S, Context> ::bincode::BorrowDecode<'de, Context> for DecimalU64<S> {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let value = u64::borrow_decode(decoder)?;
        Ok(DecimalU64(value, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use crate::U5;

    use super::*;
    use ::bincode::{Decode, Encode, config, decode_from_slice, encode_to_vec};

    type TestDecimal = DecimalU64<U5>;

    #[test]
    fn test_encode_and_decode() {
        let original = TestDecimal::new(123_456_789);
        let config = config::standard();

        // 1. Test Encoding
        let encoded = encode_to_vec(original, config).expect("Failed to encode DecimalU64");

        // 2. Test Standard Decoding
        let (decoded, bytes_read): (TestDecimal, usize) =
            decode_from_slice(&encoded, config).expect("Failed to decode DecimalU64");

        assert_eq!(original, decoded);
        assert_eq!(bytes_read, encoded.len());
    }

    #[test]
    fn test_borrow_decode() {
        let original = TestDecimal::new(987_654_321);
        let config = config::standard();

        let encoded = encode_to_vec(original, config).expect("Failed to encode DecimalU64");

        // Test BorrowDecode explicitly via decode_from_slice
        let (borrowed, _): (TestDecimal, _) =
            bincode::borrow_decode_from_slice(&encoded, config).expect("Failed to borrow-decode DecimalU64");

        assert_eq!(original, borrowed);
    }

    #[test]
    fn test_inside_derived_struct() {
        // Verifies that bincode's derive macros work on parent structs
        // without requiring MockScale to implement Encode/Decode.
        #[derive(Encode, Decode, Debug, PartialEq, Eq)]
        struct ParentStruct {
            pub id: u32,
            pub balance: TestDecimal,
        }

        let parent = ParentStruct {
            id: 42,
            balance: TestDecimal::new(5000),
        };
        let config = config::standard();

        let encoded = encode_to_vec(&parent, config).expect("Failed to encode parent struct");

        let (decoded, _): (ParentStruct, _) =
            decode_from_slice(&encoded, config).expect("Failed to decode parent struct");

        assert_eq!(parent, decoded);
    }

    #[test]
    fn test_wire_format_compatibility_with_u64() {
        // Verifies that DecimalU64 encodes to the exact same bytes as a raw u64
        let raw_val: u64 = 42_000_000;
        let decimal_val = TestDecimal::new(raw_val);
        let config = config::standard();

        let raw_encoded = encode_to_vec(raw_val, config).unwrap();
        let decimal_encoded = encode_to_vec(decimal_val, config).unwrap();

        assert_eq!(raw_encoded, decimal_encoded);
    }
}
