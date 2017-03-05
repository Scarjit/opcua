// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct StatusResult {
    pub status_code: StatusCode,
    pub diagnostic_info: DiagnosticInfo,
}

impl MessageInfo for StatusResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::StatusResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<StatusResult> for StatusResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.status_code.byte_len();
        size += self.diagnostic_info.byte_len();
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.status_code.encode(stream)?;
        size += self.diagnostic_info.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let status_code = StatusCode::decode(stream)?;
        let diagnostic_info = DiagnosticInfo::decode(stream)?;
        Ok(StatusResult {
            status_code: status_code,
            diagnostic_info: diagnostic_info,
        })
    }
}
