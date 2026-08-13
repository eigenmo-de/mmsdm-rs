#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct StpasaCasesolution3 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &StpasaCasesolution3Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaCasesolution3 {
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
pub struct StpasaCasesolution3Mapping([usize; 19]);
/// # Summary
///
/// ## STPASA_CASESOLUTION
///
/// STPASA_CASESOLUTION holds one record containing results pertaining to each entire solution
///
/// * Data Set Name: Stpasa
/// * File Name: Casesolution
/// * Data Version: 3
///
/// # Description
/// STPASA_CASESOLUTION is public data.SourceSTPASA_CASESOLUTION is updated each STPASA run (i.e. every 2 hours).VolumeRows per day: 12Mb per month: <1
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct StpasaCasesolution3Row<'data> {
    /// Unique Timestamp Identifier for this study
    pub run_datetime: chrono::NaiveDateTime,
    /// Version of the PASA solver used to solve this case
    pub pasaversion: core::ops::Range<usize>,
    /// Low Reserve Condition (LRC) flag for the case (1 - LRC in the case, 0 - No LRCs in the case) for capacity run
    pub reservecondition: Option<rust_decimal::Decimal>,
    /// Lack of Reserve Condition (LOR) flag for the case indicates the most severe condition in the case  (3 = LOR3, 2 = LOR2, 1 = LOR1, 0 = No LOR)
    pub lorcondition: Option<rust_decimal::Decimal>,
    /// Objective Function from the Capacity Adequacy run
    pub capacityobjfunction: Option<rust_decimal::Decimal>,
    /// Not populated as of 2005 End of Year Release; was the demand forecast used for capacity adequacy assessment. 0 if no assessment, 1 for 10%, 2 for 50%, 3 for 90%
    pub capacityoption: Option<rust_decimal::Decimal>,
    /// Not populated as of 2005 End of Year Release; was the demand forecast used for assessment of Maximum surplus Reserve. 0 if no assessment, 1 for 10%, 2 for 50%, 3 for 90%
    pub maxsurplusreserveoption: Option<rust_decimal::Decimal>,
    /// Not populated as of 2005 End of Year Release; was the demand forecast used for assessment of Maximum Spare Capacity. 0 if no assessment, 1 for 10%, 2 for 50%, 3 for 90%
    pub maxsparecapacityoption: Option<rust_decimal::Decimal>,
    /// The penalty for non-zero interconnector flow
    pub interconnectorflowpenalty: Option<rust_decimal::Decimal>,
    /// Date and time the record was created or modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Specifies the Probability of Exceedence (POE) demand forecast for Reliability LRC assessment (0 if no assessment, 10 for 10%, 50 for 50%, 90 for 90%)
    pub reliabilitylrcdemandoption: Option<rust_decimal::Decimal>,
    /// Specifies the Probability of Exceedence (POE) demand forecast for outage LRC assessment (0 if no assessment, 10 for 10%, 50 for 50%, 90 for 90%)
    pub outagelrcdemandoption: Option<rust_decimal::Decimal>,
    /// Specifies the Probability of Exceedence (POE) demand forecast for LOR assessment (0 if no assessment, 10 for 10%, 50 for 50%, 90 for 90%)
    pub lordemandoption: Option<rust_decimal::Decimal>,
    /// Generation Availability to be used in Reliability LRC run (either PASA or MARKET)
    pub reliabilitylrccapacityoption: core::ops::Range<usize>,
    /// Generation Availability to be used in Outage LRC run (either PASA or MARKET)
    pub outagelrccapacityoption: core::ops::Range<usize>,
    /// Generation Availability to be used in LOR run (either PASA or MARKET)
    pub lorcapacityoption: core::ops::Range<usize>,
    /// UIGF POE forecast availability used for this option
    pub loruigf_option: Option<rust_decimal::Decimal>,
    /// UIGF POE forecast availability used for this option
    pub reliability_lrcuigf_option: Option<rust_decimal::Decimal>,
    /// UIGF POE forecast availability used for this option
    pub outage_lrcuigf_option: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> StpasaCasesolution3Row<'data> {
    pub fn pasaversion(&self) -> Option<&str> {
        if self.pasaversion.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.pasaversion.clone(),
                ),
            )
        }
    }
    pub fn reliabilitylrccapacityoption(&self) -> Option<&str> {
        if self.reliabilitylrccapacityoption.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.reliabilitylrccapacityoption.clone(),
                ),
            )
        }
    }
    pub fn outagelrccapacityoption(&self) -> Option<&str> {
        if self.outagelrccapacityoption.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.outagelrccapacityoption.clone(),
                ),
            )
        }
    }
    pub fn lorcapacityoption(&self) -> Option<&str> {
        if self.lorcapacityoption.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.lorcapacityoption.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for StpasaCasesolution3 {
    const VERSION: i32 = 3;
    const DATA_SET_NAME: &'static str = "STPASA";
    const TABLE_NAME: &'static str = "CASESOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = StpasaCasesolution3Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "PASAVERSION",
        "RESERVECONDITION",
        "LORCONDITION",
        "CAPACITYOBJFUNCTION",
        "CAPACITYOPTION",
        "MAXSURPLUSRESERVEOPTION",
        "MAXSPARECAPACITYOPTION",
        "INTERCONNECTORFLOWPENALTY",
        "LASTCHANGED",
        "RELIABILITYLRCDEMANDOPTION",
        "OUTAGELRCDEMANDOPTION",
        "LORDEMANDOPTION",
        "RELIABILITYLRCCAPACITYOPTION",
        "OUTAGELRCCAPACITYOPTION",
        "LORCAPACITYOPTION",
        "LORUIGFOption",
        "ReliabilityLRCUIGFOption",
        "OutageLRCUIGFOption",
    ];
    type Row<'row> = StpasaCasesolution3Row<'row>;
    type FieldMapping = StpasaCasesolution3Mapping;
    type PrimaryKey = StpasaCasesolution3PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(StpasaCasesolution3Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            pasaversion: row.get_opt_range("pasaversion", field_mapping.0[1])?,
            reservecondition: row
                .get_opt_custom_parsed_at_idx(
                    "reservecondition",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lorcondition: row
                .get_opt_custom_parsed_at_idx(
                    "lorcondition",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            capacityobjfunction: row
                .get_opt_custom_parsed_at_idx(
                    "capacityobjfunction",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            capacityoption: row
                .get_opt_custom_parsed_at_idx(
                    "capacityoption",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxsurplusreserveoption: row
                .get_opt_custom_parsed_at_idx(
                    "maxsurplusreserveoption",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxsparecapacityoption: row
                .get_opt_custom_parsed_at_idx(
                    "maxsparecapacityoption",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interconnectorflowpenalty: row
                .get_opt_custom_parsed_at_idx(
                    "interconnectorflowpenalty",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[9],
                    mmsdm_core::mms_datetime::parse,
                )?,
            reliabilitylrcdemandoption: row
                .get_opt_custom_parsed_at_idx(
                    "reliabilitylrcdemandoption",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            outagelrcdemandoption: row
                .get_opt_custom_parsed_at_idx(
                    "outagelrcdemandoption",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lordemandoption: row
                .get_opt_custom_parsed_at_idx(
                    "lordemandoption",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            reliabilitylrccapacityoption: row
                .get_opt_range("reliabilitylrccapacityoption", field_mapping.0[13])?,
            outagelrccapacityoption: row
                .get_opt_range("outagelrccapacityoption", field_mapping.0[14])?,
            lorcapacityoption: row
                .get_opt_range("lorcapacityoption", field_mapping.0[15])?,
            loruigf_option: row
                .get_opt_custom_parsed_at_idx(
                    "loruigf_option",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            reliability_lrcuigf_option: row
                .get_opt_custom_parsed_at_idx(
                    "reliability_lrcuigf_option",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            outage_lrcuigf_option: row
                .get_opt_custom_parsed_at_idx(
                    "outage_lrcuigf_option",
                    field_mapping.0[18],
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
        Ok(StpasaCasesolution3Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> StpasaCasesolution3PrimaryKey {
        StpasaCasesolution3PrimaryKey {
            run_datetime: row.run_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("stpasa_casesolution_v3_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        StpasaCasesolution3Row {
            run_datetime: row.run_datetime.clone(),
            pasaversion: row.pasaversion.clone(),
            reservecondition: row.reservecondition.clone(),
            lorcondition: row.lorcondition.clone(),
            capacityobjfunction: row.capacityobjfunction.clone(),
            capacityoption: row.capacityoption.clone(),
            maxsurplusreserveoption: row.maxsurplusreserveoption.clone(),
            maxsparecapacityoption: row.maxsparecapacityoption.clone(),
            interconnectorflowpenalty: row.interconnectorflowpenalty.clone(),
            lastchanged: row.lastchanged.clone(),
            reliabilitylrcdemandoption: row.reliabilitylrcdemandoption.clone(),
            outagelrcdemandoption: row.outagelrcdemandoption.clone(),
            lordemandoption: row.lordemandoption.clone(),
            reliabilitylrccapacityoption: row.reliabilitylrccapacityoption.clone(),
            outagelrccapacityoption: row.outagelrccapacityoption.clone(),
            lorcapacityoption: row.lorcapacityoption.clone(),
            loruigf_option: row.loruigf_option.clone(),
            reliability_lrcuigf_option: row.reliability_lrcuigf_option.clone(),
            outage_lrcuigf_option: row.outage_lrcuigf_option.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StpasaCasesolution3PrimaryKey {
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for StpasaCasesolution3PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for StpasaCasesolution3Row<'data> {
    type Row<'other> = StpasaCasesolution3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for StpasaCasesolution3Row<'data> {
    type PrimaryKey = StpasaCasesolution3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for StpasaCasesolution3PrimaryKey {
    type Row<'other> = StpasaCasesolution3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for StpasaCasesolution3PrimaryKey {
    type PrimaryKey = StpasaCasesolution3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for StpasaCasesolution3 {
    type Builder = StpasaCasesolution3Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "pasaversion",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int16),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reservecondition",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lorcondition",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "capacityobjfunction",
                    arrow::datatypes::DataType::Decimal128(12, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "capacityoption",
                    arrow::datatypes::DataType::Decimal128(12, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxsurplusreserveoption",
                    arrow::datatypes::DataType::Decimal128(12, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxsparecapacityoption",
                    arrow::datatypes::DataType::Decimal128(12, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorflowpenalty",
                    arrow::datatypes::DataType::Decimal128(12, 3),
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
                    "reliabilitylrcdemandoption",
                    arrow::datatypes::DataType::Decimal128(12, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "outagelrcdemandoption",
                    arrow::datatypes::DataType::Decimal128(12, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lordemandoption",
                    arrow::datatypes::DataType::Decimal128(12, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reliabilitylrccapacityoption",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int16),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "outagelrccapacityoption",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int16),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lorcapacityoption",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int16),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "loruigf_option",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reliability_lrcuigf_option",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "outage_lrcuigf_option",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        StpasaCasesolution3Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            pasaversion_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int16Type,
            >::new(),
            reservecondition_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            lorcondition_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            capacityobjfunction_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            capacityoption_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            maxsurplusreserveoption_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            maxsparecapacityoption_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            interconnectorflowpenalty_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            reliabilitylrcdemandoption_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            outagelrcdemandoption_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            lordemandoption_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            reliabilitylrccapacityoption_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int16Type,
            >::new(),
            outagelrccapacityoption_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int16Type,
            >::new(),
            lorcapacityoption_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int16Type,
            >::new(),
            loruigf_option_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            reliability_lrcuigf_option_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            outage_lrcuigf_option_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.pasaversion_array.append_option(row.pasaversion());
        builder
            .reservecondition_array
            .append_option({
                row.reservecondition
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lorcondition_array
            .append_option({
                row.lorcondition
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .capacityobjfunction_array
            .append_option({
                row.capacityobjfunction
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .capacityoption_array
            .append_option({
                row.capacityoption
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .maxsurplusreserveoption_array
            .append_option({
                row.maxsurplusreserveoption
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .maxsparecapacityoption_array
            .append_option({
                row.maxsparecapacityoption
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .interconnectorflowpenalty_array
            .append_option({
                row.interconnectorflowpenalty
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .reliabilitylrcdemandoption_array
            .append_option({
                row.reliabilitylrcdemandoption
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .outagelrcdemandoption_array
            .append_option({
                row.outagelrcdemandoption
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .lordemandoption_array
            .append_option({
                row.lordemandoption
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .reliabilitylrccapacityoption_array
            .append_option(row.reliabilitylrccapacityoption());
        builder
            .outagelrccapacityoption_array
            .append_option(row.outagelrccapacityoption());
        builder.lorcapacityoption_array.append_option(row.lorcapacityoption());
        builder
            .loruigf_option_array
            .append_option({
                row.loruigf_option
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .reliability_lrcuigf_option_array
            .append_option({
                row.reliability_lrcuigf_option
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .outage_lrcuigf_option_array
            .append_option({
                row.outage_lrcuigf_option
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pasaversion_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reservecondition_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lorcondition_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.capacityobjfunction_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.capacityoption_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxsurplusreserveoption_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxsparecapacityoption_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.interconnectorflowpenalty_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.reliabilitylrcdemandoption_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.outagelrcdemandoption_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lordemandoption_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.reliabilitylrccapacityoption_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.outagelrccapacityoption_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lorcapacityoption_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.loruigf_option_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.reliability_lrcuigf_option_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.outage_lrcuigf_option_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct StpasaCasesolution3Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    pasaversion_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int16Type,
    >,
    reservecondition_array: arrow::array::builder::Decimal128Builder,
    lorcondition_array: arrow::array::builder::Decimal128Builder,
    capacityobjfunction_array: arrow::array::builder::Decimal128Builder,
    capacityoption_array: arrow::array::builder::Decimal128Builder,
    maxsurplusreserveoption_array: arrow::array::builder::Decimal128Builder,
    maxsparecapacityoption_array: arrow::array::builder::Decimal128Builder,
    interconnectorflowpenalty_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    reliabilitylrcdemandoption_array: arrow::array::builder::Decimal128Builder,
    outagelrcdemandoption_array: arrow::array::builder::Decimal128Builder,
    lordemandoption_array: arrow::array::builder::Decimal128Builder,
    reliabilitylrccapacityoption_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int16Type,
    >,
    outagelrccapacityoption_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int16Type,
    >,
    lorcapacityoption_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int16Type,
    >,
    loruigf_option_array: arrow::array::builder::Decimal128Builder,
    reliability_lrcuigf_option_array: arrow::array::builder::Decimal128Builder,
    outage_lrcuigf_option_array: arrow::array::builder::Decimal128Builder,
}
pub struct StpasaConstraintsolution3 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &StpasaConstraintsolution3Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaConstraintsolution3 {
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
pub struct StpasaConstraintsolution3Mapping([usize; 9]);
/// # Summary
///
/// ## STPASA_CONSTRAINTSOLUTION
///
/// STPASA_CONSTRAINTSOLUTION shows binding and violated constraint results from the capacity evaluation, including the RHS value.
///
/// * Data Set Name: Stpasa
/// * File Name: Constraintsolution
/// * Data Version: 3
///
/// # Description
/// STPASA_CONSTRAINTSOLUTION is public data.SourceSTPASA_CONSTRAINTSOLUTION is updated each STPASA run (i.e. every 2 hours).VolumeRows per day: 19000 (est.)Mb per month: 90
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
/// * RUNTYPE
/// * STUDYREGIONID
#[derive(Debug, PartialEq, Eq)]
pub struct StpasaConstraintsolution3Row<'data> {
    /// Unique Timestamp Identifier for this study
    pub run_datetime: chrono::NaiveDateTime,
    /// The unique identifier for the interval within this study
    pub interval_datetime: chrono::NaiveDateTime,
    /// Constraint identifier (synonymous with GenConID)
    pub constraintid: core::ops::Range<usize>,
    /// The RHS value in the capacity evaluation.
    pub capacityrhs: Option<rust_decimal::Decimal>,
    /// Capacity adequacy assessment marginal value, 0 if not binding
    pub capacitymarginalvalue: Option<rust_decimal::Decimal>,
    /// Capacity adequacy assessment violation degree for generic constraint; 0 if not violating
    pub capacityviolationdegree: Option<rust_decimal::Decimal>,
    /// Last changed date of this record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Type of run. Values are RELIABILITY_LRC, OUTAGE_LRC and LOR. Note that the STPASA RELIABILITY_LRC and OUTAGE_LRC Run Types are discontinued from 31 July 2025, with only the LOR Run Type reported.
    pub runtype: core::ops::Range<usize>,
    /// Primary Region for LP Solve (or MARKET if none).
    pub studyregionid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> StpasaConstraintsolution3Row<'data> {
    pub fn constraintid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.constraintid.clone())
    }
    pub fn runtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.runtype.clone())
    }
    pub fn studyregionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.studyregionid.clone())
    }
}
impl mmsdm_core::GetTable for StpasaConstraintsolution3 {
    const VERSION: i32 = 3;
    const DATA_SET_NAME: &'static str = "STPASA";
    const TABLE_NAME: &'static str = "CONSTRAINTSOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = StpasaConstraintsolution3Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "INTERVAL_DATETIME",
        "CONSTRAINTID",
        "CAPACITYRHS",
        "CAPACITYMARGINALVALUE",
        "CAPACITYVIOLATIONDEGREE",
        "LASTCHANGED",
        "RUNTYPE",
        "STUDYREGIONID",
    ];
    type Row<'row> = StpasaConstraintsolution3Row<'row>;
    type FieldMapping = StpasaConstraintsolution3Mapping;
    type PrimaryKey = StpasaConstraintsolution3PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(StpasaConstraintsolution3Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            constraintid: row.get_range("constraintid", field_mapping.0[2])?,
            capacityrhs: row
                .get_opt_custom_parsed_at_idx(
                    "capacityrhs",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            capacitymarginalvalue: row
                .get_opt_custom_parsed_at_idx(
                    "capacitymarginalvalue",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            capacityviolationdegree: row
                .get_opt_custom_parsed_at_idx(
                    "capacityviolationdegree",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runtype: row.get_range("runtype", field_mapping.0[7])?,
            studyregionid: row.get_range("studyregionid", field_mapping.0[8])?,
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
        Ok(StpasaConstraintsolution3Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> StpasaConstraintsolution3PrimaryKey {
        StpasaConstraintsolution3PrimaryKey {
            constraintid: row.constraintid().to_string(),
            interval_datetime: row.interval_datetime,
            run_datetime: row.run_datetime,
            runtype: row.runtype().to_string(),
            studyregionid: row.studyregionid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("stpasa_constraintsolution_v3_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        StpasaConstraintsolution3Row {
            run_datetime: row.run_datetime.clone(),
            interval_datetime: row.interval_datetime.clone(),
            constraintid: row.constraintid.clone(),
            capacityrhs: row.capacityrhs.clone(),
            capacitymarginalvalue: row.capacitymarginalvalue.clone(),
            capacityviolationdegree: row.capacityviolationdegree.clone(),
            lastchanged: row.lastchanged.clone(),
            runtype: row.runtype.clone(),
            studyregionid: row.studyregionid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StpasaConstraintsolution3PrimaryKey {
    pub constraintid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
    pub runtype: alloc::string::String,
    pub studyregionid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for StpasaConstraintsolution3PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for StpasaConstraintsolution3Row<'data> {
    type Row<'other> = StpasaConstraintsolution3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid() == row.constraintid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime && self.runtype() == row.runtype()
            && self.studyregionid() == row.studyregionid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for StpasaConstraintsolution3Row<'data> {
    type PrimaryKey = StpasaConstraintsolution3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid() == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime && self.runtype() == key.runtype
            && self.studyregionid() == key.studyregionid
    }
}
impl<'data> mmsdm_core::CompareWithRow for StpasaConstraintsolution3PrimaryKey {
    type Row<'other> = StpasaConstraintsolution3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid == row.constraintid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime && self.runtype == row.runtype()
            && self.studyregionid == row.studyregionid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for StpasaConstraintsolution3PrimaryKey {
    type PrimaryKey = StpasaConstraintsolution3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime && self.runtype == key.runtype
            && self.studyregionid == key.studyregionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for StpasaConstraintsolution3 {
    type Builder = StpasaConstraintsolution3Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "constraintid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "capacityrhs",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "capacitymarginalvalue",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "capacityviolationdegree",
                    arrow::datatypes::DataType::Decimal128(12, 2),
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
                    "runtype",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "studyregionid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        StpasaConstraintsolution3Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            constraintid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            capacityrhs_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            capacitymarginalvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            capacityviolationdegree_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runtype_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            studyregionid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.constraintid_array.append_value(row.constraintid());
        builder
            .capacityrhs_array
            .append_option({
                row.capacityrhs
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .capacitymarginalvalue_array
            .append_option({
                row.capacitymarginalvalue
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .capacityviolationdegree_array
            .append_option({
                row.capacityviolationdegree
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.runtype_array.append_value(row.runtype());
        builder.studyregionid_array.append_value(row.studyregionid());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.capacityrhs_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.capacitymarginalvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.capacityviolationdegree_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.studyregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct StpasaConstraintsolution3Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    constraintid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    capacityrhs_array: arrow::array::builder::Decimal128Builder,
    capacitymarginalvalue_array: arrow::array::builder::Decimal128Builder,
    capacityviolationdegree_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    runtype_array: arrow::array::StringDictionaryBuilder<arrow::array::types::Int32Type>,
    studyregionid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
}
pub struct StpasaDuidavailability1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &StpasaDuidavailability1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaDuidavailability1 {
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
pub struct StpasaDuidavailability1Mapping([usize; 10]);
/// # Summary
///
/// ## STPASA_DUIDAVAILABILITY
///
/// This report delivers available capacity, PASA availability and given recall period for all scheduled resources. Note that for an MNSP, DUID = LINKID in the MNSP_INTERCONNECTOR table.
///
/// * Data Set Name: Stpasa
/// * File Name: Duidavailability
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
/// * DUID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct StpasaDuidavailability1Row<'data> {
    /// STPASA run, identified by the nominal start time of the run.
    pub run_datetime: chrono::NaiveDateTime,
    /// Half hour ended interval
    pub interval_datetime: chrono::NaiveDateTime,
    /// NEM Dispatchable Unit Identifier
    pub duid: core::ops::Range<usize>,
    /// Available Capacity for a scheduled generating unit, semi-scheduled generating unit, BDU (Gen side), WDR or MNSP.
    pub generation_max_availability: Option<rust_decimal::Decimal>,
    /// PASA Availability for a scheduled generating unit, BDU (Gen side), WDR or MNSP. Null for a semi-scheduled generating unit.
    pub generation_pasa_availability: Option<rust_decimal::Decimal>,
    /// Recall Period associated with the PASA Availability for a scheduled generating unit, BDU (Gen side), WDR or MNSP. Null for a semi-scheduled generating unit.
    pub generation_recall_period: Option<rust_decimal::Decimal>,
    /// Available Capacity for a scheduled load or BDU(Load side).
    pub load_max_availability: Option<rust_decimal::Decimal>,
    /// PASA Availability for a scheduled load or BDU(Load side).
    pub load_pasa_availability: Option<rust_decimal::Decimal>,
    /// Recall Period associated with the PASA Availability for a scheduled load or BDU(Load side).
    pub load_recall_period: Option<rust_decimal::Decimal>,
    /// Report Creation Date Time.
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> StpasaDuidavailability1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
}
impl mmsdm_core::GetTable for StpasaDuidavailability1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "STPASA";
    const TABLE_NAME: &'static str = "DUIDAVAILABILITY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = StpasaDuidavailability1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "INTERVAL_DATETIME",
        "DUID",
        "GENERATION_MAX_AVAILABILITY",
        "GENERATION_PASA_AVAILABILITY",
        "GENERATION_RECALL_PERIOD",
        "LOAD_MAX_AVAILABILITY",
        "LOAD_PASA_AVAILABILITY",
        "LOAD_RECALL_PERIOD",
        "LASTCHANGED",
    ];
    type Row<'row> = StpasaDuidavailability1Row<'row>;
    type FieldMapping = StpasaDuidavailability1Mapping;
    type PrimaryKey = StpasaDuidavailability1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(StpasaDuidavailability1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[2])?,
            generation_max_availability: row
                .get_opt_custom_parsed_at_idx(
                    "generation_max_availability",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            generation_pasa_availability: row
                .get_opt_custom_parsed_at_idx(
                    "generation_pasa_availability",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            generation_recall_period: row
                .get_opt_custom_parsed_at_idx(
                    "generation_recall_period",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            load_max_availability: row
                .get_opt_custom_parsed_at_idx(
                    "load_max_availability",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            load_pasa_availability: row
                .get_opt_custom_parsed_at_idx(
                    "load_pasa_availability",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            load_recall_period: row
                .get_opt_custom_parsed_at_idx(
                    "load_recall_period",
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
        Ok(StpasaDuidavailability1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> StpasaDuidavailability1PrimaryKey {
        StpasaDuidavailability1PrimaryKey {
            duid: row.duid().to_string(),
            interval_datetime: row.interval_datetime,
            run_datetime: row.run_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("stpasa_duidavailability_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        StpasaDuidavailability1Row {
            run_datetime: row.run_datetime.clone(),
            interval_datetime: row.interval_datetime.clone(),
            duid: row.duid.clone(),
            generation_max_availability: row.generation_max_availability.clone(),
            generation_pasa_availability: row.generation_pasa_availability.clone(),
            generation_recall_period: row.generation_recall_period.clone(),
            load_max_availability: row.load_max_availability.clone(),
            load_pasa_availability: row.load_pasa_availability.clone(),
            load_recall_period: row.load_recall_period.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StpasaDuidavailability1PrimaryKey {
    pub duid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for StpasaDuidavailability1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for StpasaDuidavailability1Row<'data> {
    type Row<'other> = StpasaDuidavailability1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for StpasaDuidavailability1Row<'data> {
    type PrimaryKey = StpasaDuidavailability1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for StpasaDuidavailability1PrimaryKey {
    type Row<'other> = StpasaDuidavailability1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for StpasaDuidavailability1PrimaryKey {
    type PrimaryKey = StpasaDuidavailability1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for StpasaDuidavailability1 {
    type Builder = StpasaDuidavailability1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "generation_max_availability",
                    arrow::datatypes::DataType::Decimal128(12, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "generation_pasa_availability",
                    arrow::datatypes::DataType::Decimal128(12, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "generation_recall_period",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "load_max_availability",
                    arrow::datatypes::DataType::Decimal128(12, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "load_pasa_availability",
                    arrow::datatypes::DataType::Decimal128(12, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "load_recall_period",
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
        StpasaDuidavailability1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            generation_max_availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 6)),
            generation_pasa_availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 0)),
            generation_recall_period_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            load_max_availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 6)),
            load_pasa_availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 0)),
            load_recall_period_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .generation_max_availability_array
            .append_option({
                row.generation_max_availability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .generation_pasa_availability_array
            .append_option({
                row.generation_pasa_availability
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .generation_recall_period_array
            .append_option({
                row.generation_recall_period
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .load_max_availability_array
            .append_option({
                row.load_max_availability
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .load_pasa_availability_array
            .append_option({
                row.load_pasa_availability
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .load_recall_period_array
            .append_option({
                row.load_recall_period
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.generation_max_availability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.generation_pasa_availability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.generation_recall_period_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.load_max_availability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.load_pasa_availability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.load_recall_period_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct StpasaDuidavailability1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::StringDictionaryBuilder<arrow::array::types::Int32Type>,
    generation_max_availability_array: arrow::array::builder::Decimal128Builder,
    generation_pasa_availability_array: arrow::array::builder::Decimal128Builder,
    generation_recall_period_array: arrow::array::builder::Decimal128Builder,
    load_max_availability_array: arrow::array::builder::Decimal128Builder,
    load_pasa_availability_array: arrow::array::builder::Decimal128Builder,
    load_recall_period_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct StpasaFnmCasesolution1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &StpasaFnmCasesolution1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaFnmCasesolution1 {
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
pub struct StpasaFnmCasesolution1Mapping([usize; 8]);
/// # Summary
///
/// ## STPASA_FNM_CASESOLUTION
///
/// STPASA_FNM_CASESOLUTION shows the case run details, including the available run types, LOR and Deficit condition for each case.
///
/// * Data Set Name: Stpasa
/// * File Name: Fnm Casesolution
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
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct StpasaFnmCasesolution1Row<'data> {
    /// Unique Timestamp Identifier for this run, identified by the first half hour ended interval of the run
    pub run_datetime: chrono::NaiveDateTime,
    /// LORCONDITION is only set if supply deficit exists in a Zone that contains the Regional Reference NodeLORCONDITION indicates the most severe condition for the case:LORCONDITION = 3 if deficit in BASE run, else = 2 if deficit in RELIABILITY run, else = 1 if deficit in WARNING run, else 0
    pub lorcondition: Option<rust_decimal::Decimal>,
    /// DEFICITCONDITION is only set if supply deficit exists in a Zone that does NOT contain the Regional Reference NodeDEFICITCONDITION indicates the most severe condition for the case:DEFICITCONDITION = 3 if deficit in BASE run, else = 2 if deficit in RELIABILITY run, else = 1 if deficit in WARNING run, else 0
    pub deficitcondition: Option<rust_decimal::Decimal>,
    /// YES = Available, NO = Not Available
    pub base_run_available: core::ops::Range<usize>,
    /// YES = Available, NO = Not Available
    pub reliability_run_available: core::ops::Range<usize>,
    /// YES = Available, NO = Not Available
    pub warning_run_available: core::ops::Range<usize>,
    /// Version of the PASA solver used to solve this case
    pub pasaversion: core::ops::Range<usize>,
    /// Date time this record was created
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> StpasaFnmCasesolution1Row<'data> {
    pub fn base_run_available(&self) -> Option<&str> {
        if self.base_run_available.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.base_run_available.clone(),
                ),
            )
        }
    }
    pub fn reliability_run_available(&self) -> Option<&str> {
        if self.reliability_run_available.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.reliability_run_available.clone(),
                ),
            )
        }
    }
    pub fn warning_run_available(&self) -> Option<&str> {
        if self.warning_run_available.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.warning_run_available.clone(),
                ),
            )
        }
    }
    pub fn pasaversion(&self) -> Option<&str> {
        if self.pasaversion.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.pasaversion.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for StpasaFnmCasesolution1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "STPASA";
    const TABLE_NAME: &'static str = "FNM_CASESOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = StpasaFnmCasesolution1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "LORCONDITION",
        "DEFICITCONDITION",
        "BASE_RUN_AVAILABLE",
        "RELIABILITY_RUN_AVAILABLE",
        "WARNING_RUN_AVAILABLE",
        "PASAVERSION",
        "LASTCHANGED",
    ];
    type Row<'row> = StpasaFnmCasesolution1Row<'row>;
    type FieldMapping = StpasaFnmCasesolution1Mapping;
    type PrimaryKey = StpasaFnmCasesolution1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(StpasaFnmCasesolution1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lorcondition: row
                .get_opt_custom_parsed_at_idx(
                    "lorcondition",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            deficitcondition: row
                .get_opt_custom_parsed_at_idx(
                    "deficitcondition",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            base_run_available: row
                .get_opt_range("base_run_available", field_mapping.0[3])?,
            reliability_run_available: row
                .get_opt_range("reliability_run_available", field_mapping.0[4])?,
            warning_run_available: row
                .get_opt_range("warning_run_available", field_mapping.0[5])?,
            pasaversion: row.get_opt_range("pasaversion", field_mapping.0[6])?,
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
        Ok(StpasaFnmCasesolution1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> StpasaFnmCasesolution1PrimaryKey {
        StpasaFnmCasesolution1PrimaryKey {
            run_datetime: row.run_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("stpasa_fnm_casesolution_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        StpasaFnmCasesolution1Row {
            run_datetime: row.run_datetime.clone(),
            lorcondition: row.lorcondition.clone(),
            deficitcondition: row.deficitcondition.clone(),
            base_run_available: row.base_run_available.clone(),
            reliability_run_available: row.reliability_run_available.clone(),
            warning_run_available: row.warning_run_available.clone(),
            pasaversion: row.pasaversion.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StpasaFnmCasesolution1PrimaryKey {
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for StpasaFnmCasesolution1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmCasesolution1Row<'data> {
    type Row<'other> = StpasaFnmCasesolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for StpasaFnmCasesolution1Row<'data> {
    type PrimaryKey = StpasaFnmCasesolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmCasesolution1PrimaryKey {
    type Row<'other> = StpasaFnmCasesolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for StpasaFnmCasesolution1PrimaryKey {
    type PrimaryKey = StpasaFnmCasesolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for StpasaFnmCasesolution1 {
    type Builder = StpasaFnmCasesolution1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "lorcondition",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "deficitcondition",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "base_run_available",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int16),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reliability_run_available",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int16),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "warning_run_available",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int16),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "pasaversion",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
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
        StpasaFnmCasesolution1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lorcondition_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            deficitcondition_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            base_run_available_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int16Type,
            >::new(),
            reliability_run_available_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int16Type,
            >::new(),
            warning_run_available_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int16Type,
            >::new(),
            pasaversion_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder
            .lorcondition_array
            .append_option({
                row.lorcondition
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .deficitcondition_array
            .append_option({
                row.deficitcondition
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.base_run_available_array.append_option(row.base_run_available());
        builder
            .reliability_run_available_array
            .append_option(row.reliability_run_available());
        builder.warning_run_available_array.append_option(row.warning_run_available());
        builder.pasaversion_array.append_option(row.pasaversion());
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lorcondition_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.deficitcondition_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.base_run_available_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.reliability_run_available_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.warning_run_available_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.pasaversion_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct StpasaFnmCasesolution1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    lorcondition_array: arrow::array::builder::Decimal128Builder,
    deficitcondition_array: arrow::array::builder::Decimal128Builder,
    base_run_available_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int16Type,
    >,
    reliability_run_available_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int16Type,
    >,
    warning_run_available_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int16Type,
    >,
    pasaversion_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct StpasaFnmConstraintsolution1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &StpasaFnmConstraintsolution1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaFnmConstraintsolution1 {
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
pub struct StpasaFnmConstraintsolution1Mapping([usize; 9]);
/// # Summary
///
/// ## STPASA_FNM_CONSTRAINTSOLUTION
///
/// STPASA_FNM_CONSTRAINTSOLUTION shows the manual or thermal constraint (created by PASA), including marginal value, violation degree, LHS and RHS.
///
/// * Data Set Name: Stpasa
/// * File Name: Fnm Constraintsolution
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
/// * CONSTRAINTID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
/// * RUNTYPE
#[derive(Debug, PartialEq, Eq)]
pub struct StpasaFnmConstraintsolution1Row<'data> {
    /// Unique Timestamp Identifier for this run, identified by the first half hour ended interval of the run
    pub run_datetime: chrono::NaiveDateTime,
    /// Run Type (BASE, RELIABILITY, WARNING)
    pub runtype: core::ops::Range<usize>,
    /// End date time of the interval
    pub interval_datetime: chrono::NaiveDateTime,
    /// Constraint identifier, either manual constraint (synonymous with GenConID) or thermal constraint created by PASA with format 'BASE_<BranchName>' or '<ContingencyID>_<BranchName>'
    pub constraintid: core::ops::Range<usize>,
    /// Constraint Marginal Value ($/MW)
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Constraint Violation Degree (MW)
    pub violationdegree: Option<rust_decimal::Decimal>,
    /// Constraint LHS (MW)
    pub lhs: Option<rust_decimal::Decimal>,
    /// Constraint RHS (MW)
    pub rhs: Option<rust_decimal::Decimal>,
    /// Date time this record was created
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> StpasaFnmConstraintsolution1Row<'data> {
    pub fn runtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.runtype.clone())
    }
    pub fn constraintid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.constraintid.clone())
    }
}
impl mmsdm_core::GetTable for StpasaFnmConstraintsolution1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "STPASA";
    const TABLE_NAME: &'static str = "FNM_CONSTRAINTSOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = StpasaFnmConstraintsolution1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "RUNTYPE",
        "INTERVAL_DATETIME",
        "CONSTRAINTID",
        "MARGINALVALUE",
        "VIOLATIONDEGREE",
        "LHS",
        "RHS",
        "LASTCHANGED",
    ];
    type Row<'row> = StpasaFnmConstraintsolution1Row<'row>;
    type FieldMapping = StpasaFnmConstraintsolution1Mapping;
    type PrimaryKey = StpasaFnmConstraintsolution1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(StpasaFnmConstraintsolution1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runtype: row.get_range("runtype", field_mapping.0[1])?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            constraintid: row.get_range("constraintid", field_mapping.0[3])?,
            marginalvalue: row
                .get_opt_custom_parsed_at_idx(
                    "marginalvalue",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            violationdegree: row
                .get_opt_custom_parsed_at_idx(
                    "violationdegree",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lhs: row
                .get_opt_custom_parsed_at_idx(
                    "lhs",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rhs: row
                .get_opt_custom_parsed_at_idx(
                    "rhs",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
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
        Ok(StpasaFnmConstraintsolution1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> StpasaFnmConstraintsolution1PrimaryKey {
        StpasaFnmConstraintsolution1PrimaryKey {
            constraintid: row.constraintid().to_string(),
            interval_datetime: row.interval_datetime,
            run_datetime: row.run_datetime,
            runtype: row.runtype().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("stpasa_fnm_constraintsolution_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        StpasaFnmConstraintsolution1Row {
            run_datetime: row.run_datetime.clone(),
            runtype: row.runtype.clone(),
            interval_datetime: row.interval_datetime.clone(),
            constraintid: row.constraintid.clone(),
            marginalvalue: row.marginalvalue.clone(),
            violationdegree: row.violationdegree.clone(),
            lhs: row.lhs.clone(),
            rhs: row.rhs.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StpasaFnmConstraintsolution1PrimaryKey {
    pub constraintid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
    pub runtype: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for StpasaFnmConstraintsolution1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmConstraintsolution1Row<'data> {
    type Row<'other> = StpasaFnmConstraintsolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid() == row.constraintid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime && self.runtype() == row.runtype()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for StpasaFnmConstraintsolution1Row<'data> {
    type PrimaryKey = StpasaFnmConstraintsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid() == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime && self.runtype() == key.runtype
    }
}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmConstraintsolution1PrimaryKey {
    type Row<'other> = StpasaFnmConstraintsolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid == row.constraintid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime && self.runtype == row.runtype()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for StpasaFnmConstraintsolution1PrimaryKey {
    type PrimaryKey = StpasaFnmConstraintsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for StpasaFnmConstraintsolution1 {
    type Builder = StpasaFnmConstraintsolution1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runtype",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "constraintid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "marginalvalue",
                    arrow::datatypes::DataType::Decimal128(20, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "violationdegree",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lhs",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rhs",
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
        StpasaFnmConstraintsolution1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runtype_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            constraintid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            marginalvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(20, 5)),
            violationdegree_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lhs_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rhs_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.runtype_array.append_value(row.runtype());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.constraintid_array.append_value(row.constraintid());
        builder
            .marginalvalue_array
            .append_option({
                row.marginalvalue
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .violationdegree_array
            .append_option({
                row.violationdegree
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lhs_array
            .append_option({
                row.lhs
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rhs_array
            .append_option({
                row.rhs
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marginalvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.violationdegree_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lhs_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rhs_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct StpasaFnmConstraintsolution1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    runtype_array: arrow::array::StringDictionaryBuilder<arrow::array::types::Int32Type>,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    constraintid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    marginalvalue_array: arrow::array::builder::Decimal128Builder,
    violationdegree_array: arrow::array::builder::Decimal128Builder,
    lhs_array: arrow::array::builder::Decimal128Builder,
    rhs_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct StpasaFnmDuidavailability1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &StpasaFnmDuidavailability1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaFnmDuidavailability1 {
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
pub struct StpasaFnmDuidavailability1Mapping([usize; 12]);
/// # Summary
///
/// ## STPASA_FNM_DUIDAVAILABILITY
///
/// STPASA_FNM_DUIDAVAILABILITY shows Available Capacity, PASA Availability and given Recall Period for all scheduled
///
/// * Data Set Name: Stpasa
/// * File Name: Fnm Duidavailability
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
/// * DUID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct StpasaFnmDuidavailability1Row<'data> {
    /// Unique Timestamp Identifier for this run, identified by the first half hour ended interval of the run
    pub run_datetime: chrono::NaiveDateTime,
    /// End date time of the interval
    pub interval_datetime: chrono::NaiveDateTime,
    /// NEM Dispatchable Unit Identifier
    pub duid: core::ops::Range<usize>,
    /// Trading Date of the energy bid
    pub bid_tradingdate: Option<chrono::NaiveDateTime>,
    /// Date Time that the energy bid was received
    pub bid_offerdatetime: Option<chrono::NaiveDateTime>,
    /// Available Capacity for a scheduled generating unit, semi-scheduled generating unit, BDU (Gen side), WDR or MNSP
    pub generation_max_availability: Option<rust_decimal::Decimal>,
    /// PASA Availability for a scheduled generating unit, BDU (Gen side), WDR or MNSP. Null for a semi-scheduled generating unit (MW)
    pub generation_pasa_availability: Option<rust_decimal::Decimal>,
    /// Recall Period associated with the PASA Availability for a scheduled generating unit, BDU (Gen side), WDR or MNSP. Null for a semi-scheduled generating unit (Hours)
    pub generation_recall_period: Option<rust_decimal::Decimal>,
    /// Available Capacity for a scheduled load or BDU (Load side) (MW)
    pub load_max_availability: Option<rust_decimal::Decimal>,
    /// PASA Availability for a scheduled load or BDU (Load side) (MW)
    pub load_pasa_availability: Option<rust_decimal::Decimal>,
    /// Recall Period associated with the PASA Availability for a scheduled load or BDU (Load side) (Hours)
    pub load_recall_period: Option<rust_decimal::Decimal>,
    /// Date time this record was created
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> StpasaFnmDuidavailability1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
}
impl mmsdm_core::GetTable for StpasaFnmDuidavailability1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "STPASA";
    const TABLE_NAME: &'static str = "FNM_DUIDAVAILABILITY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = StpasaFnmDuidavailability1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "INTERVAL_DATETIME",
        "DUID",
        "BID_TRADINGDATE",
        "BID_OFFERDATETIME",
        "GENERATION_MAX_AVAILABILITY",
        "GENERATION_PASA_AVAILABILITY",
        "GENERATION_RECALL_PERIOD",
        "LOAD_MAX_AVAILABILITY",
        "LOAD_PASA_AVAILABILITY",
        "LOAD_RECALL_PERIOD",
        "LASTCHANGED",
    ];
    type Row<'row> = StpasaFnmDuidavailability1Row<'row>;
    type FieldMapping = StpasaFnmDuidavailability1Mapping;
    type PrimaryKey = StpasaFnmDuidavailability1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(StpasaFnmDuidavailability1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            duid: row.get_range("duid", field_mapping.0[2])?,
            bid_tradingdate: row
                .get_opt_custom_parsed_at_idx(
                    "bid_tradingdate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            bid_offerdatetime: row
                .get_opt_custom_parsed_at_idx(
                    "bid_offerdatetime",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            generation_max_availability: row
                .get_opt_custom_parsed_at_idx(
                    "generation_max_availability",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            generation_pasa_availability: row
                .get_opt_custom_parsed_at_idx(
                    "generation_pasa_availability",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            generation_recall_period: row
                .get_opt_custom_parsed_at_idx(
                    "generation_recall_period",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            load_max_availability: row
                .get_opt_custom_parsed_at_idx(
                    "load_max_availability",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            load_pasa_availability: row
                .get_opt_custom_parsed_at_idx(
                    "load_pasa_availability",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            load_recall_period: row
                .get_opt_custom_parsed_at_idx(
                    "load_recall_period",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[11],
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
        Ok(StpasaFnmDuidavailability1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> StpasaFnmDuidavailability1PrimaryKey {
        StpasaFnmDuidavailability1PrimaryKey {
            duid: row.duid().to_string(),
            interval_datetime: row.interval_datetime,
            run_datetime: row.run_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("stpasa_fnm_duidavailability_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        StpasaFnmDuidavailability1Row {
            run_datetime: row.run_datetime.clone(),
            interval_datetime: row.interval_datetime.clone(),
            duid: row.duid.clone(),
            bid_tradingdate: row.bid_tradingdate.clone(),
            bid_offerdatetime: row.bid_offerdatetime.clone(),
            generation_max_availability: row.generation_max_availability.clone(),
            generation_pasa_availability: row.generation_pasa_availability.clone(),
            generation_recall_period: row.generation_recall_period.clone(),
            load_max_availability: row.load_max_availability.clone(),
            load_pasa_availability: row.load_pasa_availability.clone(),
            load_recall_period: row.load_recall_period.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StpasaFnmDuidavailability1PrimaryKey {
    pub duid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for StpasaFnmDuidavailability1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmDuidavailability1Row<'data> {
    type Row<'other> = StpasaFnmDuidavailability1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for StpasaFnmDuidavailability1Row<'data> {
    type PrimaryKey = StpasaFnmDuidavailability1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmDuidavailability1PrimaryKey {
    type Row<'other> = StpasaFnmDuidavailability1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for StpasaFnmDuidavailability1PrimaryKey {
    type PrimaryKey = StpasaFnmDuidavailability1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for StpasaFnmDuidavailability1 {
    type Builder = StpasaFnmDuidavailability1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "bid_tradingdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bid_offerdatetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "generation_max_availability",
                    arrow::datatypes::DataType::Decimal128(12, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "generation_pasa_availability",
                    arrow::datatypes::DataType::Decimal128(12, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "generation_recall_period",
                    arrow::datatypes::DataType::Decimal128(8, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "load_max_availability",
                    arrow::datatypes::DataType::Decimal128(12, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "load_pasa_availability",
                    arrow::datatypes::DataType::Decimal128(12, 3),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "load_recall_period",
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
        StpasaFnmDuidavailability1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            bid_tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            bid_offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            generation_max_availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            generation_pasa_availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            generation_recall_period_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            load_max_availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            load_pasa_availability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 3)),
            load_recall_period_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 3)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.duid_array.append_value(row.duid());
        builder
            .bid_tradingdate_array
            .append_option(
                row.bid_tradingdate.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .bid_offerdatetime_array
            .append_option(
                row.bid_offerdatetime.map(|val| val.and_utc().timestamp_millis()),
            );
        builder
            .generation_max_availability_array
            .append_option({
                row.generation_max_availability
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .generation_pasa_availability_array
            .append_option({
                row.generation_pasa_availability
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .generation_recall_period_array
            .append_option({
                row.generation_recall_period
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .load_max_availability_array
            .append_option({
                row.load_max_availability
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .load_pasa_availability_array
            .append_option({
                row.load_pasa_availability
                    .map(|mut val| {
                        val.rescale(3);
                        val.mantissa()
                    })
            });
        builder
            .load_recall_period_array
            .append_option({
                row.load_recall_period
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bid_tradingdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bid_offerdatetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.generation_max_availability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.generation_pasa_availability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.generation_recall_period_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.load_max_availability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.load_pasa_availability_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.load_recall_period_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct StpasaFnmDuidavailability1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::StringDictionaryBuilder<arrow::array::types::Int32Type>,
    bid_tradingdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    bid_offerdatetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    generation_max_availability_array: arrow::array::builder::Decimal128Builder,
    generation_pasa_availability_array: arrow::array::builder::Decimal128Builder,
    generation_recall_period_array: arrow::array::builder::Decimal128Builder,
    load_max_availability_array: arrow::array::builder::Decimal128Builder,
    load_pasa_availability_array: arrow::array::builder::Decimal128Builder,
    load_recall_period_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct StpasaFnmInterconnectorsoln1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &StpasaFnmInterconnectorsoln1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaFnmInterconnectorsoln1 {
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
pub struct StpasaFnmInterconnectorsoln1Mapping([usize; 6]);
/// # Summary
///
/// ## STPASA_FNM_INTERCONNECTORSOLN
///
/// STPASA_FNM_INTERCONNECTORSOLN shows cleared Interconnector flow for the interval.
///
/// * Data Set Name: Stpasa
/// * File Name: Fnm Interconnectorsoln
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
/// * INTERCONNECTORID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
/// * RUNTYPE
#[derive(Debug, PartialEq, Eq)]
pub struct StpasaFnmInterconnectorsoln1Row<'data> {
    /// Unique Timestamp Identifier for this run, identified by the first half hour ended interval of the run
    pub run_datetime: chrono::NaiveDateTime,
    /// Run Type (BASE, RELIABILITY, WARNING)
    pub runtype: core::ops::Range<usize>,
    /// End date time of the interval
    pub interval_datetime: chrono::NaiveDateTime,
    /// Interconnector Identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// Cleared Interconnector flow (MW)
    pub clearedflow: Option<rust_decimal::Decimal>,
    /// Date time this record was created
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> StpasaFnmInterconnectorsoln1Row<'data> {
    pub fn runtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.runtype.clone())
    }
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
}
impl mmsdm_core::GetTable for StpasaFnmInterconnectorsoln1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "STPASA";
    const TABLE_NAME: &'static str = "FNM_INTERCONNECTORSOLN";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = StpasaFnmInterconnectorsoln1Mapping([
        4, 5, 6, 7, 8, 9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "RUNTYPE",
        "INTERVAL_DATETIME",
        "INTERCONNECTORID",
        "CLEAREDFLOW",
        "LASTCHANGED",
    ];
    type Row<'row> = StpasaFnmInterconnectorsoln1Row<'row>;
    type FieldMapping = StpasaFnmInterconnectorsoln1Mapping;
    type PrimaryKey = StpasaFnmInterconnectorsoln1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(StpasaFnmInterconnectorsoln1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runtype: row.get_range("runtype", field_mapping.0[1])?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[3])?,
            clearedflow: row
                .get_opt_custom_parsed_at_idx(
                    "clearedflow",
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
        Ok(StpasaFnmInterconnectorsoln1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> StpasaFnmInterconnectorsoln1PrimaryKey {
        StpasaFnmInterconnectorsoln1PrimaryKey {
            interconnectorid: row.interconnectorid().to_string(),
            interval_datetime: row.interval_datetime,
            run_datetime: row.run_datetime,
            runtype: row.runtype().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("stpasa_fnm_interconnectorsoln_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        StpasaFnmInterconnectorsoln1Row {
            run_datetime: row.run_datetime.clone(),
            runtype: row.runtype.clone(),
            interval_datetime: row.interval_datetime.clone(),
            interconnectorid: row.interconnectorid.clone(),
            clearedflow: row.clearedflow.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StpasaFnmInterconnectorsoln1PrimaryKey {
    pub interconnectorid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
    pub runtype: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for StpasaFnmInterconnectorsoln1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmInterconnectorsoln1Row<'data> {
    type Row<'other> = StpasaFnmInterconnectorsoln1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interconnectorid() == row.interconnectorid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime && self.runtype() == row.runtype()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for StpasaFnmInterconnectorsoln1Row<'data> {
    type PrimaryKey = StpasaFnmInterconnectorsoln1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid() == key.interconnectorid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime && self.runtype() == key.runtype
    }
}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmInterconnectorsoln1PrimaryKey {
    type Row<'other> = StpasaFnmInterconnectorsoln1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interconnectorid == row.interconnectorid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime && self.runtype == row.runtype()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for StpasaFnmInterconnectorsoln1PrimaryKey {
    type PrimaryKey = StpasaFnmInterconnectorsoln1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for StpasaFnmInterconnectorsoln1 {
    type Builder = StpasaFnmInterconnectorsoln1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runtype",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int16),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "clearedflow",
                    arrow::datatypes::DataType::Decimal128(12, 2),
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
        StpasaFnmInterconnectorsoln1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runtype_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interconnectorid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int16Type,
            >::new(),
            clearedflow_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.runtype_array.append_value(row.runtype());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder
            .clearedflow_array
            .append_option({
                row.clearedflow
                    .map(|mut val| {
                        val.rescale(2);
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clearedflow_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct StpasaFnmInterconnectorsoln1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    runtype_array: arrow::array::StringDictionaryBuilder<arrow::array::types::Int32Type>,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interconnectorid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int16Type,
    >,
    clearedflow_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct StpasaFnmInterzonalsolution1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &StpasaFnmInterzonalsolution1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaFnmInterzonalsolution1 {
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
pub struct StpasaFnmInterzonalsolution1Mapping([usize; 8]);
/// # Summary
///
/// ## STPASA_FNM_INTERZONALSOLUTION
///
/// STPASA_FNM_INTERZONALSOLUTION shows cleared inter zonal flow for the interval and run type.
///
/// * Data Set Name: Stpasa
/// * File Name: Fnm Interzonalsolution
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
/// * INTERVAL_DATETIME
/// * INTERZONALCONNECTORID
/// * RUN_DATETIME
/// * RUNTYPE
#[derive(Debug, PartialEq, Eq)]
pub struct StpasaFnmInterzonalsolution1Row<'data> {
    /// Unique Timestamp Identifier for this run, identified by the first half hour ended interval of the run
    pub run_datetime: chrono::NaiveDateTime,
    /// Run Type (BASE, RELIABILITY, WARNING)
    pub runtype: core::ops::Range<usize>,
    /// End date time of the interval
    pub interval_datetime: chrono::NaiveDateTime,
    /// InterzonalConnector Identifier
    pub interzonalconnectorid: core::ops::Range<usize>,
    /// FromZoneID of the InterZonalConnectorID
    pub fromzoneid: core::ops::Range<usize>,
    /// ToZoneID of the InterZonalConnectorID
    pub tozoneid: core::ops::Range<usize>,
    /// Cleared Interzonal flow (MW)
    pub clearedflow: Option<rust_decimal::Decimal>,
    /// Date time this record was created
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> StpasaFnmInterzonalsolution1Row<'data> {
    pub fn runtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.runtype.clone())
    }
    pub fn interzonalconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interzonalconnectorid.clone(),
        )
    }
    pub fn fromzoneid(&self) -> Option<&str> {
        if self.fromzoneid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.fromzoneid.clone(),
                ),
            )
        }
    }
    pub fn tozoneid(&self) -> Option<&str> {
        if self.tozoneid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.tozoneid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for StpasaFnmInterzonalsolution1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "STPASA";
    const TABLE_NAME: &'static str = "FNM_INTERZONALSOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = StpasaFnmInterzonalsolution1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "RUNTYPE",
        "INTERVAL_DATETIME",
        "INTERZONALCONNECTORID",
        "FROMZONEID",
        "TOZONEID",
        "CLEAREDFLOW",
        "LASTCHANGED",
    ];
    type Row<'row> = StpasaFnmInterzonalsolution1Row<'row>;
    type FieldMapping = StpasaFnmInterzonalsolution1Mapping;
    type PrimaryKey = StpasaFnmInterzonalsolution1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(StpasaFnmInterzonalsolution1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runtype: row.get_range("runtype", field_mapping.0[1])?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interzonalconnectorid: row
                .get_range("interzonalconnectorid", field_mapping.0[3])?,
            fromzoneid: row.get_opt_range("fromzoneid", field_mapping.0[4])?,
            tozoneid: row.get_opt_range("tozoneid", field_mapping.0[5])?,
            clearedflow: row
                .get_opt_custom_parsed_at_idx(
                    "clearedflow",
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
        Ok(StpasaFnmInterzonalsolution1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> StpasaFnmInterzonalsolution1PrimaryKey {
        StpasaFnmInterzonalsolution1PrimaryKey {
            interval_datetime: row.interval_datetime,
            interzonalconnectorid: row.interzonalconnectorid().to_string(),
            run_datetime: row.run_datetime,
            runtype: row.runtype().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("stpasa_fnm_interzonalsolution_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        StpasaFnmInterzonalsolution1Row {
            run_datetime: row.run_datetime.clone(),
            runtype: row.runtype.clone(),
            interval_datetime: row.interval_datetime.clone(),
            interzonalconnectorid: row.interzonalconnectorid.clone(),
            fromzoneid: row.fromzoneid.clone(),
            tozoneid: row.tozoneid.clone(),
            clearedflow: row.clearedflow.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StpasaFnmInterzonalsolution1PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub interzonalconnectorid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
    pub runtype: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for StpasaFnmInterzonalsolution1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmInterzonalsolution1Row<'data> {
    type Row<'other> = StpasaFnmInterzonalsolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.interzonalconnectorid() == row.interzonalconnectorid()
            && self.run_datetime == row.run_datetime && self.runtype() == row.runtype()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for StpasaFnmInterzonalsolution1Row<'data> {
    type PrimaryKey = StpasaFnmInterzonalsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.interzonalconnectorid() == key.interzonalconnectorid
            && self.run_datetime == key.run_datetime && self.runtype() == key.runtype
    }
}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmInterzonalsolution1PrimaryKey {
    type Row<'other> = StpasaFnmInterzonalsolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.interzonalconnectorid == row.interzonalconnectorid()
            && self.run_datetime == row.run_datetime && self.runtype == row.runtype()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for StpasaFnmInterzonalsolution1PrimaryKey {
    type PrimaryKey = StpasaFnmInterzonalsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.interzonalconnectorid == key.interzonalconnectorid
            && self.run_datetime == key.run_datetime && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for StpasaFnmInterzonalsolution1 {
    type Builder = StpasaFnmInterzonalsolution1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runtype",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interzonalconnectorid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "fromzoneid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "tozoneid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "clearedflow",
                    arrow::datatypes::DataType::Decimal128(12, 2),
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
        StpasaFnmInterzonalsolution1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runtype_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interzonalconnectorid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            fromzoneid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            tozoneid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            clearedflow_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.runtype_array.append_value(row.runtype());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.interzonalconnectorid_array.append_value(row.interzonalconnectorid());
        builder.fromzoneid_array.append_option(row.fromzoneid());
        builder.tozoneid_array.append_option(row.tozoneid());
        builder
            .clearedflow_array
            .append_option({
                row.clearedflow
                    .map(|mut val| {
                        val.rescale(2);
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interzonalconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromzoneid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tozoneid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clearedflow_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct StpasaFnmInterzonalsolution1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    runtype_array: arrow::array::StringDictionaryBuilder<arrow::array::types::Int32Type>,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interzonalconnectorid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    fromzoneid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    tozoneid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    clearedflow_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct StpasaFnmRegionsolution1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &StpasaFnmRegionsolution1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaFnmRegionsolution1 {
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
pub struct StpasaFnmRegionsolution1Mapping([usize; 26]);
/// # Summary
///
/// ## STPASA_FNM_REGIONSOLUTION
///
/// STPASA_FNM_REGIONSOLUTION shows regional demand, cleared values of resources, spare capacity, losses for each run type and intervals.
///
/// * Data Set Name: Stpasa
/// * File Name: Fnm Regionsolution
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
/// * INTERVAL_DATETIME
/// * REGIONID
/// * RUN_DATETIME
/// * RUNTYPE
#[derive(Debug, PartialEq, Eq)]
pub struct StpasaFnmRegionsolution1Row<'data> {
    /// Unique Timestamp Identifier for this run, identified by the first half hour ended interval of the run
    pub run_datetime: chrono::NaiveDateTime,
    /// Run Type (BASE, RELIABILITY, WARNING)
    pub runtype: core::ops::Range<usize>,
    /// End date time of the interval
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Lack of Reserve Condition (LORCONDITION) >0 if a supply deficit exists in the Zone for this Region that contains its Regional Reference NodeLORCONDITION = 3 if deficit in BASE runLORCONDITION = 2 if deficit in RELIABILITY runLORCONDITION = 1 if deficit in WARNING run
    pub lorcondition: Option<rust_decimal::Decimal>,
    /// Deficit Condition (DEFICITCONDITION) >0 if a supply deficit only exists in a Zone for this Region that does not contain its Regional Reference NodeDEFICITCONDITION = 3 if deficit in BASE runDEFICITCONDITION = 2 if deficit in RELIABILITY runDEFICITCONDITION = 1 if deficit in WARNING run
    pub deficitcondition: Option<rust_decimal::Decimal>,
    /// Most probable Demand Forecast adjusted by Demand Uncertainty Margin (MW)
    pub initialdemand: Option<rust_decimal::Decimal>,
    /// Aggregate Uncertainty Margin adjustment to most probable Demand Forecast (MW)
    pub demand_uncertainty_margin: Option<rust_decimal::Decimal>,
    /// Aggregate Uncertainty Margin adjustment to Scheduled Generation Bid Max Avail (MW)
    pub sched_gen_uncertainty_margin: Option<rust_decimal::Decimal>,
    /// Aggregate Uncertainty Margin adjustment to most probable VRE Forecast (MW)
    pub vre_gen_uncertainty_margin: Option<rust_decimal::Decimal>,
    /// Aggregate Auxiliary Load adjustment to uncertainty-adjusted Bid MaxAvail of all scheduled generating units (MW)
    pub sched_gen_aux_load: Option<rust_decimal::Decimal>,
    /// Cleared Generation from non energy-constrained resources - that is, excluding bidirectional units and generating units subject to daily energy limits (MW)
    pub energyunconstrained_cleared: Option<rust_decimal::Decimal>,
    /// Cleared Generation from energy-constrained resources - that is, from bidirectional units and generating units subject to daily energy limits (MW)
    pub energyconstrained_cleared: Option<rust_decimal::Decimal>,
    /// Cleared Generation (positive) or Consumption (negative) from bidirectional units (MW)
    pub bdu_cleared: Option<rust_decimal::Decimal>,
    /// Cleared Generation from semi-scheduled generating units (MW)
    pub ss_cleared: Option<rust_decimal::Decimal>,
    /// Cleared Generation from semi-scheduled solar generating units (MW)
    pub ss_solar_cleared: Option<rust_decimal::Decimal>,
    /// Cleared Generation from semi-scheduled wind generating units (MW)
    pub ss_wind_cleared: Option<rust_decimal::Decimal>,
    /// Spare Generation Capacity = max(0, Available Generation minus [Cleared Generation minus Cleared Net Interchange]) (MW)
    pub sparecapacity: Option<rust_decimal::Decimal>,
    /// Cleared Generation (MW)
    pub clearedsupply: Option<rust_decimal::Decimal>,
    /// Cleared Grid Losses (MW)
    pub clearedlosses: Option<rust_decimal::Decimal>,
    /// Cleared Demand (MW)
    pub cleareddemand: Option<rust_decimal::Decimal>,
    /// Cleared Net Export (positive) or Net Import (negative) (MW)
    pub clearednetinterchange: Option<rust_decimal::Decimal>,
    /// Supply Deficit (MW) across at all loads in the Region = Max(0, Initial Demand minus Cleared Demand) where Cleared Demand = (Cleared Generation minus Cleared Losses minus Cleared Net Interchange).Supply Deficit = Supply Deficit_RRN + Supply Deficit_NonRRN
    pub supplydeficit: Option<rust_decimal::Decimal>,
    /// Supply Deficit across all loads in the Zone that contains the Regional Reference Node (MW)
    pub supplydeficit_rrn: Option<rust_decimal::Decimal>,
    /// Supply Deficit across all loads in the Zone(s) that do not contain the Regional Reference Node (MW)
    pub supplydeficit_nonrrn: Option<rust_decimal::Decimal>,
    /// Date time this record was created
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> StpasaFnmRegionsolution1Row<'data> {
    pub fn runtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.runtype.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for StpasaFnmRegionsolution1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "STPASA";
    const TABLE_NAME: &'static str = "FNM_REGIONSOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = StpasaFnmRegionsolution1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "RUNTYPE",
        "INTERVAL_DATETIME",
        "REGIONID",
        "LORCONDITION",
        "DEFICITCONDITION",
        "INITIALDEMAND",
        "DEMAND_UNCERTAINTY_MARGIN",
        "SCHED_GEN_UNCERTAINTY_MARGIN",
        "VRE_GEN_UNCERTAINTY_MARGIN",
        "SCHED_GEN_AUX_LOAD",
        "ENERGYUNCONSTRAINED_CLEARED",
        "ENERGYCONSTRAINED_CLEARED",
        "BDU_CLEARED",
        "SS_CLEARED",
        "SS_SOLAR_CLEARED",
        "SS_WIND_CLEARED",
        "SPARECAPACITY",
        "CLEAREDSUPPLY",
        "CLEAREDLOSSES",
        "CLEAREDDEMAND",
        "CLEAREDNETINTERCHANGE",
        "SUPPLYDEFICIT",
        "SUPPLYDEFICIT_RRN",
        "SUPPLYDEFICIT_NONRRN",
        "LASTCHANGED",
    ];
    type Row<'row> = StpasaFnmRegionsolution1Row<'row>;
    type FieldMapping = StpasaFnmRegionsolution1Mapping;
    type PrimaryKey = StpasaFnmRegionsolution1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(StpasaFnmRegionsolution1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runtype: row.get_range("runtype", field_mapping.0[1])?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            lorcondition: row
                .get_opt_custom_parsed_at_idx(
                    "lorcondition",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            deficitcondition: row
                .get_opt_custom_parsed_at_idx(
                    "deficitcondition",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            initialdemand: row
                .get_opt_custom_parsed_at_idx(
                    "initialdemand",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand_uncertainty_margin: row
                .get_opt_custom_parsed_at_idx(
                    "demand_uncertainty_margin",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sched_gen_uncertainty_margin: row
                .get_opt_custom_parsed_at_idx(
                    "sched_gen_uncertainty_margin",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            vre_gen_uncertainty_margin: row
                .get_opt_custom_parsed_at_idx(
                    "vre_gen_uncertainty_margin",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sched_gen_aux_load: row
                .get_opt_custom_parsed_at_idx(
                    "sched_gen_aux_load",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energyunconstrained_cleared: row
                .get_opt_custom_parsed_at_idx(
                    "energyunconstrained_cleared",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energyconstrained_cleared: row
                .get_opt_custom_parsed_at_idx(
                    "energyconstrained_cleared",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bdu_cleared: row
                .get_opt_custom_parsed_at_idx(
                    "bdu_cleared",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_cleared: row
                .get_opt_custom_parsed_at_idx(
                    "ss_cleared",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_solar_cleared: row
                .get_opt_custom_parsed_at_idx(
                    "ss_solar_cleared",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_wind_cleared: row
                .get_opt_custom_parsed_at_idx(
                    "ss_wind_cleared",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sparecapacity: row
                .get_opt_custom_parsed_at_idx(
                    "sparecapacity",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            clearedsupply: row
                .get_opt_custom_parsed_at_idx(
                    "clearedsupply",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            clearedlosses: row
                .get_opt_custom_parsed_at_idx(
                    "clearedlosses",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cleareddemand: row
                .get_opt_custom_parsed_at_idx(
                    "cleareddemand",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            clearednetinterchange: row
                .get_opt_custom_parsed_at_idx(
                    "clearednetinterchange",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            supplydeficit: row
                .get_opt_custom_parsed_at_idx(
                    "supplydeficit",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            supplydeficit_rrn: row
                .get_opt_custom_parsed_at_idx(
                    "supplydeficit_rrn",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            supplydeficit_nonrrn: row
                .get_opt_custom_parsed_at_idx(
                    "supplydeficit_nonrrn",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[25],
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
        Ok(StpasaFnmRegionsolution1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> StpasaFnmRegionsolution1PrimaryKey {
        StpasaFnmRegionsolution1PrimaryKey {
            interval_datetime: row.interval_datetime,
            regionid: row.regionid().to_string(),
            run_datetime: row.run_datetime,
            runtype: row.runtype().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("stpasa_fnm_regionsolution_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        StpasaFnmRegionsolution1Row {
            run_datetime: row.run_datetime.clone(),
            runtype: row.runtype.clone(),
            interval_datetime: row.interval_datetime.clone(),
            regionid: row.regionid.clone(),
            lorcondition: row.lorcondition.clone(),
            deficitcondition: row.deficitcondition.clone(),
            initialdemand: row.initialdemand.clone(),
            demand_uncertainty_margin: row.demand_uncertainty_margin.clone(),
            sched_gen_uncertainty_margin: row.sched_gen_uncertainty_margin.clone(),
            vre_gen_uncertainty_margin: row.vre_gen_uncertainty_margin.clone(),
            sched_gen_aux_load: row.sched_gen_aux_load.clone(),
            energyunconstrained_cleared: row.energyunconstrained_cleared.clone(),
            energyconstrained_cleared: row.energyconstrained_cleared.clone(),
            bdu_cleared: row.bdu_cleared.clone(),
            ss_cleared: row.ss_cleared.clone(),
            ss_solar_cleared: row.ss_solar_cleared.clone(),
            ss_wind_cleared: row.ss_wind_cleared.clone(),
            sparecapacity: row.sparecapacity.clone(),
            clearedsupply: row.clearedsupply.clone(),
            clearedlosses: row.clearedlosses.clone(),
            cleareddemand: row.cleareddemand.clone(),
            clearednetinterchange: row.clearednetinterchange.clone(),
            supplydeficit: row.supplydeficit.clone(),
            supplydeficit_rrn: row.supplydeficit_rrn.clone(),
            supplydeficit_nonrrn: row.supplydeficit_nonrrn.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StpasaFnmRegionsolution1PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
    pub runtype: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for StpasaFnmRegionsolution1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmRegionsolution1Row<'data> {
    type Row<'other> = StpasaFnmRegionsolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid() == row.regionid() && self.run_datetime == row.run_datetime
            && self.runtype() == row.runtype()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for StpasaFnmRegionsolution1Row<'data> {
    type PrimaryKey = StpasaFnmRegionsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid() == key.regionid && self.run_datetime == key.run_datetime
            && self.runtype() == key.runtype
    }
}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmRegionsolution1PrimaryKey {
    type Row<'other> = StpasaFnmRegionsolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid() && self.run_datetime == row.run_datetime
            && self.runtype == row.runtype()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for StpasaFnmRegionsolution1PrimaryKey {
    type PrimaryKey = StpasaFnmRegionsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for StpasaFnmRegionsolution1 {
    type Builder = StpasaFnmRegionsolution1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runtype",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "lorcondition",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "deficitcondition",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "initialdemand",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand_uncertainty_margin",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sched_gen_uncertainty_margin",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "vre_gen_uncertainty_margin",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sched_gen_aux_load",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energyunconstrained_cleared",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energyconstrained_cleared",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bdu_cleared",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_cleared",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_solar_cleared",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_wind_cleared",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sparecapacity",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "clearedsupply",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "clearedlosses",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cleareddemand",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "clearednetinterchange",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "supplydeficit",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "supplydeficit_rrn",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "supplydeficit_nonrrn",
                    arrow::datatypes::DataType::Decimal128(12, 2),
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
        StpasaFnmRegionsolution1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runtype_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            lorcondition_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            deficitcondition_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            initialdemand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            demand_uncertainty_margin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            sched_gen_uncertainty_margin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            vre_gen_uncertainty_margin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            sched_gen_aux_load_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            energyunconstrained_cleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            energyconstrained_cleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            bdu_cleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_cleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_solar_cleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_wind_cleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            sparecapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            clearedsupply_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            clearedlosses_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            cleareddemand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            clearednetinterchange_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            supplydeficit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            supplydeficit_rrn_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            supplydeficit_nonrrn_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.runtype_array.append_value(row.runtype());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder
            .lorcondition_array
            .append_option({
                row.lorcondition
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .deficitcondition_array
            .append_option({
                row.deficitcondition
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .initialdemand_array
            .append_option({
                row.initialdemand
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .demand_uncertainty_margin_array
            .append_option({
                row.demand_uncertainty_margin
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .sched_gen_uncertainty_margin_array
            .append_option({
                row.sched_gen_uncertainty_margin
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .vre_gen_uncertainty_margin_array
            .append_option({
                row.vre_gen_uncertainty_margin
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .sched_gen_aux_load_array
            .append_option({
                row.sched_gen_aux_load
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .energyunconstrained_cleared_array
            .append_option({
                row.energyunconstrained_cleared
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .energyconstrained_cleared_array
            .append_option({
                row.energyconstrained_cleared
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .bdu_cleared_array
            .append_option({
                row.bdu_cleared
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_cleared_array
            .append_option({
                row.ss_cleared
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_solar_cleared_array
            .append_option({
                row.ss_solar_cleared
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_wind_cleared_array
            .append_option({
                row.ss_wind_cleared
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .sparecapacity_array
            .append_option({
                row.sparecapacity
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .clearedsupply_array
            .append_option({
                row.clearedsupply
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .clearedlosses_array
            .append_option({
                row.clearedlosses
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .cleareddemand_array
            .append_option({
                row.cleareddemand
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .clearednetinterchange_array
            .append_option({
                row.clearednetinterchange
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .supplydeficit_array
            .append_option({
                row.supplydeficit
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .supplydeficit_rrn_array
            .append_option({
                row.supplydeficit_rrn
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .supplydeficit_nonrrn_array
            .append_option({
                row.supplydeficit_nonrrn
                    .map(|mut val| {
                        val.rescale(2);
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lorcondition_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.deficitcondition_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.initialdemand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.demand_uncertainty_margin_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.sched_gen_uncertainty_margin_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.vre_gen_uncertainty_margin_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sched_gen_aux_load_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.energyunconstrained_cleared_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.energyconstrained_cleared_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bdu_cleared_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_cleared_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_solar_cleared_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_wind_cleared_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sparecapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clearedsupply_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clearedlosses_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cleareddemand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clearednetinterchange_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.supplydeficit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.supplydeficit_rrn_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.supplydeficit_nonrrn_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct StpasaFnmRegionsolution1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    runtype_array: arrow::array::StringDictionaryBuilder<arrow::array::types::Int32Type>,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    lorcondition_array: arrow::array::builder::Decimal128Builder,
    deficitcondition_array: arrow::array::builder::Decimal128Builder,
    initialdemand_array: arrow::array::builder::Decimal128Builder,
    demand_uncertainty_margin_array: arrow::array::builder::Decimal128Builder,
    sched_gen_uncertainty_margin_array: arrow::array::builder::Decimal128Builder,
    vre_gen_uncertainty_margin_array: arrow::array::builder::Decimal128Builder,
    sched_gen_aux_load_array: arrow::array::builder::Decimal128Builder,
    energyunconstrained_cleared_array: arrow::array::builder::Decimal128Builder,
    energyconstrained_cleared_array: arrow::array::builder::Decimal128Builder,
    bdu_cleared_array: arrow::array::builder::Decimal128Builder,
    ss_cleared_array: arrow::array::builder::Decimal128Builder,
    ss_solar_cleared_array: arrow::array::builder::Decimal128Builder,
    ss_wind_cleared_array: arrow::array::builder::Decimal128Builder,
    sparecapacity_array: arrow::array::builder::Decimal128Builder,
    clearedsupply_array: arrow::array::builder::Decimal128Builder,
    clearedlosses_array: arrow::array::builder::Decimal128Builder,
    cleareddemand_array: arrow::array::builder::Decimal128Builder,
    clearednetinterchange_array: arrow::array::builder::Decimal128Builder,
    supplydeficit_array: arrow::array::builder::Decimal128Builder,
    supplydeficit_rrn_array: arrow::array::builder::Decimal128Builder,
    supplydeficit_nonrrn_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct StpasaFnmRegionsummary1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &StpasaFnmRegionsummary1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaFnmRegionsummary1 {
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
pub struct StpasaFnmRegionsummary1Mapping([usize; 15]);
/// # Summary
///
/// ## STPASA_FNM_REGIONSUMMARY
///
/// STPASA_FNM_REGIONSUMMARY shows the summary of STPASA outcome for each region.
///
/// * Data Set Name: Stpasa
/// * File Name: Fnm Regionsummary
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
/// * INTERVAL_DATETIME
/// * REGIONID
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct StpasaFnmRegionsummary1Row<'data> {
    /// Unique Timestamp Identifier for this run, identified by the first half hour ended interval of the run
    pub run_datetime: chrono::NaiveDateTime,
    /// End date time of the interval
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region identifier
    pub regionid: core::ops::Range<usize>,
    /// Lack of Reserve Condition (LORCONDITION) >0 if a supply deficit exists in the Zone for this Region that contains its Regional Reference NodeLORCONDITION indicates the most severe condition:LORCONDITION = 3 if deficit in BASE run; elseLORCONDITION = 2 if deficit in RELIABILITY run; elseLORCONDITION = 1 if deficit in WARNING run
    pub lorcondition: Option<rust_decimal::Decimal>,
    /// Deficit Condition (DEFICITCONDITION) >0 if a supply deficit only exists in a Zone for this Region that does not contain its Regional Reference Node.DEFICITCONDITION indicates the most severe condition:DEFICITCONDITION = 3 if deficit in BASE run; elseDEFICITCONDITION = 2 if deficit in RELIABILITY run; elseDEFICITCONDITION = 1 if deficit in WARNING run
    pub deficitcondition: Option<rust_decimal::Decimal>,
    /// 50% Probability of Exceedance demand forecast (MW)
    pub demand50: Option<rust_decimal::Decimal>,
    /// 50% Probability of Exceedance demand forecast plus Aggregate Generation Forecast of all non-scheduled and exempt generation (MW)
    pub demand50_unsched_gen: Option<rust_decimal::Decimal>,
    /// Aggregate Bid MaxAvail of all scheduled generating units, scheduled bidirectional units (Gen side) and semi-scheduled generating units, with latter capped at UIGF (MW)
    pub sched_ss_gen_capacityavail: Option<rust_decimal::Decimal>,
    /// Aggregate Generation Forecast of all non-scheduled and exempt generation (MW)
    pub unsched_gen_capacityavail: Option<rust_decimal::Decimal>,
    /// Aggregate Bid PASAAvailability of all scheduled generating units and scheduled bidirectional units (Gen side) with a Bid Recall Period less than (Interval_DateTime minus Run_DateTime) plus UIGF for all semi-scheduled generating units (MW)
    pub sched_ss_gen_pasaavail: Option<rust_decimal::Decimal>,
    /// Aggregate Bid MaxAvail of all scheduled loads (MW)
    pub sched_load_capacityavail: Option<rust_decimal::Decimal>,
    /// Aggregate 50% Probability of Exceedance Unconstrained Intermittent Generation Forecast (UIGF) of all semi-scheduled generating units (MW)
    pub ss_uigf: Option<rust_decimal::Decimal>,
    /// Aggregate 50% Probability of Exceedance Unconstrained Intermittent Generation Forecast (UIGF) of all solar semi-scheduled generating units (MW)
    pub ss_solar_uigf: Option<rust_decimal::Decimal>,
    /// Aggregate 50% Probability of Exceedance Unconstrained Intermittent Generation Forecast (UIGF) of all wind semi-scheduled generating units (MW)
    pub ss_wind_uigf: Option<rust_decimal::Decimal>,
    /// Date time this record was created
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> StpasaFnmRegionsummary1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for StpasaFnmRegionsummary1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "STPASA";
    const TABLE_NAME: &'static str = "FNM_REGIONSUMMARY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = StpasaFnmRegionsummary1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "INTERVAL_DATETIME",
        "REGIONID",
        "LORCONDITION",
        "DEFICITCONDITION",
        "DEMAND50",
        "DEMAND50_UNSCHED_GEN",
        "SCHED_SS_GEN_CAPACITYAVAIL",
        "UNSCHED_GEN_CAPACITYAVAIL",
        "SCHED_SS_GEN_PASAAVAIL",
        "SCHED_LOAD_CAPACITYAVAIL",
        "SS_UIGF",
        "SS_SOLAR_UIGF",
        "SS_WIND_UIGF",
        "LASTCHANGED",
    ];
    type Row<'row> = StpasaFnmRegionsummary1Row<'row>;
    type FieldMapping = StpasaFnmRegionsummary1Mapping;
    type PrimaryKey = StpasaFnmRegionsummary1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(StpasaFnmRegionsummary1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            lorcondition: row
                .get_opt_custom_parsed_at_idx(
                    "lorcondition",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            deficitcondition: row
                .get_opt_custom_parsed_at_idx(
                    "deficitcondition",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand50: row
                .get_opt_custom_parsed_at_idx(
                    "demand50",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand50_unsched_gen: row
                .get_opt_custom_parsed_at_idx(
                    "demand50_unsched_gen",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sched_ss_gen_capacityavail: row
                .get_opt_custom_parsed_at_idx(
                    "sched_ss_gen_capacityavail",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            unsched_gen_capacityavail: row
                .get_opt_custom_parsed_at_idx(
                    "unsched_gen_capacityavail",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sched_ss_gen_pasaavail: row
                .get_opt_custom_parsed_at_idx(
                    "sched_ss_gen_pasaavail",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sched_load_capacityavail: row
                .get_opt_custom_parsed_at_idx(
                    "sched_load_capacityavail",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_uigf: row
                .get_opt_custom_parsed_at_idx(
                    "ss_uigf",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_solar_uigf: row
                .get_opt_custom_parsed_at_idx(
                    "ss_solar_uigf",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_wind_uigf: row
                .get_opt_custom_parsed_at_idx(
                    "ss_wind_uigf",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[14],
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
        Ok(StpasaFnmRegionsummary1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> StpasaFnmRegionsummary1PrimaryKey {
        StpasaFnmRegionsummary1PrimaryKey {
            interval_datetime: row.interval_datetime,
            regionid: row.regionid().to_string(),
            run_datetime: row.run_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("stpasa_fnm_regionsummary_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        StpasaFnmRegionsummary1Row {
            run_datetime: row.run_datetime.clone(),
            interval_datetime: row.interval_datetime.clone(),
            regionid: row.regionid.clone(),
            lorcondition: row.lorcondition.clone(),
            deficitcondition: row.deficitcondition.clone(),
            demand50: row.demand50.clone(),
            demand50_unsched_gen: row.demand50_unsched_gen.clone(),
            sched_ss_gen_capacityavail: row.sched_ss_gen_capacityavail.clone(),
            unsched_gen_capacityavail: row.unsched_gen_capacityavail.clone(),
            sched_ss_gen_pasaavail: row.sched_ss_gen_pasaavail.clone(),
            sched_load_capacityavail: row.sched_load_capacityavail.clone(),
            ss_uigf: row.ss_uigf.clone(),
            ss_solar_uigf: row.ss_solar_uigf.clone(),
            ss_wind_uigf: row.ss_wind_uigf.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StpasaFnmRegionsummary1PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for StpasaFnmRegionsummary1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmRegionsummary1Row<'data> {
    type Row<'other> = StpasaFnmRegionsummary1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid() == row.regionid() && self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for StpasaFnmRegionsummary1Row<'data> {
    type PrimaryKey = StpasaFnmRegionsummary1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid() == key.regionid && self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmRegionsummary1PrimaryKey {
    type Row<'other> = StpasaFnmRegionsummary1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid() && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for StpasaFnmRegionsummary1PrimaryKey {
    type PrimaryKey = StpasaFnmRegionsummary1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for StpasaFnmRegionsummary1 {
    type Builder = StpasaFnmRegionsummary1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "lorcondition",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "deficitcondition",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand50",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand50_unsched_gen",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sched_ss_gen_capacityavail",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unsched_gen_capacityavail",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sched_ss_gen_pasaavail",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sched_load_capacityavail",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_uigf",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_solar_uigf",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_wind_uigf",
                    arrow::datatypes::DataType::Decimal128(12, 2),
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
        StpasaFnmRegionsummary1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            lorcondition_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            deficitcondition_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            demand50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            demand50_unsched_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            sched_ss_gen_capacityavail_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            unsched_gen_capacityavail_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            sched_ss_gen_pasaavail_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            sched_load_capacityavail_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_uigf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_solar_uigf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_wind_uigf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder
            .lorcondition_array
            .append_option({
                row.lorcondition
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .deficitcondition_array
            .append_option({
                row.deficitcondition
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .demand50_array
            .append_option({
                row.demand50
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .demand50_unsched_gen_array
            .append_option({
                row.demand50_unsched_gen
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .sched_ss_gen_capacityavail_array
            .append_option({
                row.sched_ss_gen_capacityavail
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .unsched_gen_capacityavail_array
            .append_option({
                row.unsched_gen_capacityavail
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .sched_ss_gen_pasaavail_array
            .append_option({
                row.sched_ss_gen_pasaavail
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .sched_load_capacityavail_array
            .append_option({
                row.sched_load_capacityavail
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_uigf_array
            .append_option({
                row.ss_uigf
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_solar_uigf_array
            .append_option({
                row.ss_solar_uigf
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_wind_uigf_array
            .append_option({
                row.ss_wind_uigf
                    .map(|mut val| {
                        val.rescale(2);
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lorcondition_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.deficitcondition_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand50_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand50_unsched_gen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.sched_ss_gen_capacityavail_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.unsched_gen_capacityavail_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sched_ss_gen_pasaavail_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.sched_load_capacityavail_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_uigf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_solar_uigf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_wind_uigf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct StpasaFnmRegionsummary1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    lorcondition_array: arrow::array::builder::Decimal128Builder,
    deficitcondition_array: arrow::array::builder::Decimal128Builder,
    demand50_array: arrow::array::builder::Decimal128Builder,
    demand50_unsched_gen_array: arrow::array::builder::Decimal128Builder,
    sched_ss_gen_capacityavail_array: arrow::array::builder::Decimal128Builder,
    unsched_gen_capacityavail_array: arrow::array::builder::Decimal128Builder,
    sched_ss_gen_pasaavail_array: arrow::array::builder::Decimal128Builder,
    sched_load_capacityavail_array: arrow::array::builder::Decimal128Builder,
    ss_uigf_array: arrow::array::builder::Decimal128Builder,
    ss_solar_uigf_array: arrow::array::builder::Decimal128Builder,
    ss_wind_uigf_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct StpasaFnmZonesolution1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &StpasaFnmZonesolution1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaFnmZonesolution1 {
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
pub struct StpasaFnmZonesolution1Mapping([usize; 25]);
/// # Summary
///
/// ## STPASA_FNM_ZONESOLUTION
///
/// STPASA_FNM_ZONESOLUTION shows zone demand, cleared value of resources, spare capacity, losses for each run type and interval.
///
/// * Data Set Name: Stpasa
/// * File Name: Fnm Zonesolution
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
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
/// * RUNTYPE
/// * ZONEID
#[derive(Debug, PartialEq, Eq)]
pub struct StpasaFnmZonesolution1Row<'data> {
    /// Unique Timestamp Identifier for this run, identified by the first half hour ended interval of the run
    pub run_datetime: chrono::NaiveDateTime,
    /// Run Type (BASE, RELIABILITY, WARNING)
    pub runtype: core::ops::Range<usize>,
    /// End date time of the interval
    pub interval_datetime: chrono::NaiveDateTime,
    /// Zone identifier
    pub zoneid: core::ops::Range<usize>,
    /// Region identifier of the Region containing this Zone
    pub regionid: core::ops::Range<usize>,
    /// Lack of Reserve Condition (LORCONDITION) >0 if a supply deficit exists and this Zone contains the Regional Reference NodeLORCONDITION = 3 if deficit in BASE runLORCONDITION = 2 if deficit in RELIABILITY runLORCONDITION = 1 if deficit in WARNING run
    pub lorcondition: Option<rust_decimal::Decimal>,
    /// Deficit Condition (DEFICITCONDITION) >0 if a supply deficit exists and this Zone does not contain the Regional Reference Node.DEFICITCONDITION = 3 if deficit in BASE runDEFICITCONDITION = 2 if deficit in RELIABILITY runDEFICITCONDITION = 1 if deficit in WARNING run
    pub deficitcondition: Option<rust_decimal::Decimal>,
    /// Most probable Demand Forecast adjusted by Demand Uncertainty Margin (MW)
    pub initialdemand: Option<rust_decimal::Decimal>,
    /// Aggregate Uncertainty Margin adjustment (increase) to most probable Demand Forecast (MW)
    pub demand_uncertainty_margin: Option<rust_decimal::Decimal>,
    /// Aggregate Uncertainty Margin adjustment (decrease) to Scheduled Generation Bid Max Avail (MW)
    pub sched_gen_uncertainty_margin: Option<rust_decimal::Decimal>,
    /// Aggregate Uncertainty Margin adjustment (decrease) to most probable VRE Forecast (MW)
    pub vre_gen_uncertainty_margin: Option<rust_decimal::Decimal>,
    /// Aggregate Auxiliary Load adjustment to uncertainty-adjusted Bid MaxAvail of all scheduled generating units (MW)
    pub sched_gen_aux_load: Option<rust_decimal::Decimal>,
    /// Cleared Generation from non energy-constrained resources - that is, excluding bidirectional units and generating units subject to daily energy limits (MW)
    pub energyunconstrained_cleared: Option<rust_decimal::Decimal>,
    /// Cleared Generation from energy-constrained resources - that is, from bidirectional units and generating units subject to daily energy limits (MW)
    pub energyconstrained_cleared: Option<rust_decimal::Decimal>,
    /// Cleared Generation (positive) or Consumption (negative) from bidirectional units (MW)
    pub bdu_cleared: Option<rust_decimal::Decimal>,
    /// Cleared Generation from semi-scheduled generating units (MW)
    pub ss_cleared: Option<rust_decimal::Decimal>,
    /// Cleared Generation from semi-scheduled solar generating units (MW)
    pub ss_solar_cleared: Option<rust_decimal::Decimal>,
    /// Cleared Generation from semi-scheduled wind generating units (MW)
    pub ss_wind_cleared: Option<rust_decimal::Decimal>,
    /// Spare generation capacity = max(0, Available Generation minus [Cleared Generation minus Cleared Net Interchange]) (MW)
    pub sparecapacity: Option<rust_decimal::Decimal>,
    /// Cleared Generation (MW)
    pub clearedsupply: Option<rust_decimal::Decimal>,
    /// Cleared Grid Losses (MW)
    pub clearedlosses: Option<rust_decimal::Decimal>,
    /// Cleared Net Export (positive) or Net Import (negative) (MW)
    pub clearednetinterchange: Option<rust_decimal::Decimal>,
    /// Cleared Demand (MW)
    pub cleareddemand: Option<rust_decimal::Decimal>,
    /// Supply Deficit at loads = Max(0, Initial Demand minus Cleared Demand) where Cleared Demand = (Cleared Generation minus Cleared Losses minus Cleared Net Interchange) (MW)
    pub supplydeficit: Option<rust_decimal::Decimal>,
    /// Date time this record was created
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> StpasaFnmZonesolution1Row<'data> {
    pub fn runtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.runtype.clone())
    }
    pub fn zoneid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.zoneid.clone())
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
impl mmsdm_core::GetTable for StpasaFnmZonesolution1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "STPASA";
    const TABLE_NAME: &'static str = "FNM_ZONESOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = StpasaFnmZonesolution1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "RUNTYPE",
        "INTERVAL_DATETIME",
        "ZONEID",
        "REGIONID",
        "LORCONDITION",
        "DEFICITCONDITION",
        "INITIALDEMAND",
        "DEMAND_UNCERTAINTY_MARGIN",
        "SCHED_GEN_UNCERTAINTY_MARGIN",
        "VRE_GEN_UNCERTAINTY_MARGIN",
        "SCHED_GEN_AUX_LOAD",
        "ENERGYUNCONSTRAINED_CLEARED",
        "ENERGYCONSTRAINED_CLEARED",
        "BDU_CLEARED",
        "SS_CLEARED",
        "SS_SOLAR_CLEARED",
        "SS_WIND_CLEARED",
        "SPARECAPACITY",
        "CLEAREDSUPPLY",
        "CLEAREDLOSSES",
        "CLEAREDNETINTERCHANGE",
        "CLEAREDDEMAND",
        "SUPPLYDEFICIT",
        "LASTCHANGED",
    ];
    type Row<'row> = StpasaFnmZonesolution1Row<'row>;
    type FieldMapping = StpasaFnmZonesolution1Mapping;
    type PrimaryKey = StpasaFnmZonesolution1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(StpasaFnmZonesolution1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runtype: row.get_range("runtype", field_mapping.0[1])?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            zoneid: row.get_range("zoneid", field_mapping.0[3])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[4])?,
            lorcondition: row
                .get_opt_custom_parsed_at_idx(
                    "lorcondition",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            deficitcondition: row
                .get_opt_custom_parsed_at_idx(
                    "deficitcondition",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            initialdemand: row
                .get_opt_custom_parsed_at_idx(
                    "initialdemand",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand_uncertainty_margin: row
                .get_opt_custom_parsed_at_idx(
                    "demand_uncertainty_margin",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sched_gen_uncertainty_margin: row
                .get_opt_custom_parsed_at_idx(
                    "sched_gen_uncertainty_margin",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            vre_gen_uncertainty_margin: row
                .get_opt_custom_parsed_at_idx(
                    "vre_gen_uncertainty_margin",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sched_gen_aux_load: row
                .get_opt_custom_parsed_at_idx(
                    "sched_gen_aux_load",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energyunconstrained_cleared: row
                .get_opt_custom_parsed_at_idx(
                    "energyunconstrained_cleared",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energyconstrained_cleared: row
                .get_opt_custom_parsed_at_idx(
                    "energyconstrained_cleared",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bdu_cleared: row
                .get_opt_custom_parsed_at_idx(
                    "bdu_cleared",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_cleared: row
                .get_opt_custom_parsed_at_idx(
                    "ss_cleared",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_solar_cleared: row
                .get_opt_custom_parsed_at_idx(
                    "ss_solar_cleared",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_wind_cleared: row
                .get_opt_custom_parsed_at_idx(
                    "ss_wind_cleared",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sparecapacity: row
                .get_opt_custom_parsed_at_idx(
                    "sparecapacity",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            clearedsupply: row
                .get_opt_custom_parsed_at_idx(
                    "clearedsupply",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            clearedlosses: row
                .get_opt_custom_parsed_at_idx(
                    "clearedlosses",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            clearednetinterchange: row
                .get_opt_custom_parsed_at_idx(
                    "clearednetinterchange",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cleareddemand: row
                .get_opt_custom_parsed_at_idx(
                    "cleareddemand",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            supplydeficit: row
                .get_opt_custom_parsed_at_idx(
                    "supplydeficit",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[24],
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
        Ok(StpasaFnmZonesolution1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> StpasaFnmZonesolution1PrimaryKey {
        StpasaFnmZonesolution1PrimaryKey {
            interval_datetime: row.interval_datetime,
            run_datetime: row.run_datetime,
            runtype: row.runtype().to_string(),
            zoneid: row.zoneid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("stpasa_fnm_zonesolution_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        StpasaFnmZonesolution1Row {
            run_datetime: row.run_datetime.clone(),
            runtype: row.runtype.clone(),
            interval_datetime: row.interval_datetime.clone(),
            zoneid: row.zoneid.clone(),
            regionid: row.regionid.clone(),
            lorcondition: row.lorcondition.clone(),
            deficitcondition: row.deficitcondition.clone(),
            initialdemand: row.initialdemand.clone(),
            demand_uncertainty_margin: row.demand_uncertainty_margin.clone(),
            sched_gen_uncertainty_margin: row.sched_gen_uncertainty_margin.clone(),
            vre_gen_uncertainty_margin: row.vre_gen_uncertainty_margin.clone(),
            sched_gen_aux_load: row.sched_gen_aux_load.clone(),
            energyunconstrained_cleared: row.energyunconstrained_cleared.clone(),
            energyconstrained_cleared: row.energyconstrained_cleared.clone(),
            bdu_cleared: row.bdu_cleared.clone(),
            ss_cleared: row.ss_cleared.clone(),
            ss_solar_cleared: row.ss_solar_cleared.clone(),
            ss_wind_cleared: row.ss_wind_cleared.clone(),
            sparecapacity: row.sparecapacity.clone(),
            clearedsupply: row.clearedsupply.clone(),
            clearedlosses: row.clearedlosses.clone(),
            clearednetinterchange: row.clearednetinterchange.clone(),
            cleareddemand: row.cleareddemand.clone(),
            supplydeficit: row.supplydeficit.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StpasaFnmZonesolution1PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
    pub runtype: alloc::string::String,
    pub zoneid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for StpasaFnmZonesolution1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmZonesolution1Row<'data> {
    type Row<'other> = StpasaFnmZonesolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime && self.runtype() == row.runtype()
            && self.zoneid() == row.zoneid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for StpasaFnmZonesolution1Row<'data> {
    type PrimaryKey = StpasaFnmZonesolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime && self.runtype() == key.runtype
            && self.zoneid() == key.zoneid
    }
}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmZonesolution1PrimaryKey {
    type Row<'other> = StpasaFnmZonesolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime && self.runtype == row.runtype()
            && self.zoneid == row.zoneid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for StpasaFnmZonesolution1PrimaryKey {
    type PrimaryKey = StpasaFnmZonesolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime && self.runtype == key.runtype
            && self.zoneid == key.zoneid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for StpasaFnmZonesolution1 {
    type Builder = StpasaFnmZonesolution1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runtype",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "zoneid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lorcondition",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "deficitcondition",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "initialdemand",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand_uncertainty_margin",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sched_gen_uncertainty_margin",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "vre_gen_uncertainty_margin",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sched_gen_aux_load",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energyunconstrained_cleared",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energyconstrained_cleared",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bdu_cleared",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_cleared",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_solar_cleared",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_wind_cleared",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sparecapacity",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "clearedsupply",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "clearedlosses",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "clearednetinterchange",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cleareddemand",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "supplydeficit",
                    arrow::datatypes::DataType::Decimal128(12, 2),
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
        StpasaFnmZonesolution1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runtype_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            zoneid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            regionid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            lorcondition_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            deficitcondition_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            initialdemand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            demand_uncertainty_margin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            sched_gen_uncertainty_margin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            vre_gen_uncertainty_margin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            sched_gen_aux_load_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            energyunconstrained_cleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            energyconstrained_cleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            bdu_cleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_cleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_solar_cleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_wind_cleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            sparecapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            clearedsupply_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            clearedlosses_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            clearednetinterchange_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            cleareddemand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            supplydeficit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder.runtype_array.append_value(row.runtype());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.zoneid_array.append_value(row.zoneid());
        builder.regionid_array.append_option(row.regionid());
        builder
            .lorcondition_array
            .append_option({
                row.lorcondition
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .deficitcondition_array
            .append_option({
                row.deficitcondition
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .initialdemand_array
            .append_option({
                row.initialdemand
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .demand_uncertainty_margin_array
            .append_option({
                row.demand_uncertainty_margin
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .sched_gen_uncertainty_margin_array
            .append_option({
                row.sched_gen_uncertainty_margin
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .vre_gen_uncertainty_margin_array
            .append_option({
                row.vre_gen_uncertainty_margin
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .sched_gen_aux_load_array
            .append_option({
                row.sched_gen_aux_load
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .energyunconstrained_cleared_array
            .append_option({
                row.energyunconstrained_cleared
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .energyconstrained_cleared_array
            .append_option({
                row.energyconstrained_cleared
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .bdu_cleared_array
            .append_option({
                row.bdu_cleared
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_cleared_array
            .append_option({
                row.ss_cleared
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_solar_cleared_array
            .append_option({
                row.ss_solar_cleared
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_wind_cleared_array
            .append_option({
                row.ss_wind_cleared
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .sparecapacity_array
            .append_option({
                row.sparecapacity
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .clearedsupply_array
            .append_option({
                row.clearedsupply
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .clearedlosses_array
            .append_option({
                row.clearedlosses
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .clearednetinterchange_array
            .append_option({
                row.clearednetinterchange
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .cleareddemand_array
            .append_option({
                row.cleareddemand
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .supplydeficit_array
            .append_option({
                row.supplydeficit
                    .map(|mut val| {
                        val.rescale(2);
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.zoneid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lorcondition_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.deficitcondition_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.initialdemand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.demand_uncertainty_margin_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.sched_gen_uncertainty_margin_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.vre_gen_uncertainty_margin_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sched_gen_aux_load_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.energyunconstrained_cleared_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.energyconstrained_cleared_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bdu_cleared_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_cleared_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_solar_cleared_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_wind_cleared_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sparecapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clearedsupply_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clearedlosses_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clearednetinterchange_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cleareddemand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.supplydeficit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct StpasaFnmZonesolution1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    runtype_array: arrow::array::StringDictionaryBuilder<arrow::array::types::Int32Type>,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    zoneid_array: arrow::array::StringDictionaryBuilder<arrow::array::types::Int32Type>,
    regionid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    lorcondition_array: arrow::array::builder::Decimal128Builder,
    deficitcondition_array: arrow::array::builder::Decimal128Builder,
    initialdemand_array: arrow::array::builder::Decimal128Builder,
    demand_uncertainty_margin_array: arrow::array::builder::Decimal128Builder,
    sched_gen_uncertainty_margin_array: arrow::array::builder::Decimal128Builder,
    vre_gen_uncertainty_margin_array: arrow::array::builder::Decimal128Builder,
    sched_gen_aux_load_array: arrow::array::builder::Decimal128Builder,
    energyunconstrained_cleared_array: arrow::array::builder::Decimal128Builder,
    energyconstrained_cleared_array: arrow::array::builder::Decimal128Builder,
    bdu_cleared_array: arrow::array::builder::Decimal128Builder,
    ss_cleared_array: arrow::array::builder::Decimal128Builder,
    ss_solar_cleared_array: arrow::array::builder::Decimal128Builder,
    ss_wind_cleared_array: arrow::array::builder::Decimal128Builder,
    sparecapacity_array: arrow::array::builder::Decimal128Builder,
    clearedsupply_array: arrow::array::builder::Decimal128Builder,
    clearedlosses_array: arrow::array::builder::Decimal128Builder,
    clearednetinterchange_array: arrow::array::builder::Decimal128Builder,
    cleareddemand_array: arrow::array::builder::Decimal128Builder,
    supplydeficit_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct StpasaFnmZonesummary1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &StpasaFnmZonesummary1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaFnmZonesummary1 {
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
pub struct StpasaFnmZonesummary1Mapping([usize; 16]);
/// # Summary
///
/// ## STPASA_FNM_ZONESUMMARY
///
/// STPASA_FNM_ZONESUMMARY shows the summary of STPASA outcome for each zone.
///
/// * Data Set Name: Stpasa
/// * File Name: Fnm Zonesummary
/// * Data Version: 1
///
/// # Description
/// STPASA_INTERCONNECTORSOLN is public so is available to all participants.SourceSTPASA_INTERCONNECTORSOLN is updated each STPASA run (i.e. every 2 hours).VolumeRows per day: 576Mb per month: 4
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
/// * ZONEID
#[derive(Debug, PartialEq, Eq)]
pub struct StpasaFnmZonesummary1Row<'data> {
    /// Unique Timestamp Identifier for this run, identified by the first half hour ended interval of the run
    pub run_datetime: chrono::NaiveDateTime,
    /// End date time of the interval
    pub interval_datetime: chrono::NaiveDateTime,
    /// Zone identifier
    pub zoneid: core::ops::Range<usize>,
    /// Region identifier of the Region containing this Zone
    pub regionid: core::ops::Range<usize>,
    /// Lack of Reserve Condition (LORCONDITION) >0 if a supply deficit exists and this Zone contains the Regional Reference NodeLORCONDITION indicates the most severe condition:LORCONDITION = 3 if deficit in BASE run; elseLORCONDITION = 2 if deficit in RELIABILITY run; elseLORCONDITION = 1 if deficit in WARNING run
    pub lorcondition: Option<rust_decimal::Decimal>,
    /// Deficit Condition (DEFICITCONDITION) >0 if a supply deficit only exists in a Zone for this Region that does not contain the Regional Reference NodeDEFICITCONDITION indicates the most severe condition:DEFICITCONDITION = 3 if deficit in BASE run; elseDEFICITCONDITION = 2 if deficit in RELIABILITY run; elseDEFICITCONDITION = 1 if deficit in WARNING run
    pub deficitcondition: Option<rust_decimal::Decimal>,
    /// 50% Probability of Exceedance demand forecast (MW)
    pub demand50: Option<rust_decimal::Decimal>,
    /// 50% Probability of Exceedance demand forecast plus Aggregate Generation Forecast of all non-scheduled and exempt generation (MW)
    pub demand50_unsched_gen: Option<rust_decimal::Decimal>,
    /// Aggregate Bid MaxAvail of all scheduled generating units, scheduled bidirectional units (Gen side) and semi-scheduled generating units, with latter capped at UIGF (MW)
    pub sched_ss_gen_capacityavail: Option<rust_decimal::Decimal>,
    /// Aggregate Generation Forecast of all non-scheduled and exempt generation (MW)
    pub unsched_gen_capacityavail: Option<rust_decimal::Decimal>,
    /// Aggregate Bid PASAAvailability of all scheduled generating units and scheduled bidirectional units (Gen side) with a Bid Recall Period less than (Interval_DateTime minus Run_DateTime) plus UIGF for all semi-scheduled generating units (MW)
    pub sched_ss_gen_pasaavail: Option<rust_decimal::Decimal>,
    /// Aggregate Bid MaxAvail of all scheduled loads (MW)
    pub sched_load_capacityavail: Option<rust_decimal::Decimal>,
    /// Aggregate 50% Probability of Exceedance Unconstrained Intermittent Generation Forecast (UIGF) of all semi-scheduled generating units (MW)
    pub ss_uigf: Option<rust_decimal::Decimal>,
    /// Aggregate 50% Probability of Exceedance Unconstrained Intermittent Generation Forecast (UIGF) of all solar semi-scheduled generating units (MW)
    pub ss_solar_uigf: Option<rust_decimal::Decimal>,
    /// Aggregate 50% Probability of Exceedance Unconstrained Intermittent Generation Forecast (UIGF) of all wind semi-scheduled generating units (MW)
    pub ss_wind_uigf: Option<rust_decimal::Decimal>,
    /// Date time this record was created
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> StpasaFnmZonesummary1Row<'data> {
    pub fn zoneid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.zoneid.clone())
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
impl mmsdm_core::GetTable for StpasaFnmZonesummary1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "STPASA";
    const TABLE_NAME: &'static str = "FNM_ZONESUMMARY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = StpasaFnmZonesummary1Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "INTERVAL_DATETIME",
        "ZONEID",
        "REGIONID",
        "LORCONDITION",
        "DEFICITCONDITION",
        "DEMAND50",
        "DEMAND50_UNSCHED_GEN",
        "SCHED_SS_GEN_CAPACITYAVAIL",
        "UNSCHED_GEN_CAPACITYAVAIL",
        "SCHED_SS_GEN_PASAAVAIL",
        "SCHED_LOAD_CAPACITYAVAIL",
        "SS_UIGF",
        "SS_SOLAR_UIGF",
        "SS_WIND_UIGF",
        "LASTCHANGED",
    ];
    type Row<'row> = StpasaFnmZonesummary1Row<'row>;
    type FieldMapping = StpasaFnmZonesummary1Mapping;
    type PrimaryKey = StpasaFnmZonesummary1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(StpasaFnmZonesummary1Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            zoneid: row.get_range("zoneid", field_mapping.0[2])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[3])?,
            lorcondition: row
                .get_opt_custom_parsed_at_idx(
                    "lorcondition",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            deficitcondition: row
                .get_opt_custom_parsed_at_idx(
                    "deficitcondition",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand50: row
                .get_opt_custom_parsed_at_idx(
                    "demand50",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand50_unsched_gen: row
                .get_opt_custom_parsed_at_idx(
                    "demand50_unsched_gen",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sched_ss_gen_capacityavail: row
                .get_opt_custom_parsed_at_idx(
                    "sched_ss_gen_capacityavail",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            unsched_gen_capacityavail: row
                .get_opt_custom_parsed_at_idx(
                    "unsched_gen_capacityavail",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sched_ss_gen_pasaavail: row
                .get_opt_custom_parsed_at_idx(
                    "sched_ss_gen_pasaavail",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sched_load_capacityavail: row
                .get_opt_custom_parsed_at_idx(
                    "sched_load_capacityavail",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_uigf: row
                .get_opt_custom_parsed_at_idx(
                    "ss_uigf",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_solar_uigf: row
                .get_opt_custom_parsed_at_idx(
                    "ss_solar_uigf",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_wind_uigf: row
                .get_opt_custom_parsed_at_idx(
                    "ss_wind_uigf",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[15],
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
        Ok(StpasaFnmZonesummary1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> StpasaFnmZonesummary1PrimaryKey {
        StpasaFnmZonesummary1PrimaryKey {
            interval_datetime: row.interval_datetime,
            run_datetime: row.run_datetime,
            zoneid: row.zoneid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("stpasa_fnm_zonesummary_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        StpasaFnmZonesummary1Row {
            run_datetime: row.run_datetime.clone(),
            interval_datetime: row.interval_datetime.clone(),
            zoneid: row.zoneid.clone(),
            regionid: row.regionid.clone(),
            lorcondition: row.lorcondition.clone(),
            deficitcondition: row.deficitcondition.clone(),
            demand50: row.demand50.clone(),
            demand50_unsched_gen: row.demand50_unsched_gen.clone(),
            sched_ss_gen_capacityavail: row.sched_ss_gen_capacityavail.clone(),
            unsched_gen_capacityavail: row.unsched_gen_capacityavail.clone(),
            sched_ss_gen_pasaavail: row.sched_ss_gen_pasaavail.clone(),
            sched_load_capacityavail: row.sched_load_capacityavail.clone(),
            ss_uigf: row.ss_uigf.clone(),
            ss_solar_uigf: row.ss_solar_uigf.clone(),
            ss_wind_uigf: row.ss_wind_uigf.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StpasaFnmZonesummary1PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
    pub zoneid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for StpasaFnmZonesummary1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmZonesummary1Row<'data> {
    type Row<'other> = StpasaFnmZonesummary1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime && self.zoneid() == row.zoneid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for StpasaFnmZonesummary1Row<'data> {
    type PrimaryKey = StpasaFnmZonesummary1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime && self.zoneid() == key.zoneid
    }
}
impl<'data> mmsdm_core::CompareWithRow for StpasaFnmZonesummary1PrimaryKey {
    type Row<'other> = StpasaFnmZonesummary1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime && self.zoneid == row.zoneid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for StpasaFnmZonesummary1PrimaryKey {
    type PrimaryKey = StpasaFnmZonesummary1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime && self.zoneid == key.zoneid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for StpasaFnmZonesummary1 {
    type Builder = StpasaFnmZonesummary1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "zoneid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lorcondition",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "deficitcondition",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand50",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand50_unsched_gen",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sched_ss_gen_capacityavail",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unsched_gen_capacityavail",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sched_ss_gen_pasaavail",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sched_load_capacityavail",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_uigf",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_solar_uigf",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_wind_uigf",
                    arrow::datatypes::DataType::Decimal128(12, 2),
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
        StpasaFnmZonesummary1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            zoneid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            regionid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            lorcondition_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            deficitcondition_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            demand50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            demand50_unsched_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            sched_ss_gen_capacityavail_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            unsched_gen_capacityavail_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            sched_ss_gen_pasaavail_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            sched_load_capacityavail_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_uigf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_solar_uigf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_wind_uigf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.zoneid_array.append_value(row.zoneid());
        builder.regionid_array.append_option(row.regionid());
        builder
            .lorcondition_array
            .append_option({
                row.lorcondition
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .deficitcondition_array
            .append_option({
                row.deficitcondition
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .demand50_array
            .append_option({
                row.demand50
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .demand50_unsched_gen_array
            .append_option({
                row.demand50_unsched_gen
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .sched_ss_gen_capacityavail_array
            .append_option({
                row.sched_ss_gen_capacityavail
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .unsched_gen_capacityavail_array
            .append_option({
                row.unsched_gen_capacityavail
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .sched_ss_gen_pasaavail_array
            .append_option({
                row.sched_ss_gen_pasaavail
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .sched_load_capacityavail_array
            .append_option({
                row.sched_load_capacityavail
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_uigf_array
            .append_option({
                row.ss_uigf
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_solar_uigf_array
            .append_option({
                row.ss_solar_uigf
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_wind_uigf_array
            .append_option({
                row.ss_wind_uigf
                    .map(|mut val| {
                        val.rescale(2);
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.zoneid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lorcondition_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.deficitcondition_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand50_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand50_unsched_gen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.sched_ss_gen_capacityavail_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.unsched_gen_capacityavail_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sched_ss_gen_pasaavail_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.sched_load_capacityavail_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_uigf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_solar_uigf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_wind_uigf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct StpasaFnmZonesummary1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    zoneid_array: arrow::array::StringDictionaryBuilder<arrow::array::types::Int32Type>,
    regionid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    lorcondition_array: arrow::array::builder::Decimal128Builder,
    deficitcondition_array: arrow::array::builder::Decimal128Builder,
    demand50_array: arrow::array::builder::Decimal128Builder,
    demand50_unsched_gen_array: arrow::array::builder::Decimal128Builder,
    sched_ss_gen_capacityavail_array: arrow::array::builder::Decimal128Builder,
    unsched_gen_capacityavail_array: arrow::array::builder::Decimal128Builder,
    sched_ss_gen_pasaavail_array: arrow::array::builder::Decimal128Builder,
    sched_load_capacityavail_array: arrow::array::builder::Decimal128Builder,
    ss_uigf_array: arrow::array::builder::Decimal128Builder,
    ss_solar_uigf_array: arrow::array::builder::Decimal128Builder,
    ss_wind_uigf_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct StpasaInterconnectorsoln3 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &StpasaInterconnectorsoln3Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaInterconnectorsoln3 {
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
pub struct StpasaInterconnectorsoln3Mapping([usize; 13]);
/// # Summary
///
/// ## STPASA_INTERCONNECTORSOLN
///
/// STPASA_INTERCONNECTORSOLN shows the results of the capacity evaluation for Interconnectors, including the calculated limits for the interval.
///
/// * Data Set Name: Stpasa
/// * File Name: Interconnectorsoln
/// * Data Version: 3
///
/// # Description
/// STPASA_INTERCONNECTORSOLN is public so is available to all participants.SourceSTPASA_INTERCONNECTORSOLN is updated each STPASA run (i.e. every 2 hours).VolumeRows per day: 576Mb per month: 4
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * INTERCONNECTORID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
/// * RUNTYPE
/// * STUDYREGIONID
#[derive(Debug, PartialEq, Eq)]
pub struct StpasaInterconnectorsoln3Row<'data> {
    /// Unique Timestamp Identifier for this study
    pub run_datetime: chrono::NaiveDateTime,
    /// The unique identifier for the interval within this study
    pub interval_datetime: chrono::NaiveDateTime,
    /// Interconnector Identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// Interconnector loading level (MW) that can be reached in case of capacity scarcity in neighbouring regions subject to network and energy constraints
    pub capacitymwflow: Option<rust_decimal::Decimal>,
    /// Capacity adequacy assessment marginal value, 0 if not binding
    pub capacitymarginalvalue: Option<rust_decimal::Decimal>,
    /// Capacity adequacy assessment violation degree for interconnector capacity; 0 if not violating
    pub capacityviolationdegree: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of exporting energy on the basis of invoked constraints and static interconnector export limit
    pub calculatedexportlimit: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of importing energy on the basis of invoked constraints and static interconnector import limit. Note unlike the input interconnector import limit this is a directional quantity and should be defined with respect to the interconnector flow.
    pub calculatedimportlimit: Option<rust_decimal::Decimal>,
    /// Last changed date of this record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Type of run. Values are RELIABILITY_LRC, OUTAGE_LRC and LOR. Note that the STPASA RELIABILITY_LRC and OUTAGE_LRC Run Types are discontinued from 31 July 2025, with only the LOR Run Type reported.
    pub runtype: core::ops::Range<usize>,
    /// ID of the constraint that sets the Interconnector Export Limit
    pub exportlimitconstraintid: core::ops::Range<usize>,
    /// ID of the constraint that sets the Interconnector Import Limit
    pub importlimitconstraintid: core::ops::Range<usize>,
    /// Primary Region for LP Solve (or MARKET if none).
    pub studyregionid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> StpasaInterconnectorsoln3Row<'data> {
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn runtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.runtype.clone())
    }
    pub fn exportlimitconstraintid(&self) -> Option<&str> {
        if self.exportlimitconstraintid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.exportlimitconstraintid.clone(),
                ),
            )
        }
    }
    pub fn importlimitconstraintid(&self) -> Option<&str> {
        if self.importlimitconstraintid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.importlimitconstraintid.clone(),
                ),
            )
        }
    }
    pub fn studyregionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.studyregionid.clone())
    }
}
impl mmsdm_core::GetTable for StpasaInterconnectorsoln3 {
    const VERSION: i32 = 3;
    const DATA_SET_NAME: &'static str = "STPASA";
    const TABLE_NAME: &'static str = "INTERCONNECTORSOLN";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = StpasaInterconnectorsoln3Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "INTERVAL_DATETIME",
        "INTERCONNECTORID",
        "CAPACITYMWFLOW",
        "CAPACITYMARGINALVALUE",
        "CAPACITYVIOLATIONDEGREE",
        "CALCULATEDEXPORTLIMIT",
        "CALCULATEDIMPORTLIMIT",
        "LASTCHANGED",
        "RUNTYPE",
        "EXPORTLIMITCONSTRAINTID",
        "IMPORTLIMITCONSTRAINTID",
        "STUDYREGIONID",
    ];
    type Row<'row> = StpasaInterconnectorsoln3Row<'row>;
    type FieldMapping = StpasaInterconnectorsoln3Mapping;
    type PrimaryKey = StpasaInterconnectorsoln3PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(StpasaInterconnectorsoln3Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[2])?,
            capacitymwflow: row
                .get_opt_custom_parsed_at_idx(
                    "capacitymwflow",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            capacitymarginalvalue: row
                .get_opt_custom_parsed_at_idx(
                    "capacitymarginalvalue",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            capacityviolationdegree: row
                .get_opt_custom_parsed_at_idx(
                    "capacityviolationdegree",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            calculatedexportlimit: row
                .get_opt_custom_parsed_at_idx(
                    "calculatedexportlimit",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            calculatedimportlimit: row
                .get_opt_custom_parsed_at_idx(
                    "calculatedimportlimit",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runtype: row.get_range("runtype", field_mapping.0[9])?,
            exportlimitconstraintid: row
                .get_opt_range("exportlimitconstraintid", field_mapping.0[10])?,
            importlimitconstraintid: row
                .get_opt_range("importlimitconstraintid", field_mapping.0[11])?,
            studyregionid: row.get_range("studyregionid", field_mapping.0[12])?,
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
        Ok(StpasaInterconnectorsoln3Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> StpasaInterconnectorsoln3PrimaryKey {
        StpasaInterconnectorsoln3PrimaryKey {
            interconnectorid: row.interconnectorid().to_string(),
            interval_datetime: row.interval_datetime,
            run_datetime: row.run_datetime,
            runtype: row.runtype().to_string(),
            studyregionid: row.studyregionid().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("stpasa_interconnectorsoln_v3_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        StpasaInterconnectorsoln3Row {
            run_datetime: row.run_datetime.clone(),
            interval_datetime: row.interval_datetime.clone(),
            interconnectorid: row.interconnectorid.clone(),
            capacitymwflow: row.capacitymwflow.clone(),
            capacitymarginalvalue: row.capacitymarginalvalue.clone(),
            capacityviolationdegree: row.capacityviolationdegree.clone(),
            calculatedexportlimit: row.calculatedexportlimit.clone(),
            calculatedimportlimit: row.calculatedimportlimit.clone(),
            lastchanged: row.lastchanged.clone(),
            runtype: row.runtype.clone(),
            exportlimitconstraintid: row.exportlimitconstraintid.clone(),
            importlimitconstraintid: row.importlimitconstraintid.clone(),
            studyregionid: row.studyregionid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StpasaInterconnectorsoln3PrimaryKey {
    pub interconnectorid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
    pub runtype: alloc::string::String,
    pub studyregionid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for StpasaInterconnectorsoln3PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for StpasaInterconnectorsoln3Row<'data> {
    type Row<'other> = StpasaInterconnectorsoln3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interconnectorid() == row.interconnectorid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime && self.runtype() == row.runtype()
            && self.studyregionid() == row.studyregionid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for StpasaInterconnectorsoln3Row<'data> {
    type PrimaryKey = StpasaInterconnectorsoln3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid() == key.interconnectorid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime && self.runtype() == key.runtype
            && self.studyregionid() == key.studyregionid
    }
}
impl<'data> mmsdm_core::CompareWithRow for StpasaInterconnectorsoln3PrimaryKey {
    type Row<'other> = StpasaInterconnectorsoln3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interconnectorid == row.interconnectorid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime && self.runtype == row.runtype()
            && self.studyregionid == row.studyregionid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for StpasaInterconnectorsoln3PrimaryKey {
    type PrimaryKey = StpasaInterconnectorsoln3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime && self.runtype == key.runtype
            && self.studyregionid == key.studyregionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for StpasaInterconnectorsoln3 {
    type Builder = StpasaInterconnectorsoln3Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int16),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "capacitymwflow",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "capacitymarginalvalue",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "capacityviolationdegree",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "calculatedexportlimit",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "calculatedimportlimit",
                    arrow::datatypes::DataType::Decimal128(12, 2),
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
                    "runtype",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "exportlimitconstraintid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "importlimitconstraintid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "studyregionid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        StpasaInterconnectorsoln3Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interconnectorid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int16Type,
            >::new(),
            capacitymwflow_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            capacitymarginalvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            capacityviolationdegree_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            calculatedexportlimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            calculatedimportlimit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runtype_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            exportlimitconstraintid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            importlimitconstraintid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            studyregionid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder
            .capacitymwflow_array
            .append_option({
                row.capacitymwflow
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .capacitymarginalvalue_array
            .append_option({
                row.capacitymarginalvalue
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .capacityviolationdegree_array
            .append_option({
                row.capacityviolationdegree
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .calculatedexportlimit_array
            .append_option({
                row.calculatedexportlimit
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .calculatedimportlimit_array
            .append_option({
                row.calculatedimportlimit
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder.runtype_array.append_value(row.runtype());
        builder
            .exportlimitconstraintid_array
            .append_option(row.exportlimitconstraintid());
        builder
            .importlimitconstraintid_array
            .append_option(row.importlimitconstraintid());
        builder.studyregionid_array.append_value(row.studyregionid());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.capacitymwflow_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.capacitymarginalvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.capacityviolationdegree_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.calculatedexportlimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.calculatedimportlimit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exportlimitconstraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.importlimitconstraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.studyregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct StpasaInterconnectorsoln3Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interconnectorid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int16Type,
    >,
    capacitymwflow_array: arrow::array::builder::Decimal128Builder,
    capacitymarginalvalue_array: arrow::array::builder::Decimal128Builder,
    capacityviolationdegree_array: arrow::array::builder::Decimal128Builder,
    calculatedexportlimit_array: arrow::array::builder::Decimal128Builder,
    calculatedimportlimit_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    runtype_array: arrow::array::StringDictionaryBuilder<arrow::array::types::Int32Type>,
    exportlimitconstraintid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    importlimitconstraintid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
    studyregionid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int32Type,
    >,
}
pub struct StpasaRegionsolution7 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &StpasaRegionsolution7Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaRegionsolution7 {
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
pub struct StpasaRegionsolution7Mapping([usize; 45]);
/// # Summary
///
/// ## STPASA_REGIONSOLUTION
///
/// STPASA_REGIONSOLUTION shows the results of the regional capacity, maximum surplus reserve and maximum spare capacity evaluations for each period of the study. Note that the RELIABILITY_LRC and OUTAGE_LRC Run Types are no longer reported from 31 July 2025.
///
/// * Data Set Name: Stpasa
/// * File Name: Regionsolution
/// * Data Version: 7
///
/// # Description
/// STPASA_REGIONSOLUTION is public so is available to all participants.SourceSTPASA_REGIONSOLUTION is updated each STPASA run (i.e every 2 hours).VolumeRows per day: 480Mb per month: 8
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
/// * RUN_DATETIME
/// * RUNTYPE
#[derive(Debug, PartialEq, Eq)]
pub struct StpasaRegionsolution7Row<'data> {
    /// Unique Timestamp Identifier for this run, identified by the nominal start time of the run.
    pub run_datetime: chrono::NaiveDateTime,
    /// End date time of the interval.
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// 10% Probability of Exceedance demand forecast.
    pub demand10: Option<rust_decimal::Decimal>,
    /// 50% Probability of Exceedance demand forecast.
    pub demand50: Option<rust_decimal::Decimal>,
    /// 90% Probability of Exceedance demand forecast.
    pub demand90: Option<rust_decimal::Decimal>,
    /// Reserve Requirement (MW). This field is not populated after 30 July 2025.
    pub reservereq: Option<rust_decimal::Decimal>,
    /// Demand + Reserve requirements (MW). This field is not populated after 30 July 2025.
    pub capacityreq: Option<rust_decimal::Decimal>,
    /// Sum of: (Region Demand50)/Period (sum by trading day, entered in first period of trading day, GWh).
    pub energyreqdemand50: Option<rust_decimal::Decimal>,
    /// Aggregate generation + WDR capacity from Non-Energy Constrained plant subjected to restrictions due to network constraints.
    pub unconstrainedcapacity: Option<rust_decimal::Decimal>,
    /// Aggregate generation + WDR capacity from Energy Constrained plant subjected to restrictions due to network constraints.
    pub constrainedcapacity: Option<rust_decimal::Decimal>,
    /// Net export (MW) out of this region in the LOR evaluation. Export if >0, Import if <0. This value is the same as LORNETINTERCHANGEUNDERSCARCITY.
    pub netinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    /// Regional surplus capacity (MW), +/- values indicate surplus/deficit capacity respectively. This value reflects Regional LOR reserve.
    pub surpluscapacity: Option<rust_decimal::Decimal>,
    /// Regional surplus reserve (MW). This value also reflects Regional LOR reserve. Note: For LOR runs, RESERVEREQ requirement input is not used.
    pub surplusreserve: Option<rust_decimal::Decimal>,
    /// Regional reserve condition from LRC run. This field is not populated after 30 July 2025.
    pub reservecondition: Option<rust_decimal::Decimal>,
    /// Maximum Surplus Reserve (MW) evaluated for this region from LRC runs. This field is no longer populated.
    pub maxsurplusreserve: Option<rust_decimal::Decimal>,
    /// Maximum Spare Capacity (MW) evaluated for this region. Calculated for each region in turn. This value reflects Regional LOR reserve.
    pub maxsparecapacity: Option<rust_decimal::Decimal>,
    /// The LOR Condition determined from the Maximum Spare Capacity value: 0 - no condition, 1 - LOR1 condition, 2 - LOR2 condition, 3 - LOR3 condition
    pub lorcondition: Option<rust_decimal::Decimal>,
    /// Sum of MAXAVAIL quantities offered by all Scheduled units and Availability of all semi-scheduled units limited by MAXAVAIL in a given Region for a given PERIODID
    pub aggregatecapacityavailable: Option<rust_decimal::Decimal>,
    /// Sum of  MAXAVAIL quantities bid by of all Scheduled Loads in a given Region for a given PERIODID.
    pub aggregatescheduledload: Option<rust_decimal::Decimal>,
    /// Date time this record was created.
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Sum of PASAAVAILABILITY for all scheduled generating units and scheduled bidirectional units (Gen side) with a Recall_Period <= 24 hours plus the sum of Unconstrained Intermittent Generation Forecasts (UIGF) for all semi-scheduled generating units. For the RELIABILITY_LRC and OUTAGE_LRC runs, UIGF is the POE90 forecast. For the LOR Run, UIGF is the POE50 forecast. Note that the RELIABILITY_LRC and OUTAGE_LRC Run Types are discontinued from 31 July 2025. From March 2026, AGGREGATEPASAAVAILABILITY changes from that with Recall_Period <= 24 to that achievable by the relevant INTERVAL_DATETIME if recalled at the start of the run.
    pub aggregatepasaavailability: Option<rust_decimal::Decimal>,
    /// Type of run. Values are RELIABILITY_LRC, OUTAGE_LRC and LOR. Note that the STPASA RELIABILITY_LRC and OUTAGE_LRC Run Types are discontinued from 31 July 2025, with only the LOR Run Type reported.
    pub runtype: core::ops::Range<usize>,
    /// Energy (GWh) required for this energy block based on the 10% probability of exceedance demand. Listed in the first interval of the energy block
    pub energyreqdemand10: Option<rust_decimal::Decimal>,
    /// Region Reserve Level for LOR1 used. Can be static value or calculated value if an interconnector is a credible contingency
    pub calculatedlor1level: Option<rust_decimal::Decimal>,
    /// Region Reserve Level for LOR2 used. Can be static value or calculated value if an interconnector is a credible contingency
    pub calculatedlor2level: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the region for this interval from the MSR assessment. This field is no longer populated.
    pub msrnetinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the region for this interval from the LOR assessment
    pub lornetinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    /// Allowance made for non-scheduled generation in the demand forecast (MW).
    pub totalintermittentgeneration: Option<rust_decimal::Decimal>,
    /// Sum of Cleared Scheduled generation, imported generation (at the region boundary) and allowances made for non-scheduled generation (MW).
    pub demand_and_nonschedgen: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW).
    pub uigf: Option<rust_decimal::Decimal>,
    /// Constrained generation forecast (MW) for semi-scheduled units for the region. For RELIABILITY_LRC run, semi-scheduled generation is constrained only by System Normal constraints. For OUTAGE_LRC run and LOR run, semi-scheduled generation is constrained by both System Normal and Outage constraints. All three run types (RELIABILITY_LRC, OUTAGE_LRC, LOR) incorporate MAXAVAIL limits.
    pub semischeduledcapacity: Option<rust_decimal::Decimal>,
    /// Constrained generation forecast for semi-scheduled units for the region for the LOR run. Semi-scheduled generation is constrained by both System Normal and Outage constraints, and incorporate MAXAVAIL limits.
    pub lor_semischeduledcapacity: Option<rust_decimal::Decimal>,
    /// Largest Credible Risk. MW value for highest credible contingency
    pub lcr: Option<rust_decimal::Decimal>,
    /// Two Largest Credible Risks. MW value for highest two credible contingencies.
    pub lcr2: Option<rust_decimal::Decimal>,
    /// Forecasting Uncertainty Measure. MW value of reserve calculated as defined in the Reserve Level Declaration Guidelines
    pub fum: Option<rust_decimal::Decimal>,
    /// Unconstrained Intermittent Generation Forecast for solar for the region. For RELIABILITY_LRC and OUTAGE_LRC run this is the POE90 forecast (determined by LRCUIGFOption in CaseSolution). For LOR run this is the POE50 forecast
    pub ss_solar_uigf: Option<rust_decimal::Decimal>,
    /// Unconstrained Intermittent Generation Forecast for wind for the region. For RELIABILITY_LRC and OUTAGE_LRC run this is the POE90 forecast (determined by LRCUIGFOption in CaseSolution). For LOR run this is the POE50 forecast
    pub ss_wind_uigf: Option<rust_decimal::Decimal>,
    /// Constrained generation forecast for solar for the region. For RELIABILITY_LRC run solar generation is constrained only by System Normal constraints. For OUTAGE_LRC run and LOR run solar generation is constrained by both System Normal and Outage constraints. All three run types (RELIABILITY_LRC, OUTAGE_LRC, LOR) incorporate MAXAVAIL limits.
    pub ss_solar_capacity: Option<rust_decimal::Decimal>,
    /// Constrained generation forecast for wind for the region. For RELIABILITY_LRC run wind generation is constrained only by System Normal constraints. For OUTAGE_LRC run and LOR run wind generation is constrained by both System Normal and Outage constraints. All three run types (RELIABILITY_LRC, OUTAGE_LRC, LOR) incorporate MAXAVAIL limits.
    pub ss_wind_capacity: Option<rust_decimal::Decimal>,
    /// Constrained generation forecast (MW) for solar for the region. For RELIABILITY_LRC run, solar generation is constrained only by System Normal constraints. For OUTAGE_LRC run and LOR run, solar generation is constrained by both System Normal and Outage constraints. All three run types (RELIABILITY_LRC, OUTAGE_LRC, LOR) incorporate MAXAVAIL limits.This value is the same as SS_SOLAR_CAPACITY.
    pub ss_solar_cleared: Option<rust_decimal::Decimal>,
    /// Constrained generation forecast (MW) for wind for the region. For RELIABILITY_LRC run, wind generation is constrained only by System Normal constraints. For OUTAGE_LRC run and LOR run, wind generation is constrained by both System Normal and Outage constraints. All three run types (RELIABILITY_LRC, OUTAGE_LRC, LOR) incorporate MAXAVAIL limits.This value is the same as SS_WIND_CAPACITY.
    pub ss_wind_cleared: Option<rust_decimal::Decimal>,
    /// Regional aggregated Wholesale Demand Response (WDR) availability in MW.
    pub wdr_available: Option<rust_decimal::Decimal>,
    /// Regional aggregated Wholesale Demand Response (WDR) PASA availability in MW.
    pub wdr_pasaavailable: Option<rust_decimal::Decimal>,
    /// Regional aggregated Wholesale Demand Response (WDR) capacity in MW.
    pub wdr_capacity: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> StpasaRegionsolution7Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn runtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.runtype.clone())
    }
}
impl mmsdm_core::GetTable for StpasaRegionsolution7 {
    const VERSION: i32 = 7;
    const DATA_SET_NAME: &'static str = "STPASA";
    const TABLE_NAME: &'static str = "REGIONSOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = StpasaRegionsolution7Mapping([
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
        46, 47, 48,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "RUN_DATETIME",
        "INTERVAL_DATETIME",
        "REGIONID",
        "DEMAND10",
        "DEMAND50",
        "DEMAND90",
        "RESERVEREQ",
        "CAPACITYREQ",
        "ENERGYREQDEMAND50",
        "UNCONSTRAINEDCAPACITY",
        "CONSTRAINEDCAPACITY",
        "NETINTERCHANGEUNDERSCARCITY",
        "SURPLUSCAPACITY",
        "SURPLUSRESERVE",
        "RESERVECONDITION",
        "MAXSURPLUSRESERVE",
        "MAXSPARECAPACITY",
        "LORCONDITION",
        "AGGREGATECAPACITYAVAILABLE",
        "AGGREGATESCHEDULEDLOAD",
        "LASTCHANGED",
        "AGGREGATEPASAAVAILABILITY",
        "RUNTYPE",
        "ENERGYREQDEMAND10",
        "CALCULATEDLOR1LEVEL",
        "CALCULATEDLOR2LEVEL",
        "MSRNETINTERCHANGEUNDERSCARCITY",
        "LORNETINTERCHANGEUNDERSCARCITY",
        "TOTALINTERMITTENTGENERATION",
        "DEMAND_AND_NONSCHEDGEN",
        "UIGF",
        "SEMISCHEDULEDCAPACITY",
        "LOR_SEMISCHEDULEDCAPACITY",
        "LCR",
        "LCR2",
        "FUM",
        "SS_SOLAR_UIGF",
        "SS_WIND_UIGF",
        "SS_SOLAR_CAPACITY",
        "SS_WIND_CAPACITY",
        "SS_SOLAR_CLEARED",
        "SS_WIND_CLEARED",
        "WDR_AVAILABLE",
        "WDR_PASAAVAILABLE",
        "WDR_CAPACITY",
    ];
    type Row<'row> = StpasaRegionsolution7Row<'row>;
    type FieldMapping = StpasaRegionsolution7Mapping;
    type PrimaryKey = StpasaRegionsolution7PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(StpasaRegionsolution7Row {
            run_datetime: row
                .get_custom_parsed_at_idx(
                    "run_datetime",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[2])?,
            demand10: row
                .get_opt_custom_parsed_at_idx(
                    "demand10",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand50: row
                .get_opt_custom_parsed_at_idx(
                    "demand50",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand90: row
                .get_opt_custom_parsed_at_idx(
                    "demand90",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            reservereq: row
                .get_opt_custom_parsed_at_idx(
                    "reservereq",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            capacityreq: row
                .get_opt_custom_parsed_at_idx(
                    "capacityreq",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energyreqdemand50: row
                .get_opt_custom_parsed_at_idx(
                    "energyreqdemand50",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            unconstrainedcapacity: row
                .get_opt_custom_parsed_at_idx(
                    "unconstrainedcapacity",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            constrainedcapacity: row
                .get_opt_custom_parsed_at_idx(
                    "constrainedcapacity",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            netinterchangeunderscarcity: row
                .get_opt_custom_parsed_at_idx(
                    "netinterchangeunderscarcity",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            surpluscapacity: row
                .get_opt_custom_parsed_at_idx(
                    "surpluscapacity",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            surplusreserve: row
                .get_opt_custom_parsed_at_idx(
                    "surplusreserve",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            reservecondition: row
                .get_opt_custom_parsed_at_idx(
                    "reservecondition",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxsurplusreserve: row
                .get_opt_custom_parsed_at_idx(
                    "maxsurplusreserve",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            maxsparecapacity: row
                .get_opt_custom_parsed_at_idx(
                    "maxsparecapacity",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lorcondition: row
                .get_opt_custom_parsed_at_idx(
                    "lorcondition",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            aggregatecapacityavailable: row
                .get_opt_custom_parsed_at_idx(
                    "aggregatecapacityavailable",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            aggregatescheduledload: row
                .get_opt_custom_parsed_at_idx(
                    "aggregatescheduledload",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[20],
                    mmsdm_core::mms_datetime::parse,
                )?,
            aggregatepasaavailability: row
                .get_opt_custom_parsed_at_idx(
                    "aggregatepasaavailability",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            runtype: row.get_range("runtype", field_mapping.0[22])?,
            energyreqdemand10: row
                .get_opt_custom_parsed_at_idx(
                    "energyreqdemand10",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            calculatedlor1level: row
                .get_opt_custom_parsed_at_idx(
                    "calculatedlor1level",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            calculatedlor2level: row
                .get_opt_custom_parsed_at_idx(
                    "calculatedlor2level",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            msrnetinterchangeunderscarcity: row
                .get_opt_custom_parsed_at_idx(
                    "msrnetinterchangeunderscarcity",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lornetinterchangeunderscarcity: row
                .get_opt_custom_parsed_at_idx(
                    "lornetinterchangeunderscarcity",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            totalintermittentgeneration: row
                .get_opt_custom_parsed_at_idx(
                    "totalintermittentgeneration",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            demand_and_nonschedgen: row
                .get_opt_custom_parsed_at_idx(
                    "demand_and_nonschedgen",
                    field_mapping.0[29],
                    mmsdm_core::mms_decimal::parse,
                )?,
            uigf: row
                .get_opt_custom_parsed_at_idx(
                    "uigf",
                    field_mapping.0[30],
                    mmsdm_core::mms_decimal::parse,
                )?,
            semischeduledcapacity: row
                .get_opt_custom_parsed_at_idx(
                    "semischeduledcapacity",
                    field_mapping.0[31],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lor_semischeduledcapacity: row
                .get_opt_custom_parsed_at_idx(
                    "lor_semischeduledcapacity",
                    field_mapping.0[32],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lcr: row
                .get_opt_custom_parsed_at_idx(
                    "lcr",
                    field_mapping.0[33],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lcr2: row
                .get_opt_custom_parsed_at_idx(
                    "lcr2",
                    field_mapping.0[34],
                    mmsdm_core::mms_decimal::parse,
                )?,
            fum: row
                .get_opt_custom_parsed_at_idx(
                    "fum",
                    field_mapping.0[35],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_solar_uigf: row
                .get_opt_custom_parsed_at_idx(
                    "ss_solar_uigf",
                    field_mapping.0[36],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_wind_uigf: row
                .get_opt_custom_parsed_at_idx(
                    "ss_wind_uigf",
                    field_mapping.0[37],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_solar_capacity: row
                .get_opt_custom_parsed_at_idx(
                    "ss_solar_capacity",
                    field_mapping.0[38],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_wind_capacity: row
                .get_opt_custom_parsed_at_idx(
                    "ss_wind_capacity",
                    field_mapping.0[39],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_solar_cleared: row
                .get_opt_custom_parsed_at_idx(
                    "ss_solar_cleared",
                    field_mapping.0[40],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ss_wind_cleared: row
                .get_opt_custom_parsed_at_idx(
                    "ss_wind_cleared",
                    field_mapping.0[41],
                    mmsdm_core::mms_decimal::parse,
                )?,
            wdr_available: row
                .get_opt_custom_parsed_at_idx(
                    "wdr_available",
                    field_mapping.0[42],
                    mmsdm_core::mms_decimal::parse,
                )?,
            wdr_pasaavailable: row
                .get_opt_custom_parsed_at_idx(
                    "wdr_pasaavailable",
                    field_mapping.0[43],
                    mmsdm_core::mms_decimal::parse,
                )?,
            wdr_capacity: row
                .get_opt_custom_parsed_at_idx(
                    "wdr_capacity",
                    field_mapping.0[44],
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
        Ok(StpasaRegionsolution7Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> StpasaRegionsolution7PrimaryKey {
        StpasaRegionsolution7PrimaryKey {
            interval_datetime: row.interval_datetime,
            regionid: row.regionid().to_string(),
            run_datetime: row.run_datetime,
            runtype: row.runtype().to_string(),
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("stpasa_regionsolution_v7_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        StpasaRegionsolution7Row {
            run_datetime: row.run_datetime.clone(),
            interval_datetime: row.interval_datetime.clone(),
            regionid: row.regionid.clone(),
            demand10: row.demand10.clone(),
            demand50: row.demand50.clone(),
            demand90: row.demand90.clone(),
            reservereq: row.reservereq.clone(),
            capacityreq: row.capacityreq.clone(),
            energyreqdemand50: row.energyreqdemand50.clone(),
            unconstrainedcapacity: row.unconstrainedcapacity.clone(),
            constrainedcapacity: row.constrainedcapacity.clone(),
            netinterchangeunderscarcity: row.netinterchangeunderscarcity.clone(),
            surpluscapacity: row.surpluscapacity.clone(),
            surplusreserve: row.surplusreserve.clone(),
            reservecondition: row.reservecondition.clone(),
            maxsurplusreserve: row.maxsurplusreserve.clone(),
            maxsparecapacity: row.maxsparecapacity.clone(),
            lorcondition: row.lorcondition.clone(),
            aggregatecapacityavailable: row.aggregatecapacityavailable.clone(),
            aggregatescheduledload: row.aggregatescheduledload.clone(),
            lastchanged: row.lastchanged.clone(),
            aggregatepasaavailability: row.aggregatepasaavailability.clone(),
            runtype: row.runtype.clone(),
            energyreqdemand10: row.energyreqdemand10.clone(),
            calculatedlor1level: row.calculatedlor1level.clone(),
            calculatedlor2level: row.calculatedlor2level.clone(),
            msrnetinterchangeunderscarcity: row.msrnetinterchangeunderscarcity.clone(),
            lornetinterchangeunderscarcity: row.lornetinterchangeunderscarcity.clone(),
            totalintermittentgeneration: row.totalintermittentgeneration.clone(),
            demand_and_nonschedgen: row.demand_and_nonschedgen.clone(),
            uigf: row.uigf.clone(),
            semischeduledcapacity: row.semischeduledcapacity.clone(),
            lor_semischeduledcapacity: row.lor_semischeduledcapacity.clone(),
            lcr: row.lcr.clone(),
            lcr2: row.lcr2.clone(),
            fum: row.fum.clone(),
            ss_solar_uigf: row.ss_solar_uigf.clone(),
            ss_wind_uigf: row.ss_wind_uigf.clone(),
            ss_solar_capacity: row.ss_solar_capacity.clone(),
            ss_wind_capacity: row.ss_wind_capacity.clone(),
            ss_solar_cleared: row.ss_solar_cleared.clone(),
            ss_wind_cleared: row.ss_wind_cleared.clone(),
            wdr_available: row.wdr_available.clone(),
            wdr_pasaavailable: row.wdr_pasaavailable.clone(),
            wdr_capacity: row.wdr_capacity.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StpasaRegionsolution7PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
    pub runtype: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for StpasaRegionsolution7PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for StpasaRegionsolution7Row<'data> {
    type Row<'other> = StpasaRegionsolution7Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid() == row.regionid() && self.run_datetime == row.run_datetime
            && self.runtype() == row.runtype()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for StpasaRegionsolution7Row<'data> {
    type PrimaryKey = StpasaRegionsolution7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid() == key.regionid && self.run_datetime == key.run_datetime
            && self.runtype() == key.runtype
    }
}
impl<'data> mmsdm_core::CompareWithRow for StpasaRegionsolution7PrimaryKey {
    type Row<'other> = StpasaRegionsolution7Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid() && self.run_datetime == row.run_datetime
            && self.runtype == row.runtype()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for StpasaRegionsolution7PrimaryKey {
    type PrimaryKey = StpasaRegionsolution7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for StpasaRegionsolution7 {
    type Builder = StpasaRegionsolution7Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "run_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interval_datetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int16),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "demand10",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand50",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand90",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reservereq",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "capacityreq",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energyreqdemand50",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unconstrainedcapacity",
                    arrow::datatypes::DataType::Decimal128(12, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "constrainedcapacity",
                    arrow::datatypes::DataType::Decimal128(12, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "netinterchangeunderscarcity",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "surpluscapacity",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "surplusreserve",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reservecondition",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxsurplusreserve",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "maxsparecapacity",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lorcondition",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "aggregatecapacityavailable",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "aggregatescheduledload",
                    arrow::datatypes::DataType::Decimal128(12, 2),
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
                    "aggregatepasaavailability",
                    arrow::datatypes::DataType::Decimal128(12, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "runtype",
                    arrow::datatypes::DataType::Dictionary(
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Int32),
                        alloc::boxed::Box::new(arrow::datatypes::DataType::Utf8),
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "energyreqdemand10",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "calculatedlor1level",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "calculatedlor2level",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "msrnetinterchangeunderscarcity",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lornetinterchangeunderscarcity",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "totalintermittentgeneration",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "demand_and_nonschedgen",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "uigf",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "semischeduledcapacity",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lor_semischeduledcapacity",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lcr",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lcr2",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "fum",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_solar_uigf",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_wind_uigf",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_solar_capacity",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_wind_capacity",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_solar_cleared",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ss_wind_cleared",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "wdr_available",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "wdr_pasaavailable",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "wdr_capacity",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        StpasaRegionsolution7Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int16Type,
            >::new(),
            demand10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            demand50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            demand90_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            reservereq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            capacityreq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            energyreqdemand50_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            unconstrainedcapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 0)),
            constrainedcapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 0)),
            netinterchangeunderscarcity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            surpluscapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            surplusreserve_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            reservecondition_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            maxsurplusreserve_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            maxsparecapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lorcondition_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            aggregatecapacityavailable_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            aggregatescheduledload_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            aggregatepasaavailability_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 0)),
            runtype_array: arrow::array::StringDictionaryBuilder::<
                arrow::array::types::Int32Type,
            >::new(),
            energyreqdemand10_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            calculatedlor1level_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            calculatedlor2level_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            msrnetinterchangeunderscarcity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lornetinterchangeunderscarcity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            totalintermittentgeneration_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            demand_and_nonschedgen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            uigf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            semischeduledcapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lor_semischeduledcapacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lcr_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lcr2_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            fum_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            ss_solar_uigf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_wind_uigf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_solar_capacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_wind_capacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_solar_cleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            ss_wind_cleared_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            wdr_available_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            wdr_pasaavailable_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            wdr_capacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .run_datetime_array
            .append_value(row.run_datetime.and_utc().timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.and_utc().timestamp_millis());
        builder.regionid_array.append_value(row.regionid());
        builder
            .demand10_array
            .append_option({
                row.demand10
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .demand50_array
            .append_option({
                row.demand50
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .demand90_array
            .append_option({
                row.demand90
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .reservereq_array
            .append_option({
                row.reservereq
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .capacityreq_array
            .append_option({
                row.capacityreq
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .energyreqdemand50_array
            .append_option({
                row.energyreqdemand50
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .unconstrainedcapacity_array
            .append_option({
                row.unconstrainedcapacity
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .constrainedcapacity_array
            .append_option({
                row.constrainedcapacity
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .netinterchangeunderscarcity_array
            .append_option({
                row.netinterchangeunderscarcity
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .surpluscapacity_array
            .append_option({
                row.surpluscapacity
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .surplusreserve_array
            .append_option({
                row.surplusreserve
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .reservecondition_array
            .append_option({
                row.reservecondition
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .maxsurplusreserve_array
            .append_option({
                row.maxsurplusreserve
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .maxsparecapacity_array
            .append_option({
                row.maxsparecapacity
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .lorcondition_array
            .append_option({
                row.lorcondition
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .aggregatecapacityavailable_array
            .append_option({
                row.aggregatecapacityavailable
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .aggregatescheduledload_array
            .append_option({
                row.aggregatescheduledload
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.and_utc().timestamp_millis()));
        builder
            .aggregatepasaavailability_array
            .append_option({
                row.aggregatepasaavailability
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.runtype_array.append_value(row.runtype());
        builder
            .energyreqdemand10_array
            .append_option({
                row.energyreqdemand10
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .calculatedlor1level_array
            .append_option({
                row.calculatedlor1level
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .calculatedlor2level_array
            .append_option({
                row.calculatedlor2level
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .msrnetinterchangeunderscarcity_array
            .append_option({
                row.msrnetinterchangeunderscarcity
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .lornetinterchangeunderscarcity_array
            .append_option({
                row.lornetinterchangeunderscarcity
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .totalintermittentgeneration_array
            .append_option({
                row.totalintermittentgeneration
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .demand_and_nonschedgen_array
            .append_option({
                row.demand_and_nonschedgen
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .uigf_array
            .append_option({
                row.uigf
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .semischeduledcapacity_array
            .append_option({
                row.semischeduledcapacity
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .lor_semischeduledcapacity_array
            .append_option({
                row.lor_semischeduledcapacity
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .lcr_array
            .append_option({
                row.lcr
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lcr2_array
            .append_option({
                row.lcr2
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .fum_array
            .append_option({
                row.fum
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .ss_solar_uigf_array
            .append_option({
                row.ss_solar_uigf
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_wind_uigf_array
            .append_option({
                row.ss_wind_uigf
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_solar_capacity_array
            .append_option({
                row.ss_solar_capacity
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_wind_capacity_array
            .append_option({
                row.ss_wind_capacity
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_solar_cleared_array
            .append_option({
                row.ss_solar_cleared
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .ss_wind_cleared_array
            .append_option({
                row.ss_wind_cleared
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .wdr_available_array
            .append_option({
                row.wdr_available
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .wdr_pasaavailable_array
            .append_option({
                row.wdr_pasaavailable
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .wdr_capacity_array
            .append_option({
                row.wdr_capacity
                    .map(|mut val| {
                        val.rescale(2);
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
                    alloc::sync::Arc::new(builder.run_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand50_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand90_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reservereq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.capacityreq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.energyreqdemand50_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unconstrainedcapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constrainedcapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.netinterchangeunderscarcity_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.surpluscapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.surplusreserve_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reservecondition_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxsurplusreserve_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.maxsparecapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lorcondition_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.aggregatecapacityavailable_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.aggregatescheduledload_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.aggregatepasaavailability_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.energyreqdemand10_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.calculatedlor1level_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.calculatedlor2level_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.msrnetinterchangeunderscarcity_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.lornetinterchangeunderscarcity_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.totalintermittentgeneration_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.demand_and_nonschedgen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.uigf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.semischeduledcapacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.lor_semischeduledcapacity_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lcr_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lcr2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fum_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_solar_uigf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_wind_uigf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_solar_capacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_wind_capacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_solar_cleared_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ss_wind_cleared_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.wdr_available_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.wdr_pasaavailable_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.wdr_capacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct StpasaRegionsolution7Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::StringDictionaryBuilder<
        arrow::array::types::Int16Type,
    >,
    demand10_array: arrow::array::builder::Decimal128Builder,
    demand50_array: arrow::array::builder::Decimal128Builder,
    demand90_array: arrow::array::builder::Decimal128Builder,
    reservereq_array: arrow::array::builder::Decimal128Builder,
    capacityreq_array: arrow::array::builder::Decimal128Builder,
    energyreqdemand50_array: arrow::array::builder::Decimal128Builder,
    unconstrainedcapacity_array: arrow::array::builder::Decimal128Builder,
    constrainedcapacity_array: arrow::array::builder::Decimal128Builder,
    netinterchangeunderscarcity_array: arrow::array::builder::Decimal128Builder,
    surpluscapacity_array: arrow::array::builder::Decimal128Builder,
    surplusreserve_array: arrow::array::builder::Decimal128Builder,
    reservecondition_array: arrow::array::builder::Decimal128Builder,
    maxsurplusreserve_array: arrow::array::builder::Decimal128Builder,
    maxsparecapacity_array: arrow::array::builder::Decimal128Builder,
    lorcondition_array: arrow::array::builder::Decimal128Builder,
    aggregatecapacityavailable_array: arrow::array::builder::Decimal128Builder,
    aggregatescheduledload_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    aggregatepasaavailability_array: arrow::array::builder::Decimal128Builder,
    runtype_array: arrow::array::StringDictionaryBuilder<arrow::array::types::Int32Type>,
    energyreqdemand10_array: arrow::array::builder::Decimal128Builder,
    calculatedlor1level_array: arrow::array::builder::Decimal128Builder,
    calculatedlor2level_array: arrow::array::builder::Decimal128Builder,
    msrnetinterchangeunderscarcity_array: arrow::array::builder::Decimal128Builder,
    lornetinterchangeunderscarcity_array: arrow::array::builder::Decimal128Builder,
    totalintermittentgeneration_array: arrow::array::builder::Decimal128Builder,
    demand_and_nonschedgen_array: arrow::array::builder::Decimal128Builder,
    uigf_array: arrow::array::builder::Decimal128Builder,
    semischeduledcapacity_array: arrow::array::builder::Decimal128Builder,
    lor_semischeduledcapacity_array: arrow::array::builder::Decimal128Builder,
    lcr_array: arrow::array::builder::Decimal128Builder,
    lcr2_array: arrow::array::builder::Decimal128Builder,
    fum_array: arrow::array::builder::Decimal128Builder,
    ss_solar_uigf_array: arrow::array::builder::Decimal128Builder,
    ss_wind_uigf_array: arrow::array::builder::Decimal128Builder,
    ss_solar_capacity_array: arrow::array::builder::Decimal128Builder,
    ss_wind_capacity_array: arrow::array::builder::Decimal128Builder,
    ss_solar_cleared_array: arrow::array::builder::Decimal128Builder,
    ss_wind_cleared_array: arrow::array::builder::Decimal128Builder,
    wdr_available_array: arrow::array::builder::Decimal128Builder,
    wdr_pasaavailable_array: arrow::array::builder::Decimal128Builder,
    wdr_capacity_array: arrow::array::builder::Decimal128Builder,
}
