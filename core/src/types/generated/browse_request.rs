// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

/// Browse the references for one or more nodes from the server address space.
#[derive(Debug, Clone, PartialEq)]
pub struct BrowseRequest {
    pub request_header: RequestHeader,
    pub view: ViewDescription,
    pub requested_max_references_per_node: UInt32,
    pub nodes_to_browse: Option<Vec<BrowseDescription>>,
}

impl MessageInfo for BrowseRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::BrowseRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<BrowseRequest> for BrowseRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.view.byte_len();
        size += self.requested_max_references_per_node.byte_len();
        size += byte_len_array(&self.nodes_to_browse);
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.view.encode(stream)?;
        size += self.requested_max_references_per_node.encode(stream)?;
        size += write_array(stream, &self.nodes_to_browse)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream)?;
        let view = ViewDescription::decode(stream)?;
        let requested_max_references_per_node = UInt32::decode(stream)?;
        let nodes_to_browse: Option<Vec<BrowseDescription>> = read_array(stream)?;
        Ok(BrowseRequest {
            request_header: request_header,
            view: view,
            requested_max_references_per_node: requested_max_references_per_node,
            nodes_to_browse: nodes_to_browse,
        })
    }
}
