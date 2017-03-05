// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticChangeStructureDataType {
    pub affected: NodeId,
    pub affected_type: NodeId,
}

impl MessageInfo for SemanticChangeStructureDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::SemanticChangeStructureDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<SemanticChangeStructureDataType> for SemanticChangeStructureDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.affected.byte_len();
        size += self.affected_type.byte_len();
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.affected.encode(stream)?;
        size += self.affected_type.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let affected = NodeId::decode(stream)?;
        let affected_type = NodeId::decode(stream)?;
        Ok(SemanticChangeStructureDataType {
            affected: affected,
            affected_type: affected_type,
        })
    }
}
