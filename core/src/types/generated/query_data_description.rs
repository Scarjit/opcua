// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct QueryDataDescription {
    pub relative_path: RelativePath,
    pub attribute_id: UInt32,
    pub index_range: UAString,
}

impl MessageInfo for QueryDataDescription {
    fn object_id(&self) -> ObjectId {
        ObjectId::QueryDataDescription_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<QueryDataDescription> for QueryDataDescription {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.relative_path.byte_len();
        size += self.attribute_id.byte_len();
        size += self.index_range.byte_len();
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.relative_path.encode(stream)?;
        size += self.attribute_id.encode(stream)?;
        size += self.index_range.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let relative_path = RelativePath::decode(stream)?;
        let attribute_id = UInt32::decode(stream)?;
        let index_range = UAString::decode(stream)?;
        Ok(QueryDataDescription {
            relative_path: relative_path,
            attribute_id: attribute_id,
            index_range: index_range,
        })
    }
}
