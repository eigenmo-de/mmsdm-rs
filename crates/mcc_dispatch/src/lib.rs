/// # Summary
///
/// ## MCC_CASESOLUTION
///  _Top level table for each MCC dispatch rerun process. Note there will be one record for each dispatch interval_
///
/// * Data Set Name: Mcc
/// * File Name: Casesolution
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MccCasesolution1 {
    /// 5-minute Dispatch Run identifier
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::GetTable for MccCasesolution1 {
    type PrimaryKey = MccCasesolution1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MCC".into(),
            table_name: Some("CASESOLUTION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> MccCasesolution1PrimaryKey {
        MccCasesolution1PrimaryKey {
            run_datetime: self.run_datetime,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "mcc_casesolution_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MccCasesolution1PrimaryKey {
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MccCasesolution1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MccCasesolution1 {
    type Row = MccCasesolution1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MccCasesolution1 {
    type PrimaryKey = MccCasesolution1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
    }
}
impl mmsdm_core::CompareWithRow for MccCasesolution1PrimaryKey {
    type Row = MccCasesolution1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MccCasesolution1PrimaryKey {
    type PrimaryKey = MccCasesolution1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MccCasesolution1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![arrow2::datatypes::Field::new(
            "run_datetime",
            arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
            false,
        )])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut run_datetime_array = Vec::new();
        for row in partition {
            run_datetime_array.push(row.run_datetime.timestamp());
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![std::sync::Arc::new(
                arrow2::array::PrimitiveArray::from_vec(run_datetime_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                ),
            ) as std::sync::Arc<dyn arrow2::array::Array>],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MCC_CONSTRAINTSOLUTION
///  _Constraint solution data from the MCC dispatch rerun process. Note only constraints with a non-zero marginal value are published._
///
/// * Data Set Name: Mcc
/// * File Name: Constraintsolution
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MccConstraintsolution1 {
    /// 5-minute Dispatch Run identifier
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Generic Constraint identifier (synonymous with GenConID)
    pub constraintid: String,
    /// Generic Constraint RHS Value for this MCC run
    pub rhs: Option<rust_decimal::Decimal>,
    /// Generic Constraint Marginal Value for this MCC run
    pub marginalvalue: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for MccConstraintsolution1 {
    type PrimaryKey = MccConstraintsolution1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MCC".into(),
            table_name: Some("CONSTRAINTSOLUTION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> MccConstraintsolution1PrimaryKey {
        MccConstraintsolution1PrimaryKey {
            constraintid: self.constraintid.clone(),
            run_datetime: self.run_datetime,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "mcc_constraintsolution_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MccConstraintsolution1PrimaryKey {
    pub constraintid: String,
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MccConstraintsolution1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MccConstraintsolution1 {
    type Row = MccConstraintsolution1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MccConstraintsolution1 {
    type PrimaryKey = MccConstraintsolution1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.run_datetime == key.run_datetime
    }
}
impl mmsdm_core::CompareWithRow for MccConstraintsolution1PrimaryKey {
    type Row = MccConstraintsolution1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid && self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MccConstraintsolution1PrimaryKey {
    type PrimaryKey = MccConstraintsolution1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.run_datetime == key.run_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MccConstraintsolution1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "run_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "constraintid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("rhs", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new(
                "marginalvalue",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut run_datetime_array = Vec::new();
        let mut constraintid_array = Vec::new();
        let mut rhs_array = Vec::new();
        let mut marginalvalue_array = Vec::new();
        for row in partition {
            run_datetime_array.push(row.run_datetime.timestamp());
            constraintid_array.push(row.constraintid);
            rhs_array.push({
                row.rhs.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            marginalvalue_array.push({
                row.marginalvalue.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(run_datetime_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    constraintid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rhs_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginalvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
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
        (Some("CASESOLUTION"), version) if version <= 1_i32 => {
            let d: Vec<MccCasesolution1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertMccCasesolution1 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("CONSTRAINTSOLUTION"), version) if version <= 1_i32 => {
            let d: Vec<MccConstraintsolution1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertMccConstraintsolution1 @P1, @P2",
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
