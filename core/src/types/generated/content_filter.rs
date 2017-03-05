// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ContentFilter {
    pub elements: Option<Vec<ContentFilterElement>>,
}

impl MessageInfo for ContentFilter {
    fn object_id(&self) -> ObjectId {
        ObjectId::ContentFilter_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ContentFilter> for ContentFilter {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.elements);
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.elements)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let elements: Option<Vec<ContentFilterElement>> = read_array(stream)?;
        Ok(ContentFilter {
            elements: elements,
        })
    }
}
