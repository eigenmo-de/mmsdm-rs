use chrono::Datelike as _;
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
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * COMPANY_ID
/// * PRUDENTIAL_DATE
/// * RUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PrudentialCompanyPosition1 {
    /// The prudential date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date
    pub runno: i64,
    /// The company identifier
    pub company_id: String,
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
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for PrudentialCompanyPosition1 {
    type PrimaryKey = PrudentialCompanyPosition1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PRUDENTIAL".into(),
            table_name: Some("COMPANY_POSITION".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> PrudentialCompanyPosition1PrimaryKey {
        PrudentialCompanyPosition1PrimaryKey {
            company_id: self.company_id.clone(),
            prudential_date: self.prudential_date,
            runno: self.runno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "prudential_company_position_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PrudentialCompanyPosition1PrimaryKey {
    pub company_id: String,
    pub prudential_date: chrono::NaiveDateTime,
    pub runno: i64,
}
impl mmsdm_core::PrimaryKey for PrudentialCompanyPosition1PrimaryKey {}
impl mmsdm_core::CompareWithRow for PrudentialCompanyPosition1 {
    type Row = PrudentialCompanyPosition1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.company_id == row.company_id && self.prudential_date == row.prudential_date
            && self.runno == row.runno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PrudentialCompanyPosition1 {
    type PrimaryKey = PrudentialCompanyPosition1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.company_id == key.company_id && self.prudential_date == key.prudential_date
            && self.runno == key.runno
    }
}
impl mmsdm_core::CompareWithRow for PrudentialCompanyPosition1PrimaryKey {
    type Row = PrudentialCompanyPosition1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.company_id == row.company_id && self.prudential_date == row.prudential_date
            && self.runno == row.runno
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("prudential_date",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("runno",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("company_id",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("mcl",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("credit_support",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("trading_limit",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("current_amount_balance",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("security_deposit_provision",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("security_deposit_offset",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("security_deposit_balance",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("expost_realloc_balance",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("default_balance",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("outstandings",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("trading_margin",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("typical_accrual",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("prudential_margin",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("early_payment_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("percentage_outstandings",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut prudential_date_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut company_id_array = Vec::new();
        let mut mcl_array = Vec::new();
        let mut credit_support_array = Vec::new();
        let mut trading_limit_array = Vec::new();
        let mut current_amount_balance_array = Vec::new();
        let mut security_deposit_provision_array = Vec::new();
        let mut security_deposit_offset_array = Vec::new();
        let mut security_deposit_balance_array = Vec::new();
        let mut expost_realloc_balance_array = Vec::new();
        let mut default_balance_array = Vec::new();
        let mut outstandings_array = Vec::new();
        let mut trading_margin_array = Vec::new();
        let mut typical_accrual_array = Vec::new();
        let mut prudential_margin_array = Vec::new();
        let mut early_payment_amount_array = Vec::new();
        let mut percentage_outstandings_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            prudential_date_array.push(row.prudential_date.timestamp());
            runno_array.push(row.runno);
            company_id_array.push(row.company_id);
            mcl_array
                .push({
                    row.mcl
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            credit_support_array
                .push({
                    row.credit_support
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            trading_limit_array
                .push({
                    row.trading_limit
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            current_amount_balance_array
                .push({
                    row.current_amount_balance
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            security_deposit_provision_array
                .push({
                    row.security_deposit_provision
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            security_deposit_offset_array
                .push({
                    row.security_deposit_offset
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            security_deposit_balance_array
                .push({
                    row.security_deposit_balance
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            expost_realloc_balance_array
                .push({
                    row.expost_realloc_balance
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            default_balance_array
                .push({
                    row.default_balance
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            outstandings_array
                .push({
                    row.outstandings
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            trading_margin_array
                .push({
                    row.trading_margin
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            typical_accrual_array
                .push({
                    row.typical_accrual
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            prudential_margin_array
                .push({
                    row.prudential_margin
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            early_payment_amount_array
                .push({
                    row.early_payment_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            percentage_outstandings_array
                .push({
                    row.percentage_outstandings
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(prudential_date_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(runno_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(company_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(mcl_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(credit_support_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(trading_limit_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(current_amount_balance_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(security_deposit_provision_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(security_deposit_offset_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(security_deposit_balance_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(expost_realloc_balance_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(default_balance_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(outstandings_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(trading_margin_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(typical_accrual_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(prudential_margin_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(early_payment_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(percentage_outstandings_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
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
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * PRUDENTIAL_DATE
/// * RUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PrudentialRuntrk1 {
    /// The prudential date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date
    pub runno: i64,
    /// The user that authorised the prudential run
    pub authorisedby: Option<String>,
    /// The datetime that the prudential run was authorised
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// The datetime that the record was last changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for PrudentialRuntrk1 {
    type PrimaryKey = PrudentialRuntrk1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PRUDENTIAL".into(),
            table_name: Some("RUNTRK".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> PrudentialRuntrk1PrimaryKey {
        PrudentialRuntrk1PrimaryKey {
            prudential_date: self.prudential_date,
            runno: self.runno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "prudential_runtrk_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PrudentialRuntrk1PrimaryKey {
    pub prudential_date: chrono::NaiveDateTime,
    pub runno: i64,
}
impl mmsdm_core::PrimaryKey for PrudentialRuntrk1PrimaryKey {}
impl mmsdm_core::CompareWithRow for PrudentialRuntrk1 {
    type Row = PrudentialRuntrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.prudential_date == row.prudential_date && self.runno == row.runno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PrudentialRuntrk1 {
    type PrimaryKey = PrudentialRuntrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.prudential_date == key.prudential_date && self.runno == key.runno
    }
}
impl mmsdm_core::CompareWithRow for PrudentialRuntrk1PrimaryKey {
    type Row = PrudentialRuntrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
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
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("prudential_date",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("runno",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("authorisedby",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("authoriseddate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut prudential_date_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            prudential_date_array.push(row.prudential_date.timestamp());
            runno_array.push(row.runno);
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(prudential_date_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(runno_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(authorisedby_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "sql_server")]
pub async fn save<'a, S>(
    mms_file: &mut mmsdm_core::MmsFile<'a>,
    file_key: &mmsdm_core::FileKey,
    client: &mut tiberius::Client<S>,
    chunk_size: Option<usize>,
) -> mmsdm_core::Result<()>
where
    S: futures_util::AsyncRead + futures_util::AsyncWrite + Unpin + Send,
{
    match (file_key.table_name.as_deref(), file_key.version) {
        (Some("COMPANY_POSITION"), version) if version <= 1_i32 => {
            let d: Vec<PrudentialCompanyPosition1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertPrudentialCompanyPosition1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("RUNTRK"), version) if version <= 1_i32 => {
            let d: Vec<PrudentialRuntrk1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertPrudentialRuntrk1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        _ => {
            log::error!("Unexpected file key {:?}", file_key);
        }
    }
    Ok(())
}
