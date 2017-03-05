// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct HistoryReadResult {
    pub status_code: StatusCode,
    pub continuation_point: ByteString,
    pub history_data: ExtensionObject,
}

impl MessageInfo for HistoryReadResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::HistoryReadResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<HistoryReadResult> for HistoryReadResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.status_code.byte_len();
        size += self.continuation_point.byte_len();
        size += self.history_data.byte_len();
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.status_code.encode(stream)?;
        size += self.continuation_point.encode(stream)?;
        size += self.history_data.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let status_code = StatusCode::decode(stream)?;
        let continuation_point = ByteString::decode(stream)?;
        let history_data = ExtensionObject::decode(stream)?;
        Ok(HistoryReadResult {
            status_code: status_code,
            continuation_point: continuation_point,
            history_data: history_data,
        })
    }
}
