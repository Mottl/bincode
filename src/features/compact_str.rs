//! Support for compact_str integration. Enable this with the `compact_str` feature.

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use compact_str::CompactString;
use std::borrow::Cow;

use crate::{
    de::Decoder,
    enc::Encoder,
    error::{DecodeError, EncodeError},
    impl_borrow_decode, Decode, Encode,
};

impl Encode for CompactString {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_str().encode(encoder)
    }
}

impl<Context> Decode<Context> for CompactString {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        // Decode as a Cow to borrow from the decoder's buffer
        let s = <Cow<'_, str> as Decode<Context>>::decode(decoder)?;
        Ok(Self::from(s.as_ref()))
    }
}

impl_borrow_decode!(CompactString);

#[cfg(test)]
mod test {
    use compact_str::CompactString;

    #[test]
    fn test_bincode_compact() {
        let mut buf: [u8; 64] = [0; 64];
        let short_str = "Hello world";
        let long_str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit";

        let config = crate::config::standard();
        let compact = CompactString::new(short_str);
        let len = crate::encode_into_slice(compact, &mut buf, config).unwrap();
        let compact: CompactString = crate::decode_from_slice(&buf[..len], config).unwrap().0;
        assert_eq!(compact, short_str);

        let compact = CompactString::new(long_str);
        let len = crate::encode_into_slice(compact, &mut buf, config).unwrap();
        let compact: CompactString = crate::decode_from_slice(&buf[..len], config).unwrap().0;
        assert_eq!(compact, long_str);
    }

    #[test]
    fn test_bincode_lazy_compact() {
        let mut buf: [u8; 64] = [0; 64];
        let short_str = "Hello world";
        let long_str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit";

        let config = crate::config::standard();
        let smartstring = CompactString::new(short_str);
        let len = crate::encode_into_slice(smartstring, &mut buf, config).unwrap();
        let smartstring: CompactString = crate::decode_from_slice(&buf[..len], config).unwrap().0;
        assert_eq!(smartstring, short_str);

        let smartstring = CompactString::new(long_str);
        let len = crate::encode_into_slice(smartstring, &mut buf, config).unwrap();
        let smartstring: CompactString = crate::decode_from_slice(&buf[..len], config).unwrap().0;
        assert_eq!(smartstring, long_str);
    }
}
