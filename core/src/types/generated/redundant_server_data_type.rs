// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct RedundantServerDataType {
    pub server_id: UAString,
    pub service_level: Byte,
    pub server_state: ServerState,
}

impl MessageInfo for RedundantServerDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::RedundantServerDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<RedundantServerDataType> for RedundantServerDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.server_id.byte_len();
        size += self.service_level.byte_len();
        size += self.server_state.byte_len();
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.server_id.encode(stream)?;
        size += self.service_level.encode(stream)?;
        size += self.server_state.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let server_id = UAString::decode(stream)?;
        let service_level = Byte::decode(stream)?;
        let server_state = ServerState::decode(stream)?;
        Ok(RedundantServerDataType {
            server_id: server_id,
            service_level: service_level,
            server_state: server_state,
        })
    }
}
