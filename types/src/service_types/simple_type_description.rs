// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2020 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    node_id::NodeId,
    qualified_name::QualifiedName,
};

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleTypeDescription {
    pub data_type_id: NodeId,
    pub name: QualifiedName,
    pub base_data_type: NodeId,
    pub built_in_type: u8,
}

impl BinaryEncoder<SimpleTypeDescription> for SimpleTypeDescription {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.data_type_id.byte_len();
        size += self.name.byte_len();
        size += self.base_data_type.byte_len();
        size += self.built_in_type.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.data_type_id.encode(stream)?;
        size += self.name.encode(stream)?;
        size += self.base_data_type.encode(stream)?;
        size += self.built_in_type.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let data_type_id = NodeId::decode(stream, decoding_limits)?;
        let name = QualifiedName::decode(stream, decoding_limits)?;
        let base_data_type = NodeId::decode(stream, decoding_limits)?;
        let built_in_type = u8::decode(stream, decoding_limits)?;
        Ok(SimpleTypeDescription {
            data_type_id,
            name,
            base_data_type,
            built_in_type,
        })
    }
}