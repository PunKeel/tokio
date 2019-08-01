use std::{fmt, io, usize};

use bytes::{BufMut, Bytes, BytesMut};

use crate::decoder::Decoder;
use crate::encoder::Encoder;

/// A simple `Codec` implementation that reads a byte
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct LengthFieldBasedCodec {
    max_frame_length: usize
}

impl LengthFieldBasedCodec {
    /// Creates a new `LengthFieldBasedCodec` for shipping around raw bytes.
    pub fn new(max_frame_length: usize) -> LengthFieldBasedCodec {
        LengthFieldBasedCodec {
            max_frame_length
        }
    }
}

impl Decoder for LengthFieldBasedCodec {
    type Item = BytesMut;
    type Error = LengthFieldBasedCodecError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<BytesMut>, LengthFieldBasedCodecError> {
        if !buf.is_empty() {
            let length = buf[0] as usize;
            if length > self.max_frame_length {
                Err(LengthFieldBasedCodecError::MaxFrameLengthExceeded)
            } else {
                let len = buf.len();
                if len >= length {
                    buf.advance(1);
                    return Ok(Some(buf.split_to(length)));
                } else {
                    Ok(None)
                }
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder for LengthFieldBasedCodec {
    type Item = Bytes;
    type Error = io::Error;

    fn encode(&mut self, data: Bytes, buf: &mut BytesMut) -> Result<(), io::Error> {
        buf.reserve(data.len());
        buf.put(data);
        Ok(())
    }
}


/// An error occurred while encoding or decoding a frame.
#[derive(Debug)]
pub enum LengthFieldBasedCodecError {
    /// The maximum frame length was exceeded.
    MaxFrameLengthExceeded,
    /// An IO error occured.
    Io(io::Error),
}

impl fmt::Display for LengthFieldBasedCodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LengthFieldBasedCodecError::MaxFrameLengthExceeded => write!(f, "max frame length exceeded"),
            LengthFieldBasedCodecError::Io(e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for LengthFieldBasedCodecError {
    fn from(e: io::Error) -> LengthFieldBasedCodecError {
        LengthFieldBasedCodecError::Io(e)
    }
}

impl std::error::Error for LengthFieldBasedCodecError {}
