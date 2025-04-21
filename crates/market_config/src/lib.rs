#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct MarketConfigBidtypes1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MarketConfigBidtypes1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MarketConfigBidtypes1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MarketConfigBidtypes1Mapping([usize; 9]);
/// # Summary
///
/// ## BIDTYPES
///
/// BIDTYPES, together with the associated tracking data in BIDTYPESTRK, define a set of ancillary services with bidding parameters from a given date.BIDTYPES is static data describing each type of bid quantity, the number of applicable bands, how many days ahead a price lock down becomes effective and the validation rule that applies.
///
/// * Data Set Name: Market Config
/// * File Name: Bidtypes
/// * Data Version: 1
///
/// # Description
/// BIDTYPES is public to participantsSourceBIDTYPES updates when the static data relating to an ancillary service type is modified.VolumeExpect modifications to be rare. Allow for approximately 20 records per year.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct MarketConfigBidtypes1Row<'data> {
    /// Bid Type Identifier
    pub bidtype: core::ops::Range<usize>,
    /// Market date starting at 04:30 inclusive
    pub effectivedate: chrono::NaiveDateTime,
    /// Record version number
    pub versionno: rust_decimal::Decimal,
    /// Description of this Bid Type
    pub description: core::ops::Range<usize>,
    /// Number of active bands (1 to 10)
    pub numberofbands: Option<rust_decimal::Decimal>,
    /// Number of days prior to the Market Day when prices are locked from 12:30pm
    pub numdaysaheadpricelocked: Option<rust_decimal::Decimal>,
    /// ENERGY or AS validation rules to apply.
    pub validationrule: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Alias for this BIDTYPE used in the SPD Solver
    pub spdalias: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MarketConfigBidtypes1Row<'data> {
    pub fn bidtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bidtype.clone())
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
    pub fn validationrule(&self) -> Option<&str> {
        if self.validationrule.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.validationrule.clone(),
                ),
            )
        }
    }
    pub fn spdalias(&self) -> Option<&str> {
        if self.spdalias.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.spdalias.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for MarketConfigBidtypes1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MARKET_CONFIG";
    const TABLE_NAME: &'static str = "BIDTYPES";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketConfigBidtypes1Mapping([
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
        "BIDTYPE",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "DESCRIPTION",
        "NUMBEROFBANDS",
        "NUMDAYSAHEADPRICELOCKED",
        "VALIDATIONRULE",
        "LASTCHANGED",
        "SPDALIAS",
    ];
    type Row<'row> = MarketConfigBidtypes1Row<'row>;
    type FieldMapping = MarketConfigBidtypes1Mapping;
    type PrimaryKey = MarketConfigBidtypes1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketConfigBidtypes1Row {
            bidtype: row.get_range("bidtype", field_mapping.0[0])?,
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
            description: row.get_opt_range("description", field_mapping.0[3])?,
            numberofbands: row
                .get_opt_custom_parsed_at_idx(
                    "numberofbands",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            numdaysaheadpricelocked: row
                .get_opt_custom_parsed_at_idx(
                    "numdaysaheadpricelocked",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            validationrule: row.get_opt_range("validationrule", field_mapping.0[6])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            spdalias: row.get_opt_range("spdalias", field_mapping.0[8])?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !row.is_heading() {
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
        Ok(MarketConfigBidtypes1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MarketConfigBidtypes1PrimaryKey {
        MarketConfigBidtypes1PrimaryKey {
            bidtype: row.bidtype().to_string(),
            effectivedate: row.effectivedate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("market_config_bidtypes_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketConfigBidtypes1Row {
            bidtype: row.bidtype.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            description: row.description.clone(),
            numberofbands: row.numberofbands.clone(),
            numdaysaheadpricelocked: row.numdaysaheadpricelocked.clone(),
            validationrule: row.validationrule.clone(),
            lastchanged: row.lastchanged.clone(),
            spdalias: row.spdalias.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketConfigBidtypes1PrimaryKey {
    pub bidtype: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigBidtypes1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigBidtypes1Row<'data> {
    type Row<'other> = MarketConfigBidtypes1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype() && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MarketConfigBidtypes1Row<'data> {
    type PrimaryKey = MarketConfigBidtypes1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigBidtypes1PrimaryKey {
    type Row<'other> = MarketConfigBidtypes1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype() && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigBidtypes1PrimaryKey {
    type PrimaryKey = MarketConfigBidtypes1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigBidtypes1 {
    type Builder = MarketConfigBidtypes1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "bidtype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
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
                    "description",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "numberofbands",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "numdaysaheadpricelocked",
                    arrow::datatypes::DataType::Decimal128(2, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "validationrule",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "spdalias",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketConfigBidtypes1Builder {
            bidtype_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            description_array: arrow::array::builder::StringBuilder::new(),
            numberofbands_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            numdaysaheadpricelocked_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(2, 0)),
            validationrule_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            spdalias_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.bidtype_array.append_value(row.bidtype());
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.description_array.append_option(row.description());
        builder
            .numberofbands_array
            .append_option({
                row.numberofbands
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .numdaysaheadpricelocked_array
            .append_option({
                row.numdaysaheadpricelocked
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.validationrule_array.append_option(row.validationrule());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.spdalias_array.append_option(row.spdalias());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.bidtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.numberofbands_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.numdaysaheadpricelocked_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.validationrule_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.spdalias_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MarketConfigBidtypes1Builder {
    bidtype_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    description_array: arrow::array::builder::StringBuilder,
    numberofbands_array: arrow::array::builder::Decimal128Builder,
    numdaysaheadpricelocked_array: arrow::array::builder::Decimal128Builder,
    validationrule_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    spdalias_array: arrow::array::builder::StringBuilder,
}
pub struct MarketConfigBidtypestrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MarketConfigBidtypestrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MarketConfigBidtypestrk1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MarketConfigBidtypestrk1Mapping([usize; 5]);
/// # Summary
///
/// ## BIDTYPESTRK
///
/// BIDTYPESTRK, together with the associated data in BIDTYPES, define a set of ancillary services with bidding parameters from a given date.
///
/// * Data Set Name: Market Config
/// * File Name: Bidtypestrk
/// * Data Version: 1
///
/// # Description
/// BIDTYPESTRK is public to participantsSourceBIDTYPESTRK updates when the static data relating to an ancillary service type is modified.VolumeExpect modifications to be rare. Allow for approximately 20 records per year.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct MarketConfigBidtypestrk1Row<'data> {
    /// Market date starting at 04:30 inclusive
    pub effectivedate: chrono::NaiveDateTime,
    /// Record version number
    pub versionno: rust_decimal::Decimal,
    /// Date of record authorisation. A NULL value indicates the record is not authorised.
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User that authorised record. A NULL value indicates the record is not authorised.
    pub authorisedby: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MarketConfigBidtypestrk1Row<'data> {
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
impl mmsdm_core::GetTable for MarketConfigBidtypestrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MARKET_CONFIG";
    const TABLE_NAME: &'static str = "BIDTYPESTRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketConfigBidtypestrk1Mapping([
        4,
        5,
        6,
        7,
        8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSIONNO",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "LASTCHANGED",
    ];
    type Row<'row> = MarketConfigBidtypestrk1Row<'row>;
    type FieldMapping = MarketConfigBidtypestrk1Mapping;
    type PrimaryKey = MarketConfigBidtypestrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketConfigBidtypestrk1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[3])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !row.is_heading() {
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
        Ok(MarketConfigBidtypestrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MarketConfigBidtypestrk1PrimaryKey {
        MarketConfigBidtypestrk1PrimaryKey {
            effectivedate: row.effectivedate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("market_config_bidtypestrk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketConfigBidtypestrk1Row {
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
pub struct MarketConfigBidtypestrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigBidtypestrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigBidtypestrk1Row<'data> {
    type Row<'other> = MarketConfigBidtypestrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MarketConfigBidtypestrk1Row<'data> {
    type PrimaryKey = MarketConfigBidtypestrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigBidtypestrk1PrimaryKey {
    type Row<'other> = MarketConfigBidtypestrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigBidtypestrk1PrimaryKey {
    type PrimaryKey = MarketConfigBidtypestrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigBidtypestrk1 {
    type Builder = MarketConfigBidtypestrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
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
                        arrow::datatypes::TimeUnit::Millisecond,
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
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketConfigBidtypestrk1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
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
pub struct MarketConfigBidtypestrk1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MarketConfigFcasregusefactors1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MarketConfigFcasregusefactors1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MarketConfigFcasregusefactors1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MarketConfigFcasregusefactors1Mapping([usize; 7]);
/// # Summary
///
/// ## FCAS_REGU_USAGE_FACTORS
///
/// Stores the proportion of enabled regulation FCAS dispatch that is typically consumed for frequency regulation. Used to calculate the projected state of charge for energy storage systems.
///
/// * Data Set Name: Market Config
/// * File Name: Fcasregusefactors
/// * Data Version: 1
///
/// # Description
///
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * EFFECTIVEDATE
/// * PERIODID
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct MarketConfigFcasregusefactors1Row<'data> {
    /// The effective date for this regulation FCAS usage factor
    pub effectivedate: chrono::NaiveDateTime,
    /// Version with respect to effective date
    pub versionno: rust_decimal::Decimal,
    /// Unique RegionID
    pub regionid: core::ops::Range<usize>,
    /// The type of regulation FCAS service [RAISEREG,LOWERREG]
    pub bidtype: core::ops::Range<usize>,
    /// The Period ID (1 - 48) within the calendar day to which this usage factor applies
    pub periodid: rust_decimal::Decimal,
    /// The proportion of cleared regulation FCAS that is assumed to be used within a dispatch interval. Expressed as a fractional amount between 0 and 1
    pub usage_factor: Option<rust_decimal::Decimal>,
    /// The last time the data has been changed/updated
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MarketConfigFcasregusefactors1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn bidtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bidtype.clone())
    }
}
impl mmsdm_core::GetTable for MarketConfigFcasregusefactors1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MARKET_CONFIG";
    const TABLE_NAME: &'static str = "FCASREGUSEFACTORS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketConfigFcasregusefactors1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSIONNO",
        "REGIONID",
        "BIDTYPE",
        "PERIODID",
        "USAGE_FACTOR",
        "LASTCHANGED",
    ];
    type Row<'row> = MarketConfigFcasregusefactors1Row<'row>;
    type FieldMapping = MarketConfigFcasregusefactors1Mapping;
    type PrimaryKey = MarketConfigFcasregusefactors1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketConfigFcasregusefactors1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            bidtype: row.get_range("bidtype", field_mapping.0[3])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            usage_factor: row
                .get_opt_custom_parsed_at_idx(
                    "usage_factor",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !row.is_heading() {
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
        Ok(MarketConfigFcasregusefactors1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MarketConfigFcasregusefactors1PrimaryKey {
        MarketConfigFcasregusefactors1PrimaryKey {
            bidtype: row.bidtype().to_string(),
            effectivedate: row.effectivedate,
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "market_config_fcasregusefactors_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketConfigFcasregusefactors1Row {
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            regionid: row.regionid.clone(),
            bidtype: row.bidtype.clone(),
            periodid: row.periodid.clone(),
            usage_factor: row.usage_factor.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketConfigFcasregusefactors1PrimaryKey {
    pub bidtype: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigFcasregusefactors1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigFcasregusefactors1Row<'data> {
    type Row<'other> = MarketConfigFcasregusefactors1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype() && self.effectivedate == row.effectivedate
            && self.periodid == row.periodid && self.regionid() == row.regionid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for MarketConfigFcasregusefactors1Row<'data> {
    type PrimaryKey = MarketConfigFcasregusefactors1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype && self.effectivedate == key.effectivedate
            && self.periodid == key.periodid && self.regionid() == key.regionid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigFcasregusefactors1PrimaryKey {
    type Row<'other> = MarketConfigFcasregusefactors1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype() && self.effectivedate == row.effectivedate
            && self.periodid == row.periodid && self.regionid == row.regionid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigFcasregusefactors1PrimaryKey {
    type PrimaryKey = MarketConfigFcasregusefactors1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.effectivedate == key.effectivedate
            && self.periodid == key.periodid && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigFcasregusefactors1 {
    type Builder = MarketConfigFcasregusefactors1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
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
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "bidtype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "usage_factor",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketConfigFcasregusefactors1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            bidtype_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            usage_factor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.regionid_array.append_value(row.regionid());
        builder.bidtype_array.append_value(row.bidtype());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .usage_factor_array
            .append_option({
                row.usage_factor
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.usage_factor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MarketConfigFcasregusefactors1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    bidtype_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    usage_factor_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MarketConfigFcasregusefactorsTrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MarketConfigFcasregusefactorsTrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MarketConfigFcasregusefactorsTrk1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MarketConfigFcasregusefactorsTrk1Mapping([usize; 4]);
/// # Summary
///
/// ## FCAS_REGU_USAGE_FACTORS_TRK
///
/// Stores the proportion of enabled regulation FCAS dispatch that is typically consumed for frequency regulation. Used to calculate the projected state of charge for energy storage systems.
///
/// * Data Set Name: Market Config
/// * File Name: Fcasregusefactors Trk
/// * Data Version: 1
///
/// # Description
/// INTERCONNECTOR is public data, available to all participants.SourceINTERCONNECTOR changes infrequently, usually annually.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct MarketConfigFcasregusefactorsTrk1Row<'data> {
    /// The effective date for this regulation FCAS usage factor
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of the date with respect to effective date
    pub versionno: rust_decimal::Decimal,
    /// The date time that this set of usage factors was authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// The last time the data has been changed/updated
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: core::marker::PhantomData<&'data ()>,
}
impl<'data> MarketConfigFcasregusefactorsTrk1Row<'data> {}
impl mmsdm_core::GetTable for MarketConfigFcasregusefactorsTrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MARKET_CONFIG";
    const TABLE_NAME: &'static str = "FCASREGUSEFACTORS_TRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketConfigFcasregusefactorsTrk1Mapping([
        4,
        5,
        6,
        7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSIONNO",
        "AUTHORISEDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = MarketConfigFcasregusefactorsTrk1Row<'row>;
    type FieldMapping = MarketConfigFcasregusefactorsTrk1Mapping;
    type PrimaryKey = MarketConfigFcasregusefactorsTrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketConfigFcasregusefactorsTrk1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: core::marker::PhantomData,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !row.is_heading() {
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
        Ok(MarketConfigFcasregusefactorsTrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MarketConfigFcasregusefactorsTrk1PrimaryKey {
        MarketConfigFcasregusefactorsTrk1PrimaryKey {
            effectivedate: row.effectivedate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "market_config_fcasregusefactors_trk_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketConfigFcasregusefactorsTrk1Row {
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            authoriseddate: row.authoriseddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: core::marker::PhantomData,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketConfigFcasregusefactorsTrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigFcasregusefactorsTrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigFcasregusefactorsTrk1Row<'data> {
    type Row<'other> = MarketConfigFcasregusefactorsTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for MarketConfigFcasregusefactorsTrk1Row<'data> {
    type PrimaryKey = MarketConfigFcasregusefactorsTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigFcasregusefactorsTrk1PrimaryKey {
    type Row<'other> = MarketConfigFcasregusefactorsTrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigFcasregusefactorsTrk1PrimaryKey {
    type PrimaryKey = MarketConfigFcasregusefactorsTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigFcasregusefactorsTrk1 {
    type Builder = MarketConfigFcasregusefactorsTrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
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
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketConfigFcasregusefactorsTrk1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
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
pub struct MarketConfigFcasregusefactorsTrk1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MarketConfigInterconnector1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MarketConfigInterconnector1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MarketConfigInterconnector1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MarketConfigInterconnector1Mapping([usize; 6]);
/// # Summary
///
/// ## INTERCONNECTOR
///
/// INTERCONNECTOR sets out valid identifiers for each interconnector.
///
/// * Data Set Name: Market Config
/// * File Name: Interconnector
/// * Data Version: 1
///
/// # Description
/// INTERCONNECTOR is public data, available to all participants.SourceINTERCONNECTOR changes infrequently, usually annually.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * INTERCONNECTORID
#[derive(Debug, PartialEq, Eq)]
pub struct MarketConfigInterconnector1Row<'data> {
    /// Unique Id of this interconnector
    pub interconnectorid: core::ops::Range<usize>,
    /// Starting region of the interconnect
    pub regionfrom: core::ops::Range<usize>,
    /// Not used
    pub rsoid: core::ops::Range<usize>,
    /// Ending region of the interconnect
    pub regionto: core::ops::Range<usize>,
    /// Description of interconnector
    pub description: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MarketConfigInterconnector1Row<'data> {
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn regionfrom(&self) -> Option<&str> {
        if self.regionfrom.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.regionfrom.clone(),
                ),
            )
        }
    }
    pub fn rsoid(&self) -> Option<&str> {
        if self.rsoid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(self.backing_data.as_slice(), self.rsoid.clone()),
            )
        }
    }
    pub fn regionto(&self) -> Option<&str> {
        if self.regionto.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.regionto.clone(),
                ),
            )
        }
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
}
impl mmsdm_core::GetTable for MarketConfigInterconnector1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MARKET_CONFIG";
    const TABLE_NAME: &'static str = "INTERCONNECTOR";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketConfigInterconnector1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "INTERCONNECTORID",
        "REGIONFROM",
        "RSOID",
        "REGIONTO",
        "DESCRIPTION",
        "LASTCHANGED",
    ];
    type Row<'row> = MarketConfigInterconnector1Row<'row>;
    type FieldMapping = MarketConfigInterconnector1Mapping;
    type PrimaryKey = MarketConfigInterconnector1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketConfigInterconnector1Row {
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[0])?,
            regionfrom: row.get_opt_range("regionfrom", field_mapping.0[1])?,
            rsoid: row.get_opt_range("rsoid", field_mapping.0[2])?,
            regionto: row.get_opt_range("regionto", field_mapping.0[3])?,
            description: row.get_opt_range("description", field_mapping.0[4])?,
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
        if !row.is_heading() {
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
        Ok(MarketConfigInterconnector1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MarketConfigInterconnector1PrimaryKey {
        MarketConfigInterconnector1PrimaryKey {
            interconnectorid: row.interconnectorid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("market_config_interconnector_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketConfigInterconnector1Row {
            interconnectorid: row.interconnectorid.clone(),
            regionfrom: row.regionfrom.clone(),
            rsoid: row.rsoid.clone(),
            regionto: row.regionto.clone(),
            description: row.description.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketConfigInterconnector1PrimaryKey {
    pub interconnectorid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for MarketConfigInterconnector1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigInterconnector1Row<'data> {
    type Row<'other> = MarketConfigInterconnector1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interconnectorid() == row.interconnectorid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MarketConfigInterconnector1Row<'data> {
    type PrimaryKey = MarketConfigInterconnector1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid() == key.interconnectorid
    }
}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigInterconnector1PrimaryKey {
    type Row<'other> = MarketConfigInterconnector1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interconnectorid == row.interconnectorid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigInterconnector1PrimaryKey {
    type PrimaryKey = MarketConfigInterconnector1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigInterconnector1 {
    type Builder = MarketConfigInterconnector1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionfrom",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rsoid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regionto",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "description",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketConfigInterconnector1Builder {
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            regionfrom_array: arrow::array::builder::StringBuilder::new(),
            rsoid_array: arrow::array::builder::StringBuilder::new(),
            regionto_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.regionfrom_array.append_option(row.regionfrom());
        builder.rsoid_array.append_option(row.rsoid());
        builder.regionto_array.append_option(row.regionto());
        builder.description_array.append_option(row.description());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionfrom_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rsoid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionto_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MarketConfigInterconnector1Builder {
    interconnectorid_array: arrow::array::builder::StringBuilder,
    regionfrom_array: arrow::array::builder::StringBuilder,
    rsoid_array: arrow::array::builder::StringBuilder,
    regionto_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MarketConfigInterconnectoralloc1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MarketConfigInterconnectoralloc1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MarketConfigInterconnectoralloc1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MarketConfigInterconnectoralloc1Mapping([usize; 7]);
/// # Summary
///
/// ## INTERCONNECTORALLOC
///
/// INTERCONNECTORALLOC shows allocations of interconnector residues to Network Service Providers.
///
/// * Data Set Name: Market Config
/// * File Name: Interconnectoralloc
/// * Data Version: 1
///
/// # Description
/// INTERCONNECTORALLOC data is confidential to the relevant participant.SourceINTERCONNECTORALLOC changes infrequently, typically annually.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct MarketConfigInterconnectoralloc1Row<'data> {
    /// Effective Date of Allocation Details
    pub effectivedate: chrono::NaiveDateTime,
    /// Version No in respect to effective date
    pub versionno: rust_decimal::Decimal,
    /// Interconnector identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Allocation % / 100
    pub allocation: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MarketConfigInterconnectoralloc1Row<'data> {
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
}
impl mmsdm_core::GetTable for MarketConfigInterconnectoralloc1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MARKET_CONFIG";
    const TABLE_NAME: &'static str = "INTERCONNECTORALLOC";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketConfigInterconnectoralloc1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSIONNO",
        "INTERCONNECTORID",
        "REGIONID",
        "PARTICIPANTID",
        "ALLOCATION",
        "LASTCHANGED",
    ];
    type Row<'row> = MarketConfigInterconnectoralloc1Row<'row>;
    type FieldMapping = MarketConfigInterconnectoralloc1Mapping;
    type PrimaryKey = MarketConfigInterconnectoralloc1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketConfigInterconnectoralloc1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[2])?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            participantid: row.get_range("participantid", field_mapping.0[4])?,
            allocation: row
                .get_opt_custom_parsed_at_idx(
                    "allocation",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !row.is_heading() {
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
        Ok(MarketConfigInterconnectoralloc1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MarketConfigInterconnectoralloc1PrimaryKey {
        MarketConfigInterconnectoralloc1PrimaryKey {
            effectivedate: row.effectivedate,
            interconnectorid: row.interconnectorid().to_string(),
            participantid: row.participantid().to_string(),
            regionid: row.regionid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "market_config_interconnectoralloc_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketConfigInterconnectoralloc1Row {
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            interconnectorid: row.interconnectorid.clone(),
            regionid: row.regionid.clone(),
            participantid: row.participantid.clone(),
            allocation: row.allocation.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketConfigInterconnectoralloc1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub interconnectorid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub regionid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigInterconnectoralloc1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigInterconnectoralloc1Row<'data> {
    type Row<'other> = MarketConfigInterconnectoralloc1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid() == row.interconnectorid()
            && self.participantid() == row.participantid()
            && self.regionid() == row.regionid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for MarketConfigInterconnectoralloc1Row<'data> {
    type PrimaryKey = MarketConfigInterconnectoralloc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid() == key.interconnectorid
            && self.participantid() == key.participantid
            && self.regionid() == key.regionid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigInterconnectoralloc1PrimaryKey {
    type Row<'other> = MarketConfigInterconnectoralloc1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid == row.interconnectorid()
            && self.participantid == row.participantid()
            && self.regionid == row.regionid() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigInterconnectoralloc1PrimaryKey {
    type PrimaryKey = MarketConfigInterconnectoralloc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigInterconnectoralloc1 {
    type Builder = MarketConfigInterconnectoralloc1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(5, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "allocation",
                    arrow::datatypes::DataType::Decimal128(12, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketConfigInterconnectoralloc1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 0)),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            allocation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.regionid_array.append_value(row.regionid());
        builder.participantid_array.append_value(row.participantid());
        builder
            .allocation_array
            .append_option({
                row.allocation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.allocation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MarketConfigInterconnectoralloc1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    allocation_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MarketConfigInterconnectorconstraint1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MarketConfigInterconnectorconstraint1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MarketConfigInterconnectorconstraint1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MarketConfigInterconnectorconstraint1Mapping([usize; 22]);
/// # Summary
///
/// ## INTERCONNECTORCONSTRAINT
///
/// INTERCONNECTORCONSTRAINT sets out Interconnector limit data used as defaults in dispatch, predispatch and STPASA and used by SPD in calculating flows. INTERCONNECTORCONSTRAINT includes an additional field to restrict an interconnector from support transfer of FCAS.
///
/// * Data Set Name: Market Config
/// * File Name: Interconnectorconstraint
/// * Data Version: 1
///
/// # Description
/// INTERCONNECTORCONSTRAINT is public data, available to all participants.SourceINTERCONNECTORCONSTRAINT changes infrequently, typically annually.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * INTERCONNECTORID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct MarketConfigInterconnectorconstraint1Row<'data> {
    /// SPD Factor
    pub reserveoverallloadfactor: Option<rust_decimal::Decimal>,
    /// Loss share attributable to from region
    pub fromregionlossshare: Option<rust_decimal::Decimal>,
    /// Date that this limit is effective from
    pub effectivedate: chrono::NaiveDateTime,
    /// Version for this date
    pub versionno: rust_decimal::Decimal,
    /// Unique Id of this interconnector
    pub interconnectorid: core::ops::Range<usize>,
    /// Limit of energy flowing into the RegionFrom
    pub maxmwin: Option<rust_decimal::Decimal>,
    /// Limit of energy flowing out of the Region
    pub maxmwout: Option<rust_decimal::Decimal>,
    /// Constant Loss factor
    pub lossconstant: Option<rust_decimal::Decimal>,
    /// Linear coefficient of loss factor calculation
    pub lossflowcoefficient: Option<rust_decimal::Decimal>,
    /// Identifies the EMS entity that represents the interconnector flow
    pub emsmeasurand: core::ops::Range<usize>,
    /// User authorising record
    pub authorisedby: core::ops::Range<usize>,
    /// Date record authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Not used
    pub dynamicrhs: core::ops::Range<usize>,
    /// Interconnector import limit
    pub importlimit: Option<rust_decimal::Decimal>,
    /// Interconnector export limit
    pub exportlimit: Option<rust_decimal::Decimal>,
    /// SPD Factor
    pub outagederationfactor: Option<rust_decimal::Decimal>,
    /// Factor for non-physical losses rerun
    pub nonphysicallossfactor: Option<rust_decimal::Decimal>,
    /// Interconnector overload for 60 sec
    pub overloadfactor60sec: Option<rust_decimal::Decimal>,
    /// Interconnector overload for 6 sec
    pub overloadfactor6sec: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag to indicate that the interconnector cannot support FCAS Transfers
    pub fcassupportunavailable: Option<rust_decimal::Decimal>,
    /// Interconnector type - Currently either "REGULATED"or "MNSP"
    pub ictype: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MarketConfigInterconnectorconstraint1Row<'data> {
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn emsmeasurand(&self) -> Option<&str> {
        if self.emsmeasurand.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.emsmeasurand.clone(),
                ),
            )
        }
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
    pub fn dynamicrhs(&self) -> Option<&str> {
        if self.dynamicrhs.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.dynamicrhs.clone(),
                ),
            )
        }
    }
    pub fn ictype(&self) -> Option<&str> {
        if self.ictype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.ictype.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for MarketConfigInterconnectorconstraint1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MARKET_CONFIG";
    const TABLE_NAME: &'static str = "INTERCONNECTORCONSTRAINT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketConfigInterconnectorconstraint1Mapping([
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
        20,
        21,
        22,
        23,
        24,
        25,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RESERVEOVERALLLOADFACTOR",
        "FROMREGIONLOSSSHARE",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "INTERCONNECTORID",
        "MAXMWIN",
        "MAXMWOUT",
        "LOSSCONSTANT",
        "LOSSFLOWCOEFFICIENT",
        "EMSMEASURAND",
        "AUTHORISEDBY",
        "AUTHORISEDDATE",
        "DYNAMICRHS",
        "IMPORTLIMIT",
        "EXPORTLIMIT",
        "OUTAGEDERATIONFACTOR",
        "NONPHYSICALLOSSFACTOR",
        "OVERLOADFACTOR60SEC",
        "OVERLOADFACTOR6SEC",
        "LASTCHANGED",
        "FCASSUPPORTUNAVAILABLE",
        "ICTYPE",
    ];
    type Row<'row> = MarketConfigInterconnectorconstraint1Row<'row>;
    type FieldMapping = MarketConfigInterconnectorconstraint1Mapping;
    type PrimaryKey = MarketConfigInterconnectorconstraint1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketConfigInterconnectorconstraint1Row {
            reserveoverallloadfactor: row
                .get_opt_custom_parsed_at_idx(
                    "reserveoverallloadfactor",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            fromregionlossshare: row
                .get_opt_custom_parsed_at_idx(
                    "fromregionlossshare",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[4])?,
            maxmwin: row
                .get_opt_custom_parsed_at_idx(
                    "maxmwin",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxmwout: row
                .get_opt_custom_parsed_at_idx(
                    "maxmwout",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lossconstant: row
                .get_opt_custom_parsed_at_idx(
                    "lossconstant",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lossflowcoefficient: row
                .get_opt_custom_parsed_at_idx(
                    "lossflowcoefficient",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            emsmeasurand: row.get_opt_range("emsmeasurand", field_mapping.0[9])?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[10])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[11],
                    mmsdm_core::mms_datetime::parse,
                )?,
            dynamicrhs: row.get_opt_range("dynamicrhs", field_mapping.0[12])?,
            importlimit: row
                .get_opt_custom_parsed_at_idx(
                    "importlimit",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            exportlimit: row
                .get_opt_custom_parsed_at_idx(
                    "exportlimit",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            outagederationfactor: row
                .get_opt_custom_parsed_at_idx(
                    "outagederationfactor",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            nonphysicallossfactor: row
                .get_opt_custom_parsed_at_idx(
                    "nonphysicallossfactor",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            overloadfactor60sec: row
                .get_opt_custom_parsed_at_idx(
                    "overloadfactor60sec",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            overloadfactor6sec: row
                .get_opt_custom_parsed_at_idx(
                    "overloadfactor6sec",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[19],
                    mmsdm_core::mms_datetime::parse,
                )?,
            fcassupportunavailable: row
                .get_opt_custom_parsed_at_idx(
                    "fcassupportunavailable",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ictype: row.get_opt_range("ictype", field_mapping.0[21])?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !row.is_heading() {
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
        Ok(MarketConfigInterconnectorconstraint1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> MarketConfigInterconnectorconstraint1PrimaryKey {
        MarketConfigInterconnectorconstraint1PrimaryKey {
            effectivedate: row.effectivedate,
            interconnectorid: row.interconnectorid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "market_config_interconnectorconstraint_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketConfigInterconnectorconstraint1Row {
            reserveoverallloadfactor: row.reserveoverallloadfactor.clone(),
            fromregionlossshare: row.fromregionlossshare.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            interconnectorid: row.interconnectorid.clone(),
            maxmwin: row.maxmwin.clone(),
            maxmwout: row.maxmwout.clone(),
            lossconstant: row.lossconstant.clone(),
            lossflowcoefficient: row.lossflowcoefficient.clone(),
            emsmeasurand: row.emsmeasurand.clone(),
            authorisedby: row.authorisedby.clone(),
            authoriseddate: row.authoriseddate.clone(),
            dynamicrhs: row.dynamicrhs.clone(),
            importlimit: row.importlimit.clone(),
            exportlimit: row.exportlimit.clone(),
            outagederationfactor: row.outagederationfactor.clone(),
            nonphysicallossfactor: row.nonphysicallossfactor.clone(),
            overloadfactor60sec: row.overloadfactor60sec.clone(),
            overloadfactor6sec: row.overloadfactor6sec.clone(),
            lastchanged: row.lastchanged.clone(),
            fcassupportunavailable: row.fcassupportunavailable.clone(),
            ictype: row.ictype.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketConfigInterconnectorconstraint1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub interconnectorid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigInterconnectorconstraint1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for MarketConfigInterconnectorconstraint1Row<'data> {
    type Row<'other> = MarketConfigInterconnectorconstraint1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid() == row.interconnectorid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for MarketConfigInterconnectorconstraint1Row<'data> {
    type PrimaryKey = MarketConfigInterconnectorconstraint1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid() == key.interconnectorid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for MarketConfigInterconnectorconstraint1PrimaryKey {
    type Row<'other> = MarketConfigInterconnectorconstraint1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid == row.interconnectorid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for MarketConfigInterconnectorconstraint1PrimaryKey {
    type PrimaryKey = MarketConfigInterconnectorconstraint1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid == key.interconnectorid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigInterconnectorconstraint1 {
    type Builder = MarketConfigInterconnectorconstraint1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "reserveoverallloadfactor",
                    arrow::datatypes::DataType::Decimal128(5, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "fromregionlossshare",
                    arrow::datatypes::DataType::Decimal128(5, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
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
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "maxmwin",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxmwout",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lossconstant",
                    arrow::datatypes::DataType::Decimal128(15, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lossflowcoefficient",
                    arrow::datatypes::DataType::Decimal128(27, 17),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "emsmeasurand",
                    arrow::datatypes::DataType::Utf8,
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
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dynamicrhs",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "importlimit",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "exportlimit",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "outagederationfactor",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "nonphysicallossfactor",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "overloadfactor60sec",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "overloadfactor6sec",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "fcassupportunavailable",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ictype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketConfigInterconnectorconstraint1Builder {
            reserveoverallloadfactor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 2)),
            fromregionlossshare_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 2)),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            maxmwin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            maxmwout_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lossconstant_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 6)),
            lossflowcoefficient_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 17)),
            emsmeasurand_array: arrow::array::builder::StringBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            dynamicrhs_array: arrow::array::builder::StringBuilder::new(),
            importlimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            exportlimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            outagederationfactor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            nonphysicallossfactor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            overloadfactor60sec_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            overloadfactor6sec_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            fcassupportunavailable_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            ictype_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .reserveoverallloadfactor_array
            .append_option({
                row.reserveoverallloadfactor
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .fromregionlossshare_array
            .append_option({
                row.fromregionlossshare
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder
            .maxmwin_array
            .append_option({
                row.maxmwin
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .maxmwout_array
            .append_option({
                row.maxmwout
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lossconstant_array
            .append_option({
                row.lossconstant
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lossflowcoefficient_array
            .append_option({
                row.lossflowcoefficient
                    .map(|mut val| {
                        val.rescale(17);
                        val.mantissa()
                    })
            });
        builder.emsmeasurand_array.append_option(row.emsmeasurand());
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder.dynamicrhs_array.append_option(row.dynamicrhs());
        builder
            .importlimit_array
            .append_option({
                row.importlimit
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .exportlimit_array
            .append_option({
                row.exportlimit
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .outagederationfactor_array
            .append_option({
                row.outagederationfactor
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .nonphysicallossfactor_array
            .append_option({
                row.nonphysicallossfactor
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .overloadfactor60sec_array
            .append_option({
                row.overloadfactor60sec
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .overloadfactor6sec_array
            .append_option({
                row.overloadfactor6sec
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .fcassupportunavailable_array
            .append_option({
                row.fcassupportunavailable
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.ictype_array.append_option(row.ictype());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(
                        builder.reserveoverallloadfactor_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregionlossshare_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxmwin_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxmwout_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lossconstant_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lossflowcoefficient_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.emsmeasurand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dynamicrhs_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.importlimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exportlimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.outagederationfactor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.nonphysicallossfactor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.overloadfactor60sec_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.overloadfactor6sec_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fcassupportunavailable_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ictype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MarketConfigInterconnectorconstraint1Builder {
    reserveoverallloadfactor_array: arrow::array::builder::Decimal128Builder,
    fromregionlossshare_array: arrow::array::builder::Decimal128Builder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    maxmwin_array: arrow::array::builder::Decimal128Builder,
    maxmwout_array: arrow::array::builder::Decimal128Builder,
    lossconstant_array: arrow::array::builder::Decimal128Builder,
    lossflowcoefficient_array: arrow::array::builder::Decimal128Builder,
    emsmeasurand_array: arrow::array::builder::StringBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    dynamicrhs_array: arrow::array::builder::StringBuilder,
    importlimit_array: arrow::array::builder::Decimal128Builder,
    exportlimit_array: arrow::array::builder::Decimal128Builder,
    outagederationfactor_array: arrow::array::builder::Decimal128Builder,
    nonphysicallossfactor_array: arrow::array::builder::Decimal128Builder,
    overloadfactor60sec_array: arrow::array::builder::Decimal128Builder,
    overloadfactor6sec_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    fcassupportunavailable_array: arrow::array::builder::Decimal128Builder,
    ictype_array: arrow::array::builder::StringBuilder,
}
pub struct MarketConfigIntraregionalloc1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MarketConfigIntraregionalloc1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MarketConfigIntraregionalloc1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MarketConfigIntraregionalloc1Mapping([usize; 6]);
/// # Summary
///
/// ## INTRAREGIONALLOC
///
/// INTRAREGIONALLOC shows allocations of intra-regional residues to participants.
///
/// * Data Set Name: Market Config
/// * File Name: Intraregionalloc
/// * Data Version: 1
///
/// # Description
/// INTRAREGIONALLOC data is confidential to the relevant participant.SourceThe data in INTRAREGIONALLOC changes infrequently.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PARTICIPANTID
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct MarketConfigIntraregionalloc1Row<'data> {
    /// Effective Date of Allocation Details
    pub effectivedate: chrono::NaiveDateTime,
    /// Version No in respect to effective date
    pub versionno: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Allocation Percent / 100
    pub allocation: Option<rust_decimal::Decimal>,
    /// Last changed date/time
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MarketConfigIntraregionalloc1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
}
impl mmsdm_core::GetTable for MarketConfigIntraregionalloc1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MARKET_CONFIG";
    const TABLE_NAME: &'static str = "INTRAREGIONALLOC";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketConfigIntraregionalloc1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSIONNO",
        "REGIONID",
        "PARTICIPANTID",
        "ALLOCATION",
        "LASTCHANGED",
    ];
    type Row<'row> = MarketConfigIntraregionalloc1Row<'row>;
    type FieldMapping = MarketConfigIntraregionalloc1Mapping;
    type PrimaryKey = MarketConfigIntraregionalloc1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketConfigIntraregionalloc1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            participantid: row.get_range("participantid", field_mapping.0[3])?,
            allocation: row
                .get_opt_custom_parsed_at_idx(
                    "allocation",
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
        if !row.is_heading() {
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
        Ok(MarketConfigIntraregionalloc1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MarketConfigIntraregionalloc1PrimaryKey {
        MarketConfigIntraregionalloc1PrimaryKey {
            effectivedate: row.effectivedate,
            participantid: row.participantid().to_string(),
            regionid: row.regionid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("market_config_intraregionalloc_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketConfigIntraregionalloc1Row {
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            regionid: row.regionid.clone(),
            participantid: row.participantid.clone(),
            allocation: row.allocation.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketConfigIntraregionalloc1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: alloc::string::String,
    pub regionid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigIntraregionalloc1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigIntraregionalloc1Row<'data> {
    type Row<'other> = MarketConfigIntraregionalloc1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid() == row.participantid()
            && self.regionid() == row.regionid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for MarketConfigIntraregionalloc1Row<'data> {
    type PrimaryKey = MarketConfigIntraregionalloc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid() == key.participantid
            && self.regionid() == key.regionid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigIntraregionalloc1PrimaryKey {
    type Row<'other> = MarketConfigIntraregionalloc1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid()
            && self.regionid == row.regionid() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigIntraregionalloc1PrimaryKey {
    type PrimaryKey = MarketConfigIntraregionalloc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigIntraregionalloc1 {
    type Builder = MarketConfigIntraregionalloc1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(5, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "allocation",
                    arrow::datatypes::DataType::Decimal128(12, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketConfigIntraregionalloc1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(5, 0)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            allocation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.regionid_array.append_value(row.regionid());
        builder.participantid_array.append_value(row.participantid());
        builder
            .allocation_array
            .append_option({
                row.allocation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.allocation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MarketConfigIntraregionalloc1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    allocation_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MarketConfigLossfactormodel1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MarketConfigLossfactormodel1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MarketConfigLossfactormodel1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MarketConfigLossfactormodel1Mapping([usize; 6]);
/// # Summary
///
/// ## LOSSFACTORMODEL
///
/// LOSSFACTORMODEL sets out the demand coefficients for each interconnector, used by LP Solver modelling of interconnector flows.
///
/// * Data Set Name: Market Config
/// * File Name: Lossfactormodel
/// * Data Version: 1
///
/// # Description
/// LOSSFACTORMODEL is public data, so is available to all participants.SourceLOSSFACTORMODEL only changes annually, when there is a change in the interconnector.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * INTERCONNECTORID
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct MarketConfigLossfactormodel1Row<'data> {
    /// Calendar date data set is effective
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number within effective date of the status proposed
    pub versionno: rust_decimal::Decimal,
    /// The unique identifier for the interconnector.
    pub interconnectorid: core::ops::Range<usize>,
    /// The unique region identifier for a connection point of the interconnector
    pub regionid: core::ops::Range<usize>,
    /// The coefficient applied to the region demand in the calculation of the interconnector loss factor
    pub demandcoefficient: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MarketConfigLossfactormodel1Row<'data> {
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for MarketConfigLossfactormodel1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MARKET_CONFIG";
    const TABLE_NAME: &'static str = "LOSSFACTORMODEL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketConfigLossfactormodel1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSIONNO",
        "INTERCONNECTORID",
        "REGIONID",
        "DEMANDCOEFFICIENT",
        "LASTCHANGED",
    ];
    type Row<'row> = MarketConfigLossfactormodel1Row<'row>;
    type FieldMapping = MarketConfigLossfactormodel1Mapping;
    type PrimaryKey = MarketConfigLossfactormodel1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketConfigLossfactormodel1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[2])?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            demandcoefficient: row
                .get_opt_custom_parsed_at_idx(
                    "demandcoefficient",
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
        if !row.is_heading() {
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
        Ok(MarketConfigLossfactormodel1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MarketConfigLossfactormodel1PrimaryKey {
        MarketConfigLossfactormodel1PrimaryKey {
            effectivedate: row.effectivedate,
            interconnectorid: row.interconnectorid().to_string(),
            regionid: row.regionid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("market_config_lossfactormodel_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketConfigLossfactormodel1Row {
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            interconnectorid: row.interconnectorid.clone(),
            regionid: row.regionid.clone(),
            demandcoefficient: row.demandcoefficient.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketConfigLossfactormodel1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub interconnectorid: alloc::string::String,
    pub regionid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigLossfactormodel1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigLossfactormodel1Row<'data> {
    type Row<'other> = MarketConfigLossfactormodel1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid() == row.interconnectorid()
            && self.regionid() == row.regionid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for MarketConfigLossfactormodel1Row<'data> {
    type PrimaryKey = MarketConfigLossfactormodel1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid() == key.interconnectorid
            && self.regionid() == key.regionid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigLossfactormodel1PrimaryKey {
    type Row<'other> = MarketConfigLossfactormodel1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid == row.interconnectorid()
            && self.regionid == row.regionid() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigLossfactormodel1PrimaryKey {
    type PrimaryKey = MarketConfigLossfactormodel1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid == key.interconnectorid
            && self.regionid == key.regionid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigLossfactormodel1 {
    type Builder = MarketConfigLossfactormodel1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
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
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "demandcoefficient",
                    arrow::datatypes::DataType::Decimal128(27, 17),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketConfigLossfactormodel1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            demandcoefficient_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 17)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.regionid_array.append_value(row.regionid());
        builder
            .demandcoefficient_array
            .append_option({
                row.demandcoefficient
                    .map(|mut val| {
                        val.rescale(17);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demandcoefficient_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MarketConfigLossfactormodel1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    demandcoefficient_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MarketConfigLossmodel1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MarketConfigLossmodel1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MarketConfigLossmodel1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MarketConfigLossmodel1Mapping([usize; 8]);
/// # Summary
///
/// ## LOSSMODEL
///
/// LOSSMODEL sets out segment breakpoints in loss model for each interconnector, used by LP Solver modelling of interconnector flows.
///
/// * Data Set Name: Market Config
/// * File Name: Lossmodel
/// * Data Version: 1
///
/// # Description
/// LOSSMODEL data is public, so is available to all participants.SourceLOSSMODEL only changes annually, when there is a change in the interconnector.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * INTERCONNECTORID
/// * LOSSSEGMENT
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct MarketConfigLossmodel1Row<'data> {
    /// Calendar date data set is effective
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number within effective date
    pub versionno: rust_decimal::Decimal,
    /// Interconnector identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// Not used
    pub periodid: core::ops::Range<usize>,
    /// Segment Identifier (1 to 80 at present)
    pub losssegment: rust_decimal::Decimal,
    /// MW Value for segment
    pub mwbreakpoint: Option<rust_decimal::Decimal>,
    /// Not used
    pub lossfactor: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MarketConfigLossmodel1Row<'data> {
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn periodid(&self) -> Option<&str> {
        if self.periodid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.periodid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for MarketConfigLossmodel1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MARKET_CONFIG";
    const TABLE_NAME: &'static str = "LOSSMODEL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketConfigLossmodel1Mapping([
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
        "EFFECTIVEDATE",
        "VERSIONNO",
        "INTERCONNECTORID",
        "PERIODID",
        "LOSSSEGMENT",
        "MWBREAKPOINT",
        "LOSSFACTOR",
        "LASTCHANGED",
    ];
    type Row<'row> = MarketConfigLossmodel1Row<'row>;
    type FieldMapping = MarketConfigLossmodel1Mapping;
    type PrimaryKey = MarketConfigLossmodel1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketConfigLossmodel1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[2])?,
            periodid: row.get_opt_range("periodid", field_mapping.0[3])?,
            losssegment: row
                .get_custom_parsed_at_idx(
                    "losssegment",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mwbreakpoint: row
                .get_opt_custom_parsed_at_idx(
                    "mwbreakpoint",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lossfactor: row
                .get_opt_custom_parsed_at_idx(
                    "lossfactor",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
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
        if !row.is_heading() {
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
        Ok(MarketConfigLossmodel1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MarketConfigLossmodel1PrimaryKey {
        MarketConfigLossmodel1PrimaryKey {
            effectivedate: row.effectivedate,
            interconnectorid: row.interconnectorid().to_string(),
            losssegment: row.losssegment,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("market_config_lossmodel_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketConfigLossmodel1Row {
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            interconnectorid: row.interconnectorid.clone(),
            periodid: row.periodid.clone(),
            losssegment: row.losssegment.clone(),
            mwbreakpoint: row.mwbreakpoint.clone(),
            lossfactor: row.lossfactor.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketConfigLossmodel1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub interconnectorid: alloc::string::String,
    pub losssegment: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigLossmodel1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigLossmodel1Row<'data> {
    type Row<'other> = MarketConfigLossmodel1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid() == row.interconnectorid()
            && self.losssegment == row.losssegment && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MarketConfigLossmodel1Row<'data> {
    type PrimaryKey = MarketConfigLossmodel1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid() == key.interconnectorid
            && self.losssegment == key.losssegment && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigLossmodel1PrimaryKey {
    type Row<'other> = MarketConfigLossmodel1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid == row.interconnectorid()
            && self.losssegment == row.losssegment && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigLossmodel1PrimaryKey {
    type PrimaryKey = MarketConfigLossmodel1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid == key.interconnectorid
            && self.losssegment == key.losssegment && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigLossmodel1 {
    type Builder = MarketConfigLossmodel1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
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
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "losssegment",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "mwbreakpoint",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lossfactor",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketConfigLossmodel1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::StringBuilder::new(),
            losssegment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            mwbreakpoint_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            lossfactor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.periodid_array.append_option(row.periodid());
        builder
            .losssegment_array
            .append_value({
                let mut val = row.losssegment;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .mwbreakpoint_array
            .append_option({
                row.mwbreakpoint
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lossfactor_array
            .append_option({
                row.lossfactor
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.losssegment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwbreakpoint_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lossfactor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MarketConfigLossmodel1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::StringBuilder,
    losssegment_array: arrow::array::builder::Decimal128Builder,
    mwbreakpoint_array: arrow::array::builder::Decimal128Builder,
    lossfactor_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MarketConfigMarketPriceThresholds1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MarketConfigMarketPriceThresholds1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MarketConfigMarketPriceThresholds1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MarketConfigMarketPriceThresholds1Mapping([usize; 8]);
/// # Summary
///
/// ## MARKET_PRICE_THRESHOLDS
///
/// MARKET_PRICE_THRESHOLDS sets out the market cap , floor and administered price thresholds applying to the electricity market
///
/// * Data Set Name: Market Config
/// * File Name: Market Price Thresholds
/// * Data Version: 1
///
/// # Description
/// MARKET_PRICE_THRESHOLDS data is public, so is available to all participants.SourceMARKET_PRICE_THRESHOLDS only changes when a change is made to a market price threshold. This table changes infrequently.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct MarketConfigMarketPriceThresholds1Row<'data> {
    /// Calendar date that this record becomes effective
    pub effectivedate: chrono::NaiveDateTime,
    /// version no for the effective date
    pub versionno: rust_decimal::Decimal,
    /// value of lost load if total supply falls short of demand after load management then involuntary load
    pub voll: Option<rust_decimal::Decimal>,
    /// The floor price that the spot market price will not fall below.
    pub marketpricefloor: Option<rust_decimal::Decimal>,
    /// Threshold value beyond which Aggregate Prices per Region over 336 Trade Intervals (Energy), or 2016 Dispatch Intervals (FCAS), will result in an Administered Price declaration
    pub administered_price_threshold: Option<rust_decimal::Decimal>,
    /// date data authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// user authorising
    pub authorisedby: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MarketConfigMarketPriceThresholds1Row<'data> {
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
impl mmsdm_core::GetTable for MarketConfigMarketPriceThresholds1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MARKET_CONFIG";
    const TABLE_NAME: &'static str = "MARKET_PRICE_THRESHOLDS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketConfigMarketPriceThresholds1Mapping([
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
        "EFFECTIVEDATE",
        "VERSIONNO",
        "VOLL",
        "MARKETPRICEFLOOR",
        "ADMINISTERED_PRICE_THRESHOLD",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "LASTCHANGED",
    ];
    type Row<'row> = MarketConfigMarketPriceThresholds1Row<'row>;
    type FieldMapping = MarketConfigMarketPriceThresholds1Mapping;
    type PrimaryKey = MarketConfigMarketPriceThresholds1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketConfigMarketPriceThresholds1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            voll: row
                .get_opt_custom_parsed_at_idx(
                    "voll",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marketpricefloor: row
                .get_opt_custom_parsed_at_idx(
                    "marketpricefloor",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            administered_price_threshold: row
                .get_opt_custom_parsed_at_idx(
                    "administered_price_threshold",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[6])?,
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
        if !row.is_heading() {
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
        Ok(MarketConfigMarketPriceThresholds1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MarketConfigMarketPriceThresholds1PrimaryKey {
        MarketConfigMarketPriceThresholds1PrimaryKey {
            effectivedate: row.effectivedate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "market_config_market_price_thresholds_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketConfigMarketPriceThresholds1Row {
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            voll: row.voll.clone(),
            marketpricefloor: row.marketpricefloor.clone(),
            administered_price_threshold: row.administered_price_threshold.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketConfigMarketPriceThresholds1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigMarketPriceThresholds1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigMarketPriceThresholds1Row<'data> {
    type Row<'other> = MarketConfigMarketPriceThresholds1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for MarketConfigMarketPriceThresholds1Row<'data> {
    type PrimaryKey = MarketConfigMarketPriceThresholds1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigMarketPriceThresholds1PrimaryKey {
    type Row<'other> = MarketConfigMarketPriceThresholds1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigMarketPriceThresholds1PrimaryKey {
    type PrimaryKey = MarketConfigMarketPriceThresholds1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigMarketPriceThresholds1 {
    type Builder = MarketConfigMarketPriceThresholds1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "voll",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "marketpricefloor",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "administered_price_threshold",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authoriseddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
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
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketConfigMarketPriceThresholds1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            voll_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            marketpricefloor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            administered_price_threshold_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .voll_array
            .append_option({
                row.voll
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .marketpricefloor_array
            .append_option({
                row.marketpricefloor
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .administered_price_threshold_array
            .append_option({
                row.administered_price_threshold
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.voll_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marketpricefloor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.administered_price_threshold_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
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
pub struct MarketConfigMarketPriceThresholds1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    voll_array: arrow::array::builder::Decimal128Builder,
    marketpricefloor_array: arrow::array::builder::Decimal128Builder,
    administered_price_threshold_array: arrow::array::builder::Decimal128Builder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MarketConfigRegion1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MarketConfigRegion1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MarketConfigRegion1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MarketConfigRegion1Mapping([usize; 4]);
/// # Summary
///
/// ## REGION
///
/// REGION sets out valid region IDs.
///
/// * Data Set Name: Market Config
/// * File Name: Region
/// * Data Version: 1
///
/// # Description
/// REGION data is public, so is available to all participants.SourceREGION updates if a change is ever made to a region. This table is static data and is likely to change very infrequently.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * REGIONID
#[derive(Debug, PartialEq, Eq)]
pub struct MarketConfigRegion1Row<'data> {
    /// Differentiates this region from all other regions
    pub regionid: core::ops::Range<usize>,
    /// Full description of region
    pub description: core::ops::Range<usize>,
    /// Status of the region e.g. working, inactive, archive.
    pub regionstatus: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MarketConfigRegion1Row<'data> {
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
    pub fn regionstatus(&self) -> Option<&str> {
        if self.regionstatus.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.regionstatus.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for MarketConfigRegion1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MARKET_CONFIG";
    const TABLE_NAME: &'static str = "REGION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketConfigRegion1Mapping([
        4,
        5,
        6,
        7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "REGIONID",
        "DESCRIPTION",
        "REGIONSTATUS",
        "LASTCHANGED",
    ];
    type Row<'row> = MarketConfigRegion1Row<'row>;
    type FieldMapping = MarketConfigRegion1Mapping;
    type PrimaryKey = MarketConfigRegion1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketConfigRegion1Row {
            regionid: row.get_range("regionid", field_mapping.0[0])?,
            description: row.get_opt_range("description", field_mapping.0[1])?,
            regionstatus: row.get_opt_range("regionstatus", field_mapping.0[2])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !row.is_heading() {
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
        Ok(MarketConfigRegion1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MarketConfigRegion1PrimaryKey {
        MarketConfigRegion1PrimaryKey {
            regionid: row.regionid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("market_config_region_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketConfigRegion1Row {
            regionid: row.regionid.clone(),
            description: row.description.clone(),
            regionstatus: row.regionstatus.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketConfigRegion1PrimaryKey {
    pub regionid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for MarketConfigRegion1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigRegion1Row<'data> {
    type Row<'other> = MarketConfigRegion1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.regionid() == row.regionid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for MarketConfigRegion1Row<'data> {
    type PrimaryKey = MarketConfigRegion1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid() == key.regionid
    }
}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigRegion1PrimaryKey {
    type Row<'other> = MarketConfigRegion1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.regionid == row.regionid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigRegion1PrimaryKey {
    type PrimaryKey = MarketConfigRegion1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigRegion1 {
    type Builder = MarketConfigRegion1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "description",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regionstatus",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketConfigRegion1Builder {
            regionid_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            regionstatus_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.regionid_array.append_value(row.regionid());
        builder.description_array.append_option(row.description());
        builder.regionstatus_array.append_option(row.regionstatus());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionstatus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MarketConfigRegion1Builder {
    regionid_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    regionstatus_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MarketConfigRegionstandingdata1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MarketConfigRegionstandingdata1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MarketConfigRegionstandingdata1 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MarketConfigRegionstandingdata1Mapping([usize; 10]);
/// # Summary
///
/// ## REGIONSTANDINGDATA
///
/// REGIONSTANDINGDATA sets out standing region data including the region reference node.
///
/// * Data Set Name: Market Config
/// * File Name: Regionstandingdata
/// * Data Version: 1
///
/// # Description
/// REGIONSTANDINGDATA data is public, so is available to all participants.SourceREGIONSTANDINGDATA only changes when a change is made to a region. This table changes infrequently.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct MarketConfigRegionstandingdata1Row<'data> {
    /// Effective date of this record, only the latest date applies
    pub effectivedate: chrono::NaiveDateTime,
    /// Version No of the standing data that should be effective on this date
    pub versionno: rust_decimal::Decimal,
    /// Differentiates this region from all other regions
    pub regionid: core::ops::Range<usize>,
    /// the unique identifier of the participant with responsibility for the region.
    pub rsoid: core::ops::Range<usize>,
    /// unique id of a connection point, being the reference point for this region
    pub regionalreferencepointid: core::ops::Range<usize>,
    /// Period identifier of the peak trading period of this connection point
    pub peaktradingperiod: Option<rust_decimal::Decimal>,
    /// Date record authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorising record
    pub authorisedby: core::ops::Range<usize>,
    /// Scaling factor for regional FCAS requirement
    pub scalingfactor: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MarketConfigRegionstandingdata1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn rsoid(&self) -> Option<&str> {
        if self.rsoid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(self.backing_data.as_slice(), self.rsoid.clone()),
            )
        }
    }
    pub fn regionalreferencepointid(&self) -> Option<&str> {
        if self.regionalreferencepointid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.regionalreferencepointid.clone(),
                ),
            )
        }
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
impl mmsdm_core::GetTable for MarketConfigRegionstandingdata1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "MARKET_CONFIG";
    const TABLE_NAME: &'static str = "REGIONSTANDINGDATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketConfigRegionstandingdata1Mapping([
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
        "EFFECTIVEDATE",
        "VERSIONNO",
        "REGIONID",
        "RSOID",
        "REGIONALREFERENCEPOINTID",
        "PEAKTRADINGPERIOD",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "SCALINGFACTOR",
        "LASTCHANGED",
    ];
    type Row<'row> = MarketConfigRegionstandingdata1Row<'row>;
    type FieldMapping = MarketConfigRegionstandingdata1Mapping;
    type PrimaryKey = MarketConfigRegionstandingdata1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketConfigRegionstandingdata1Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            rsoid: row.get_opt_range("rsoid", field_mapping.0[3])?,
            regionalreferencepointid: row
                .get_opt_range("regionalreferencepointid", field_mapping.0[4])?,
            peaktradingperiod: row
                .get_opt_custom_parsed_at_idx(
                    "peaktradingperiod",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[7])?,
            scalingfactor: row
                .get_opt_custom_parsed_at_idx(
                    "scalingfactor",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
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
        if !row.is_heading() {
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
        Ok(MarketConfigRegionstandingdata1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> MarketConfigRegionstandingdata1PrimaryKey {
        MarketConfigRegionstandingdata1PrimaryKey {
            effectivedate: row.effectivedate,
            regionid: row.regionid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "market_config_regionstandingdata_v1_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketConfigRegionstandingdata1Row {
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            regionid: row.regionid.clone(),
            rsoid: row.rsoid.clone(),
            regionalreferencepointid: row.regionalreferencepointid.clone(),
            peaktradingperiod: row.peaktradingperiod.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            scalingfactor: row.scalingfactor.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketConfigRegionstandingdata1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigRegionstandingdata1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigRegionstandingdata1Row<'data> {
    type Row<'other> = MarketConfigRegionstandingdata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.regionid() == row.regionid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for MarketConfigRegionstandingdata1Row<'data> {
    type PrimaryKey = MarketConfigRegionstandingdata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.regionid() == key.regionid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for MarketConfigRegionstandingdata1PrimaryKey {
    type Row<'other> = MarketConfigRegionstandingdata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.regionid == row.regionid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigRegionstandingdata1PrimaryKey {
    type PrimaryKey = MarketConfigRegionstandingdata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigRegionstandingdata1 {
    type Builder = MarketConfigRegionstandingdata1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
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
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "rsoid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regionalreferencepointid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "peaktradingperiod",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authoriseddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
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
                    "scalingfactor",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketConfigRegionstandingdata1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            rsoid_array: arrow::array::builder::StringBuilder::new(),
            regionalreferencepointid_array: arrow::array::builder::StringBuilder::new(),
            peaktradingperiod_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            scalingfactor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.regionid_array.append_value(row.regionid());
        builder.rsoid_array.append_option(row.rsoid());
        builder
            .regionalreferencepointid_array
            .append_option(row.regionalreferencepointid());
        builder
            .peaktradingperiod_array
            .append_option({
                row.peaktradingperiod
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .scalingfactor_array
            .append_option({
                row.scalingfactor
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rsoid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.regionalreferencepointid_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.peaktradingperiod_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.scalingfactor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MarketConfigRegionstandingdata1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    rsoid_array: arrow::array::builder::StringBuilder,
    regionalreferencepointid_array: arrow::array::builder::StringBuilder,
    peaktradingperiod_array: arrow::array::builder::Decimal128Builder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    scalingfactor_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct MarketConfigTransmissionlossfactor2 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &MarketConfigTransmissionlossfactor2Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl MarketConfigTransmissionlossfactor2 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            extract_row_partition: alloc::boxed::Box::new(func),
            row_partition_key,
        }
    }
}
pub struct MarketConfigTransmissionlossfactor2Mapping([usize; 7]);
/// # Summary
///
/// ## TRANSMISSIONLOSSFACTOR
///
/// TRANSMISSIONLOSSFACTOR shows the Transmission Loss factors applied at each connection point.
///
/// * Data Set Name: Market Config
/// * File Name: Transmissionlossfactor
/// * Data Version: 2
///
/// # Description
/// TRANSMISSIONLOSSFACTOR is public data, and is available to all participants.SourceTRANSMISSIONLOSSFACTOR updates when new connection points are created or loss factors change.
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * CONNECTIONPOINTID
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct MarketConfigTransmissionlossfactor2Row<'data> {
    /// Used in Bidding, Dispatch and Settlements. For Bidding and Dispatch, where the DUID is a BDU with DISPATCHTYPE of BIDIRECTIONAL, the TLF for the load component of the BDU. For Settlements, where dual TLFs apply, the primary TLF is applied to all energy (load and generation) when the Net Energy Flow of the ConnectionPointID in the interval is negative (net load).
    pub transmissionlossfactor: rust_decimal::Decimal,
    /// Effective date of record
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of record for given effective date
    pub versionno: rust_decimal::Decimal,
    /// Connection Point ID
    pub connectionpointid: core::ops::Range<usize>,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Record creation timestamp
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Used in Bidding, Dispatch and Settlements, only populated where Dual TLFs apply. For Bidding and Dispatch, the TLF for the generation component of a BDU, when null the TRANSMISSIONLOSSFACTOR is used for both the load and generation components. For Settlements, the secondary TLF is applied to all energy (load and generation) when the Net Energy Flow of the ConnectionPointID in the interval is positive (net generation).
    pub secondary_tlf: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> MarketConfigTransmissionlossfactor2Row<'data> {
    pub fn connectionpointid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.connectionpointid.clone(),
        )
    }
    pub fn regionid(&self) -> Option<&str> {
        if self.regionid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.regionid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for MarketConfigTransmissionlossfactor2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "MARKET_CONFIG";
    const TABLE_NAME: &'static str = "TRANSMISSIONLOSSFACTOR";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = MarketConfigTransmissionlossfactor2Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "TRANSMISSIONLOSSFACTOR",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "CONNECTIONPOINTID",
        "REGIONID",
        "LASTCHANGED",
        "SECONDARY_TLF",
    ];
    type Row<'row> = MarketConfigTransmissionlossfactor2Row<'row>;
    type FieldMapping = MarketConfigTransmissionlossfactor2Mapping;
    type PrimaryKey = MarketConfigTransmissionlossfactor2PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(MarketConfigTransmissionlossfactor2Row {
            transmissionlossfactor: row
                .get_custom_parsed_at_idx(
                    "transmissionlossfactor",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
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
            connectionpointid: row.get_range("connectionpointid", field_mapping.0[3])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[4])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            secondary_tlf: row
                .get_opt_custom_parsed_at_idx(
                    "secondary_tlf",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !row.is_heading() {
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
        Ok(MarketConfigTransmissionlossfactor2Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(
        row: &Self::Row<'_>,
    ) -> MarketConfigTransmissionlossfactor2PrimaryKey {
        MarketConfigTransmissionlossfactor2PrimaryKey {
            connectionpointid: row.connectionpointid().to_string(),
            effectivedate: row.effectivedate,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "market_config_transmissionlossfactor_v2_{}", self.partition_value(row)
        )
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        MarketConfigTransmissionlossfactor2Row {
            transmissionlossfactor: row.transmissionlossfactor.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            connectionpointid: row.connectionpointid.clone(),
            regionid: row.regionid.clone(),
            lastchanged: row.lastchanged.clone(),
            secondary_tlf: row.secondary_tlf.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketConfigTransmissionlossfactor2PrimaryKey {
    pub connectionpointid: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigTransmissionlossfactor2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow
for MarketConfigTransmissionlossfactor2Row<'data> {
    type Row<'other> = MarketConfigTransmissionlossfactor2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.connectionpointid() == row.connectionpointid()
            && self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for MarketConfigTransmissionlossfactor2Row<'data> {
    type PrimaryKey = MarketConfigTransmissionlossfactor2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid() == key.connectionpointid
            && self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow
for MarketConfigTransmissionlossfactor2PrimaryKey {
    type Row<'other> = MarketConfigTransmissionlossfactor2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.connectionpointid == row.connectionpointid()
            && self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for MarketConfigTransmissionlossfactor2PrimaryKey {
    type PrimaryKey = MarketConfigTransmissionlossfactor2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid == key.connectionpointid
            && self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigTransmissionlossfactor2 {
    type Builder = MarketConfigTransmissionlossfactor2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "transmissionlossfactor",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "secondary_tlf",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        MarketConfigTransmissionlossfactor2Builder {
            transmissionlossfactor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            secondary_tlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .transmissionlossfactor_array
            .append_value({
                let mut val = row.transmissionlossfactor;
                val.rescale(5);
                val.mantissa()
            });
        builder
            .effectivedate_array
            .append_value(row.effectivedate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.connectionpointid_array.append_value(row.connectionpointid());
        builder.regionid_array.append_option(row.regionid());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .secondary_tlf_array
            .append_option({
                row.secondary_tlf
                    .map(|mut val| {
                        val.rescale(8);
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
                    alloc::sync::Arc::new(builder.transmissionlossfactor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.secondary_tlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct MarketConfigTransmissionlossfactor2Builder {
    transmissionlossfactor_array: arrow::array::builder::Decimal128Builder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    secondary_tlf_array: arrow::array::builder::Decimal128Builder,
}
