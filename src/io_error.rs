//! std::io::Result Encode/Decode implementation

use crate::{
    de::Decoder,
    enc::Encoder,
    error::{DecodeError, EncodeError},
    Decode, Encode,
};
use std::{
    io::{self, ErrorKind},
    string::{String, ToString},
};

impl Encode for io::Error {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.kind().encode(encoder)?;
        self.to_string().encode(encoder)
    }
}

impl<Context> Decode<Context> for io::Error {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let kind: ErrorKind = Decode::decode(decoder)?;
        let msg: String = Decode::decode(decoder)?;
        Ok(io::Error::new(kind, msg))
    }
}

impl Encode for ErrorKind {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let byte: u8 = match self {
            ErrorKind::NotFound => 0,
            ErrorKind::PermissionDenied => 1,
            ErrorKind::ConnectionRefused => 2,
            ErrorKind::ConnectionReset => 3,
            ErrorKind::HostUnreachable => 4,
            ErrorKind::NetworkUnreachable => 5,
            ErrorKind::ConnectionAborted => 6,
            ErrorKind::NotConnected => 7,
            ErrorKind::AddrInUse => 8,
            ErrorKind::AddrNotAvailable => 9,
            ErrorKind::NetworkDown => 10,
            ErrorKind::BrokenPipe => 11,
            ErrorKind::AlreadyExists => 12,
            ErrorKind::WouldBlock => 13,
            ErrorKind::NotADirectory => 14,
            ErrorKind::IsADirectory => 15,
            ErrorKind::DirectoryNotEmpty => 16,
            ErrorKind::ReadOnlyFilesystem => 17,
            ErrorKind::StaleNetworkFileHandle => 18,
            ErrorKind::InvalidInput => 19,
            ErrorKind::InvalidData => 20,
            ErrorKind::TimedOut => 21,
            ErrorKind::WriteZero => 22,
            ErrorKind::StorageFull => 23,
            ErrorKind::NotSeekable => 24,
            ErrorKind::QuotaExceeded => 25,
            ErrorKind::FileTooLarge => 26,
            ErrorKind::ResourceBusy => 27,
            ErrorKind::ExecutableFileBusy => 28,
            ErrorKind::Deadlock => 29,
            ErrorKind::CrossesDevices => 30,
            ErrorKind::TooManyLinks => 31,
            ErrorKind::InvalidFilename => 32,
            ErrorKind::ArgumentListTooLong => 33,
            ErrorKind::Interrupted => 34,
            ErrorKind::Unsupported => 35,
            ErrorKind::UnexpectedEof => 36,
            ErrorKind::OutOfMemory => 37,
            ErrorKind::Other => 254,
            _ => 255,
        };
        byte.encode(encoder)
    }
}

impl<Context> Decode<Context> for ErrorKind {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let byte: u8 = Decode::decode(decoder)?;
        Ok(match byte {
            0 => ErrorKind::NotFound,
            1 => ErrorKind::PermissionDenied,
            2 => ErrorKind::ConnectionRefused,
            3 => ErrorKind::ConnectionReset,
            4 => ErrorKind::HostUnreachable,
            5 => ErrorKind::NetworkUnreachable,
            6 => ErrorKind::ConnectionAborted,
            7 => ErrorKind::NotConnected,
            8 => ErrorKind::AddrInUse,
            9 => ErrorKind::AddrNotAvailable,
            10 => ErrorKind::NetworkDown,
            11 => ErrorKind::BrokenPipe,
            12 => ErrorKind::AlreadyExists,
            13 => ErrorKind::WouldBlock,
            14 => ErrorKind::NotADirectory,
            15 => ErrorKind::IsADirectory,
            16 => ErrorKind::DirectoryNotEmpty,
            17 => ErrorKind::ReadOnlyFilesystem,
            18 => ErrorKind::StaleNetworkFileHandle,
            19 => ErrorKind::InvalidInput,
            20 => ErrorKind::InvalidData,
            21 => ErrorKind::TimedOut,
            22 => ErrorKind::WriteZero,
            23 => ErrorKind::StorageFull,
            24 => ErrorKind::NotSeekable,
            25 => ErrorKind::QuotaExceeded,
            26 => ErrorKind::FileTooLarge,
            27 => ErrorKind::ResourceBusy,
            28 => ErrorKind::ExecutableFileBusy,
            29 => ErrorKind::Deadlock,
            30 => ErrorKind::CrossesDevices,
            31 => ErrorKind::TooManyLinks,
            32 => ErrorKind::InvalidFilename,
            33 => ErrorKind::ArgumentListTooLong,
            34 => ErrorKind::Interrupted,
            35 => ErrorKind::Unsupported,
            36 => ErrorKind::UnexpectedEof,
            37 => ErrorKind::OutOfMemory,
            254 => ErrorKind::Other,
            _ => ErrorKind::Other,
        })
    }
}

impl_borrow_decode!(io::Error);
impl_borrow_decode!(io::ErrorKind);

#[cfg(test)]
mod tests {
    use std::format;

    use super::*;
    use crate::{config, Decode, Encode};

    fn roundtrip<T: Encode + for<'a> Decode<()>>(val: &T) -> T {
        let encoded = crate::encode_to_vec(val, config::standard()).unwrap();
        let (decoded, _): (T, usize) =
            crate::decode_from_slice(&encoded, config::standard()).unwrap();
        decoded
    }

    #[test]
    fn test_error_roundtrip_preserves_kind_and_message() {
        let original = io::Error::new(ErrorKind::PermissionDenied, "access denied");
        let restored = roundtrip(&original);
        assert_eq!(restored.kind(), ErrorKind::PermissionDenied);
        assert_eq!(restored.to_string(), "access denied");
    }

    #[test]
    fn test_error_other_kind() {
        let original = io::Error::other("custom error");
        let restored = roundtrip(&original);
        assert_eq!(restored.kind(), ErrorKind::Other);
        assert_eq!(restored.to_string(), "custom error");
    }

    #[test]
    fn test_unknown_kind_falls_back_to_other() {
        let mut bytes = crate::encode_to_vec(200u8, config::standard()).unwrap();
        let msg_bytes = crate::encode_to_vec("future", config::standard()).unwrap();
        bytes.extend_from_slice(&msg_bytes);

        let (decoded, _): (io::Error, usize) =
            crate::decode_from_slice(&bytes, config::standard()).unwrap();
        assert_eq!(decoded.kind(), ErrorKind::Other);
        assert_eq!(decoded.to_string(), "future");
    }

    #[test]
    fn test_all_stable_kinds_roundtrip() {
        let kinds = [
            ErrorKind::NotFound,
            ErrorKind::PermissionDenied,
            ErrorKind::ConnectionRefused,
            ErrorKind::ConnectionReset,
            ErrorKind::HostUnreachable,
            ErrorKind::NetworkUnreachable,
            ErrorKind::ConnectionAborted,
            ErrorKind::NotConnected,
            ErrorKind::AddrInUse,
            ErrorKind::AddrNotAvailable,
            ErrorKind::NetworkDown,
            ErrorKind::BrokenPipe,
            ErrorKind::AlreadyExists,
            ErrorKind::WouldBlock,
            ErrorKind::NotADirectory,
            ErrorKind::IsADirectory,
            ErrorKind::DirectoryNotEmpty,
            ErrorKind::ReadOnlyFilesystem,
            ErrorKind::StaleNetworkFileHandle,
            ErrorKind::InvalidInput,
            ErrorKind::InvalidData,
            ErrorKind::TimedOut,
            ErrorKind::WriteZero,
            ErrorKind::StorageFull,
            ErrorKind::NotSeekable,
            ErrorKind::QuotaExceeded,
            ErrorKind::FileTooLarge,
            ErrorKind::ResourceBusy,
            ErrorKind::ExecutableFileBusy,
            ErrorKind::Deadlock,
            ErrorKind::CrossesDevices,
            ErrorKind::TooManyLinks,
            ErrorKind::InvalidFilename,
            ErrorKind::ArgumentListTooLong,
            ErrorKind::Interrupted,
            ErrorKind::Unsupported,
            ErrorKind::UnexpectedEof,
            ErrorKind::OutOfMemory,
        ];

        for kind in kinds {
            let original = io::Error::new(kind, format!("test {:?}", kind));
            let restored = roundtrip(&original);
            assert_eq!(restored.kind(), kind, "Failed for {:?}", kind);
        }
    }

    #[test]
    fn test_error_kind_standalone_roundtrip() {
        let kind = ErrorKind::TimedOut;
        let restored: ErrorKind = roundtrip(&kind);
        assert_eq!(restored, kind);
    }

    #[test]
    fn test_error_borrow_decode() {
        let original = io::Error::new(ErrorKind::TimedOut, "deadline exceeded");
        let encoded = crate::encode_to_vec(&original, config::standard()).unwrap();
        let (restored, _): (io::Error, usize) =
            crate::borrow_decode_from_slice(&encoded, config::standard()).unwrap();
        assert_eq!(restored.kind(), ErrorKind::TimedOut);
        assert_eq!(restored.to_string(), "deadline exceeded");
    }

    #[test]
    fn test_error_kind_borrow_decode() {
        let original = ErrorKind::ConnectionReset;
        let encoded = crate::encode_to_vec(&original, config::standard()).unwrap();
        let (restored, _): (ErrorKind, usize) =
            crate::borrow_decode_from_slice(&encoded, config::standard()).unwrap();
        assert_eq!(restored, original);
    }
}
