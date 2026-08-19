//! arrayvec::{ArrayVec, ArrayString} Encode/Decode implementation

use arrayvec::{ArrayString, ArrayVec};

use crate::{
    de::{decode_slice_len, read::Reader as _, BorrowDecoder, Decoder},
    enc::Encoder,
    error::{DecodeError, EncodeError},
    BorrowDecode, Decode, Encode,
};

impl<T: Encode, const CAP: usize> Encode for ArrayVec<T, CAP> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_slice().encode(encoder)
    }
}

impl<const CAP: usize> Encode for ArrayString<CAP> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_bytes().encode(encoder)
    }
}

impl<Context, T, const CAP: usize> Decode<Context> for ArrayVec<T, CAP>
where
    T: Decode<Context>,
{
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        const CAPACITY_EXCEEDED: &str = "ArrayVec: decoded length exceeds capacity";
        let len = decode_slice_len(decoder)?;
        if len > CAP {
            return Err(DecodeError::Other(CAPACITY_EXCEEDED));
        }

        if unty::type_equal::<T, u8>() {
            let mut vec = ArrayVec::<T, CAP>::new();
            {
                // SAFETY: We've already checked, that len <= CAP
                let bytes: &mut [u8] = unsafe {
                    vec.set_len(len);
                    core::slice::from_raw_parts_mut(vec.as_mut_ptr() as *mut u8, len)
                };
                decoder.reader().read(bytes)?;
            }
            return Ok(vec);
        }

        let mut vec = ArrayVec::new();
        for _ in 0..len {
            let item = <T as Decode<Context>>::decode(decoder)?;
            vec.push(item);
        }

        Ok(vec)
    }
}

impl<Context, const CAP: usize> Decode<Context> for ArrayString<CAP> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        const CAPACITY_EXCEEDED: &str = "ArrayString: decoded length exceeds capacity";
        let len = decode_slice_len(decoder)?;
        if len > CAP {
            return Err(DecodeError::Other(CAPACITY_EXCEEDED));
        }

        let mut s = ArrayString::<CAP>::new();
        {
            // SAFETY: We've already checked, that len <= CAP
            let bytes: &mut [u8] = unsafe {
                s.set_len(len);
                core::slice::from_raw_parts_mut(s.as_mut_ptr(), len)
            };
            decoder.reader().read(bytes)?;
        }
        Ok(s)
    }
}

impl<'de, Context, T, const CAP: usize> BorrowDecode<'de, Context> for ArrayVec<T, CAP>
where
    T: Decode<Context>,
{
    #[inline]
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        <Self as Decode<Context>>::decode(decoder)
    }
}

impl<'de, Context, const CAP: usize> BorrowDecode<'de, Context> for ArrayString<CAP> {
    #[inline]
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        <Self as Decode<Context>>::decode(decoder)
    }
}

#[cfg(test)]
mod tests {
    use crate::{config, decode_from_slice, encode_to_vec};
    use arrayvec::{ArrayString, ArrayVec};
    use std::format;

    #[test]
    fn test_arrayvec_encode_decode() {
        let mut vec = ArrayVec::<i32, 4>::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        let config = config::standard();
        let encoded = encode_to_vec(&vec, config).unwrap();

        let (decoded, _): (ArrayVec<i32, 4>, usize) = decode_from_slice(&encoded, config).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_arrayvec_u8_fast_path() {
        let mut vec = ArrayVec::<u8, 4>::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        let config = config::standard();
        let encoded = encode_to_vec(&vec, config).unwrap();

        let (decoded, _): (ArrayVec<u8, 4>, usize) = decode_from_slice(&encoded, config).unwrap();
        assert_eq!(vec, decoded);
    }

    #[test]
    fn test_arrayvec_capacity_exceeded() {
        let mut vec = ArrayVec::<i32, 10>::new();
        for i in 0..5 {
            vec.push(i);
        }

        let config = config::standard();
        let encoded = encode_to_vec(&vec, config).unwrap();

        let result: Result<(ArrayVec<i32, 2>, usize), _> = decode_from_slice(&encoded, config);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(format!("{:?}", err).contains("ArrayVec: decoded length exceeds capacity"));
    }

    #[test]
    fn test_arraystring_encode_decode() {
        let mut s = ArrayString::<10>::new();
        s.push_str("hello");

        let config = config::standard();
        let encoded = encode_to_vec(s, config).unwrap();

        let (decoded, _): (ArrayString<10>, usize) = decode_from_slice(&encoded, config).unwrap();
        assert_eq!(s, decoded);
    }

    #[test]
    fn test_arraystring_capacity_exceeded() {
        let mut s = ArrayString::<10>::new();
        s.push_str("hello");

        let config = config::standard();
        let encoded = encode_to_vec(s, config).unwrap();

        let result: Result<(ArrayString<2>, usize), _> = decode_from_slice(&encoded, config);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(format!("{:?}", err).contains("ArrayString: decoded length exceeds capacity"));
    }

    #[test]
    fn test_empty_collections() {
        let vec: ArrayVec<i32, 4> = ArrayVec::new();
        let config = config::standard();
        let encoded = encode_to_vec(&vec, config).unwrap();
        let (decoded, _): (ArrayVec<i32, 4>, usize) = decode_from_slice(&encoded, config).unwrap();
        assert_eq!(vec, decoded);

        let s: ArrayString<4> = ArrayString::new();
        let encoded = encode_to_vec(s, config).unwrap();
        let (decoded, _): (ArrayString<4>, usize) = decode_from_slice(&encoded, config).unwrap();
        assert_eq!(s, decoded);
    }

    #[test]
    fn test_arrayvec_complex_type() {
        let mut vec = ArrayVec::<(i32, i32), 2>::new();
        vec.push((1, 2));
        vec.push((3, 4));

        let config = config::standard();
        let encoded = encode_to_vec(&vec, config).unwrap();

        let (decoded, _): (ArrayVec<(i32, i32), 2>, usize) =
            decode_from_slice(&encoded, config).unwrap();
        assert_eq!(vec, decoded);
    }
}
