#![no_main]
use libfuzzer_sys::fuzz_target;

use bytes::BytesMut;
use tokio_util::codec::Decoder;

use opcua::core::prelude::*;

pub fn decode(buf: &mut BytesMut, codec: &mut TcpCodec) -> Result<Option<Message>, std::io::Error> {
    codec.decode(buf)
}

fuzz_target!(|data: &[u8]| {
    // With some random data, just try and deserialize it
    let decoding_options = DecodingOptions::default();
    let mut codec = TcpCodec::new(decoding_options);
    let mut buf = BytesMut::from(data);
    let _ = decode(&mut buf, &mut codec);
});
