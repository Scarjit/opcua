// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

/// Registers a server with the discovery server.
#[derive(Debug, Clone, PartialEq)]
pub struct RegisterServerRequest {
    pub request_header: RequestHeader,
    pub server: RegisteredServer,
}

impl MessageInfo for RegisterServerRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::RegisterServerRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<RegisterServerRequest> for RegisterServerRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.server.byte_len();
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.server.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let server = RegisteredServer::decode(stream)?;
        Ok(RegisterServerRequest {
            request_header: request_header,
            server: server,
        })
    }
}
