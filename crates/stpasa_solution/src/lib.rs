#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct StpasaCasesolution3 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&StpasaCasesolution3Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaCasesolution3 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
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
///  _STPASA_CASESOLUTION holds one record containing results pertaining to each entire solution_
///
/// * Data Set Name: Stpasa
/// * File Name: Casesolution
/// * Data Version: 3
///
/// # Description
///  STPASA_CASESOLUTION is public data. Source STPASA_CASESOLUTION is updated each STPASA run (i.e. every 2 hours). Volume Rows per day: 12 Mb per month: &lt;1
///
///
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
                    arrow::datatypes::DataType::Utf8,
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
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "outagelrccapacityoption",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lorcapacityoption",
                    arrow::datatypes::DataType::Utf8,
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
            pasaversion_array: arrow::array::builder::StringBuilder::new(),
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
            reliabilitylrccapacityoption_array: arrow::array::builder::StringBuilder::new(),
            outagelrccapacityoption_array: arrow::array::builder::StringBuilder::new(),
            lorcapacityoption_array: arrow::array::builder::StringBuilder::new(),
            loruigf_option_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            reliability_lrcuigf_option_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            outage_lrcuigf_option_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.run_datetime_array.append_value(row.run_datetime.timestamp_millis());
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
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
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
    pasaversion_array: arrow::array::builder::StringBuilder,
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
    reliabilitylrccapacityoption_array: arrow::array::builder::StringBuilder,
    outagelrccapacityoption_array: arrow::array::builder::StringBuilder,
    lorcapacityoption_array: arrow::array::builder::StringBuilder,
    loruigf_option_array: arrow::array::builder::Decimal128Builder,
    reliability_lrcuigf_option_array: arrow::array::builder::Decimal128Builder,
    outage_lrcuigf_option_array: arrow::array::builder::Decimal128Builder,
}
pub struct StpasaConstraintsolution3 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&StpasaConstraintsolution3Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaConstraintsolution3 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
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
///  _STPASA_CONSTRAINTSOLUTION shows binding and violated constraint results from the capacity evaluation, including the RHS value._
///
/// * Data Set Name: Stpasa
/// * File Name: Constraintsolution
/// * Data Version: 3
///
/// # Description
///  STPASA_CONSTRAINTSOLUTION is public data. Source STPASA_CONSTRAINTSOLUTION is updated each STPASA run (i.e. every 2 hours). Volume Rows per day: 19000 (est.) Mb per month: 90
///
///
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
    /// Type of run. Values are RELIABILITY_LRC, OUTAGE_LRC and LOR.
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
                    arrow::datatypes::DataType::Utf8,
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
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "studyregionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        StpasaConstraintsolution3Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            constraintid_array: arrow::array::builder::StringBuilder::new(),
            capacityrhs_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            capacitymarginalvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            capacityviolationdegree_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runtype_array: arrow::array::builder::StringBuilder::new(),
            studyregionid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.run_datetime_array.append_value(row.run_datetime.timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.timestamp_millis());
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
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
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
    constraintid_array: arrow::array::builder::StringBuilder,
    capacityrhs_array: arrow::array::builder::Decimal128Builder,
    capacitymarginalvalue_array: arrow::array::builder::Decimal128Builder,
    capacityviolationdegree_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    runtype_array: arrow::array::builder::StringBuilder,
    studyregionid_array: arrow::array::builder::StringBuilder,
}
pub struct StpasaInterconnectorsoln3 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&StpasaInterconnectorsoln3Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaInterconnectorsoln3 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
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
///  _STPASA_INTERCONNECTORSOLN shows the results of the capacity evaluation for Interconnectors, including the calculated limits for the interval._
///
/// * Data Set Name: Stpasa
/// * File Name: Interconnectorsoln
/// * Data Version: 3
///
/// # Description
///  STPASA_INTERCONNECTORSOLN is public so is available to all participants. Source STPASA_INTERCONNECTORSOLN is updated each STPASA run (i.e. every 2 hours). Volume Rows per day: 576 Mb per month: 4
///
///
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
    /// Type of run. Values are RELIABILITY_LRC, OUTAGE_LRC and LOR.
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
                    arrow::datatypes::DataType::Utf8,
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
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "exportlimitconstraintid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "importlimitconstraintid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "studyregionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        StpasaInterconnectorsoln3Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
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
            runtype_array: arrow::array::builder::StringBuilder::new(),
            exportlimitconstraintid_array: arrow::array::builder::StringBuilder::new(),
            importlimitconstraintid_array: arrow::array::builder::StringBuilder::new(),
            studyregionid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.run_datetime_array.append_value(row.run_datetime.timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.timestamp_millis());
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
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
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
    interconnectorid_array: arrow::array::builder::StringBuilder,
    capacitymwflow_array: arrow::array::builder::Decimal128Builder,
    capacitymarginalvalue_array: arrow::array::builder::Decimal128Builder,
    capacityviolationdegree_array: arrow::array::builder::Decimal128Builder,
    calculatedexportlimit_array: arrow::array::builder::Decimal128Builder,
    calculatedimportlimit_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    runtype_array: arrow::array::builder::StringBuilder,
    exportlimitconstraintid_array: arrow::array::builder::StringBuilder,
    importlimitconstraintid_array: arrow::array::builder::StringBuilder,
    studyregionid_array: arrow::array::builder::StringBuilder,
}
pub struct StpasaRegionsolution7 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(&StpasaRegionsolution7Row<'_>) -> mmsdm_core::PartitionValue,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl StpasaRegionsolution7 {
    pub fn new(
        row_partition_key: mmsdm_core::PartitionKey,
        func: impl Fn(
            &<Self as mmsdm_core::GetTable>::Row<'_>,
        ) -> mmsdm_core::PartitionValue + 'static,
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
///  _STPASA_REGIONSOLUTION shows the results of the regional capacity, maximum surplus reserve and maximum spare capacity evaluations for each period of the study._
///
/// * Data Set Name: Stpasa
/// * File Name: Regionsolution
/// * Data Version: 7
///
/// # Description
///  STPASA_REGIONSOLUTION is public so is available to all participants. Source STPASA_REGIONSOLUTION is updated each STPASA run (i.e every 2 hours). Volume Rows per day: 480 Mb per month: 8
///
///
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
/// * RUN_DATETIME
/// * RUNTYPE
#[derive(Debug, PartialEq, Eq)]
pub struct StpasaRegionsolution7Row<'data> {
    /// Unique Timestamp Identifier for this study
    pub run_datetime: chrono::NaiveDateTime,
    /// The unique identifier for the interval within this study
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Input value for 10% probability demand
    pub demand10: Option<rust_decimal::Decimal>,
    /// Input value for 50% probability demand
    pub demand50: Option<rust_decimal::Decimal>,
    /// Input value for 90% probability demand
    pub demand90: Option<rust_decimal::Decimal>,
    /// Input reserve requirement
    pub reservereq: Option<rust_decimal::Decimal>,
    /// Demand + Reserve Requirement
    pub capacityreq: Option<rust_decimal::Decimal>,
    /// Sum of: (Region Period Demand - given Demand50)/Period (sum by trading day, entered in first period of trading day, GWh)
    pub energyreqdemand50: Option<rust_decimal::Decimal>,
    /// In a Region, capacity from generation/Load with no Daily Energy Constraint, subject to network security constraints
    pub unconstrainedcapacity: Option<rust_decimal::Decimal>,
    /// In a Region, capacity from generation/Load with non-zero Daily Energy Constraint, subject to network security constraints
    pub constrainedcapacity: Option<rust_decimal::Decimal>,
    /// Net export in MW out of this region in the capacity adequacy evaluation. Export if &gt; 0, Import if &lt; 0.
    pub netinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    /// Regional surplus capacity MW, +/- values indicate surplus/deficit capacity respectively
    pub surpluscapacity: Option<rust_decimal::Decimal>,
    /// Regional reserve surplus. +/- values indicate surplus/deficit reserve respectively
    pub surplusreserve: Option<rust_decimal::Decimal>,
    /// The regional reserve condition: 0  Adequate, 1  LRC
    pub reservecondition: Option<rust_decimal::Decimal>,
    /// The Maximum Surplus Reserve evaluated for this region in this period.  Calculated for each region in turn.
    pub maxsurplusreserve: Option<rust_decimal::Decimal>,
    /// The Maximum Spare Capacity evaluated for this region in this period. Calculated for each region in turn.
    pub maxsparecapacity: Option<rust_decimal::Decimal>,
    /// The LOR Condition determined from the Maximum Spare Capacity value: 0 - no condition, 1 - LOR1 condition, 2 - LOR2 condition, 3 - LOR3 condition
    pub lorcondition: Option<rust_decimal::Decimal>,
    /// Sum of MAXAVAIL quantities offered by all Scheduled units and Availability of all semi-scheduled units limited by MAXAVAIL in a given Region for a given PERIODID
    pub aggregatecapacityavailable: Option<rust_decimal::Decimal>,
    /// Sum of  MAXAVAIL quantities bid by of all Scheduled Loads in a given Region for a given PERIODID.
    pub aggregatescheduledload: Option<rust_decimal::Decimal>,
    /// Last changed date of this record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Sum of PASAAVAILABILITY for all scheduled generating units and the Unconstrained Intermittent Generation Forecasts (UIGF) for all semi-scheduled generating units in a given Region for a given PERIODID.<br>For the RELIABILITY_LRC and OUTAGE_LRC runs, UIGF is the POE90 forecast. For the LOR run, UIGF is the POE50 forecast.
    pub aggregatepasaavailability: Option<rust_decimal::Decimal>,
    /// Type of run. Values are RELIABILITY_LRC, OUTAGE_LRC and LOR.
    pub runtype: core::ops::Range<usize>,
    /// Energy (GWh) required for this energy block based on the 10% probability of exceedance demand. Listed in the first interval of the energy block
    pub energyreqdemand10: Option<rust_decimal::Decimal>,
    /// Region Reserve Level for LOR1 used. Can be static value or calculated value if an interconnector is a credible contingency
    pub calculatedlor1level: Option<rust_decimal::Decimal>,
    /// Region Reserve Level for LOR2 used. Can be static value or calculated value if an interconnector is a credible contingency
    pub calculatedlor2level: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the region for this interval from the MSR assessment
    pub msrnetinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the region for this interval from the LOR assessment
    pub lornetinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    /// Allowance made for non-scheduled generation in the demand forecast (MW).
    pub totalintermittentgeneration: Option<rust_decimal::Decimal>,
    /// Sum of Cleared Scheduled generation, imported generation (at the region boundary) and allowances made for non-scheduled generation (MW).
    pub demand_and_nonschedgen: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW).
    pub uigf: Option<rust_decimal::Decimal>,
    /// Constrained generation forecast for semi-scheduled units for the region. For RELIABILITY_LRC run semi-scheduled generation is constrained only by System Normal constraints. For OUTAGE_LRC run and LOR run semi-scheduled generation is constrained by both System Normal and Outage constraints. All three run types (RELIABILITY_LRC, OUTAGE_LRC, LOR) incorporate MAXAVAIL limits.
    pub semischeduledcapacity: Option<rust_decimal::Decimal>,
    /// Constrained generation forecast for semi-scheduled units for the region for the LOR run type. Semi-scheduled generation is constrained by both System Normal and Outage constraints, and incorporate MAXAVAIL limits.
    pub lor_semischeduledcapacity: Option<rust_decimal::Decimal>,
    /// Largest Credible Risk. MW value for highest credible contingency
    pub lcr: Option<rust_decimal::Decimal>,
    /// Two Largest Creditable Risks. MW value for highest two credible contingencies.
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
    /// Constrained generation forecast for solar for the region. For RELIABILITY_LRC run solar generation is constrained only by System Normal constraints. For OUTAGE_LRC run and LOR run solar generation is constrained by both System Normal and Outage constraints. All three run types (RELIABILITY_LRC, OUTAGE_LRC, LOR) incorporate MAXAVAIL limits.
    pub ss_solar_cleared: Option<rust_decimal::Decimal>,
    /// Constrained generation forecast for wind for the region. For RELIABILITY_LRC run wind generation is constrained only by System Normal constraints. For OUTAGE_LRC run and LOR run wind generation is constrained by both System Normal and Outage constraints. All three run types (RELIABILITY_LRC, OUTAGE_LRC, LOR) incorporate MAXAVAIL limits.
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
        26,
        27,
        28,
        29,
        30,
        31,
        32,
        33,
        34,
        35,
        36,
        37,
        38,
        39,
        40,
        41,
        42,
        43,
        44,
        45,
        46,
        47,
        48,
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
                    arrow::datatypes::DataType::Utf8,
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
                    arrow::datatypes::DataType::Utf8,
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
            regionid_array: arrow::array::builder::StringBuilder::new(),
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
            runtype_array: arrow::array::builder::StringBuilder::new(),
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
        builder.run_datetime_array.append_value(row.run_datetime.timestamp_millis());
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.timestamp_millis());
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
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
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
    regionid_array: arrow::array::builder::StringBuilder,
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
    runtype_array: arrow::array::builder::StringBuilder,
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
