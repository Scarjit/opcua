// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

/// Creates a secure channel with a server.
#[derive(Debug, Clone, PartialEq)]
pub struct OpenSecureChannelRequest {
    pub request_header: RequestHeader,
    pub client_protocol_version: UInt32,
    pub request_type: SecurityTokenRequestType,
    pub security_mode: MessageSecurityMode,
    pub client_nonce: ByteString,
    pub requested_lifetime: UInt32,
}

impl MessageInfo for OpenSecureChannelRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::OpenSecureChannelRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<OpenSecureChannelRequest> for OpenSecureChannelRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.client_protocol_version.byte_len();
        size += self.request_type.byte_len();
        size += self.security_mode.byte_len();
        size += self.client_nonce.byte_len();
        size += self.requested_lifetime.byte_len();
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.client_protocol_version.encode(stream)?;
        size += self.request_type.encode(stream)?;
        size += self.security_mode.encode(stream)?;
        size += self.client_nonce.encode(stream)?;
        size += self.requested_lifetime.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let client_protocol_version = UInt32::decode(stream)?;
        let request_type = SecurityTokenRequestType::decode(stream)?;
        let security_mode = MessageSecurityMode::decode(stream)?;
        let client_nonce = ByteString::decode(stream)?;
        let requested_lifetime = UInt32::decode(stream)?;
        Ok(OpenSecureChannelRequest {
            request_header: request_header,
            client_protocol_version: client_protocol_version,
            request_type: request_type,
            security_mode: security_mode,
            client_nonce: client_nonce,
            requested_lifetime: requested_lifetime,
        })
    }
}
