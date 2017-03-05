// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct HistoryModifiedData {
    pub data_values: Option<Vec<DataValue>>,
    pub modification_infos: Option<Vec<ModificationInfo>>,
}

impl BinaryEncoder<HistoryModifiedData> for HistoryModifiedData {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.data_values);
        size += byte_len_array(&self.modification_infos);
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.data_values)?;
        size += write_array(stream, &self.modification_infos)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let data_values: Option<Vec<DataValue>> = read_array(stream)?;
        let modification_infos: Option<Vec<ModificationInfo>> = read_array(stream)?;
        Ok(HistoryModifiedData {
            data_values: data_values,
            modification_infos: modification_infos,
        })
    }
}
