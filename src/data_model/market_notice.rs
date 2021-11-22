/// # Summary
///
/// ## MARKETNOTICEDATA
///  _MARKETNOTICEDATA shows market notices data provided to all participants (market) and specific participants (participant)._
///
/// * Data Set Name: Market Notice
/// * File Name: Marketnoticedata
/// * Data Version: 1
///
/// # Description
///  MARKETNOTICEDATA data is confidential to each participant, although some notices are sent to all participants. Source MARKETNOTICEDATA updates immediately available.
///
/// # Notes
///  * (Visibility) Data in this table is: Private &amp; Public
///
/// # Primary Key Columns
///
/// * NOTICEID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketNoticeMarketnoticedata1 {
    /// Notice Identifier
    pub noticeid: rust_decimal::Decimal,
    /// Effective Date of Market notice
    #[serde(with = "crate::mms_datetime_opt")]
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// Market Notice Type Identifier (Market - all participants. Participant - selected participants)
    pub typeid: Option<String>,
    /// Market Notice Type
    pub noticetype: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Detail of market notices.
    pub reason: Option<String>,
    /// External Reference for extra data pertaining to market notice
    pub externalreference: Option<String>,
}
impl crate::GetTable for MarketNoticeMarketnoticedata1 {
    type PrimaryKey = MarketNoticeMarketnoticedata1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MARKET_NOTICE".into(),
            table_name: Some("MARKETNOTICEDATA".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> MarketNoticeMarketnoticedata1PrimaryKey {
        MarketNoticeMarketnoticedata1PrimaryKey {
            noticeid: self.noticeid,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "market_notice_marketnoticedata_v1".to_string()
    }
}
impl crate::CompareWithRow for MarketNoticeMarketnoticedata1 {
    type Row = MarketNoticeMarketnoticedata1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.noticeid == row.noticeid
    }
}
impl crate::CompareWithPrimaryKey for MarketNoticeMarketnoticedata1 {
    type PrimaryKey = MarketNoticeMarketnoticedata1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.noticeid == key.noticeid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketNoticeMarketnoticedata1PrimaryKey {
    pub noticeid: rust_decimal::Decimal,
}
impl crate::CompareWithRow for MarketNoticeMarketnoticedata1PrimaryKey {
    type Row = MarketNoticeMarketnoticedata1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.noticeid == row.noticeid
    }
}
impl crate::CompareWithPrimaryKey for MarketNoticeMarketnoticedata1PrimaryKey {
    type PrimaryKey = MarketNoticeMarketnoticedata1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.noticeid == key.noticeid
    }
}
impl crate::PrimaryKey for MarketNoticeMarketnoticedata1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for MarketNoticeMarketnoticedata1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "noticeid",
                arrow2::datatypes::DataType::Decimal(10, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new("typeid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "noticetype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("reason", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "externalreference",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut noticeid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut typeid_array = Vec::new();
        let mut noticetype_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut reason_array = Vec::new();
        let mut externalreference_array = Vec::new();
        for (_, row) in partition {
            noticeid_array.push({
                let mut val = row.noticeid;
                val.rescale(0);
                val.mantissa()
            });
            effectivedate_array.push(row.effectivedate.map(|val| val.timestamp_millis()));
            typeid_array.push(row.typeid);
            noticetype_array.push(row.noticetype);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            reason_array.push(row.reason);
            externalreference_array.push(row.externalreference);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(noticeid_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(typeid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(noticetype_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(reason_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    externalreference_array,
                )),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MARKETNOTICETYPE
///  _MARKETNOTICETYPE sets out the different types of market notices (e.g. market systems)._
///
/// * Data Set Name: Market Notice
/// * File Name: Marketnoticetype
/// * Data Version: 1
///
/// # Description
///  MARKETNOTICETYPE data is public, so is available to all participants. Source MARKETNOTICETYPE updates whenever market notice types change.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * TYPEID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketNoticeMarketnoticetype1 {
    /// Identifier for market notice type
    pub typeid: String,
    /// Type description
    pub description: Option<String>,
    /// Not used
    pub raisedby: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MarketNoticeMarketnoticetype1 {
    type PrimaryKey = MarketNoticeMarketnoticetype1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MARKET_NOTICE".into(),
            table_name: Some("MARKETNOTICETYPE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> MarketNoticeMarketnoticetype1PrimaryKey {
        MarketNoticeMarketnoticetype1PrimaryKey {
            typeid: self.typeid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "market_notice_marketnoticetype_v1".to_string()
    }
}
impl crate::CompareWithRow for MarketNoticeMarketnoticetype1 {
    type Row = MarketNoticeMarketnoticetype1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.typeid == row.typeid
    }
}
impl crate::CompareWithPrimaryKey for MarketNoticeMarketnoticetype1 {
    type PrimaryKey = MarketNoticeMarketnoticetype1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.typeid == key.typeid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketNoticeMarketnoticetype1PrimaryKey {
    pub typeid: String,
}
impl crate::CompareWithRow for MarketNoticeMarketnoticetype1PrimaryKey {
    type Row = MarketNoticeMarketnoticetype1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.typeid == row.typeid
    }
}
impl crate::CompareWithPrimaryKey for MarketNoticeMarketnoticetype1PrimaryKey {
    type PrimaryKey = MarketNoticeMarketnoticetype1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.typeid == key.typeid
    }
}
impl crate::PrimaryKey for MarketNoticeMarketnoticetype1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for MarketNoticeMarketnoticetype1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("typeid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("raisedby", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut typeid_array = Vec::new();
        let mut description_array = Vec::new();
        let mut raisedby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            typeid_array.push(row.typeid);
            description_array.push(row.description);
            raisedby_array.push(row.raisedby);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(typeid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(raisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PARTICIPANTNOTICETRK
///  _PARTICIPANTNOTICETRK provides the cross-reference between participant market notices and participants._
///
/// * Data Set Name: Market Notice
/// * File Name: Participantnoticetrk
/// * Data Version: 1
///
/// # Description
///  PARTICIPANTNOTICETRK data is Confidential to the relevant participant. Source PARTICIPANTNOTICETRK updates immediately, whenever a participant notice is issued.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * NOTICEID
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketNoticeParticipantnoticetrk1 {
    /// Unique participant identifier
    pub participantid: String,
    /// Market notice identifier
    pub noticeid: rust_decimal::Decimal,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MarketNoticeParticipantnoticetrk1 {
    type PrimaryKey = MarketNoticeParticipantnoticetrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MARKET_NOTICE".into(),
            table_name: Some("PARTICIPANTNOTICETRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> MarketNoticeParticipantnoticetrk1PrimaryKey {
        MarketNoticeParticipantnoticetrk1PrimaryKey {
            noticeid: self.noticeid,
            participantid: self.participantid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "market_notice_participantnoticetrk_v1".to_string()
    }
}
impl crate::CompareWithRow for MarketNoticeParticipantnoticetrk1 {
    type Row = MarketNoticeParticipantnoticetrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.noticeid == row.noticeid && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for MarketNoticeParticipantnoticetrk1 {
    type PrimaryKey = MarketNoticeParticipantnoticetrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.noticeid == key.noticeid && self.participantid == key.participantid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketNoticeParticipantnoticetrk1PrimaryKey {
    pub noticeid: rust_decimal::Decimal,
    pub participantid: String,
}
impl crate::CompareWithRow for MarketNoticeParticipantnoticetrk1PrimaryKey {
    type Row = MarketNoticeParticipantnoticetrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.noticeid == row.noticeid && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for MarketNoticeParticipantnoticetrk1PrimaryKey {
    type PrimaryKey = MarketNoticeParticipantnoticetrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.noticeid == key.noticeid && self.participantid == key.participantid
    }
}
impl crate::PrimaryKey for MarketNoticeParticipantnoticetrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for MarketNoticeParticipantnoticetrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "noticeid",
                arrow2::datatypes::DataType::Decimal(10, 0),
                false,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut participantid_array = Vec::new();
        let mut noticeid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            participantid_array.push(row.participantid);
            noticeid_array.push({
                let mut val = row.noticeid;
                val.rescale(0);
                val.mantissa()
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(noticeid_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
