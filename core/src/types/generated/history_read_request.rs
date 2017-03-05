// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct HistoryReadRequest {
    pub request_header: RequestHeader,
    pub history_read_details: ExtensionObject,
    pub timestamps_to_return: TimestampsToReturn,
    pub release_continuation_points: Boolean,
    pub nodes_to_read: Option<Vec<HistoryReadValueId>>,
}

impl MessageInfo for HistoryReadRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::HistoryReadRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<HistoryReadRequest> for HistoryReadRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.history_read_details.byte_len();
        size += self.timestamps_to_return.byte_len();
        size += self.release_continuation_points.byte_len();
        size += byte_len_array(&self.nodes_to_read);
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.history_read_details.encode(stream)?;
        size += self.timestamps_to_return.encode(stream)?;
        size += self.release_continuation_points.encode(stream)?;
        size += write_array(stream, &self.nodes_to_read)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let history_read_details = ExtensionObject::decode(stream)?;
        let timestamps_to_return = TimestampsToReturn::decode(stream)?;
        let release_continuation_points = Boolean::decode(stream)?;
        let nodes_to_read: Option<Vec<HistoryReadValueId>> = read_array(stream)?;
        Ok(HistoryReadRequest {
            request_header: request_header,
            history_read_details: history_read_details,
            timestamps_to_return: timestamps_to_return,
            release_continuation_points: release_continuation_points,
            nodes_to_read: nodes_to_read,
        })
    }
}
