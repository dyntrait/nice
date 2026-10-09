

use std::collections::HashMap;

use arrow::record_batch::RecordBatch;
use nice_model::data::{Data, InstrumentStatus};

use super::{
    DecodeDataFromRecordBatch, DecodeTypedFromRecordBatch, EncodingError,
    json::{JsonFieldSpec, impl_json_arrow},
};

const INSTRUMENT_STATUS_FIELDS: &[JsonFieldSpec] = &[
    JsonFieldSpec::utf8("instrument_id", false),
    JsonFieldSpec::utf8("action", false),
    JsonFieldSpec::u64("ts_event", false),
    JsonFieldSpec::u64("ts_init", false),
    JsonFieldSpec::utf8("reason", true),
    JsonFieldSpec::utf8("trading_event", true),
    JsonFieldSpec::boolean("is_trading", true),
    JsonFieldSpec::boolean("is_quoting", true),
    JsonFieldSpec::boolean("is_short_sell_restricted", true),
];

impl_json_arrow!(instrument InstrumentStatus, "InstrumentStatus", INSTRUMENT_STATUS_FIELDS);

impl DecodeDataFromRecordBatch for InstrumentStatus {
    fn decode_data_batch(
        metadata: &HashMap<String, String>,
        record_batch: RecordBatch,
    ) -> Result<Vec<Data>, EncodingError> {
        let items: Vec<Self> = Self::decode_typed_batch(metadata, record_batch)?;
        Ok(items.into_iter().map(Data::from).collect())
    }
}

#[cfg(test)]
mod tests {
    use nice_model::{enums::MarketStatusAction, identifiers::InstrumentId};
    use rstest::rstest;
    use ustr::Ustr;

    use super::*;
    use crate::arrow::{EncodeToRecordBatch, KEY_INSTRUMENT_ID};

    #[rstest]
    fn test_encode_decode_round_trip() {
        let instrument_id = InstrumentId::from("AAPL.XNAS");
        let metadata = HashMap::from([(KEY_INSTRUMENT_ID.to_string(), instrument_id.to_string())]);

        let status1 = InstrumentStatus::new(
            instrument_id,
            MarketStatusAction::Trading,
            1_000_000_000.into(),
            1_000_000_001.into(),
            Some(Ustr::from("Normal trading")),
            Some(Ustr::from("MARKET_OPEN")),
            Some(true),
            Some(true),
            Some(false),
        );

        let status2 = InstrumentStatus::new(
            instrument_id,
            MarketStatusAction::Halt,
            2_000_000_000.into(),
            2_000_000_001.into(),
            None,
            None,
            None,
            None,
            None,
        );

        let original = vec![status1, status2];
        let record_batch = InstrumentStatus::encode_batch(&metadata, &original).unwrap();
        let decoded: Vec<Data> =
            InstrumentStatus::decode_data_batch(&metadata, record_batch).unwrap();

        assert_eq!(decoded.len(), original.len());
        for (orig, dec) in original.iter().zip(decoded.iter()) {
            match dec {
                Data::InstrumentStatus(s) => assert_eq!(s, orig),
                other => panic!("expected Data::InstrumentStatus, was {other:?}"),
            }
        }
    }
}
