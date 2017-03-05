// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EndpointConfiguration {
    pub operation_timeout: Int32,
    pub use_binary_encoding: Boolean,
    pub max_string_length: Int32,
    pub max_byte_string_length: Int32,
    pub max_array_length: Int32,
    pub max_message_size: Int32,
    pub max_buffer_size: Int32,
    pub channel_lifetime: Int32,
    pub security_token_lifetime: Int32,
}

impl MessageInfo for EndpointConfiguration {
    fn object_id(&self) -> ObjectId {
        ObjectId::EndpointConfiguration_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<EndpointConfiguration> for EndpointConfiguration {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.operation_timeout.byte_len();
        size += self.use_binary_encoding.byte_len();
        size += self.max_string_length.byte_len();
        size += self.max_byte_string_length.byte_len();
        size += self.max_array_length.byte_len();
        size += self.max_message_size.byte_len();
        size += self.max_buffer_size.byte_len();
        size += self.channel_lifetime.byte_len();
        size += self.security_token_lifetime.byte_len();
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.operation_timeout.encode(stream)?;
        size += self.use_binary_encoding.encode(stream)?;
        size += self.max_string_length.encode(stream)?;
        size += self.max_byte_string_length.encode(stream)?;
        size += self.max_array_length.encode(stream)?;
        size += self.max_message_size.encode(stream)?;
        size += self.max_buffer_size.encode(stream)?;
        size += self.channel_lifetime.encode(stream)?;
        size += self.security_token_lifetime.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let operation_timeout = Int32::decode(stream)?;
        let use_binary_encoding = Boolean::decode(stream)?;
        let max_string_length = Int32::decode(stream)?;
        let max_byte_string_length = Int32::decode(stream)?;
        let max_array_length = Int32::decode(stream)?;
        let max_message_size = Int32::decode(stream)?;
        let max_buffer_size = Int32::decode(stream)?;
        let channel_lifetime = Int32::decode(stream)?;
        let security_token_lifetime = Int32::decode(stream)?;
        Ok(EndpointConfiguration {
            operation_timeout: operation_timeout,
            use_binary_encoding: use_binary_encoding,
            max_string_length: max_string_length,
            max_byte_string_length: max_byte_string_length,
            max_array_length: max_array_length,
            max_message_size: max_message_size,
            max_buffer_size: max_buffer_size,
            channel_lifetime: channel_lifetime,
            security_token_lifetime: security_token_lifetime,
        })
    }
}
