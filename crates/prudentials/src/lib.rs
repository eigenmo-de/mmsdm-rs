#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct PrudentialCompanyPosition1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &PrudentialCompanyPosition1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl PrudentialCompanyPosition1 {
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
pub struct PrudentialCompanyPosition1Mapping([usize; 19]);
/// # Summary
///
/// ## PRUDENTIALCOMPANYPOSITION
///  _The prudential position of each company as at the datetime of a specific prudential run_
///
/// * Data Set Name: Prudential
/// * File Name: Company Position
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * COMPANY_ID
/// * PRUDENTIAL_DATE
/// * RUNNO
#[derive(Debug, PartialEq, Eq)]
pub struct PrudentialCompanyPosition1Row<'data> {
    /// The prudential date
    pub prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date
    pub runno: i64,
    /// The company identifier
    pub company_id: core::ops::Range<usize>,
    /// The Maximum Credit Limit of the company at the time of the prudential run
    pub mcl: Option<rust_decimal::Decimal>,
    /// The Credit Support of the company at the time of the prudential run
    pub credit_support: Option<rust_decimal::Decimal>,
    /// The Trading Limit of the company at the time of the prudential run
    pub trading_limit: Option<rust_decimal::Decimal>,
    /// The balance of the company for all unpaid billing weeks at the time of the prudential run
    pub current_amount_balance: Option<rust_decimal::Decimal>,
    /// The sum of all active security deposit provision amounts at the time of the prudential run
    pub security_deposit_provision: Option<rust_decimal::Decimal>,
    /// The sum of all active security deposit application amounts at the time of the prudential run
    pub security_deposit_offset: Option<rust_decimal::Decimal>,
    /// The balance of all active security deposits at the time of the prudential run
    pub security_deposit_balance: Option<rust_decimal::Decimal>,
    /// The balance of all ex-post reallocations for the company that were calculated outside of billing runs at the time of the prudential run
    pub expost_realloc_balance: Option<rust_decimal::Decimal>,
    /// The balance of all defaults for the company at the time of the prudential run
    pub default_balance: Option<rust_decimal::Decimal>,
    /// The total outstandings for the company at the time of the prudential run
    pub outstandings: Option<rust_decimal::Decimal>,
    /// The trading margin for the company at the time of the prudential run
    pub trading_margin: Option<rust_decimal::Decimal>,
    /// The typical accrual for the company at the time of the prudential run
    pub typical_accrual: Option<rust_decimal::Decimal>,
    /// The prudential margin is the current value determined by AEMO for the registered participant. It represents the buffer below the value of credit support which is used to set the trading limit
    pub prudential_margin: Option<rust_decimal::Decimal>,
    /// The early payment amount deducted from Outstandings in the prudential run
    pub early_payment_amount: Option<rust_decimal::Decimal>,
    /// The percentage of outstandings calculated against the trading margin and prudential margin
    pub percentage_outstandings: Option<rust_decimal::Decimal>,
    /// The datetime that the record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> PrudentialCompanyPosition1Row<'data> {
    pub fn company_id(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.company_id.clone())
    }
}
impl mmsdm_core::GetTable for PrudentialCompanyPosition1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PRUDENTIAL";
    const TABLE_NAME: &'static str = "COMPANY_POSITION";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PrudentialCompanyPosition1Mapping([
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
        "PRUDENTIAL_DATE",
        "RUNNO",
        "COMPANY_ID",
        "MCL",
        "CREDIT_SUPPORT",
        "TRADING_LIMIT",
        "CURRENT_AMOUNT_BALANCE",
        "SECURITY_DEPOSIT_PROVISION",
        "SECURITY_DEPOSIT_OFFSET",
        "SECURITY_DEPOSIT_BALANCE",
        "EXPOST_REALLOC_BALANCE",
        "DEFAULT_BALANCE",
        "OUTSTANDINGS",
        "TRADING_MARGIN",
        "TYPICAL_ACCRUAL",
        "PRUDENTIAL_MARGIN",
        "EARLY_PAYMENT_AMOUNT",
        "PERCENTAGE_OUTSTANDINGS",
        "LASTCHANGED",
    ];
    type Row<'row> = PrudentialCompanyPosition1Row<'row>;
    type FieldMapping = PrudentialCompanyPosition1Mapping;
    type PrimaryKey = PrudentialCompanyPosition1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PrudentialCompanyPosition1Row {
            prudential_date: row
                .get_custom_parsed_at_idx(
                    "prudential_date",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row.get_parsed_at_idx("runno", field_mapping.0[1])?,
            company_id: row.get_range("company_id", field_mapping.0[2])?,
            mcl: row
                .get_opt_custom_parsed_at_idx(
                    "mcl",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            credit_support: row
                .get_opt_custom_parsed_at_idx(
                    "credit_support",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            trading_limit: row
                .get_opt_custom_parsed_at_idx(
                    "trading_limit",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            current_amount_balance: row
                .get_opt_custom_parsed_at_idx(
                    "current_amount_balance",
                    field_mapping.0[6],
                    mmsdm_core::mms_decimal::parse,
                )?,
            security_deposit_provision: row
                .get_opt_custom_parsed_at_idx(
                    "security_deposit_provision",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            security_deposit_offset: row
                .get_opt_custom_parsed_at_idx(
                    "security_deposit_offset",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            security_deposit_balance: row
                .get_opt_custom_parsed_at_idx(
                    "security_deposit_balance",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            expost_realloc_balance: row
                .get_opt_custom_parsed_at_idx(
                    "expost_realloc_balance",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            default_balance: row
                .get_opt_custom_parsed_at_idx(
                    "default_balance",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            outstandings: row
                .get_opt_custom_parsed_at_idx(
                    "outstandings",
                    field_mapping.0[12],
                    mmsdm_core::mms_decimal::parse,
                )?,
            trading_margin: row
                .get_opt_custom_parsed_at_idx(
                    "trading_margin",
                    field_mapping.0[13],
                    mmsdm_core::mms_decimal::parse,
                )?,
            typical_accrual: row
                .get_opt_custom_parsed_at_idx(
                    "typical_accrual",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            prudential_margin: row
                .get_opt_custom_parsed_at_idx(
                    "prudential_margin",
                    field_mapping.0[15],
                    mmsdm_core::mms_decimal::parse,
                )?,
            early_payment_amount: row
                .get_opt_custom_parsed_at_idx(
                    "early_payment_amount",
                    field_mapping.0[16],
                    mmsdm_core::mms_decimal::parse,
                )?,
            percentage_outstandings: row
                .get_opt_custom_parsed_at_idx(
                    "percentage_outstandings",
                    field_mapping.0[17],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[18],
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
        Ok(PrudentialCompanyPosition1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PrudentialCompanyPosition1PrimaryKey {
        PrudentialCompanyPosition1PrimaryKey {
            company_id: row.company_id().to_string(),
            prudential_date: row.prudential_date,
            runno: row.runno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("prudential_company_position_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PrudentialCompanyPosition1Row {
            prudential_date: row.prudential_date.clone(),
            runno: row.runno.clone(),
            company_id: row.company_id.clone(),
            mcl: row.mcl.clone(),
            credit_support: row.credit_support.clone(),
            trading_limit: row.trading_limit.clone(),
            current_amount_balance: row.current_amount_balance.clone(),
            security_deposit_provision: row.security_deposit_provision.clone(),
            security_deposit_offset: row.security_deposit_offset.clone(),
            security_deposit_balance: row.security_deposit_balance.clone(),
            expost_realloc_balance: row.expost_realloc_balance.clone(),
            default_balance: row.default_balance.clone(),
            outstandings: row.outstandings.clone(),
            trading_margin: row.trading_margin.clone(),
            typical_accrual: row.typical_accrual.clone(),
            prudential_margin: row.prudential_margin.clone(),
            early_payment_amount: row.early_payment_amount.clone(),
            percentage_outstandings: row.percentage_outstandings.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PrudentialCompanyPosition1PrimaryKey {
    pub company_id: alloc::string::String,
    pub prudential_date: chrono::NaiveDateTime,
    pub runno: i64,
}
impl mmsdm_core::PrimaryKey for PrudentialCompanyPosition1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PrudentialCompanyPosition1Row<'data> {
    type Row<'other> = PrudentialCompanyPosition1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.company_id() == row.company_id()
            && self.prudential_date == row.prudential_date && self.runno == row.runno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for PrudentialCompanyPosition1Row<'data> {
    type PrimaryKey = PrudentialCompanyPosition1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.company_id() == key.company_id
            && self.prudential_date == key.prudential_date && self.runno == key.runno
    }
}
impl<'data> mmsdm_core::CompareWithRow for PrudentialCompanyPosition1PrimaryKey {
    type Row<'other> = PrudentialCompanyPosition1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.company_id == row.company_id()
            && self.prudential_date == row.prudential_date && self.runno == row.runno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PrudentialCompanyPosition1PrimaryKey {
    type PrimaryKey = PrudentialCompanyPosition1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.company_id == key.company_id && self.prudential_date == key.prudential_date
            && self.runno == key.runno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PrudentialCompanyPosition1 {
    type Builder = PrudentialCompanyPosition1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "prudential_date",
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
                    "company_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "mcl",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "credit_support",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "trading_limit",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "current_amount_balance",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "security_deposit_provision",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "security_deposit_offset",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "security_deposit_balance",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "expost_realloc_balance",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "default_balance",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "outstandings",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "trading_margin",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "typical_accrual",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "prudential_margin",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "early_payment_amount",
                    arrow::datatypes::DataType::Decimal128(18, 8),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "percentage_outstandings",
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
        PrudentialCompanyPosition1Builder {
            prudential_date_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Int64Builder::new(),
            company_id_array: arrow::array::builder::StringBuilder::new(),
            mcl_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            credit_support_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            trading_limit_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            current_amount_balance_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            security_deposit_provision_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            security_deposit_offset_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            security_deposit_balance_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            expost_realloc_balance_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            default_balance_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            outstandings_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            trading_margin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            typical_accrual_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            prudential_margin_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            early_payment_amount_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            percentage_outstandings_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(18, 8)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .prudential_date_array
            .append_value(row.prudential_date.and_utc().timestamp_millis());
        builder.runno_array.append_value(row.runno);
        builder.company_id_array.append_value(row.company_id());
        builder
            .mcl_array
            .append_option({
                row.mcl
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .credit_support_array
            .append_option({
                row.credit_support
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .trading_limit_array
            .append_option({
                row.trading_limit
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .current_amount_balance_array
            .append_option({
                row.current_amount_balance
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .security_deposit_provision_array
            .append_option({
                row.security_deposit_provision
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .security_deposit_offset_array
            .append_option({
                row.security_deposit_offset
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .security_deposit_balance_array
            .append_option({
                row.security_deposit_balance
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .expost_realloc_balance_array
            .append_option({
                row.expost_realloc_balance
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .default_balance_array
            .append_option({
                row.default_balance
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .outstandings_array
            .append_option({
                row.outstandings
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .trading_margin_array
            .append_option({
                row.trading_margin
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .typical_accrual_array
            .append_option({
                row.typical_accrual
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .prudential_margin_array
            .append_option({
                row.prudential_margin
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .early_payment_amount_array
            .append_option({
                row.early_payment_amount
                    .map(|mut val| {
                        val.rescale(8);
                        val.mantissa()
                    })
            });
        builder
            .percentage_outstandings_array
            .append_option({
                row.percentage_outstandings
                    .map(|mut val| {
                        val.rescale(8);
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
                    alloc::sync::Arc::new(builder.prudential_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.company_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mcl_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.credit_support_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.trading_limit_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.current_amount_balance_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.security_deposit_provision_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.security_deposit_offset_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(
                        builder.security_deposit_balance_array.finish(),
                    ) as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.expost_realloc_balance_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.default_balance_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.outstandings_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.trading_margin_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.typical_accrual_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.prudential_margin_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.early_payment_amount_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.percentage_outstandings_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct PrudentialCompanyPosition1Builder {
    prudential_date_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Int64Builder,
    company_id_array: arrow::array::builder::StringBuilder,
    mcl_array: arrow::array::builder::Decimal128Builder,
    credit_support_array: arrow::array::builder::Decimal128Builder,
    trading_limit_array: arrow::array::builder::Decimal128Builder,
    current_amount_balance_array: arrow::array::builder::Decimal128Builder,
    security_deposit_provision_array: arrow::array::builder::Decimal128Builder,
    security_deposit_offset_array: arrow::array::builder::Decimal128Builder,
    security_deposit_balance_array: arrow::array::builder::Decimal128Builder,
    expost_realloc_balance_array: arrow::array::builder::Decimal128Builder,
    default_balance_array: arrow::array::builder::Decimal128Builder,
    outstandings_array: arrow::array::builder::Decimal128Builder,
    trading_margin_array: arrow::array::builder::Decimal128Builder,
    typical_accrual_array: arrow::array::builder::Decimal128Builder,
    prudential_margin_array: arrow::array::builder::Decimal128Builder,
    early_payment_amount_array: arrow::array::builder::Decimal128Builder,
    percentage_outstandings_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct PrudentialRuntrk1 {
    extract_row_partition: alloc::boxed::Box<
        dyn Fn(
            &PrudentialRuntrk1Row<'_>,
        ) -> mmsdm_core::PartitionValue + Send + Sync + 'static,
    >,
    row_partition_key: mmsdm_core::PartitionKey,
}
impl PrudentialRuntrk1 {
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
pub struct PrudentialRuntrk1Mapping([usize; 5]);
/// # Summary
///
/// ## PRUDENTIALRUNTRK
///  _Records the prudential run accepted by Settlements staff for each prudential date_
///
/// * Data Set Name: Prudential
/// * File Name: Runtrk
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * PRUDENTIAL_DATE
/// * RUNNO
#[derive(Debug, PartialEq, Eq)]
pub struct PrudentialRuntrk1Row<'data> {
    /// The prudential date
    pub prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date
    pub runno: i64,
    /// The user that authorised the prudential run
    pub authorisedby: core::ops::Range<usize>,
    /// The datetime that the prudential run was authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// The datetime that the record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> PrudentialRuntrk1Row<'data> {
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
impl mmsdm_core::GetTable for PrudentialRuntrk1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "PRUDENTIAL";
    const TABLE_NAME: &'static str = "RUNTRK";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = PrudentialRuntrk1Mapping([
        4,
        5,
        6,
        7,
        8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "PRUDENTIAL_DATE",
        "RUNNO",
        "AUTHORISEDBY",
        "AUTHORISEDDATE",
        "LASTCHANGED",
    ];
    type Row<'row> = PrudentialRuntrk1Row<'row>;
    type FieldMapping = PrudentialRuntrk1Mapping;
    type PrimaryKey = PrudentialRuntrk1PrimaryKey;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(PrudentialRuntrk1Row {
            prudential_date: row
                .get_custom_parsed_at_idx(
                    "prudential_date",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            runno: row.get_parsed_at_idx("runno", field_mapping.0[1])?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[2])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
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
        Ok(PrudentialRuntrk1Mapping(base_mapping))
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> PrudentialRuntrk1PrimaryKey {
        PrudentialRuntrk1PrimaryKey {
            prudential_date: row.prudential_date,
            runno: row.runno,
        }
    }
    fn partition_value(&self, row: &Self::Row<'_>) -> mmsdm_core::PartitionValue {
        (self.extract_row_partition)(row)
    }
    fn partition_name(&self, row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!("prudential_runtrk_v1_{}", self.partition_value(row))
    }
    fn partition_key(&self) -> mmsdm_core::PartitionKey {
        self.row_partition_key
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        PrudentialRuntrk1Row {
            prudential_date: row.prudential_date.clone(),
            runno: row.runno.clone(),
            authorisedby: row.authorisedby.clone(),
            authoriseddate: row.authoriseddate.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PrudentialRuntrk1PrimaryKey {
    pub prudential_date: chrono::NaiveDateTime,
    pub runno: i64,
}
impl mmsdm_core::PrimaryKey for PrudentialRuntrk1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for PrudentialRuntrk1Row<'data> {
    type Row<'other> = PrudentialRuntrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.prudential_date == row.prudential_date && self.runno == row.runno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for PrudentialRuntrk1Row<'data> {
    type PrimaryKey = PrudentialRuntrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.prudential_date == key.prudential_date && self.runno == key.runno
    }
}
impl<'data> mmsdm_core::CompareWithRow for PrudentialRuntrk1PrimaryKey {
    type Row<'other> = PrudentialRuntrk1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.prudential_date == row.prudential_date && self.runno == row.runno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PrudentialRuntrk1PrimaryKey {
    type PrimaryKey = PrudentialRuntrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.prudential_date == key.prudential_date && self.runno == key.runno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PrudentialRuntrk1 {
    type Builder = PrudentialRuntrk1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "prudential_date",
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
        PrudentialRuntrk1Builder {
            prudential_date_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            runno_array: arrow::array::builder::Int64Builder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .prudential_date_array
            .append_value(row.prudential_date.and_utc().timestamp_millis());
        builder.runno_array.append_value(row.runno);
        builder.authorisedby_array.append_option(row.authorisedby());
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
                    alloc::sync::Arc::new(builder.prudential_date_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.runno_array.finish())
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
pub struct PrudentialRuntrk1Builder {
    prudential_date_array: arrow::array::builder::TimestampMillisecondBuilder,
    runno_array: arrow::array::builder::Int64Builder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
