// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct QueryFirstResponse {
    pub response_header: ResponseHeader,
    pub query_data_sets: Option<Vec<QueryDataSet>>,
    pub continuation_point: ByteString,
    pub parsing_results: Option<Vec<ParsingResult>>,
    pub diagnostic_infos: Option<Vec<DiagnosticInfo>>,
    pub filter_result: ContentFilterResult,
}

impl MessageInfo for QueryFirstResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::QueryFirstResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<QueryFirstResponse> for QueryFirstResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += byte_len_array(&self.query_data_sets);
        size += self.continuation_point.byte_len();
        size += byte_len_array(&self.parsing_results);
        size += byte_len_array(&self.diagnostic_infos);
        size += self.filter_result.byte_len();
        size
    }

    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += write_array(stream, &self.query_data_sets)?;
        size += self.continuation_point.encode(stream)?;
        size += write_array(stream, &self.parsing_results)?;
        size += write_array(stream, &self.diagnostic_infos)?;
        size += self.filter_result.encode(stream)?;
        Ok(size)
    }

    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream)?;
        let query_data_sets: Option<Vec<QueryDataSet>> = read_array(stream)?;
        let continuation_point = ByteString::decode(stream)?;
        let parsing_results: Option<Vec<ParsingResult>> = read_array(stream)?;
        let diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream)?;
        let filter_result = ContentFilterResult::decode(stream)?;
        Ok(QueryFirstResponse {
            response_header: response_header,
            query_data_sets: query_data_sets,
            continuation_point: continuation_point,
            parsing_results: parsing_results,
            diagnostic_infos: diagnostic_infos,
            filter_result: filter_result,
        })
    }
}
