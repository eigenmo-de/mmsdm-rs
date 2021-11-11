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
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
}
impl crate::GetTable for MccCasesolution1 {
    type PrimaryKey = MccCasesolution1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MCC".into(),
            table_name: Some("CASESOLUTION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> MccCasesolution1PrimaryKey {
        MccCasesolution1PrimaryKey {
            run_datetime: self.run_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "mcc_casesolution_v1".to_string()
    }
}
impl crate::CompareWithRow for MccCasesolution1 {
    type Row = MccCasesolution1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for MccCasesolution1 {
    type PrimaryKey = MccCasesolution1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MccCasesolution1PrimaryKey {
    pub run_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for MccCasesolution1PrimaryKey {
    type Row = MccCasesolution1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for MccCasesolution1PrimaryKey {
    type PrimaryKey = MccCasesolution1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
    }
}
impl crate::PrimaryKey for MccCasesolution1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for MccCasesolution1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![arrow2::datatypes::Field::new(
            "run_datetime",
            arrow2::datatypes::DataType::Date32,
            false,
        )])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut run_datetime_array = Vec::new();
        for (_, row) in partition {
            run_datetime_array.push(
                i32::try_from(
                    (row.run_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![std::sync::Arc::new(
                arrow2::array::PrimitiveArray::from_slice(run_datetime_array)
                    .to(arrow2::datatypes::DataType::Date32),
            )],
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
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Generic Constraint identifier (synonymous with GenConID)
    pub constraintid: String,
    /// Generic Constraint RHS Value for this MCC run
    pub rhs: Option<rust_decimal::Decimal>,
    /// Generic Constraint Marginal Value for this MCC run
    pub marginalvalue: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for MccConstraintsolution1 {
    type PrimaryKey = MccConstraintsolution1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MCC".into(),
            table_name: Some("CONSTRAINTSOLUTION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> MccConstraintsolution1PrimaryKey {
        MccConstraintsolution1PrimaryKey {
            constraintid: self.constraintid.clone(),
            run_datetime: self.run_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "mcc_constraintsolution_v1".to_string()
    }
}
impl crate::CompareWithRow for MccConstraintsolution1 {
    type Row = MccConstraintsolution1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for MccConstraintsolution1 {
    type PrimaryKey = MccConstraintsolution1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.run_datetime == key.run_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MccConstraintsolution1PrimaryKey {
    pub constraintid: String,
    pub run_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for MccConstraintsolution1PrimaryKey {
    type Row = MccConstraintsolution1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for MccConstraintsolution1PrimaryKey {
    type PrimaryKey = MccConstraintsolution1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.run_datetime == key.run_datetime
    }
}
impl crate::PrimaryKey for MccConstraintsolution1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for MccConstraintsolution1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "run_datetime",
                arrow2::datatypes::DataType::Date32,
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

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut run_datetime_array = Vec::new();
        let mut constraintid_array = Vec::new();
        let mut rhs_array = Vec::new();
        let mut marginalvalue_array = Vec::new();
        for (_, row) in partition {
            run_datetime_array.push(
                i32::try_from(
                    (row.run_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
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

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(run_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    constraintid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rhs_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginalvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
