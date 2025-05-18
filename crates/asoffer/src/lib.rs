#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct AsofferOfferagcdata1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &AsofferOfferagcdata1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl AsofferOfferagcdata1 {
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
pub struct AsofferOfferagcdata1Mapping([usize; 13]);
/// # Summary
///
/// ## OFFERAGCDATA
///
/// OFFERAGCDATA shows availability reoffers of Automatic Generation Control.
///
/// * Data Set Name: Asoffer
/// * File Name: Offeragcdata
/// * Data Version: 1
///
/// # Description
/// OFFERAGCDATA data is confidential to the relevant participant.SourceOFFERAGCDATA updates as reoffers submitted.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * EFFECTIVEDATE
/// * PERIODID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct AsofferOfferagcdata1Row<'data> {
    /// Contract Identifier
    pub contractid: core::ops::Range<usize>,
    /// Market date of offer
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of record
    pub versionno: rust_decimal::Decimal,
    /// Availability flag (0 or 1)
    pub availability: Option<rust_decimal::Decimal>,
    /// Upper control limit. This is used by SPD.
    pub upperlimit: Option<rust_decimal::Decimal>,
    /// Lower control limit MW. This is used by SPD.
    pub lowerlimit: Option<rust_decimal::Decimal>,
    /// Authorised date
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorised by
    pub authorisedby: core::ops::Range<usize>,
    /// Name of reoffer file
    pub filename: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Market day trading interval number
    pub periodid: rust_decimal::Decimal,
    /// AGC Ramp Rate Up. This is used by SPD.
    pub agcup: Option<rust_decimal::Decimal>,
    /// AGC Ramp Rate Down. This is used by SPD.
    pub agcdown: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> AsofferOfferagcdata1Row<'data> {
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
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
    pub fn filename(&self) -> Option<&str> {
        if self.filename.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.filename.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for AsofferOfferagcdata1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "ASOFFER";
    const TABLE_NAME: &'static str = "OFFERAGCDATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = AsofferOfferagcdata1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CONTRACTID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "AVAILABILITY",
        "UPPERLIMIT",
        "LOWERLIMIT",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "FILENAME",
        "LASTCHANGED",
        "PERIODID",
        "AGCUP",
        "AGCDOWN",
    ];
    type Row<'row> = AsofferOfferagcdata1Row<'row>;
    type FieldMapping = AsofferOfferagcdata1Mapping;
    type PrimaryKey = AsofferOfferagcdata1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(AsofferOfferagcdata1Row {
            contractid: row.get_range("contractid", field_mapping.0[0])?,
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
            availability: row
                .get_opt_custom_parsed_at_idx(
                    "availability",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            upperlimit: row
                .get_opt_custom_parsed_at_idx(
                    "upperlimit",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerlimit: row
                .get_opt_custom_parsed_at_idx(
                    "lowerlimit",
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
            filename: row.get_opt_range("filename", field_mapping.0[8])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[9],
                    mmsdm_core::mms_datetime::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            agcup: row
                .get_opt_custom_parsed_at_idx(
                    "agcup",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            agcdown: row
                .get_opt_custom_parsed_at_idx(
                    "agcdown",
                    field_mapping.0[12],
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
        Ok(AsofferOfferagcdata1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> AsofferOfferagcdata1PrimaryKey {
        AsofferOfferagcdata1PrimaryKey {
            contractid: row.contractid().to_string(),
            effectivedate: row.effectivedate,
            periodid: row.periodid,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("asoffer_offeragcdata_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        AsofferOfferagcdata1Row {
            contractid: row.contractid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            availability: row.availability.clone(),
            upperlimit: row.upperlimit.clone(),
            lowerlimit: row.lowerlimit.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            filename: row.filename.clone(),
            lastchanged: row.lastchanged.clone(),
            periodid: row.periodid.clone(),
            agcup: row.agcup.clone(),
            agcdown: row.agcdown.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AsofferOfferagcdata1PrimaryKey {
    pub contractid: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for AsofferOfferagcdata1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for AsofferOfferagcdata1Row<'data> {
    type Row<'other> = AsofferOfferagcdata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid() && self.effectivedate == row.effectivedate
            && self.periodid == row.periodid && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for AsofferOfferagcdata1Row<'data> {
    type PrimaryKey = AsofferOfferagcdata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.effectivedate == key.effectivedate
            && self.periodid == key.periodid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for AsofferOfferagcdata1PrimaryKey {
    type Row<'other> = AsofferOfferagcdata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.effectivedate == row.effectivedate
            && self.periodid == row.periodid && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for AsofferOfferagcdata1PrimaryKey {
    type PrimaryKey = AsofferOfferagcdata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.effectivedate == key.effectivedate
            && self.periodid == key.periodid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for AsofferOfferagcdata1 {
    type Builder = AsofferOfferagcdata1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractid",
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
                    "availability",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "upperlimit",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerlimit",
                    arrow::datatypes::DataType::Decimal128(4, 0),
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
                    "filename",
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
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "agcup",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "agcdown",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        AsofferOfferagcdata1Builder {
            contractid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            upperlimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            lowerlimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            filename_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            agcup_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            agcdown_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.contractid_array.append_value(row.contractid());
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
            .availability_array
            .append_option({
                row.availability
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .upperlimit_array
            .append_option({
                row.upperlimit
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lowerlimit_array
            .append_option({
                row.lowerlimit
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
        builder.filename_array.append_option(row.filename());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .agcup_array
            .append_option({
                row.agcup
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .agcdown_array
            .append_option({
                row.agcdown
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
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.availability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.upperlimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerlimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.filename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.agcup_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.agcdown_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct AsofferOfferagcdata1Builder {
    contractid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    availability_array: arrow::array::builder::Decimal128Builder,
    upperlimit_array: arrow::array::builder::Decimal128Builder,
    lowerlimit_array: arrow::array::builder::Decimal128Builder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    filename_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    agcup_array: arrow::array::builder::Decimal128Builder,
    agcdown_array: arrow::array::builder::Decimal128Builder,
}
pub struct AsofferOfferastrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &AsofferOfferastrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl AsofferOfferastrk1 {
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
pub struct AsofferOfferastrk1Mapping([usize; 5]);
/// # Summary
///
/// ## OFFERASTRK
///
/// OFFERASTRK tracks successfully acknowledged ancillary service reoffers.
///
/// * Data Set Name: Asoffer
/// * File Name: Offerastrk
/// * Data Version: 1
///
/// # Description
/// OFFERASTRK data is confidential to the relevant participant.SourceOFFERASTRK is updated as offers are successfully acknowledged.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct AsofferOfferastrk1Row<'data> {
    /// Market day starting at 4:00 am
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of the offer for that date
    pub versionno: rust_decimal::Decimal,
    /// Participant ID
    pub participantid: core::ops::Range<usize>,
    /// Submitted file name.
    pub filename: core::ops::Range<usize>,
    /// Last changed date and time.
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> AsofferOfferastrk1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn filename(&self) -> Option<&str> {
        if self.filename.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.filename.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for AsofferOfferastrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "ASOFFER";
    const TABLE_NAME: &'static str = "OFFERASTRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = AsofferOfferastrk1Mapping([
        4, 5, 6, 7, 8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSIONNO",
        "PARTICIPANTID",
        "FILENAME",
        "LASTCHANGED",
    ];
    type Row<'row> = AsofferOfferastrk1Row<'row>;
    type FieldMapping = AsofferOfferastrk1Mapping;
    type PrimaryKey = AsofferOfferastrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(AsofferOfferastrk1Row {
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
            participantid: row.get_range("participantid", field_mapping.0[2])?,
            filename: row.get_opt_range("filename", field_mapping.0[3])?,
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
        Ok(AsofferOfferastrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> AsofferOfferastrk1PrimaryKey {
        AsofferOfferastrk1PrimaryKey {
            effectivedate: row.effectivedate,
            participantid: row.participantid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("asoffer_offerastrk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        AsofferOfferastrk1Row {
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            participantid: row.participantid.clone(),
            filename: row.filename.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AsofferOfferastrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for AsofferOfferastrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for AsofferOfferastrk1Row<'data> {
    type Row<'other> = AsofferOfferastrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid() == row.participantid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for AsofferOfferastrk1Row<'data> {
    type PrimaryKey = AsofferOfferastrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid() == key.participantid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for AsofferOfferastrk1PrimaryKey {
    type Row<'other> = AsofferOfferastrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for AsofferOfferastrk1PrimaryKey {
    type PrimaryKey = AsofferOfferastrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for AsofferOfferastrk1 {
    type Builder = AsofferOfferastrk1Builder;
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
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "filename",
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
        AsofferOfferastrk1Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            filename_array: arrow::array::builder::StringBuilder::new(),
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
        builder.participantid_array.append_value(row.participantid());
        builder.filename_array.append_option(row.filename());
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
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.filename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct AsofferOfferastrk1Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    filename_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct AsofferOfferlsheddata1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &AsofferOfferlsheddata1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl AsofferOfferlsheddata1 {
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
pub struct AsofferOfferlsheddata1Mapping([usize; 9]);
/// # Summary
///
/// ## OFFERLSHEDDATA
///
/// OFFERLSHEDDATA shows reoffers of load shed including available load shed quantity. This Table may also be used for NSCAS and Type 1 transitional services procured by AEMO under the ISF framework during 2025 and prior to the implementation of all system changes. During this time descriptions in these tables may not be correct.
///
/// * Data Set Name: Asoffer
/// * File Name: Offerlsheddata
/// * Data Version: 1
///
/// # Description
/// OFFERLSHEDDATA data is confidential to the relevant participant.SourceOFFERLSHEDDATA updates as reoffers process.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * EFFECTIVEDATE
/// * PERIODID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct AsofferOfferlsheddata1Row<'data> {
    /// Contract identifier
    pub contractid: core::ops::Range<usize>,
    /// Market date of reoffer
    pub effectivedate: chrono::NaiveDateTime,
    /// Version No of reoffer
    pub versionno: rust_decimal::Decimal,
    /// Available load
    pub availableload: Option<rust_decimal::Decimal>,
    /// Authorised date
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorised by
    pub authorisedby: core::ops::Range<usize>,
    /// Name of reoffer file
    pub filename: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Market day trading interval number
    pub periodid: rust_decimal::Decimal,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> AsofferOfferlsheddata1Row<'data> {
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
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
    pub fn filename(&self) -> Option<&str> {
        if self.filename.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.filename.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for AsofferOfferlsheddata1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "ASOFFER";
    const TABLE_NAME: &'static str = "OFFERLSHEDDATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = AsofferOfferlsheddata1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CONTRACTID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "AVAILABLELOAD",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "FILENAME",
        "LASTCHANGED",
        "PERIODID",
    ];
    type Row<'row> = AsofferOfferlsheddata1Row<'row>;
    type FieldMapping = AsofferOfferlsheddata1Mapping;
    type PrimaryKey = AsofferOfferlsheddata1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(AsofferOfferlsheddata1Row {
            contractid: row.get_range("contractid", field_mapping.0[0])?,
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
            availableload: row
                .get_opt_custom_parsed_at_idx(
                    "availableload",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[5])?,
            filename: row.get_opt_range("filename", field_mapping.0[6])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[8],
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
        Ok(AsofferOfferlsheddata1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> AsofferOfferlsheddata1PrimaryKey {
        AsofferOfferlsheddata1PrimaryKey {
            contractid: row.contractid().to_string(),
            effectivedate: row.effectivedate,
            periodid: row.periodid,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("asoffer_offerlsheddata_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        AsofferOfferlsheddata1Row {
            contractid: row.contractid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            availableload: row.availableload.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            filename: row.filename.clone(),
            lastchanged: row.lastchanged.clone(),
            periodid: row.periodid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AsofferOfferlsheddata1PrimaryKey {
    pub contractid: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for AsofferOfferlsheddata1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for AsofferOfferlsheddata1Row<'data> {
    type Row<'other> = AsofferOfferlsheddata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid() && self.effectivedate == row.effectivedate
            && self.periodid == row.periodid && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for AsofferOfferlsheddata1Row<'data> {
    type PrimaryKey = AsofferOfferlsheddata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.effectivedate == key.effectivedate
            && self.periodid == key.periodid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for AsofferOfferlsheddata1PrimaryKey {
    type Row<'other> = AsofferOfferlsheddata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.effectivedate == row.effectivedate
            && self.periodid == row.periodid && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for AsofferOfferlsheddata1PrimaryKey {
    type PrimaryKey = AsofferOfferlsheddata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.effectivedate == key.effectivedate
            && self.periodid == key.periodid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for AsofferOfferlsheddata1 {
    type Builder = AsofferOfferlsheddata1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractid",
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
                    "availableload",
                    arrow::datatypes::DataType::Decimal128(4, 0),
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
                    "filename",
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
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        AsofferOfferlsheddata1Builder {
            contractid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            availableload_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            filename_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.contractid_array.append_value(row.contractid());
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
            .availableload_array
            .append_option({
                row.availableload
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
        builder.filename_array.append_option(row.filename());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.availableload_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.filename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct AsofferOfferlsheddata1Builder {
    contractid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    availableload_array: arrow::array::builder::Decimal128Builder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    filename_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
}
pub struct AsofferOfferrestartdata1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &AsofferOfferrestartdata1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl AsofferOfferrestartdata1 {
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
pub struct AsofferOfferrestartdata1Mapping([usize; 9]);
/// # Summary
///
/// ## OFFERRESTARTDATA
///
/// OFFERRESTARTDATA sets out reoffers of system restart availability.
///
/// * Data Set Name: Asoffer
/// * File Name: Offerrestartdata
/// * Data Version: 1
///
/// # Description
/// OFFERRESTARTDATA data is confidential to the relevant participant.SourceOFFERRESTARTDATA updates as reoffers process.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * OFFERDATE
/// * PERIODID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct AsofferOfferrestartdata1Row<'data> {
    /// Contract identifier
    pub contractid: core::ops::Range<usize>,
    /// Effective date of contract
    pub offerdate: chrono::NaiveDateTime,
    /// Version No of contract
    pub versionno: rust_decimal::Decimal,
    /// Available load
    pub availability: core::ops::Range<usize>,
    /// Authorised date
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorised by
    pub authorisedby: core::ops::Range<usize>,
    /// Name of reoffer file
    pub filename: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Market day trading interval number
    pub periodid: rust_decimal::Decimal,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> AsofferOfferrestartdata1Row<'data> {
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
    pub fn availability(&self) -> Option<&str> {
        if self.availability.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.availability.clone(),
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
    pub fn filename(&self) -> Option<&str> {
        if self.filename.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.filename.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for AsofferOfferrestartdata1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "ASOFFER";
    const TABLE_NAME: &'static str = "OFFERRESTARTDATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = AsofferOfferrestartdata1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CONTRACTID",
        "OFFERDATE",
        "VERSIONNO",
        "AVAILABILITY",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "FILENAME",
        "LASTCHANGED",
        "PERIODID",
    ];
    type Row<'row> = AsofferOfferrestartdata1Row<'row>;
    type FieldMapping = AsofferOfferrestartdata1Mapping;
    type PrimaryKey = AsofferOfferrestartdata1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(AsofferOfferrestartdata1Row {
            contractid: row.get_range("contractid", field_mapping.0[0])?,
            offerdate: row
                .get_custom_parsed_at_idx(
                    "offerdate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            availability: row.get_opt_range("availability", field_mapping.0[3])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[5])?,
            filename: row.get_opt_range("filename", field_mapping.0[6])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[8],
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
        Ok(AsofferOfferrestartdata1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> AsofferOfferrestartdata1PrimaryKey {
        AsofferOfferrestartdata1PrimaryKey {
            contractid: row.contractid().to_string(),
            offerdate: row.offerdate,
            periodid: row.periodid,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("asoffer_offerrestartdata_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        AsofferOfferrestartdata1Row {
            contractid: row.contractid.clone(),
            offerdate: row.offerdate.clone(),
            versionno: row.versionno.clone(),
            availability: row.availability.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            filename: row.filename.clone(),
            lastchanged: row.lastchanged.clone(),
            periodid: row.periodid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AsofferOfferrestartdata1PrimaryKey {
    pub contractid: alloc::string::String,
    pub offerdate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for AsofferOfferrestartdata1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for AsofferOfferrestartdata1Row<'data> {
    type Row<'other> = AsofferOfferrestartdata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid() && self.offerdate == row.offerdate
            && self.periodid == row.periodid && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for AsofferOfferrestartdata1Row<'data> {
    type PrimaryKey = AsofferOfferrestartdata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.offerdate == key.offerdate
            && self.periodid == key.periodid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for AsofferOfferrestartdata1PrimaryKey {
    type Row<'other> = AsofferOfferrestartdata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.offerdate == row.offerdate
            && self.periodid == row.periodid && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for AsofferOfferrestartdata1PrimaryKey {
    type PrimaryKey = AsofferOfferrestartdata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.offerdate == key.offerdate
            && self.periodid == key.periodid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for AsofferOfferrestartdata1 {
    type Builder = AsofferOfferrestartdata1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "offerdate",
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
                    "availability",
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
                    "authorisedby",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "filename",
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
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        AsofferOfferrestartdata1Builder {
            contractid_array: arrow::array::builder::StringBuilder::new(),
            offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            availability_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            filename_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.contractid_array.append_value(row.contractid());
        builder.offerdate_array.append_value(row.offerdate.and_utc().timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.availability_array.append_option(row.availability());
        builder
            .authoriseddate_array
            .append_option(
                row.authoriseddate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder.authorisedby_array.append_option(row.authorisedby());
        builder.filename_array.append_option(row.filename());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.availability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.filename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct AsofferOfferrestartdata1Builder {
    contractid_array: arrow::array::builder::StringBuilder,
    offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    availability_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    filename_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
}
pub struct AsofferOfferrpowerdata1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &AsofferOfferrpowerdata1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl AsofferOfferrpowerdata1 {
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
pub struct AsofferOfferrpowerdata1Mapping([usize; 11]);
/// # Summary
///
/// ## OFFERRPOWERDATA
///
/// OFFERRPOWERDATA shows reoffers of reactive power capability and settlement measurements. This Table may also be used for NSCAS and Type 1 transitional services procured by AEMO under the ISF framework during 2025 and prior to the implementation of all system changes. During this time descriptions in these tables may not be correct.
///
/// * Data Set Name: Asoffer
/// * File Name: Offerrpowerdata
/// * Data Version: 1
///
/// # Description
/// OFFERRPOWERDATA data is confidential to the relevant participant.SourceOFFERRPOWERDATA updates as reoffers process.
///
/// # Notes
/// * (Visibility)  Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * EFFECTIVEDATE
/// * PERIODID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct AsofferOfferrpowerdata1Row<'data> {
    /// Contract Version No.
    pub contractid: core::ops::Range<usize>,
    /// Contract Version No.
    pub effectivedate: chrono::NaiveDateTime,
    /// Version No. of Re-Offer
    pub versionno: rust_decimal::Decimal,
    /// Market trading interval
    pub periodid: rust_decimal::Decimal,
    /// Availability of service
    pub availability: Option<rust_decimal::Decimal>,
    /// Reactive Power Absorption Capability (MVar)
    pub mta: Option<rust_decimal::Decimal>,
    /// Reactive Power Generation Capability (MVar)
    pub mtg: Option<rust_decimal::Decimal>,
    /// Date Contract was Authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User Name
    pub authorisedby: core::ops::Range<usize>,
    /// File name of Re-Offer file
    pub filename: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> AsofferOfferrpowerdata1Row<'data> {
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
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
    pub fn filename(&self) -> Option<&str> {
        if self.filename.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.filename.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for AsofferOfferrpowerdata1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "ASOFFER";
    const TABLE_NAME: &'static str = "OFFERRPOWERDATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = AsofferOfferrpowerdata1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CONTRACTID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "PERIODID",
        "AVAILABILITY",
        "MTA",
        "MTG",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "FILENAME",
        "LASTCHANGED",
    ];
    type Row<'row> = AsofferOfferrpowerdata1Row<'row>;
    type FieldMapping = AsofferOfferrpowerdata1Mapping;
    type PrimaryKey = AsofferOfferrpowerdata1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(AsofferOfferrpowerdata1Row {
            contractid: row.get_range("contractid", field_mapping.0[0])?,
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
            availability: row
                .get_opt_custom_parsed_at_idx(
                    "availability",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mta: row
                .get_opt_custom_parsed_at_idx(
                    "mta",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mtg: row
                .get_opt_custom_parsed_at_idx(
                    "mtg",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[8])?,
            filename: row.get_opt_range("filename", field_mapping.0[9])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[10],
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
        Ok(AsofferOfferrpowerdata1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> AsofferOfferrpowerdata1PrimaryKey {
        AsofferOfferrpowerdata1PrimaryKey {
            contractid: row.contractid().to_string(),
            effectivedate: row.effectivedate,
            periodid: row.periodid,
            versionno: row.versionno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("asoffer_offerrpowerdata_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        AsofferOfferrpowerdata1Row {
            contractid: row.contractid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            periodid: row.periodid.clone(),
            availability: row.availability.clone(),
            mta: row.mta.clone(),
            mtg: row.mtg.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            filename: row.filename.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AsofferOfferrpowerdata1PrimaryKey {
    pub contractid: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for AsofferOfferrpowerdata1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for AsofferOfferrpowerdata1Row<'data> {
    type Row<'other> = AsofferOfferrpowerdata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid() && self.effectivedate == row.effectivedate
            && self.periodid == row.periodid && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for AsofferOfferrpowerdata1Row<'data> {
    type PrimaryKey = AsofferOfferrpowerdata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.effectivedate == key.effectivedate
            && self.periodid == key.periodid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for AsofferOfferrpowerdata1PrimaryKey {
    type Row<'other> = AsofferOfferrpowerdata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.effectivedate == row.effectivedate
            && self.periodid == row.periodid && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for AsofferOfferrpowerdata1PrimaryKey {
    type PrimaryKey = AsofferOfferrpowerdata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.effectivedate == key.effectivedate
            && self.periodid == key.periodid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for AsofferOfferrpowerdata1 {
    type Builder = AsofferOfferrpowerdata1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "contractid",
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
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "availability",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mta",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mtg",
                    arrow::datatypes::DataType::Decimal128(6, 0),
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
                    "filename",
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
        AsofferOfferrpowerdata1Builder {
            contractid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            mta_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            mtg_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            filename_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.contractid_array.append_value(row.contractid());
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
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .availability_array
            .append_option({
                row.availability
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .mta_array
            .append_option({
                row.mta
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .mtg_array
            .append_option({
                row.mtg
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
        builder.filename_array.append_option(row.filename());
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
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.availability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mta_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mtg_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.filename_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct AsofferOfferrpowerdata1Builder {
    contractid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    availability_array: arrow::array::builder::Decimal128Builder,
    mta_array: arrow::array::builder::Decimal128Builder,
    mtg_array: arrow::array::builder::Decimal128Builder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    filename_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
