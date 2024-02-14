#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct ApApevent1;
pub struct ApApevent1Mapping([usize; 9]);
/// # Summary
///
/// ## APEVENT
///  _APEVENT is the driving data defining the existence and timeframes of an administered pricing event._
///
/// * Data Set Name: Ap
/// * File Name: Apevent
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * APEVENTID
#[derive(Debug, PartialEq, Eq)]
pub struct ApApevent1Row<'data> {
    /// Unique identifier for this administered pricing event
    pub apeventid: rust_decimal::Decimal,
    /// Date Time of the first Dispatch Interval to which the administered event applies
    pub effectivefrominterval: Option<chrono::NaiveDateTime>,
    /// Date Time of the final Dispatch Interval to which the administered event applies
    pub effectivetointerval: Option<chrono::NaiveDateTime>,
    /// Description of the driver for the Event
    pub reason: core::ops::Range<usize>,
    /// Authorising staff for start of AP event
    pub startauthorisedby: core::ops::Range<usize>,
    /// Date-Time start authorised
    pub startauthoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorising staff for end of AP event
    pub endauthorisedby: core::ops::Range<usize>,
    /// Date Time end authorised
    pub endauthoriseddate: Option<chrono::NaiveDateTime>,
    /// Date-Time the record was last modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ApApevent1Row<'data> {
    pub fn reason(&self) -> Option<&str> {
        if self.reason.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.reason.clone(),
                ),
            )
        }
    }
    pub fn startauthorisedby(&self) -> Option<&str> {
        if self.startauthorisedby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.startauthorisedby.clone(),
                ),
            )
        }
    }
    pub fn endauthorisedby(&self) -> Option<&str> {
        if self.endauthorisedby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.endauthorisedby.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for ApApevent1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "AP";
    const TABLE_NAME: &'static str = "APEVENT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ApApevent1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "APEVENTID",
        "EFFECTIVEFROMINTERVAL",
        "EFFECTIVETOINTERVAL",
        "REASON",
        "STARTAUTHORISEDBY",
        "STARTAUTHORISEDDATE",
        "ENDAUTHORISEDBY",
        "ENDAUTHORISEDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = ApApevent1Row<'row>;
    type FieldMapping = ApApevent1Mapping;
    type PrimaryKey = ApApevent1PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ApApevent1Row {
            apeventid: row
                .get_custom_parsed_at_idx(
                    "apeventid",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            effectivefrominterval: row
                .get_opt_custom_parsed_at_idx(
                    "effectivefrominterval",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            effectivetointerval: row
                .get_opt_custom_parsed_at_idx(
                    "effectivetointerval",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            reason: row.get_opt_range("reason", field_mapping.0[3])?,
            startauthorisedby: row
                .get_opt_range("startauthorisedby", field_mapping.0[4])?,
            startauthoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "startauthoriseddate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            endauthorisedby: row.get_opt_range("endauthorisedby", field_mapping.0[6])?,
            endauthoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "endauthoriseddate",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(ApApevent1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ApApevent1PrimaryKey {
        ApApevent1PrimaryKey {
            apeventid: row.apeventid,
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "ap_apevent_v1".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ApApevent1Row {
            apeventid: row.apeventid.clone(),
            effectivefrominterval: row.effectivefrominterval.clone(),
            effectivetointerval: row.effectivetointerval.clone(),
            reason: row.reason.clone(),
            startauthorisedby: row.startauthorisedby.clone(),
            startauthoriseddate: row.startauthoriseddate.clone(),
            endauthorisedby: row.endauthorisedby.clone(),
            endauthoriseddate: row.endauthoriseddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApApevent1PrimaryKey {
    pub apeventid: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ApApevent1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ApApevent1Row<'data> {
    type Row<'other> = ApApevent1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.apeventid == row.apeventid
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for ApApevent1Row<'data> {
    type PrimaryKey = ApApevent1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid
    }
}
impl<'data> mmsdm_core::CompareWithRow for ApApevent1PrimaryKey {
    type Row<'other> = ApApevent1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.apeventid == row.apeventid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ApApevent1PrimaryKey {
    type PrimaryKey = ApApevent1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ApApevent1 {
    type Builder = ApApevent1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "apeventid",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivefrominterval",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "effectivetointerval",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reason",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "startauthorisedby",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "startauthoriseddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "endauthorisedby",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "endauthoriseddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ApApevent1Builder {
            apeventid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            effectivefrominterval_array: arrow::array::builder::TimestampSecondBuilder::new(),
            effectivetointerval_array: arrow::array::builder::TimestampSecondBuilder::new(),
            reason_array: arrow::array::builder::StringBuilder::new(),
            startauthorisedby_array: arrow::array::builder::StringBuilder::new(),
            startauthoriseddate_array: arrow::array::builder::TimestampSecondBuilder::new(),
            endauthorisedby_array: arrow::array::builder::StringBuilder::new(),
            endauthoriseddate_array: arrow::array::builder::TimestampSecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampSecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .apeventid_array
            .append_value({
                let mut val = row.apeventid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .effectivefrominterval_array
            .append_option(row.effectivefrominterval.map(|val| val.timestamp()));
        builder
            .effectivetointerval_array
            .append_option(row.effectivetointerval.map(|val| val.timestamp()));
        builder.reason_array.append_option(row.reason());
        builder.startauthorisedby_array.append_option(row.startauthorisedby());
        builder
            .startauthoriseddate_array
            .append_option(row.startauthoriseddate.map(|val| val.timestamp()));
        builder.endauthorisedby_array.append_option(row.endauthorisedby());
        builder
            .endauthoriseddate_array
            .append_option(row.endauthoriseddate.map(|val| val.timestamp()));
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.apeventid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivefrominterval_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivetointerval_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reason_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startauthorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startauthoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.endauthorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.endauthoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ApApevent1Builder {
    apeventid_array: arrow::array::builder::Decimal128Builder,
    effectivefrominterval_array: arrow::array::builder::TimestampSecondBuilder,
    effectivetointerval_array: arrow::array::builder::TimestampSecondBuilder,
    reason_array: arrow::array::builder::StringBuilder,
    startauthorisedby_array: arrow::array::builder::StringBuilder,
    startauthoriseddate_array: arrow::array::builder::TimestampSecondBuilder,
    endauthorisedby_array: arrow::array::builder::StringBuilder,
    endauthoriseddate_array: arrow::array::builder::TimestampSecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampSecondBuilder,
}
pub struct ApApeventregion2;
pub struct ApApeventregion2Mapping([usize; 14]);
/// # Summary
///
/// ## APEVENTREGION
///  _APEVENTREGION is the Region detail for an administered pricing event defined through APEVENT._
///
/// * Data Set Name: Ap
/// * File Name: Apeventregion
/// * Data Version: 2
///
///
///
///
///
/// # Primary Key Columns
///
/// * APEVENTID
/// * REGIONID
#[derive(Debug, PartialEq, Eq)]
pub struct ApApeventregion2Row<'data> {
    /// Unique identifier for this administered pricing event
    pub apeventid: rust_decimal::Decimal,
    /// Date-Time of the first Dispatch Interval to which the administered event applies
    pub regionid: core::ops::Range<usize>,
    /// Date Time of the final Dispatch Interval to which the administered event applies
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// flag indicating if the apevent covers an energy AP
    pub energyapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a raise6sec AP
    pub raise6secapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a raise60sec AP
    pub raise60secapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a raise5min AP
    pub raise5minapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a raisereg AP
    pub raiseregapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a lower6sec AP
    pub lower6secapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a lower60sec AP<br>flag indicating if the apevent covers a lower5min AP<br>flag indicating if the apevent covers a lowerreg AP<br>flag indicating if the apevent covers a lower60sec AP
    pub lower60secapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a lower5min AP
    pub lower5minapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a lowerreg AP
    pub lowerregapflag: Option<rust_decimal::Decimal>,
    /// Flag indicating if the APEvent covers a Raise1Sec AP
    pub raise1secapflag: Option<rust_decimal::Decimal>,
    /// Flag indicating if the APEvent covers a Lower1Sec AP
    pub lower1secapflag: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ApApeventregion2Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for ApApeventregion2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "AP";
    const TABLE_NAME: &'static str = "APEVENTREGION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ApApeventregion2Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "APEVENTID",
        "REGIONID",
        "LASTCHANGED",
        "ENERGYAPFLAG",
        "RAISE6SECAPFLAG",
        "RAISE60SECAPFLAG",
        "RAISE5MINAPFLAG",
        "RAISEREGAPFLAG",
        "LOWER6SECAPFLAG",
        "LOWER60SECAPFLAG",
        "LOWER5MINAPFLAG",
        "LOWERREGAPFLAG",
        "RAISE1SECAPFLAG",
        "LOWER1SECAPFLAG",
    ];
    type Row<'row> = ApApeventregion2Row<'row>;
    type FieldMapping = ApApeventregion2Mapping;
    type PrimaryKey = ApApeventregion2PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ApApeventregion2Row {
            apeventid: row
                .get_custom_parsed_at_idx(
                    "apeventid",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[1])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            energyapflag: row
                .get_opt_custom_parsed_at_idx(
                    "energyapflag",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6secapflag: row
                .get_opt_custom_parsed_at_idx(
                    "raise6secapflag",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60secapflag: row
                .get_opt_custom_parsed_at_idx(
                    "raise60secapflag",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5minapflag: row
                .get_opt_custom_parsed_at_idx(
                    "raise5minapflag",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raiseregapflag: row
                .get_opt_custom_parsed_at_idx(
                    "raiseregapflag",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6secapflag: row
                .get_opt_custom_parsed_at_idx(
                    "lower6secapflag",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60secapflag: row
                .get_opt_custom_parsed_at_idx(
                    "lower60secapflag",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5minapflag: row
                .get_opt_custom_parsed_at_idx(
                    "lower5minapflag",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerregapflag: row
                .get_opt_custom_parsed_at_idx(
                    "lowerregapflag",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1secapflag: row
                .get_opt_custom_parsed_at_idx(
                    "raise1secapflag",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1secapflag: row
                .get_opt_custom_parsed_at_idx(
                    "lower1secapflag",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(ApApeventregion2Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ApApeventregion2PrimaryKey {
        ApApeventregion2PrimaryKey {
            apeventid: row.apeventid,
            regionid: row.regionid().to_string(),
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "ap_apeventregion_v2".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ApApeventregion2Row {
            apeventid: row.apeventid.clone(),
            regionid: row.regionid.clone(),
            lastchanged: row.lastchanged.clone(),
            energyapflag: row.energyapflag.clone(),
            raise6secapflag: row.raise6secapflag.clone(),
            raise60secapflag: row.raise60secapflag.clone(),
            raise5minapflag: row.raise5minapflag.clone(),
            raiseregapflag: row.raiseregapflag.clone(),
            lower6secapflag: row.lower6secapflag.clone(),
            lower60secapflag: row.lower60secapflag.clone(),
            lower5minapflag: row.lower5minapflag.clone(),
            lowerregapflag: row.lowerregapflag.clone(),
            raise1secapflag: row.raise1secapflag.clone(),
            lower1secapflag: row.lower1secapflag.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApApeventregion2PrimaryKey {
    pub apeventid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for ApApeventregion2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ApApeventregion2Row<'data> {
    type Row<'other> = ApApeventregion2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.apeventid == row.apeventid && self.regionid() == row.regionid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for ApApeventregion2Row<'data> {
    type PrimaryKey = ApApeventregion2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid && self.regionid() == key.regionid
    }
}
impl<'data> mmsdm_core::CompareWithRow for ApApeventregion2PrimaryKey {
    type Row<'other> = ApApeventregion2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.apeventid == row.apeventid && self.regionid == row.regionid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ApApeventregion2PrimaryKey {
    type PrimaryKey = ApApeventregion2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid && self.regionid == key.regionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ApApeventregion2 {
    type Builder = ApApeventregion2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "apeventid",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energyapflag",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6secapflag",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60secapflag",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5minapflag",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raiseregapflag",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower6secapflag",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60secapflag",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5minapflag",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerregapflag",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise1secapflag",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower1secapflag",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ApApeventregion2Builder {
            apeventid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampSecondBuilder::new(),
            energyapflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            raise6secapflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            raise60secapflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            raise5minapflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            raiseregapflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            lower6secapflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            lower60secapflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            lower5minapflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            lowerregapflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            raise1secapflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lower1secapflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .apeventid_array
            .append_value({
                let mut val = row.apeventid;
                val.rescale(0);
                val.mantissa()
            });
        builder.regionid_array.append_value(row.regionid());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp()));
        builder
            .energyapflag_array
            .append_option({
                row.energyapflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .raise6secapflag_array
            .append_option({
                row.raise6secapflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .raise60secapflag_array
            .append_option({
                row.raise60secapflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .raise5minapflag_array
            .append_option({
                row.raise5minapflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .raiseregapflag_array
            .append_option({
                row.raiseregapflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lower6secapflag_array
            .append_option({
                row.lower6secapflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lower60secapflag_array
            .append_option({
                row.lower60secapflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lower5minapflag_array
            .append_option({
                row.lower5minapflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lowerregapflag_array
            .append_option({
                row.lowerregapflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .raise1secapflag_array
            .append_option({
                row.raise1secapflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lower1secapflag_array
            .append_option({
                row.lower1secapflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.apeventid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.energyapflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6secapflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60secapflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5minapflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raiseregapflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6secapflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60secapflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5minapflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerregapflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise1secapflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower1secapflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ApApeventregion2Builder {
    apeventid_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampSecondBuilder,
    energyapflag_array: arrow::array::builder::Decimal128Builder,
    raise6secapflag_array: arrow::array::builder::Decimal128Builder,
    raise60secapflag_array: arrow::array::builder::Decimal128Builder,
    raise5minapflag_array: arrow::array::builder::Decimal128Builder,
    raiseregapflag_array: arrow::array::builder::Decimal128Builder,
    lower6secapflag_array: arrow::array::builder::Decimal128Builder,
    lower60secapflag_array: arrow::array::builder::Decimal128Builder,
    lower5minapflag_array: arrow::array::builder::Decimal128Builder,
    lowerregapflag_array: arrow::array::builder::Decimal128Builder,
    raise1secapflag_array: arrow::array::builder::Decimal128Builder,
    lower1secapflag_array: arrow::array::builder::Decimal128Builder,
}
pub struct ForceMajeureIrfmamount1;
pub struct ForceMajeureIrfmamount1Mapping([usize; 8]);
/// # Summary
///
/// ## IRFMAMOUNT
///  _IRFMAMOUNT sets out settlement amounts associated with Industrial Relations Forced Majeure events._
///
/// * Data Set Name: Force Majeure
/// * File Name: Irfmamount
/// * Data Version: 1
///
/// # Description
///  IRFMAMOUNTis public data. Source IRFMAMOUNT is obsolete; was updated with each settlement run as required.
///
///
///
/// # Primary Key Columns
///
/// * IRFMID
/// * PERIODID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct ForceMajeureIrfmamount1Row<'data> {
    /// Unique Industrial Relations Force Majeure event
    pub irfmid: core::ops::Range<usize>,
    /// Date of event
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// Version number of record of event
    pub versionno: rust_decimal::Decimal,
    /// Settlement period
    pub periodid: rust_decimal::Decimal,
    /// Total settlement amount in $
    pub amount: Option<rust_decimal::Decimal>,
    /// Person authorising amount
    pub authorisedby: core::ops::Range<usize>,
    /// Authorised date
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ForceMajeureIrfmamount1Row<'data> {
    pub fn irfmid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.irfmid.clone())
    }
    pub fn authorisedby(&self) -> Option<&str> {
        if self.authorisedby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.authorisedby.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for ForceMajeureIrfmamount1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "FORCE_MAJEURE";
    const TABLE_NAME: &'static str = "IRFMAMOUNT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ForceMajeureIrfmamount1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "IRFMID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "PERIODID",
        "AMOUNT",
        "AUTHORISEDBY",
        "AUTHORISEDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = ForceMajeureIrfmamount1Row<'row>;
    type FieldMapping = ForceMajeureIrfmamount1Mapping;
    type PrimaryKey = ForceMajeureIrfmamount1PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ForceMajeureIrfmamount1Row {
            irfmid: row.get_range("irfmid", field_mapping.0[0])?,
            effectivedate: row
                .get_opt_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            amount: row
                .get_opt_custom_parsed_at_idx(
                    "amount",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[5])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(ForceMajeureIrfmamount1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ForceMajeureIrfmamount1PrimaryKey {
        ForceMajeureIrfmamount1PrimaryKey {
            irfmid: row.irfmid().to_string(),
            periodid: row.periodid,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "force_majeure_irfmamount_v1".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ForceMajeureIrfmamount1Row {
            irfmid: row.irfmid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            periodid: row.periodid.clone(),
            amount: row.amount.clone(),
            authorisedby: row.authorisedby.clone(),
            authoriseddate: row.authoriseddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ForceMajeureIrfmamount1PrimaryKey {
    pub irfmid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ForceMajeureIrfmamount1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ForceMajeureIrfmamount1Row<'data> {
    type Row<'other> = ForceMajeureIrfmamount1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.irfmid() == row.irfmid() && self.periodid == row.periodid
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for ForceMajeureIrfmamount1Row<'data> {
    type PrimaryKey = ForceMajeureIrfmamount1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.irfmid() == key.irfmid && self.periodid == key.periodid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for ForceMajeureIrfmamount1PrimaryKey {
    type Row<'other> = ForceMajeureIrfmamount1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.irfmid == row.irfmid() && self.periodid == row.periodid
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ForceMajeureIrfmamount1PrimaryKey {
    type PrimaryKey = ForceMajeureIrfmamount1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.irfmid == key.irfmid && self.periodid == key.periodid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ForceMajeureIrfmamount1 {
    type Builder = ForceMajeureIrfmamount1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "irfmid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "amount",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authorisedby",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authoriseddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ForceMajeureIrfmamount1Builder {
            irfmid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampSecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampSecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampSecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.irfmid_array.append_value(row.irfmid());
        builder
            .effectivedate_array
            .append_option(row.effectivedate.map(|val| val.timestamp()));
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .amount_array
            .append_option({
                row.amount
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .authoriseddate_array
            .append_option(row.authoriseddate.map(|val| val.timestamp()));
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.irfmid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ForceMajeureIrfmamount1Builder {
    irfmid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampSecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    amount_array: arrow::array::builder::Decimal128Builder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampSecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampSecondBuilder,
}
pub struct ForceMajeureIrfmevents1;
pub struct ForceMajeureIrfmevents1Mapping([usize; 6]);
/// # Summary
///
/// ## IRFMEVENTS
///  _IRFMEVENTS sets out specific Industrial Relations Forced Majeure events._
///
/// * Data Set Name: Force Majeure
/// * File Name: Irfmevents
/// * Data Version: 1
///
/// # Description
///  IRFMEVENTS is public data. Source IRFMEVENTS updates with the occurrence of any such events.
///
///
///
/// # Primary Key Columns
///
/// * IRFMID
#[derive(Debug, PartialEq, Eq)]
pub struct ForceMajeureIrfmevents1Row<'data> {
    /// &nbsp;
    pub irfmid: core::ops::Range<usize>,
    /// &nbsp;
    pub startdate: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub startperiod: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub enddate: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub endperiod: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ForceMajeureIrfmevents1Row<'data> {
    pub fn irfmid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.irfmid.clone())
    }
}
impl mmsdm_core::GetTable for ForceMajeureIrfmevents1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "FORCE_MAJEURE";
    const TABLE_NAME: &'static str = "IRFMEVENTS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ForceMajeureIrfmevents1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "IRFMID",
        "STARTDATE",
        "STARTPERIOD",
        "ENDDATE",
        "ENDPERIOD",
        "LASTCHANGED",
    ];
    type Row<'row> = ForceMajeureIrfmevents1Row<'row>;
    type FieldMapping = ForceMajeureIrfmevents1Mapping;
    type PrimaryKey = ForceMajeureIrfmevents1PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ForceMajeureIrfmevents1Row {
            irfmid: row.get_range("irfmid", field_mapping.0[0])?,
            startdate: row
                .get_opt_custom_parsed_at_idx(
                    "startdate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            startperiod: row
                .get_opt_custom_parsed_at_idx(
                    "startperiod",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            enddate: row
                .get_opt_custom_parsed_at_idx(
                    "enddate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            endperiod: row
                .get_opt_custom_parsed_at_idx(
                    "endperiod",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(ForceMajeureIrfmevents1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ForceMajeureIrfmevents1PrimaryKey {
        ForceMajeureIrfmevents1PrimaryKey {
            irfmid: row.irfmid().to_string(),
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "force_majeure_irfmevents_v1".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ForceMajeureIrfmevents1Row {
            irfmid: row.irfmid.clone(),
            startdate: row.startdate.clone(),
            startperiod: row.startperiod.clone(),
            enddate: row.enddate.clone(),
            endperiod: row.endperiod.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ForceMajeureIrfmevents1PrimaryKey {
    pub irfmid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for ForceMajeureIrfmevents1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ForceMajeureIrfmevents1Row<'data> {
    type Row<'other> = ForceMajeureIrfmevents1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.irfmid() == row.irfmid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for ForceMajeureIrfmevents1Row<'data> {
    type PrimaryKey = ForceMajeureIrfmevents1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.irfmid() == key.irfmid
    }
}
impl<'data> mmsdm_core::CompareWithRow for ForceMajeureIrfmevents1PrimaryKey {
    type Row<'other> = ForceMajeureIrfmevents1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.irfmid == row.irfmid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ForceMajeureIrfmevents1PrimaryKey {
    type PrimaryKey = ForceMajeureIrfmevents1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.irfmid == key.irfmid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ForceMajeureIrfmevents1 {
    type Builder = ForceMajeureIrfmevents1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "irfmid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "startdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "startperiod",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "enddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "endperiod",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ForceMajeureIrfmevents1Builder {
            irfmid_array: arrow::array::builder::StringBuilder::new(),
            startdate_array: arrow::array::builder::TimestampSecondBuilder::new(),
            startperiod_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            enddate_array: arrow::array::builder::TimestampSecondBuilder::new(),
            endperiod_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lastchanged_array: arrow::array::builder::TimestampSecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.irfmid_array.append_value(row.irfmid());
        builder.startdate_array.append_option(row.startdate.map(|val| val.timestamp()));
        builder
            .startperiod_array
            .append_option({
                row.startperiod
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.enddate_array.append_option(row.enddate.map(|val| val.timestamp()));
        builder
            .endperiod_array
            .append_option({
                row.endperiod
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.irfmid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startperiod_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.endperiod_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ForceMajeureIrfmevents1Builder {
    irfmid_array: arrow::array::builder::StringBuilder,
    startdate_array: arrow::array::builder::TimestampSecondBuilder,
    startperiod_array: arrow::array::builder::Decimal128Builder,
    enddate_array: arrow::array::builder::TimestampSecondBuilder,
    endperiod_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampSecondBuilder,
}
pub struct ForceMajeureMarketSuspendRegimeSum1;
pub struct ForceMajeureMarketSuspendRegimeSum1Mapping([usize; 6]);
/// # Summary
///
/// ## MARKET_SUSPEND_REGIME_SUM
///  _Tracks the evolution of pricing regimes applied to the suspended region and from which Dispatch Interval_
///
/// * Data Set Name: Force Majeure
/// * File Name: Market Suspend Regime Sum
/// * Data Version: 1
///
/// # Description
///  MARKET_SUSPEND_REGIME_SUM is public data, so is available to all participants.
///
///
///
/// # Primary Key Columns
///
/// * REGIONID
/// * START_INTERVAL
/// * SUSPENSION_ID
#[derive(Debug, PartialEq, Eq)]
pub struct ForceMajeureMarketSuspendRegimeSum1Row<'data> {
    /// Unique identifier for this suspension event
    pub suspension_id: core::ops::Range<usize>,
    /// Region(s) covered by this evolution of the event
    pub regionid: core::ops::Range<usize>,
    /// First Dispatch interval from which this regime applies
    pub start_interval: chrono::NaiveDateTime,
    /// Last Dispatch interval for which this regime applies
    pub end_interval: Option<chrono::NaiveDateTime>,
    /// Pricing Regime applied
    pub pricing_regime: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ForceMajeureMarketSuspendRegimeSum1Row<'data> {
    pub fn suspension_id(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.suspension_id.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn pricing_regime(&self) -> Option<&str> {
        if self.pricing_regime.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.pricing_regime.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for ForceMajeureMarketSuspendRegimeSum1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "FORCE_MAJEURE";
    const TABLE_NAME: &'static str = "MARKET_SUSPEND_REGIME_SUM";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ForceMajeureMarketSuspendRegimeSum1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SUSPENSION_ID",
        "REGIONID",
        "START_INTERVAL",
        "END_INTERVAL",
        "PRICING_REGIME",
        "LASTCHANGED",
    ];
    type Row<'row> = ForceMajeureMarketSuspendRegimeSum1Row<'row>;
    type FieldMapping = ForceMajeureMarketSuspendRegimeSum1Mapping;
    type PrimaryKey = ForceMajeureMarketSuspendRegimeSum1PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ForceMajeureMarketSuspendRegimeSum1Row {
            suspension_id: row.get_range("suspension_id", field_mapping.0[0])?,
            regionid: row.get_range("regionid", field_mapping.0[1])?,
            start_interval: row
                .get_custom_parsed_at_idx(
                    "start_interval",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            end_interval: row
                .get_opt_custom_parsed_at_idx(
                    "end_interval",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            pricing_regime: row.get_opt_range("pricing_regime", field_mapping.0[4])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(ForceMajeureMarketSuspendRegimeSum1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ForceMajeureMarketSuspendRegimeSum1PrimaryKey {
        ForceMajeureMarketSuspendRegimeSum1PrimaryKey {
            regionid: row.regionid().to_string(),
            start_interval: row.start_interval,
            suspension_id: row.suspension_id().to_string(),
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "force_majeure_market_suspend_regime_sum_v1".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ForceMajeureMarketSuspendRegimeSum1Row {
            suspension_id: row.suspension_id.clone(),
            regionid: row.regionid.clone(),
            start_interval: row.start_interval.clone(),
            end_interval: row.end_interval.clone(),
            pricing_regime: row.pricing_regime.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ForceMajeureMarketSuspendRegimeSum1PrimaryKey {
    pub regionid: alloc::string::String,
    pub start_interval: chrono::NaiveDateTime,
    pub suspension_id: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for ForceMajeureMarketSuspendRegimeSum1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ForceMajeureMarketSuspendRegimeSum1Row<'data> {
    type Row<'other> = ForceMajeureMarketSuspendRegimeSum1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.regionid() == row.regionid() && self.start_interval == row.start_interval
            && self.suspension_id() == row.suspension_id()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ForceMajeureMarketSuspendRegimeSum1Row<'data> {
    type PrimaryKey = ForceMajeureMarketSuspendRegimeSum1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid() == key.regionid && self.start_interval == key.start_interval
            && self.suspension_id() == key.suspension_id
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ForceMajeureMarketSuspendRegimeSum1PrimaryKey {
    type Row<'other> = ForceMajeureMarketSuspendRegimeSum1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.regionid == row.regionid() && self.start_interval == row.start_interval
            && self.suspension_id == row.suspension_id()
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ForceMajeureMarketSuspendRegimeSum1PrimaryKey {
    type PrimaryKey = ForceMajeureMarketSuspendRegimeSum1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid && self.start_interval == key.start_interval
            && self.suspension_id == key.suspension_id
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ForceMajeureMarketSuspendRegimeSum1 {
    type Builder = ForceMajeureMarketSuspendRegimeSum1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "suspension_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "start_interval",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "end_interval",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pricing_regime",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ForceMajeureMarketSuspendRegimeSum1Builder {
            suspension_id_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            start_interval_array: arrow::array::builder::TimestampSecondBuilder::new(),
            end_interval_array: arrow::array::builder::TimestampSecondBuilder::new(),
            pricing_regime_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampSecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.suspension_id_array.append_value(row.suspension_id());
        builder.regionid_array.append_value(row.regionid());
        builder.start_interval_array.append_value(row.start_interval.timestamp());
        builder
            .end_interval_array
            .append_option(row.end_interval.map(|val| val.timestamp()));
        builder.pricing_regime_array.append_option(row.pricing_regime());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.suspension_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.start_interval_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.end_interval_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pricing_regime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ForceMajeureMarketSuspendRegimeSum1Builder {
    suspension_id_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    start_interval_array: arrow::array::builder::TimestampSecondBuilder,
    end_interval_array: arrow::array::builder::TimestampSecondBuilder,
    pricing_regime_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampSecondBuilder,
}
pub struct ForceMajeureMarketSuspendRegionSum1;
pub struct ForceMajeureMarketSuspendRegionSum1Mapping([usize; 6]);
/// # Summary
///
/// ## MARKET_SUSPEND_REGION_SUM
///  _Summary of Market Suspension timings_
///
/// * Data Set Name: Force Majeure
/// * File Name: Market Suspend Region Sum
/// * Data Version: 1
///
/// # Description
///  MARKET_SUSPEND is public data, so is available to all participants.
///
///
///
/// # Primary Key Columns
///
/// * REGIONID
/// * SUSPENSION_ID
#[derive(Debug, PartialEq, Eq)]
pub struct ForceMajeureMarketSuspendRegionSum1Row<'data> {
    /// Unique identifier for this suspension event
    pub suspension_id: core::ops::Range<usize>,
    /// Region(s) covered by the Suspension event
    pub regionid: core::ops::Range<usize>,
    /// Initial interval of the Suspension event
    pub initial_interval: Option<chrono::NaiveDateTime>,
    /// Last Dispatch interval for the Suspension event for this Region
    pub end_region_interval: Option<chrono::NaiveDateTime>,
    /// Last Dispatch interval for the Suspension event
    pub end_suspension_interval: Option<chrono::NaiveDateTime>,
    /// Last DateTime the Suspension was administered
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ForceMajeureMarketSuspendRegionSum1Row<'data> {
    pub fn suspension_id(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.suspension_id.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for ForceMajeureMarketSuspendRegionSum1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "FORCE_MAJEURE";
    const TABLE_NAME: &'static str = "MARKET_SUSPEND_REGION_SUM";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ForceMajeureMarketSuspendRegionSum1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SUSPENSION_ID",
        "REGIONID",
        "INITIAL_INTERVAL",
        "END_REGION_INTERVAL",
        "END_SUSPENSION_INTERVAL",
        "LASTCHANGED",
    ];
    type Row<'row> = ForceMajeureMarketSuspendRegionSum1Row<'row>;
    type FieldMapping = ForceMajeureMarketSuspendRegionSum1Mapping;
    type PrimaryKey = ForceMajeureMarketSuspendRegionSum1PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ForceMajeureMarketSuspendRegionSum1Row {
            suspension_id: row.get_range("suspension_id", field_mapping.0[0])?,
            regionid: row.get_range("regionid", field_mapping.0[1])?,
            initial_interval: row
                .get_opt_custom_parsed_at_idx(
                    "initial_interval",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            end_region_interval: row
                .get_opt_custom_parsed_at_idx(
                    "end_region_interval",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            end_suspension_interval: row
                .get_opt_custom_parsed_at_idx(
                    "end_suspension_interval",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(ForceMajeureMarketSuspendRegionSum1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ForceMajeureMarketSuspendRegionSum1PrimaryKey {
        ForceMajeureMarketSuspendRegionSum1PrimaryKey {
            regionid: row.regionid().to_string(),
            suspension_id: row.suspension_id().to_string(),
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "force_majeure_market_suspend_region_sum_v1".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ForceMajeureMarketSuspendRegionSum1Row {
            suspension_id: row.suspension_id.clone(),
            regionid: row.regionid.clone(),
            initial_interval: row.initial_interval.clone(),
            end_region_interval: row.end_region_interval.clone(),
            end_suspension_interval: row.end_suspension_interval.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ForceMajeureMarketSuspendRegionSum1PrimaryKey {
    pub regionid: alloc::string::String,
    pub suspension_id: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for ForceMajeureMarketSuspendRegionSum1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ForceMajeureMarketSuspendRegionSum1Row<'data> {
    type Row<'other> = ForceMajeureMarketSuspendRegionSum1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.regionid() == row.regionid() && self.suspension_id() == row.suspension_id()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ForceMajeureMarketSuspendRegionSum1Row<'data> {
    type PrimaryKey = ForceMajeureMarketSuspendRegionSum1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid() == key.regionid && self.suspension_id() == key.suspension_id
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ForceMajeureMarketSuspendRegionSum1PrimaryKey {
    type Row<'other> = ForceMajeureMarketSuspendRegionSum1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.regionid == row.regionid() && self.suspension_id == row.suspension_id()
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ForceMajeureMarketSuspendRegionSum1PrimaryKey {
    type PrimaryKey = ForceMajeureMarketSuspendRegionSum1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid && self.suspension_id == key.suspension_id
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ForceMajeureMarketSuspendRegionSum1 {
    type Builder = ForceMajeureMarketSuspendRegionSum1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "suspension_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "initial_interval",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "end_region_interval",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "end_suspension_interval",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ForceMajeureMarketSuspendRegionSum1Builder {
            suspension_id_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            initial_interval_array: arrow::array::builder::TimestampSecondBuilder::new(),
            end_region_interval_array: arrow::array::builder::TimestampSecondBuilder::new(),
            end_suspension_interval_array: arrow::array::builder::TimestampSecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampSecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.suspension_id_array.append_value(row.suspension_id());
        builder.regionid_array.append_value(row.regionid());
        builder
            .initial_interval_array
            .append_option(row.initial_interval.map(|val| val.timestamp()));
        builder
            .end_region_interval_array
            .append_option(row.end_region_interval.map(|val| val.timestamp()));
        builder
            .end_suspension_interval_array
            .append_option(row.end_suspension_interval.map(|val| val.timestamp()));
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.suspension_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.initial_interval_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.end_region_interval_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.end_suspension_interval_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ForceMajeureMarketSuspendRegionSum1Builder {
    suspension_id_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    initial_interval_array: arrow::array::builder::TimestampSecondBuilder,
    end_region_interval_array: arrow::array::builder::TimestampSecondBuilder,
    end_suspension_interval_array: arrow::array::builder::TimestampSecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampSecondBuilder,
}
pub struct ForceMajeureMarketSuspendSchedule2;
pub struct ForceMajeureMarketSuspendSchedule2Mapping([usize; 16]);
/// # Summary
///
/// ## MARKET_SUSPEND_SCHEDULE
///  _Trading prices that will apply in the event of a market suspension event updated weekly._
///
/// * Data Set Name: Force Majeure
/// * File Name: Market Suspend Schedule
/// * Data Version: 2
///
/// # Description
///  MARKET_SUSPEND_SCHEDULE is public data, so is available to all participants.
///
///
///
/// # Primary Key Columns
///
/// * DAY_TYPE
/// * EFFECTIVEDATE
/// * PERIODID
/// * REGIONID
#[derive(Debug, PartialEq, Eq)]
pub struct ForceMajeureMarketSuspendSchedule2Row<'data> {
    /// Calendar date from when this record set is effective
    pub effectivedate: chrono::NaiveDateTime,
    /// Distinguishes which record set to apply - at time of writing this was Business or Non-business day but may change in the future depending on outcome of consultation
    pub day_type: core::ops::Range<usize>,
    /// Region affected.
    pub regionid: core::ops::Range<usize>,
    /// 48 intervals for a day, midnight base (equates to 00:30 - 00:00)
    pub periodid: rust_decimal::Decimal,
    /// Energy Price applied for this period for this Day Type
    pub energy_rrp: Option<rust_decimal::Decimal>,
    /// Raise 6Sec contingency Price applied for this period for this Day Type
    pub r6_rrp: Option<rust_decimal::Decimal>,
    /// Raise 60Sec contingency Price applied for this period for this Day Type
    pub r60_rrp: Option<rust_decimal::Decimal>,
    /// Raise 5Min contingency Price applied for this period for this Day Type
    pub r5_rrp: Option<rust_decimal::Decimal>,
    /// Raise Regulation contingency Price applied for this period for this Day Type
    pub rreg_rrp: Option<rust_decimal::Decimal>,
    /// Lower 6Sec contingency Price applied for this period for this Day Type
    pub l6_rrp: Option<rust_decimal::Decimal>,
    /// Lower 60Sec contingency Price applied for this period for this Day Type
    pub l60_rrp: Option<rust_decimal::Decimal>,
    /// Lower 5Min contingency Price applied for this period for this Day Type
    pub l5_rrp: Option<rust_decimal::Decimal>,
    /// Lower Regulation Price applied for this period for this Day Type
    pub lreg_rrp: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Lower 1Sec contingency Price applied for this period for this Day Type
    pub l1_rrp: Option<rust_decimal::Decimal>,
    /// Raise 1Sec contingency Price applied for this period for this Day Type
    pub r1_rrp: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ForceMajeureMarketSuspendSchedule2Row<'data> {
    pub fn day_type(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.day_type.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for ForceMajeureMarketSuspendSchedule2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "FORCE_MAJEURE";
    const TABLE_NAME: &'static str = "MARKET_SUSPEND_SCHEDULE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ForceMajeureMarketSuspendSchedule2Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "DAY_TYPE",
        "REGIONID",
        "PERIODID",
        "ENERGY_RRP",
        "R6_RRP",
        "R60_RRP",
        "R5_RRP",
        "RREG_RRP",
        "L6_RRP",
        "L60_RRP",
        "L5_RRP",
        "LREG_RRP",
        "LASTCHANGED",
        "L1_RRP",
        "R1_RRP",
    ];
    type Row<'row> = ForceMajeureMarketSuspendSchedule2Row<'row>;
    type FieldMapping = ForceMajeureMarketSuspendSchedule2Mapping;
    type PrimaryKey = ForceMajeureMarketSuspendSchedule2PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ForceMajeureMarketSuspendSchedule2Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            day_type: row.get_range("day_type", field_mapping.0[1])?,
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energy_rrp: row
                .get_opt_custom_parsed_at_idx(
                    "energy_rrp",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            r6_rrp: row
                .get_opt_custom_parsed_at_idx(
                    "r6_rrp",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            r60_rrp: row
                .get_opt_custom_parsed_at_idx(
                    "r60_rrp",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            r5_rrp: row
                .get_opt_custom_parsed_at_idx(
                    "r5_rrp",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rreg_rrp: row
                .get_opt_custom_parsed_at_idx(
                    "rreg_rrp",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            l6_rrp: row
                .get_opt_custom_parsed_at_idx(
                    "l6_rrp",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            l60_rrp: row
                .get_opt_custom_parsed_at_idx(
                    "l60_rrp",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            l5_rrp: row
                .get_opt_custom_parsed_at_idx(
                    "l5_rrp",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lreg_rrp: row
                .get_opt_custom_parsed_at_idx(
                    "lreg_rrp",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[13],
                    mmsdm_core::mms_datetime::parse,
                )?,
            l1_rrp: row
                .get_opt_custom_parsed_at_idx(
                    "l1_rrp",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            r1_rrp: row
                .get_opt_custom_parsed_at_idx(
                    "r1_rrp",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(ForceMajeureMarketSuspendSchedule2Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let effectivedate = row
            .get_custom_parsed_at_idx(
                "effectivedate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(effectivedate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ForceMajeureMarketSuspendSchedule2PrimaryKey {
        ForceMajeureMarketSuspendSchedule2PrimaryKey {
            day_type: row.day_type().to_string(),
            effectivedate: row.effectivedate,
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.effectivedate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "force_majeure_market_suspend_schedule_v2_{}_{}", Self::partition_suffix(&
            row).year, Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ForceMajeureMarketSuspendSchedule2Row {
            effectivedate: row.effectivedate.clone(),
            day_type: row.day_type.clone(),
            regionid: row.regionid.clone(),
            periodid: row.periodid.clone(),
            energy_rrp: row.energy_rrp.clone(),
            r6_rrp: row.r6_rrp.clone(),
            r60_rrp: row.r60_rrp.clone(),
            r5_rrp: row.r5_rrp.clone(),
            rreg_rrp: row.rreg_rrp.clone(),
            l6_rrp: row.l6_rrp.clone(),
            l60_rrp: row.l60_rrp.clone(),
            l5_rrp: row.l5_rrp.clone(),
            lreg_rrp: row.lreg_rrp.clone(),
            lastchanged: row.lastchanged.clone(),
            l1_rrp: row.l1_rrp.clone(),
            r1_rrp: row.r1_rrp.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ForceMajeureMarketSuspendSchedule2PrimaryKey {
    pub day_type: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for ForceMajeureMarketSuspendSchedule2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ForceMajeureMarketSuspendSchedule2Row<'data> {
    type Row<'other> = ForceMajeureMarketSuspendSchedule2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.day_type() == row.day_type() && self.effectivedate == row.effectivedate
            && self.periodid == row.periodid && self.regionid() == row.regionid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ForceMajeureMarketSuspendSchedule2Row<'data> {
    type PrimaryKey = ForceMajeureMarketSuspendSchedule2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day_type() == key.day_type && self.effectivedate == key.effectivedate
            && self.periodid == key.periodid && self.regionid() == key.regionid
    }
}
impl<'data> mmsdm_core::CompareWithRow for ForceMajeureMarketSuspendSchedule2PrimaryKey {
    type Row<'other> = ForceMajeureMarketSuspendSchedule2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.day_type == row.day_type() && self.effectivedate == row.effectivedate
            && self.periodid == row.periodid && self.regionid == row.regionid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ForceMajeureMarketSuspendSchedule2PrimaryKey {
    type PrimaryKey = ForceMajeureMarketSuspendSchedule2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day_type == key.day_type && self.effectivedate == key.effectivedate
            && self.periodid == key.periodid && self.regionid == key.regionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ForceMajeureMarketSuspendSchedule2 {
    type Builder = ForceMajeureMarketSuspendSchedule2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "day_type",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "energy_rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "r6_rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "r60_rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "r5_rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rreg_rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "l6_rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "l60_rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "l5_rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lreg_rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "l1_rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "r1_rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ForceMajeureMarketSuspendSchedule2Builder {
            effectivedate_array: arrow::array::builder::TimestampSecondBuilder::new(),
            day_type_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            energy_rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            r6_rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            r60_rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            r5_rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rreg_rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            l6_rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            l60_rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            l5_rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lreg_rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampSecondBuilder::new(),
            l1_rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            r1_rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.effectivedate_array.append_value(row.effectivedate.timestamp());
        builder.day_type_array.append_value(row.day_type());
        builder.regionid_array.append_value(row.regionid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .energy_rrp_array
            .append_option({
                row.energy_rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .r6_rrp_array
            .append_option({
                row.r6_rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .r60_rrp_array
            .append_option({
                row.r60_rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .r5_rrp_array
            .append_option({
                row.r5_rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rreg_rrp_array
            .append_option({
                row.rreg_rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .l6_rrp_array
            .append_option({
                row.l6_rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .l60_rrp_array
            .append_option({
                row.l60_rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .l5_rrp_array
            .append_option({
                row.l5_rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lreg_rrp_array
            .append_option({
                row.lreg_rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp()));
        builder
            .l1_rrp_array
            .append_option({
                row.l1_rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .r1_rrp_array
            .append_option({
                row.r1_rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.day_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.energy_rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.r6_rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.r60_rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.r5_rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rreg_rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.l6_rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.l60_rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.l5_rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lreg_rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.l1_rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.r1_rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ForceMajeureMarketSuspendSchedule2Builder {
    effectivedate_array: arrow::array::builder::TimestampSecondBuilder,
    day_type_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    energy_rrp_array: arrow::array::builder::Decimal128Builder,
    r6_rrp_array: arrow::array::builder::Decimal128Builder,
    r60_rrp_array: arrow::array::builder::Decimal128Builder,
    r5_rrp_array: arrow::array::builder::Decimal128Builder,
    rreg_rrp_array: arrow::array::builder::Decimal128Builder,
    l6_rrp_array: arrow::array::builder::Decimal128Builder,
    l60_rrp_array: arrow::array::builder::Decimal128Builder,
    l5_rrp_array: arrow::array::builder::Decimal128Builder,
    lreg_rrp_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampSecondBuilder,
    l1_rrp_array: arrow::array::builder::Decimal128Builder,
    r1_rrp_array: arrow::array::builder::Decimal128Builder,
}
pub struct ForceMajeureMarketSuspendScheduleTrk1;
pub struct ForceMajeureMarketSuspendScheduleTrk1Mapping([usize; 6]);
/// # Summary
///
/// ## MARKET_SUSPEND_SCHEDULE_TRK
///  _Parent table for pricing regimes used in suspensions_
///
/// * Data Set Name: Force Majeure
/// * File Name: Market Suspend Schedule Trk
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
#[derive(Debug, PartialEq, Eq)]
pub struct ForceMajeureMarketSuspendScheduleTrk1Row<'data> {
    /// Calendar date from when this record set is effective
    pub effectivedate: chrono::NaiveDateTime,
    /// Start Date of the date range for the source data
    pub source_start_date: Option<chrono::NaiveDateTime>,
    /// End Date of the date range for the source data
    pub source_end_date: Option<chrono::NaiveDateTime>,
    /// Reason why this regime was applied
    pub comments: core::ops::Range<usize>,
    /// DateTime this record set was loaded
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ForceMajeureMarketSuspendScheduleTrk1Row<'data> {
    pub fn comments(&self) -> Option<&str> {
        if self.comments.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.comments.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for ForceMajeureMarketSuspendScheduleTrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "FORCE_MAJEURE";
    const TABLE_NAME: &'static str = "MARKET_SUSPEND_SCHEDULE_TRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ForceMajeureMarketSuspendScheduleTrk1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "SOURCE_START_DATE",
        "SOURCE_END_DATE",
        "COMMENTS",
        "AUTHORISEDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = ForceMajeureMarketSuspendScheduleTrk1Row<'row>;
    type FieldMapping = ForceMajeureMarketSuspendScheduleTrk1Mapping;
    type PrimaryKey = ForceMajeureMarketSuspendScheduleTrk1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ForceMajeureMarketSuspendScheduleTrk1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            source_start_date: row
                .get_opt_custom_parsed_at_idx(
                    "source_start_date",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            source_end_date: row
                .get_opt_custom_parsed_at_idx(
                    "source_end_date",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            comments: row.get_opt_range("comments", field_mapping.0[3])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(ForceMajeureMarketSuspendScheduleTrk1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let effectivedate = row
            .get_custom_parsed_at_idx(
                "effectivedate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(effectivedate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> ForceMajeureMarketSuspendScheduleTrk1PrimaryKey {
        ForceMajeureMarketSuspendScheduleTrk1PrimaryKey {
            effectivedate: row.effectivedate,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.effectivedate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "force_majeure_market_suspend_schedule_trk_v1_{}_{}",
            Self::partition_suffix(& row).year, Self::partition_suffix(& row).month
            .number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ForceMajeureMarketSuspendScheduleTrk1Row {
            effectivedate: row.effectivedate.clone(),
            source_start_date: row.source_start_date.clone(),
            source_end_date: row.source_end_date.clone(),
            comments: row.comments.clone(),
            authoriseddate: row.authoriseddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ForceMajeureMarketSuspendScheduleTrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for ForceMajeureMarketSuspendScheduleTrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for ForceMajeureMarketSuspendScheduleTrk1Row<'data> {
    type Row<'other> = ForceMajeureMarketSuspendScheduleTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for ForceMajeureMarketSuspendScheduleTrk1Row<'data> {
    type PrimaryKey = ForceMajeureMarketSuspendScheduleTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
    }
}
impl<'data> mmsdm_core::CompareWithRow
for ForceMajeureMarketSuspendScheduleTrk1PrimaryKey {
    type Row<'other> = ForceMajeureMarketSuspendScheduleTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for ForceMajeureMarketSuspendScheduleTrk1PrimaryKey {
    type PrimaryKey = ForceMajeureMarketSuspendScheduleTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ForceMajeureMarketSuspendScheduleTrk1 {
    type Builder = ForceMajeureMarketSuspendScheduleTrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "source_start_date",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "source_end_date",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "comments",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authoriseddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ForceMajeureMarketSuspendScheduleTrk1Builder {
            effectivedate_array: arrow::array::builder::TimestampSecondBuilder::new(),
            source_start_date_array: arrow::array::builder::TimestampSecondBuilder::new(),
            source_end_date_array: arrow::array::builder::TimestampSecondBuilder::new(),
            comments_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampSecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampSecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.effectivedate_array.append_value(row.effectivedate.timestamp());
        builder
            .source_start_date_array
            .append_option(row.source_start_date.map(|val| val.timestamp()));
        builder
            .source_end_date_array
            .append_option(row.source_end_date.map(|val| val.timestamp()));
        builder.comments_array.append_option(row.comments());
        builder
            .authoriseddate_array
            .append_option(row.authoriseddate.map(|val| val.timestamp()));
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.source_start_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.source_end_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.comments_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ForceMajeureMarketSuspendScheduleTrk1Builder {
    effectivedate_array: arrow::array::builder::TimestampSecondBuilder,
    source_start_date_array: arrow::array::builder::TimestampSecondBuilder,
    source_end_date_array: arrow::array::builder::TimestampSecondBuilder,
    comments_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampSecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampSecondBuilder,
}
pub struct ForceMajeureOverriderrp1;
pub struct ForceMajeureOverriderrp1Mapping([usize; 10]);
/// # Summary
///
/// ## OVERRIDERRP
///  _OVERRIDERRP shows details of override price periods._
///
/// * Data Set Name: Force Majeure
/// * File Name: Overriderrp
/// * Data Version: 1
///
/// # Description
///  OVERRIDERRP data is public, so is available to all participants. Source OVERRIDERRP updates every five minutes when override prices apply for the period.
///
///
///
/// # Primary Key Columns
///
/// * REGIONID
/// * STARTDATE
/// * STARTPERIOD
#[derive(Debug, PartialEq, Eq)]
pub struct ForceMajeureOverriderrp1Row<'data> {
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Starting date of override
    pub startdate: chrono::NaiveDateTime,
    /// Starting period of override
    pub startperiod: rust_decimal::Decimal,
    /// Termination date of override
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Terminate period of override
    pub endperiod: Option<rust_decimal::Decimal>,
    /// Dispatch Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Description of reason for override
    pub description: core::ops::Range<usize>,
    /// Authorise Start of Override
    pub authorisestart: core::ops::Range<usize>,
    /// Authorise End of Override
    pub authoriseend: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ForceMajeureOverriderrp1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn description(&self) -> Option<&str> {
        if self.description.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.description.clone(),
                ),
            )
        }
    }
    pub fn authorisestart(&self) -> Option<&str> {
        if self.authorisestart.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.authorisestart.clone(),
                ),
            )
        }
    }
    pub fn authoriseend(&self) -> Option<&str> {
        if self.authoriseend.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.authoriseend.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for ForceMajeureOverriderrp1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "FORCE_MAJEURE";
    const TABLE_NAME: &'static str = "OVERRIDERRP";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ForceMajeureOverriderrp1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "REGIONID",
        "STARTDATE",
        "STARTPERIOD",
        "ENDDATE",
        "ENDPERIOD",
        "RRP",
        "DESCRIPTION",
        "AUTHORISESTART",
        "AUTHORISEEND",
        "LASTCHANGED",
    ];
    type Row<'row> = ForceMajeureOverriderrp1Row<'row>;
    type FieldMapping = ForceMajeureOverriderrp1Mapping;
    type PrimaryKey = ForceMajeureOverriderrp1PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ForceMajeureOverriderrp1Row {
            regionid: row.get_range("regionid", field_mapping.0[0])?,
            startdate: row
                .get_custom_parsed_at_idx(
                    "startdate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            startperiod: row
                .get_custom_parsed_at_idx(
                    "startperiod",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            enddate: row
                .get_opt_custom_parsed_at_idx(
                    "enddate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            endperiod: row
                .get_opt_custom_parsed_at_idx(
                    "endperiod",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp: row
                .get_opt_custom_parsed_at_idx(
                    "rrp",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            description: row.get_opt_range("description", field_mapping.0[6])?,
            authorisestart: row.get_opt_range("authorisestart", field_mapping.0[7])?,
            authoriseend: row.get_opt_range("authoriseend", field_mapping.0[8])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[9],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(ForceMajeureOverriderrp1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ForceMajeureOverriderrp1PrimaryKey {
        ForceMajeureOverriderrp1PrimaryKey {
            regionid: row.regionid().to_string(),
            startdate: row.startdate,
            startperiod: row.startperiod,
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "force_majeure_overriderrp_v1".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ForceMajeureOverriderrp1Row {
            regionid: row.regionid.clone(),
            startdate: row.startdate.clone(),
            startperiod: row.startperiod.clone(),
            enddate: row.enddate.clone(),
            endperiod: row.endperiod.clone(),
            rrp: row.rrp.clone(),
            description: row.description.clone(),
            authorisestart: row.authorisestart.clone(),
            authoriseend: row.authoriseend.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ForceMajeureOverriderrp1PrimaryKey {
    pub regionid: alloc::string::String,
    pub startdate: chrono::NaiveDateTime,
    pub startperiod: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ForceMajeureOverriderrp1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ForceMajeureOverriderrp1Row<'data> {
    type Row<'other> = ForceMajeureOverriderrp1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.regionid() == row.regionid() && self.startdate == row.startdate
            && self.startperiod == row.startperiod
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for ForceMajeureOverriderrp1Row<'data> {
    type PrimaryKey = ForceMajeureOverriderrp1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid() == key.regionid && self.startdate == key.startdate
            && self.startperiod == key.startperiod
    }
}
impl<'data> mmsdm_core::CompareWithRow for ForceMajeureOverriderrp1PrimaryKey {
    type Row<'other> = ForceMajeureOverriderrp1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.regionid == row.regionid() && self.startdate == row.startdate
            && self.startperiod == row.startperiod
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ForceMajeureOverriderrp1PrimaryKey {
    type PrimaryKey = ForceMajeureOverriderrp1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid && self.startdate == key.startdate
            && self.startperiod == key.startperiod
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ForceMajeureOverriderrp1 {
    type Builder = ForceMajeureOverriderrp1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "startdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "startperiod",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "enddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "endperiod",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp",
                    arrow::datatypes::DataType::Decimal128(15, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "description",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authorisestart",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authoriseend",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ForceMajeureOverriderrp1Builder {
            regionid_array: arrow::array::builder::StringBuilder::new(),
            startdate_array: arrow::array::builder::TimestampSecondBuilder::new(),
            startperiod_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            enddate_array: arrow::array::builder::TimestampSecondBuilder::new(),
            endperiod_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 0)),
            description_array: arrow::array::builder::StringBuilder::new(),
            authorisestart_array: arrow::array::builder::StringBuilder::new(),
            authoriseend_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampSecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.regionid_array.append_value(row.regionid());
        builder.startdate_array.append_value(row.startdate.timestamp());
        builder
            .startperiod_array
            .append_value({
                let mut val = row.startperiod;
                val.rescale(0);
                val.mantissa()
            });
        builder.enddate_array.append_option(row.enddate.map(|val| val.timestamp()));
        builder
            .endperiod_array
            .append_option({
                row.endperiod
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .rrp_array
            .append_option({
                row.rrp
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.description_array.append_option(row.description());
        builder.authorisestart_array.append_option(row.authorisestart());
        builder.authoriseend_array.append_option(row.authoriseend());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startperiod_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.endperiod_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisestart_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseend_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ForceMajeureOverriderrp1Builder {
    regionid_array: arrow::array::builder::StringBuilder,
    startdate_array: arrow::array::builder::TimestampSecondBuilder,
    startperiod_array: arrow::array::builder::Decimal128Builder,
    enddate_array: arrow::array::builder::TimestampSecondBuilder,
    endperiod_array: arrow::array::builder::Decimal128Builder,
    rrp_array: arrow::array::builder::Decimal128Builder,
    description_array: arrow::array::builder::StringBuilder,
    authorisestart_array: arrow::array::builder::StringBuilder,
    authoriseend_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampSecondBuilder,
}
pub struct ApRegionapc1;
pub struct ApRegionapc1Mapping([usize; 6]);
/// # Summary
///
/// ## REGIONAPC
///  _REGIONAPC defines Administered Price profiles (Energy and FCAS) for a region._
///
/// * Data Set Name: Ap
/// * File Name: Regionapc
/// * Data Version: 1
///
/// # Description
///  REGIONAPC data is public, so is available to all participants. Source REGIONAPC updates when a change is ever made to the Administered Price Cap details. Changes to this table are infrequent.
///
///
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct ApRegionapc1Row<'data> {
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Date the APC profile applies from
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number for the same date
    pub versionno: rust_decimal::Decimal,
    /// Authorised date
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorised by
    pub authorisedby: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ApRegionapc1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn authorisedby(&self) -> Option<&str> {
        if self.authorisedby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.authorisedby.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for ApRegionapc1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "AP";
    const TABLE_NAME: &'static str = "REGIONAPC";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ApRegionapc1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "REGIONID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "LASTCHANGED",
    ];
    type Row<'row> = ApRegionapc1Row<'row>;
    type FieldMapping = ApRegionapc1Mapping;
    type PrimaryKey = ApRegionapc1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ApRegionapc1Row {
            regionid: row.get_range("regionid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[4])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(ApRegionapc1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let effectivedate = row
            .get_custom_parsed_at_idx(
                "effectivedate",
                5,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(effectivedate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ApRegionapc1PrimaryKey {
        ApRegionapc1PrimaryKey {
            effectivedate: row.effectivedate,
            regionid: row.regionid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.effectivedate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "ap_regionapc_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ApRegionapc1Row {
            regionid: row.regionid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApRegionapc1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ApRegionapc1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ApRegionapc1Row<'data> {
    type Row<'other> = ApRegionapc1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.regionid() == row.regionid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for ApRegionapc1Row<'data> {
    type PrimaryKey = ApRegionapc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.regionid() == key.regionid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for ApRegionapc1PrimaryKey {
    type Row<'other> = ApRegionapc1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.regionid == row.regionid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ApRegionapc1PrimaryKey {
    type PrimaryKey = ApRegionapc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ApRegionapc1 {
    type Builder = ApRegionapc1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "authoriseddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authorisedby",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ApRegionapc1Builder {
            regionid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampSecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            authoriseddate_array: arrow::array::builder::TimestampSecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampSecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.regionid_array.append_value(row.regionid());
        builder.effectivedate_array.append_value(row.effectivedate.timestamp());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .authoriseddate_array
            .append_option(row.authoriseddate.map(|val| val.timestamp()));
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ApRegionapc1Builder {
    regionid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampSecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    authoriseddate_array: arrow::array::builder::TimestampSecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampSecondBuilder,
}
pub struct ApRegionapcintervals1;
pub struct ApRegionapcintervals1Mapping([usize; 9]);
/// # Summary
///
/// ## REGIONAPCINTERVALS
///  _REGIONAPCINTERVALS contains Administered Price profiles (Energy and FCAS) applicable to each interval for a region._
///
/// * Data Set Name: Ap
/// * File Name: Regionapcintervals
/// * Data Version: 1
///
/// # Description
///  REGIONAPCINTERVALS data is public, so is available to all participants. Source REGIONAPCINTERVALS is updated whenever an Administered Price Cap occurs.
///
///
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PERIODID
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct ApRegionapcintervals1Row<'data> {
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Date the APC profile applies from
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number for the same date
    pub versionno: rust_decimal::Decimal,
    /// 30-minute period
    pub periodid: rust_decimal::Decimal,
    /// Administered price cap in $
    pub apcvalue: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// not used
    pub apctype: Option<rust_decimal::Decimal>,
    /// FCAS Administered price cap in $
    pub fcasapcvalue: Option<rust_decimal::Decimal>,
    /// Administered price floor in $
    pub apfvalue: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> ApRegionapcintervals1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for ApRegionapcintervals1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "AP";
    const TABLE_NAME: &'static str = "REGIONAPCINTERVALS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = ApRegionapcintervals1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "REGIONID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "PERIODID",
        "APCVALUE",
        "LASTCHANGED",
        "APCTYPE",
        "FCASAPCVALUE",
        "APFVALUE",
    ];
    type Row<'row> = ApRegionapcintervals1Row<'row>;
    type FieldMapping = ApRegionapcintervals1Mapping;
    type PrimaryKey = ApRegionapcintervals1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(ApRegionapcintervals1Row {
            regionid: row.get_range("regionid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            apcvalue: row
                .get_opt_custom_parsed_at_idx(
                    "apcvalue",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            apctype: row
                .get_opt_custom_parsed_at_idx(
                    "apctype",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            fcasapcvalue: row
                .get_opt_custom_parsed_at_idx(
                    "fcasapcvalue",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            apfvalue: row
                .get_opt_custom_parsed_at_idx(
                    "apfvalue",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(ApRegionapcintervals1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let effectivedate = row
            .get_custom_parsed_at_idx(
                "effectivedate",
                5,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(effectivedate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> ApRegionapcintervals1PrimaryKey {
        ApRegionapcintervals1PrimaryKey {
            effectivedate: row.effectivedate,
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.effectivedate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "ap_regionapcintervals_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        ApRegionapcintervals1Row {
            regionid: row.regionid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            periodid: row.periodid.clone(),
            apcvalue: row.apcvalue.clone(),
            lastchanged: row.lastchanged.clone(),
            apctype: row.apctype.clone(),
            fcasapcvalue: row.fcasapcvalue.clone(),
            apfvalue: row.apfvalue.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApRegionapcintervals1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ApRegionapcintervals1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for ApRegionapcintervals1Row<'data> {
    type Row<'other> = ApRegionapcintervals1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.periodid == row.periodid
            && self.regionid() == row.regionid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for ApRegionapcintervals1Row<'data> {
    type PrimaryKey = ApRegionapcintervals1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.periodid == key.periodid
            && self.regionid() == key.regionid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for ApRegionapcintervals1PrimaryKey {
    type Row<'other> = ApRegionapcintervals1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.periodid == row.periodid
            && self.regionid == row.regionid() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ApRegionapcintervals1PrimaryKey {
    type PrimaryKey = ApRegionapcintervals1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.periodid == key.periodid
            && self.regionid == key.regionid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ApRegionapcintervals1 {
    type Builder = ApRegionapcintervals1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "apcvalue",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Second,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "apctype",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "fcasapcvalue",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "apfvalue",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        ApRegionapcintervals1Builder {
            regionid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampSecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            apcvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampSecondBuilder::new(),
            apctype_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            fcasapcvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            apfvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.regionid_array.append_value(row.regionid());
        builder.effectivedate_array.append_value(row.effectivedate.timestamp());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .apcvalue_array
            .append_option({
                row.apcvalue
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp()));
        builder
            .apctype_array
            .append_option({
                row.apctype
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .fcasapcvalue_array
            .append_option({
                row.fcasapcvalue
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .apfvalue_array
            .append_option({
                row.apfvalue
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.apcvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.apctype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fcasapcvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.apfvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct ApRegionapcintervals1Builder {
    regionid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampSecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    apcvalue_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampSecondBuilder,
    apctype_array: arrow::array::builder::Decimal128Builder,
    fcasapcvalue_array: arrow::array::builder::Decimal128Builder,
    apfvalue_array: arrow::array::builder::Decimal128Builder,
}
