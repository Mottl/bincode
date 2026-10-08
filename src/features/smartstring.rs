//! Support for smartstring integration. Enable this with the `smartstring` feature.

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use smartstring::{Compact, LazyCompact, SmartString, SmartStringMode};
use std::borrow::Cow;

use crate::{
    de::Decoder,
    enc::Encoder,
    error::{DecodeError, EncodeError},
    impl_borrow_decode, Decode, Encode,
};

impl<T: SmartStringMode> Encode for SmartString<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_str().encode(encoder)
    }
}

impl<T: SmartStringMode, Context> Decode<Context> for SmartString<T> {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        // Decode as a Cow to borrow from the decoder's buffer
        let s = <Cow<'_, str> as Decode<Context>>::decode(decoder)?;
        Ok(Self::from(s.as_ref()))
    }
}

impl_borrow_decode!(SmartString<Compact>);
impl_borrow_decode!(SmartString<LazyCompact>);

#[cfg(test)]
mod test {
    use smartstring::{Compact, LazyCompact, SmartString};

    #[test]
    fn test_bincode_compact() {
        let mut buf: [u8; 64] = [0; 64];
        let short_str = "Hello world";
        let long_str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit";

        let config = crate::config::standard();
        let smartstring = SmartString::<Compact>::from(short_str);
        let len = crate::encode_into_slice(smartstring, &mut buf, config).unwrap();
        let smartstring: SmartString<Compact> =
            crate::decode_from_slice(&buf[..len], config).unwrap().0;
        assert_eq!(smartstring, short_str);

        let smartstring = SmartString::<Compact>::from(long_str);
        let len = crate::encode_into_slice(smartstring, &mut buf, config).unwrap();
        let smartstring: SmartString<Compact> =
            crate::decode_from_slice(&buf[..len], config).unwrap().0;
        assert_eq!(smartstring, long_str);
    }

    #[test]
    fn test_bincode_lazy_compact() {
        let mut buf: [u8; 64] = [0; 64];
        let short_str = "Hello world";
        let long_str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit";

        let config = crate::config::standard();
        let smartstring = SmartString::<LazyCompact>::from(short_str);
        let len = crate::encode_into_slice(smartstring, &mut buf, config).unwrap();
        let smartstring: SmartString<Compact> =
            crate::decode_from_slice(&buf[..len], config).unwrap().0;
        assert_eq!(smartstring, short_str);

        let smartstring = SmartString::<LazyCompact>::from(long_str);
        let len = crate::encode_into_slice(smartstring, &mut buf, config).unwrap();
        let smartstring: SmartString<Compact> =
            crate::decode_from_slice(&buf[..len], config).unwrap().0;
        assert_eq!(smartstring, long_str);
    }
}
