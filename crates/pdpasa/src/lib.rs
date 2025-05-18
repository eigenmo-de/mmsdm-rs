#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct PdpasaCasesolution3 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &PdpasaCasesolution3Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl PdpasaCasesolution3 {
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
pub struct PdpasaCasesolution3Mapping([usize; 19]);
/// # Summary
///
/// ## PDPASA_CASESOLUTION
///
/// The top-level table identifying a PDPASA case, reporting options applied in the case and summary results
///
/// * Data Set Name: Pdpasa
/// * File Name: Casesolution
/// * Data Version: 3
///
/// # Description
/// PDPASA_CASESOLUTION is public data.SourcePDPASA_CASESOLUTION is updated each PDPASA run (i.e. half-hourly).VolumeRows per day: 48Mb per month: <1
///
/// # Notes
/// * (Visibility)  Public
///
/// # Primary Key Columns
///
/// * RUN_DATETIME
#[derive(Debug, PartialEq, Eq)]
pub struct PdpasaCasesolution3Row<'data> {
    /// Case identifier by the time the case was run
    pub run_datetime: chrono::NaiveDateTime,
    /// Version of the PASA solver used to solve this case
    pub pasaversion: core::ops::Range<usize>,
    /// Low Reserve Condition (LRC) flag for the case (1 - LRC in the case, 0 - No LRCs in the case) for capacity run
    pub reservecondition: Option<rust_decimal::Decimal>,
    /// Lack of Reserve Condition (LOR) flag for the case indicates the most severe condition in the case  (3 = LOR3, 2 = LOR2, 1 = LOR1, 0 = No LOR)
    pub lorcondition: Option<rust_decimal::Decimal>,
    /// Objective Function from the Capacity Adequacy run
    pub capacityobjfunction: Option<rust_decimal::Decimal>,
    /// Not populated as of 2005 End of Year Release; was the Probability of Exceedance (POE) demand forecast used for capacity adequacy (LRC) assessment. 0 if no assessment, 1 for 10% POE, 2 for 50% POE, 3 for 90% POE.
    pub capacityoption: Option<rust_decimal::Decimal>,
    /// Not populated as of 2005 End of Year Release; was the Probability of Exceedance (POE) demand forecast used for assessment of Maximum surplus Reserve. 0 if no assessment, 1 for 10% POE, 2 for 50% POE, 3 for 90% POE
    pub maxsurplusreserveoption: Option<rust_decimal::Decimal>,
    /// Not populated as of 2005 End of Year Release; was the Probability of Exceedance (POE) demand forecast used for assessment of Maximum Spare Capacity. 0 if no assessment, 1 for 10% POE, 2 for 50% POE, 3 for 90% POE
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
impl<'data> PdpasaCasesolution3Row<'data> {
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
impl mmsdm_core::GetTable for PdpasaCasesolution3 {
    const VERSION: i32 = 3;
    const DATA_SET_NAME: &'static str = "PDPASA";
    const TABLE_NAME: &'static str = "CASESOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PdpasaCasesolution3Mapping([
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
    type Row<'row> = PdpasaCasesolution3Row<'row>;
    type FieldMapping = PdpasaCasesolution3Mapping;
    type PrimaryKey = PdpasaCasesolution3PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PdpasaCasesolution3Row {
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
        Ok(PdpasaCasesolution3Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PdpasaCasesolution3PrimaryKey {
        PdpasaCasesolution3PrimaryKey {
            run_datetime: row.run_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("pdpasa_casesolution_v3_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PdpasaCasesolution3Row {
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
pub struct PdpasaCasesolution3PrimaryKey {
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for PdpasaCasesolution3PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PdpasaCasesolution3Row<'data> {
    type Row<'other> = PdpasaCasesolution3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for PdpasaCasesolution3Row<'data> {
    type PrimaryKey = PdpasaCasesolution3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for PdpasaCasesolution3PrimaryKey {
    type Row<'other> = PdpasaCasesolution3Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PdpasaCasesolution3PrimaryKey {
    type PrimaryKey = PdpasaCasesolution3PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PdpasaCasesolution3 {
    type Builder = PdpasaCasesolution3Builder;
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
        PdpasaCasesolution3Builder {
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
pub struct PdpasaCasesolution3Builder {
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
pub struct PdpasaConstraintsolution1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &PdpasaConstraintsolution1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl PdpasaConstraintsolution1 {
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
pub struct PdpasaConstraintsolution1Mapping([usize; 9]);
/// # Summary
///
/// ## PDPASA_CONSTRAINTSOLUTION
///
/// PDPASA_CONSTRAINTSOLUTION shows binding and violated constraint results from the capacity evaluation, including the RHS value.
///
/// * Data Set Name: Pdpasa
/// * File Name: Constraintsolution
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
/// * STUDYREGIONID
#[derive(Debug, PartialEq, Eq)]
pub struct PdpasaConstraintsolution1Row<'data> {
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
impl<'data> PdpasaConstraintsolution1Row<'data> {
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
impl mmsdm_core::GetTable for PdpasaConstraintsolution1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PDPASA";
    const TABLE_NAME: &'static str = "CONSTRAINTSOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PdpasaConstraintsolution1Mapping([
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
    type Row<'row> = PdpasaConstraintsolution1Row<'row>;
    type FieldMapping = PdpasaConstraintsolution1Mapping;
    type PrimaryKey = PdpasaConstraintsolution1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PdpasaConstraintsolution1Row {
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
        Ok(PdpasaConstraintsolution1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PdpasaConstraintsolution1PrimaryKey {
        PdpasaConstraintsolution1PrimaryKey {
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
        alloc::format!("pdpasa_constraintsolution_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PdpasaConstraintsolution1Row {
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
pub struct PdpasaConstraintsolution1PrimaryKey {
    pub constraintid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
    pub runtype: alloc::string::String,
    pub studyregionid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for PdpasaConstraintsolution1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PdpasaConstraintsolution1Row<'data> {
    type Row<'other> = PdpasaConstraintsolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid() == row.constraintid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime && self.runtype() == row.runtype()
            && self.studyregionid() == row.studyregionid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for PdpasaConstraintsolution1Row<'data> {
    type PrimaryKey = PdpasaConstraintsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid() == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime && self.runtype() == key.runtype
            && self.studyregionid() == key.studyregionid
    }
}
impl<'data> mmsdm_core::CompareWithRow for PdpasaConstraintsolution1PrimaryKey {
    type Row<'other> = PdpasaConstraintsolution1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid == row.constraintid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime && self.runtype == row.runtype()
            && self.studyregionid == row.studyregionid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PdpasaConstraintsolution1PrimaryKey {
    type PrimaryKey = PdpasaConstraintsolution1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime && self.runtype == key.runtype
            && self.studyregionid == key.studyregionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PdpasaConstraintsolution1 {
    type Builder = PdpasaConstraintsolution1Builder;
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
        PdpasaConstraintsolution1Builder {
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
pub struct PdpasaConstraintsolution1Builder {
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
pub struct PdpasaDuidavailability1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &PdpasaDuidavailability1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl PdpasaDuidavailability1 {
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
pub struct PdpasaDuidavailability1Mapping([usize; 10]);
/// # Summary
///
/// ## PDPASA_DUIDAVAILABILITY
///
/// This report delivers available capacity, PASA availability and given recall period for all scheduled resources. Note that for an MNSP, DUID = LINKID in the MNSP_INTERCONNECTOR table
///
/// * Data Set Name: Pdpasa
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
pub struct PdpasaDuidavailability1Row<'data> {
    /// First half hour ended interval of the run
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
    /// The Last changed Date time of the record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> PdpasaDuidavailability1Row<'data> {
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
}
impl mmsdm_core::GetTable for PdpasaDuidavailability1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PDPASA";
    const TABLE_NAME: &'static str = "DUIDAVAILABILITY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PdpasaDuidavailability1Mapping([
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
    type Row<'row> = PdpasaDuidavailability1Row<'row>;
    type FieldMapping = PdpasaDuidavailability1Mapping;
    type PrimaryKey = PdpasaDuidavailability1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PdpasaDuidavailability1Row {
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
        Ok(PdpasaDuidavailability1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PdpasaDuidavailability1PrimaryKey {
        PdpasaDuidavailability1PrimaryKey {
            duid: row.duid().to_string(),
            interval_datetime: row.interval_datetime,
            run_datetime: row.run_datetime,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("pdpasa_duidavailability_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PdpasaDuidavailability1Row {
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
pub struct PdpasaDuidavailability1PrimaryKey {
    pub duid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for PdpasaDuidavailability1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PdpasaDuidavailability1Row<'data> {
    type Row<'other> = PdpasaDuidavailability1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for PdpasaDuidavailability1Row<'data> {
    type PrimaryKey = PdpasaDuidavailability1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
impl<'data> mmsdm_core::CompareWithRow for PdpasaDuidavailability1PrimaryKey {
    type Row<'other> = PdpasaDuidavailability1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PdpasaDuidavailability1PrimaryKey {
    type PrimaryKey = PdpasaDuidavailability1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PdpasaDuidavailability1 {
    type Builder = PdpasaDuidavailability1Builder;
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
                    arrow::datatypes::DataType::Utf8,
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
        PdpasaDuidavailability1Builder {
            run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
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
pub struct PdpasaDuidavailability1Builder {
    run_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    generation_max_availability_array: arrow::array::builder::Decimal128Builder,
    generation_pasa_availability_array: arrow::array::builder::Decimal128Builder,
    generation_recall_period_array: arrow::array::builder::Decimal128Builder,
    load_max_availability_array: arrow::array::builder::Decimal128Builder,
    load_pasa_availability_array: arrow::array::builder::Decimal128Builder,
    load_recall_period_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct PdpasaInterconnectorsoln1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &PdpasaInterconnectorsoln1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl PdpasaInterconnectorsoln1 {
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
pub struct PdpasaInterconnectorsoln1Mapping([usize; 13]);
/// # Summary
///
/// ## PDPASA_INTERCONNECTORSOLN
///
/// PDPASA_INTERCONNECTORSOLN shows the results of the capacity evaluation for Interconnectors, including the calculated limits for the interval.
///
/// * Data Set Name: Pdpasa
/// * File Name: Interconnectorsoln
/// * Data Version: 1
///
/// # Description
/// PDPASA_REGIONSOLUTION is public so is available to all participants.SourcePDPASA_REGIONSOLUTION is updated each PDPASA run (i.e. half-hourly).VolumeRows per day: 32000NotesLRC DeterminationSURPLUSRESERVE is the surplus reserve in a region based on meeting the demand plus the reserve requirement in all regions simultaneously. Note that any surplus above the network restrictions and system reserve requirements is reported in the region it is generated, thus a surplus of zero can mean that a region is importing to meet a requirement or that it has exported all surplus to meet an adjacent region’s requirement.The PASA processes also calculate a regionally optimised surplus called the Maximum LRC Surplus (MAXSURPLUSRESERVE) being a figure on how much generation could be brought to this region subject to meeting requirements in other regions.LOR DeterminationMAXSPARECAPACITY is a regionally optimised figure representing the surplus generation able to be brought to a region subject to meeting the demand in all other regions.Participants are directed to the first half hour of the Predispatch PASA (PDPASA) reports as NEMMCO's latest reserve determination for a given half hour.
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
pub struct PdpasaInterconnectorsoln1Row<'data> {
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
impl<'data> PdpasaInterconnectorsoln1Row<'data> {
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
impl mmsdm_core::GetTable for PdpasaInterconnectorsoln1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PDPASA";
    const TABLE_NAME: &'static str = "INTERCONNECTORSOLN";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PdpasaInterconnectorsoln1Mapping([
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
    type Row<'row> = PdpasaInterconnectorsoln1Row<'row>;
    type FieldMapping = PdpasaInterconnectorsoln1Mapping;
    type PrimaryKey = PdpasaInterconnectorsoln1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PdpasaInterconnectorsoln1Row {
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
        Ok(PdpasaInterconnectorsoln1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PdpasaInterconnectorsoln1PrimaryKey {
        PdpasaInterconnectorsoln1PrimaryKey {
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
        alloc::format!("pdpasa_interconnectorsoln_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PdpasaInterconnectorsoln1Row {
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
pub struct PdpasaInterconnectorsoln1PrimaryKey {
    pub interconnectorid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
    pub runtype: alloc::string::String,
    pub studyregionid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for PdpasaInterconnectorsoln1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PdpasaInterconnectorsoln1Row<'data> {
    type Row<'other> = PdpasaInterconnectorsoln1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interconnectorid() == row.interconnectorid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime && self.runtype() == row.runtype()
            && self.studyregionid() == row.studyregionid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for PdpasaInterconnectorsoln1Row<'data> {
    type PrimaryKey = PdpasaInterconnectorsoln1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid() == key.interconnectorid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime && self.runtype() == key.runtype
            && self.studyregionid() == key.studyregionid
    }
}
impl<'data> mmsdm_core::CompareWithRow for PdpasaInterconnectorsoln1PrimaryKey {
    type Row<'other> = PdpasaInterconnectorsoln1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interconnectorid == row.interconnectorid()
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime && self.runtype == row.runtype()
            && self.studyregionid == row.studyregionid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PdpasaInterconnectorsoln1PrimaryKey {
    type PrimaryKey = PdpasaInterconnectorsoln1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime && self.runtype == key.runtype
            && self.studyregionid == key.studyregionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PdpasaInterconnectorsoln1 {
    type Builder = PdpasaInterconnectorsoln1Builder;
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
        PdpasaInterconnectorsoln1Builder {
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
pub struct PdpasaInterconnectorsoln1Builder {
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
pub struct PdpasaRegionsolution7 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &PdpasaRegionsolution7Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl PdpasaRegionsolution7 {
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
pub struct PdpasaRegionsolution7Mapping([usize; 45]);
/// # Summary
///
/// ## PDPASA_REGIONSOLUTION
///
/// The PDPASA region solution data
///
/// * Data Set Name: Pdpasa
/// * File Name: Regionsolution
/// * Data Version: 7
///
/// # Description
/// PDPASA_REGIONSOLUTION is public so is available to all participants.SourcePDPASA_REGIONSOLUTION is updated each PDPASA run (i.e. half-hourly).VolumeRows per day: 32000NotesLRC DeterminationSURPLUSRESERVE is the surplus reserve in a region based on meeting the demand plus the reserve requirement in all regions simultaneously. Note that any surplus above the network restrictions and system reserve requirements is reported in the region it is generated, thus a surplus of zero can mean that a region is importing to meet a requirement or that it has exported all surplus to meet an adjacent region’s requirement.The PASA processes also calculate a regionally optimised surplus called the Maximum LRC Surplus (MAXSURPLUSRESERVE) being a figure on how much generation could be brought to this region subject to meeting requirements in other regions.LOR DeterminationMAXSPARECAPACITY is a regionally optimised figure representing the surplus generation able to be brought to a region subject to meeting the demand in all other regions.Participants are directed to the first half hour of the Predispatch PASA (PDPASA) reports as NEMMCO's latest reserve determination for a given half hour.
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
pub struct PdpasaRegionsolution7Row<'data> {
    /// Case identifier by the time the case was run
    pub run_datetime: chrono::NaiveDateTime,
    /// End date time of the interval
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region identifier
    pub regionid: core::ops::Range<usize>,
    /// 10% Probability of Exceedance demand forecast
    pub demand10: Option<rust_decimal::Decimal>,
    /// 50% Probability of Exceedance demand forecast
    pub demand50: Option<rust_decimal::Decimal>,
    /// 90% Probability of Exceedance demand forecast
    pub demand90: Option<rust_decimal::Decimal>,
    /// Region reserve requirement (MW)
    pub reservereq: Option<rust_decimal::Decimal>,
    /// Capacity required to meet the demand and reserve levels in the capacity adequacy assessment.
    pub capacityreq: Option<rust_decimal::Decimal>,
    /// Energy (GWh) required for this energy block based on the 50% probability of exceedance demand. Listed in the first interval of the energy block.
    pub energyreqdemand50: Option<rust_decimal::Decimal>,
    /// Aggregate generator capability from Non Energy Constrained plant including restrictions due to network constraints from the capacity adequacy (LRC) assessment.
    pub unconstrainedcapacity: Option<rust_decimal::Decimal>,
    /// Aggregate generator capability from Energy Constrained plant including restrictions due to network constraints
    pub constrainedcapacity: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the region for this interval from the capacity adequacy (LRC) assessment.
    pub netinterchangeunderscarcity: Option<rust_decimal::Decimal>,
    /// Surplus capacity (MW) above the demand, scheduled load and net interchange in this region from the capacity adequacy (LRC) assessment.
    pub surpluscapacity: Option<rust_decimal::Decimal>,
    /// Surplus reserve (MW) above the demand, scheduled load,  net interchange and reserve requirement in this region from the capacity adequacy (LRC) assessment.
    pub surplusreserve: Option<rust_decimal::Decimal>,
    /// Low Reserve Condition (LRC) flag for this region in this interval (1 - LRC, 0 - No LRC)
    pub reservecondition: Option<rust_decimal::Decimal>,
    /// Maximum surplus reserve (MW) above the demand + reserve requirement able to be sourced to this region while meeting demand + reserve requirements in other regions.
    pub maxsurplusreserve: Option<rust_decimal::Decimal>,
    /// Maximum spare capacity (MW) above the demand able to be sourced to this region while meeting demands in other regions.
    pub maxsparecapacity: Option<rust_decimal::Decimal>,
    /// Lack of Reserve Condition (LOR) flag for this region and interval   (3 = LOR3, 2 = LOR2, 1 = LOR1, 0 = No LOR)
    pub lorcondition: Option<rust_decimal::Decimal>,
    /// Sum of MAXAVAIL quantities offered by all Scheduled units and Availability of all semi-scheduled units limited by MAXAVAIL in a given Region for a given PERIODID
    pub aggregatecapacityavailable: Option<rust_decimal::Decimal>,
    /// Sum of  MAXAVAIL quantities bid by of all Scheduled Loads in a given Region for a given PERIODID.
    pub aggregatescheduledload: Option<rust_decimal::Decimal>,
    /// Date time the record was created or modified changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Sum of PASAAVAILABILITY for all scheduled generating units and scheduled bidirectional units (Gen side) with a Recall_Period of null or <= 24 hours plus the sum of Unconstrained Intermittent Generation Forecasts (UIGF) for all semi-scheduled generating units. For the OUTAGE_LRC run, UIGF is the POE90 forecast. For the LOR Run, UIGF is the POE50 forecast. Note that the OUTAGE_LRC Run Type is discontinued from 31 July 2025.
    pub aggregatepasaavailability: Option<rust_decimal::Decimal>,
    /// Type of run. Values are RELIABILITY_LRC, OUTAGE_LRC and LOR. Note that the PDPASA OUTAGE_LRC Run Type is discontinued from 31 July 2025, with only the LOR Run Type reported.
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
    pub semi_scheduled_capacity: Option<rust_decimal::Decimal>,
    /// Constrained generation forecast for semi-scheduled units for the region for the LOR run. Semi-scheduled generation is constrained by both System Normal and Outage constraints, and incorporate MAXAVAIL limits.
    pub lor_semi_scheduled_capacity: Option<rust_decimal::Decimal>,
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
impl<'data> PdpasaRegionsolution7Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn runtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.runtype.clone())
    }
}
impl mmsdm_core::GetTable for PdpasaRegionsolution7 {
    const VERSION: i32 = 7;
    const DATA_SET_NAME: &'static str = "PDPASA";
    const TABLE_NAME: &'static str = "REGIONSOLUTION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PdpasaRegionsolution7Mapping([
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
        "SemiScheduledCapacity",
        "LOR_SemiScheduledCapacity",
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
    type Row<'row> = PdpasaRegionsolution7Row<'row>;
    type FieldMapping = PdpasaRegionsolution7Mapping;
    type PrimaryKey = PdpasaRegionsolution7PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PdpasaRegionsolution7Row {
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
            semi_scheduled_capacity: row
                .get_opt_custom_parsed_at_idx(
                    "semi_scheduled_capacity",
                    field_mapping.0[31],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lor_semi_scheduled_capacity: row
                .get_opt_custom_parsed_at_idx(
                    "lor_semi_scheduled_capacity",
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
        Ok(PdpasaRegionsolution7Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PdpasaRegionsolution7PrimaryKey {
        PdpasaRegionsolution7PrimaryKey {
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
        alloc::format!("pdpasa_regionsolution_v7_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PdpasaRegionsolution7Row {
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
            semi_scheduled_capacity: row.semi_scheduled_capacity.clone(),
            lor_semi_scheduled_capacity: row.lor_semi_scheduled_capacity.clone(),
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
pub struct PdpasaRegionsolution7PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: alloc::string::String,
    pub run_datetime: chrono::NaiveDateTime,
    pub runtype: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for PdpasaRegionsolution7PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PdpasaRegionsolution7Row<'data> {
    type Row<'other> = PdpasaRegionsolution7Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid() == row.regionid() && self.run_datetime == row.run_datetime
            && self.runtype() == row.runtype()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for PdpasaRegionsolution7Row<'data> {
    type PrimaryKey = PdpasaRegionsolution7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid() == key.regionid && self.run_datetime == key.run_datetime
            && self.runtype() == key.runtype
    }
}
impl<'data> mmsdm_core::CompareWithRow for PdpasaRegionsolution7PrimaryKey {
    type Row<'other> = PdpasaRegionsolution7Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid() && self.run_datetime == row.run_datetime
            && self.runtype == row.runtype()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PdpasaRegionsolution7PrimaryKey {
    type PrimaryKey = PdpasaRegionsolution7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PdpasaRegionsolution7 {
    type Builder = PdpasaRegionsolution7Builder;
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
                    "semi_scheduled_capacity",
                    arrow::datatypes::DataType::Decimal128(12, 2),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lor_semi_scheduled_capacity",
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
        PdpasaRegionsolution7Builder {
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
            semi_scheduled_capacity_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(12, 2)),
            lor_semi_scheduled_capacity_array: arrow::array::builder::Decimal128Builder::new()
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
            .semi_scheduled_capacity_array
            .append_option({
                row.semi_scheduled_capacity
                    .map(|mut val| {
                        val.rescale(2);
                        val.mantissa()
                    })
            });
        builder
            .lor_semi_scheduled_capacity_array
            .append_option({
                row.lor_semi_scheduled_capacity
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
                    alloc::sync::Arc::new(builder.semi_scheduled_capacity_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.lor_semi_scheduled_capacity_array.finish(),
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
pub struct PdpasaRegionsolution7Builder {
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
    semi_scheduled_capacity_array: arrow::array::builder::Decimal128Builder,
    lor_semi_scheduled_capacity_array: arrow::array::builder::Decimal128Builder,
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
