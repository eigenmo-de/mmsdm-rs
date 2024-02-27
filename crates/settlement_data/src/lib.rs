#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct SettlementsDaytrack6;
pub struct SettlementsDaytrack6Mapping([usize; 8]);
/// # Summary
///
/// ## DAYTRACK
///  _DAYTRACK identifies the actual settlement run processed for each settlement day. Settlement run is in the column EXPOSTRUNNO. Generally the number of the settlement run used in the latest statement is the maximum number._
///
/// * Data Set Name: Settlements
/// * File Name: Daytrack
/// * Data Version: 6
///
/// # Description
///  DAYTRACK is a public data, and is available to all participants. Source DAYTRACK is populated by the posting of a billing run. Volume Daily billing runs insert one row per day. A non-interim statement has seven records inserted per week. An indicative maximum is 35 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * EXPOSTRUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsDaytrack6Row<'data> {
    /// Calendar Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Not Used
    pub regionid: core::ops::Range<usize>,
    /// Not Used
    pub exanterunstatus: core::ops::Range<usize>,
    /// Not Used
    pub exanterunno: Option<rust_decimal::Decimal>,
    /// Not Used
    pub expostrunstatus: core::ops::Range<usize>,
    /// Settlement Run No
    pub expostrunno: rust_decimal::Decimal,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Length of a settlement interval, in minutes (was 30 minutes, will be 5 minutes starting the commencement of 5MS rule change date).
    pub settlementintervallength: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsDaytrack6Row<'data> {
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
    pub fn exanterunstatus(&self) -> Option<&str> {
        if self.exanterunstatus.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.exanterunstatus.clone(),
                ),
            )
        }
    }
    pub fn expostrunstatus(&self) -> Option<&str> {
        if self.expostrunstatus.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.expostrunstatus.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for SettlementsDaytrack6 {
    const VERSION: i32 = 6;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "DAYTRACK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsDaytrack6Mapping([
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
        "SETTLEMENTDATE",
        "REGIONID",
        "EXANTERUNSTATUS",
        "EXANTERUNNO",
        "EXPOSTRUNSTATUS",
        "EXPOSTRUNNO",
        "LASTCHANGED",
        "SETTLEMENTINTERVALLENGTH",
    ];
    type Row<'row> = SettlementsDaytrack6Row<'row>;
    type FieldMapping = SettlementsDaytrack6Mapping;
    type PrimaryKey = SettlementsDaytrack6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsDaytrack6Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_opt_range("regionid", field_mapping.0[1])?,
            exanterunstatus: row.get_opt_range("exanterunstatus", field_mapping.0[2])?,
            exanterunno: row
                .get_opt_custom_parsed_at_idx(
                    "exanterunno",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            expostrunstatus: row.get_opt_range("expostrunstatus", field_mapping.0[4])?,
            expostrunno: row
                .get_custom_parsed_at_idx(
                    "expostrunno",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            settlementintervallength: row
                .get_opt_custom_parsed_at_idx(
                    "settlementintervallength",
                    field_mapping.0[7],
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
        Ok(SettlementsDaytrack6Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsDaytrack6PrimaryKey {
        SettlementsDaytrack6PrimaryKey {
            expostrunno: row.expostrunno,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_daytrack_v6_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsDaytrack6Row {
            settlementdate: row.settlementdate.clone(),
            regionid: row.regionid.clone(),
            exanterunstatus: row.exanterunstatus.clone(),
            exanterunno: row.exanterunno.clone(),
            expostrunstatus: row.expostrunstatus.clone(),
            expostrunno: row.expostrunno.clone(),
            lastchanged: row.lastchanged.clone(),
            settlementintervallength: row.settlementintervallength.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsDaytrack6PrimaryKey {
    pub expostrunno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for SettlementsDaytrack6PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsDaytrack6Row<'data> {
    type Row<'other> = SettlementsDaytrack6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.expostrunno == row.expostrunno && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsDaytrack6Row<'data> {
    type PrimaryKey = SettlementsDaytrack6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.expostrunno == key.expostrunno && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsDaytrack6PrimaryKey {
    type Row<'other> = SettlementsDaytrack6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.expostrunno == row.expostrunno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsDaytrack6PrimaryKey {
    type PrimaryKey = SettlementsDaytrack6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.expostrunno == key.expostrunno && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsDaytrack6 {
    type Builder = SettlementsDaytrack6Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "exanterunstatus",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "exanterunno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "expostrunstatus",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "expostrunno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
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
                    "settlementintervallength",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsDaytrack6Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            exanterunstatus_array: arrow::array::builder::StringBuilder::new(),
            exanterunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            expostrunstatus_array: arrow::array::builder::StringBuilder::new(),
            expostrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            settlementintervallength_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder.regionid_array.append_option(row.regionid());
        builder.exanterunstatus_array.append_option(row.exanterunstatus());
        builder
            .exanterunno_array
            .append_option({
                row.exanterunno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.expostrunstatus_array.append_option(row.expostrunstatus());
        builder
            .expostrunno_array
            .append_value({
                let mut val = row.expostrunno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .settlementintervallength_array
            .append_option({
                row.settlementintervallength
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exanterunstatus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exanterunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.expostrunstatus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.expostrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.settlementintervallength_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsDaytrack6Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    exanterunstatus_array: arrow::array::builder::StringBuilder,
    exanterunno_array: arrow::array::builder::Decimal128Builder,
    expostrunstatus_array: arrow::array::builder::StringBuilder,
    expostrunno_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    settlementintervallength_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsCpdata7;
pub struct SettlementsCpdata7Mapping([usize; 32]);
/// # Summary
///
/// ## SETCPDATA
///  _SETCPDATA shows meter settlement data for each connection point. This is the key view for retailers to verify energy charges. A regional summary view is also provided. As the view has values for each connection point by period, for each meter data file, it is a very large view._
///
/// * Data Set Name: Settlements
/// * File Name: Cpdata
/// * Data Version: 7
///
/// # Description
///  The Connection point details (in SETCPDATA) are confidential to the participant and host retailer that the connection points relate to. By comparison, the regional data (SETCPDATAREGION) is publically available. Source SETCPDATA updates with each Settlement run.
///
///
///
/// # Primary Key Columns
///
/// * MDA
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * TCPID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsCpdata7Row<'data> {
    /// Calendar Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Connection point identifier
    pub tcpid: core::ops::Range<usize>,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Import Gross energy into the pool - MWh
    pub igenergy: Option<rust_decimal::Decimal>,
    /// Export Gross energy from the pool - MWh
    pub xgenergy: Option<rust_decimal::Decimal>,
    /// Import Nett energy into the pool - MWh, plus UFEA if the UFEA amount is positive. When GS commences, this includes the UFEA amount in the settlement runs.
    pub inenergy: Option<rust_decimal::Decimal>,
    /// Export Nett energy from the pool - MWh, plus (UFEA * -1) if the UFEA amount is negative. When GS commences, this includes the UFEA amount in the settlement runs.
    pub xnenergy: Option<rust_decimal::Decimal>,
    /// Import reactive power
    pub ipower: Option<rust_decimal::Decimal>,
    /// Export reactive power
    pub xpower: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess Energy Price
    pub eep: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Connection Point Price = RRP * TLF
    pub cprrp: Option<rust_decimal::Decimal>,
    /// Connection Point Excess Energy Price = EEP * TLF
    pub cpeep: Option<rust_decimal::Decimal>,
    /// Export - Import of Net energy (MWh)
    pub ta: Option<rust_decimal::Decimal>,
    /// settlement amount in $ for trading period
    pub ep: Option<rust_decimal::Decimal>,
    /// Not used
    pub apc: Option<rust_decimal::Decimal>,
    /// Not used
    pub resc: Option<rust_decimal::Decimal>,
    /// Not used
    pub resp: Option<rust_decimal::Decimal>,
    /// Meter Run Number = version number of the meter file
    pub meterrunno: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Not used
    pub hostdistributor: core::ops::Range<usize>,
    /// Metering Data Agent
    pub mda: core::ops::Range<usize>,
    /// Accounted For Energy for this Market Customer FRMP and TNI in the Settlements Trading Interval, excluding any UFEA component
    pub afe: Option<rust_decimal::Decimal>,
    /// Sum of ME- for all NMIs at this Market Customer FRMP and TNI in the Settlements Trading Interval.
    pub dme: Option<rust_decimal::Decimal>,
    /// Share of UFE allocated to this FRMP and TNI in the Settlements Trading Interval.
    pub ufea: Option<rust_decimal::Decimal>,
    /// Adjusted Gross Energy for this Market Customer FRMP and TNI in the Settlements Trading Interval. When GS commences, this includes the UFEA amount in the settlement runs.
    pub age: Option<rust_decimal::Decimal>,
    /// The total cost of energy sold at the connection point by the participant in this settlement interval
    pub importenergycost: Option<rust_decimal::Decimal>,
    /// The total cost of energy purchased at the connection point by the participant in this settlement interval
    pub exportenergycost: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsCpdata7Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn tcpid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.tcpid.clone())
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
    pub fn hostdistributor(&self) -> Option<&str> {
        if self.hostdistributor.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.hostdistributor.clone(),
                ),
            )
        }
    }
    pub fn mda(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.mda.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsCpdata7 {
    const VERSION: i32 = 7;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "CPDATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsCpdata7Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PERIODID",
        "PARTICIPANTID",
        "TCPID",
        "REGIONID",
        "IGENERGY",
        "XGENERGY",
        "INENERGY",
        "XNENERGY",
        "IPOWER",
        "XPOWER",
        "RRP",
        "EEP",
        "TLF",
        "CPRRP",
        "CPEEP",
        "TA",
        "EP",
        "APC",
        "RESC",
        "RESP",
        "METERRUNNO",
        "LASTCHANGED",
        "HOSTDISTRIBUTOR",
        "MDA",
        "AFE",
        "DME",
        "UFEA",
        "AGE",
        "IMPORTENERGYCOST",
        "EXPORTENERGYCOST",
    ];
    type Row<'row> = SettlementsCpdata7Row<'row>;
    type FieldMapping = SettlementsCpdata7Mapping;
    type PrimaryKey = SettlementsCpdata7PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsCpdata7Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[3])?,
            tcpid: row.get_range("tcpid", field_mapping.0[4])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[5])?,
            igenergy: row
                .get_opt_custom_parsed_at_idx(
                    "igenergy",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            xgenergy: row
                .get_opt_custom_parsed_at_idx(
                    "xgenergy",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            inenergy: row
                .get_opt_custom_parsed_at_idx(
                    "inenergy",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            xnenergy: row
                .get_opt_custom_parsed_at_idx(
                    "xnenergy",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ipower: row
                .get_opt_custom_parsed_at_idx(
                    "ipower",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            xpower: row
                .get_opt_custom_parsed_at_idx(
                    "xpower",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp: row
                .get_opt_custom_parsed_at_idx(
                    "rrp",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            eep: row
                .get_opt_custom_parsed_at_idx(
                    "eep",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            tlf: row
                .get_opt_custom_parsed_at_idx(
                    "tlf",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cprrp: row
                .get_opt_custom_parsed_at_idx(
                    "cprrp",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cpeep: row
                .get_opt_custom_parsed_at_idx(
                    "cpeep",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ta: row
                .get_opt_custom_parsed_at_idx(
                    "ta",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ep: row
                .get_opt_custom_parsed_at_idx(
                    "ep",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            apc: row
                .get_opt_custom_parsed_at_idx(
                    "apc",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            resc: row
                .get_opt_custom_parsed_at_idx(
                    "resc",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            resp: row
                .get_opt_custom_parsed_at_idx(
                    "resp",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            meterrunno: row
                .get_opt_custom_parsed_at_idx(
                    "meterrunno",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[23],
                    mmsdm_core::mms_datetime::parse,
                )?,
            hostdistributor: row.get_opt_range("hostdistributor", field_mapping.0[24])?,
            mda: row.get_range("mda", field_mapping.0[25])?,
            afe: row
                .get_opt_custom_parsed_at_idx(
                    "afe",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            dme: row
                .get_opt_custom_parsed_at_idx(
                    "dme",
                    field_mapping.0[27],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ufea: row
                .get_opt_custom_parsed_at_idx(
                    "ufea",
                    field_mapping.0[28],
                    mmsdm_core::mms_decimal::parse,
                )?,
            age: row
                .get_opt_custom_parsed_at_idx(
                    "age",
                    field_mapping.0[29],
                    mmsdm_core::mms_decimal::parse,
                )?,
            importenergycost: row
                .get_opt_custom_parsed_at_idx(
                    "importenergycost",
                    field_mapping.0[30],
                    mmsdm_core::mms_decimal::parse,
                )?,
            exportenergycost: row
                .get_opt_custom_parsed_at_idx(
                    "exportenergycost",
                    field_mapping.0[31],
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
        Ok(SettlementsCpdata7Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsCpdata7PrimaryKey {
        SettlementsCpdata7PrimaryKey {
            mda: row.mda().to_string(),
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
            tcpid: row.tcpid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_cpdata_v7_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsCpdata7Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            periodid: row.periodid.clone(),
            participantid: row.participantid.clone(),
            tcpid: row.tcpid.clone(),
            regionid: row.regionid.clone(),
            igenergy: row.igenergy.clone(),
            xgenergy: row.xgenergy.clone(),
            inenergy: row.inenergy.clone(),
            xnenergy: row.xnenergy.clone(),
            ipower: row.ipower.clone(),
            xpower: row.xpower.clone(),
            rrp: row.rrp.clone(),
            eep: row.eep.clone(),
            tlf: row.tlf.clone(),
            cprrp: row.cprrp.clone(),
            cpeep: row.cpeep.clone(),
            ta: row.ta.clone(),
            ep: row.ep.clone(),
            apc: row.apc.clone(),
            resc: row.resc.clone(),
            resp: row.resp.clone(),
            meterrunno: row.meterrunno.clone(),
            lastchanged: row.lastchanged.clone(),
            hostdistributor: row.hostdistributor.clone(),
            mda: row.mda.clone(),
            afe: row.afe.clone(),
            dme: row.dme.clone(),
            ufea: row.ufea.clone(),
            age: row.age.clone(),
            importenergycost: row.importenergycost.clone(),
            exportenergycost: row.exportenergycost.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsCpdata7PrimaryKey {
    pub mda: alloc::string::String,
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub tcpid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsCpdata7PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsCpdata7Row<'data> {
    type Row<'other> = SettlementsCpdata7Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.mda() == row.mda() && self.participantid() == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.tcpid() == row.tcpid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsCpdata7Row<'data> {
    type PrimaryKey = SettlementsCpdata7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mda() == key.mda && self.participantid() == key.participantid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
            && self.tcpid() == key.tcpid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsCpdata7PrimaryKey {
    type Row<'other> = SettlementsCpdata7Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.mda == row.mda() && self.participantid == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.tcpid == row.tcpid() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsCpdata7PrimaryKey {
    type PrimaryKey = SettlementsCpdata7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mda == key.mda && self.participantid == key.participantid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
            && self.tcpid == key.tcpid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsCpdata7 {
    type Builder = SettlementsCpdata7Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "tcpid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "igenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "xgenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "inenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "xnenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ipower",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "xpower",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp",
                    arrow::datatypes::DataType::Decimal128(20, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "eep",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "tlf",
                    arrow::datatypes::DataType::Decimal128(7, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cprrp",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cpeep",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ta",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ep",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "apc",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "resc",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "resp",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "meterrunno",
                    arrow::datatypes::DataType::Decimal128(10, 0),
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
                    "hostdistributor",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mda",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "afe",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dme",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ufea",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "age",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "importenergycost",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "exportenergycost",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsCpdata7Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            tcpid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            igenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            xgenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            inenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            xnenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            ipower_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            xpower_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(20, 5)),
            eep_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            tlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(7, 5)),
            cprrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            cpeep_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            ta_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            ep_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            apc_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            resc_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            resp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            meterrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            hostdistributor_array: arrow::array::builder::StringBuilder::new(),
            mda_array: arrow::array::builder::StringBuilder::new(),
            afe_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            dme_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            ufea_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            age_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            importenergycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            exportenergycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
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
        builder.participantid_array.append_value(row.participantid());
        builder.tcpid_array.append_value(row.tcpid());
        builder.regionid_array.append_option(row.regionid());
        builder
            .igenergy_array
            .append_option({
                row.igenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .xgenergy_array
            .append_option({
                row.xgenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .inenergy_array
            .append_option({
                row.inenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .xnenergy_array
            .append_option({
                row.xnenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .ipower_array
            .append_option({
                row.ipower
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .xpower_array
            .append_option({
                row.xpower
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .rrp_array
            .append_option({
                row.rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .eep_array
            .append_option({
                row.eep
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .tlf_array
            .append_option({
                row.tlf
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .cprrp_array
            .append_option({
                row.cprrp
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .cpeep_array
            .append_option({
                row.cpeep
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .ta_array
            .append_option({
                row.ta
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .ep_array
            .append_option({
                row.ep
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .apc_array
            .append_option({
                row.apc
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .resc_array
            .append_option({
                row.resc
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .resp_array
            .append_option({
                row.resp
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .meterrunno_array
            .append_option({
                row.meterrunno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder.hostdistributor_array.append_option(row.hostdistributor());
        builder.mda_array.append_value(row.mda());
        builder
            .afe_array
            .append_option({
                row.afe
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .dme_array
            .append_option({
                row.dme
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .ufea_array
            .append_option({
                row.ufea
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .age_array
            .append_option({
                row.age
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .importenergycost_array
            .append_option({
                row.importenergycost
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .exportenergycost_array
            .append_option({
                row.exportenergycost
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tcpid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.igenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.xgenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.inenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.xnenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ipower_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.xpower_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.eep_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cprrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cpeep_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ta_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ep_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.apc_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.resc_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.resp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meterrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.hostdistributor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mda_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.afe_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dme_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ufea_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.age_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.importenergycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exportenergycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsCpdata7Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    tcpid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    igenergy_array: arrow::array::builder::Decimal128Builder,
    xgenergy_array: arrow::array::builder::Decimal128Builder,
    inenergy_array: arrow::array::builder::Decimal128Builder,
    xnenergy_array: arrow::array::builder::Decimal128Builder,
    ipower_array: arrow::array::builder::Decimal128Builder,
    xpower_array: arrow::array::builder::Decimal128Builder,
    rrp_array: arrow::array::builder::Decimal128Builder,
    eep_array: arrow::array::builder::Decimal128Builder,
    tlf_array: arrow::array::builder::Decimal128Builder,
    cprrp_array: arrow::array::builder::Decimal128Builder,
    cpeep_array: arrow::array::builder::Decimal128Builder,
    ta_array: arrow::array::builder::Decimal128Builder,
    ep_array: arrow::array::builder::Decimal128Builder,
    apc_array: arrow::array::builder::Decimal128Builder,
    resc_array: arrow::array::builder::Decimal128Builder,
    resp_array: arrow::array::builder::Decimal128Builder,
    meterrunno_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    hostdistributor_array: arrow::array::builder::StringBuilder,
    mda_array: arrow::array::builder::StringBuilder,
    afe_array: arrow::array::builder::Decimal128Builder,
    dme_array: arrow::array::builder::Decimal128Builder,
    ufea_array: arrow::array::builder::Decimal128Builder,
    age_array: arrow::array::builder::Decimal128Builder,
    importenergycost_array: arrow::array::builder::Decimal128Builder,
    exportenergycost_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsCpdataregion5;
pub struct SettlementsCpdataregion5Mapping([usize; 12]);
/// # Summary
///
/// ## SETCPDATAREGION
///  _SETCPDATAREGION sets out summary meter settlement data for each region._
///
/// * Data Set Name: Settlements
/// * File Name: Cpdataregion
/// * Data Version: 5
///
/// # Description
///  SETCPDATAREGION data is public, so is available to all participants. Source SETCPDATAREGION is a summary based on grouping on SETCPDATA and is updated with each settlement run.
///
///
///
/// # Primary Key Columns
///
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsCpdataregion5Row<'data> {
    /// Calendar Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Import Gross energy into the pool - MWh
    pub sumigenergy: Option<rust_decimal::Decimal>,
    /// Export Gross energy from the pool - MWh
    pub sumxgenergy: Option<rust_decimal::Decimal>,
    /// Import Nett energy into the pool - MWh
    pub suminenergy: Option<rust_decimal::Decimal>,
    /// Export Nett energy from the pool - MWh
    pub sumxnenergy: Option<rust_decimal::Decimal>,
    /// Not used
    pub sumipower: Option<rust_decimal::Decimal>,
    /// Not used
    pub sumxpower: Option<rust_decimal::Decimal>,
    /// current system date, to enable automatic replication
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Sum of energy price across the region
    pub sumep: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsCpdataregion5Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsCpdataregion5 {
    const VERSION: i32 = 5;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "CPDATAREGION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsCpdataregion5Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PERIODID",
        "REGIONID",
        "SUMIGENERGY",
        "SUMXGENERGY",
        "SUMINENERGY",
        "SUMXNENERGY",
        "SUMIPOWER",
        "SUMXPOWER",
        "LASTCHANGED",
        "SUMEP",
    ];
    type Row<'row> = SettlementsCpdataregion5Row<'row>;
    type FieldMapping = SettlementsCpdataregion5Mapping;
    type PrimaryKey = SettlementsCpdataregion5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsCpdataregion5Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            sumigenergy: row
                .get_opt_custom_parsed_at_idx(
                    "sumigenergy",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sumxgenergy: row
                .get_opt_custom_parsed_at_idx(
                    "sumxgenergy",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            suminenergy: row
                .get_opt_custom_parsed_at_idx(
                    "suminenergy",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sumxnenergy: row
                .get_opt_custom_parsed_at_idx(
                    "sumxnenergy",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sumipower: row
                .get_opt_custom_parsed_at_idx(
                    "sumipower",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sumxpower: row
                .get_opt_custom_parsed_at_idx(
                    "sumxpower",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            sumep: row
                .get_opt_custom_parsed_at_idx(
                    "sumep",
                    field_mapping.0[11],
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
        Ok(SettlementsCpdataregion5Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsCpdataregion5PrimaryKey {
        SettlementsCpdataregion5PrimaryKey {
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_cpdataregion_v5_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsCpdataregion5Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            periodid: row.periodid.clone(),
            regionid: row.regionid.clone(),
            sumigenergy: row.sumigenergy.clone(),
            sumxgenergy: row.sumxgenergy.clone(),
            suminenergy: row.suminenergy.clone(),
            sumxnenergy: row.sumxnenergy.clone(),
            sumipower: row.sumipower.clone(),
            sumxpower: row.sumxpower.clone(),
            lastchanged: row.lastchanged.clone(),
            sumep: row.sumep.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsCpdataregion5PrimaryKey {
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsCpdataregion5PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsCpdataregion5Row<'data> {
    type Row<'other> = SettlementsCpdataregion5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.periodid == row.periodid && self.regionid() == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsCpdataregion5Row<'data> {
    type PrimaryKey = SettlementsCpdataregion5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid() == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsCpdataregion5PrimaryKey {
    type Row<'other> = SettlementsCpdataregion5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.periodid == row.periodid && self.regionid == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsCpdataregion5PrimaryKey {
    type PrimaryKey = SettlementsCpdataregion5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsCpdataregion5 {
    type Builder = SettlementsCpdataregion5Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(22, 10),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(22, 10),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "sumigenergy",
                    arrow::datatypes::DataType::Decimal128(27, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sumxgenergy",
                    arrow::datatypes::DataType::Decimal128(27, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "suminenergy",
                    arrow::datatypes::DataType::Decimal128(27, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sumxnenergy",
                    arrow::datatypes::DataType::Decimal128(27, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sumipower",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sumxpower",
                    arrow::datatypes::DataType::Decimal128(22, 0),
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
                    "sumep",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsCpdataregion5Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 10)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 10)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            sumigenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 5)),
            sumxgenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 5)),
            suminenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 5)),
            sumxnenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 5)),
            sumipower_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            sumxpower_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            sumep_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(10);
                val.mantissa()
            });
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(10);
                val.mantissa()
            });
        builder.regionid_array.append_value(row.regionid());
        builder
            .sumigenergy_array
            .append_option({
                row.sumigenergy
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .sumxgenergy_array
            .append_option({
                row.sumxgenergy
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .suminenergy_array
            .append_option({
                row.suminenergy
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .sumxnenergy_array
            .append_option({
                row.sumxnenergy
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .sumipower_array
            .append_option({
                row.sumipower
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .sumxpower_array
            .append_option({
                row.sumxpower
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .sumep_array
            .append_option({
                row.sumep
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sumigenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sumxgenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.suminenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sumxnenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sumipower_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sumxpower_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sumep_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsCpdataregion5Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    sumigenergy_array: arrow::array::builder::Decimal128Builder,
    sumxgenergy_array: arrow::array::builder::Decimal128Builder,
    suminenergy_array: arrow::array::builder::Decimal128Builder,
    sumxnenergy_array: arrow::array::builder::Decimal128Builder,
    sumipower_array: arrow::array::builder::Decimal128Builder,
    sumxpower_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    sumep_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsFcasregionrecovery5;
pub struct SettlementsFcasregionrecovery5Mapping([usize; 9]);
/// # Summary
///
/// ## SETFCASREGIONRECOVERY
///  _SETFCASREGIONRECOVERY shows FCAS Regional Recovery Data against each Trading Interval._
///
/// * Data Set Name: Settlements
/// * File Name: Fcasregionrecovery
/// * Data Version: 5
///
/// # Description
///  SETFCASREGIONRECOVERY contains public data and is available to all participants. Source SETFCASREGIONRECOVERY updates with each settlements run. Volume Approximately 10,000 rows per day
///
///
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsFcasregionrecovery5Row<'data> {
    /// Settlement Date of trading interval
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// FCAS Service Type
    pub bidtype: core::ops::Range<usize>,
    /// RegionID
    pub regionid: core::ops::Range<usize>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Generator Regional Energy Amount
    pub generatorregionenergy: Option<rust_decimal::Decimal>,
    /// Customer Region Energy Amount
    pub customerregionenergy: Option<rust_decimal::Decimal>,
    /// The NEM Regional Recovery Amount for FCAS
    pub regionrecovery: Option<rust_decimal::Decimal>,
    /// Last Date record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsFcasregionrecovery5Row<'data> {
    pub fn bidtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bidtype.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsFcasregionrecovery5 {
    const VERSION: i32 = 5;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "FCASREGIONRECOVERY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsFcasregionrecovery5Mapping([
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
        "SETTLEMENTDATE",
        "VERSIONNO",
        "BIDTYPE",
        "REGIONID",
        "PERIODID",
        "GENERATORREGIONENERGY",
        "CUSTOMERREGIONENERGY",
        "REGIONRECOVERY",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementsFcasregionrecovery5Row<'row>;
    type FieldMapping = SettlementsFcasregionrecovery5Mapping;
    type PrimaryKey = SettlementsFcasregionrecovery5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsFcasregionrecovery5Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bidtype: row.get_range("bidtype", field_mapping.0[2])?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            generatorregionenergy: row
                .get_opt_custom_parsed_at_idx(
                    "generatorregionenergy",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            customerregionenergy: row
                .get_opt_custom_parsed_at_idx(
                    "customerregionenergy",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionrecovery: row
                .get_opt_custom_parsed_at_idx(
                    "regionrecovery",
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
        Ok(SettlementsFcasregionrecovery5Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsFcasregionrecovery5PrimaryKey {
        SettlementsFcasregionrecovery5PrimaryKey {
            bidtype: row.bidtype().to_string(),
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_fcasregionrecovery_v5_{}_{}", Self::partition_suffix(& row)
            .year, Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsFcasregionrecovery5Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            bidtype: row.bidtype.clone(),
            regionid: row.regionid.clone(),
            periodid: row.periodid.clone(),
            generatorregionenergy: row.generatorregionenergy.clone(),
            customerregionenergy: row.customerregionenergy.clone(),
            regionrecovery: row.regionrecovery.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsFcasregionrecovery5PrimaryKey {
    pub bidtype: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsFcasregionrecovery5PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsFcasregionrecovery5Row<'data> {
    type Row<'other> = SettlementsFcasregionrecovery5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype() && self.periodid == row.periodid
            && self.regionid() == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementsFcasregionrecovery5Row<'data> {
    type PrimaryKey = SettlementsFcasregionrecovery5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype && self.periodid == key.periodid
            && self.regionid() == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsFcasregionrecovery5PrimaryKey {
    type Row<'other> = SettlementsFcasregionrecovery5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype() && self.periodid == row.periodid
            && self.regionid == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsFcasregionrecovery5PrimaryKey {
    type PrimaryKey = SettlementsFcasregionrecovery5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.periodid == key.periodid
            && self.regionid == key.regionid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsFcasregionrecovery5 {
    type Builder = SettlementsFcasregionrecovery5Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
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
                    "bidtype",
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
                    "generatorregionenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "customerregionenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regionrecovery",
                    arrow::datatypes::DataType::Decimal128(18, 8),
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
        SettlementsFcasregionrecovery5Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            bidtype_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            generatorregionenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            customerregionenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            regionrecovery_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.bidtype_array.append_value(row.bidtype());
        builder.regionid_array.append_value(row.regionid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .generatorregionenergy_array
            .append_option({
                row.generatorregionenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .customerregionenergy_array
            .append_option({
                row.customerregionenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .regionrecovery_array
            .append_option({
                row.regionrecovery
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.generatorregionenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.customerregionenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionrecovery_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsFcasregionrecovery5Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    bidtype_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    generatorregionenergy_array: arrow::array::builder::Decimal128Builder,
    customerregionenergy_array: arrow::array::builder::Decimal128Builder,
    regionrecovery_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementsGendata6;
pub struct SettlementsGendata6Mapping([usize; 29]);
/// # Summary
///
/// ## SETGENDATA
///  _SETGENDATA shows meter settlement data for each generation meter point. A regional summary is also provided._
///
/// * Data Set Name: Settlements
/// * File Name: Gendata
/// * Data Version: 6
///
/// # Description
///  SETGENDATA shows generator meter details, and SETGENDATA data is confidential to the participant. By comparison, the regional summary (SETGENDATAREGION) is public data. Source SETGENDATA updates with each Settlement run.
///
///
///
/// # Primary Key Columns
///
/// * DUID
/// * GENSETID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * STATIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsGendata6Row<'data> {
    /// Calendar Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Station Identifier
    pub stationid: core::ops::Range<usize>,
    /// Dispatchable Unit identifier
    pub duid: core::ops::Range<usize>,
    /// Physical unit identifier
    pub gensetid: core::ops::Range<usize>,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Generated energy
    pub genergy: Option<rust_decimal::Decimal>,
    /// Purchased Energy
    pub aenergy: Option<rust_decimal::Decimal>,
    /// Not used
    pub gpower: Option<rust_decimal::Decimal>,
    /// Not used
    pub apower: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess Energy Price
    pub eep: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Connection Point Price = RRP * TLF
    pub cprrp: Option<rust_decimal::Decimal>,
    /// Connection Point Excess Energy Price = EEP * TLF
    pub cpeep: Option<rust_decimal::Decimal>,
    /// Generated energy
    pub netenergy: Option<rust_decimal::Decimal>,
    /// Cost of net energy $
    pub energycost: Option<rust_decimal::Decimal>,
    /// Cost of excess energy $
    pub excessenergycost: Option<rust_decimal::Decimal>,
    /// Administered Price Compensation
    pub apc: Option<rust_decimal::Decimal>,
    /// Not used
    pub resc: Option<rust_decimal::Decimal>,
    /// Not used
    pub resp: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Export Energy (Generator Purchases) (MWh)
    pub expenergy: Option<rust_decimal::Decimal>,
    /// Export Energy Cost ($)
    pub expenergycost: Option<rust_decimal::Decimal>,
    /// Identifier of the meter run used in this settlement calculation
    pub meterrunno: Option<rust_decimal::Decimal>,
    /// Metering Data Agent
    pub mda: core::ops::Range<usize>,
    /// Secondary Transmission Loss Factor
    pub secondary_tlf: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsGendata6Row<'data> {
    pub fn participantid(&self) -> Option<&str> {
        if self.participantid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.participantid.clone(),
                ),
            )
        }
    }
    pub fn stationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.stationid.clone())
    }
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
    }
    pub fn gensetid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.gensetid.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn mda(&self) -> Option<&str> {
        if self.mda.is_empty() {
            None
        } else {
            Some(core::ops::Index::index(self.backing_data.as_slice(), self.mda.clone()))
        }
    }
}
impl mmsdm_core::GetTable for SettlementsGendata6 {
    const VERSION: i32 = 6;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "GENDATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsGendata6Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PERIODID",
        "PARTICIPANTID",
        "STATIONID",
        "DUID",
        "GENSETID",
        "REGIONID",
        "GENERGY",
        "AENERGY",
        "GPOWER",
        "APOWER",
        "RRP",
        "EEP",
        "TLF",
        "CPRRP",
        "CPEEP",
        "NETENERGY",
        "ENERGYCOST",
        "EXCESSENERGYCOST",
        "APC",
        "RESC",
        "RESP",
        "LASTCHANGED",
        "EXPENERGY",
        "EXPENERGYCOST",
        "METERRUNNO",
        "MDA",
        "SECONDARY_TLF",
    ];
    type Row<'row> = SettlementsGendata6Row<'row>;
    type FieldMapping = SettlementsGendata6Mapping;
    type PrimaryKey = SettlementsGendata6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsGendata6Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_opt_range("participantid", field_mapping.0[3])?,
            stationid: row.get_range("stationid", field_mapping.0[4])?,
            duid: row.get_range("duid", field_mapping.0[5])?,
            gensetid: row.get_range("gensetid", field_mapping.0[6])?,
            regionid: row.get_range("regionid", field_mapping.0[7])?,
            genergy: row
                .get_opt_custom_parsed_at_idx(
                    "genergy",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            aenergy: row
                .get_opt_custom_parsed_at_idx(
                    "aenergy",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            gpower: row
                .get_opt_custom_parsed_at_idx(
                    "gpower",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            apower: row
                .get_opt_custom_parsed_at_idx(
                    "apower",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp: row
                .get_opt_custom_parsed_at_idx(
                    "rrp",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            eep: row
                .get_opt_custom_parsed_at_idx(
                    "eep",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            tlf: row
                .get_opt_custom_parsed_at_idx(
                    "tlf",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cprrp: row
                .get_opt_custom_parsed_at_idx(
                    "cprrp",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            cpeep: row
                .get_opt_custom_parsed_at_idx(
                    "cpeep",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            netenergy: row
                .get_opt_custom_parsed_at_idx(
                    "netenergy",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energycost: row
                .get_opt_custom_parsed_at_idx(
                    "energycost",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            excessenergycost: row
                .get_opt_custom_parsed_at_idx(
                    "excessenergycost",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            apc: row
                .get_opt_custom_parsed_at_idx(
                    "apc",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            resc: row
                .get_opt_custom_parsed_at_idx(
                    "resc",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            resp: row
                .get_opt_custom_parsed_at_idx(
                    "resp",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[23],
                    mmsdm_core::mms_datetime::parse,
                )?,
            expenergy: row
                .get_opt_custom_parsed_at_idx(
                    "expenergy",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            expenergycost: row
                .get_opt_custom_parsed_at_idx(
                    "expenergycost",
                    field_mapping.0[25],
                    mmsdm_core::mms_decimal::parse,
                )?,
            meterrunno: row
                .get_opt_custom_parsed_at_idx(
                    "meterrunno",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mda: row.get_opt_range("mda", field_mapping.0[27])?,
            secondary_tlf: row
                .get_opt_custom_parsed_at_idx(
                    "secondary_tlf",
                    field_mapping.0[28],
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
        Ok(SettlementsGendata6Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsGendata6PrimaryKey {
        SettlementsGendata6PrimaryKey {
            duid: row.duid().to_string(),
            gensetid: row.gensetid().to_string(),
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            settlementdate: row.settlementdate,
            stationid: row.stationid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_gendata_v6_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsGendata6Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            periodid: row.periodid.clone(),
            participantid: row.participantid.clone(),
            stationid: row.stationid.clone(),
            duid: row.duid.clone(),
            gensetid: row.gensetid.clone(),
            regionid: row.regionid.clone(),
            genergy: row.genergy.clone(),
            aenergy: row.aenergy.clone(),
            gpower: row.gpower.clone(),
            apower: row.apower.clone(),
            rrp: row.rrp.clone(),
            eep: row.eep.clone(),
            tlf: row.tlf.clone(),
            cprrp: row.cprrp.clone(),
            cpeep: row.cpeep.clone(),
            netenergy: row.netenergy.clone(),
            energycost: row.energycost.clone(),
            excessenergycost: row.excessenergycost.clone(),
            apc: row.apc.clone(),
            resc: row.resc.clone(),
            resp: row.resp.clone(),
            lastchanged: row.lastchanged.clone(),
            expenergy: row.expenergy.clone(),
            expenergycost: row.expenergycost.clone(),
            meterrunno: row.meterrunno.clone(),
            mda: row.mda.clone(),
            secondary_tlf: row.secondary_tlf.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsGendata6PrimaryKey {
    pub duid: alloc::string::String,
    pub gensetid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub stationid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsGendata6PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsGendata6Row<'data> {
    type Row<'other> = SettlementsGendata6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.gensetid() == row.gensetid()
            && self.periodid == row.periodid && self.regionid() == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.stationid() == row.stationid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsGendata6Row<'data> {
    type PrimaryKey = SettlementsGendata6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.gensetid() == key.gensetid
            && self.periodid == key.periodid && self.regionid() == key.regionid
            && self.settlementdate == key.settlementdate
            && self.stationid() == key.stationid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsGendata6PrimaryKey {
    type Row<'other> = SettlementsGendata6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.gensetid == row.gensetid()
            && self.periodid == row.periodid && self.regionid == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.stationid == row.stationid() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsGendata6PrimaryKey {
    type PrimaryKey = SettlementsGendata6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.gensetid == key.gensetid
            && self.periodid == key.periodid && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.stationid == key.stationid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsGendata6 {
    type Builder = SettlementsGendata6Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "stationid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "gensetid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "genergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "aenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "gpower",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "apower",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp",
                    arrow::datatypes::DataType::Decimal128(20, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "eep",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "tlf",
                    arrow::datatypes::DataType::Decimal128(7, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cprrp",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "cpeep",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "netenergy",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energycost",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "excessenergycost",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "apc",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "resc",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "resp",
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
                arrow::datatypes::Field::new(
                    "expenergy",
                    arrow::datatypes::DataType::Decimal128(15, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "expenergycost",
                    arrow::datatypes::DataType::Decimal128(15, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "meterrunno",
                    arrow::datatypes::DataType::Decimal128(6, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mda",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "secondary_tlf",
                    arrow::datatypes::DataType::Decimal128(7, 5),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsGendata6Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            stationid_array: arrow::array::builder::StringBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            gensetid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            genergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            aenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            gpower_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            apower_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(20, 5)),
            eep_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            tlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(7, 5)),
            cprrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            cpeep_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            netenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            energycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            excessenergycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            apc_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            resc_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            resp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            expenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 6)),
            expenergycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 6)),
            meterrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(6, 0)),
            mda_array: arrow::array::builder::StringBuilder::new(),
            secondary_tlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(7, 5)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
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
        builder.participantid_array.append_option(row.participantid());
        builder.stationid_array.append_value(row.stationid());
        builder.duid_array.append_value(row.duid());
        builder.gensetid_array.append_value(row.gensetid());
        builder.regionid_array.append_value(row.regionid());
        builder
            .genergy_array
            .append_option({
                row.genergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .aenergy_array
            .append_option({
                row.aenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .gpower_array
            .append_option({
                row.gpower
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .apower_array
            .append_option({
                row.apower
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .rrp_array
            .append_option({
                row.rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .eep_array
            .append_option({
                row.eep
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .tlf_array
            .append_option({
                row.tlf
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .cprrp_array
            .append_option({
                row.cprrp
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .cpeep_array
            .append_option({
                row.cpeep
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .netenergy_array
            .append_option({
                row.netenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .energycost_array
            .append_option({
                row.energycost
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .excessenergycost_array
            .append_option({
                row.excessenergycost
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .apc_array
            .append_option({
                row.apc
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .resc_array
            .append_option({
                row.resc
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .resp_array
            .append_option({
                row.resp
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .expenergy_array
            .append_option({
                row.expenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .expenergycost_array
            .append_option({
                row.expenergycost
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .meterrunno_array
            .append_option({
                row.meterrunno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.mda_array.append_option(row.mda());
        builder
            .secondary_tlf_array
            .append_option({
                row.secondary_tlf
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.stationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gensetid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.aenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gpower_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.apower_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.eep_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cprrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cpeep_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.netenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.energycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.excessenergycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.apc_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.resc_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.resp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.expenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.expenergycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.meterrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mda_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.secondary_tlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsGendata6Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    stationid_array: arrow::array::builder::StringBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    gensetid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    genergy_array: arrow::array::builder::Decimal128Builder,
    aenergy_array: arrow::array::builder::Decimal128Builder,
    gpower_array: arrow::array::builder::Decimal128Builder,
    apower_array: arrow::array::builder::Decimal128Builder,
    rrp_array: arrow::array::builder::Decimal128Builder,
    eep_array: arrow::array::builder::Decimal128Builder,
    tlf_array: arrow::array::builder::Decimal128Builder,
    cprrp_array: arrow::array::builder::Decimal128Builder,
    cpeep_array: arrow::array::builder::Decimal128Builder,
    netenergy_array: arrow::array::builder::Decimal128Builder,
    energycost_array: arrow::array::builder::Decimal128Builder,
    excessenergycost_array: arrow::array::builder::Decimal128Builder,
    apc_array: arrow::array::builder::Decimal128Builder,
    resc_array: arrow::array::builder::Decimal128Builder,
    resp_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    expenergy_array: arrow::array::builder::Decimal128Builder,
    expenergycost_array: arrow::array::builder::Decimal128Builder,
    meterrunno_array: arrow::array::builder::Decimal128Builder,
    mda_array: arrow::array::builder::StringBuilder,
    secondary_tlf_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsGendataregion5;
pub struct SettlementsGendataregion5Mapping([usize; 14]);
/// # Summary
///
/// ## SETGENDATAREGION
///  _SETGENDATAREGION sets out summary settlement data for generation within the specified region._
///
/// * Data Set Name: Settlements
/// * File Name: Gendataregion
/// * Data Version: 5
///
/// # Description
///  SETGENDATAREGION shows the regional summary. SETGENDATAREGION is public data. Source SETGENDATAREGION updates with each Settlement run.
///
///
///
/// # Primary Key Columns
///
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsGendataregion5Row<'data> {
    /// Calendar Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Generated energy - Not used in MMS Data Model
    pub genergy: Option<rust_decimal::Decimal>,
    /// Purchased Energy - Not used in MMS Data Model
    pub aenergy: Option<rust_decimal::Decimal>,
    /// Not used in MMS Data Model
    pub gpower: Option<rust_decimal::Decimal>,
    /// Not used in MMS Data Model
    pub apower: Option<rust_decimal::Decimal>,
    /// Net energy MW/hours
    pub netenergy: Option<rust_decimal::Decimal>,
    /// Cost of net energy $
    pub energycost: Option<rust_decimal::Decimal>,
    /// Cost of excess energy $
    pub excessenergycost: Option<rust_decimal::Decimal>,
    /// Export Energy (Generator Purchases)
    pub expenergy: Option<rust_decimal::Decimal>,
    /// Export Energy Cost
    pub expenergycost: Option<rust_decimal::Decimal>,
    /// current system date, to enable automatic replication
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsGendataregion5Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsGendataregion5 {
    const VERSION: i32 = 5;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "GENDATAREGION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsGendataregion5Mapping([
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
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PERIODID",
        "REGIONID",
        "GENERGY",
        "AENERGY",
        "GPOWER",
        "APOWER",
        "NETENERGY",
        "ENERGYCOST",
        "EXCESSENERGYCOST",
        "EXPENERGY",
        "EXPENERGYCOST",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementsGendataregion5Row<'row>;
    type FieldMapping = SettlementsGendataregion5Mapping;
    type PrimaryKey = SettlementsGendataregion5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsGendataregion5Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            genergy: row
                .get_opt_custom_parsed_at_idx(
                    "genergy",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            aenergy: row
                .get_opt_custom_parsed_at_idx(
                    "aenergy",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            gpower: row
                .get_opt_custom_parsed_at_idx(
                    "gpower",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            apower: row
                .get_opt_custom_parsed_at_idx(
                    "apower",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            netenergy: row
                .get_opt_custom_parsed_at_idx(
                    "netenergy",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energycost: row
                .get_opt_custom_parsed_at_idx(
                    "energycost",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            excessenergycost: row
                .get_opt_custom_parsed_at_idx(
                    "excessenergycost",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            expenergy: row
                .get_opt_custom_parsed_at_idx(
                    "expenergy",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            expenergycost: row
                .get_opt_custom_parsed_at_idx(
                    "expenergycost",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[13],
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
        Ok(SettlementsGendataregion5Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsGendataregion5PrimaryKey {
        SettlementsGendataregion5PrimaryKey {
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_gendataregion_v5_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsGendataregion5Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            periodid: row.periodid.clone(),
            regionid: row.regionid.clone(),
            genergy: row.genergy.clone(),
            aenergy: row.aenergy.clone(),
            gpower: row.gpower.clone(),
            apower: row.apower.clone(),
            netenergy: row.netenergy.clone(),
            energycost: row.energycost.clone(),
            excessenergycost: row.excessenergycost.clone(),
            expenergy: row.expenergy.clone(),
            expenergycost: row.expenergycost.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsGendataregion5PrimaryKey {
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsGendataregion5PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsGendataregion5Row<'data> {
    type Row<'other> = SettlementsGendataregion5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.periodid == row.periodid && self.regionid() == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsGendataregion5Row<'data> {
    type PrimaryKey = SettlementsGendataregion5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid() == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsGendataregion5PrimaryKey {
    type Row<'other> = SettlementsGendataregion5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.periodid == row.periodid && self.regionid == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsGendataregion5PrimaryKey {
    type PrimaryKey = SettlementsGendataregion5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsGendataregion5 {
    type Builder = SettlementsGendataregion5Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(22, 10),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(22, 10),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "genergy",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "aenergy",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "gpower",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "apower",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "netenergy",
                    arrow::datatypes::DataType::Decimal128(27, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energycost",
                    arrow::datatypes::DataType::Decimal128(27, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "excessenergycost",
                    arrow::datatypes::DataType::Decimal128(27, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "expenergy",
                    arrow::datatypes::DataType::Decimal128(27, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "expenergycost",
                    arrow::datatypes::DataType::Decimal128(27, 6),
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
        SettlementsGendataregion5Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 10)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 10)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            genergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            aenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            gpower_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            apower_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            netenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 5)),
            energycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 5)),
            excessenergycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 5)),
            expenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 6)),
            expenergycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(27, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(10);
                val.mantissa()
            });
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(10);
                val.mantissa()
            });
        builder.regionid_array.append_value(row.regionid());
        builder
            .genergy_array
            .append_option({
                row.genergy
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .aenergy_array
            .append_option({
                row.aenergy
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .gpower_array
            .append_option({
                row.gpower
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .apower_array
            .append_option({
                row.apower
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .netenergy_array
            .append_option({
                row.netenergy
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .energycost_array
            .append_option({
                row.energycost
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .excessenergycost_array
            .append_option({
                row.excessenergycost
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .expenergy_array
            .append_option({
                row.expenergy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .expenergycost_array
            .append_option({
                row.expenergycost
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.aenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.gpower_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.apower_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.netenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.energycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.excessenergycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.expenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.expenergycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsGendataregion5Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    genergy_array: arrow::array::builder::Decimal128Builder,
    aenergy_array: arrow::array::builder::Decimal128Builder,
    gpower_array: arrow::array::builder::Decimal128Builder,
    apower_array: arrow::array::builder::Decimal128Builder,
    netenergy_array: arrow::array::builder::Decimal128Builder,
    energycost_array: arrow::array::builder::Decimal128Builder,
    excessenergycost_array: arrow::array::builder::Decimal128Builder,
    expenergy_array: arrow::array::builder::Decimal128Builder,
    expenergycost_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementsIntraregionresidues5;
pub struct SettlementsIntraregionresidues5Mapping([usize; 10]);
/// # Summary
///
/// ## SETINTRAREGIONRESIDUES
///  _&nbsp; _
///
/// * Data Set Name: Settlements
/// * File Name: Intraregionresidues
/// * Data Version: 5
///
/// # Description
///  SETINTRAREGIONRESIDUES data is public to all participants. Source SETINTRAREGIONRESIDUES updates with each settlement run. Note The relationship between the data columns for each key is expressed in the following formula: EP + EC + (EXP * RRP) = IRSS
///
///
///
/// # Primary Key Columns
///
/// * PERIODID
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsIntraregionresidues5Row<'data> {
    /// Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub runno: i64,
    /// Settlements Trading Interval.
    pub periodid: i64,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Energy payments to generators
    pub ep: Option<rust_decimal::Decimal>,
    /// Energy purchased by customers
    pub ec: Option<rust_decimal::Decimal>,
    /// Regional price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Net import in MWh into the region calculated at the regional reference node (export is negative)
    pub exp: Option<rust_decimal::Decimal>,
    /// Intra-regional surplus (a negative sign indicates surplus, and a positive sign indicates a deficiency)
    pub irss: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsIntraregionresidues5Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsIntraregionresidues5 {
    const VERSION: i32 = 5;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "INTRAREGIONRESIDUES";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsIntraregionresidues5Mapping([
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
        "SETTLEMENTDATE",
        "RUNNO",
        "PERIODID",
        "REGIONID",
        "EP",
        "EC",
        "RRP",
        "EXP",
        "IRSS",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementsIntraregionresidues5Row<'row>;
    type FieldMapping = SettlementsIntraregionresidues5Mapping;
    type PrimaryKey = SettlementsIntraregionresidues5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsIntraregionresidues5Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row.get_parsed_at_idx("runno", field_mapping.0[1])?,
            periodid: row.get_parsed_at_idx("periodid", field_mapping.0[2])?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            ep: row
                .get_opt_custom_parsed_at_idx(
                    "ep",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ec: row
                .get_opt_custom_parsed_at_idx(
                    "ec",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp: row
                .get_opt_custom_parsed_at_idx(
                    "rrp",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            exp: row
                .get_opt_custom_parsed_at_idx(
                    "exp",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            irss: row
                .get_opt_custom_parsed_at_idx(
                    "irss",
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
        Ok(SettlementsIntraregionresidues5Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsIntraregionresidues5PrimaryKey {
        SettlementsIntraregionresidues5PrimaryKey {
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            runno: row.runno,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_intraregionresidues_v5_{}_{}", Self::partition_suffix(& row)
            .year, Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsIntraregionresidues5Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            periodid: row.periodid.clone(),
            regionid: row.regionid.clone(),
            ep: row.ep.clone(),
            ec: row.ec.clone(),
            rrp: row.rrp.clone(),
            exp: row.exp.clone(),
            irss: row.irss.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsIntraregionresidues5PrimaryKey {
    pub periodid: i64,
    pub regionid: alloc::string::String,
    pub runno: i64,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for SettlementsIntraregionresidues5PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsIntraregionresidues5Row<'data> {
    type Row<'other> = SettlementsIntraregionresidues5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.periodid == row.periodid && self.regionid() == row.regionid()
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementsIntraregionresidues5Row<'data> {
    type PrimaryKey = SettlementsIntraregionresidues5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid() == key.regionid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsIntraregionresidues5PrimaryKey {
    type Row<'other> = SettlementsIntraregionresidues5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.periodid == row.periodid && self.regionid == row.regionid()
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIntraregionresidues5PrimaryKey {
    type PrimaryKey = SettlementsIntraregionresidues5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid == key.regionid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsIntraregionresidues5 {
    type Builder = SettlementsIntraregionresidues5Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "ep",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ec",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "exp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "irss",
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
        SettlementsIntraregionresidues5Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Int64Builder::new(),
            periodid_array: arrow::array::builder::Int64Builder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            ep_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            ec_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            exp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            irss_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder.runno_array.append_value(row.runno);
        builder.periodid_array.append_value(row.periodid);
        builder.regionid_array.append_value(row.regionid());
        builder
            .ep_array
            .append_option({
                row.ep
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .ec_array
            .append_option({
                row.ec
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp_array
            .append_option({
                row.rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .exp_array
            .append_option({
                row.exp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .irss_array
            .append_option({
                row.irss
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ep_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ec_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.irss_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsIntraregionresidues5Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Int64Builder,
    periodid_array: arrow::array::builder::Int64Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    ep_array: arrow::array::builder::Decimal128Builder,
    ec_array: arrow::array::builder::Decimal128Builder,
    rrp_array: arrow::array::builder::Decimal128Builder,
    exp_array: arrow::array::builder::Decimal128Builder,
    irss_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementsIraucsurplus6;
pub struct SettlementsIraucsurplus6Mapping([usize; 13]);
/// # Summary
///
/// ## SETIRAUCSURPLUS
///  _This view supports the Settlements Residue Auction, by holding the NSP participant allocations of IRSurplus arising as a result of the unsold units for a quarter._
///
/// * Data Set Name: Settlements
/// * File Name: Iraucsurplus
/// * Data Version: 6
///
/// # Description
///  SETIRAUCSURPLUS data is confidential to the relevant participant. Source SETIRAUCSURPLUS updates with each settlement run. Volume SETIRAUCSURPLUS contains a maximum of 10 million records per year.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsIraucsurplus6Row<'data> {
    /// Calendar Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    pub contractid: core::ops::Range<usize>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Contracted Interconnector identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// Nominated source region for Interconnector
    pub fromregionid: core::ops::Range<usize>,
    /// Total value of surplus before allocation
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// Percentage allocated to participant
    pub contractallocation: Option<rust_decimal::Decimal>,
    /// Amount NSP is paid for Inter/intra-Regional surplus energy produced
    pub surplusvalue: Option<rust_decimal::Decimal>,
    /// Date and time this record was last modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsIraucsurplus6Row<'data> {
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn fromregionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.fromregionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsIraucsurplus6 {
    const VERSION: i32 = 6;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "IRAUCSURPLUS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsIraucsurplus6Mapping([
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
        "SETTLEMENTDATE",
        "SETTLEMENTRUNNO",
        "CONTRACTID",
        "PERIODID",
        "PARTICIPANTID",
        "INTERCONNECTORID",
        "FROMREGIONID",
        "TOTALSURPLUS",
        "CONTRACTALLOCATION",
        "SURPLUSVALUE",
        "LASTCHANGED",
        "CSP_DEROGATION_AMOUNT",
        "UNADJUSTED_IRSR",
    ];
    type Row<'row> = SettlementsIraucsurplus6Row<'row>;
    type FieldMapping = SettlementsIraucsurplus6Mapping;
    type PrimaryKey = SettlementsIraucsurplus6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsIraucsurplus6Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            settlementrunno: row
                .get_custom_parsed_at_idx(
                    "settlementrunno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            contractid: row.get_range("contractid", field_mapping.0[2])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[4])?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[5])?,
            fromregionid: row.get_range("fromregionid", field_mapping.0[6])?,
            totalsurplus: row
                .get_opt_custom_parsed_at_idx(
                    "totalsurplus",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            contractallocation: row
                .get_opt_custom_parsed_at_idx(
                    "contractallocation",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            surplusvalue: row
                .get_opt_custom_parsed_at_idx(
                    "surplusvalue",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            csp_derogation_amount: row
                .get_opt_custom_parsed_at_idx(
                    "csp_derogation_amount",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            unadjusted_irsr: row
                .get_opt_custom_parsed_at_idx(
                    "unadjusted_irsr",
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
        Ok(SettlementsIraucsurplus6Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsIraucsurplus6PrimaryKey {
        SettlementsIraucsurplus6PrimaryKey {
            contractid: row.contractid().to_string(),
            fromregionid: row.fromregionid().to_string(),
            interconnectorid: row.interconnectorid().to_string(),
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
            settlementrunno: row.settlementrunno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_iraucsurplus_v6_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsIraucsurplus6Row {
            settlementdate: row.settlementdate.clone(),
            settlementrunno: row.settlementrunno.clone(),
            contractid: row.contractid.clone(),
            periodid: row.periodid.clone(),
            participantid: row.participantid.clone(),
            interconnectorid: row.interconnectorid.clone(),
            fromregionid: row.fromregionid.clone(),
            totalsurplus: row.totalsurplus.clone(),
            contractallocation: row.contractallocation.clone(),
            surplusvalue: row.surplusvalue.clone(),
            lastchanged: row.lastchanged.clone(),
            csp_derogation_amount: row.csp_derogation_amount.clone(),
            unadjusted_irsr: row.unadjusted_irsr.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsIraucsurplus6PrimaryKey {
    pub contractid: alloc::string::String,
    pub fromregionid: alloc::string::String,
    pub interconnectorid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsIraucsurplus6PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsIraucsurplus6Row<'data> {
    type Row<'other> = SettlementsIraucsurplus6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid()
            && self.fromregionid() == row.fromregionid()
            && self.interconnectorid() == row.interconnectorid()
            && self.participantid() == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsIraucsurplus6Row<'data> {
    type PrimaryKey = SettlementsIraucsurplus6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.fromregionid() == key.fromregionid
            && self.interconnectorid() == key.interconnectorid
            && self.participantid() == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsIraucsurplus6PrimaryKey {
    type Row<'other> = SettlementsIraucsurplus6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.fromregionid == row.fromregionid()
            && self.interconnectorid == row.interconnectorid()
            && self.participantid == row.participantid() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIraucsurplus6PrimaryKey {
    type PrimaryKey = SettlementsIraucsurplus6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsIraucsurplus6 {
    type Builder = SettlementsIraucsurplus6Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementrunno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "fromregionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "totalsurplus",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "contractallocation",
                    arrow::datatypes::DataType::Decimal128(8, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "surplusvalue",
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
                    "csp_derogation_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unadjusted_irsr",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsIraucsurplus6Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            settlementrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            contractid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            fromregionid_array: arrow::array::builder::StringBuilder::new(),
            totalsurplus_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            contractallocation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 5)),
            surplusvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            csp_derogation_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            unadjusted_irsr_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .settlementrunno_array
            .append_value({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
        builder.contractid_array.append_value(row.contractid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.fromregionid_array.append_value(row.fromregionid());
        builder
            .totalsurplus_array
            .append_option({
                row.totalsurplus
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .contractallocation_array
            .append_option({
                row.contractallocation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .surplusvalue_array
            .append_option({
                row.surplusvalue
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .csp_derogation_amount_array
            .append_option({
                row.csp_derogation_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .unadjusted_irsr_array
            .append_option({
                row.unadjusted_irsr
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalsurplus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractallocation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.surplusvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.csp_derogation_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unadjusted_irsr_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsIraucsurplus6Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    settlementrunno_array: arrow::array::builder::Decimal128Builder,
    contractid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    fromregionid_array: arrow::array::builder::StringBuilder,
    totalsurplus_array: arrow::array::builder::Decimal128Builder,
    contractallocation_array: arrow::array::builder::Decimal128Builder,
    surplusvalue_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    csp_derogation_amount_array: arrow::array::builder::Decimal128Builder,
    unadjusted_irsr_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsIrnspsurplus6;
pub struct SettlementsIrnspsurplus6Mapping([usize; 13]);
/// # Summary
///
/// ## SETIRNSPSURPLUS
///  _This view supports the Settlements Residue Auction, by showing the TNSP participant allocations of  Interconnector Residue (IR) Surplus (i.e. derogated amounts) arising as a result of the sold units for a quarter._
///
/// * Data Set Name: Settlements
/// * File Name: Irnspsurplus
/// * Data Version: 6
///
/// # Description
///  SETIRNSPSURPLUS data is confidential to the relevant participant. Source SETIRNSPSURPLUS updates with each settlement run. Volume SETIRNSPSURPLUS contains a maximum of 10 million records per year.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsIrnspsurplus6Row<'data> {
    /// Settlement date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    pub contractid: core::ops::Range<usize>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Participant unique identifier
    pub participantid: core::ops::Range<usize>,
    /// Identifier of Contracted Interconnector
    pub interconnectorid: core::ops::Range<usize>,
    /// Nominated source region for Interconnector
    pub fromregionid: core::ops::Range<usize>,
    /// Total value of surplus
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// Percentage of total surplus allocated to participant
    pub contractallocation: Option<rust_decimal::Decimal>,
    /// Amount NSP is paid for Inter/intra-Regional surplus energy produced by the participant
    pub surplusvalue: Option<rust_decimal::Decimal>,
    /// Date and time this record was last modified
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsIrnspsurplus6Row<'data> {
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn fromregionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.fromregionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsIrnspsurplus6 {
    const VERSION: i32 = 6;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "IRNSPSURPLUS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsIrnspsurplus6Mapping([
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
        "SETTLEMENTDATE",
        "SETTLEMENTRUNNO",
        "CONTRACTID",
        "PERIODID",
        "PARTICIPANTID",
        "INTERCONNECTORID",
        "FROMREGIONID",
        "TOTALSURPLUS",
        "CONTRACTALLOCATION",
        "SURPLUSVALUE",
        "LASTCHANGED",
        "CSP_DEROGATION_AMOUNT",
        "UNADJUSTED_IRSR",
    ];
    type Row<'row> = SettlementsIrnspsurplus6Row<'row>;
    type FieldMapping = SettlementsIrnspsurplus6Mapping;
    type PrimaryKey = SettlementsIrnspsurplus6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsIrnspsurplus6Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            settlementrunno: row
                .get_custom_parsed_at_idx(
                    "settlementrunno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            contractid: row.get_range("contractid", field_mapping.0[2])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[4])?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[5])?,
            fromregionid: row.get_range("fromregionid", field_mapping.0[6])?,
            totalsurplus: row
                .get_opt_custom_parsed_at_idx(
                    "totalsurplus",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            contractallocation: row
                .get_opt_custom_parsed_at_idx(
                    "contractallocation",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            surplusvalue: row
                .get_opt_custom_parsed_at_idx(
                    "surplusvalue",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            csp_derogation_amount: row
                .get_opt_custom_parsed_at_idx(
                    "csp_derogation_amount",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            unadjusted_irsr: row
                .get_opt_custom_parsed_at_idx(
                    "unadjusted_irsr",
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
        Ok(SettlementsIrnspsurplus6Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsIrnspsurplus6PrimaryKey {
        SettlementsIrnspsurplus6PrimaryKey {
            contractid: row.contractid().to_string(),
            fromregionid: row.fromregionid().to_string(),
            interconnectorid: row.interconnectorid().to_string(),
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
            settlementrunno: row.settlementrunno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_irnspsurplus_v6_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsIrnspsurplus6Row {
            settlementdate: row.settlementdate.clone(),
            settlementrunno: row.settlementrunno.clone(),
            contractid: row.contractid.clone(),
            periodid: row.periodid.clone(),
            participantid: row.participantid.clone(),
            interconnectorid: row.interconnectorid.clone(),
            fromregionid: row.fromregionid.clone(),
            totalsurplus: row.totalsurplus.clone(),
            contractallocation: row.contractallocation.clone(),
            surplusvalue: row.surplusvalue.clone(),
            lastchanged: row.lastchanged.clone(),
            csp_derogation_amount: row.csp_derogation_amount.clone(),
            unadjusted_irsr: row.unadjusted_irsr.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsIrnspsurplus6PrimaryKey {
    pub contractid: alloc::string::String,
    pub fromregionid: alloc::string::String,
    pub interconnectorid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsIrnspsurplus6PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsIrnspsurplus6Row<'data> {
    type Row<'other> = SettlementsIrnspsurplus6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid()
            && self.fromregionid() == row.fromregionid()
            && self.interconnectorid() == row.interconnectorid()
            && self.participantid() == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsIrnspsurplus6Row<'data> {
    type PrimaryKey = SettlementsIrnspsurplus6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.fromregionid() == key.fromregionid
            && self.interconnectorid() == key.interconnectorid
            && self.participantid() == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsIrnspsurplus6PrimaryKey {
    type Row<'other> = SettlementsIrnspsurplus6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.fromregionid == row.fromregionid()
            && self.interconnectorid == row.interconnectorid()
            && self.participantid == row.participantid() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIrnspsurplus6PrimaryKey {
    type PrimaryKey = SettlementsIrnspsurplus6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsIrnspsurplus6 {
    type Builder = SettlementsIrnspsurplus6Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementrunno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "fromregionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "totalsurplus",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "contractallocation",
                    arrow::datatypes::DataType::Decimal128(8, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "surplusvalue",
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
                    "csp_derogation_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unadjusted_irsr",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsIrnspsurplus6Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            settlementrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            contractid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            fromregionid_array: arrow::array::builder::StringBuilder::new(),
            totalsurplus_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            contractallocation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 5)),
            surplusvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            csp_derogation_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            unadjusted_irsr_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .settlementrunno_array
            .append_value({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
        builder.contractid_array.append_value(row.contractid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.fromregionid_array.append_value(row.fromregionid());
        builder
            .totalsurplus_array
            .append_option({
                row.totalsurplus
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .contractallocation_array
            .append_option({
                row.contractallocation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .surplusvalue_array
            .append_option({
                row.surplusvalue
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .csp_derogation_amount_array
            .append_option({
                row.csp_derogation_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .unadjusted_irsr_array
            .append_option({
                row.unadjusted_irsr
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalsurplus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractallocation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.surplusvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.csp_derogation_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unadjusted_irsr_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsIrnspsurplus6Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    settlementrunno_array: arrow::array::builder::Decimal128Builder,
    contractid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    fromregionid_array: arrow::array::builder::StringBuilder,
    totalsurplus_array: arrow::array::builder::Decimal128Builder,
    contractallocation_array: arrow::array::builder::Decimal128Builder,
    surplusvalue_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    csp_derogation_amount_array: arrow::array::builder::Decimal128Builder,
    unadjusted_irsr_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsIrpartsurplus6;
pub struct SettlementsIrpartsurplus6Mapping([usize; 13]);
/// # Summary
///
/// ## SETIRPARTSURPLUS
///  _This view supports the Settlements Residue Auction, holding the participant allocations of IRSurplus._
///
/// * Data Set Name: Settlements
/// * File Name: Irpartsurplus
/// * Data Version: 6
///
/// # Description
///  SETIRPARTSURPLUS data is confidential to each participant. Source SETIRPARTSURPLUS updates with each settlement run. Volume SETIRPARTSURPLUS contains a maximum of 20 million records per year.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsIrpartsurplus6Row<'data> {
    /// Settlement date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// Ancillary Service Contract
    pub contractid: core::ops::Range<usize>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Participant unique identifier
    pub participantid: core::ops::Range<usize>,
    /// Identifier of the Contracted Interconnector
    pub interconnectorid: core::ops::Range<usize>,
    /// Nominated source region for Interconnector
    pub fromregionid: core::ops::Range<usize>,
    /// Total value of surplus before allocation
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// Allocated percentage to participant
    pub contractallocation: Option<rust_decimal::Decimal>,
    /// Amount NSP is paid for Inter/intra-Regional surplus energy produced
    pub surplusvalue: Option<rust_decimal::Decimal>,
    /// Date and time this record was last updated
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsIrpartsurplus6Row<'data> {
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn fromregionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.fromregionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsIrpartsurplus6 {
    const VERSION: i32 = 6;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "IRPARTSURPLUS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsIrpartsurplus6Mapping([
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
        "SETTLEMENTDATE",
        "SETTLEMENTRUNNO",
        "CONTRACTID",
        "PERIODID",
        "PARTICIPANTID",
        "INTERCONNECTORID",
        "FROMREGIONID",
        "TOTALSURPLUS",
        "CONTRACTALLOCATION",
        "SURPLUSVALUE",
        "LASTCHANGED",
        "CSP_DEROGATION_AMOUNT",
        "UNADJUSTED_IRSR",
    ];
    type Row<'row> = SettlementsIrpartsurplus6Row<'row>;
    type FieldMapping = SettlementsIrpartsurplus6Mapping;
    type PrimaryKey = SettlementsIrpartsurplus6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsIrpartsurplus6Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            settlementrunno: row
                .get_custom_parsed_at_idx(
                    "settlementrunno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            contractid: row.get_range("contractid", field_mapping.0[2])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[4])?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[5])?,
            fromregionid: row.get_range("fromregionid", field_mapping.0[6])?,
            totalsurplus: row
                .get_opt_custom_parsed_at_idx(
                    "totalsurplus",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            contractallocation: row
                .get_opt_custom_parsed_at_idx(
                    "contractallocation",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            surplusvalue: row
                .get_opt_custom_parsed_at_idx(
                    "surplusvalue",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            csp_derogation_amount: row
                .get_opt_custom_parsed_at_idx(
                    "csp_derogation_amount",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            unadjusted_irsr: row
                .get_opt_custom_parsed_at_idx(
                    "unadjusted_irsr",
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
        Ok(SettlementsIrpartsurplus6Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsIrpartsurplus6PrimaryKey {
        SettlementsIrpartsurplus6PrimaryKey {
            contractid: row.contractid().to_string(),
            fromregionid: row.fromregionid().to_string(),
            interconnectorid: row.interconnectorid().to_string(),
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
            settlementrunno: row.settlementrunno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_irpartsurplus_v6_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsIrpartsurplus6Row {
            settlementdate: row.settlementdate.clone(),
            settlementrunno: row.settlementrunno.clone(),
            contractid: row.contractid.clone(),
            periodid: row.periodid.clone(),
            participantid: row.participantid.clone(),
            interconnectorid: row.interconnectorid.clone(),
            fromregionid: row.fromregionid.clone(),
            totalsurplus: row.totalsurplus.clone(),
            contractallocation: row.contractallocation.clone(),
            surplusvalue: row.surplusvalue.clone(),
            lastchanged: row.lastchanged.clone(),
            csp_derogation_amount: row.csp_derogation_amount.clone(),
            unadjusted_irsr: row.unadjusted_irsr.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsIrpartsurplus6PrimaryKey {
    pub contractid: alloc::string::String,
    pub fromregionid: alloc::string::String,
    pub interconnectorid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsIrpartsurplus6PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsIrpartsurplus6Row<'data> {
    type Row<'other> = SettlementsIrpartsurplus6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid()
            && self.fromregionid() == row.fromregionid()
            && self.interconnectorid() == row.interconnectorid()
            && self.participantid() == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsIrpartsurplus6Row<'data> {
    type PrimaryKey = SettlementsIrpartsurplus6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.fromregionid() == key.fromregionid
            && self.interconnectorid() == key.interconnectorid
            && self.participantid() == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsIrpartsurplus6PrimaryKey {
    type Row<'other> = SettlementsIrpartsurplus6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.fromregionid == row.fromregionid()
            && self.interconnectorid == row.interconnectorid()
            && self.participantid == row.participantid() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIrpartsurplus6PrimaryKey {
    type PrimaryKey = SettlementsIrpartsurplus6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsIrpartsurplus6 {
    type Builder = SettlementsIrpartsurplus6Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementrunno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "fromregionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "totalsurplus",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "contractallocation",
                    arrow::datatypes::DataType::Decimal128(8, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "surplusvalue",
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
                    "csp_derogation_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unadjusted_irsr",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsIrpartsurplus6Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            settlementrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            contractid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            fromregionid_array: arrow::array::builder::StringBuilder::new(),
            totalsurplus_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            contractallocation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(8, 5)),
            surplusvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            csp_derogation_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            unadjusted_irsr_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .settlementrunno_array
            .append_value({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
        builder.contractid_array.append_value(row.contractid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.fromregionid_array.append_value(row.fromregionid());
        builder
            .totalsurplus_array
            .append_option({
                row.totalsurplus
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .contractallocation_array
            .append_option({
                row.contractallocation
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .surplusvalue_array
            .append_option({
                row.surplusvalue
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .csp_derogation_amount_array
            .append_option({
                row.csp_derogation_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .unadjusted_irsr_array
            .append_option({
                row.unadjusted_irsr
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.fromregionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.totalsurplus_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractallocation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.surplusvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.csp_derogation_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unadjusted_irsr_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsIrpartsurplus6Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    settlementrunno_array: arrow::array::builder::Decimal128Builder,
    contractid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    fromregionid_array: arrow::array::builder::StringBuilder,
    totalsurplus_array: arrow::array::builder::Decimal128Builder,
    contractallocation_array: arrow::array::builder::Decimal128Builder,
    surplusvalue_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    csp_derogation_amount_array: arrow::array::builder::Decimal128Builder,
    unadjusted_irsr_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsIrsurplus6;
pub struct SettlementsIrsurplus6Mapping([usize; 11]);
/// # Summary
///
/// ## SETIRSURPLUS
///  _SETIRSURPLUS records the interregional residue calculation for each interconnector and each side of the interconnector._
///
/// * Data Set Name: Settlements
/// * File Name: Irsurplus
/// * Data Version: 6
///
/// # Description
///  SETIRSURPLUS data is public, so is available to all participants. Source SETIRSURPLUS updates once a day at 8am. Note MWFLOW and LOSSFACTOR are now both calculated as MWh (energy) values for the half hour, and not MW (average demand) values. By way of clarification, the MWFLOW value is derived from half-hour revenue class metering, adjusted by a fixed fraction of the LOSSFACTOR value. The LOSSFACTOR value is taken to be exactly half of the MWLOSSES value in the TRADINGINTERCONNECT table. The METEREDMWFLOW field in the TRADINGINTERCONNECT table contains averaged SCADA metering demand values available in “real time”, whereas the MWFLOW field in the SETIRSURPLUS table contains settlement energy metering values available only after a settlement run is posted.
///
///
///
/// # Primary Key Columns
///
/// * INTERCONNECTORID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsIrsurplus6Row<'data> {
    /// Settlement date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Interconnector
    pub interconnectorid: core::ops::Range<usize>,
    /// Side of interconnector
    pub regionid: core::ops::Range<usize>,
    /// Net flow at the regional node (MWh), including losses
    pub mwflow: Option<rust_decimal::Decimal>,
    /// MW losses along interconnector NOTE: This is not a loss factor, but a loss figure expressed in MWH
    pub lossfactor: Option<rust_decimal::Decimal>,
    /// Amount of surplus in $
    pub surplusvalue: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsIrsurplus6Row<'data> {
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
impl mmsdm_core::GetTable for SettlementsIrsurplus6 {
    const VERSION: i32 = 6;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "IRSURPLUS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsIrsurplus6Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "SETTLEMENTRUNNO",
        "PERIODID",
        "INTERCONNECTORID",
        "REGIONID",
        "MWFLOW",
        "LOSSFACTOR",
        "SURPLUSVALUE",
        "LASTCHANGED",
        "CSP_DEROGATION_AMOUNT",
        "UNADJUSTED_IRSR",
    ];
    type Row<'row> = SettlementsIrsurplus6Row<'row>;
    type FieldMapping = SettlementsIrsurplus6Mapping;
    type PrimaryKey = SettlementsIrsurplus6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsIrsurplus6Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            settlementrunno: row
                .get_custom_parsed_at_idx(
                    "settlementrunno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[3])?,
            regionid: row.get_range("regionid", field_mapping.0[4])?,
            mwflow: row
                .get_opt_custom_parsed_at_idx(
                    "mwflow",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lossfactor: row
                .get_opt_custom_parsed_at_idx(
                    "lossfactor",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            surplusvalue: row
                .get_opt_custom_parsed_at_idx(
                    "surplusvalue",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
                )?,
            csp_derogation_amount: row
                .get_opt_custom_parsed_at_idx(
                    "csp_derogation_amount",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            unadjusted_irsr: row
                .get_opt_custom_parsed_at_idx(
                    "unadjusted_irsr",
                    field_mapping.0[10],
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
        Ok(SettlementsIrsurplus6Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsIrsurplus6PrimaryKey {
        SettlementsIrsurplus6PrimaryKey {
            interconnectorid: row.interconnectorid().to_string(),
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            settlementdate: row.settlementdate,
            settlementrunno: row.settlementrunno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_irsurplus_v6_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsIrsurplus6Row {
            settlementdate: row.settlementdate.clone(),
            settlementrunno: row.settlementrunno.clone(),
            periodid: row.periodid.clone(),
            interconnectorid: row.interconnectorid.clone(),
            regionid: row.regionid.clone(),
            mwflow: row.mwflow.clone(),
            lossfactor: row.lossfactor.clone(),
            surplusvalue: row.surplusvalue.clone(),
            lastchanged: row.lastchanged.clone(),
            csp_derogation_amount: row.csp_derogation_amount.clone(),
            unadjusted_irsr: row.unadjusted_irsr.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsIrsurplus6PrimaryKey {
    pub interconnectorid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsIrsurplus6PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsIrsurplus6Row<'data> {
    type Row<'other> = SettlementsIrsurplus6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interconnectorid() == row.interconnectorid()
            && self.periodid == row.periodid && self.regionid() == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsIrsurplus6Row<'data> {
    type PrimaryKey = SettlementsIrsurplus6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid() == key.interconnectorid && self.periodid == key.periodid
            && self.regionid() == key.regionid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsIrsurplus6PrimaryKey {
    type Row<'other> = SettlementsIrsurplus6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.interconnectorid == row.interconnectorid() && self.periodid == row.periodid
            && self.regionid == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIrsurplus6PrimaryKey {
    type PrimaryKey = SettlementsIrsurplus6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid && self.periodid == key.periodid
            && self.regionid == key.regionid && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsIrsurplus6 {
    type Builder = SettlementsIrsurplus6Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementrunno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
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
                    "mwflow",
                    arrow::datatypes::DataType::Decimal128(15, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lossfactor",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "surplusvalue",
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
                    "csp_derogation_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unadjusted_irsr",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsIrsurplus6Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            settlementrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            mwflow_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 6)),
            lossfactor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            surplusvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            csp_derogation_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            unadjusted_irsr_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .settlementrunno_array
            .append_value({
                let mut val = row.settlementrunno;
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
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.regionid_array.append_value(row.regionid());
        builder
            .mwflow_array
            .append_option({
                row.mwflow
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lossfactor_array
            .append_option({
                row.lossfactor
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .surplusvalue_array
            .append_option({
                row.surplusvalue
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .csp_derogation_amount_array
            .append_option({
                row.csp_derogation_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .unadjusted_irsr_array
            .append_option({
                row.unadjusted_irsr
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mwflow_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lossfactor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.surplusvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.csp_derogation_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unadjusted_irsr_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsIrsurplus6Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    settlementrunno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    interconnectorid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    mwflow_array: arrow::array::builder::Decimal128Builder,
    lossfactor_array: arrow::array::builder::Decimal128Builder,
    surplusvalue_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    csp_derogation_amount_array: arrow::array::builder::Decimal128Builder,
    unadjusted_irsr_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsLocalareaenergy1;
pub struct SettlementsLocalareaenergy1Mapping([usize; 10]);
/// # Summary
///
/// ## SETLOCALAREAENERGY
///  _SETLOCALAREAENERGY shows the UFE, AGE and associated values for each local area and trading interval in a settlement run._
///
/// * Data Set Name: Settlements
/// * File Name: Localareaenergy
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * LOCALAREAID
/// * PERIODID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsLocalareaenergy1Row<'data> {
    /// Settlement date of the settlement run
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number of the settlement run
    pub settlementrunno: rust_decimal::Decimal,
    /// Unique identifier for the local area
    pub localareaid: core::ops::Range<usize>,
    /// Settlement Trading Interval
    pub periodid: rust_decimal::Decimal,
    /// Total unaccounted-for energy for the local area in this trading interval, in MWh
    pub ufe: Option<rust_decimal::Decimal>,
    /// DDME component of UFE for the local area in this trading interval, in MWh.
    pub ddme: Option<rust_decimal::Decimal>,
    /// TME component of UFE for the local area in this trading interval, in MWh.
    pub tme: Option<rust_decimal::Decimal>,
    /// ADME component of UFE for the local area in this trading interval, in MWh.
    pub adme: Option<rust_decimal::Decimal>,
    /// The sum of all DME amounts for each Market Customer FRMP and TNI in the local area, in this trading interval.
    pub admela: Option<rust_decimal::Decimal>,
    /// Last changed date time for the record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsLocalareaenergy1Row<'data> {
    pub fn localareaid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.localareaid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsLocalareaenergy1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "LOCALAREAENERGY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsLocalareaenergy1Mapping([
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
        "SETTLEMENTDATE",
        "SETTLEMENTRUNNO",
        "LOCALAREAID",
        "PERIODID",
        "UFE",
        "DDME",
        "TME",
        "ADME",
        "ADMELA",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementsLocalareaenergy1Row<'row>;
    type FieldMapping = SettlementsLocalareaenergy1Mapping;
    type PrimaryKey = SettlementsLocalareaenergy1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsLocalareaenergy1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            settlementrunno: row
                .get_custom_parsed_at_idx(
                    "settlementrunno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            localareaid: row.get_range("localareaid", field_mapping.0[2])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ufe: row
                .get_opt_custom_parsed_at_idx(
                    "ufe",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ddme: row
                .get_opt_custom_parsed_at_idx(
                    "ddme",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            tme: row
                .get_opt_custom_parsed_at_idx(
                    "tme",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            adme: row
                .get_opt_custom_parsed_at_idx(
                    "adme",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            admela: row
                .get_opt_custom_parsed_at_idx(
                    "admela",
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
        Ok(SettlementsLocalareaenergy1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsLocalareaenergy1PrimaryKey {
        SettlementsLocalareaenergy1PrimaryKey {
            localareaid: row.localareaid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
            settlementrunno: row.settlementrunno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_localareaenergy_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsLocalareaenergy1Row {
            settlementdate: row.settlementdate.clone(),
            settlementrunno: row.settlementrunno.clone(),
            localareaid: row.localareaid.clone(),
            periodid: row.periodid.clone(),
            ufe: row.ufe.clone(),
            ddme: row.ddme.clone(),
            tme: row.tme.clone(),
            adme: row.adme.clone(),
            admela: row.admela.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsLocalareaenergy1PrimaryKey {
    pub localareaid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsLocalareaenergy1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsLocalareaenergy1Row<'data> {
    type Row<'other> = SettlementsLocalareaenergy1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.localareaid() == row.localareaid() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsLocalareaenergy1Row<'data> {
    type PrimaryKey = SettlementsLocalareaenergy1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.localareaid() == key.localareaid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsLocalareaenergy1PrimaryKey {
    type Row<'other> = SettlementsLocalareaenergy1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.localareaid == row.localareaid() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsLocalareaenergy1PrimaryKey {
    type PrimaryKey = SettlementsLocalareaenergy1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.localareaid == key.localareaid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsLocalareaenergy1 {
    type Builder = SettlementsLocalareaenergy1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementrunno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "localareaid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "ufe",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ddme",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "tme",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "adme",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "admela",
                    arrow::datatypes::DataType::Decimal128(18, 8),
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
        SettlementsLocalareaenergy1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            settlementrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            localareaid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            ufe_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            ddme_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            tme_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            adme_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            admela_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .settlementrunno_array
            .append_value({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
        builder.localareaid_array.append_value(row.localareaid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .ufe_array
            .append_option({
                row.ufe
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .ddme_array
            .append_option({
                row.ddme
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .tme_array
            .append_option({
                row.tme
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .adme_array
            .append_option({
                row.adme
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .admela_array
            .append_option({
                row.admela
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.localareaid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ufe_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ddme_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tme_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.adme_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.admela_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsLocalareaenergy1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    settlementrunno_array: arrow::array::builder::Decimal128Builder,
    localareaid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    ufe_array: arrow::array::builder::Decimal128Builder,
    ddme_array: arrow::array::builder::Decimal128Builder,
    tme_array: arrow::array::builder::Decimal128Builder,
    adme_array: arrow::array::builder::Decimal128Builder,
    admela_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementsLocalareatni1;
pub struct SettlementsLocalareatni1Mapping([usize; 5]);
/// # Summary
///
/// ## SETLOCALAREATNI
///  _SETLOCALAREATNI shows the list of TNIs constituent to a local area in a settlement run. _
///
/// * Data Set Name: Settlements
/// * File Name: Localareatni
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * LOCALAREAID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
/// * TNI
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsLocalareatni1Row<'data> {
    /// Settlement date of the settlement run
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number of the settlement run
    pub settlementrunno: rust_decimal::Decimal,
    /// Unique identifier for the local area
    pub localareaid: core::ops::Range<usize>,
    /// Unique identifier for a TNI constituent to the local area as at the settlement run
    pub tni: core::ops::Range<usize>,
    /// Last changed date time for the record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsLocalareatni1Row<'data> {
    pub fn localareaid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.localareaid.clone())
    }
    pub fn tni(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.tni.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsLocalareatni1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "LOCALAREATNI";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsLocalareatni1Mapping([
        4,
        5,
        6,
        7,
        8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "SETTLEMENTRUNNO",
        "LOCALAREAID",
        "TNI",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementsLocalareatni1Row<'row>;
    type FieldMapping = SettlementsLocalareatni1Mapping;
    type PrimaryKey = SettlementsLocalareatni1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsLocalareatni1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            settlementrunno: row
                .get_custom_parsed_at_idx(
                    "settlementrunno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            localareaid: row.get_range("localareaid", field_mapping.0[2])?,
            tni: row.get_range("tni", field_mapping.0[3])?,
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
        Ok(SettlementsLocalareatni1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsLocalareatni1PrimaryKey {
        SettlementsLocalareatni1PrimaryKey {
            localareaid: row.localareaid().to_string(),
            settlementdate: row.settlementdate,
            settlementrunno: row.settlementrunno,
            tni: row.tni().to_string(),
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_localareatni_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsLocalareatni1Row {
            settlementdate: row.settlementdate.clone(),
            settlementrunno: row.settlementrunno.clone(),
            localareaid: row.localareaid.clone(),
            tni: row.tni.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsLocalareatni1PrimaryKey {
    pub localareaid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
    pub tni: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for SettlementsLocalareatni1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsLocalareatni1Row<'data> {
    type Row<'other> = SettlementsLocalareatni1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.localareaid() == row.localareaid()
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno && self.tni() == row.tni()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsLocalareatni1Row<'data> {
    type PrimaryKey = SettlementsLocalareatni1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.localareaid() == key.localareaid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno && self.tni() == key.tni
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsLocalareatni1PrimaryKey {
    type Row<'other> = SettlementsLocalareatni1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.localareaid == row.localareaid()
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno && self.tni == row.tni()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsLocalareatni1PrimaryKey {
    type PrimaryKey = SettlementsLocalareatni1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.localareaid == key.localareaid && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno && self.tni == key.tni
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsLocalareatni1 {
    type Builder = SettlementsLocalareatni1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementrunno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "localareaid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "tni",
                    arrow::datatypes::DataType::Utf8,
                    false,
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
        SettlementsLocalareatni1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            settlementrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            localareaid_array: arrow::array::builder::StringBuilder::new(),
            tni_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .settlementrunno_array
            .append_value({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
        builder.localareaid_array.append_value(row.localareaid());
        builder.tni_array.append_value(row.tni());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.localareaid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tni_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsLocalareatni1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    settlementrunno_array: arrow::array::builder::Decimal128Builder,
    localareaid_array: arrow::array::builder::StringBuilder,
    tni_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementsLshedpayment5;
pub struct SettlementsLshedpayment5Mapping([usize; 24]);
/// # Summary
///
/// ## SETLSHEDPAYMENT
///  _SETLSHEDPAYMENT shows specific payment details for load shed services by period._
///
/// * Data Set Name: Settlements
/// * File Name: Lshedpayment
/// * Data Version: 5
///
/// # Description
///  SETLSHEDPAYMENT data is confidential to the relevant participant. Source SETLSHEDPAYMENT updates with each settlement run.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsLshedpayment5Row<'data> {
    /// Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: core::ops::Range<usize>,
    /// AS Contract Identifier
    pub contractid: core::ops::Range<usize>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Dispatchable Unit Identifier
    pub duid: core::ops::Range<usize>,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Load Shed Enabling Price
    pub lseprice: Option<rust_decimal::Decimal>,
    /// Minimum Compensation Price
    pub mcpprice: Option<rust_decimal::Decimal>,
    /// Load Shed Control Range
    pub lscr: Option<rust_decimal::Decimal>,
    /// Load Shed Enabling Payment
    pub lsepayment: Option<rust_decimal::Decimal>,
    /// Compensation Payment
    pub ccpayment: Option<rust_decimal::Decimal>,
    /// Cleared MW of unit at time of load shed usage
    pub constrainedmw: Option<rust_decimal::Decimal>,
    /// Unconstrained MW of unit at time of load shed usage
    pub unconstrainedmw: Option<rust_decimal::Decimal>,
    /// Amount of load shed
    pub als: Option<rust_decimal::Decimal>,
    /// Initial demand of unit at time of load shed usage
    pub initialdemand: Option<rust_decimal::Decimal>,
    /// Final demand of unit at time of load shed usage
    pub finaldemand: Option<rust_decimal::Decimal>,
    /// AS Contract Version No.
    pub contractversionno: Option<rust_decimal::Decimal>,
    /// Re-offer offer date
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// Re-Offer Version No.
    pub offerversionno: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Payment amount for the Load Shed Availability service
    pub availabilitypayment: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsLshedpayment5Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
    pub fn duid(&self) -> Option<&str> {
        if self.duid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone()),
            )
        }
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
impl mmsdm_core::GetTable for SettlementsLshedpayment5 {
    const VERSION: i32 = 5;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "LSHEDPAYMENT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsLshedpayment5Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PARTICIPANTID",
        "CONTRACTID",
        "PERIODID",
        "DUID",
        "REGIONID",
        "TLF",
        "RRP",
        "LSEPRICE",
        "MCPPRICE",
        "LSCR",
        "LSEPAYMENT",
        "CCPAYMENT",
        "CONSTRAINEDMW",
        "UNCONSTRAINEDMW",
        "ALS",
        "INITIALDEMAND",
        "FINALDEMAND",
        "CONTRACTVERSIONNO",
        "OFFERDATE",
        "OFFERVERSIONNO",
        "LASTCHANGED",
        "AVAILABILITYPAYMENT",
    ];
    type Row<'row> = SettlementsLshedpayment5Row<'row>;
    type FieldMapping = SettlementsLshedpayment5Mapping;
    type PrimaryKey = SettlementsLshedpayment5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsLshedpayment5Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
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
            contractid: row.get_range("contractid", field_mapping.0[3])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            duid: row.get_opt_range("duid", field_mapping.0[5])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[6])?,
            tlf: row
                .get_opt_custom_parsed_at_idx(
                    "tlf",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp: row
                .get_opt_custom_parsed_at_idx(
                    "rrp",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lseprice: row
                .get_opt_custom_parsed_at_idx(
                    "lseprice",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mcpprice: row
                .get_opt_custom_parsed_at_idx(
                    "mcpprice",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lscr: row
                .get_opt_custom_parsed_at_idx(
                    "lscr",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lsepayment: row
                .get_opt_custom_parsed_at_idx(
                    "lsepayment",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ccpayment: row
                .get_opt_custom_parsed_at_idx(
                    "ccpayment",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            constrainedmw: row
                .get_opt_custom_parsed_at_idx(
                    "constrainedmw",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            unconstrainedmw: row
                .get_opt_custom_parsed_at_idx(
                    "unconstrainedmw",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            als: row
                .get_opt_custom_parsed_at_idx(
                    "als",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            initialdemand: row
                .get_opt_custom_parsed_at_idx(
                    "initialdemand",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            finaldemand: row
                .get_opt_custom_parsed_at_idx(
                    "finaldemand",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            contractversionno: row
                .get_opt_custom_parsed_at_idx(
                    "contractversionno",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            offerdate: row
                .get_opt_custom_parsed_at_idx(
                    "offerdate",
                    field_mapping.0[20],
                    mmsdm_core::mms_datetime::parse,
                )?,
            offerversionno: row
                .get_opt_custom_parsed_at_idx(
                    "offerversionno",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[22],
                    mmsdm_core::mms_datetime::parse,
                )?,
            availabilitypayment: row
                .get_opt_custom_parsed_at_idx(
                    "availabilitypayment",
                    field_mapping.0[23],
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
        Ok(SettlementsLshedpayment5Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsLshedpayment5PrimaryKey {
        SettlementsLshedpayment5PrimaryKey {
            contractid: row.contractid().to_string(),
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_lshedpayment_v5_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsLshedpayment5Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            participantid: row.participantid.clone(),
            contractid: row.contractid.clone(),
            periodid: row.periodid.clone(),
            duid: row.duid.clone(),
            regionid: row.regionid.clone(),
            tlf: row.tlf.clone(),
            rrp: row.rrp.clone(),
            lseprice: row.lseprice.clone(),
            mcpprice: row.mcpprice.clone(),
            lscr: row.lscr.clone(),
            lsepayment: row.lsepayment.clone(),
            ccpayment: row.ccpayment.clone(),
            constrainedmw: row.constrainedmw.clone(),
            unconstrainedmw: row.unconstrainedmw.clone(),
            als: row.als.clone(),
            initialdemand: row.initialdemand.clone(),
            finaldemand: row.finaldemand.clone(),
            contractversionno: row.contractversionno.clone(),
            offerdate: row.offerdate.clone(),
            offerversionno: row.offerversionno.clone(),
            lastchanged: row.lastchanged.clone(),
            availabilitypayment: row.availabilitypayment.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsLshedpayment5PrimaryKey {
    pub contractid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsLshedpayment5PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsLshedpayment5Row<'data> {
    type Row<'other> = SettlementsLshedpayment5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid()
            && self.participantid() == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsLshedpayment5Row<'data> {
    type PrimaryKey = SettlementsLshedpayment5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.participantid() == key.participantid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsLshedpayment5PrimaryKey {
    type Row<'other> = SettlementsLshedpayment5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.participantid == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsLshedpayment5PrimaryKey {
    type PrimaryKey = SettlementsLshedpayment5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.participantid == key.participantid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsLshedpayment5 {
    type Builder = SettlementsLshedpayment5Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
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
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "tlf",
                    arrow::datatypes::DataType::Decimal128(7, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lseprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mcpprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lscr",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lsepayment",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ccpayment",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "constrainedmw",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unconstrainedmw",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "als",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "initialdemand",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "finaldemand",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "contractversionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offerdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offerversionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
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
                    "availabilitypayment",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsLshedpayment5Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            contractid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            duid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            tlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(7, 5)),
            rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lseprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mcpprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lscr_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            lsepayment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            ccpayment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            constrainedmw_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            unconstrainedmw_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            als_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            initialdemand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            finaldemand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            contractversionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offerversionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            availabilitypayment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.contractid_array.append_value(row.contractid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder.duid_array.append_option(row.duid());
        builder.regionid_array.append_option(row.regionid());
        builder
            .tlf_array
            .append_option({
                row.tlf
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp_array
            .append_option({
                row.rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lseprice_array
            .append_option({
                row.lseprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mcpprice_array
            .append_option({
                row.mcpprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lscr_array
            .append_option({
                row.lscr
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lsepayment_array
            .append_option({
                row.lsepayment
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .ccpayment_array
            .append_option({
                row.ccpayment
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .constrainedmw_array
            .append_option({
                row.constrainedmw
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .unconstrainedmw_array
            .append_option({
                row.unconstrainedmw
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .als_array
            .append_option({
                row.als
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .initialdemand_array
            .append_option({
                row.initialdemand
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .finaldemand_array
            .append_option({
                row.finaldemand
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .contractversionno_array
            .append_option({
                row.contractversionno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .offerdate_array
            .append_option(row.offerdate.map(|val| val.timestamp_millis()));
        builder
            .offerversionno_array
            .append_option({
                row.offerversionno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .availabilitypayment_array
            .append_option({
                row.availabilitypayment
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lseprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mcpprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lscr_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lsepayment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ccpayment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constrainedmw_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unconstrainedmw_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.als_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.initialdemand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.finaldemand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractversionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerversionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.availabilitypayment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsLshedpayment5Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    contractid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    duid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    tlf_array: arrow::array::builder::Decimal128Builder,
    rrp_array: arrow::array::builder::Decimal128Builder,
    lseprice_array: arrow::array::builder::Decimal128Builder,
    mcpprice_array: arrow::array::builder::Decimal128Builder,
    lscr_array: arrow::array::builder::Decimal128Builder,
    lsepayment_array: arrow::array::builder::Decimal128Builder,
    ccpayment_array: arrow::array::builder::Decimal128Builder,
    constrainedmw_array: arrow::array::builder::Decimal128Builder,
    unconstrainedmw_array: arrow::array::builder::Decimal128Builder,
    als_array: arrow::array::builder::Decimal128Builder,
    initialdemand_array: arrow::array::builder::Decimal128Builder,
    finaldemand_array: arrow::array::builder::Decimal128Builder,
    contractversionno_array: arrow::array::builder::Decimal128Builder,
    offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offerversionno_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    availabilitypayment_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsMarketfees6;
pub struct SettlementsMarketfees6Mapping([usize; 11]);
/// # Summary
///
/// ## SETMARKETFEES
///  _SETMARKETFEES shows payments for market fees for each settlement date._
///
/// * Data Set Name: Settlements
/// * File Name: Marketfees
/// * Data Version: 6
///
/// # Description
///  SETMARKETFEES is confidential data. Source SETMARKETFEES updates with each settlement run.
///
///
///
/// # Primary Key Columns
///
/// * MARKETFEEID
/// * PARTICIPANTCATEGORYID
/// * PARTICIPANTID
/// * PERIODID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsMarketfees6Row<'data> {
    /// Settlement date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub runno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Market fee identifier (e.g. V_EST)
    pub marketfeeid: core::ops::Range<usize>,
    /// Fee charge
    pub marketfeevalue: Option<rust_decimal::Decimal>,
    /// Energy amount for variable fees
    pub energy: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The participant category that the market fee recovery pertains to. Corresponds to the PARTICIPANTCATEGORYID column of the PARTICIPANT_BANDFEE_CATEGORYALLOC_C_V view for BAND$ type fees, or to the MARKETFEETYPE column of the MARKETFEE_P_V view for all other fee types.
    pub participantcategoryid: core::ops::Range<usize>,
    /// The rate applied to this fee for the participant at the settlement date
    pub feerate: Option<rust_decimal::Decimal>,
    /// The number of units applicable to this fee for the participant, in the trading interval.
    pub feeunits: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsMarketfees6Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn marketfeeid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.marketfeeid.clone())
    }
    pub fn participantcategoryid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.participantcategoryid.clone(),
        )
    }
}
impl mmsdm_core::GetTable for SettlementsMarketfees6 {
    const VERSION: i32 = 6;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "MARKETFEES";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsMarketfees6Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "RUNNO",
        "PARTICIPANTID",
        "PERIODID",
        "MARKETFEEID",
        "MARKETFEEVALUE",
        "ENERGY",
        "LASTCHANGED",
        "PARTICIPANTCATEGORYID",
        "FEERATE",
        "FEEUNITS",
    ];
    type Row<'row> = SettlementsMarketfees6Row<'row>;
    type FieldMapping = SettlementsMarketfees6Mapping;
    type PrimaryKey = SettlementsMarketfees6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsMarketfees6Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row
                .get_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[2])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            marketfeeid: row.get_range("marketfeeid", field_mapping.0[4])?,
            marketfeevalue: row
                .get_opt_custom_parsed_at_idx(
                    "marketfeevalue",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energy: row
                .get_opt_custom_parsed_at_idx(
                    "energy",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            participantcategoryid: row
                .get_range("participantcategoryid", field_mapping.0[8])?,
            feerate: row
                .get_opt_custom_parsed_at_idx(
                    "feerate",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            feeunits: row
                .get_opt_custom_parsed_at_idx(
                    "feeunits",
                    field_mapping.0[10],
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
        Ok(SettlementsMarketfees6Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsMarketfees6PrimaryKey {
        SettlementsMarketfees6PrimaryKey {
            marketfeeid: row.marketfeeid().to_string(),
            participantcategoryid: row.participantcategoryid().to_string(),
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            runno: row.runno,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_marketfees_v6_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsMarketfees6Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            participantid: row.participantid.clone(),
            periodid: row.periodid.clone(),
            marketfeeid: row.marketfeeid.clone(),
            marketfeevalue: row.marketfeevalue.clone(),
            energy: row.energy.clone(),
            lastchanged: row.lastchanged.clone(),
            participantcategoryid: row.participantcategoryid.clone(),
            feerate: row.feerate.clone(),
            feeunits: row.feeunits.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsMarketfees6PrimaryKey {
    pub marketfeeid: alloc::string::String,
    pub participantcategoryid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for SettlementsMarketfees6PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsMarketfees6Row<'data> {
    type Row<'other> = SettlementsMarketfees6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.marketfeeid() == row.marketfeeid()
            && self.participantcategoryid() == row.participantcategoryid()
            && self.participantid() == row.participantid()
            && self.periodid == row.periodid && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsMarketfees6Row<'data> {
    type PrimaryKey = SettlementsMarketfees6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.marketfeeid() == key.marketfeeid
            && self.participantcategoryid() == key.participantcategoryid
            && self.participantid() == key.participantid && self.periodid == key.periodid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsMarketfees6PrimaryKey {
    type Row<'other> = SettlementsMarketfees6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.marketfeeid == row.marketfeeid()
            && self.participantcategoryid == row.participantcategoryid()
            && self.participantid == row.participantid() && self.periodid == row.periodid
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsMarketfees6PrimaryKey {
    type PrimaryKey = SettlementsMarketfees6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.marketfeeid == key.marketfeeid
            && self.participantcategoryid == key.participantcategoryid
            && self.participantid == key.participantid && self.periodid == key.periodid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsMarketfees6 {
    type Builder = SettlementsMarketfees6Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "marketfeeid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "marketfeevalue",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energy",
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
                arrow::datatypes::Field::new(
                    "participantcategoryid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "feerate",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "feeunits",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsMarketfees6Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            marketfeeid_array: arrow::array::builder::StringBuilder::new(),
            marketfeevalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            energy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            participantcategoryid_array: arrow::array::builder::StringBuilder::new(),
            feerate_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            feeunits_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .runno_array
            .append_value({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder.marketfeeid_array.append_value(row.marketfeeid());
        builder
            .marketfeevalue_array
            .append_option({
                row.marketfeevalue
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .energy_array
            .append_option({
                row.energy
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder.participantcategoryid_array.append_value(row.participantcategoryid());
        builder
            .feerate_array
            .append_option({
                row.feerate
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .feeunits_array
            .append_option({
                row.feeunits
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marketfeeid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.marketfeevalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.energy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantcategoryid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.feerate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.feeunits_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsMarketfees6Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    marketfeeid_array: arrow::array::builder::StringBuilder,
    marketfeevalue_array: arrow::array::builder::Decimal128Builder,
    energy_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    participantcategoryid_array: arrow::array::builder::StringBuilder,
    feerate_array: arrow::array::builder::Decimal128Builder,
    feeunits_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsReallocations5;
pub struct SettlementsReallocations5Mapping([usize; 9]);
/// # Summary
///
/// ## SETREALLOCATIONS
///  _SETREALLOCATIONS shows the trading interval value of reallocations processed, for those participants whose reallocation submissions have been accepted by AEMO._
///
/// * Data Set Name: Settlements
/// * File Name: Reallocations
/// * Data Version: 5
///
/// # Description
///  SETREALLOCATIONS data is confidential to participants party to the reallocation. Source SETREALLOCATIONS updates by the posting of a billing run. Volume Generally, there are approximately 550 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * PERIODID
/// * REALLOCATIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsReallocations5Row<'data> {
    /// Calendar Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub runno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Reallocation contract identifier
    pub reallocationid: core::ops::Range<usize>,
    /// Reallocation value in $
    pub reallocationvalue: Option<rust_decimal::Decimal>,
    /// Energy in MWh if reallocation agreement type is MWh
    pub energy: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsReallocations5Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn reallocationid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.reallocationid.clone(),
        )
    }
}
impl mmsdm_core::GetTable for SettlementsReallocations5 {
    const VERSION: i32 = 5;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "REALLOCATIONS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsReallocations5Mapping([
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
        "SETTLEMENTDATE",
        "RUNNO",
        "PERIODID",
        "PARTICIPANTID",
        "REALLOCATIONID",
        "REALLOCATIONVALUE",
        "ENERGY",
        "RRP",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementsReallocations5Row<'row>;
    type FieldMapping = SettlementsReallocations5Mapping;
    type PrimaryKey = SettlementsReallocations5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsReallocations5Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row
                .get_custom_parsed_at_idx(
                    "runno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[3])?,
            reallocationid: row.get_range("reallocationid", field_mapping.0[4])?,
            reallocationvalue: row
                .get_opt_custom_parsed_at_idx(
                    "reallocationvalue",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            energy: row
                .get_opt_custom_parsed_at_idx(
                    "energy",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp: row
                .get_opt_custom_parsed_at_idx(
                    "rrp",
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
        Ok(SettlementsReallocations5Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsReallocations5PrimaryKey {
        SettlementsReallocations5PrimaryKey {
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            reallocationid: row.reallocationid().to_string(),
            runno: row.runno,
            settlementdate: row.settlementdate,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_reallocations_v5_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsReallocations5Row {
            settlementdate: row.settlementdate.clone(),
            runno: row.runno.clone(),
            periodid: row.periodid.clone(),
            participantid: row.participantid.clone(),
            reallocationid: row.reallocationid.clone(),
            reallocationvalue: row.reallocationvalue.clone(),
            energy: row.energy.clone(),
            rrp: row.rrp.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsReallocations5PrimaryKey {
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub reallocationid: alloc::string::String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for SettlementsReallocations5PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsReallocations5Row<'data> {
    type Row<'other> = SettlementsReallocations5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid() == row.participantid() && self.periodid == row.periodid
            && self.reallocationid() == row.reallocationid() && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsReallocations5Row<'data> {
    type PrimaryKey = SettlementsReallocations5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid() == key.participantid && self.periodid == key.periodid
            && self.reallocationid() == key.reallocationid && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsReallocations5PrimaryKey {
    type Row<'other> = SettlementsReallocations5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid == row.participantid() && self.periodid == row.periodid
            && self.reallocationid == row.reallocationid() && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsReallocations5PrimaryKey {
    type PrimaryKey = SettlementsReallocations5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid && self.periodid == key.periodid
            && self.reallocationid == key.reallocationid && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsReallocations5 {
    type Builder = SettlementsReallocations5Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "runno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "reallocationid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "reallocationvalue",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "energy",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp",
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
        SettlementsReallocations5Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            reallocationid_array: arrow::array::builder::StringBuilder::new(),
            reallocationvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            energy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .runno_array
            .append_value({
                let mut val = row.runno;
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
        builder.participantid_array.append_value(row.participantid());
        builder.reallocationid_array.append_value(row.reallocationid());
        builder
            .reallocationvalue_array
            .append_option({
                row.reallocationvalue
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .energy_array
            .append_option({
                row.energy
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp_array
            .append_option({
                row.rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reallocationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reallocationvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.energy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsReallocations5Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    reallocationid_array: arrow::array::builder::StringBuilder,
    reallocationvalue_array: arrow::array::builder::Decimal128Builder,
    energy_array: arrow::array::builder::Decimal128Builder,
    rrp_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementsRestartpayment6;
pub struct SettlementsRestartpayment6Mapping([usize; 16]);
/// # Summary
///
/// ## SETRESTARTPAYMENT
///  _SETRESTARTPAYMENT shows specific payment details for System Restart services by period._
///
/// * Data Set Name: Settlements
/// * File Name: Restartpayment
/// * Data Version: 6
///
/// # Description
///  SETRESTARTPAYMENT data is confidential to the relevant participant. Source SETRESTARTPAYMENT updates with each settlement run.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsRestartpayment6Row<'data> {
    /// Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: core::ops::Range<usize>,
    /// Contract Identifier
    pub contractid: core::ops::Range<usize>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// System Restart Type (0 = FRC, 1 = GRC, 2 = TTH)
    pub restarttype: Option<rust_decimal::Decimal>,
    /// Availability Flag
    pub avaflag: Option<rust_decimal::Decimal>,
    /// Availability Price
    pub availabilityprice: Option<rust_decimal::Decimal>,
    /// Service Test Flag
    pub tcf: Option<rust_decimal::Decimal>,
    /// Availability Payment
    pub availabilitypayment: Option<rust_decimal::Decimal>,
    /// Contract Version No.
    pub contractversionno: Option<rust_decimal::Decimal>,
    /// Re-offer offer date
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// Re-Offer Version No.
    pub offerversionno: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The enabling payment made for system restart in this half-hour interval
    pub enablingpayment: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsRestartpayment6Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
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
impl mmsdm_core::GetTable for SettlementsRestartpayment6 {
    const VERSION: i32 = 6;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "RESTARTPAYMENT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsRestartpayment6Mapping([
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
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PARTICIPANTID",
        "CONTRACTID",
        "PERIODID",
        "REGIONID",
        "RESTARTTYPE",
        "AVAFLAG",
        "AVAILABILITYPRICE",
        "TCF",
        "AVAILABILITYPAYMENT",
        "CONTRACTVERSIONNO",
        "OFFERDATE",
        "OFFERVERSIONNO",
        "LASTCHANGED",
        "ENABLINGPAYMENT",
    ];
    type Row<'row> = SettlementsRestartpayment6Row<'row>;
    type FieldMapping = SettlementsRestartpayment6Mapping;
    type PrimaryKey = SettlementsRestartpayment6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsRestartpayment6Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
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
            contractid: row.get_range("contractid", field_mapping.0[3])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_opt_range("regionid", field_mapping.0[5])?,
            restarttype: row
                .get_opt_custom_parsed_at_idx(
                    "restarttype",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            avaflag: row
                .get_opt_custom_parsed_at_idx(
                    "avaflag",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            availabilityprice: row
                .get_opt_custom_parsed_at_idx(
                    "availabilityprice",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            tcf: row
                .get_opt_custom_parsed_at_idx(
                    "tcf",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            availabilitypayment: row
                .get_opt_custom_parsed_at_idx(
                    "availabilitypayment",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            contractversionno: row
                .get_opt_custom_parsed_at_idx(
                    "contractversionno",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            offerdate: row
                .get_opt_custom_parsed_at_idx(
                    "offerdate",
                    field_mapping.0[12],
                    mmsdm_core::mms_datetime::parse,
                )?,
            offerversionno: row
                .get_opt_custom_parsed_at_idx(
                    "offerversionno",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[14],
                    mmsdm_core::mms_datetime::parse,
                )?,
            enablingpayment: row
                .get_opt_custom_parsed_at_idx(
                    "enablingpayment",
                    field_mapping.0[15],
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
        Ok(SettlementsRestartpayment6Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsRestartpayment6PrimaryKey {
        SettlementsRestartpayment6PrimaryKey {
            contractid: row.contractid().to_string(),
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_restartpayment_v6_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsRestartpayment6Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            participantid: row.participantid.clone(),
            contractid: row.contractid.clone(),
            periodid: row.periodid.clone(),
            regionid: row.regionid.clone(),
            restarttype: row.restarttype.clone(),
            avaflag: row.avaflag.clone(),
            availabilityprice: row.availabilityprice.clone(),
            tcf: row.tcf.clone(),
            availabilitypayment: row.availabilitypayment.clone(),
            contractversionno: row.contractversionno.clone(),
            offerdate: row.offerdate.clone(),
            offerversionno: row.offerversionno.clone(),
            lastchanged: row.lastchanged.clone(),
            enablingpayment: row.enablingpayment.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsRestartpayment6PrimaryKey {
    pub contractid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsRestartpayment6PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsRestartpayment6Row<'data> {
    type Row<'other> = SettlementsRestartpayment6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid()
            && self.participantid() == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsRestartpayment6Row<'data> {
    type PrimaryKey = SettlementsRestartpayment6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.participantid() == key.participantid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsRestartpayment6PrimaryKey {
    type Row<'other> = SettlementsRestartpayment6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.participantid == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsRestartpayment6PrimaryKey {
    type PrimaryKey = SettlementsRestartpayment6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.participantid == key.participantid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsRestartpayment6 {
    type Builder = SettlementsRestartpayment6Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
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
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "restarttype",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "avaflag",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "availabilityprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "tcf",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "availabilitypayment",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "contractversionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offerdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offerversionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
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
                    "enablingpayment",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsRestartpayment6Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            contractid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            restarttype_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            avaflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            availabilityprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            tcf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            availabilitypayment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            contractversionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offerversionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            enablingpayment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.contractid_array.append_value(row.contractid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder.regionid_array.append_option(row.regionid());
        builder
            .restarttype_array
            .append_option({
                row.restarttype
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .avaflag_array
            .append_option({
                row.avaflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .availabilityprice_array
            .append_option({
                row.availabilityprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .tcf_array
            .append_option({
                row.tcf
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .availabilitypayment_array
            .append_option({
                row.availabilitypayment
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .contractversionno_array
            .append_option({
                row.contractversionno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .offerdate_array
            .append_option(row.offerdate.map(|val| val.timestamp_millis()));
        builder
            .offerversionno_array
            .append_option({
                row.offerversionno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .enablingpayment_array
            .append_option({
                row.enablingpayment
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.restarttype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.avaflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.availabilityprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tcf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.availabilitypayment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractversionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerversionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enablingpayment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsRestartpayment6Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    contractid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    restarttype_array: arrow::array::builder::Decimal128Builder,
    avaflag_array: arrow::array::builder::Decimal128Builder,
    availabilityprice_array: arrow::array::builder::Decimal128Builder,
    tcf_array: arrow::array::builder::Decimal128Builder,
    availabilitypayment_array: arrow::array::builder::Decimal128Builder,
    contractversionno_array: arrow::array::builder::Decimal128Builder,
    offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offerversionno_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    enablingpayment_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsRestartrecovery6;
pub struct SettlementsRestartrecovery6Mapping([usize; 17]);
/// # Summary
///
/// ## SETRESTARTRECOVERY
///  _SETRESTARTRECOVERY shows reimbursements for system restart Ancillary Services to be recovered from participants. (Data no longer created for Settlement Days from 01/07/2012)_
///
/// * Data Set Name: Settlements
/// * File Name: Restartrecovery
/// * Data Version: 6
///
/// # Description
///  SETRESTARTRECOVERY data is confidential to the relevant participant. Source SETRESTARTRECOVERY updates with each settlement run.
///
///
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsRestartrecovery6Row<'data> {
    /// Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant to pay recovery
    pub participantid: core::ops::Range<usize>,
    /// Contract Identifier
    pub contractid: core::ops::Range<usize>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Availability Payment
    pub availabilitypayment: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region
    pub participantdemand: Option<rust_decimal::Decimal>,
    /// NEM Demand (NB sum of ALL Regions)
    pub regiondemand: Option<rust_decimal::Decimal>,
    /// Availability Recovery
    pub availabilityrecovery: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Availability Recovery for Generator
    pub availabilityrecovery_gen: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region for Generator
    pub participantdemand_gen: Option<rust_decimal::Decimal>,
    /// Sum of all generation including SGA generation across all regions of the NEM and floored to zero
    pub regiondemand_gen: Option<rust_decimal::Decimal>,
    /// The enabling payment made for system restart in this half-hour interval
    pub enablingpayment: Option<rust_decimal::Decimal>,
    /// The enabling recovery amount for system restart in this half-hour interval attributable to customer activity
    pub enablingrecovery: Option<rust_decimal::Decimal>,
    /// The enabling recovery amount for system restart in this half-hour interval attributable to generator activity
    pub enablingrecovery_gen: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsRestartrecovery6Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn contractid(&self) -> Option<&str> {
        if self.contractid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.contractid.clone(),
                ),
            )
        }
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsRestartrecovery6 {
    const VERSION: i32 = 6;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "RESTARTRECOVERY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsRestartrecovery6Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PARTICIPANTID",
        "CONTRACTID",
        "PERIODID",
        "REGIONID",
        "AVAILABILITYPAYMENT",
        "PARTICIPANTDEMAND",
        "REGIONDEMAND",
        "AVAILABILITYRECOVERY",
        "LASTCHANGED",
        "AVAILABILITYRECOVERY_GEN",
        "PARTICIPANTDEMAND_GEN",
        "REGIONDEMAND_GEN",
        "ENABLINGPAYMENT",
        "ENABLINGRECOVERY",
        "ENABLINGRECOVERY_GEN",
    ];
    type Row<'row> = SettlementsRestartrecovery6Row<'row>;
    type FieldMapping = SettlementsRestartrecovery6Mapping;
    type PrimaryKey = SettlementsRestartrecovery6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsRestartrecovery6Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
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
            contractid: row.get_opt_range("contractid", field_mapping.0[3])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[5])?,
            availabilitypayment: row
                .get_opt_custom_parsed_at_idx(
                    "availabilitypayment",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantdemand: row
                .get_opt_custom_parsed_at_idx(
                    "participantdemand",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regiondemand: row
                .get_opt_custom_parsed_at_idx(
                    "regiondemand",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            availabilityrecovery: row
                .get_opt_custom_parsed_at_idx(
                    "availabilityrecovery",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            availabilityrecovery_gen: row
                .get_opt_custom_parsed_at_idx(
                    "availabilityrecovery_gen",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantdemand_gen: row
                .get_opt_custom_parsed_at_idx(
                    "participantdemand_gen",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regiondemand_gen: row
                .get_opt_custom_parsed_at_idx(
                    "regiondemand_gen",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            enablingpayment: row
                .get_opt_custom_parsed_at_idx(
                    "enablingpayment",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            enablingrecovery: row
                .get_opt_custom_parsed_at_idx(
                    "enablingrecovery",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            enablingrecovery_gen: row
                .get_opt_custom_parsed_at_idx(
                    "enablingrecovery_gen",
                    field_mapping.0[16],
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
        Ok(SettlementsRestartrecovery6Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsRestartrecovery6PrimaryKey {
        SettlementsRestartrecovery6PrimaryKey {
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_restartrecovery_v6_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsRestartrecovery6Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            participantid: row.participantid.clone(),
            contractid: row.contractid.clone(),
            periodid: row.periodid.clone(),
            regionid: row.regionid.clone(),
            availabilitypayment: row.availabilitypayment.clone(),
            participantdemand: row.participantdemand.clone(),
            regiondemand: row.regiondemand.clone(),
            availabilityrecovery: row.availabilityrecovery.clone(),
            lastchanged: row.lastchanged.clone(),
            availabilityrecovery_gen: row.availabilityrecovery_gen.clone(),
            participantdemand_gen: row.participantdemand_gen.clone(),
            regiondemand_gen: row.regiondemand_gen.clone(),
            enablingpayment: row.enablingpayment.clone(),
            enablingrecovery: row.enablingrecovery.clone(),
            enablingrecovery_gen: row.enablingrecovery_gen.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsRestartrecovery6PrimaryKey {
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsRestartrecovery6PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsRestartrecovery6Row<'data> {
    type Row<'other> = SettlementsRestartrecovery6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid() == row.participantid() && self.periodid == row.periodid
            && self.regionid() == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsRestartrecovery6Row<'data> {
    type PrimaryKey = SettlementsRestartrecovery6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid() == key.participantid && self.periodid == key.periodid
            && self.regionid() == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsRestartrecovery6PrimaryKey {
    type Row<'other> = SettlementsRestartrecovery6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid == row.participantid() && self.periodid == row.periodid
            && self.regionid == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsRestartrecovery6PrimaryKey {
    type PrimaryKey = SettlementsRestartrecovery6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid && self.periodid == key.periodid
            && self.regionid == key.regionid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsRestartrecovery6 {
    type Builder = SettlementsRestartrecovery6Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
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
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "availabilitypayment",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "participantdemand",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regiondemand",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "availabilityrecovery",
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
                    "availabilityrecovery_gen",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "participantdemand_gen",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regiondemand_gen",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "enablingpayment",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "enablingrecovery",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "enablingrecovery_gen",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsRestartrecovery6Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            contractid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            availabilitypayment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            participantdemand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            regiondemand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            availabilityrecovery_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            availabilityrecovery_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            participantdemand_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            regiondemand_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            enablingpayment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            enablingrecovery_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            enablingrecovery_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.contractid_array.append_option(row.contractid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder.regionid_array.append_value(row.regionid());
        builder
            .availabilitypayment_array
            .append_option({
                row.availabilitypayment
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .participantdemand_array
            .append_option({
                row.participantdemand
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .regiondemand_array
            .append_option({
                row.regiondemand
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .availabilityrecovery_array
            .append_option({
                row.availabilityrecovery
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .availabilityrecovery_gen_array
            .append_option({
                row.availabilityrecovery_gen
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .participantdemand_gen_array
            .append_option({
                row.participantdemand_gen
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .regiondemand_gen_array
            .append_option({
                row.regiondemand_gen
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .enablingpayment_array
            .append_option({
                row.enablingpayment
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .enablingrecovery_array
            .append_option({
                row.enablingrecovery
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .enablingrecovery_gen_array
            .append_option({
                row.enablingrecovery_gen
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.availabilitypayment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantdemand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regiondemand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.availabilityrecovery_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.availabilityrecovery_gen_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantdemand_gen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regiondemand_gen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enablingpayment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enablingrecovery_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enablingrecovery_gen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsRestartrecovery6Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    contractid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    availabilitypayment_array: arrow::array::builder::Decimal128Builder,
    participantdemand_array: arrow::array::builder::Decimal128Builder,
    regiondemand_array: arrow::array::builder::Decimal128Builder,
    availabilityrecovery_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    availabilityrecovery_gen_array: arrow::array::builder::Decimal128Builder,
    participantdemand_gen_array: arrow::array::builder::Decimal128Builder,
    regiondemand_gen_array: arrow::array::builder::Decimal128Builder,
    enablingpayment_array: arrow::array::builder::Decimal128Builder,
    enablingrecovery_array: arrow::array::builder::Decimal128Builder,
    enablingrecovery_gen_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsRpowerpayment6;
pub struct SettlementsRpowerpayment6Mapping([usize; 29]);
/// # Summary
///
/// ## SETRPOWERPAYMENT
///  _SETRPOWERPAYMENT shows specific payment details for Reactive power services by period._
///
/// * Data Set Name: Settlements
/// * File Name: Rpowerpayment
/// * Data Version: 6
///
/// # Description
///  SETRPOWERPAYMENT data is confidential to the relevant participant. Source SETRPOWERPAYMENT updates with each settlement run.
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsRpowerpayment6Row<'data> {
    /// Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: core::ops::Range<usize>,
    /// AS Contract Identifier
    pub contractid: core::ops::Range<usize>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Dispatchable Unit Identifier
    pub duid: core::ops::Range<usize>,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Eligible Bid Price
    pub ebp: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Availability price per MVAr of RP absorption capability
    pub mvaraprice: Option<rust_decimal::Decimal>,
    /// Enabling Price
    pub mvareprice: Option<rust_decimal::Decimal>,
    /// Availability price per MVAr of RP generation capability
    pub mvargprice: Option<rust_decimal::Decimal>,
    /// Compensation Cap
    pub ccprice: Option<rust_decimal::Decimal>,
    /// Sync Compensation Flag
    pub synccompensation: Option<rust_decimal::Decimal>,
    /// Reactive Power Absorption Capability (MVAr)
    pub mta: Option<rust_decimal::Decimal>,
    /// Reactive Power Generation Capability (MVAr)
    pub mtg: Option<rust_decimal::Decimal>,
    /// Block size of unit
    pub blocksize: Option<rust_decimal::Decimal>,
    /// Availability Flag
    pub avaflag: Option<rust_decimal::Decimal>,
    /// Cleared MW of unit
    pub clearedmw: Option<rust_decimal::Decimal>,
    /// Unconstrained MW of unit
    pub unconstrainedmw: Option<rust_decimal::Decimal>,
    /// Availability Payment
    pub availabilitypayment: Option<rust_decimal::Decimal>,
    /// Enabling Payment
    pub enablingpayment: Option<rust_decimal::Decimal>,
    /// Compensation Payment
    pub ccpayment: Option<rust_decimal::Decimal>,
    /// AS Contract Version No.
    pub contractversionno: Option<rust_decimal::Decimal>,
    /// Re-offer offer date
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// Re-Offer Version No.
    pub offerversionno: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The rebate amount if MegaVar (MVAr) is below the threshold.
    pub availabilitypayment_rebate: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsRpowerpayment6Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
    pub fn duid(&self) -> Option<&str> {
        if self.duid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone()),
            )
        }
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
impl mmsdm_core::GetTable for SettlementsRpowerpayment6 {
    const VERSION: i32 = 6;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "RPOWERPAYMENT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsRpowerpayment6Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PARTICIPANTID",
        "CONTRACTID",
        "PERIODID",
        "DUID",
        "REGIONID",
        "TLF",
        "EBP",
        "RRP",
        "MVARAPRICE",
        "MVAREPRICE",
        "MVARGPRICE",
        "CCPRICE",
        "SYNCCOMPENSATION",
        "MTA",
        "MTG",
        "BLOCKSIZE",
        "AVAFLAG",
        "CLEAREDMW",
        "UNCONSTRAINEDMW",
        "AVAILABILITYPAYMENT",
        "ENABLINGPAYMENT",
        "CCPAYMENT",
        "CONTRACTVERSIONNO",
        "OFFERDATE",
        "OFFERVERSIONNO",
        "LASTCHANGED",
        "AVAILABILITYPAYMENT_REBATE",
    ];
    type Row<'row> = SettlementsRpowerpayment6Row<'row>;
    type FieldMapping = SettlementsRpowerpayment6Mapping;
    type PrimaryKey = SettlementsRpowerpayment6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsRpowerpayment6Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
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
            contractid: row.get_range("contractid", field_mapping.0[3])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            duid: row.get_opt_range("duid", field_mapping.0[5])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[6])?,
            tlf: row
                .get_opt_custom_parsed_at_idx(
                    "tlf",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ebp: row
                .get_opt_custom_parsed_at_idx(
                    "ebp",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp: row
                .get_opt_custom_parsed_at_idx(
                    "rrp",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mvaraprice: row
                .get_opt_custom_parsed_at_idx(
                    "mvaraprice",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mvareprice: row
                .get_opt_custom_parsed_at_idx(
                    "mvareprice",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mvargprice: row
                .get_opt_custom_parsed_at_idx(
                    "mvargprice",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ccprice: row
                .get_opt_custom_parsed_at_idx(
                    "ccprice",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            synccompensation: row
                .get_opt_custom_parsed_at_idx(
                    "synccompensation",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mta: row
                .get_opt_custom_parsed_at_idx(
                    "mta",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mtg: row
                .get_opt_custom_parsed_at_idx(
                    "mtg",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            blocksize: row
                .get_opt_custom_parsed_at_idx(
                    "blocksize",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            avaflag: row
                .get_opt_custom_parsed_at_idx(
                    "avaflag",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            clearedmw: row
                .get_opt_custom_parsed_at_idx(
                    "clearedmw",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            unconstrainedmw: row
                .get_opt_custom_parsed_at_idx(
                    "unconstrainedmw",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            availabilitypayment: row
                .get_opt_custom_parsed_at_idx(
                    "availabilitypayment",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            enablingpayment: row
                .get_opt_custom_parsed_at_idx(
                    "enablingpayment",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            ccpayment: row
                .get_opt_custom_parsed_at_idx(
                    "ccpayment",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            contractversionno: row
                .get_opt_custom_parsed_at_idx(
                    "contractversionno",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            offerdate: row
                .get_opt_custom_parsed_at_idx(
                    "offerdate",
                    field_mapping.0[25],
                    mmsdm_core::mms_datetime::parse,
                )?,
            offerversionno: row
                .get_opt_custom_parsed_at_idx(
                    "offerversionno",
                    field_mapping.0[26],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[27],
                    mmsdm_core::mms_datetime::parse,
                )?,
            availabilitypayment_rebate: row
                .get_opt_custom_parsed_at_idx(
                    "availabilitypayment_rebate",
                    field_mapping.0[28],
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
        Ok(SettlementsRpowerpayment6Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsRpowerpayment6PrimaryKey {
        SettlementsRpowerpayment6PrimaryKey {
            contractid: row.contractid().to_string(),
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_rpowerpayment_v6_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsRpowerpayment6Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            participantid: row.participantid.clone(),
            contractid: row.contractid.clone(),
            periodid: row.periodid.clone(),
            duid: row.duid.clone(),
            regionid: row.regionid.clone(),
            tlf: row.tlf.clone(),
            ebp: row.ebp.clone(),
            rrp: row.rrp.clone(),
            mvaraprice: row.mvaraprice.clone(),
            mvareprice: row.mvareprice.clone(),
            mvargprice: row.mvargprice.clone(),
            ccprice: row.ccprice.clone(),
            synccompensation: row.synccompensation.clone(),
            mta: row.mta.clone(),
            mtg: row.mtg.clone(),
            blocksize: row.blocksize.clone(),
            avaflag: row.avaflag.clone(),
            clearedmw: row.clearedmw.clone(),
            unconstrainedmw: row.unconstrainedmw.clone(),
            availabilitypayment: row.availabilitypayment.clone(),
            enablingpayment: row.enablingpayment.clone(),
            ccpayment: row.ccpayment.clone(),
            contractversionno: row.contractversionno.clone(),
            offerdate: row.offerdate.clone(),
            offerversionno: row.offerversionno.clone(),
            lastchanged: row.lastchanged.clone(),
            availabilitypayment_rebate: row.availabilitypayment_rebate.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsRpowerpayment6PrimaryKey {
    pub contractid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsRpowerpayment6PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsRpowerpayment6Row<'data> {
    type Row<'other> = SettlementsRpowerpayment6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid()
            && self.participantid() == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsRpowerpayment6Row<'data> {
    type PrimaryKey = SettlementsRpowerpayment6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.participantid() == key.participantid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsRpowerpayment6PrimaryKey {
    type Row<'other> = SettlementsRpowerpayment6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.participantid == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsRpowerpayment6PrimaryKey {
    type PrimaryKey = SettlementsRpowerpayment6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.participantid == key.participantid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsRpowerpayment6 {
    type Builder = SettlementsRpowerpayment6Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
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
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "tlf",
                    arrow::datatypes::DataType::Decimal128(7, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ebp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mvaraprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mvareprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mvargprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ccprice",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "synccompensation",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mta",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mtg",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "blocksize",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "avaflag",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "clearedmw",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "unconstrainedmw",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "availabilitypayment",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "enablingpayment",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ccpayment",
                    arrow::datatypes::DataType::Decimal128(15, 5),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "contractversionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offerdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "offerversionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
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
                    "availabilitypayment_rebate",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsRpowerpayment6Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            contractid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            duid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            tlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(7, 5)),
            ebp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mvaraprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mvareprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mvargprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            ccprice_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            synccompensation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            mta_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            mtg_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            blocksize_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            avaflag_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            clearedmw_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            unconstrainedmw_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            availabilitypayment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            enablingpayment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            ccpayment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            contractversionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            offerdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            offerversionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            availabilitypayment_rebate_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.contractid_array.append_value(row.contractid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder.duid_array.append_option(row.duid());
        builder.regionid_array.append_option(row.regionid());
        builder
            .tlf_array
            .append_option({
                row.tlf
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .ebp_array
            .append_option({
                row.ebp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .rrp_array
            .append_option({
                row.rrp
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mvaraprice_array
            .append_option({
                row.mvaraprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mvareprice_array
            .append_option({
                row.mvareprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mvargprice_array
            .append_option({
                row.mvargprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .ccprice_array
            .append_option({
                row.ccprice
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .synccompensation_array
            .append_option({
                row.synccompensation
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
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .mtg_array
            .append_option({
                row.mtg
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .blocksize_array
            .append_option({
                row.blocksize
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .avaflag_array
            .append_option({
                row.avaflag
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .clearedmw_array
            .append_option({
                row.clearedmw
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .unconstrainedmw_array
            .append_option({
                row.unconstrainedmw
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .availabilitypayment_array
            .append_option({
                row.availabilitypayment
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .enablingpayment_array
            .append_option({
                row.enablingpayment
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .ccpayment_array
            .append_option({
                row.ccpayment
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .contractversionno_array
            .append_option({
                row.contractversionno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .offerdate_array
            .append_option(row.offerdate.map(|val| val.timestamp_millis()));
        builder
            .offerversionno_array
            .append_option({
                row.offerversionno
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .availabilitypayment_rebate_array
            .append_option({
                row.availabilitypayment_rebate
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ebp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mvaraprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mvareprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mvargprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ccprice_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.synccompensation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mta_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mtg_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.blocksize_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.avaflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.clearedmw_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.unconstrainedmw_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.availabilitypayment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enablingpayment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ccpayment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractversionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.offerversionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.availabilitypayment_rebate_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsRpowerpayment6Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    contractid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    duid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    tlf_array: arrow::array::builder::Decimal128Builder,
    ebp_array: arrow::array::builder::Decimal128Builder,
    rrp_array: arrow::array::builder::Decimal128Builder,
    mvaraprice_array: arrow::array::builder::Decimal128Builder,
    mvareprice_array: arrow::array::builder::Decimal128Builder,
    mvargprice_array: arrow::array::builder::Decimal128Builder,
    ccprice_array: arrow::array::builder::Decimal128Builder,
    synccompensation_array: arrow::array::builder::Decimal128Builder,
    mta_array: arrow::array::builder::Decimal128Builder,
    mtg_array: arrow::array::builder::Decimal128Builder,
    blocksize_array: arrow::array::builder::Decimal128Builder,
    avaflag_array: arrow::array::builder::Decimal128Builder,
    clearedmw_array: arrow::array::builder::Decimal128Builder,
    unconstrainedmw_array: arrow::array::builder::Decimal128Builder,
    availabilitypayment_array: arrow::array::builder::Decimal128Builder,
    enablingpayment_array: arrow::array::builder::Decimal128Builder,
    ccpayment_array: arrow::array::builder::Decimal128Builder,
    contractversionno_array: arrow::array::builder::Decimal128Builder,
    offerdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    offerversionno_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    availabilitypayment_rebate_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsSmallgendata1;
pub struct SettlementsSmallgendata1Mapping([usize; 13]);
/// # Summary
///
/// ## SETSMALLGENDATA
///  _Publishes metering data and associated settlement values for with a registered Small Generator Aggregator participants connection points._
///
/// * Data Set Name: Settlements
/// * File Name: Smallgendata
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CONNECTIONPOINTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsSmallgendata1Row<'data> {
    /// Settlement date
    pub settlementdate: chrono::NaiveDateTime,
    /// Version number of the record for the settlement date
    pub versionno: rust_decimal::Decimal,
    /// Transmission Node Identifier (TNI)
    pub connectionpointid: core::ops::Range<usize>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// The import direction value for the meter read (MWh)
    pub importenergy: Option<rust_decimal::Decimal>,
    /// The export direction value for the meter read (MWh)
    pub exportenergy: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Import Energy Cost ($)
    pub impenergycost: Option<rust_decimal::Decimal>,
    /// Export Energy Cost ($)
    pub expenergycost: Option<rust_decimal::Decimal>,
    /// Last date and time the record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsSmallgendata1Row<'data> {
    pub fn connectionpointid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.connectionpointid.clone(),
        )
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
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
impl mmsdm_core::GetTable for SettlementsSmallgendata1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "SMALLGENDATA";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsSmallgendata1Mapping([
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
        "SETTLEMENTDATE",
        "VERSIONNO",
        "CONNECTIONPOINTID",
        "PERIODID",
        "PARTICIPANTID",
        "REGIONID",
        "IMPORTENERGY",
        "EXPORTENERGY",
        "RRP",
        "TLF",
        "IMPENERGYCOST",
        "EXPENERGYCOST",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementsSmallgendata1Row<'row>;
    type FieldMapping = SettlementsSmallgendata1Mapping;
    type PrimaryKey = SettlementsSmallgendata1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsSmallgendata1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            connectionpointid: row.get_range("connectionpointid", field_mapping.0[2])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[4])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[5])?,
            importenergy: row
                .get_opt_custom_parsed_at_idx(
                    "importenergy",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            exportenergy: row
                .get_opt_custom_parsed_at_idx(
                    "exportenergy",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp: row
                .get_opt_custom_parsed_at_idx(
                    "rrp",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            tlf: row
                .get_opt_custom_parsed_at_idx(
                    "tlf",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            impenergycost: row
                .get_opt_custom_parsed_at_idx(
                    "impenergycost",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            expenergycost: row
                .get_opt_custom_parsed_at_idx(
                    "expenergycost",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[12],
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
        Ok(SettlementsSmallgendata1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsSmallgendata1PrimaryKey {
        SettlementsSmallgendata1PrimaryKey {
            connectionpointid: row.connectionpointid().to_string(),
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_smallgendata_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsSmallgendata1Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            connectionpointid: row.connectionpointid.clone(),
            periodid: row.periodid.clone(),
            participantid: row.participantid.clone(),
            regionid: row.regionid.clone(),
            importenergy: row.importenergy.clone(),
            exportenergy: row.exportenergy.clone(),
            rrp: row.rrp.clone(),
            tlf: row.tlf.clone(),
            impenergycost: row.impenergycost.clone(),
            expenergycost: row.expenergycost.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsSmallgendata1PrimaryKey {
    pub connectionpointid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsSmallgendata1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsSmallgendata1Row<'data> {
    type Row<'other> = SettlementsSmallgendata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.connectionpointid() == row.connectionpointid()
            && self.participantid() == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsSmallgendata1Row<'data> {
    type PrimaryKey = SettlementsSmallgendata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid() == key.connectionpointid
            && self.participantid() == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsSmallgendata1PrimaryKey {
    type Row<'other> = SettlementsSmallgendata1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.connectionpointid == row.connectionpointid()
            && self.participantid == row.participantid() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsSmallgendata1PrimaryKey {
    type PrimaryKey = SettlementsSmallgendata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid == key.connectionpointid
            && self.participantid == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsSmallgendata1 {
    type Builder = SettlementsSmallgendata1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
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
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "importenergy",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "exportenergy",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "tlf",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "impenergycost",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "expenergycost",
                    arrow::datatypes::DataType::Decimal128(18, 8),
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
        SettlementsSmallgendata1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            importenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            exportenergy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            tlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            impenergycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            expenergycost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.connectionpointid_array.append_value(row.connectionpointid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.regionid_array.append_option(row.regionid());
        builder
            .importenergy_array
            .append_option({
                row.importenergy
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .exportenergy_array
            .append_option({
                row.exportenergy
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .rrp_array
            .append_option({
                row.rrp
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .tlf_array
            .append_option({
                row.tlf
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .impenergycost_array
            .append_option({
                row.impenergycost
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .expenergycost_array
            .append_option({
                row.expenergycost
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.importenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.exportenergy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.impenergycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.expenergycost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsSmallgendata1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    connectionpointid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    importenergy_array: arrow::array::builder::Decimal128Builder,
    exportenergy_array: arrow::array::builder::Decimal128Builder,
    rrp_array: arrow::array::builder::Decimal128Builder,
    tlf_array: arrow::array::builder::Decimal128Builder,
    impenergycost_array: arrow::array::builder::Decimal128Builder,
    expenergycost_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementsAncillarySummary5;
pub struct SettlementsAncillarySummary5Mapping([usize; 8]);
/// # Summary
///
/// ## SET_ANCILLARY_SUMMARY
///  _SET_ANCILLARY_SUMMARY summarises payments for all Ancillary Services to participants on the basis of regions and trading intervals._
///
/// * Data Set Name: Settlements
/// * File Name: Ancillary Summary
/// * Data Version: 5
///
/// # Description
///  SET_ANCILLARY_SUMMARY data is available to all participants. Volume Approximately 30, 000 per week.
///
///
///
/// # Primary Key Columns
///
/// * PAYMENTTYPE
/// * PERIODID
/// * REGIONID
/// * SERVICE
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsAncillarySummary5Row<'data> {
    /// Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No
    pub versionno: rust_decimal::Decimal,
    /// Ancillary service identifier (e.g. REACTIVE_POWER)
    pub service: core::ops::Range<usize>,
    /// Payment type identifier (e.g. COMPENSATION)
    pub paymenttype: core::ops::Range<usize>,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Trading interval
    pub periodid: rust_decimal::Decimal,
    /// The NEM ancillary summary regional payment amount ($)
    pub paymentamount: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsAncillarySummary5Row<'data> {
    pub fn service(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.service.clone())
    }
    pub fn paymenttype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.paymenttype.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsAncillarySummary5 {
    const VERSION: i32 = 5;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "ANCILLARY_SUMMARY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsAncillarySummary5Mapping([
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
        "SETTLEMENTDATE",
        "VERSIONNO",
        "SERVICE",
        "PAYMENTTYPE",
        "REGIONID",
        "PERIODID",
        "PAYMENTAMOUNT",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementsAncillarySummary5Row<'row>;
    type FieldMapping = SettlementsAncillarySummary5Mapping;
    type PrimaryKey = SettlementsAncillarySummary5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsAncillarySummary5Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            service: row.get_range("service", field_mapping.0[2])?,
            paymenttype: row.get_range("paymenttype", field_mapping.0[3])?,
            regionid: row.get_range("regionid", field_mapping.0[4])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            paymentamount: row
                .get_opt_custom_parsed_at_idx(
                    "paymentamount",
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
        Ok(SettlementsAncillarySummary5Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsAncillarySummary5PrimaryKey {
        SettlementsAncillarySummary5PrimaryKey {
            paymenttype: row.paymenttype().to_string(),
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            service: row.service().to_string(),
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_ancillary_summary_v5_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsAncillarySummary5Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            service: row.service.clone(),
            paymenttype: row.paymenttype.clone(),
            regionid: row.regionid.clone(),
            periodid: row.periodid.clone(),
            paymentamount: row.paymentamount.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsAncillarySummary5PrimaryKey {
    pub paymenttype: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub service: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsAncillarySummary5PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsAncillarySummary5Row<'data> {
    type Row<'other> = SettlementsAncillarySummary5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.paymenttype() == row.paymenttype() && self.periodid == row.periodid
            && self.regionid() == row.regionid() && self.service() == row.service()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementsAncillarySummary5Row<'data> {
    type PrimaryKey = SettlementsAncillarySummary5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.paymenttype() == key.paymenttype && self.periodid == key.periodid
            && self.regionid() == key.regionid && self.service() == key.service
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsAncillarySummary5PrimaryKey {
    type Row<'other> = SettlementsAncillarySummary5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.paymenttype == row.paymenttype() && self.periodid == row.periodid
            && self.regionid == row.regionid() && self.service == row.service()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsAncillarySummary5PrimaryKey {
    type PrimaryKey = SettlementsAncillarySummary5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.paymenttype == key.paymenttype && self.periodid == key.periodid
            && self.regionid == key.regionid && self.service == key.service
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsAncillarySummary5 {
    type Builder = SettlementsAncillarySummary5Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
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
                    "service",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "paymenttype",
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
                    "paymentamount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
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
        SettlementsAncillarySummary5Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            service_array: arrow::array::builder::StringBuilder::new(),
            paymenttype_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            paymentamount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.service_array.append_value(row.service());
        builder.paymenttype_array.append_value(row.paymenttype());
        builder.regionid_array.append_value(row.regionid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .paymentamount_array
            .append_option({
                row.paymentamount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.service_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.paymenttype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.paymentamount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsAncillarySummary5Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    service_array: arrow::array::builder::StringBuilder,
    paymenttype_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    paymentamount_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementsApcCompensation1;
pub struct SettlementsApcCompensation1Mapping([usize; 7]);
/// # Summary
///
/// ## SET_APC_COMPENSATION
///  _APC Compensation payment amounts in the Settlements timeframe_
///
/// * Data Set Name: Settlements
/// * File Name: Apc Compensation
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * APEVENTID
/// * CLAIMID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsApcCompensation1Row<'data> {
    /// Settlement run date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub versionno: i64,
    /// AP Event Id
    pub apeventid: i64,
    /// AP Event Claim Id
    pub claimid: i64,
    /// Participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Trading interval identifier
    pub periodid: i64,
    /// Compensation amount for the event claim in this interval
    pub compensation_amount: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsApcCompensation1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsApcCompensation1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "APC_COMPENSATION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsApcCompensation1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "APEVENTID",
        "CLAIMID",
        "PARTICIPANTID",
        "PERIODID",
        "COMPENSATION_AMOUNT",
    ];
    type Row<'row> = SettlementsApcCompensation1Row<'row>;
    type FieldMapping = SettlementsApcCompensation1Mapping;
    type PrimaryKey = SettlementsApcCompensation1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsApcCompensation1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row.get_parsed_at_idx("versionno", field_mapping.0[1])?,
            apeventid: row.get_parsed_at_idx("apeventid", field_mapping.0[2])?,
            claimid: row.get_parsed_at_idx("claimid", field_mapping.0[3])?,
            participantid: row.get_range("participantid", field_mapping.0[4])?,
            periodid: row.get_parsed_at_idx("periodid", field_mapping.0[5])?,
            compensation_amount: row
                .get_opt_custom_parsed_at_idx(
                    "compensation_amount",
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
        Ok(SettlementsApcCompensation1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsApcCompensation1PrimaryKey {
        SettlementsApcCompensation1PrimaryKey {
            apeventid: row.apeventid,
            claimid: row.claimid,
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_apc_compensation_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsApcCompensation1Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            apeventid: row.apeventid.clone(),
            claimid: row.claimid.clone(),
            participantid: row.participantid.clone(),
            periodid: row.periodid.clone(),
            compensation_amount: row.compensation_amount.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsApcCompensation1PrimaryKey {
    pub apeventid: i64,
    pub claimid: i64,
    pub participantid: alloc::string::String,
    pub periodid: i64,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: i64,
}
impl mmsdm_core::PrimaryKey for SettlementsApcCompensation1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsApcCompensation1Row<'data> {
    type Row<'other> = SettlementsApcCompensation1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.apeventid == row.apeventid && self.claimid == row.claimid
            && self.participantid() == row.participantid()
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsApcCompensation1Row<'data> {
    type PrimaryKey = SettlementsApcCompensation1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid && self.claimid == key.claimid
            && self.participantid() == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsApcCompensation1PrimaryKey {
    type Row<'other> = SettlementsApcCompensation1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.apeventid == row.apeventid && self.claimid == row.claimid
            && self.participantid == row.participantid() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsApcCompensation1PrimaryKey {
    type PrimaryKey = SettlementsApcCompensation1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid && self.claimid == key.claimid
            && self.participantid == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsApcCompensation1 {
    type Builder = SettlementsApcCompensation1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "apeventid",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "claimid",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "compensation_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsApcCompensation1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Int64Builder::new(),
            apeventid_array: arrow::array::builder::Int64Builder::new(),
            claimid_array: arrow::array::builder::Int64Builder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Int64Builder::new(),
            compensation_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder.versionno_array.append_value(row.versionno);
        builder.apeventid_array.append_value(row.apeventid);
        builder.claimid_array.append_value(row.claimid);
        builder.participantid_array.append_value(row.participantid());
        builder.periodid_array.append_value(row.periodid);
        builder
            .compensation_amount_array
            .append_option({
                row.compensation_amount
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.apeventid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.claimid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.compensation_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsApcCompensation1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Int64Builder,
    apeventid_array: arrow::array::builder::Int64Builder,
    claimid_array: arrow::array::builder::Int64Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Int64Builder,
    compensation_amount_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsApcRecovery1;
pub struct SettlementsApcRecovery1Mapping([usize; 9]);
/// # Summary
///
/// ## SET_APC_RECOVERY
///  _APC Compensation recovery amounts in the Settlements timeframe_
///
/// * Data Set Name: Settlements
/// * File Name: Apc Recovery
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * APEVENTID
/// * CLAIMID
/// * PARTICIPANTID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsApcRecovery1Row<'data> {
    /// Settlement run date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub versionno: i64,
    /// AP Event Id
    pub apeventid: i64,
    /// AP Event Claim Id
    pub claimid: i64,
    /// Participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Settlements Trading Interval.
    pub periodid: i64,
    /// Region id for the recovery amount
    pub regionid: core::ops::Range<usize>,
    /// Recovery amount in the region attributable to the participant for the event claim in this interval
    pub recovery_amount: Option<rust_decimal::Decimal>,
    /// Total Recovery amount in the region for the event claim in this interval
    pub region_recovery_br_amount: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsApcRecovery1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsApcRecovery1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "APC_RECOVERY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsApcRecovery1Mapping([
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
        "SETTLEMENTDATE",
        "VERSIONNO",
        "APEVENTID",
        "CLAIMID",
        "PARTICIPANTID",
        "PERIODID",
        "REGIONID",
        "RECOVERY_AMOUNT",
        "REGION_RECOVERY_<br>AMOUNT",
    ];
    type Row<'row> = SettlementsApcRecovery1Row<'row>;
    type FieldMapping = SettlementsApcRecovery1Mapping;
    type PrimaryKey = SettlementsApcRecovery1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsApcRecovery1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row.get_parsed_at_idx("versionno", field_mapping.0[1])?,
            apeventid: row.get_parsed_at_idx("apeventid", field_mapping.0[2])?,
            claimid: row.get_parsed_at_idx("claimid", field_mapping.0[3])?,
            participantid: row.get_range("participantid", field_mapping.0[4])?,
            periodid: row.get_parsed_at_idx("periodid", field_mapping.0[5])?,
            regionid: row.get_range("regionid", field_mapping.0[6])?,
            recovery_amount: row
                .get_opt_custom_parsed_at_idx(
                    "recovery_amount",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            region_recovery_br_amount: row
                .get_opt_custom_parsed_at_idx(
                    "region_recovery_br_amount",
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
        Ok(SettlementsApcRecovery1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsApcRecovery1PrimaryKey {
        SettlementsApcRecovery1PrimaryKey {
            apeventid: row.apeventid,
            claimid: row.claimid,
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_apc_recovery_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsApcRecovery1Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            apeventid: row.apeventid.clone(),
            claimid: row.claimid.clone(),
            participantid: row.participantid.clone(),
            periodid: row.periodid.clone(),
            regionid: row.regionid.clone(),
            recovery_amount: row.recovery_amount.clone(),
            region_recovery_br_amount: row.region_recovery_br_amount.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsApcRecovery1PrimaryKey {
    pub apeventid: i64,
    pub claimid: i64,
    pub participantid: alloc::string::String,
    pub periodid: i64,
    pub regionid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: i64,
}
impl mmsdm_core::PrimaryKey for SettlementsApcRecovery1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsApcRecovery1Row<'data> {
    type Row<'other> = SettlementsApcRecovery1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.apeventid == row.apeventid && self.claimid == row.claimid
            && self.participantid() == row.participantid()
            && self.periodid == row.periodid && self.regionid() == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsApcRecovery1Row<'data> {
    type PrimaryKey = SettlementsApcRecovery1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid && self.claimid == key.claimid
            && self.participantid() == key.participantid && self.periodid == key.periodid
            && self.regionid() == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsApcRecovery1PrimaryKey {
    type Row<'other> = SettlementsApcRecovery1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.apeventid == row.apeventid && self.claimid == row.claimid
            && self.participantid == row.participantid() && self.periodid == row.periodid
            && self.regionid == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsApcRecovery1PrimaryKey {
    type PrimaryKey = SettlementsApcRecovery1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid && self.claimid == key.claimid
            && self.participantid == key.participantid && self.periodid == key.periodid
            && self.regionid == key.regionid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsApcRecovery1 {
    type Builder = SettlementsApcRecovery1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "apeventid",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "claimid",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "recovery_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "region_recovery_br_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsApcRecovery1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Int64Builder::new(),
            apeventid_array: arrow::array::builder::Int64Builder::new(),
            claimid_array: arrow::array::builder::Int64Builder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Int64Builder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            recovery_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            region_recovery_br_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder.versionno_array.append_value(row.versionno);
        builder.apeventid_array.append_value(row.apeventid);
        builder.claimid_array.append_value(row.claimid);
        builder.participantid_array.append_value(row.participantid());
        builder.periodid_array.append_value(row.periodid);
        builder.regionid_array.append_value(row.regionid());
        builder
            .recovery_amount_array
            .append_option({
                row.recovery_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .region_recovery_br_amount_array
            .append_option({
                row.region_recovery_br_amount
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.apeventid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.claimid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.recovery_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.region_recovery_br_amount_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsApcRecovery1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Int64Builder,
    apeventid_array: arrow::array::builder::Int64Builder,
    claimid_array: arrow::array::builder::Int64Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Int64Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    recovery_amount_array: arrow::array::builder::Decimal128Builder,
    region_recovery_br_amount_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsEnergyTranSaps1;
pub struct SettlementsEnergyTranSaps1Mapping([usize; 12]);
/// # Summary
///
/// ## SET_ENERGY_TRAN_SAPS
///  _The table shows the Transaction Details for the SAPS Connection Points. The table contains both the MSRPs and Retailers data_
///
/// * Data Set Name: Settlements
/// * File Name: Energy Tran Saps
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * TNI
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsEnergyTranSaps1Row<'data> {
    /// The Settlement Date of the Billing Week
    pub settlementdate: chrono::NaiveDateTime,
    /// The Settlement Run No
    pub versionno: rust_decimal::Decimal,
    /// The Period Id identifier
    pub periodid: rust_decimal::Decimal,
    /// The Participant ID for the SAPS TNI
    pub participantid: core::ops::Range<usize>,
    /// The SAPS Connection Point Identifier
    pub tni: core::ops::Range<usize>,
    /// The SAPS Region ID
    pub regionid: core::ops::Range<usize>,
    /// The SAPS Settlement Price for the Region
    pub saps_rrp: Option<rust_decimal::Decimal>,
    /// The Energy MWh Consumed for that TNI for the Participant ID
    pub consumed_energy_mwh: Option<rust_decimal::Decimal>,
    /// The Energy MWh Sent Out for the TNI for the Participant Id
    pub sentout_energy_mwh: Option<rust_decimal::Decimal>,
    /// The Cost of the Consumed Energy
    pub consumed_energy_cost: Option<rust_decimal::Decimal>,
    /// The Cost of the Sent Out Energy
    pub sentout_energy_cost: Option<rust_decimal::Decimal>,
    /// The Last changed Date time of the record
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsEnergyTranSaps1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn tni(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.tni.clone())
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
impl mmsdm_core::GetTable for SettlementsEnergyTranSaps1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "ENERGY_TRAN_SAPS";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsEnergyTranSaps1Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PERIODID",
        "PARTICIPANTID",
        "TNI",
        "REGIONID",
        "SAPS_RRP",
        "CONSUMED_ENERGY_MWH",
        "SENTOUT_ENERGY_MWH",
        "CONSUMED_ENERGY_COST",
        "SENTOUT_ENERGY_COST",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementsEnergyTranSaps1Row<'row>;
    type FieldMapping = SettlementsEnergyTranSaps1Mapping;
    type PrimaryKey = SettlementsEnergyTranSaps1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsEnergyTranSaps1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[3])?,
            tni: row.get_range("tni", field_mapping.0[4])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[5])?,
            saps_rrp: row
                .get_opt_custom_parsed_at_idx(
                    "saps_rrp",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            consumed_energy_mwh: row
                .get_opt_custom_parsed_at_idx(
                    "consumed_energy_mwh",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sentout_energy_mwh: row
                .get_opt_custom_parsed_at_idx(
                    "sentout_energy_mwh",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            consumed_energy_cost: row
                .get_opt_custom_parsed_at_idx(
                    "consumed_energy_cost",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            sentout_energy_cost: row
                .get_opt_custom_parsed_at_idx(
                    "sentout_energy_cost",
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
        Ok(SettlementsEnergyTranSaps1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsEnergyTranSaps1PrimaryKey {
        SettlementsEnergyTranSaps1PrimaryKey {
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
            tni: row.tni().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_energy_tran_saps_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsEnergyTranSaps1Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            periodid: row.periodid.clone(),
            participantid: row.participantid.clone(),
            tni: row.tni.clone(),
            regionid: row.regionid.clone(),
            saps_rrp: row.saps_rrp.clone(),
            consumed_energy_mwh: row.consumed_energy_mwh.clone(),
            sentout_energy_mwh: row.sentout_energy_mwh.clone(),
            consumed_energy_cost: row.consumed_energy_cost.clone(),
            sentout_energy_cost: row.sentout_energy_cost.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsEnergyTranSaps1PrimaryKey {
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub tni: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsEnergyTranSaps1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsEnergyTranSaps1Row<'data> {
    type Row<'other> = SettlementsEnergyTranSaps1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid() == row.participantid() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate && self.tni() == row.tni()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsEnergyTranSaps1Row<'data> {
    type PrimaryKey = SettlementsEnergyTranSaps1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid() == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate && self.tni() == key.tni
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsEnergyTranSaps1PrimaryKey {
    type Row<'other> = SettlementsEnergyTranSaps1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid == row.participantid() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate && self.tni == row.tni()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsEnergyTranSaps1PrimaryKey {
    type PrimaryKey = SettlementsEnergyTranSaps1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate && self.tni == key.tni
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsEnergyTranSaps1 {
    type Builder = SettlementsEnergyTranSaps1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
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
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "tni",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "saps_rrp",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "consumed_energy_mwh",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sentout_energy_mwh",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "consumed_energy_cost",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "sentout_energy_cost",
                    arrow::datatypes::DataType::Decimal128(18, 8),
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
        SettlementsEnergyTranSaps1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            tni_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            saps_rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            consumed_energy_mwh_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            sentout_energy_mwh_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            consumed_energy_cost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            sentout_energy_cost_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
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
        builder.participantid_array.append_value(row.participantid());
        builder.tni_array.append_value(row.tni());
        builder.regionid_array.append_option(row.regionid());
        builder
            .saps_rrp_array
            .append_option({
                row.saps_rrp
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .consumed_energy_mwh_array
            .append_option({
                row.consumed_energy_mwh
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .sentout_energy_mwh_array
            .append_option({
                row.sentout_energy_mwh
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .consumed_energy_cost_array
            .append_option({
                row.consumed_energy_cost
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .sentout_energy_cost_array
            .append_option({
                row.sentout_energy_cost
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tni_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.saps_rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.consumed_energy_mwh_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sentout_energy_mwh_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.consumed_energy_cost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.sentout_energy_cost_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsEnergyTranSaps1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    tni_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    saps_rrp_array: arrow::array::builder::Decimal128Builder,
    consumed_energy_mwh_array: arrow::array::builder::Decimal128Builder,
    sentout_energy_mwh_array: arrow::array::builder::Decimal128Builder,
    consumed_energy_cost_array: arrow::array::builder::Decimal128Builder,
    sentout_energy_cost_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementsFcasPayment6;
pub struct SettlementsFcasPayment6Mapping([usize; 17]);
/// # Summary
///
/// ## SET_FCAS_PAYMENT
///  _SET_FCAS_PAYMENT sets out the enabling payment details for frequency controlled Ancillary Services._
///
/// * Data Set Name: Settlements
/// * File Name: Fcas Payment
/// * Data Version: 6
///
/// # Description
///  SET_FCAS_PAYMENT data is confidential to the relevant participant. Volume Approximately 150,000 per week.
///
///
///
/// # Primary Key Columns
///
/// * DUID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsFcasPayment6Row<'data> {
    /// Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No
    pub versionno: rust_decimal::Decimal,
    /// Participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Dispatchable unit identifier
    pub duid: core::ops::Range<usize>,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Lower 6 Second Payment
    pub lower6sec_payment: Option<rust_decimal::Decimal>,
    /// Raise 6 Second Payment
    pub raise6sec_payment: Option<rust_decimal::Decimal>,
    /// Lower 60 Second Payment
    pub lower60sec_payment: Option<rust_decimal::Decimal>,
    /// Raise 60 Second Payment
    pub raise60sec_payment: Option<rust_decimal::Decimal>,
    /// Lower 5 Minute Payment
    pub lower5min_payment: Option<rust_decimal::Decimal>,
    /// Raise 5 Minute Payment
    pub raise5min_payment: Option<rust_decimal::Decimal>,
    /// Lower 5 Minute Regulation Payment
    pub lowerreg_payment: Option<rust_decimal::Decimal>,
    /// Raise 5 Minute Regulation Payment
    pub raisereg_payment: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Payment amount for the very fast raise service
    pub raise1sec_payment: Option<rust_decimal::Decimal>,
    /// Payment amount for the very fast lower service
    pub lower1sec_payment: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsFcasPayment6Row<'data> {
    pub fn participantid(&self) -> Option<&str> {
        if self.participantid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.participantid.clone(),
                ),
            )
        }
    }
    pub fn duid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.duid.clone())
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
impl mmsdm_core::GetTable for SettlementsFcasPayment6 {
    const VERSION: i32 = 6;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "FCAS_PAYMENT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsFcasPayment6Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PARTICIPANTID",
        "DUID",
        "REGIONID",
        "PERIODID",
        "LOWER6SEC_PAYMENT",
        "RAISE6SEC_PAYMENT",
        "LOWER60SEC_PAYMENT",
        "RAISE60SEC_PAYMENT",
        "LOWER5MIN_PAYMENT",
        "RAISE5MIN_PAYMENT",
        "LOWERREG_PAYMENT",
        "RAISEREG_PAYMENT",
        "LASTCHANGED",
        "RAISE1SEC_PAYMENT",
        "LOWER1SEC_PAYMENT",
    ];
    type Row<'row> = SettlementsFcasPayment6Row<'row>;
    type FieldMapping = SettlementsFcasPayment6Mapping;
    type PrimaryKey = SettlementsFcasPayment6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsFcasPayment6Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_opt_range("participantid", field_mapping.0[2])?,
            duid: row.get_range("duid", field_mapping.0[3])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[4])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6sec_payment: row
                .get_opt_custom_parsed_at_idx(
                    "lower6sec_payment",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6sec_payment: row
                .get_opt_custom_parsed_at_idx(
                    "raise6sec_payment",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60sec_payment: row
                .get_opt_custom_parsed_at_idx(
                    "lower60sec_payment",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60sec_payment: row
                .get_opt_custom_parsed_at_idx(
                    "raise60sec_payment",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5min_payment: row
                .get_opt_custom_parsed_at_idx(
                    "lower5min_payment",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5min_payment: row
                .get_opt_custom_parsed_at_idx(
                    "raise5min_payment",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerreg_payment: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreg_payment",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereg_payment: row
                .get_opt_custom_parsed_at_idx(
                    "raisereg_payment",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[14],
                    mmsdm_core::mms_datetime::parse,
                )?,
            raise1sec_payment: row
                .get_opt_custom_parsed_at_idx(
                    "raise1sec_payment",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1sec_payment: row
                .get_opt_custom_parsed_at_idx(
                    "lower1sec_payment",
                    field_mapping.0[16],
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
        Ok(SettlementsFcasPayment6Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsFcasPayment6PrimaryKey {
        SettlementsFcasPayment6PrimaryKey {
            duid: row.duid().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_fcas_payment_v6_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsFcasPayment6Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            participantid: row.participantid.clone(),
            duid: row.duid.clone(),
            regionid: row.regionid.clone(),
            periodid: row.periodid.clone(),
            lower6sec_payment: row.lower6sec_payment.clone(),
            raise6sec_payment: row.raise6sec_payment.clone(),
            lower60sec_payment: row.lower60sec_payment.clone(),
            raise60sec_payment: row.raise60sec_payment.clone(),
            lower5min_payment: row.lower5min_payment.clone(),
            raise5min_payment: row.raise5min_payment.clone(),
            lowerreg_payment: row.lowerreg_payment.clone(),
            raisereg_payment: row.raisereg_payment.clone(),
            lastchanged: row.lastchanged.clone(),
            raise1sec_payment: row.raise1sec_payment.clone(),
            lower1sec_payment: row.lower1sec_payment.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsFcasPayment6PrimaryKey {
    pub duid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsFcasPayment6PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsFcasPayment6Row<'data> {
    type Row<'other> = SettlementsFcasPayment6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid() == row.duid() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsFcasPayment6Row<'data> {
    type PrimaryKey = SettlementsFcasPayment6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid() == key.duid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsFcasPayment6PrimaryKey {
    type Row<'other> = SettlementsFcasPayment6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.duid == row.duid() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsFcasPayment6PrimaryKey {
    type PrimaryKey = SettlementsFcasPayment6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsFcasPayment6 {
    type Builder = SettlementsFcasPayment6Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
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
                    true,
                ),
                arrow::datatypes::Field::new(
                    "duid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "lower6sec_payment",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6sec_payment",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60sec_payment",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60sec_payment",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5min_payment",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5min_payment",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerreg_payment",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raisereg_payment",
                    arrow::datatypes::DataType::Decimal128(18, 8),
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
                    "raise1sec_payment",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower1sec_payment",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsFcasPayment6Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            duid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lower6sec_payment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            raise6sec_payment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lower60sec_payment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            raise60sec_payment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lower5min_payment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            raise5min_payment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lowerreg_payment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            raisereg_payment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            raise1sec_payment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lower1sec_payment_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_option(row.participantid());
        builder.duid_array.append_value(row.duid());
        builder.regionid_array.append_option(row.regionid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .lower6sec_payment_array
            .append_option({
                row.lower6sec_payment
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .raise6sec_payment_array
            .append_option({
                row.raise6sec_payment
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lower60sec_payment_array
            .append_option({
                row.lower60sec_payment
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .raise60sec_payment_array
            .append_option({
                row.raise60sec_payment
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lower5min_payment_array
            .append_option({
                row.lower5min_payment
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .raise5min_payment_array
            .append_option({
                row.raise5min_payment
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lowerreg_payment_array
            .append_option({
                row.lowerreg_payment
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .raisereg_payment_array
            .append_option({
                row.raisereg_payment
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .raise1sec_payment_array
            .append_option({
                row.raise1sec_payment
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lower1sec_payment_array
            .append_option({
                row.lower1sec_payment
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.duid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6sec_payment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6sec_payment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60sec_payment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60sec_payment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5min_payment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5min_payment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerreg_payment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raisereg_payment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise1sec_payment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower1sec_payment_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsFcasPayment6Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    duid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    lower6sec_payment_array: arrow::array::builder::Decimal128Builder,
    raise6sec_payment_array: arrow::array::builder::Decimal128Builder,
    lower60sec_payment_array: arrow::array::builder::Decimal128Builder,
    raise60sec_payment_array: arrow::array::builder::Decimal128Builder,
    lower5min_payment_array: arrow::array::builder::Decimal128Builder,
    raise5min_payment_array: arrow::array::builder::Decimal128Builder,
    lowerreg_payment_array: arrow::array::builder::Decimal128Builder,
    raisereg_payment_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    raise1sec_payment_array: arrow::array::builder::Decimal128Builder,
    lower1sec_payment_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsFcasRecovery7;
pub struct SettlementsFcasRecovery7Mapping([usize; 26]);
/// # Summary
///
/// ## SET_FCAS_RECOVERY
///  _SET_FCAS_RECOVERY shows reimbursements for the Frequency Control Ancillary Services (FCAS) to be recovered from participants. Beware of potential confusion with the table SETFCASRECOVERY, which reports reimbursements for Frequency Control Ancillary Services Compensation (now unused)._
///
/// * Data Set Name: Settlements
/// * File Name: Fcas Recovery
/// * Data Version: 7
///
/// # Description
///  SET_FCAS_RECOVERY data is confidential to the relevant participant. Volume Approximately 1, 500, 000 per week.
///
///
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsFcasRecovery7Row<'data> {
    /// Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No
    pub versionno: core::ops::Range<usize>,
    /// Participant identifier
    pub participantid: core::ops::Range<usize>,
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Recovery amount for the Lower 6 Second service attributable to customer connection points
    pub lower6sec_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 6 Second service attributable to customer connection points
    pub raise6sec_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 60 Second service attributable to customer connection points
    pub lower60sec_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 60 Second service attributable to customer connection points
    pub raise60sec_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 5 Minute service attributable to customer connection points
    pub lower5min_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 5 Minute service attributable to customer connection points
    pub raise5min_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower Regulation service attributable to customer connection points
    pub lowerreg_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise Regulation Second service attributable to customer connection points
    pub raisereg_recovery: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Recovery amount for the Lower 6 Second service attributable to generator connection points
    pub lower6sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 6 Second service attributable to generator connection points
    pub raise6sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 60 Second service attributable to generator connection points
    pub lower60sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 60 Second service attributable to generator connection points
    pub raise60sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 5 Minute service attributable to generator connection points
    pub lower5min_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 5 Minute service attributable to generator connection points
    pub raise5min_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower Regulation service attributable to generator connection points
    pub lowerreg_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise Regulation Second service attributable to generator connection points
    pub raisereg_recovery_gen: Option<rust_decimal::Decimal>,
    /// Customer recovery amount for the very fast raise service
    pub raise1sec_recovery: Option<rust_decimal::Decimal>,
    /// Customer recovery amount for the very fast lower service
    pub lower1sec_recovery: Option<rust_decimal::Decimal>,
    /// Generator recovery amount for the very fast raise service
    pub raise1sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Generator recovery amount for the very fast lower service
    pub lower1sec_recovery_gen: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsFcasRecovery7Row<'data> {
    pub fn versionno(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.versionno.clone())
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsFcasRecovery7 {
    const VERSION: i32 = 7;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "FCAS_RECOVERY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsFcasRecovery7Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PARTICIPANTID",
        "REGIONID",
        "PERIODID",
        "LOWER6SEC_RECOVERY",
        "RAISE6SEC_RECOVERY",
        "LOWER60SEC_RECOVERY",
        "RAISE60SEC_RECOVERY",
        "LOWER5MIN_RECOVERY",
        "RAISE5MIN_RECOVERY",
        "LOWERREG_RECOVERY",
        "RAISEREG_RECOVERY",
        "LASTCHANGED",
        "LOWER6SEC_RECOVERY_GEN",
        "RAISE6SEC_RECOVERY_GEN",
        "LOWER60SEC_RECOVERY_GEN",
        "RAISE60SEC_RECOVERY_GEN",
        "LOWER5MIN_RECOVERY_GEN",
        "RAISE5MIN_RECOVERY_GEN",
        "LOWERREG_RECOVERY_GEN",
        "RAISEREG_RECOVERY_GEN",
        "RAISE1SEC_RECOVERY",
        "LOWER1SEC_RECOVERY",
        "RAISE1SEC_RECOVERY_GEN",
        "LOWER1SEC_RECOVERY_GEN",
    ];
    type Row<'row> = SettlementsFcasRecovery7Row<'row>;
    type FieldMapping = SettlementsFcasRecovery7Mapping;
    type PrimaryKey = SettlementsFcasRecovery7PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsFcasRecovery7Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row.get_range("versionno", field_mapping.0[1])?,
            participantid: row.get_range("participantid", field_mapping.0[2])?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower6sec_recovery: row
                .get_opt_custom_parsed_at_idx(
                    "lower6sec_recovery",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6sec_recovery: row
                .get_opt_custom_parsed_at_idx(
                    "raise6sec_recovery",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60sec_recovery: row
                .get_opt_custom_parsed_at_idx(
                    "lower60sec_recovery",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60sec_recovery: row
                .get_opt_custom_parsed_at_idx(
                    "raise60sec_recovery",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5min_recovery: row
                .get_opt_custom_parsed_at_idx(
                    "lower5min_recovery",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5min_recovery: row
                .get_opt_custom_parsed_at_idx(
                    "raise5min_recovery",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerreg_recovery: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreg_recovery",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereg_recovery: row
                .get_opt_custom_parsed_at_idx(
                    "raisereg_recovery",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[13],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lower6sec_recovery_gen: row
                .get_opt_custom_parsed_at_idx(
                    "lower6sec_recovery_gen",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise6sec_recovery_gen: row
                .get_opt_custom_parsed_at_idx(
                    "raise6sec_recovery_gen",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower60sec_recovery_gen: row
                .get_opt_custom_parsed_at_idx(
                    "lower60sec_recovery_gen",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise60sec_recovery_gen: row
                .get_opt_custom_parsed_at_idx(
                    "raise60sec_recovery_gen",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower5min_recovery_gen: row
                .get_opt_custom_parsed_at_idx(
                    "lower5min_recovery_gen",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise5min_recovery_gen: row
                .get_opt_custom_parsed_at_idx(
                    "raise5min_recovery_gen",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lowerreg_recovery_gen: row
                .get_opt_custom_parsed_at_idx(
                    "lowerreg_recovery_gen",
                    field_mapping.0[20],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raisereg_recovery_gen: row
                .get_opt_custom_parsed_at_idx(
                    "raisereg_recovery_gen",
                    field_mapping.0[21],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1sec_recovery: row
                .get_opt_custom_parsed_at_idx(
                    "raise1sec_recovery",
                    field_mapping.0[22],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1sec_recovery: row
                .get_opt_custom_parsed_at_idx(
                    "lower1sec_recovery",
                    field_mapping.0[23],
                    mmsdm_core::mms_decimal::parse,
                )?,
            raise1sec_recovery_gen: row
                .get_opt_custom_parsed_at_idx(
                    "raise1sec_recovery_gen",
                    field_mapping.0[24],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lower1sec_recovery_gen: row
                .get_opt_custom_parsed_at_idx(
                    "lower1sec_recovery_gen",
                    field_mapping.0[25],
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
        Ok(SettlementsFcasRecovery7Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsFcasRecovery7PrimaryKey {
        SettlementsFcasRecovery7PrimaryKey {
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            settlementdate: row.settlementdate,
            versionno: row.versionno().to_string(),
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_fcas_recovery_v7_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsFcasRecovery7Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            participantid: row.participantid.clone(),
            regionid: row.regionid.clone(),
            periodid: row.periodid.clone(),
            lower6sec_recovery: row.lower6sec_recovery.clone(),
            raise6sec_recovery: row.raise6sec_recovery.clone(),
            lower60sec_recovery: row.lower60sec_recovery.clone(),
            raise60sec_recovery: row.raise60sec_recovery.clone(),
            lower5min_recovery: row.lower5min_recovery.clone(),
            raise5min_recovery: row.raise5min_recovery.clone(),
            lowerreg_recovery: row.lowerreg_recovery.clone(),
            raisereg_recovery: row.raisereg_recovery.clone(),
            lastchanged: row.lastchanged.clone(),
            lower6sec_recovery_gen: row.lower6sec_recovery_gen.clone(),
            raise6sec_recovery_gen: row.raise6sec_recovery_gen.clone(),
            lower60sec_recovery_gen: row.lower60sec_recovery_gen.clone(),
            raise60sec_recovery_gen: row.raise60sec_recovery_gen.clone(),
            lower5min_recovery_gen: row.lower5min_recovery_gen.clone(),
            raise5min_recovery_gen: row.raise5min_recovery_gen.clone(),
            lowerreg_recovery_gen: row.lowerreg_recovery_gen.clone(),
            raisereg_recovery_gen: row.raisereg_recovery_gen.clone(),
            raise1sec_recovery: row.raise1sec_recovery.clone(),
            lower1sec_recovery: row.lower1sec_recovery.clone(),
            raise1sec_recovery_gen: row.raise1sec_recovery_gen.clone(),
            lower1sec_recovery_gen: row.lower1sec_recovery_gen.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsFcasRecovery7PrimaryKey {
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for SettlementsFcasRecovery7PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsFcasRecovery7Row<'data> {
    type Row<'other> = SettlementsFcasRecovery7Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid() == row.participantid() && self.periodid == row.periodid
            && self.regionid() == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno() == row.versionno()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsFcasRecovery7Row<'data> {
    type PrimaryKey = SettlementsFcasRecovery7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid() == key.participantid && self.periodid == key.periodid
            && self.regionid() == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno() == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsFcasRecovery7PrimaryKey {
    type Row<'other> = SettlementsFcasRecovery7Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid == row.participantid() && self.periodid == row.periodid
            && self.regionid == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsFcasRecovery7PrimaryKey {
    type PrimaryKey = SettlementsFcasRecovery7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid && self.periodid == key.periodid
            && self.regionid == key.regionid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsFcasRecovery7 {
    type Builder = SettlementsFcasRecovery7Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
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
                    "lower6sec_recovery",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6sec_recovery",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60sec_recovery",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60sec_recovery",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5min_recovery",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5min_recovery",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerreg_recovery",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raisereg_recovery",
                    arrow::datatypes::DataType::Decimal128(18, 8),
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
                    "lower6sec_recovery_gen",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise6sec_recovery_gen",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower60sec_recovery_gen",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise60sec_recovery_gen",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower5min_recovery_gen",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise5min_recovery_gen",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lowerreg_recovery_gen",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raisereg_recovery_gen",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise1sec_recovery",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower1sec_recovery",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "raise1sec_recovery_gen",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lower1sec_recovery_gen",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsFcasRecovery7Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lower6sec_recovery_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            raise6sec_recovery_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lower60sec_recovery_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            raise60sec_recovery_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lower5min_recovery_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            raise5min_recovery_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lowerreg_recovery_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            raisereg_recovery_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lower6sec_recovery_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            raise6sec_recovery_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lower60sec_recovery_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            raise60sec_recovery_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lower5min_recovery_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            raise5min_recovery_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lowerreg_recovery_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            raisereg_recovery_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            raise1sec_recovery_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lower1sec_recovery_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            raise1sec_recovery_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lower1sec_recovery_gen_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder.versionno_array.append_value(row.versionno());
        builder.participantid_array.append_value(row.participantid());
        builder.regionid_array.append_value(row.regionid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .lower6sec_recovery_array
            .append_option({
                row.lower6sec_recovery
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .raise6sec_recovery_array
            .append_option({
                row.raise6sec_recovery
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lower60sec_recovery_array
            .append_option({
                row.lower60sec_recovery
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .raise60sec_recovery_array
            .append_option({
                row.raise60sec_recovery
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lower5min_recovery_array
            .append_option({
                row.lower5min_recovery
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .raise5min_recovery_array
            .append_option({
                row.raise5min_recovery
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lowerreg_recovery_array
            .append_option({
                row.lowerreg_recovery
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .raisereg_recovery_array
            .append_option({
                row.raisereg_recovery
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .lower6sec_recovery_gen_array
            .append_option({
                row.lower6sec_recovery_gen
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .raise6sec_recovery_gen_array
            .append_option({
                row.raise6sec_recovery_gen
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lower60sec_recovery_gen_array
            .append_option({
                row.lower60sec_recovery_gen
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .raise60sec_recovery_gen_array
            .append_option({
                row.raise60sec_recovery_gen
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lower5min_recovery_gen_array
            .append_option({
                row.lower5min_recovery_gen
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .raise5min_recovery_gen_array
            .append_option({
                row.raise5min_recovery_gen
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lowerreg_recovery_gen_array
            .append_option({
                row.lowerreg_recovery_gen
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .raisereg_recovery_gen_array
            .append_option({
                row.raisereg_recovery_gen
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .raise1sec_recovery_array
            .append_option({
                row.raise1sec_recovery
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lower1sec_recovery_array
            .append_option({
                row.lower1sec_recovery
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .raise1sec_recovery_gen_array
            .append_option({
                row.raise1sec_recovery_gen
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lower1sec_recovery_gen_array
            .append_option({
                row.lower1sec_recovery_gen
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6sec_recovery_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6sec_recovery_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60sec_recovery_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60sec_recovery_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5min_recovery_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5min_recovery_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerreg_recovery_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raisereg_recovery_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower6sec_recovery_gen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise6sec_recovery_gen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower60sec_recovery_gen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise60sec_recovery_gen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower5min_recovery_gen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise5min_recovery_gen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lowerreg_recovery_gen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raisereg_recovery_gen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise1sec_recovery_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower1sec_recovery_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.raise1sec_recovery_gen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lower1sec_recovery_gen_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsFcasRecovery7Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    lower6sec_recovery_array: arrow::array::builder::Decimal128Builder,
    raise6sec_recovery_array: arrow::array::builder::Decimal128Builder,
    lower60sec_recovery_array: arrow::array::builder::Decimal128Builder,
    raise60sec_recovery_array: arrow::array::builder::Decimal128Builder,
    lower5min_recovery_array: arrow::array::builder::Decimal128Builder,
    raise5min_recovery_array: arrow::array::builder::Decimal128Builder,
    lowerreg_recovery_array: arrow::array::builder::Decimal128Builder,
    raisereg_recovery_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    lower6sec_recovery_gen_array: arrow::array::builder::Decimal128Builder,
    raise6sec_recovery_gen_array: arrow::array::builder::Decimal128Builder,
    lower60sec_recovery_gen_array: arrow::array::builder::Decimal128Builder,
    raise60sec_recovery_gen_array: arrow::array::builder::Decimal128Builder,
    lower5min_recovery_gen_array: arrow::array::builder::Decimal128Builder,
    raise5min_recovery_gen_array: arrow::array::builder::Decimal128Builder,
    lowerreg_recovery_gen_array: arrow::array::builder::Decimal128Builder,
    raisereg_recovery_gen_array: arrow::array::builder::Decimal128Builder,
    raise1sec_recovery_array: arrow::array::builder::Decimal128Builder,
    lower1sec_recovery_array: arrow::array::builder::Decimal128Builder,
    raise1sec_recovery_gen_array: arrow::array::builder::Decimal128Builder,
    lower1sec_recovery_gen_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsSetFcasRegulationTrk2;
pub struct SettlementsSetFcasRegulationTrk2Mapping([usize; 11]);
/// # Summary
///
/// ## SET_FCAS_REGULATION_TRK
///  _SET_FCAS_REGULATION_TRK shows FCAS Regulation Service Constraint tracking for Regional FCAS Regulation recovery_
///
/// * Data Set Name: Settlements
/// * File Name: Set Fcas Regulation Trk
/// * Data Version: 2
///
/// # Description
///  SET_FCAS_REGULATION_TRK contains public data and is available to all participants. Volume Approximately 350,000 per week.
///
///
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * INTERVAL_DATETIME
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsSetFcasRegulationTrk2Row<'data> {
    /// Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No
    pub versionno: rust_decimal::Decimal,
    /// Dispatch Interval Date Time
    pub interval_datetime: chrono::NaiveDateTime,
    /// Generic Constraint ID
    pub constraintid: core::ops::Range<usize>,
    /// Constraint Market Participant Factor
    pub cmpf: Option<rust_decimal::Decimal>,
    /// Constraint Residual Market Participant Factor
    pub crmpf: Option<rust_decimal::Decimal>,
    /// Recovery factor for CMPF based recovery
    pub recovery_factor_cmpf: Option<rust_decimal::Decimal>,
    /// Recovery factor for CRMPF based recovery
    pub recovery_factor_crmpf: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag to indication that substitute demand was used to recover this requirement
    pub usesubstitutedemand: Option<rust_decimal::Decimal>,
    /// the aggregate customer demand value used to recover the cost of this requirement
    pub requirementdemand: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsSetFcasRegulationTrk2Row<'data> {
    pub fn constraintid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.constraintid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsSetFcasRegulationTrk2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "SET_FCAS_REGULATION_TRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsSetFcasRegulationTrk2Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "INTERVAL_DATETIME",
        "CONSTRAINTID",
        "CMPF",
        "CRMPF",
        "RECOVERY_FACTOR_CMPF",
        "RECOVERY_FACTOR_CRMPF",
        "LASTCHANGED",
        "USESUBSTITUTEDEMAND",
        "REQUIREMENTDEMAND",
    ];
    type Row<'row> = SettlementsSetFcasRegulationTrk2Row<'row>;
    type FieldMapping = SettlementsSetFcasRegulationTrk2Mapping;
    type PrimaryKey = SettlementsSetFcasRegulationTrk2PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsSetFcasRegulationTrk2Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            interval_datetime: row
                .get_custom_parsed_at_idx(
                    "interval_datetime",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            constraintid: row.get_range("constraintid", field_mapping.0[3])?,
            cmpf: row
                .get_opt_custom_parsed_at_idx(
                    "cmpf",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            crmpf: row
                .get_opt_custom_parsed_at_idx(
                    "crmpf",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            recovery_factor_cmpf: row
                .get_opt_custom_parsed_at_idx(
                    "recovery_factor_cmpf",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            recovery_factor_crmpf: row
                .get_opt_custom_parsed_at_idx(
                    "recovery_factor_crmpf",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
                )?,
            usesubstitutedemand: row
                .get_opt_custom_parsed_at_idx(
                    "usesubstitutedemand",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            requirementdemand: row
                .get_opt_custom_parsed_at_idx(
                    "requirementdemand",
                    field_mapping.0[10],
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
        Ok(SettlementsSetFcasRegulationTrk2Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let interval_datetime = row
            .get_custom_parsed_at_idx(
                "interval_datetime",
                6,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(interval_datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(interval_datetime).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsSetFcasRegulationTrk2PrimaryKey {
        SettlementsSetFcasRegulationTrk2PrimaryKey {
            constraintid: row.constraintid().to_string(),
            interval_datetime: row.interval_datetime,
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.interval_datetime).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.interval_datetime).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_set_fcas_regulation_trk_v2_{}_{}", Self::partition_suffix(& row)
            .year, Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsSetFcasRegulationTrk2Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            interval_datetime: row.interval_datetime.clone(),
            constraintid: row.constraintid.clone(),
            cmpf: row.cmpf.clone(),
            crmpf: row.crmpf.clone(),
            recovery_factor_cmpf: row.recovery_factor_cmpf.clone(),
            recovery_factor_crmpf: row.recovery_factor_crmpf.clone(),
            lastchanged: row.lastchanged.clone(),
            usesubstitutedemand: row.usesubstitutedemand.clone(),
            requirementdemand: row.requirementdemand.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsSetFcasRegulationTrk2PrimaryKey {
    pub constraintid: alloc::string::String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsSetFcasRegulationTrk2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsSetFcasRegulationTrk2Row<'data> {
    type Row<'other> = SettlementsSetFcasRegulationTrk2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid() == row.constraintid()
            && self.interval_datetime == row.interval_datetime
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey
for SettlementsSetFcasRegulationTrk2Row<'data> {
    type PrimaryKey = SettlementsSetFcasRegulationTrk2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid() == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsSetFcasRegulationTrk2PrimaryKey {
    type Row<'other> = SettlementsSetFcasRegulationTrk2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.constraintid == row.constraintid()
            && self.interval_datetime == row.interval_datetime
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsSetFcasRegulationTrk2PrimaryKey {
    type PrimaryKey = SettlementsSetFcasRegulationTrk2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsSetFcasRegulationTrk2 {
    type Builder = SettlementsSetFcasRegulationTrk2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
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
                    "cmpf",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "crmpf",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recovery_factor_cmpf",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recovery_factor_crmpf",
                    arrow::datatypes::DataType::Decimal128(18, 8),
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
                    "usesubstitutedemand",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "requirementdemand",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsSetFcasRegulationTrk2Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            constraintid_array: arrow::array::builder::StringBuilder::new(),
            cmpf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            crmpf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            recovery_factor_cmpf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            recovery_factor_crmpf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            usesubstitutedemand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            requirementdemand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .interval_datetime_array
            .append_value(row.interval_datetime.timestamp_millis());
        builder.constraintid_array.append_value(row.constraintid());
        builder
            .cmpf_array
            .append_option({
                row.cmpf
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .crmpf_array
            .append_option({
                row.crmpf
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .recovery_factor_cmpf_array
            .append_option({
                row.recovery_factor_cmpf
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .recovery_factor_crmpf_array
            .append_option({
                row.recovery_factor_crmpf
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .usesubstitutedemand_array
            .append_option({
                row.usesubstitutedemand
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .requirementdemand_array
            .append_option({
                row.requirementdemand
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.interval_datetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.cmpf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.crmpf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.recovery_factor_cmpf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.recovery_factor_crmpf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.usesubstitutedemand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.requirementdemand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsSetFcasRegulationTrk2Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    interval_datetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    constraintid_array: arrow::array::builder::StringBuilder,
    cmpf_array: arrow::array::builder::Decimal128Builder,
    crmpf_array: arrow::array::builder::Decimal128Builder,
    recovery_factor_cmpf_array: arrow::array::builder::Decimal128Builder,
    recovery_factor_crmpf_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    usesubstitutedemand_array: arrow::array::builder::Decimal128Builder,
    requirementdemand_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsNmasRecovery2;
pub struct SettlementsNmasRecovery2Mapping([usize; 18]);
/// # Summary
///
/// ## SET_NMAS_RECOVERY
///  _SET_NMAS_RECOVERY sets out the NSCAS recovery data for payments other than testing._
///
/// * Data Set Name: Settlements
/// * File Name: Nmas Recovery
/// * Data Version: 2
///
///
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * PARTICIPANTID
/// * PAYMENTTYPE
/// * PERIODID
/// * REGIONID
/// * SERVICE
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsNmasRecovery2Row<'data> {
    /// Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// The Participant from whom the amount is recovered
    pub participantid: core::ops::Range<usize>,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>- RESTART
    pub service: core::ops::Range<usize>,
    /// The NMAS Contract Id
    pub contractid: core::ops::Range<usize>,
    /// The type of payment being recovered. Valid values are:<br>- AVAILABILITY<br>- ENABLEMENT<br>- COMPENSATION<br>
    pub paymenttype: core::ops::Range<usize>,
    /// The region from where the amount is recovered
    pub regionid: core::ops::Range<usize>,
    /// The Benefitting Factor for the RegionId
    pub rbf: Option<rust_decimal::Decimal>,
    /// The total Payment Amount to recover from all  benefitting regions
    pub payment_amount: Option<rust_decimal::Decimal>,
    /// The Participant energy in MWh for the period
    pub participant_energy: Option<rust_decimal::Decimal>,
    /// The RegionId energy in MWh for the period
    pub region_energy: Option<rust_decimal::Decimal>,
    /// The Total recovery amount for the period for the PARTICIPANTID and REGIONID
    pub recovery_amount: Option<rust_decimal::Decimal>,
    /// The Last Updated date and time
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Participant Generator Energy in the benefitting region
    pub participant_generation: Option<rust_decimal::Decimal>,
    /// The generator energy in the benefitting region
    pub region_generation: Option<rust_decimal::Decimal>,
    /// The recovery amount allocated to customers
    pub recovery_amount_customer: Option<rust_decimal::Decimal>,
    /// The recovery amount allocated to generators
    pub recovery_amount_generator: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsNmasRecovery2Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn service(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.service.clone())
    }
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
    pub fn paymenttype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.paymenttype.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsNmasRecovery2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "NMAS_RECOVERY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsNmasRecovery2Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PERIODID",
        "PARTICIPANTID",
        "SERVICE",
        "CONTRACTID",
        "PAYMENTTYPE",
        "REGIONID",
        "RBF",
        "PAYMENT_AMOUNT",
        "PARTICIPANT_ENERGY",
        "REGION_ENERGY",
        "RECOVERY_AMOUNT",
        "LASTCHANGED",
        "PARTICIPANT_GENERATION",
        "REGION_GENERATION",
        "RECOVERY_AMOUNT_CUSTOMER",
        "RECOVERY_AMOUNT_GENERATOR",
    ];
    type Row<'row> = SettlementsNmasRecovery2Row<'row>;
    type FieldMapping = SettlementsNmasRecovery2Mapping;
    type PrimaryKey = SettlementsNmasRecovery2PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsNmasRecovery2Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[3])?,
            service: row.get_range("service", field_mapping.0[4])?,
            contractid: row.get_range("contractid", field_mapping.0[5])?,
            paymenttype: row.get_range("paymenttype", field_mapping.0[6])?,
            regionid: row.get_range("regionid", field_mapping.0[7])?,
            rbf: row
                .get_opt_custom_parsed_at_idx(
                    "rbf",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            payment_amount: row
                .get_opt_custom_parsed_at_idx(
                    "payment_amount",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participant_energy: row
                .get_opt_custom_parsed_at_idx(
                    "participant_energy",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            region_energy: row
                .get_opt_custom_parsed_at_idx(
                    "region_energy",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            recovery_amount: row
                .get_opt_custom_parsed_at_idx(
                    "recovery_amount",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[13],
                    mmsdm_core::mms_datetime::parse,
                )?,
            participant_generation: row
                .get_opt_custom_parsed_at_idx(
                    "participant_generation",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            region_generation: row
                .get_opt_custom_parsed_at_idx(
                    "region_generation",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            recovery_amount_customer: row
                .get_opt_custom_parsed_at_idx(
                    "recovery_amount_customer",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            recovery_amount_generator: row
                .get_opt_custom_parsed_at_idx(
                    "recovery_amount_generator",
                    field_mapping.0[17],
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
        Ok(SettlementsNmasRecovery2Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsNmasRecovery2PrimaryKey {
        SettlementsNmasRecovery2PrimaryKey {
            contractid: row.contractid().to_string(),
            participantid: row.participantid().to_string(),
            paymenttype: row.paymenttype().to_string(),
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            service: row.service().to_string(),
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_nmas_recovery_v2_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsNmasRecovery2Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            periodid: row.periodid.clone(),
            participantid: row.participantid.clone(),
            service: row.service.clone(),
            contractid: row.contractid.clone(),
            paymenttype: row.paymenttype.clone(),
            regionid: row.regionid.clone(),
            rbf: row.rbf.clone(),
            payment_amount: row.payment_amount.clone(),
            participant_energy: row.participant_energy.clone(),
            region_energy: row.region_energy.clone(),
            recovery_amount: row.recovery_amount.clone(),
            lastchanged: row.lastchanged.clone(),
            participant_generation: row.participant_generation.clone(),
            region_generation: row.region_generation.clone(),
            recovery_amount_customer: row.recovery_amount_customer.clone(),
            recovery_amount_generator: row.recovery_amount_generator.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsNmasRecovery2PrimaryKey {
    pub contractid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub paymenttype: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub service: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsNmasRecovery2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsNmasRecovery2Row<'data> {
    type Row<'other> = SettlementsNmasRecovery2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid()
            && self.participantid() == row.participantid()
            && self.paymenttype() == row.paymenttype() && self.periodid == row.periodid
            && self.regionid() == row.regionid() && self.service() == row.service()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsNmasRecovery2Row<'data> {
    type PrimaryKey = SettlementsNmasRecovery2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.participantid() == key.participantid
            && self.paymenttype() == key.paymenttype && self.periodid == key.periodid
            && self.regionid() == key.regionid && self.service() == key.service
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsNmasRecovery2PrimaryKey {
    type Row<'other> = SettlementsNmasRecovery2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.participantid == row.participantid()
            && self.paymenttype == row.paymenttype() && self.periodid == row.periodid
            && self.regionid == row.regionid() && self.service == row.service()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsNmasRecovery2PrimaryKey {
    type PrimaryKey = SettlementsNmasRecovery2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.participantid == key.participantid
            && self.paymenttype == key.paymenttype && self.periodid == key.periodid
            && self.regionid == key.regionid && self.service == key.service
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsNmasRecovery2 {
    type Builder = SettlementsNmasRecovery2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
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
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "service",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "paymenttype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "rbf",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "payment_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "participant_energy",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "region_energy",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recovery_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
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
                    "participant_generation",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "region_generation",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recovery_amount_customer",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recovery_amount_generator",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsNmasRecovery2Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            service_array: arrow::array::builder::StringBuilder::new(),
            contractid_array: arrow::array::builder::StringBuilder::new(),
            paymenttype_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            rbf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            payment_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            participant_energy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            region_energy_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            recovery_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            participant_generation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            region_generation_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            recovery_amount_customer_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            recovery_amount_generator_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
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
        builder.participantid_array.append_value(row.participantid());
        builder.service_array.append_value(row.service());
        builder.contractid_array.append_value(row.contractid());
        builder.paymenttype_array.append_value(row.paymenttype());
        builder.regionid_array.append_value(row.regionid());
        builder
            .rbf_array
            .append_option({
                row.rbf
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .payment_amount_array
            .append_option({
                row.payment_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .participant_energy_array
            .append_option({
                row.participant_energy
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .region_energy_array
            .append_option({
                row.region_energy
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .recovery_amount_array
            .append_option({
                row.recovery_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .participant_generation_array
            .append_option({
                row.participant_generation
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .region_generation_array
            .append_option({
                row.region_generation
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .recovery_amount_customer_array
            .append_option({
                row.recovery_amount_customer
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .recovery_amount_generator_array
            .append_option({
                row.recovery_amount_generator
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.service_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.paymenttype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rbf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.payment_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participant_energy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.region_energy_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.recovery_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participant_generation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.region_generation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.recovery_amount_customer_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.recovery_amount_generator_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsNmasRecovery2Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    service_array: arrow::array::builder::StringBuilder,
    contractid_array: arrow::array::builder::StringBuilder,
    paymenttype_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    rbf_array: arrow::array::builder::Decimal128Builder,
    payment_amount_array: arrow::array::builder::Decimal128Builder,
    participant_energy_array: arrow::array::builder::Decimal128Builder,
    region_energy_array: arrow::array::builder::Decimal128Builder,
    recovery_amount_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    participant_generation_array: arrow::array::builder::Decimal128Builder,
    region_generation_array: arrow::array::builder::Decimal128Builder,
    recovery_amount_customer_array: arrow::array::builder::Decimal128Builder,
    recovery_amount_generator_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsNmasRecoveryRbf1;
pub struct SettlementsNmasRecoveryRbf1Mapping([usize; 11]);
/// # Summary
///
/// ## SET_NMAS_RECOVERY_RBF
///  _SET_NMAS_RECOVERY_RBF publishes the RBF for NSCAS non testing payments on a half hourly basis._
///
/// * Data Set Name: Settlements
/// * File Name: Nmas Recovery Rbf
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * PAYMENTTYPE
/// * PERIODID
/// * REGIONID
/// * SERVICE
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsNmasRecoveryRbf1Row<'data> {
    /// Settlement Date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>
    pub service: core::ops::Range<usize>,
    /// The NMAS Contract Id
    pub contractid: core::ops::Range<usize>,
    /// The type of payment being recovered. Valid values are:<br>- AVAILABILITY<br>- ENABLEMENT<br>- COMPENSATION<br>
    pub paymenttype: core::ops::Range<usize>,
    /// The region from where the amount is recovered
    pub regionid: core::ops::Range<usize>,
    /// The Benefitting Factor for the RegionId
    pub rbf: Option<rust_decimal::Decimal>,
    /// The total Payment Amount to recover from all benefitting regions
    pub payment_amount: Option<rust_decimal::Decimal>,
    /// The Total recovery amount for the period for the REGIONID
    pub recovery_amount: Option<rust_decimal::Decimal>,
    /// The Last Updated date and time
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsNmasRecoveryRbf1Row<'data> {
    pub fn service(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.service.clone())
    }
    pub fn contractid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.contractid.clone())
    }
    pub fn paymenttype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.paymenttype.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsNmasRecoveryRbf1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "NMAS_RECOVERY_RBF";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsNmasRecoveryRbf1Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PERIODID",
        "SERVICE",
        "CONTRACTID",
        "PAYMENTTYPE",
        "REGIONID",
        "RBF",
        "PAYMENT_AMOUNT",
        "RECOVERY_AMOUNT",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementsNmasRecoveryRbf1Row<'row>;
    type FieldMapping = SettlementsNmasRecoveryRbf1Mapping;
    type PrimaryKey = SettlementsNmasRecoveryRbf1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsNmasRecoveryRbf1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            service: row.get_range("service", field_mapping.0[3])?,
            contractid: row.get_range("contractid", field_mapping.0[4])?,
            paymenttype: row.get_range("paymenttype", field_mapping.0[5])?,
            regionid: row.get_range("regionid", field_mapping.0[6])?,
            rbf: row
                .get_opt_custom_parsed_at_idx(
                    "rbf",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            payment_amount: row
                .get_opt_custom_parsed_at_idx(
                    "payment_amount",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            recovery_amount: row
                .get_opt_custom_parsed_at_idx(
                    "recovery_amount",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
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
        Ok(SettlementsNmasRecoveryRbf1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsNmasRecoveryRbf1PrimaryKey {
        SettlementsNmasRecoveryRbf1PrimaryKey {
            contractid: row.contractid().to_string(),
            paymenttype: row.paymenttype().to_string(),
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            service: row.service().to_string(),
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_nmas_recovery_rbf_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsNmasRecoveryRbf1Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            periodid: row.periodid.clone(),
            service: row.service.clone(),
            contractid: row.contractid.clone(),
            paymenttype: row.paymenttype.clone(),
            regionid: row.regionid.clone(),
            rbf: row.rbf.clone(),
            payment_amount: row.payment_amount.clone(),
            recovery_amount: row.recovery_amount.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsNmasRecoveryRbf1PrimaryKey {
    pub contractid: alloc::string::String,
    pub paymenttype: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub service: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsNmasRecoveryRbf1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsNmasRecoveryRbf1Row<'data> {
    type Row<'other> = SettlementsNmasRecoveryRbf1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid() == row.contractid() && self.paymenttype() == row.paymenttype()
            && self.periodid == row.periodid && self.regionid() == row.regionid()
            && self.service() == row.service()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsNmasRecoveryRbf1Row<'data> {
    type PrimaryKey = SettlementsNmasRecoveryRbf1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid() == key.contractid && self.paymenttype() == key.paymenttype
            && self.periodid == key.periodid && self.regionid() == key.regionid
            && self.service() == key.service && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsNmasRecoveryRbf1PrimaryKey {
    type Row<'other> = SettlementsNmasRecoveryRbf1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.contractid == row.contractid() && self.paymenttype == row.paymenttype()
            && self.periodid == row.periodid && self.regionid == row.regionid()
            && self.service == row.service() && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsNmasRecoveryRbf1PrimaryKey {
    type PrimaryKey = SettlementsNmasRecoveryRbf1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.paymenttype == key.paymenttype
            && self.periodid == key.periodid && self.regionid == key.regionid
            && self.service == key.service && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsNmasRecoveryRbf1 {
    type Builder = SettlementsNmasRecoveryRbf1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
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
                    "service",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "contractid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "paymenttype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "rbf",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "payment_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recovery_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
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
        SettlementsNmasRecoveryRbf1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            service_array: arrow::array::builder::StringBuilder::new(),
            contractid_array: arrow::array::builder::StringBuilder::new(),
            paymenttype_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            rbf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            payment_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            recovery_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
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
        builder.service_array.append_value(row.service());
        builder.contractid_array.append_value(row.contractid());
        builder.paymenttype_array.append_value(row.paymenttype());
        builder.regionid_array.append_value(row.regionid());
        builder
            .rbf_array
            .append_option({
                row.rbf
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .payment_amount_array
            .append_option({
                row.payment_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .recovery_amount_array
            .append_option({
                row.recovery_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.service_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.contractid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.paymenttype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rbf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.payment_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.recovery_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsNmasRecoveryRbf1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    service_array: arrow::array::builder::StringBuilder,
    contractid_array: arrow::array::builder::StringBuilder,
    paymenttype_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    rbf_array: arrow::array::builder::Decimal128Builder,
    payment_amount_array: arrow::array::builder::Decimal128Builder,
    recovery_amount_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementsRecoveryEnergy1;
pub struct SettlementsRecoveryEnergy1Mapping([usize; 15]);
/// # Summary
///
/// ## SET_RECOVERY_ENERGY
///  _Settlements substitution recovery energy used_
///
/// * Data Set Name: Settlements
/// * File Name: Recovery Energy
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsRecoveryEnergy1Row<'data> {
    /// Settlement date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// Unique identifier for the participant
    pub participantid: core::ops::Range<usize>,
    /// Unique Identifier for the Region to which the TNI belongs on this settlement date
    pub regionid: core::ops::Range<usize>,
    /// Trading interval identifier, with Period 1 being the first TI for the calendar day, i.e interval ending 00:05 for 5MS or 00:30 for 30MS.
    pub periodid: rust_decimal::Decimal,
    /// Actual Customer Demand
    pub customerenergyactual: Option<rust_decimal::Decimal>,
    /// Actual Customer Demand excluding TNIs that have a causer pays MPF
    pub customerenergympfexactual: Option<rust_decimal::Decimal>,
    /// Substitute Customer Demand
    pub customerenergysubstitute: Option<rust_decimal::Decimal>,
    /// Substitute Customer Demand excluding TNIs that have a causer pays MPF
    pub customerenergympfexsubstitute: Option<rust_decimal::Decimal>,
    /// Actual Generator Output
    pub generatorenergyactual: Option<rust_decimal::Decimal>,
    /// Region Total of Actual Customer Demand
    pub regioncustenergyactual: Option<rust_decimal::Decimal>,
    /// Region Total of Actual Customer Demand excluding TNIs that have a causer pays MPF.
    pub regioncustenergympfexactual: Option<rust_decimal::Decimal>,
    /// Region Total of Substitute Customer Demand
    pub regioncustenergysubst: Option<rust_decimal::Decimal>,
    /// Region total of Substitute Customer Demand excluding TNIs that have a causer pays MPF.
    pub regioncustenergympfexsubst: Option<rust_decimal::Decimal>,
    /// Region Total of Actual Generator Output
    pub regiongenenergyactual: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsRecoveryEnergy1Row<'data> {
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsRecoveryEnergy1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "RECOVERY_ENERGY";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsRecoveryEnergy1Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "SETTLEMENTRUNNO",
        "PARTICIPANTID",
        "REGIONID",
        "PERIODID",
        "CUSTOMERENERGYACTUAL",
        "CUSTOMERENERGYMPFEXACTUAL",
        "CUSTOMERENERGYSUBSTITUTE",
        "CUSTOMERENERGYMPFEXSUBSTITUTE",
        "GENERATORENERGYACTUAL",
        "REGIONCUSTENERGYACTUAL",
        "REGIONCUSTENERGYMPFEXACTUAL",
        "REGIONCUSTENERGYSUBST",
        "REGIONCUSTENERGYMPFEXSUBST",
        "REGIONGENENERGYACTUAL",
    ];
    type Row<'row> = SettlementsRecoveryEnergy1Row<'row>;
    type FieldMapping = SettlementsRecoveryEnergy1Mapping;
    type PrimaryKey = SettlementsRecoveryEnergy1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsRecoveryEnergy1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            settlementrunno: row
                .get_custom_parsed_at_idx(
                    "settlementrunno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            participantid: row.get_range("participantid", field_mapping.0[2])?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            customerenergyactual: row
                .get_opt_custom_parsed_at_idx(
                    "customerenergyactual",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            customerenergympfexactual: row
                .get_opt_custom_parsed_at_idx(
                    "customerenergympfexactual",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            customerenergysubstitute: row
                .get_opt_custom_parsed_at_idx(
                    "customerenergysubstitute",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            customerenergympfexsubstitute: row
                .get_opt_custom_parsed_at_idx(
                    "customerenergympfexsubstitute",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            generatorenergyactual: row
                .get_opt_custom_parsed_at_idx(
                    "generatorenergyactual",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regioncustenergyactual: row
                .get_opt_custom_parsed_at_idx(
                    "regioncustenergyactual",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regioncustenergympfexactual: row
                .get_opt_custom_parsed_at_idx(
                    "regioncustenergympfexactual",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regioncustenergysubst: row
                .get_opt_custom_parsed_at_idx(
                    "regioncustenergysubst",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regioncustenergympfexsubst: row
                .get_opt_custom_parsed_at_idx(
                    "regioncustenergympfexsubst",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regiongenenergyactual: row
                .get_opt_custom_parsed_at_idx(
                    "regiongenenergyactual",
                    field_mapping.0[14],
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
        Ok(SettlementsRecoveryEnergy1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsRecoveryEnergy1PrimaryKey {
        SettlementsRecoveryEnergy1PrimaryKey {
            participantid: row.participantid().to_string(),
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            settlementdate: row.settlementdate,
            settlementrunno: row.settlementrunno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_recovery_energy_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsRecoveryEnergy1Row {
            settlementdate: row.settlementdate.clone(),
            settlementrunno: row.settlementrunno.clone(),
            participantid: row.participantid.clone(),
            regionid: row.regionid.clone(),
            periodid: row.periodid.clone(),
            customerenergyactual: row.customerenergyactual.clone(),
            customerenergympfexactual: row.customerenergympfexactual.clone(),
            customerenergysubstitute: row.customerenergysubstitute.clone(),
            customerenergympfexsubstitute: row.customerenergympfexsubstitute.clone(),
            generatorenergyactual: row.generatorenergyactual.clone(),
            regioncustenergyactual: row.regioncustenergyactual.clone(),
            regioncustenergympfexactual: row.regioncustenergympfexactual.clone(),
            regioncustenergysubst: row.regioncustenergysubst.clone(),
            regioncustenergympfexsubst: row.regioncustenergympfexsubst.clone(),
            regiongenenergyactual: row.regiongenenergyactual.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsRecoveryEnergy1PrimaryKey {
    pub participantid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsRecoveryEnergy1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsRecoveryEnergy1Row<'data> {
    type Row<'other> = SettlementsRecoveryEnergy1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid() == row.participantid() && self.periodid == row.periodid
            && self.regionid() == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsRecoveryEnergy1Row<'data> {
    type PrimaryKey = SettlementsRecoveryEnergy1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid() == key.participantid && self.periodid == key.periodid
            && self.regionid() == key.regionid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsRecoveryEnergy1PrimaryKey {
    type Row<'other> = SettlementsRecoveryEnergy1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid == row.participantid() && self.periodid == row.periodid
            && self.regionid == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsRecoveryEnergy1PrimaryKey {
    type PrimaryKey = SettlementsRecoveryEnergy1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid && self.periodid == key.periodid
            && self.regionid == key.regionid && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsRecoveryEnergy1 {
    type Builder = SettlementsRecoveryEnergy1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementrunno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
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
                    "customerenergyactual",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "customerenergympfexactual",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "customerenergysubstitute",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "customerenergympfexsubstitute",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "generatorenergyactual",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regioncustenergyactual",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regioncustenergympfexactual",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regioncustenergysubst",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regioncustenergympfexsubst",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regiongenenergyactual",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsRecoveryEnergy1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            settlementrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            customerenergyactual_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            customerenergympfexactual_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            customerenergysubstitute_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            customerenergympfexsubstitute_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            generatorenergyactual_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            regioncustenergyactual_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            regioncustenergympfexactual_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            regioncustenergysubst_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            regioncustenergympfexsubst_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            regiongenenergyactual_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .settlementrunno_array
            .append_value({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
        builder.participantid_array.append_value(row.participantid());
        builder.regionid_array.append_value(row.regionid());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .customerenergyactual_array
            .append_option({
                row.customerenergyactual
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .customerenergympfexactual_array
            .append_option({
                row.customerenergympfexactual
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .customerenergysubstitute_array
            .append_option({
                row.customerenergysubstitute
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .customerenergympfexsubstitute_array
            .append_option({
                row.customerenergympfexsubstitute
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .generatorenergyactual_array
            .append_option({
                row.generatorenergyactual
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .regioncustenergyactual_array
            .append_option({
                row.regioncustenergyactual
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .regioncustenergympfexactual_array
            .append_option({
                row.regioncustenergympfexactual
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .regioncustenergysubst_array
            .append_option({
                row.regioncustenergysubst
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .regioncustenergympfexsubst_array
            .append_option({
                row.regioncustenergympfexsubst
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .regiongenenergyactual_array
            .append_option({
                row.regiongenenergyactual
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.customerenergyactual_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.customerenergympfexactual_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.customerenergysubstitute_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.customerenergympfexsubstitute_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.generatorenergyactual_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regioncustenergyactual_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.regioncustenergympfexactual_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regioncustenergysubst_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.regioncustenergympfexsubst_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regiongenenergyactual_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsRecoveryEnergy1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    settlementrunno_array: arrow::array::builder::Decimal128Builder,
    participantid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    customerenergyactual_array: arrow::array::builder::Decimal128Builder,
    customerenergympfexactual_array: arrow::array::builder::Decimal128Builder,
    customerenergysubstitute_array: arrow::array::builder::Decimal128Builder,
    customerenergympfexsubstitute_array: arrow::array::builder::Decimal128Builder,
    generatorenergyactual_array: arrow::array::builder::Decimal128Builder,
    regioncustenergyactual_array: arrow::array::builder::Decimal128Builder,
    regioncustenergympfexactual_array: arrow::array::builder::Decimal128Builder,
    regioncustenergysubst_array: arrow::array::builder::Decimal128Builder,
    regioncustenergympfexsubst_array: arrow::array::builder::Decimal128Builder,
    regiongenenergyactual_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsRunParameter5;
pub struct SettlementsRunParameter5Mapping([usize; 5]);
/// # Summary
///
/// ## SET_RUN_PARAMETER
///  _SET_RUN_PARAMETER shows the input parameters and value associated with each settlement run (e.g. Residual System Load Causer Pays Factor)._
///
/// * Data Set Name: Settlements
/// * File Name: Run Parameter
/// * Data Version: 5
///
/// # Description
///  Change History 19 August 2005 for 4.5.0: Changed index name again to have suffix of _LCX Note: primary key shows PK_ as prefix in Oracle SQL script, even though name of key has _PK as suffix - but cannot change since would not improve participant systems . &nbsp; 17 August 2005 for v4.5.0 Added tablespace (02) for recently added index, and gave index a better name
///
///
///
/// # Primary Key Columns
///
/// * PARAMETERID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsRunParameter5Row<'data> {
    /// Settlement Date (Calendar)
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run Number for this date
    pub versionno: i64,
    /// Parameter Identifier
    pub parameterid: core::ops::Range<usize>,
    /// Settlement Run Amount for the Constant Identifier
    pub numvalue: Option<rust_decimal::Decimal>,
    /// Last date the record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsRunParameter5Row<'data> {
    pub fn parameterid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.parameterid.clone())
    }
}
impl mmsdm_core::GetTable for SettlementsRunParameter5 {
    const VERSION: i32 = 5;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "RUN_PARAMETER";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsRunParameter5Mapping([
        4,
        5,
        6,
        7,
        8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "VERSIONNO",
        "PARAMETERID",
        "NUMVALUE",
        "LASTCHANGED",
    ];
    type Row<'row> = SettlementsRunParameter5Row<'row>;
    type FieldMapping = SettlementsRunParameter5Mapping;
    type PrimaryKey = SettlementsRunParameter5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsRunParameter5Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row.get_parsed_at_idx("versionno", field_mapping.0[1])?,
            parameterid: row.get_range("parameterid", field_mapping.0[2])?,
            numvalue: row
                .get_opt_custom_parsed_at_idx(
                    "numvalue",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
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
        Ok(SettlementsRunParameter5Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsRunParameter5PrimaryKey {
        SettlementsRunParameter5PrimaryKey {
            parameterid: row.parameterid().to_string(),
            settlementdate: row.settlementdate,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_run_parameter_v5_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsRunParameter5Row {
            settlementdate: row.settlementdate.clone(),
            versionno: row.versionno.clone(),
            parameterid: row.parameterid.clone(),
            numvalue: row.numvalue.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsRunParameter5PrimaryKey {
    pub parameterid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: i64,
}
impl mmsdm_core::PrimaryKey for SettlementsRunParameter5PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsRunParameter5Row<'data> {
    type Row<'other> = SettlementsRunParameter5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.parameterid() == row.parameterid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsRunParameter5Row<'data> {
    type PrimaryKey = SettlementsRunParameter5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.parameterid() == key.parameterid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsRunParameter5PrimaryKey {
    type Row<'other> = SettlementsRunParameter5Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.parameterid == row.parameterid()
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsRunParameter5PrimaryKey {
    type PrimaryKey = SettlementsRunParameter5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.parameterid == key.parameterid && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsRunParameter5 {
    type Builder = SettlementsRunParameter5Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "parameterid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "numvalue",
                    arrow::datatypes::DataType::Decimal128(18, 8),
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
        SettlementsRunParameter5Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Int64Builder::new(),
            parameterid_array: arrow::array::builder::StringBuilder::new(),
            numvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder.versionno_array.append_value(row.versionno);
        builder.parameterid_array.append_value(row.parameterid());
        builder
            .numvalue_array
            .append_option({
                row.numvalue
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.parameterid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.numvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsRunParameter5Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Int64Builder,
    parameterid_array: arrow::array::builder::StringBuilder,
    numvalue_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SettlementsSubstDemand1;
pub struct SettlementsSubstDemand1Mapping([usize; 6]);
/// # Summary
///
/// ## SET_SUBSTITUTE_DEMAND
///  _Settlements substitution demand for Zero Demand figures_
///
/// * Data Set Name: Settlements
/// * File Name: Subst Demand
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
/// * TNI
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsSubstDemand1Row<'data> {
    /// Settlement date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// Unique identifier for the connection point
    pub tni: core::ops::Range<usize>,
    /// Unique identifier for the participant
    pub participantid: core::ops::Range<usize>,
    /// Unique identifier for the region to which the TNI belongs to on this settlement date
    pub regionid: core::ops::Range<usize>,
    /// Substitute metered quantity for non-energy recovery in MWh for the TNI and participant in the trading interval. A negative value indicates net consumption and a positive value indicates net generation
    pub substitutedemand: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsSubstDemand1Row<'data> {
    pub fn tni(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.tni.clone())
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
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
impl mmsdm_core::GetTable for SettlementsSubstDemand1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "SUBST_DEMAND";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsSubstDemand1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "SETTLEMENTRUNNO",
        "TNI",
        "PARTICIPANTID",
        "REGIONID",
        "SUBSTITUTEDEMAND",
    ];
    type Row<'row> = SettlementsSubstDemand1Row<'row>;
    type FieldMapping = SettlementsSubstDemand1Mapping;
    type PrimaryKey = SettlementsSubstDemand1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsSubstDemand1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            settlementrunno: row
                .get_custom_parsed_at_idx(
                    "settlementrunno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            tni: row.get_range("tni", field_mapping.0[2])?,
            participantid: row.get_range("participantid", field_mapping.0[3])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[4])?,
            substitutedemand: row
                .get_opt_custom_parsed_at_idx(
                    "substitutedemand",
                    field_mapping.0[5],
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
        Ok(SettlementsSubstDemand1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsSubstDemand1PrimaryKey {
        SettlementsSubstDemand1PrimaryKey {
            participantid: row.participantid().to_string(),
            settlementdate: row.settlementdate,
            settlementrunno: row.settlementrunno,
            tni: row.tni().to_string(),
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_subst_demand_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsSubstDemand1Row {
            settlementdate: row.settlementdate.clone(),
            settlementrunno: row.settlementrunno.clone(),
            tni: row.tni.clone(),
            participantid: row.participantid.clone(),
            regionid: row.regionid.clone(),
            substitutedemand: row.substitutedemand.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsSubstDemand1PrimaryKey {
    pub participantid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
    pub tni: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for SettlementsSubstDemand1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsSubstDemand1Row<'data> {
    type Row<'other> = SettlementsSubstDemand1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid() == row.participantid()
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno && self.tni() == row.tni()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsSubstDemand1Row<'data> {
    type PrimaryKey = SettlementsSubstDemand1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid() == key.participantid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno && self.tni() == key.tni
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsSubstDemand1PrimaryKey {
    type Row<'other> = SettlementsSubstDemand1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.participantid == row.participantid()
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno && self.tni == row.tni()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsSubstDemand1PrimaryKey {
    type PrimaryKey = SettlementsSubstDemand1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno && self.tni == key.tni
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsSubstDemand1 {
    type Builder = SettlementsSubstDemand1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementrunno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "tni",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "participantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "substitutedemand",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsSubstDemand1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            settlementrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            tni_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            substitutedemand_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .settlementrunno_array
            .append_value({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
        builder.tni_array.append_value(row.tni());
        builder.participantid_array.append_value(row.participantid());
        builder.regionid_array.append_option(row.regionid());
        builder
            .substitutedemand_array
            .append_option({
                row.substitutedemand
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tni_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.substitutedemand_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsSubstDemand1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    settlementrunno_array: arrow::array::builder::Decimal128Builder,
    tni_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    substitutedemand_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsSubstRunVersion1;
pub struct SettlementsSubstRunVersion1Mapping([usize; 4]);
/// # Summary
///
/// ## SET_SUBST_RUN_VERSION
///  _Settlements substitution demand run version numbers_
///
/// * Data Set Name: Settlements
/// * File Name: Subst Run Version
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * REFERENCESETTLEMENTDATE
/// * REFERENCESETTLEMENTRUNNO
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsSubstRunVersion1Row<'data> {
    /// Settlement date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// The settlement date of a settlement run included in the reference period
    pub referencesettlementdate: chrono::NaiveDateTime,
    /// The settlement run number matching the settlement date for a settlement run included in the reference period
    pub referencesettlementrunno: rust_decimal::Decimal,
    backing_data: core::marker::PhantomData<&'data ()>,
}
impl<'data> SettlementsSubstRunVersion1Row<'data> {}
impl mmsdm_core::GetTable for SettlementsSubstRunVersion1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "SUBST_RUN_VERSION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsSubstRunVersion1Mapping([
        4,
        5,
        6,
        7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "SETTLEMENTRUNNO",
        "REFERENCESETTLEMENTDATE",
        "REFERENCESETTLEMENTRUNNO",
    ];
    type Row<'row> = SettlementsSubstRunVersion1Row<'row>;
    type FieldMapping = SettlementsSubstRunVersion1Mapping;
    type PrimaryKey = SettlementsSubstRunVersion1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsSubstRunVersion1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            settlementrunno: row
                .get_custom_parsed_at_idx(
                    "settlementrunno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            referencesettlementdate: row
                .get_custom_parsed_at_idx(
                    "referencesettlementdate",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            referencesettlementrunno: row
                .get_custom_parsed_at_idx(
                    "referencesettlementrunno",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
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
        Ok(SettlementsSubstRunVersion1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsSubstRunVersion1PrimaryKey {
        SettlementsSubstRunVersion1PrimaryKey {
            referencesettlementdate: row.referencesettlementdate,
            referencesettlementrunno: row.referencesettlementrunno,
            settlementdate: row.settlementdate,
            settlementrunno: row.settlementrunno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_subst_run_version_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsSubstRunVersion1Row {
            settlementdate: row.settlementdate.clone(),
            settlementrunno: row.settlementrunno.clone(),
            referencesettlementdate: row.referencesettlementdate.clone(),
            referencesettlementrunno: row.referencesettlementrunno.clone(),
            backing_data: core::marker::PhantomData,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsSubstRunVersion1PrimaryKey {
    pub referencesettlementdate: chrono::NaiveDateTime,
    pub referencesettlementrunno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsSubstRunVersion1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsSubstRunVersion1Row<'data> {
    type Row<'other> = SettlementsSubstRunVersion1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.referencesettlementdate == row.referencesettlementdate
            && self.referencesettlementrunno == row.referencesettlementrunno
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsSubstRunVersion1Row<'data> {
    type PrimaryKey = SettlementsSubstRunVersion1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.referencesettlementdate == key.referencesettlementdate
            && self.referencesettlementrunno == key.referencesettlementrunno
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsSubstRunVersion1PrimaryKey {
    type Row<'other> = SettlementsSubstRunVersion1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.referencesettlementdate == row.referencesettlementdate
            && self.referencesettlementrunno == row.referencesettlementrunno
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsSubstRunVersion1PrimaryKey {
    type PrimaryKey = SettlementsSubstRunVersion1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.referencesettlementdate == key.referencesettlementdate
            && self.referencesettlementrunno == key.referencesettlementrunno
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsSubstRunVersion1 {
    type Builder = SettlementsSubstRunVersion1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementrunno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "referencesettlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "referencesettlementrunno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsSubstRunVersion1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            settlementrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            referencesettlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            referencesettlementrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .settlementrunno_array
            .append_value({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .referencesettlementdate_array
            .append_value(row.referencesettlementdate.timestamp_millis());
        builder
            .referencesettlementrunno_array
            .append_value({
                let mut val = row.referencesettlementrunno;
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.referencesettlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.referencesettlementrunno_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsSubstRunVersion1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    settlementrunno_array: arrow::array::builder::Decimal128Builder,
    referencesettlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    referencesettlementrunno_array: arrow::array::builder::Decimal128Builder,
}
pub struct SettlementsWdrReconDetail1;
pub struct SettlementsWdrReconDetail1Mapping([usize; 21]);
/// # Summary
///
/// ## SET_WDR_RECON_DETAIL
///  _Settlements WDR reconciliation details_
///
/// * Data Set Name: Settlements
/// * File Name: Wdr Recon Detail
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * NMI
/// * PERIODID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsWdrReconDetail1Row<'data> {
    /// Settlement date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// Unique identifier for the meter to which the metering records applies
    pub nmi: core::ops::Range<usize>,
    /// Unique identifier for the transmission node to which this meter belongs on the settlement date
    pub tni: core::ops::Range<usize>,
    /// Unique identifier for the region to which the TNI belongs on the settlement date
    pub regionid: core::ops::Range<usize>,
    /// Unique identifier for the participant acting as the FRMP for this NMI on the settlement date
    pub frmp: core::ops::Range<usize>,
    /// Unique identifier for the participant acting as the DRSP for this NMI on the settlement date
    pub drsp: core::ops::Range<usize>,
    /// Trading interval identifier with Period 1 being the first TI for the calendar day, that is the interval ending 00:05
    pub periodid: rust_decimal::Decimal,
    /// WDR settlement quantity before any capping or flooring (MWh)
    pub wdrsq_uncapped: Option<rust_decimal::Decimal>,
    /// WDR settlement quantity after capping or flooring (MWh)
    pub wdrsq_capped: Option<rust_decimal::Decimal>,
    /// Maximum responsive component for the NMI (MW)
    pub mrc: Option<rust_decimal::Decimal>,
    /// Maximum responsive component settlement quantity for the NMI (MWh)
    pub mrcsq: Option<rust_decimal::Decimal>,
    /// WDR reimbursement rate for the region ($/MWh)
    pub wdrrr: Option<rust_decimal::Decimal>,
    /// Regional reference price for the region in the settlement interval ($/MWh)
    pub rrp: Option<rust_decimal::Decimal>,
    /// Transmission loss factor for the wholesale connection point associated with the NMI
    pub tlf: Option<rust_decimal::Decimal>,
    /// Metered quantity in MWh for the NMI trading interval. A negative value indicates net consumption and a positive value indicates net generation
    pub me_dlfadjusted: Option<rust_decimal::Decimal>,
    /// Baseline quantity in MWh for the NMI in the trading interval. A negative quantity indicates net consumption, while a positive value indicates net generation
    pub bq_dlfadjusted: Option<rust_decimal::Decimal>,
    /// A value of TRUE (indicated by 1) for this column indicates that financial settlement of WDR transactions for this NMI should not proceed for the settlement date and trading interval. Possible values are 1 and 0.
    pub isnoncompliant: Option<rust_decimal::Decimal>,
    /// Quality flag for the meter read. Where multiple datastreams exist against the NMI with different quality flags for each read, the lowest quality flag will be published against the NMI for the interval
    pub qualityflag: core::ops::Range<usize>,
    /// WDR transaction amount for this NMI in the settlement interval ($)
    pub transactionamount: Option<rust_decimal::Decimal>,
    /// A reference to the baseline run that produced the baseline quantity for this NMI and interval
    pub baselinecalculationid: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsWdrReconDetail1Row<'data> {
    pub fn nmi(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.nmi.clone())
    }
    pub fn tni(&self) -> Option<&str> {
        if self.tni.is_empty() {
            None
        } else {
            Some(core::ops::Index::index(self.backing_data.as_slice(), self.tni.clone()))
        }
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
    pub fn frmp(&self) -> Option<&str> {
        if self.frmp.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(self.backing_data.as_slice(), self.frmp.clone()),
            )
        }
    }
    pub fn drsp(&self) -> Option<&str> {
        if self.drsp.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(self.backing_data.as_slice(), self.drsp.clone()),
            )
        }
    }
    pub fn qualityflag(&self) -> Option<&str> {
        if self.qualityflag.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.qualityflag.clone(),
                ),
            )
        }
    }
    pub fn baselinecalculationid(&self) -> Option<&str> {
        if self.baselinecalculationid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.baselinecalculationid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for SettlementsWdrReconDetail1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "WDR_RECON_DETAIL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsWdrReconDetail1Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "SETTLEMENTRUNNO",
        "NMI",
        "TNI",
        "REGIONID",
        "FRMP",
        "DRSP",
        "PERIODID",
        "WDRSQ_UNCAPPED",
        "WDRSQ_CAPPED",
        "MRC",
        "MRCSQ",
        "WDRRR",
        "RRP",
        "TLF",
        "ME_DLFADJUSTED",
        "BQ_DLFADJUSTED",
        "ISNONCOMPLIANT",
        "QUALITYFLAG",
        "TRANSACTIONAMOUNT",
        "BASELINECALCULATIONID",
    ];
    type Row<'row> = SettlementsWdrReconDetail1Row<'row>;
    type FieldMapping = SettlementsWdrReconDetail1Mapping;
    type PrimaryKey = SettlementsWdrReconDetail1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsWdrReconDetail1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            settlementrunno: row
                .get_custom_parsed_at_idx(
                    "settlementrunno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            nmi: row.get_range("nmi", field_mapping.0[2])?,
            tni: row.get_opt_range("tni", field_mapping.0[3])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[4])?,
            frmp: row.get_opt_range("frmp", field_mapping.0[5])?,
            drsp: row.get_opt_range("drsp", field_mapping.0[6])?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            wdrsq_uncapped: row
                .get_opt_custom_parsed_at_idx(
                    "wdrsq_uncapped",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            wdrsq_capped: row
                .get_opt_custom_parsed_at_idx(
                    "wdrsq_capped",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mrc: row
                .get_opt_custom_parsed_at_idx(
                    "mrc",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            mrcsq: row
                .get_opt_custom_parsed_at_idx(
                    "mrcsq",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            wdrrr: row
                .get_opt_custom_parsed_at_idx(
                    "wdrrr",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            rrp: row
                .get_opt_custom_parsed_at_idx(
                    "rrp",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            tlf: row
                .get_opt_custom_parsed_at_idx(
                    "tlf",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            me_dlfadjusted: row
                .get_opt_custom_parsed_at_idx(
                    "me_dlfadjusted",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            bq_dlfadjusted: row
                .get_opt_custom_parsed_at_idx(
                    "bq_dlfadjusted",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            isnoncompliant: row
                .get_opt_custom_parsed_at_idx(
                    "isnoncompliant",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            qualityflag: row.get_opt_range("qualityflag", field_mapping.0[18])?,
            transactionamount: row
                .get_opt_custom_parsed_at_idx(
                    "transactionamount",
                    field_mapping.0[19],
                    mmsdm_core::mms_decimal::parse,
                )?,
            baselinecalculationid: row
                .get_opt_range("baselinecalculationid", field_mapping.0[20])?,
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
        Ok(SettlementsWdrReconDetail1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsWdrReconDetail1PrimaryKey {
        SettlementsWdrReconDetail1PrimaryKey {
            nmi: row.nmi().to_string(),
            periodid: row.periodid,
            settlementdate: row.settlementdate,
            settlementrunno: row.settlementrunno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_wdr_recon_detail_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsWdrReconDetail1Row {
            settlementdate: row.settlementdate.clone(),
            settlementrunno: row.settlementrunno.clone(),
            nmi: row.nmi.clone(),
            tni: row.tni.clone(),
            regionid: row.regionid.clone(),
            frmp: row.frmp.clone(),
            drsp: row.drsp.clone(),
            periodid: row.periodid.clone(),
            wdrsq_uncapped: row.wdrsq_uncapped.clone(),
            wdrsq_capped: row.wdrsq_capped.clone(),
            mrc: row.mrc.clone(),
            mrcsq: row.mrcsq.clone(),
            wdrrr: row.wdrrr.clone(),
            rrp: row.rrp.clone(),
            tlf: row.tlf.clone(),
            me_dlfadjusted: row.me_dlfadjusted.clone(),
            bq_dlfadjusted: row.bq_dlfadjusted.clone(),
            isnoncompliant: row.isnoncompliant.clone(),
            qualityflag: row.qualityflag.clone(),
            transactionamount: row.transactionamount.clone(),
            baselinecalculationid: row.baselinecalculationid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsWdrReconDetail1PrimaryKey {
    pub nmi: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsWdrReconDetail1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsWdrReconDetail1Row<'data> {
    type Row<'other> = SettlementsWdrReconDetail1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.nmi() == row.nmi() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsWdrReconDetail1Row<'data> {
    type PrimaryKey = SettlementsWdrReconDetail1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.nmi() == key.nmi && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsWdrReconDetail1PrimaryKey {
    type Row<'other> = SettlementsWdrReconDetail1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.nmi == row.nmi() && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsWdrReconDetail1PrimaryKey {
    type PrimaryKey = SettlementsWdrReconDetail1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.nmi == key.nmi && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsWdrReconDetail1 {
    type Builder = SettlementsWdrReconDetail1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementrunno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "nmi",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "tni",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "frmp",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "drsp",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "wdrsq_uncapped",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "wdrsq_capped",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mrc",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mrcsq",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "wdrrr",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "rrp",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "tlf",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "me_dlfadjusted",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "bq_dlfadjusted",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "isnoncompliant",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "qualityflag",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "transactionamount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "baselinecalculationid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsWdrReconDetail1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            settlementrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            nmi_array: arrow::array::builder::StringBuilder::new(),
            tni_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            frmp_array: arrow::array::builder::StringBuilder::new(),
            drsp_array: arrow::array::builder::StringBuilder::new(),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            wdrsq_uncapped_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            wdrsq_capped_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            mrc_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            mrcsq_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            wdrrr_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            rrp_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            tlf_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            me_dlfadjusted_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            bq_dlfadjusted_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            isnoncompliant_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            qualityflag_array: arrow::array::builder::StringBuilder::new(),
            transactionamount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            baselinecalculationid_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .settlementrunno_array
            .append_value({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
        builder.nmi_array.append_value(row.nmi());
        builder.tni_array.append_option(row.tni());
        builder.regionid_array.append_option(row.regionid());
        builder.frmp_array.append_option(row.frmp());
        builder.drsp_array.append_option(row.drsp());
        builder
            .periodid_array
            .append_value({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .wdrsq_uncapped_array
            .append_option({
                row.wdrsq_uncapped
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .wdrsq_capped_array
            .append_option({
                row.wdrsq_capped
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .mrc_array
            .append_option({
                row.mrc
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .mrcsq_array
            .append_option({
                row.mrcsq
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .wdrrr_array
            .append_option({
                row.wdrrr
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .rrp_array
            .append_option({
                row.rrp
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .tlf_array
            .append_option({
                row.tlf
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .me_dlfadjusted_array
            .append_option({
                row.me_dlfadjusted
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .bq_dlfadjusted_array
            .append_option({
                row.bq_dlfadjusted
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .isnoncompliant_array
            .append_option({
                row.isnoncompliant
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.qualityflag_array.append_option(row.qualityflag());
        builder
            .transactionamount_array
            .append_option({
                row.transactionamount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder.baselinecalculationid_array.append_option(row.baselinecalculationid());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.nmi_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tni_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.frmp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.drsp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.wdrsq_uncapped_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.wdrsq_capped_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mrc_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mrcsq_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.wdrrr_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.rrp_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.tlf_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.me_dlfadjusted_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bq_dlfadjusted_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.isnoncompliant_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.qualityflag_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.transactionamount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.baselinecalculationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsWdrReconDetail1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    settlementrunno_array: arrow::array::builder::Decimal128Builder,
    nmi_array: arrow::array::builder::StringBuilder,
    tni_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    frmp_array: arrow::array::builder::StringBuilder,
    drsp_array: arrow::array::builder::StringBuilder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    wdrsq_uncapped_array: arrow::array::builder::Decimal128Builder,
    wdrsq_capped_array: arrow::array::builder::Decimal128Builder,
    mrc_array: arrow::array::builder::Decimal128Builder,
    mrcsq_array: arrow::array::builder::Decimal128Builder,
    wdrrr_array: arrow::array::builder::Decimal128Builder,
    rrp_array: arrow::array::builder::Decimal128Builder,
    tlf_array: arrow::array::builder::Decimal128Builder,
    me_dlfadjusted_array: arrow::array::builder::Decimal128Builder,
    bq_dlfadjusted_array: arrow::array::builder::Decimal128Builder,
    isnoncompliant_array: arrow::array::builder::Decimal128Builder,
    qualityflag_array: arrow::array::builder::StringBuilder,
    transactionamount_array: arrow::array::builder::Decimal128Builder,
    baselinecalculationid_array: arrow::array::builder::StringBuilder,
}
pub struct SettlementsWdrTransact1;
pub struct SettlementsWdrTransact1Mapping([usize; 8]);
/// # Summary
///
/// ## SET_WDR_TRANSACT
///  _Settlements WDR transactions summary_
///
/// * Data Set Name: Settlements
/// * File Name: Wdr Transact
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * COUNTERPARTYPARTICIPANTID
/// * PARTICIPANTID
/// * PARTICIPANTROLEID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, PartialEq, Eq)]
pub struct SettlementsWdrTransact1Row<'data> {
    /// Settlement date
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// Trading interval identifier with Period 1 being the first TI for the calendar day, that is the interval ending 00:05
    pub periodid: rust_decimal::Decimal,
    /// Unique identifier for the region to which the TNI belongs on the settlement date
    pub regionid: core::ops::Range<usize>,
    /// Unique identifier for a participant
    pub participantid: core::ops::Range<usize>,
    /// Participant role identifier - FRMP or DRSP
    pub participantroleid: core::ops::Range<usize>,
    /// Unique identifier for the counter participant id.
    pub counterpartyparticipantid: core::ops::Range<usize>,
    /// Aggregate WDR transaction amount for the participant and counterparty in the settlement interval
    pub transactionamount: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SettlementsWdrTransact1Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn participantid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.participantid.clone())
    }
    pub fn participantroleid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.participantroleid.clone(),
        )
    }
    pub fn counterpartyparticipantid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.counterpartyparticipantid.clone(),
        )
    }
}
impl mmsdm_core::GetTable for SettlementsWdrTransact1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SETTLEMENTS";
    const TABLE_NAME: &'static str = "WDR_TRANSACT";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SettlementsWdrTransact1Mapping([
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
        "SETTLEMENTDATE",
        "SETTLEMENTRUNNO",
        "PERIODID",
        "REGIONID",
        "PARTICIPANTID",
        "PARTICIPANTROLEID",
        "COUNTERPARTYPARTICIPANTID",
        "TRANSACTIONAMOUNT",
    ];
    type Row<'row> = SettlementsWdrTransact1Row<'row>;
    type FieldMapping = SettlementsWdrTransact1Mapping;
    type PrimaryKey = SettlementsWdrTransact1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SettlementsWdrTransact1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            settlementrunno: row
                .get_custom_parsed_at_idx(
                    "settlementrunno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            periodid: row
                .get_custom_parsed_at_idx(
                    "periodid",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            regionid: row.get_range("regionid", field_mapping.0[3])?,
            participantid: row.get_range("participantid", field_mapping.0[4])?,
            participantroleid: row.get_range("participantroleid", field_mapping.0[5])?,
            counterpartyparticipantid: row
                .get_range("counterpartyparticipantid", field_mapping.0[6])?,
            transactionamount: row
                .get_opt_custom_parsed_at_idx(
                    "transactionamount",
                    field_mapping.0[7],
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
        Ok(SettlementsWdrTransact1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SettlementsWdrTransact1PrimaryKey {
        SettlementsWdrTransact1PrimaryKey {
            counterpartyparticipantid: row.counterpartyparticipantid().to_string(),
            participantid: row.participantid().to_string(),
            participantroleid: row.participantroleid().to_string(),
            periodid: row.periodid,
            regionid: row.regionid().to_string(),
            settlementdate: row.settlementdate,
            settlementrunno: row.settlementrunno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "settlements_wdr_transact_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SettlementsWdrTransact1Row {
            settlementdate: row.settlementdate.clone(),
            settlementrunno: row.settlementrunno.clone(),
            periodid: row.periodid.clone(),
            regionid: row.regionid.clone(),
            participantid: row.participantid.clone(),
            participantroleid: row.participantroleid.clone(),
            counterpartyparticipantid: row.counterpartyparticipantid.clone(),
            transactionamount: row.transactionamount.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementsWdrTransact1PrimaryKey {
    pub counterpartyparticipantid: alloc::string::String,
    pub participantid: alloc::string::String,
    pub participantroleid: alloc::string::String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: alloc::string::String,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsWdrTransact1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SettlementsWdrTransact1Row<'data> {
    type Row<'other> = SettlementsWdrTransact1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.counterpartyparticipantid() == row.counterpartyparticipantid()
            && self.participantid() == row.participantid()
            && self.participantroleid() == row.participantroleid()
            && self.periodid == row.periodid && self.regionid() == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SettlementsWdrTransact1Row<'data> {
    type PrimaryKey = SettlementsWdrTransact1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.counterpartyparticipantid() == key.counterpartyparticipantid
            && self.participantid() == key.participantid
            && self.participantroleid() == key.participantroleid
            && self.periodid == key.periodid && self.regionid() == key.regionid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SettlementsWdrTransact1PrimaryKey {
    type Row<'other> = SettlementsWdrTransact1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.counterpartyparticipantid == row.counterpartyparticipantid()
            && self.participantid == row.participantid()
            && self.participantroleid == row.participantroleid()
            && self.periodid == row.periodid && self.regionid == row.regionid()
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsWdrTransact1PrimaryKey {
    type PrimaryKey = SettlementsWdrTransact1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.counterpartyparticipantid == key.counterpartyparticipantid
            && self.participantid == key.participantid
            && self.participantroleid == key.participantroleid
            && self.periodid == key.periodid && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsWdrTransact1 {
    type Builder = SettlementsWdrTransact1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "settlementrunno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "periodid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
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
                    "participantroleid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "counterpartyparticipantid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "transactionamount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SettlementsWdrTransact1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            settlementrunno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            periodid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            participantid_array: arrow::array::builder::StringBuilder::new(),
            participantroleid_array: arrow::array::builder::StringBuilder::new(),
            counterpartyparticipantid_array: arrow::array::builder::StringBuilder::new(),
            transactionamount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder
            .settlementrunno_array
            .append_value({
                let mut val = row.settlementrunno;
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
        builder.regionid_array.append_value(row.regionid());
        builder.participantid_array.append_value(row.participantid());
        builder.participantroleid_array.append_value(row.participantroleid());
        builder
            .counterpartyparticipantid_array
            .append_value(row.counterpartyparticipantid());
        builder
            .transactionamount_array
            .append_option({
                row.transactionamount
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
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.settlementrunno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.periodid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.participantroleid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.counterpartyparticipantid_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.transactionamount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SettlementsWdrTransact1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    settlementrunno_array: arrow::array::builder::Decimal128Builder,
    periodid_array: arrow::array::builder::Decimal128Builder,
    regionid_array: arrow::array::builder::StringBuilder,
    participantid_array: arrow::array::builder::StringBuilder,
    participantroleid_array: arrow::array::builder::StringBuilder,
    counterpartyparticipantid_array: arrow::array::builder::StringBuilder,
    transactionamount_array: arrow::array::builder::Decimal128Builder,
}
