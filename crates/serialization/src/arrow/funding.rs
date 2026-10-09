

use std::collections::HashMap;

use arrow::record_batch::RecordBatch;
use nice_model::data::{Data, FundingRateUpdate};

use super::{
    DecodeDataFromRecordBatch, DecodeTypedFromRecordBatch, EncodingError,
    json::{JsonFieldSpec, impl_json_arrow},
};

const FUNDING_RATE_UPDATE_FIELDS: &[JsonFieldSpec] = &[
    JsonFieldSpec::utf8("instrument_id", false),
    JsonFieldSpec::utf8("rate", false),
    JsonFieldSpec::u64("interval", true),
    JsonFieldSpec::u64("next_funding_ns", true),
    JsonFieldSpec::u64("ts_event", false),
    JsonFieldSpec::u64("ts_init", false),
];

impl_json_arrow!(instrument FundingRateUpdate, "FundingRateUpdate", FUNDING_RATE_UPDATE_FIELDS);

impl DecodeDataFromRecordBatch for FundingRateUpdate {
    fn decode_data_batch(
        metadata: &HashMap<String, String>,
        record_batch: RecordBatch,
    ) -> Result<Vec<Data>, EncodingError> {
        let updates = Self::decode_typed_batch(metadata, record_batch)?;
        Ok(updates.into_iter().map(Data::from).collect())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use nice_core::UnixNanos;
    use nice_model::identifiers::InstrumentId;
    use rstest::rstest;
    use rust_decimal::Decimal;

    use super::*;
    use crate::arrow::EncodeToRecordBatch;

    #[rstest]
    fn test_funding_rate_update_round_trip_preserves_decimal_precision() {
        let update = FundingRateUpdate::new(
            InstrumentId::from("BTCUSDT-PERP.BINANCE"),
            Decimal::from_str("0.000123456789123456789").unwrap(),
            Some(480),
            Some(UnixNanos::from(9_000_000_000)),
            UnixNanos::from(1_000_000_000),
            UnixNanos::from(2_000_000_000),
        );
        let metadata = update.metadata();
        let batch = FundingRateUpdate::encode_batch(&metadata, &[update]).unwrap();
        let decoded =
            FundingRateUpdate::decode_typed_batch(batch.schema().metadata(), batch).unwrap();

        assert_eq!(decoded, vec![update]);
    }

    #[rstest]
    fn test_funding_rate_update_round_trip_null_optionals() {
        let update = FundingRateUpdate::new(
            InstrumentId::from("BTCUSDT-PERP.BINANCE"),
            Decimal::from_str("0.0001").unwrap(),
            None,
            None,
            UnixNanos::from(1_000_000_000),
            UnixNanos::from(2_000_000_000),
        );
        let metadata = update.metadata();
        let batch = FundingRateUpdate::encode_batch(&metadata, &[update]).unwrap();
        let decoded =
            FundingRateUpdate::decode_typed_batch(batch.schema().metadata(), batch).unwrap();

        assert_eq!(decoded, vec![update]);
        assert!(decoded[0].interval.is_none());
        assert!(decoded[0].next_funding_ns.is_none());
    }
}
