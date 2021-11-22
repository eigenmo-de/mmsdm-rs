/// # Summary
///
/// ## CONSTRAINTRELAXATION_OCD
///  _CONSTRAINTRELAXATION_OCD contains details of interconnector constraints and unit ancillary service constraints relaxed in the over-constrained dispatch (OCD) re-run for this interval (if there was one).<br>Note: INTERVENTION is not included in CONSTRAINTRELAXATION_OCD, since the relaxation of the same constraint is the same amount in both intervened and non-intervened cases.<br>_
///
/// * Data Set Name: Priceload
/// * File Name: Constraintrelaxation
/// * Data Version: 1
///
/// # Description
///  Source The occurrences of Over-Constrained Dispatch (OCD) re-runs are ad hoc, with significant dependencies on the configuration or events in the physical power system. Over-constrained dispatch (OCD) re-run (if there was one). Volume Rows per day: ~2 Mb per month: &lt;1 The estimates on the number of rows are based on a 1% occurrence rate for OCD runs.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * RUNNO
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PriceloadConstraintrelaxation1 {
    /// End date and time of the dispatch interval
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no
    pub runno: rust_decimal::Decimal,
    /// Constraint identifier
    pub constraintid: String,
    /// Relaxed RHS used in attempt to avoid constraint violation
    pub rhs: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Version Number
    pub versionno: rust_decimal::Decimal,
}
impl crate::GetTable for PriceloadConstraintrelaxation1 {
    type PrimaryKey = PriceloadConstraintrelaxation1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PRICELOAD".into(),
            table_name: Some("CONSTRAINTRELAXATION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> PriceloadConstraintrelaxation1PrimaryKey {
        PriceloadConstraintrelaxation1PrimaryKey {
            constraintid: self.constraintid.clone(),
            runno: self.runno,
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "priceload_constraintrelaxation_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for PriceloadConstraintrelaxation1 {
    type Row = PriceloadConstraintrelaxation1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for PriceloadConstraintrelaxation1 {
    type PrimaryKey = PriceloadConstraintrelaxation1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PriceloadConstraintrelaxation1PrimaryKey {
    pub constraintid: String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for PriceloadConstraintrelaxation1PrimaryKey {
    type Row = PriceloadConstraintrelaxation1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for PriceloadConstraintrelaxation1PrimaryKey {
    type PrimaryKey = PriceloadConstraintrelaxation1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for PriceloadConstraintrelaxation1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PriceloadConstraintrelaxation1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "constraintid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("rhs", arrow2::datatypes::DataType::Decimal(16, 6), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut constraintid_array = Vec::new();
        let mut rhs_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut versionno_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
            constraintid_array.push(row.constraintid);
            rhs_array.push({
                row.rhs.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    constraintid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rhs_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCHBLOCKEDCONSTRAINT
///  _DISPATCH Blocked Constraints lists any constraints that were blocked in a dispatch run. If no constraints are blocked, there will be no rows for that dispatch run._
///
/// * Data Set Name: Dispatch
/// * File Name: Blocked Constraints
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchBlockedConstraints1 {
    /// Dispatch Interval
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Generic Constraint identifier (synonymous with GenConID)
    pub constraintid: String,
}
impl crate::GetTable for DispatchBlockedConstraints1 {
    type PrimaryKey = DispatchBlockedConstraints1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("BLOCKED_CONSTRAINTS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DispatchBlockedConstraints1PrimaryKey {
        DispatchBlockedConstraints1PrimaryKey {
            constraintid: self.constraintid.clone(),
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "dispatch_blocked_constraints_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DispatchBlockedConstraints1 {
    type Row = DispatchBlockedConstraints1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchBlockedConstraints1 {
    type PrimaryKey = DispatchBlockedConstraints1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchBlockedConstraints1PrimaryKey {
    pub constraintid: String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DispatchBlockedConstraints1PrimaryKey {
    type Row = DispatchBlockedConstraints1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchBlockedConstraints1PrimaryKey {
    type PrimaryKey = DispatchBlockedConstraints1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for DispatchBlockedConstraints1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchBlockedConstraints1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "constraintid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut constraintid_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
            constraintid_array.push(row.constraintid);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    constraintid_array,
                )),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCHCASESOLUTION
///  _DISPATCHCASESOLUTION shows information relating to the complete dispatch run. The fields in DISPATCHCASESOLUTION provide an overview of the dispatch run results allowing immediate identification of conditions such as energy or FCAS deficiencies._
///
/// * Data Set Name: Dispatch
/// * File Name: Case Solution
/// * Data Version: 2
///
/// # Description
///  The DISPATCHCASESOLUTION data is public. Source DISPATCHCASESOLUTION updates every 5 minutes. Volume Approximately 288 records per day.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchCaseSolution2 {
    /// Date and time of the dispatch interval (e.g. five minute dispatch interval ending 28/09/2000 16:35)
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Intervention flag - refer to package documentation for definition and practical query examples
    pub intervention: rust_decimal::Decimal,
    /// Overconstrained dispatch indicator: <br>* OCD = detecting over-constrained dispatch<br>* null = no special condition
    pub casesubtype: Option<String>,
    /// If non-zero indicated one of the following conditions:<br>* 1 = Supply Scarcity, Excess generation or constraint violations<br>* X = Model failure
    pub solutionstatus: Option<rust_decimal::Decimal>,
    /// Current version of SPD
    pub spdversion: Option<String>,
    /// Non-Physical Losses algorithm invoked occurred during this run
    pub nonphysicallosses: Option<rust_decimal::Decimal>,
    /// The Objective function from the LP
    pub totalobjective: Option<rust_decimal::Decimal>,
    /// Total Region Demand violations
    pub totalareagenviolation: Option<rust_decimal::Decimal>,
    /// Total interconnector violations
    pub totalinterconnectorviolation: Option<rust_decimal::Decimal>,
    /// Total generic constraint violations
    pub totalgenericviolation: Option<rust_decimal::Decimal>,
    /// Total ramp rate violations
    pub totalramprateviolation: Option<rust_decimal::Decimal>,
    /// Total unit capacity violations
    pub totalunitmwcapacityviolation: Option<rust_decimal::Decimal>,
    /// Total of 5 minute ancillary service region violations
    pub total5minviolation: Option<rust_decimal::Decimal>,
    /// Total of Regulation ancillary service region violations
    pub totalregviolation: Option<rust_decimal::Decimal>,
    /// Total of 6 second ancillary service region violations
    pub total6secviolation: Option<rust_decimal::Decimal>,
    /// Total of 60 second ancillary service region violations
    pub total60secviolation: Option<rust_decimal::Decimal>,
    /// Total of ancillary service trader profile violations
    pub totalasprofileviolation: Option<rust_decimal::Decimal>,
    /// Total of fast start trader profile violations
    pub totalfaststartviolation: Option<rust_decimal::Decimal>,
    /// Total of unit summated offer band violations
    pub totalenergyofferviolation: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag indicating the SCADA status for FCAS Interconnector dead-band. "0" if SCADA Status or requesting Constraint not invoked. "1" if SCADA Status AND requesting Constraint is invoked
    pub switchruninitialstatus: Option<rust_decimal::Decimal>,
    /// Flag indicating which Switch run was used for the Solution – from PeriodSolution
    pub switchrunbeststatus: Option<rust_decimal::Decimal>,
    /// Flag indicating which Switch run was used for the Intervention Physical Solution - from PeriodSolution
    pub switchrunbeststatus_int: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DispatchCaseSolution2 {
    type PrimaryKey = DispatchCaseSolution2PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("CASE_SOLUTION".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> DispatchCaseSolution2PrimaryKey {
        DispatchCaseSolution2PrimaryKey {
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "dispatch_case_solution_v2_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DispatchCaseSolution2 {
    type Row = DispatchCaseSolution2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchCaseSolution2 {
    type PrimaryKey = DispatchCaseSolution2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchCaseSolution2PrimaryKey {
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DispatchCaseSolution2PrimaryKey {
    type Row = DispatchCaseSolution2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchCaseSolution2PrimaryKey {
    type PrimaryKey = DispatchCaseSolution2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for DispatchCaseSolution2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchCaseSolution2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "casesubtype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "solutionstatus",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "spdversion",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "nonphysicallosses",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "totalobjective",
                arrow2::datatypes::DataType::Decimal(27, 10),
                true,
            ),
            arrow2::datatypes::Field::new(
                "totalareagenviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "totalinterconnectorviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "totalgenericviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "totalramprateviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "totalunitmwcapacityviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "total5minviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "totalregviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "total6secviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "total60secviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "totalasprofileviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "totalfaststartviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "totalenergyofferviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "switchruninitialstatus",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "switchrunbeststatus",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "switchrunbeststatus_int",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut casesubtype_array = Vec::new();
        let mut solutionstatus_array = Vec::new();
        let mut spdversion_array = Vec::new();
        let mut nonphysicallosses_array = Vec::new();
        let mut totalobjective_array = Vec::new();
        let mut totalareagenviolation_array = Vec::new();
        let mut totalinterconnectorviolation_array = Vec::new();
        let mut totalgenericviolation_array = Vec::new();
        let mut totalramprateviolation_array = Vec::new();
        let mut totalunitmwcapacityviolation_array = Vec::new();
        let mut total5minviolation_array = Vec::new();
        let mut totalregviolation_array = Vec::new();
        let mut total6secviolation_array = Vec::new();
        let mut total60secviolation_array = Vec::new();
        let mut totalasprofileviolation_array = Vec::new();
        let mut totalfaststartviolation_array = Vec::new();
        let mut totalenergyofferviolation_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut switchruninitialstatus_array = Vec::new();
        let mut switchrunbeststatus_array = Vec::new();
        let mut switchrunbeststatus_int_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
            intervention_array.push({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
            casesubtype_array.push(row.casesubtype);
            solutionstatus_array.push({
                row.solutionstatus.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            spdversion_array.push(row.spdversion);
            nonphysicallosses_array.push({
                row.nonphysicallosses.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            totalobjective_array.push({
                row.totalobjective.map(|mut val| {
                    val.rescale(10);
                    val.mantissa()
                })
            });
            totalareagenviolation_array.push({
                row.totalareagenviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            totalinterconnectorviolation_array.push({
                row.totalinterconnectorviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            totalgenericviolation_array.push({
                row.totalgenericviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            totalramprateviolation_array.push({
                row.totalramprateviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            totalunitmwcapacityviolation_array.push({
                row.totalunitmwcapacityviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            total5minviolation_array.push({
                row.total5minviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            totalregviolation_array.push({
                row.totalregviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            total6secviolation_array.push({
                row.total6secviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            total60secviolation_array.push({
                row.total60secviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            totalasprofileviolation_array.push({
                row.totalasprofileviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            totalfaststartviolation_array.push({
                row.totalfaststartviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            totalenergyofferviolation_array.push({
                row.totalenergyofferviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            switchruninitialstatus_array.push({
                row.switchruninitialstatus.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            switchrunbeststatus_array.push({
                row.switchrunbeststatus.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            switchrunbeststatus_int_array.push({
                row.switchrunbeststatus_int.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(casesubtype_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(solutionstatus_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(spdversion_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(nonphysicallosses_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalobjective_array)
                        .to(arrow2::datatypes::DataType::Decimal(27, 10)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalareagenviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalinterconnectorviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalgenericviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalramprateviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalunitmwcapacityviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(total5minviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalregviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(total6secviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(total60secviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalasprofileviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalfaststartviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalenergyofferviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(switchruninitialstatus_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(switchrunbeststatus_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(switchrunbeststatus_int_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCHCONSTRAINT
///  _DISPATCHCONSTRAINT sets out details of all binding and interregion constraints in each dispatch run. Note: invoked constraints can be established from GENCONSETINVOKE. Binding constraints show as marginal value &gt;$0. Interconnector constraints are listed so RHS (SCADA calculated limits) can be reported._
///
/// * Data Set Name: Dispatch
/// * File Name: Constraint
/// * Data Version: 5
///
/// # Description
///  DISPATCHCONSTRAINT is public data, and is available to all participants. Source DISPATCHCONSTRAINT updates every five minutes.
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * DISPATCHINTERVAL
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchConstraint5 {
    /// Market date starting at 04:05
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Generic Constraint identifier (synonymous with GenConID)
    pub constraintid: String,
    /// Dispatch period identifier, from 001 to 288 in format YYYYMMDDPPP.
    #[serde(with = "crate::dispatch_period")]
    pub dispatchinterval: crate::DispatchPeriod,
    /// Manual Intervention flag, which, if set (1), causes predispatch to solve twice.
    pub intervention: rust_decimal::Decimal,
    /// Right hand Side value as used in dispatch.
    pub rhs: Option<rust_decimal::Decimal>,
    /// $ Value of binding constraint
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Degree of violation in MW
    pub violationdegree: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// DUID to which the Constraint is confidential. Null denotes non-confidential
    pub duid: Option<String>,
    /// Effective date of the Generic Constraint (ConstraintID). This field is used to track the version of this generic constraint applied in this dispatch interval
    #[serde(with = "crate::mms_datetime_opt")]
    pub genconid_effectivedate: Option<chrono::NaiveDateTime>,
    /// Version number of the Generic Constraint (ConstraintID). This field is used to track the version of this generic constraint applied in this dispatch interval
    pub genconid_versionno: Option<rust_decimal::Decimal>,
    /// Aggregation of the constraints LHS term solution values
    pub lhs: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DispatchConstraint5 {
    type PrimaryKey = DispatchConstraint5PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("CONSTRAINT".into()),
            version: 5,
        }
    }

    fn primary_key(&self) -> DispatchConstraint5PrimaryKey {
        DispatchConstraint5PrimaryKey {
            constraintid: self.constraintid.clone(),
            dispatchinterval: self.dispatchinterval,
            intervention: self.intervention,
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "dispatch_constraint_v5_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DispatchConstraint5 {
    type Row = DispatchConstraint5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid
            && self.dispatchinterval == row.dispatchinterval
            && self.intervention == row.intervention
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchConstraint5 {
    type PrimaryKey = DispatchConstraint5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.dispatchinterval == key.dispatchinterval
            && self.intervention == key.intervention
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchConstraint5PrimaryKey {
    pub constraintid: String,
    pub dispatchinterval: crate::DispatchPeriod,
    pub intervention: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DispatchConstraint5PrimaryKey {
    type Row = DispatchConstraint5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid
            && self.dispatchinterval == row.dispatchinterval
            && self.intervention == row.intervention
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchConstraint5PrimaryKey {
    type PrimaryKey = DispatchConstraint5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.dispatchinterval == key.dispatchinterval
            && self.intervention == key.intervention
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for DispatchConstraint5PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchConstraint5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "constraintid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "dispatchinterval",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                false,
            ),
            arrow2::datatypes::Field::new("rhs", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new(
                "marginalvalue",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "violationdegree",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "genconid_effectivedate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "genconid_versionno",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lhs", arrow2::datatypes::DataType::Decimal(15, 5), true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut constraintid_array = Vec::new();
        let mut dispatchinterval_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut rhs_array = Vec::new();
        let mut marginalvalue_array = Vec::new();
        let mut violationdegree_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut genconid_effectivedate_array = Vec::new();
        let mut genconid_versionno_array = Vec::new();
        let mut lhs_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
            constraintid_array.push(row.constraintid);
            dispatchinterval_array.push(row.dispatchinterval.start().timestamp_millis());
            intervention_array.push({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
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
            violationdegree_array.push({
                row.violationdegree.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            duid_array.push(row.duid);
            genconid_effectivedate_array
                .push(row.genconid_effectivedate.map(|val| val.timestamp_millis()));
            genconid_versionno_array.push({
                row.genconid_versionno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lhs_array.push({
                row.lhs.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    constraintid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(dispatchinterval_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rhs_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginalvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(violationdegree_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(genconid_effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(genconid_versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lhs_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCHINTERCONNECTORRES
///  _DISPATCHINTERCONNECTORRES sets out MW flow and losses on each interconnector for each dispatch period, including fields for the Frequency Controlled Ancillary Services export and import limits and extra reporting of the generic constraints set the energy import and export limits._
///
/// * Data Set Name: Dispatch
/// * File Name: Interconnectorres
/// * Data Version: 3
///
/// # Description
///  DISPATCHINTERCONNECTORRES is public data, and is available to all participants. Source DISPATCHINTERCONNECTORRES updates every 5 minutes. Note MW losses can be negative depending on the flow. The definition of direction of flow for an interconnector is that positive flow starts from the FROMREGION in the INTERCONNECTOR table.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DISPATCHINTERVAL
/// * INTERCONNECTORID
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchInterconnectorres3 {
    /// Market date starting at 04:05
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Interconnector identifier
    pub interconnectorid: String,
    /// Dispatch period identifier, from 001 to 288 in format YYYYMMDDPPP.
    #[serde(with = "crate::dispatch_period")]
    pub dispatchinterval: crate::DispatchPeriod,
    /// Intervention case or not
    pub intervention: rust_decimal::Decimal,
    /// Metered MW Flow from SCADA.
    pub meteredmwflow: Option<rust_decimal::Decimal>,
    /// Target MW Flow for next 5 mins.
    pub mwflow: Option<rust_decimal::Decimal>,
    /// Calculated MW Losses
    pub mwlosses: Option<rust_decimal::Decimal>,
    /// Shadow price resulting from thermal or reserve sharing constraints on Interconnector import/export (0 unless binding) - NEMDE Solution InterconnectorSolution element "Price" attribute
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Degree of violation on interconnector constraints
    pub violationdegree: Option<rust_decimal::Decimal>,
    /// Last changed.
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Calculated export limit applying to energy only.
    pub exportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit applying to energy only.
    pub importlimit: Option<rust_decimal::Decimal>,
    /// Marginal loss factor. Use this to adjust prices between regions.
    pub marginalloss: Option<rust_decimal::Decimal>,
    /// Generic Constraint setting the export limit
    pub exportgenconid: Option<String>,
    /// Generic Constraint setting the import limit
    pub importgenconid: Option<String>,
    /// Calculated export limit applying to energy + FCAS.
    pub fcasexportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit applying to energy + FCAS.
    pub fcasimportlimit: Option<rust_decimal::Decimal>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Export (Factor &gt;= 0)
    pub local_price_adjustment_export: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Export: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained_export: Option<rust_decimal::Decimal>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Import (Factor &gt;= 0)
    pub local_price_adjustment_import: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Import: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained_import: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DispatchInterconnectorres3 {
    type PrimaryKey = DispatchInterconnectorres3PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("INTERCONNECTORRES".into()),
            version: 3,
        }
    }

    fn primary_key(&self) -> DispatchInterconnectorres3PrimaryKey {
        DispatchInterconnectorres3PrimaryKey {
            dispatchinterval: self.dispatchinterval,
            interconnectorid: self.interconnectorid.clone(),
            intervention: self.intervention,
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "dispatch_interconnectorres_v3_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DispatchInterconnectorres3 {
    type Row = DispatchInterconnectorres3;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.dispatchinterval == row.dispatchinterval
            && self.interconnectorid == row.interconnectorid
            && self.intervention == row.intervention
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchInterconnectorres3 {
    type PrimaryKey = DispatchInterconnectorres3PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.dispatchinterval == key.dispatchinterval
            && self.interconnectorid == key.interconnectorid
            && self.intervention == key.intervention
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchInterconnectorres3PrimaryKey {
    pub dispatchinterval: crate::DispatchPeriod,
    pub interconnectorid: String,
    pub intervention: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DispatchInterconnectorres3PrimaryKey {
    type Row = DispatchInterconnectorres3;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.dispatchinterval == row.dispatchinterval
            && self.interconnectorid == row.interconnectorid
            && self.intervention == row.intervention
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchInterconnectorres3PrimaryKey {
    type PrimaryKey = DispatchInterconnectorres3PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.dispatchinterval == key.dispatchinterval
            && self.interconnectorid == key.interconnectorid
            && self.intervention == key.intervention
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for DispatchInterconnectorres3PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchInterconnectorres3 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "dispatchinterval",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "meteredmwflow",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwlosses",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "marginalvalue",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "violationdegree",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "exportlimit",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "importlimit",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "marginalloss",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "exportgenconid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "importgenconid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "fcasexportlimit",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "fcasimportlimit",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "local_price_adjustment_export",
                arrow2::datatypes::DataType::Decimal(10, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "locally_constrained_export",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "local_price_adjustment_import",
                arrow2::datatypes::DataType::Decimal(10, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "locally_constrained_import",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut dispatchinterval_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut meteredmwflow_array = Vec::new();
        let mut mwflow_array = Vec::new();
        let mut mwlosses_array = Vec::new();
        let mut marginalvalue_array = Vec::new();
        let mut violationdegree_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut exportlimit_array = Vec::new();
        let mut importlimit_array = Vec::new();
        let mut marginalloss_array = Vec::new();
        let mut exportgenconid_array = Vec::new();
        let mut importgenconid_array = Vec::new();
        let mut fcasexportlimit_array = Vec::new();
        let mut fcasimportlimit_array = Vec::new();
        let mut local_price_adjustment_export_array = Vec::new();
        let mut locally_constrained_export_array = Vec::new();
        let mut local_price_adjustment_import_array = Vec::new();
        let mut locally_constrained_import_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
            interconnectorid_array.push(row.interconnectorid);
            dispatchinterval_array.push(row.dispatchinterval.start().timestamp_millis());
            intervention_array.push({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
            meteredmwflow_array.push({
                row.meteredmwflow.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow_array.push({
                row.mwflow.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwlosses_array.push({
                row.mwlosses.map(|mut val| {
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
            violationdegree_array.push({
                row.violationdegree.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            exportlimit_array.push({
                row.exportlimit.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            importlimit_array.push({
                row.importlimit.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            marginalloss_array.push({
                row.marginalloss.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            exportgenconid_array.push(row.exportgenconid);
            importgenconid_array.push(row.importgenconid);
            fcasexportlimit_array.push({
                row.fcasexportlimit.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            fcasimportlimit_array.push({
                row.fcasimportlimit.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            local_price_adjustment_export_array.push({
                row.local_price_adjustment_export.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            locally_constrained_export_array.push({
                row.locally_constrained_export.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            local_price_adjustment_import_array.push({
                row.local_price_adjustment_import.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            locally_constrained_import_array.push({
                row.locally_constrained_import.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(dispatchinterval_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(meteredmwflow_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwlosses_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginalvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(violationdegree_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(exportlimit_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(importlimit_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginalloss_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(exportgenconid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(importgenconid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(fcasexportlimit_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(fcasimportlimit_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(local_price_adjustment_export_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(locally_constrained_export_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(local_price_adjustment_import_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(locally_constrained_import_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCHLOAD
///  _DISPATCHLOAD set out the current SCADA MW and target MW for each dispatchable unit, including relevant Frequency Control Ancillary Services (FCAS) enabling targets for each five minutes and additional fields to handle the new Ancillary Services functionality. Fast Start Plant status is indicated by dispatch mode._
///
/// * Data Set Name: Dispatch
/// * File Name: Unit Solution
/// * Data Version: 3
///
/// # Description
///  DISPATCHLOAD data is confidential for the current day, showing own details for participant and becomes public after close of business yesterday, and is available to all participants. Source DISPATCHLOAD shows data for every 5 minutes for all units, even zero targets. Volume Expect 40-50,000 records per day. All units are repeated, even zero targets. Note ** A flag exists for each ancillary service type such that a unit trapped or stranded in one or more service type can be immediately identified. The flag is defined using the low 3 bits as follows: Flag Name Bit Description Enabled 0 The unit is enabled to provide this ancillary service type. Trapped 1 The unit is enabled to provide this ancillary service type, however the profile for this service type is causing the unit to be trapped in the energy market. Stranded 2 The unit is bid available to provide this ancillary service type, however, the unit is operating in the energy market outside of the profile for this service type and is stranded from providing this service. Interpretation of the bit-flags as a number gives the following possibilities (i.e. other combinations are not possible): Numeric Value Bit (2,1,0) Meaning 0 000 Not stranded, not trapped, not enabled. 1 001 Not stranded, not trapped, is enabled. 3 011 Not stranded, is trapped, is enabled. 4 100 Is stranded, not trapped, not enabled. For example, testing for availability can be done by checking for odd (=available) or even (=unavailable) number (e.g.  mod(flag,2)  results in 0 for unavailable and 1 for available). *** "Actual FCAS availability" is determined in a post-processing step based on the energy target (TotalCleared) and bid FCAS trapezium for that interval. However, if the unit is outside the bid FCAS trapezium at the start of the interval (InitialMW), the "Actual FCAS availability" is set to zero. For regulation services, the trapezium is the most restrictive of the bid/SCADA trapezium values.
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * DUID
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchUnitSolution3 {
    /// Market date and time starting at 04:05
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Dispatchable unit identifier
    pub duid: String,
    /// Not used
    pub tradetype: Option<rust_decimal::Decimal>,
    /// Dispatch period identifier, from 001 to 288 in format YYYYMMDDPPP.
    #[serde(with = "crate::dispatch_period")]
    pub dispatchinterval: crate::DispatchPeriod,
    /// Intervention flag if intervention run
    pub intervention: rust_decimal::Decimal,
    /// Connection point identifier for DUID
    pub connectionpointid: Option<String>,
    /// Dispatch mode for fast start plant (0 to 4).
    pub dispatchmode: Option<rust_decimal::Decimal>,
    /// AGC Status from EMS<br>* 1 = on<br>* 0 = off
    pub agcstatus: Option<rust_decimal::Decimal>,
    /// Initial MW at start of period
    pub initialmw: Option<rust_decimal::Decimal>,
    /// Target MW for end of period
    pub totalcleared: Option<rust_decimal::Decimal>,
    /// Ramp down rate used in dispatch (lesser of bid or telemetered rate).
    pub rampdownrate: Option<rust_decimal::Decimal>,
    /// Ramp up rate (lesser of bid or telemetered rate).
    pub rampuprate: Option<rust_decimal::Decimal>,
    /// Lower 5 min reserve target
    pub lower5min: Option<rust_decimal::Decimal>,
    /// Lower 60 sec reserve target
    pub lower60sec: Option<rust_decimal::Decimal>,
    /// Lower 6 sec reserve target
    pub lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 5 min reserve target
    pub raise5min: Option<rust_decimal::Decimal>,
    /// Raise 60 sec reserve target
    pub raise60sec: Option<rust_decimal::Decimal>,
    /// Raise 6 sec reserve target
    pub raise6sec: Option<rust_decimal::Decimal>,
    /// Not Used
    pub downepf: Option<rust_decimal::Decimal>,
    /// Not Used
    pub upepf: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 5 min
    pub marginal5minvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 60 seconds
    pub marginal60secvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 6 seconds
    pub marginal6secvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for energy
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Violation MW 5 min
    pub violation5mindegree: Option<rust_decimal::Decimal>,
    /// Violation MW 60 seconds
    pub violation60secdegree: Option<rust_decimal::Decimal>,
    /// Violation MW 6 seconds
    pub violation6secdegree: Option<rust_decimal::Decimal>,
    /// Violation MW energy
    pub violationdegree: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Lower Regulation reserve target
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve target
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Bid energy availability
    pub availability: Option<rust_decimal::Decimal>,
    /// Raise 6sec status flag  - see
    pub raise6secflags: Option<rust_decimal::Decimal>,
    /// Raise 60sec status flag  - see
    pub raise60secflags: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise5minflags: Option<rust_decimal::Decimal>,
    /// Raise Reg status flag  - see
    pub raiseregflags: Option<rust_decimal::Decimal>,
    /// Lower 6sec status flag  - see
    pub lower6secflags: Option<rust_decimal::Decimal>,
    /// Lower 60sec status flag  
    pub lower60secflags: Option<rust_decimal::Decimal>,
    /// Lower 5min status flag  
    pub lower5minflags: Option<rust_decimal::Decimal>,
    /// Lower Reg status flag  - see
    pub lowerregflags: Option<rust_decimal::Decimal>,
    /// RaiseReg availability - minimum of bid and telemetered value
    pub raiseregavailability: Option<rust_decimal::Decimal>,
    /// RaiseReg enablement max point - minimum of bid and telemetered value
    pub raiseregenablementmax: Option<rust_decimal::Decimal>,
    /// RaiseReg Enablement Min point - maximum of bid and telemetered value
    pub raiseregenablementmin: Option<rust_decimal::Decimal>,
    /// Lower Reg availability - minimum of bid and telemetered value
    pub lowerregavailability: Option<rust_decimal::Decimal>,
    /// Lower Reg enablement Max point - minimum of bid and telemetered value
    pub lowerregenablementmax: Option<rust_decimal::Decimal>,
    /// Lower Reg Enablement Min point - maximum of bid and telemetered value
    pub lowerregenablementmin: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 6sec availability
    pub raise6secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 60sec availability
    pub raise60secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 5min availability
    pub raise5minactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise reg availability
    pub raiseregactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 6sec availability
    pub lower6secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 60sec availability
    pub lower60secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 5min availability
    pub lower5minactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower reg availability
    pub lowerregactualavailability: Option<rust_decimal::Decimal>,
    /// Boolean representation flagging if the Target is Capped
    pub semidispatchcap: Option<rust_decimal::Decimal>,
    /// Minutes for which the unit has been in the current DISPATCHMODE. From NEMDE TRADERSOLUTION element FSTARGETMODETIME attribute.
    pub dispatchmodetime: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DispatchUnitSolution3 {
    type PrimaryKey = DispatchUnitSolution3PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("UNIT_SOLUTION".into()),
            version: 3,
        }
    }

    fn primary_key(&self) -> DispatchUnitSolution3PrimaryKey {
        DispatchUnitSolution3PrimaryKey {
            duid: self.duid.clone(),
            intervention: self.intervention,
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "dispatch_unit_solution_v3_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DispatchUnitSolution3 {
    type Row = DispatchUnitSolution3;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.intervention == row.intervention
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchUnitSolution3 {
    type PrimaryKey = DispatchUnitSolution3PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.intervention == key.intervention
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchUnitSolution3PrimaryKey {
    pub duid: String,
    pub intervention: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DispatchUnitSolution3PrimaryKey {
    type Row = DispatchUnitSolution3;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.intervention == row.intervention
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchUnitSolution3PrimaryKey {
    type PrimaryKey = DispatchUnitSolution3PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.intervention == key.intervention
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for DispatchUnitSolution3PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchUnitSolution3 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "tradetype",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "dispatchinterval",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "dispatchmode",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "agcstatus",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "initialmw",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "totalcleared",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rampdownrate",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rampuprate",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5min",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60sec",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6sec",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5min",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60sec",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6sec",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "downepf",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "upepf",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "marginal5minvalue",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "marginal60secvalue",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "marginal6secvalue",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "marginalvalue",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "violation5mindegree",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "violation60secdegree",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "violation6secdegree",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "violationdegree",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "lowerreg",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raisereg",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "availability",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6secflags",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secflags",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minflags",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raiseregflags",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secflags",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secflags",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minflags",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerregflags",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raiseregavailability",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raiseregenablementmax",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raiseregenablementmin",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerregavailability",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerregenablementmax",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerregenablementmin",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6secactualavailability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secactualavailability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minactualavailability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raiseregactualavailability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secactualavailability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secactualavailability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minactualavailability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerregactualavailability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "semidispatchcap",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "dispatchmodetime",
                arrow2::datatypes::DataType::Decimal(4, 0),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut tradetype_array = Vec::new();
        let mut dispatchinterval_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut dispatchmode_array = Vec::new();
        let mut agcstatus_array = Vec::new();
        let mut initialmw_array = Vec::new();
        let mut totalcleared_array = Vec::new();
        let mut rampdownrate_array = Vec::new();
        let mut rampuprate_array = Vec::new();
        let mut lower5min_array = Vec::new();
        let mut lower60sec_array = Vec::new();
        let mut lower6sec_array = Vec::new();
        let mut raise5min_array = Vec::new();
        let mut raise60sec_array = Vec::new();
        let mut raise6sec_array = Vec::new();
        let mut downepf_array = Vec::new();
        let mut upepf_array = Vec::new();
        let mut marginal5minvalue_array = Vec::new();
        let mut marginal60secvalue_array = Vec::new();
        let mut marginal6secvalue_array = Vec::new();
        let mut marginalvalue_array = Vec::new();
        let mut violation5mindegree_array = Vec::new();
        let mut violation60secdegree_array = Vec::new();
        let mut violation6secdegree_array = Vec::new();
        let mut violationdegree_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut lowerreg_array = Vec::new();
        let mut raisereg_array = Vec::new();
        let mut availability_array = Vec::new();
        let mut raise6secflags_array = Vec::new();
        let mut raise60secflags_array = Vec::new();
        let mut raise5minflags_array = Vec::new();
        let mut raiseregflags_array = Vec::new();
        let mut lower6secflags_array = Vec::new();
        let mut lower60secflags_array = Vec::new();
        let mut lower5minflags_array = Vec::new();
        let mut lowerregflags_array = Vec::new();
        let mut raiseregavailability_array = Vec::new();
        let mut raiseregenablementmax_array = Vec::new();
        let mut raiseregenablementmin_array = Vec::new();
        let mut lowerregavailability_array = Vec::new();
        let mut lowerregenablementmax_array = Vec::new();
        let mut lowerregenablementmin_array = Vec::new();
        let mut raise6secactualavailability_array = Vec::new();
        let mut raise60secactualavailability_array = Vec::new();
        let mut raise5minactualavailability_array = Vec::new();
        let mut raiseregactualavailability_array = Vec::new();
        let mut lower6secactualavailability_array = Vec::new();
        let mut lower60secactualavailability_array = Vec::new();
        let mut lower5minactualavailability_array = Vec::new();
        let mut lowerregactualavailability_array = Vec::new();
        let mut semidispatchcap_array = Vec::new();
        let mut dispatchmodetime_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
            duid_array.push(row.duid);
            tradetype_array.push({
                row.tradetype.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            dispatchinterval_array.push(row.dispatchinterval.start().timestamp_millis());
            intervention_array.push({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
            connectionpointid_array.push(row.connectionpointid);
            dispatchmode_array.push({
                row.dispatchmode.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            agcstatus_array.push({
                row.agcstatus.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            initialmw_array.push({
                row.initialmw.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            totalcleared_array.push({
                row.totalcleared.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rampdownrate_array.push({
                row.rampdownrate.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rampuprate_array.push({
                row.rampuprate.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower5min_array.push({
                row.lower5min.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower60sec_array.push({
                row.lower60sec.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower6sec_array.push({
                row.lower6sec.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise5min_array.push({
                row.raise5min.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise60sec_array.push({
                row.raise60sec.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise6sec_array.push({
                row.raise6sec.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            downepf_array.push({
                row.downepf.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            upepf_array.push({
                row.upepf.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            marginal5minvalue_array.push({
                row.marginal5minvalue.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            marginal60secvalue_array.push({
                row.marginal60secvalue.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            marginal6secvalue_array.push({
                row.marginal6secvalue.map(|mut val| {
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
            violation5mindegree_array.push({
                row.violation5mindegree.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            violation60secdegree_array.push({
                row.violation60secdegree.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            violation6secdegree_array.push({
                row.violation6secdegree.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            violationdegree_array.push({
                row.violationdegree.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            lowerreg_array.push({
                row.lowerreg.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raisereg_array.push({
                row.raisereg.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            availability_array.push({
                row.availability.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise6secflags_array.push({
                row.raise6secflags.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            raise60secflags_array.push({
                row.raise60secflags.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            raise5minflags_array.push({
                row.raise5minflags.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            raiseregflags_array.push({
                row.raiseregflags.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lower6secflags_array.push({
                row.lower6secflags.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lower60secflags_array.push({
                row.lower60secflags.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lower5minflags_array.push({
                row.lower5minflags.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lowerregflags_array.push({
                row.lowerregflags.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            raiseregavailability_array.push({
                row.raiseregavailability.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raiseregenablementmax_array.push({
                row.raiseregenablementmax.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raiseregenablementmin_array.push({
                row.raiseregenablementmin.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lowerregavailability_array.push({
                row.lowerregavailability.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lowerregenablementmax_array.push({
                row.lowerregenablementmax.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lowerregenablementmin_array.push({
                row.lowerregenablementmin.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise6secactualavailability_array.push({
                row.raise6secactualavailability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            raise60secactualavailability_array.push({
                row.raise60secactualavailability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            raise5minactualavailability_array.push({
                row.raise5minactualavailability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            raiseregactualavailability_array.push({
                row.raiseregactualavailability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lower6secactualavailability_array.push({
                row.lower6secactualavailability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lower60secactualavailability_array.push({
                row.lower60secactualavailability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lower5minactualavailability_array.push({
                row.lower5minactualavailability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lowerregactualavailability_array.push({
                row.lowerregactualavailability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            semidispatchcap_array.push({
                row.semidispatchcap.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            dispatchmodetime_array.push({
                row.dispatchmodetime.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(tradetype_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(dispatchinterval_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    connectionpointid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(dispatchmode_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(agcstatus_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(initialmw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalcleared_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rampdownrate_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rampuprate_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5min_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60sec_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6sec_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5min_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60sec_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6sec_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(downepf_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(upepf_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginal5minvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginal60secvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginal6secvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginalvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(violation5mindegree_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(violation60secdegree_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(violation6secdegree_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(violationdegree_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerreg_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raisereg_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(availability_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secflags_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secflags_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minflags_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregflags_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secflags_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secflags_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minflags_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregflags_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregenablementmax_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregenablementmin_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregenablementmax_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregenablementmin_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(semidispatchcap_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(dispatchmodetime_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCHOFFERTRK
///  _DISPATCHOFFERTRK is the energy and ancillary service bid tracking table for the Dispatch process. The table identifies which bids from BIDDAYOFFER and BIDOFFERPERIOD were applied for a given unit and bid type for each dispatch interval._
///
/// * Data Set Name: Dispatch
/// * File Name: Offertrk
/// * Data Version: 1
///
/// # Description
///  DISPATCHOFFERTRK  data is confidential to each participant until the next trading day, when the data is public to all participants.  Source DISPATCHOFFERTRK updates every 5 minutes. Volume Approximately 250,000 records per day.
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DUID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchOffertrk1 {
    /// Date and time of the dispatch interval (e.g. five minute dispatch interval ending 28/09/2000 16:35)
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: String,
    /// Bid type Identifier - the ancillary service to which the bid applies
    pub bidtype: String,
    /// Settlement date of bid applied
    #[serde(with = "crate::mms_datetime_opt")]
    pub bidsettlementdate: Option<chrono::NaiveDateTime>,
    /// Time this bid was processed and loaded
    #[serde(with = "crate::mms_datetime_opt")]
    pub bidofferdate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for DispatchOffertrk1 {
    type PrimaryKey = DispatchOffertrk1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("OFFERTRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DispatchOffertrk1PrimaryKey {
        DispatchOffertrk1PrimaryKey {
            bidtype: self.bidtype.clone(),
            duid: self.duid.clone(),
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "dispatch_offertrk_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DispatchOffertrk1 {
    type Row = DispatchOffertrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.duid == row.duid
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchOffertrk1 {
    type PrimaryKey = DispatchOffertrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.duid == key.duid
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchOffertrk1PrimaryKey {
    pub bidtype: String,
    pub duid: String,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DispatchOffertrk1PrimaryKey {
    type Row = DispatchOffertrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.duid == row.duid
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchOffertrk1PrimaryKey {
    type PrimaryKey = DispatchOffertrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.duid == key.duid
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for DispatchOffertrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchOffertrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("bidtype", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "bidsettlementdate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "bidofferdate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut bidtype_array = Vec::new();
        let mut bidsettlementdate_array = Vec::new();
        let mut bidofferdate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            duid_array.push(row.duid);
            bidtype_array.push(row.bidtype);
            bidsettlementdate_array.push(row.bidsettlementdate.map(|val| val.timestamp_millis()));
            bidofferdate_array.push(row.bidofferdate.map(|val| val.timestamp_millis()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bidtype_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bidsettlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bidofferdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCHPRICE
///  _DISPATCHPRICE records 5 minute dispatch prices for energy and FCAS, including whether an intervention has occurred, or price override (e.g. for Administered Price Cap). DISPATCHPRICE updates when price adjustments occur, in which case the new price is written to the RRP field, and the old price to the ROP field as an audit trail._
///
/// * Data Set Name: Dispatch
/// * File Name: Price
/// * Data Version: 4
///
/// # Description
///  Source DISPATCHPRICE updates every 5 minutes. Note APCFLAG is a 5-bit Region-based field indicating that the original Dispatch Price (ROP) calculated by the Dispatch Algorithm for a region has undergone modification by one of more of the following processes: Bit Value Description 5 16 Price Scaling via Inter-regional Loss Factor (IRLF) 4 8 Price manually overwritten 3 4 MPC or MPF binding (ROP was outside of MPC/MPF) 2 2 VoLL Override applied 1 1 APC or APF binding (ROP was outside of APC/APF) Where: · MPC = Market Price Cap · MPF = Market Price Floor · APC = Administered Price Cap · APF = Administered Price Floor xxxAPCFLAGs are each a 5-bit Region-based field indicating FCAS price post-processing (where "ROP" is the original NEMDE Solver price): Bit Cum Value Description 5 16 Not applicable 4 8 Price manually overwritten 3 4 MPC ($VoLL) or MPF ($zero) binding (xxFCAS ROP was outside of MPC/MPF) 2 2 Not applicable 1 1 APC or APF binding (ROP was outside of APC/APF)
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DISPATCHINTERVAL
/// * INTERVENTION
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchPrice4 {
    /// Market date and time starting at 04:05
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Dispatch interval identifier 001 to 288 in format YYYYMMDDPPP
    #[serde(with = "crate::dispatch_period")]
    pub dispatchinterval: crate::DispatchPeriod,
    /// Manual intervention flag
    pub intervention: rust_decimal::Decimal,
    /// Regional Reference Price for this dispatch period. RRP is the price used to settle the market
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess energy price - no longer used
    pub eep: Option<rust_decimal::Decimal>,
    /// Regional Override Price, being the original price prior to any price scaling, price capping or VoLL override being applied. The APC flag allows the determination of whether capping, scaling or override occurred
    pub rop: Option<rust_decimal::Decimal>,
    /// APC Active flag (see note)
    pub apcflag: Option<rust_decimal::Decimal>,
    /// Market suspended flag
    pub marketsuspendedflag: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub raise6secrrp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise6secrop: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise6secapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise60secrrp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise60secrop: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise60secapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise5minrrp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise5minrop: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raise5minapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raiseregrrp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raiseregrop: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub raiseregapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower6secrrp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower6secrop: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower6secapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower60secrrp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower60secrop: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower60secapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower5minrrp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower5minrop: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lower5minapcflag: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lowerregrrp: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lowerregrop: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub lowerregapcflag: Option<rust_decimal::Decimal>,
    /// Status of regional prices for this dispatch interval "NOT FIRM" or "FIRM"
    pub price_status: Option<String>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_energy_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_raise6_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_raise60_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_raise5min_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_raisereg_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_lower6_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_lower60_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_lower5min_price: Option<rust_decimal::Decimal>,
    /// Price before ap capping or scaling - for rolling sum price monitoring
    pub pre_ap_lowerreg_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_energy_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_raise6_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_raise60_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_raise5min_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_raisereg_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_lower6_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_lower60_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_lower5min_price: Option<rust_decimal::Decimal>,
    /// Cumulative price that triggers administered pricing event if above the threshold
    pub cumul_pre_ap_lowerreg_price: Option<rust_decimal::Decimal>,
    /// Communicates the current OCD status for this dispatch interval.  Values of: 'NOT_OCD', 'OCD_UNRESOLVED', 'OCD_RESOLVED'.
    pub ocd_status: Option<String>,
    /// Communicates the current MII status for this dispatch interval.  Values of: 'NOT_MII', 'MII_SUBJECT_TO_REVIEW', 'MII_PRICE_REJECTED', 'MII_PRICE_ACCEPTED'.
    pub mii_status: Option<String>,
}
impl crate::GetTable for DispatchPrice4 {
    type PrimaryKey = DispatchPrice4PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("PRICE".into()),
            version: 4,
        }
    }

    fn primary_key(&self) -> DispatchPrice4PrimaryKey {
        DispatchPrice4PrimaryKey {
            dispatchinterval: self.dispatchinterval,
            intervention: self.intervention,
            regionid: self.regionid.clone(),
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "dispatch_price_v4_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DispatchPrice4 {
    type Row = DispatchPrice4;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.dispatchinterval == row.dispatchinterval
            && self.intervention == row.intervention
            && self.regionid == row.regionid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchPrice4 {
    type PrimaryKey = DispatchPrice4PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.dispatchinterval == key.dispatchinterval
            && self.intervention == key.intervention
            && self.regionid == key.regionid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchPrice4PrimaryKey {
    pub dispatchinterval: crate::DispatchPeriod,
    pub intervention: rust_decimal::Decimal,
    pub regionid: String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DispatchPrice4PrimaryKey {
    type Row = DispatchPrice4;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.dispatchinterval == row.dispatchinterval
            && self.intervention == row.intervention
            && self.regionid == row.regionid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchPrice4PrimaryKey {
    type PrimaryKey = DispatchPrice4PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.dispatchinterval == key.dispatchinterval
            && self.intervention == key.intervention
            && self.regionid == key.regionid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for DispatchPrice4PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchPrice4 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "dispatchinterval",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                false,
            ),
            arrow2::datatypes::Field::new("rrp", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new("eep", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new("rop", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new(
                "apcflag",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "marketsuspendedflag",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "raise6secrrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6secrop",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6secapcflag",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secrrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secrop",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secapcflag",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minrrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minrop",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minapcflag",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raiseregrrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raiseregrop",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raiseregapcflag",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secrrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secrop",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secapcflag",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secrrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secrop",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secapcflag",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minrrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minrop",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minapcflag",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerregrrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerregrop",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerregapcflag",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "price_status",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "pre_ap_energy_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "pre_ap_raise6_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "pre_ap_raise60_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "pre_ap_raise5min_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "pre_ap_raisereg_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "pre_ap_lower6_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "pre_ap_lower60_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "pre_ap_lower5min_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "pre_ap_lowerreg_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "cumul_pre_ap_energy_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "cumul_pre_ap_raise6_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "cumul_pre_ap_raise60_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "cumul_pre_ap_raise5min_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "cumul_pre_ap_raisereg_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "cumul_pre_ap_lower6_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "cumul_pre_ap_lower60_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "cumul_pre_ap_lower5min_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "cumul_pre_ap_lowerreg_price",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "ocd_status",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "mii_status",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut dispatchinterval_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut rrp_array = Vec::new();
        let mut eep_array = Vec::new();
        let mut rop_array = Vec::new();
        let mut apcflag_array = Vec::new();
        let mut marketsuspendedflag_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut raise6secrrp_array = Vec::new();
        let mut raise6secrop_array = Vec::new();
        let mut raise6secapcflag_array = Vec::new();
        let mut raise60secrrp_array = Vec::new();
        let mut raise60secrop_array = Vec::new();
        let mut raise60secapcflag_array = Vec::new();
        let mut raise5minrrp_array = Vec::new();
        let mut raise5minrop_array = Vec::new();
        let mut raise5minapcflag_array = Vec::new();
        let mut raiseregrrp_array = Vec::new();
        let mut raiseregrop_array = Vec::new();
        let mut raiseregapcflag_array = Vec::new();
        let mut lower6secrrp_array = Vec::new();
        let mut lower6secrop_array = Vec::new();
        let mut lower6secapcflag_array = Vec::new();
        let mut lower60secrrp_array = Vec::new();
        let mut lower60secrop_array = Vec::new();
        let mut lower60secapcflag_array = Vec::new();
        let mut lower5minrrp_array = Vec::new();
        let mut lower5minrop_array = Vec::new();
        let mut lower5minapcflag_array = Vec::new();
        let mut lowerregrrp_array = Vec::new();
        let mut lowerregrop_array = Vec::new();
        let mut lowerregapcflag_array = Vec::new();
        let mut price_status_array = Vec::new();
        let mut pre_ap_energy_price_array = Vec::new();
        let mut pre_ap_raise6_price_array = Vec::new();
        let mut pre_ap_raise60_price_array = Vec::new();
        let mut pre_ap_raise5min_price_array = Vec::new();
        let mut pre_ap_raisereg_price_array = Vec::new();
        let mut pre_ap_lower6_price_array = Vec::new();
        let mut pre_ap_lower60_price_array = Vec::new();
        let mut pre_ap_lower5min_price_array = Vec::new();
        let mut pre_ap_lowerreg_price_array = Vec::new();
        let mut cumul_pre_ap_energy_price_array = Vec::new();
        let mut cumul_pre_ap_raise6_price_array = Vec::new();
        let mut cumul_pre_ap_raise60_price_array = Vec::new();
        let mut cumul_pre_ap_raise5min_price_array = Vec::new();
        let mut cumul_pre_ap_raisereg_price_array = Vec::new();
        let mut cumul_pre_ap_lower6_price_array = Vec::new();
        let mut cumul_pre_ap_lower60_price_array = Vec::new();
        let mut cumul_pre_ap_lower5min_price_array = Vec::new();
        let mut cumul_pre_ap_lowerreg_price_array = Vec::new();
        let mut ocd_status_array = Vec::new();
        let mut mii_status_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
            regionid_array.push(row.regionid);
            dispatchinterval_array.push(row.dispatchinterval.start().timestamp_millis());
            intervention_array.push({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
            rrp_array.push({
                row.rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            eep_array.push({
                row.eep.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rop_array.push({
                row.rop.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            apcflag_array.push({
                row.apcflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            marketsuspendedflag_array.push({
                row.marketsuspendedflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            raise6secrrp_array.push({
                row.raise6secrrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise6secrop_array.push({
                row.raise6secrop.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise6secapcflag_array.push({
                row.raise6secapcflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            raise60secrrp_array.push({
                row.raise60secrrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise60secrop_array.push({
                row.raise60secrop.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise60secapcflag_array.push({
                row.raise60secapcflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            raise5minrrp_array.push({
                row.raise5minrrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise5minrop_array.push({
                row.raise5minrop.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise5minapcflag_array.push({
                row.raise5minapcflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            raiseregrrp_array.push({
                row.raiseregrrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raiseregrop_array.push({
                row.raiseregrop.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raiseregapcflag_array.push({
                row.raiseregapcflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lower6secrrp_array.push({
                row.lower6secrrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower6secrop_array.push({
                row.lower6secrop.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower6secapcflag_array.push({
                row.lower6secapcflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lower60secrrp_array.push({
                row.lower60secrrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower60secrop_array.push({
                row.lower60secrop.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower60secapcflag_array.push({
                row.lower60secapcflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lower5minrrp_array.push({
                row.lower5minrrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower5minrop_array.push({
                row.lower5minrop.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower5minapcflag_array.push({
                row.lower5minapcflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lowerregrrp_array.push({
                row.lowerregrrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lowerregrop_array.push({
                row.lowerregrop.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lowerregapcflag_array.push({
                row.lowerregapcflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            price_status_array.push(row.price_status);
            pre_ap_energy_price_array.push({
                row.pre_ap_energy_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            pre_ap_raise6_price_array.push({
                row.pre_ap_raise6_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            pre_ap_raise60_price_array.push({
                row.pre_ap_raise60_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            pre_ap_raise5min_price_array.push({
                row.pre_ap_raise5min_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            pre_ap_raisereg_price_array.push({
                row.pre_ap_raisereg_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            pre_ap_lower6_price_array.push({
                row.pre_ap_lower6_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            pre_ap_lower60_price_array.push({
                row.pre_ap_lower60_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            pre_ap_lower5min_price_array.push({
                row.pre_ap_lower5min_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            pre_ap_lowerreg_price_array.push({
                row.pre_ap_lowerreg_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            cumul_pre_ap_energy_price_array.push({
                row.cumul_pre_ap_energy_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            cumul_pre_ap_raise6_price_array.push({
                row.cumul_pre_ap_raise6_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            cumul_pre_ap_raise60_price_array.push({
                row.cumul_pre_ap_raise60_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            cumul_pre_ap_raise5min_price_array.push({
                row.cumul_pre_ap_raise5min_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            cumul_pre_ap_raisereg_price_array.push({
                row.cumul_pre_ap_raisereg_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            cumul_pre_ap_lower6_price_array.push({
                row.cumul_pre_ap_lower6_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            cumul_pre_ap_lower60_price_array.push({
                row.cumul_pre_ap_lower60_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            cumul_pre_ap_lower5min_price_array.push({
                row.cumul_pre_ap_lower5min_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            cumul_pre_ap_lowerreg_price_array.push({
                row.cumul_pre_ap_lowerreg_price.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            ocd_status_array.push(row.ocd_status);
            mii_status_array.push(row.mii_status);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(dispatchinterval_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(eep_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rop_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(apcflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marketsuspendedflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secrop_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secapcflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secrop_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secapcflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minrop_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minapcflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregrop_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregapcflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secrop_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secapcflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secrop_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secapcflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minrop_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minapcflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregrop_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregapcflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(price_status_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(pre_ap_energy_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(pre_ap_raise6_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(pre_ap_raise60_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(pre_ap_raise5min_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(pre_ap_raisereg_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(pre_ap_lower6_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(pre_ap_lower60_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(pre_ap_lower5min_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(pre_ap_lowerreg_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cumul_pre_ap_energy_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cumul_pre_ap_raise6_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cumul_pre_ap_raise60_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cumul_pre_ap_raise5min_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cumul_pre_ap_raisereg_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cumul_pre_ap_lower6_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cumul_pre_ap_lower60_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cumul_pre_ap_lower5min_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cumul_pre_ap_lowerreg_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(ocd_status_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(mii_status_array)),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCHREGIONSUM
///  _DISPATCHREGIONSUM sets out the 5-minute solution for each dispatch run for each region, including the Frequency Control Ancillary Services (FCAS) services provided. Additional fields are for the Raise Regulation and Lower Regulation Ancillary Services plus improvements to demand calculations._
///
/// * Data Set Name: Dispatch
/// * File Name: Regionsum
/// * Data Version: 6
///
/// # Description
///  DISPATCHREGIONSUM is public data, and is available to all participants. Source DISPATCHREGIONSUM updates every 5 minutes. Note For details of calculations about load calculations, refer to Chapter 3 of the "Statement of Opportunities" *** "Actual FCAS availability" is determined in a post-processing step based on the energy target (TotalCleared) and bid FCAS trapezium for that interval. However, if the unit is outside the bid FCAS trapezium at the start of the interval (InitialMW), the "Actual FCAS availability" is set to zero. For regulation services, the trapezium is the most restrictive of the bid/SCADA trapezium values. From 16 February 2006, the old reserve values are no longer populated (i.e. are null), being LORSurplus and LRCSurplus. For more details on the changes to Reporting of Reserve Condition Data, refer to AEMO Communication 2042. For the best available indicator of reserve condition in each of the regions of the NEM for each trading interval, refer to the latest run of the Pre-Dispatch PASA (see table PDPASA_REGIONSOLUTION).
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DISPATCHINTERVAL
/// * INTERVENTION
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchRegionsum6 {
    /// Market date and time starting at 04:05
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Dispatch period identifier, from 001 to 288 in format YYYYMMDDPPP.
    #[serde(with = "crate::dispatch_period")]
    pub dispatchinterval: crate::DispatchPeriod,
    /// Manual Intervention flag
    pub intervention: rust_decimal::Decimal,
    /// Demand (less loads)
    pub totaldemand: Option<rust_decimal::Decimal>,
    /// Aggregate generation bid available in region
    pub availablegeneration: Option<rust_decimal::Decimal>,
    /// Aggregate load bid available in region
    pub availableload: Option<rust_decimal::Decimal>,
    /// 5 minute forecast adjust
    pub demandforecast: Option<rust_decimal::Decimal>,
    /// Dispatched Generation
    pub dispatchablegeneration: Option<rust_decimal::Decimal>,
    /// Dispatched Load (add to total demand to get inherent region demand).
    pub dispatchableload: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the regional reference node
    pub netinterchange: Option<rust_decimal::Decimal>,
    /// MW quantity of excess
    pub excessgeneration: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW dispatch
    pub lower5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW imported
    pub lower5minimport: Option<rust_decimal::Decimal>,
    /// Lower 5 min local dispatch
    pub lower5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 5 min
    pub lower5minlocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min local requirement
    pub lower5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 5 min
    pub lower5minprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min total requirement
    pub lower5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 5 min
    pub lower5minsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW dispatch
    pub lower60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW imported
    pub lower60secimport: Option<rust_decimal::Decimal>,
    /// Lower 60 sec local dispatch
    pub lower60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 60 sec
    pub lower60seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec local requirement
    pub lower60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 60 sec
    pub lower60secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec total requirement
    pub lower60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 60 sec
    pub lower60secsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW dispatch
    pub lower6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW imported
    pub lower6secimport: Option<rust_decimal::Decimal>,
    /// Lower 6 sec local dispatch
    pub lower6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of lower 6 sec
    pub lower6seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec local requirement
    pub lower6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of lower 6 sec
    pub lower6secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec total requirement
    pub lower6secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of lower 6 sec
    pub lower6secsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min MW dispatch
    pub raise5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min MW imported
    pub raise5minimport: Option<rust_decimal::Decimal>,
    /// Raise 5 min local dispatch
    pub raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise price of lower 5 min
    pub raise5minlocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min local requirement
    pub raise5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of raise 5 min
    pub raise5minprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min total requirement
    pub raise5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of raise 5 min
    pub raise5minsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW dispatch
    pub raise60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW imported
    pub raise60secimport: Option<rust_decimal::Decimal>,
    /// Raise 60 sec local dispatch
    pub raise60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of raise 60 sec
    pub raise60seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec local requirement
    pub raise60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of raise 60 sec
    pub raise60secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec total requirement
    pub raise60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of raise 60 sec
    pub raise60secsupplyprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec MW dispatch
    pub raise6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec MW imported
    pub raise6secimport: Option<rust_decimal::Decimal>,
    /// Raise 6 sec local dispatch
    pub raise6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of raise 6 sec
    pub raise6seclocalprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec local requirement
    pub raise6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Regional price of raise 6 sec
    pub raise6secprice: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec total requirement
    pub raise6secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Supply price of raise 6 sec
    pub raise6secsupplyprice: Option<rust_decimal::Decimal>,
    /// Calculated dispatch error
    pub aggegatedispatcherror: Option<rust_decimal::Decimal>,
    /// Calculated dispatch error
    pub aggregatedispatcherror: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Sum of initial generation and import for region
    pub initialsupply: Option<rust_decimal::Decimal>,
    /// Sum of cleared generation and import for region
    pub clearedsupply: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation MW imported
    pub lowerregimport: Option<rust_decimal::Decimal>,
    /// Lower Regulation local dispatch
    pub lowerreglocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation local requirement
    pub lowerreglocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation total requirement
    pub lowerregreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation MW imported
    pub raiseregimport: Option<rust_decimal::Decimal>,
    /// Raise Regulation local dispatch
    pub raisereglocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation local requirement
    pub raisereglocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation total requirement
    pub raiseregreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 5 min local requirement
    pub raise5minlocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise Reg local requirement
    pub raisereglocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 60 sec local requirement
    pub raise60seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 6 sec local requirement
    pub raise6seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 5 min local requirement
    pub lower5minlocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower Reg local requirement
    pub lowerreglocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 60 sec local requirement
    pub lower60seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 6 sec local requirement
    pub lower6seclocalviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 5 min requirement
    pub raise5minviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise Reg requirement
    pub raiseregviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 60 seconds requirement
    pub raise60secviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Raise 6 seconds requirement
    pub raise6secviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 5 min requirement
    pub lower5minviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower Reg requirement
    pub lowerregviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 60 seconds requirement
    pub lower60secviolation: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Violation (MW) of Lower 6 seconds requirement
    pub lower6secviolation: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 6sec availability
    pub raise6secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 60sec availability
    pub raise60secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise 5min availability
    pub raise5minactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted raise reg availability
    pub raiseregactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 6sec availability
    pub lower6secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 60sec availability
    pub lower60secactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower 5min availability
    pub lower5minactualavailability: Option<rust_decimal::Decimal>,
    /// trapezium adjusted lower reg availability
    pub lowerregactualavailability: Option<rust_decimal::Decimal>,
    /// Not in use after 17 Feb 2006. Total short term generation capacity reserve used in assessing lack of reserve condition
    pub lorsurplus: Option<rust_decimal::Decimal>,
    /// Not in use after 17 Feb 2006. Total short term generation capacity reserve above the stated low reserve condition requirement
    pub lrcsurplus: Option<rust_decimal::Decimal>,
    /// Allowance made for non-scheduled generation in the demand forecast (MW).
    pub totalintermittentgeneration: Option<rust_decimal::Decimal>,
    /// Sum of Cleared Scheduled generation, imported generation (at the region boundary) and allowances made for non-scheduled generation (MW).
    pub demand_and_nonschedgen: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW).
    pub uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW
    pub semischedule_clearedmw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where Semi-Dispatch cap is enforced
    pub semischedule_compliancemw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW) where the primary fuel source is solar
    pub ss_solar_uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW) where the primary fuel source is wind
    pub ss_wind_uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where the primary fuel source is solar
    pub ss_solar_clearedmw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where the primary fuel source is wind
    pub ss_wind_clearedmw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where Semi-Dispatch cap is enforced and the primary fuel source is solar
    pub ss_solar_compliancemw: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-Schedule generator Cleared MW where Semi-Dispatch cap is enforced and the primary fuel source is wind
    pub ss_wind_compliancemw: Option<rust_decimal::Decimal>,
    /// Regional aggregated MW value at start of interval for Wholesale Demand Response (WDR) units
    pub wdr_initialmw: Option<rust_decimal::Decimal>,
    /// Regional aggregated available MW for Wholesale Demand Response (WDR) units
    pub wdr_available: Option<rust_decimal::Decimal>,
    /// Regional aggregated dispatched MW for Wholesale Demand Response (WDR) units
    pub wdr_dispatched: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DispatchRegionsum6 {
    type PrimaryKey = DispatchRegionsum6PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("REGIONSUM".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> DispatchRegionsum6PrimaryKey {
        DispatchRegionsum6PrimaryKey {
            dispatchinterval: self.dispatchinterval,
            intervention: self.intervention,
            regionid: self.regionid.clone(),
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "dispatch_regionsum_v6_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DispatchRegionsum6 {
    type Row = DispatchRegionsum6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.dispatchinterval == row.dispatchinterval
            && self.intervention == row.intervention
            && self.regionid == row.regionid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchRegionsum6 {
    type PrimaryKey = DispatchRegionsum6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.dispatchinterval == key.dispatchinterval
            && self.intervention == key.intervention
            && self.regionid == key.regionid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchRegionsum6PrimaryKey {
    pub dispatchinterval: crate::DispatchPeriod,
    pub intervention: rust_decimal::Decimal,
    pub regionid: String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DispatchRegionsum6PrimaryKey {
    type Row = DispatchRegionsum6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.dispatchinterval == row.dispatchinterval
            && self.intervention == row.intervention
            && self.regionid == row.regionid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchRegionsum6PrimaryKey {
    type PrimaryKey = DispatchRegionsum6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.dispatchinterval == key.dispatchinterval
            && self.intervention == key.intervention
            && self.regionid == key.regionid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for DispatchRegionsum6PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchRegionsum6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "dispatchinterval",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "totaldemand",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "availablegeneration",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "availableload",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "demandforecast",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "dispatchablegeneration",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "dispatchableload",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "netinterchange",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "excessgeneration",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5mindispatch",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minimport",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minlocaldispatch",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minlocalprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minlocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minsupplyprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secdispatch",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secimport",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60seclocaldispatch",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60seclocalprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60seclocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secsupplyprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secdispatch",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secimport",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6seclocaldispatch",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6seclocalprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6seclocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secsupplyprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5mindispatch",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minimport",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minlocaldispatch",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minlocalprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minlocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minsupplyprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secdispatch",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secimport",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60seclocaldispatch",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60seclocalprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60seclocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secsupplyprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6secdispatch",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6secimport",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6seclocaldispatch",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6seclocalprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6seclocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6secprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6secreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6secsupplyprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "aggegatedispatcherror",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "aggregatedispatcherror",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "initialsupply",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "clearedsupply",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerregimport",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerreglocaldispatch",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerreglocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerregreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raiseregimport",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raisereglocaldispatch",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raisereglocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raiseregreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minlocalviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raisereglocalviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60seclocalviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6seclocalviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minlocalviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerreglocalviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60seclocalviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6seclocalviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raiseregviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6secviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerregviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6secactualavailability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secactualavailability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minactualavailability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raiseregactualavailability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secactualavailability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secactualavailability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minactualavailability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerregactualavailability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lorsurplus",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lrcsurplus",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "totalintermittentgeneration",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "demand_and_nonschedgen",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "uigf",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "semischedule_clearedmw",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "semischedule_compliancemw",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "ss_solar_uigf",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "ss_wind_uigf",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "ss_solar_clearedmw",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "ss_wind_clearedmw",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "ss_solar_compliancemw",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "ss_wind_compliancemw",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "wdr_initialmw",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "wdr_available",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "wdr_dispatched",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut dispatchinterval_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut totaldemand_array = Vec::new();
        let mut availablegeneration_array = Vec::new();
        let mut availableload_array = Vec::new();
        let mut demandforecast_array = Vec::new();
        let mut dispatchablegeneration_array = Vec::new();
        let mut dispatchableload_array = Vec::new();
        let mut netinterchange_array = Vec::new();
        let mut excessgeneration_array = Vec::new();
        let mut lower5mindispatch_array = Vec::new();
        let mut lower5minimport_array = Vec::new();
        let mut lower5minlocaldispatch_array = Vec::new();
        let mut lower5minlocalprice_array = Vec::new();
        let mut lower5minlocalreq_array = Vec::new();
        let mut lower5minprice_array = Vec::new();
        let mut lower5minreq_array = Vec::new();
        let mut lower5minsupplyprice_array = Vec::new();
        let mut lower60secdispatch_array = Vec::new();
        let mut lower60secimport_array = Vec::new();
        let mut lower60seclocaldispatch_array = Vec::new();
        let mut lower60seclocalprice_array = Vec::new();
        let mut lower60seclocalreq_array = Vec::new();
        let mut lower60secprice_array = Vec::new();
        let mut lower60secreq_array = Vec::new();
        let mut lower60secsupplyprice_array = Vec::new();
        let mut lower6secdispatch_array = Vec::new();
        let mut lower6secimport_array = Vec::new();
        let mut lower6seclocaldispatch_array = Vec::new();
        let mut lower6seclocalprice_array = Vec::new();
        let mut lower6seclocalreq_array = Vec::new();
        let mut lower6secprice_array = Vec::new();
        let mut lower6secreq_array = Vec::new();
        let mut lower6secsupplyprice_array = Vec::new();
        let mut raise5mindispatch_array = Vec::new();
        let mut raise5minimport_array = Vec::new();
        let mut raise5minlocaldispatch_array = Vec::new();
        let mut raise5minlocalprice_array = Vec::new();
        let mut raise5minlocalreq_array = Vec::new();
        let mut raise5minprice_array = Vec::new();
        let mut raise5minreq_array = Vec::new();
        let mut raise5minsupplyprice_array = Vec::new();
        let mut raise60secdispatch_array = Vec::new();
        let mut raise60secimport_array = Vec::new();
        let mut raise60seclocaldispatch_array = Vec::new();
        let mut raise60seclocalprice_array = Vec::new();
        let mut raise60seclocalreq_array = Vec::new();
        let mut raise60secprice_array = Vec::new();
        let mut raise60secreq_array = Vec::new();
        let mut raise60secsupplyprice_array = Vec::new();
        let mut raise6secdispatch_array = Vec::new();
        let mut raise6secimport_array = Vec::new();
        let mut raise6seclocaldispatch_array = Vec::new();
        let mut raise6seclocalprice_array = Vec::new();
        let mut raise6seclocalreq_array = Vec::new();
        let mut raise6secprice_array = Vec::new();
        let mut raise6secreq_array = Vec::new();
        let mut raise6secsupplyprice_array = Vec::new();
        let mut aggegatedispatcherror_array = Vec::new();
        let mut aggregatedispatcherror_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut initialsupply_array = Vec::new();
        let mut clearedsupply_array = Vec::new();
        let mut lowerregimport_array = Vec::new();
        let mut lowerreglocaldispatch_array = Vec::new();
        let mut lowerreglocalreq_array = Vec::new();
        let mut lowerregreq_array = Vec::new();
        let mut raiseregimport_array = Vec::new();
        let mut raisereglocaldispatch_array = Vec::new();
        let mut raisereglocalreq_array = Vec::new();
        let mut raiseregreq_array = Vec::new();
        let mut raise5minlocalviolation_array = Vec::new();
        let mut raisereglocalviolation_array = Vec::new();
        let mut raise60seclocalviolation_array = Vec::new();
        let mut raise6seclocalviolation_array = Vec::new();
        let mut lower5minlocalviolation_array = Vec::new();
        let mut lowerreglocalviolation_array = Vec::new();
        let mut lower60seclocalviolation_array = Vec::new();
        let mut lower6seclocalviolation_array = Vec::new();
        let mut raise5minviolation_array = Vec::new();
        let mut raiseregviolation_array = Vec::new();
        let mut raise60secviolation_array = Vec::new();
        let mut raise6secviolation_array = Vec::new();
        let mut lower5minviolation_array = Vec::new();
        let mut lowerregviolation_array = Vec::new();
        let mut lower60secviolation_array = Vec::new();
        let mut lower6secviolation_array = Vec::new();
        let mut raise6secactualavailability_array = Vec::new();
        let mut raise60secactualavailability_array = Vec::new();
        let mut raise5minactualavailability_array = Vec::new();
        let mut raiseregactualavailability_array = Vec::new();
        let mut lower6secactualavailability_array = Vec::new();
        let mut lower60secactualavailability_array = Vec::new();
        let mut lower5minactualavailability_array = Vec::new();
        let mut lowerregactualavailability_array = Vec::new();
        let mut lorsurplus_array = Vec::new();
        let mut lrcsurplus_array = Vec::new();
        let mut totalintermittentgeneration_array = Vec::new();
        let mut demand_and_nonschedgen_array = Vec::new();
        let mut uigf_array = Vec::new();
        let mut semischedule_clearedmw_array = Vec::new();
        let mut semischedule_compliancemw_array = Vec::new();
        let mut ss_solar_uigf_array = Vec::new();
        let mut ss_wind_uigf_array = Vec::new();
        let mut ss_solar_clearedmw_array = Vec::new();
        let mut ss_wind_clearedmw_array = Vec::new();
        let mut ss_solar_compliancemw_array = Vec::new();
        let mut ss_wind_compliancemw_array = Vec::new();
        let mut wdr_initialmw_array = Vec::new();
        let mut wdr_available_array = Vec::new();
        let mut wdr_dispatched_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
            regionid_array.push(row.regionid);
            dispatchinterval_array.push(row.dispatchinterval.start().timestamp_millis());
            intervention_array.push({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
            totaldemand_array.push({
                row.totaldemand.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            availablegeneration_array.push({
                row.availablegeneration.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            availableload_array.push({
                row.availableload.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            demandforecast_array.push({
                row.demandforecast.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            dispatchablegeneration_array.push({
                row.dispatchablegeneration.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            dispatchableload_array.push({
                row.dispatchableload.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            netinterchange_array.push({
                row.netinterchange.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            excessgeneration_array.push({
                row.excessgeneration.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower5mindispatch_array.push({
                row.lower5mindispatch.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower5minimport_array.push({
                row.lower5minimport.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower5minlocaldispatch_array.push({
                row.lower5minlocaldispatch.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower5minlocalprice_array.push({
                row.lower5minlocalprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower5minlocalreq_array.push({
                row.lower5minlocalreq.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower5minprice_array.push({
                row.lower5minprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower5minreq_array.push({
                row.lower5minreq.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower5minsupplyprice_array.push({
                row.lower5minsupplyprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower60secdispatch_array.push({
                row.lower60secdispatch.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower60secimport_array.push({
                row.lower60secimport.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower60seclocaldispatch_array.push({
                row.lower60seclocaldispatch.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower60seclocalprice_array.push({
                row.lower60seclocalprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower60seclocalreq_array.push({
                row.lower60seclocalreq.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower60secprice_array.push({
                row.lower60secprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower60secreq_array.push({
                row.lower60secreq.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower60secsupplyprice_array.push({
                row.lower60secsupplyprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower6secdispatch_array.push({
                row.lower6secdispatch.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower6secimport_array.push({
                row.lower6secimport.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower6seclocaldispatch_array.push({
                row.lower6seclocaldispatch.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower6seclocalprice_array.push({
                row.lower6seclocalprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower6seclocalreq_array.push({
                row.lower6seclocalreq.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower6secprice_array.push({
                row.lower6secprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower6secreq_array.push({
                row.lower6secreq.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower6secsupplyprice_array.push({
                row.lower6secsupplyprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise5mindispatch_array.push({
                row.raise5mindispatch.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise5minimport_array.push({
                row.raise5minimport.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise5minlocaldispatch_array.push({
                row.raise5minlocaldispatch.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise5minlocalprice_array.push({
                row.raise5minlocalprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise5minlocalreq_array.push({
                row.raise5minlocalreq.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise5minprice_array.push({
                row.raise5minprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise5minreq_array.push({
                row.raise5minreq.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise5minsupplyprice_array.push({
                row.raise5minsupplyprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise60secdispatch_array.push({
                row.raise60secdispatch.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise60secimport_array.push({
                row.raise60secimport.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise60seclocaldispatch_array.push({
                row.raise60seclocaldispatch.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise60seclocalprice_array.push({
                row.raise60seclocalprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise60seclocalreq_array.push({
                row.raise60seclocalreq.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise60secprice_array.push({
                row.raise60secprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise60secreq_array.push({
                row.raise60secreq.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise60secsupplyprice_array.push({
                row.raise60secsupplyprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise6secdispatch_array.push({
                row.raise6secdispatch.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise6secimport_array.push({
                row.raise6secimport.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise6seclocaldispatch_array.push({
                row.raise6seclocaldispatch.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise6seclocalprice_array.push({
                row.raise6seclocalprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise6seclocalreq_array.push({
                row.raise6seclocalreq.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise6secprice_array.push({
                row.raise6secprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise6secreq_array.push({
                row.raise6secreq.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise6secsupplyprice_array.push({
                row.raise6secsupplyprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            aggegatedispatcherror_array.push({
                row.aggegatedispatcherror.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            aggregatedispatcherror_array.push({
                row.aggregatedispatcherror.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            initialsupply_array.push({
                row.initialsupply.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            clearedsupply_array.push({
                row.clearedsupply.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lowerregimport_array.push({
                row.lowerregimport.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lowerreglocaldispatch_array.push({
                row.lowerreglocaldispatch.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lowerreglocalreq_array.push({
                row.lowerreglocalreq.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lowerregreq_array.push({
                row.lowerregreq.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raiseregimport_array.push({
                row.raiseregimport.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raisereglocaldispatch_array.push({
                row.raisereglocaldispatch.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raisereglocalreq_array.push({
                row.raisereglocalreq.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raiseregreq_array.push({
                row.raiseregreq.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise5minlocalviolation_array.push({
                row.raise5minlocalviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raisereglocalviolation_array.push({
                row.raisereglocalviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise60seclocalviolation_array.push({
                row.raise60seclocalviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise6seclocalviolation_array.push({
                row.raise6seclocalviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower5minlocalviolation_array.push({
                row.lower5minlocalviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lowerreglocalviolation_array.push({
                row.lowerreglocalviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower60seclocalviolation_array.push({
                row.lower60seclocalviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower6seclocalviolation_array.push({
                row.lower6seclocalviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise5minviolation_array.push({
                row.raise5minviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raiseregviolation_array.push({
                row.raiseregviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise60secviolation_array.push({
                row.raise60secviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise6secviolation_array.push({
                row.raise6secviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower5minviolation_array.push({
                row.lower5minviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lowerregviolation_array.push({
                row.lowerregviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower60secviolation_array.push({
                row.lower60secviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower6secviolation_array.push({
                row.lower6secviolation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise6secactualavailability_array.push({
                row.raise6secactualavailability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            raise60secactualavailability_array.push({
                row.raise60secactualavailability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            raise5minactualavailability_array.push({
                row.raise5minactualavailability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            raiseregactualavailability_array.push({
                row.raiseregactualavailability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lower6secactualavailability_array.push({
                row.lower6secactualavailability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lower60secactualavailability_array.push({
                row.lower60secactualavailability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lower5minactualavailability_array.push({
                row.lower5minactualavailability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lowerregactualavailability_array.push({
                row.lowerregactualavailability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lorsurplus_array.push({
                row.lorsurplus.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lrcsurplus_array.push({
                row.lrcsurplus.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            totalintermittentgeneration_array.push({
                row.totalintermittentgeneration.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            demand_and_nonschedgen_array.push({
                row.demand_and_nonschedgen.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            uigf_array.push({
                row.uigf.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            semischedule_clearedmw_array.push({
                row.semischedule_clearedmw.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            semischedule_compliancemw_array.push({
                row.semischedule_compliancemw.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            ss_solar_uigf_array.push({
                row.ss_solar_uigf.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            ss_wind_uigf_array.push({
                row.ss_wind_uigf.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            ss_solar_clearedmw_array.push({
                row.ss_solar_clearedmw.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            ss_wind_clearedmw_array.push({
                row.ss_wind_clearedmw.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            ss_solar_compliancemw_array.push({
                row.ss_solar_compliancemw.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            ss_wind_compliancemw_array.push({
                row.ss_wind_compliancemw.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            wdr_initialmw_array.push({
                row.wdr_initialmw.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            wdr_available_array.push({
                row.wdr_available.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            wdr_dispatched_array.push({
                row.wdr_dispatched.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(dispatchinterval_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totaldemand_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(availablegeneration_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(availableload_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(demandforecast_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(dispatchablegeneration_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(dispatchableload_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(netinterchange_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(excessgeneration_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5mindispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minimport_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minlocaldispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minlocalprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minlocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minsupplyprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secdispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secimport_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60seclocaldispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60seclocalprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60seclocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secsupplyprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secdispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secimport_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6seclocaldispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6seclocalprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6seclocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secsupplyprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5mindispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minimport_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minlocaldispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minlocalprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minlocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minsupplyprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secdispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secimport_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60seclocaldispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60seclocalprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60seclocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secsupplyprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secdispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secimport_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6seclocaldispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6seclocalprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6seclocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secsupplyprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(aggegatedispatcherror_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(aggregatedispatcherror_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(initialsupply_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(clearedsupply_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregimport_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerreglocaldispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerreglocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregimport_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raisereglocaldispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raisereglocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minlocalviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raisereglocalviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60seclocalviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6seclocalviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minlocalviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerreglocalviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60seclocalviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6seclocalviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lorsurplus_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lrcsurplus_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalintermittentgeneration_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(demand_and_nonschedgen_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(uigf_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(semischedule_clearedmw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(semischedule_compliancemw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ss_solar_uigf_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ss_wind_uigf_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ss_solar_clearedmw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ss_wind_clearedmw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ss_solar_compliancemw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ss_wind_compliancemw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(wdr_initialmw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(wdr_available_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(wdr_dispatched_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCH_CONSTRAINT_FCAS_OCD
///  _FCAS constraint solution from OCD re-run._
///
/// * Data Set Name: Priceload
/// * File Name: Constraint Fcas Ocd
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PriceloadConstraintFcasOcd1 {
    /// Dispatch interval that the prices were loaded to
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: i64,
    /// Intervention 0/1
    pub intervention: i64,
    /// ConstraintID/GenconID
    pub constraintid: String,
    /// VersionNo
    pub versionno: i64,
    /// The datetime that the record was last changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// RHS from OCD re-run
    pub rhs: Option<rust_decimal::Decimal>,
    /// marginalvalue from OCD re-run
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// The violation degree of this constraint in the solution result
    pub violationdegree: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for PriceloadConstraintFcasOcd1 {
    type PrimaryKey = PriceloadConstraintFcasOcd1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PRICELOAD".into(),
            table_name: Some("CONSTRAINT_FCAS_OCD".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> PriceloadConstraintFcasOcd1PrimaryKey {
        PriceloadConstraintFcasOcd1PrimaryKey {
            constraintid: self.constraintid.clone(),
            intervention: self.intervention,
            runno: self.runno,
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "priceload_constraint_fcas_ocd_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for PriceloadConstraintFcasOcd1 {
    type Row = PriceloadConstraintFcasOcd1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid
            && self.intervention == row.intervention
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for PriceloadConstraintFcasOcd1 {
    type PrimaryKey = PriceloadConstraintFcasOcd1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.intervention == key.intervention
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PriceloadConstraintFcasOcd1PrimaryKey {
    pub constraintid: String,
    pub intervention: i64,
    pub runno: i64,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: i64,
}
impl crate::CompareWithRow for PriceloadConstraintFcasOcd1PrimaryKey {
    type Row = PriceloadConstraintFcasOcd1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid
            && self.intervention == row.intervention
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for PriceloadConstraintFcasOcd1PrimaryKey {
    type PrimaryKey = PriceloadConstraintFcasOcd1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.intervention == key.intervention
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for PriceloadConstraintFcasOcd1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PriceloadConstraintFcasOcd1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("runno", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Int64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "constraintid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("rhs", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new(
                "marginalvalue",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "violationdegree",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut constraintid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut rhs_array = Vec::new();
        let mut marginalvalue_array = Vec::new();
        let mut violationdegree_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            runno_array.push(row.runno);
            intervention_array.push(row.intervention);
            constraintid_array.push(row.constraintid);
            versionno_array.push(row.versionno);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
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
            violationdegree_array.push({
                row.violationdegree.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(runno_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(
                    intervention_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    constraintid_array,
                )),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(versionno_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rhs_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginalvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(violationdegree_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCH_FCAS_REQ
///  _DISPATCH_FCAS_REQ shows Dispatch Constraint tracking for Regional FCAS recovery._
///
/// * Data Set Name: Dispatch
/// * File Name: Fcas Req
/// * Data Version: 2
///
/// # Description
///  DISPATCH_FCAS_REQ is public data and is available to all participants. Source DISPATCH_FCAS_REQ updates with each dispatch run (5 minutes). Volume Approximately 10,000 rows per day
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * GENCONID
/// * INTERVENTION
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchFcasReq2 {
    /// Settlement date and time of Dispatch Interval
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Intervention Flag
    pub intervention: rust_decimal::Decimal,
    /// Generic Constraint ID - Join to table GenConData
    pub genconid: String,
    /// &nbsp;
    pub regionid: String,
    /// DUID offered type
    pub bidtype: String,
    /// Generic Constraint EffectiveDate - Join to table GenConData
    #[serde(with = "crate::mms_datetime_opt")]
    pub genconeffectivedate: Option<chrono::NaiveDateTime>,
    /// Generic Constraint Version number - Join to table GenConData
    pub genconversionno: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Date record is changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The base cost of the constraint for this service, before the regulation/contingency split
    pub base_cost: Option<rust_decimal::Decimal>,
    /// The adjusted cost of the constraint for this service, before the regulation/contingency split
    pub adjusted_cost: Option<rust_decimal::Decimal>,
    /// An estimated value for the constraint CMPF, based on dispatched data
    pub estimated_cmpf: Option<rust_decimal::Decimal>,
    /// An estimated value for the constraint CRMPF, based on dispatched data
    pub estimated_crmpf: Option<rust_decimal::Decimal>,
    /// Estimated recovery factor for CMPF based recovery
    pub recovery_factor_cmpf: Option<rust_decimal::Decimal>,
    /// Estimated recovery factor for CRMPF based recovery
    pub recovery_factor_crmpf: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DispatchFcasReq2 {
    type PrimaryKey = DispatchFcasReq2PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("FCAS_REQ".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> DispatchFcasReq2PrimaryKey {
        DispatchFcasReq2PrimaryKey {
            bidtype: self.bidtype.clone(),
            genconid: self.genconid.clone(),
            intervention: self.intervention,
            regionid: self.regionid.clone(),
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "dispatch_fcas_req_v2_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DispatchFcasReq2 {
    type Row = DispatchFcasReq2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.genconid == row.genconid
            && self.intervention == row.intervention
            && self.regionid == row.regionid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchFcasReq2 {
    type PrimaryKey = DispatchFcasReq2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.genconid == key.genconid
            && self.intervention == key.intervention
            && self.regionid == key.regionid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchFcasReq2PrimaryKey {
    pub bidtype: String,
    pub genconid: String,
    pub intervention: rust_decimal::Decimal,
    pub regionid: String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DispatchFcasReq2PrimaryKey {
    type Row = DispatchFcasReq2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.genconid == row.genconid
            && self.intervention == row.intervention
            && self.regionid == row.regionid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchFcasReq2PrimaryKey {
    type PrimaryKey = DispatchFcasReq2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.genconid == key.genconid
            && self.intervention == key.intervention
            && self.regionid == key.regionid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for DispatchFcasReq2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchFcasReq2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "genconid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("bidtype", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "genconeffectivedate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "genconversionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "marginalvalue",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "base_cost",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "adjusted_cost",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "estimated_cmpf",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "estimated_crmpf",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "recovery_factor_cmpf",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "recovery_factor_crmpf",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut genconid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut bidtype_array = Vec::new();
        let mut genconeffectivedate_array = Vec::new();
        let mut genconversionno_array = Vec::new();
        let mut marginalvalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut base_cost_array = Vec::new();
        let mut adjusted_cost_array = Vec::new();
        let mut estimated_cmpf_array = Vec::new();
        let mut estimated_crmpf_array = Vec::new();
        let mut recovery_factor_cmpf_array = Vec::new();
        let mut recovery_factor_crmpf_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
            intervention_array.push({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
            genconid_array.push(row.genconid);
            regionid_array.push(row.regionid);
            bidtype_array.push(row.bidtype);
            genconeffectivedate_array
                .push(row.genconeffectivedate.map(|val| val.timestamp_millis()));
            genconversionno_array.push({
                row.genconversionno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            marginalvalue_array.push({
                row.marginalvalue.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            base_cost_array.push({
                row.base_cost.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            adjusted_cost_array.push({
                row.adjusted_cost.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            estimated_cmpf_array.push({
                row.estimated_cmpf.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            estimated_crmpf_array.push({
                row.estimated_crmpf.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            recovery_factor_cmpf_array.push({
                row.recovery_factor_cmpf.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            recovery_factor_crmpf_array.push({
                row.recovery_factor_crmpf.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(genconid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bidtype_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(genconeffectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(genconversionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginalvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(base_cost_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(adjusted_cost_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(estimated_cmpf_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(estimated_crmpf_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(recovery_factor_cmpf_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(recovery_factor_crmpf_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCH_INTERCONNECTION
///  _Inter-regional flow information common to or aggregated for regulated (i.e. not MNSP) Interconnectors spanning the From-Region and To-Region - NB only the physical run is calculated'_
///
/// * Data Set Name: Dispatch
/// * File Name: Interconnection
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * FROM_REGIONID
/// * INTERVENTION
/// * RUNNO
/// * SETTLEMENTDATE
/// * TO_REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchInterconnection1 {
    /// Market date starting at 04:05
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Intervention case or not
    pub intervention: rust_decimal::Decimal,
    /// Nominated RegionID from which the energy flows
    pub from_regionid: String,
    /// Nominated RegionID to which the energy flows
    pub to_regionid: String,
    /// Dispatch period identifier, from 001 to 288 in format YYYYMMDDPPP
    #[serde(with = "crate::dispatch_period")]
    pub dispatchinterval: crate::DispatchPeriod,
    /// Inter-Regional Loss Factor. Calculated based on the MWFLOW and the nominal From and To Region losses.  
    pub irlf: Option<rust_decimal::Decimal>,
    /// Summed MW flow of the parallel regulated Interconnectors
    pub mwflow: Option<rust_decimal::Decimal>,
    /// Summed Metered MW flow of the parallel regulated Interconnectors
    pub meteredmwflow: Option<rust_decimal::Decimal>,
    /// Losses across the Interconnection attributable to the nominal From Region
    pub from_region_mw_losses: Option<rust_decimal::Decimal>,
    /// Losses across the Interconnection attributable to the nominal To Region
    pub to_region_mw_losses: Option<rust_decimal::Decimal>,
    /// The datetime that the record was last changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for DispatchInterconnection1 {
    type PrimaryKey = DispatchInterconnection1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("INTERCONNECTION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DispatchInterconnection1PrimaryKey {
        DispatchInterconnection1PrimaryKey {
            from_regionid: self.from_regionid.clone(),
            intervention: self.intervention,
            runno: self.runno,
            settlementdate: self.settlementdate,
            to_regionid: self.to_regionid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "dispatch_interconnection_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DispatchInterconnection1 {
    type Row = DispatchInterconnection1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.from_regionid == row.from_regionid
            && self.intervention == row.intervention
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
            && self.to_regionid == row.to_regionid
    }
}
impl crate::CompareWithPrimaryKey for DispatchInterconnection1 {
    type PrimaryKey = DispatchInterconnection1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.from_regionid == key.from_regionid
            && self.intervention == key.intervention
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
            && self.to_regionid == key.to_regionid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchInterconnection1PrimaryKey {
    pub from_regionid: String,
    pub intervention: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub to_regionid: String,
}
impl crate::CompareWithRow for DispatchInterconnection1PrimaryKey {
    type Row = DispatchInterconnection1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.from_regionid == row.from_regionid
            && self.intervention == row.intervention
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
            && self.to_regionid == row.to_regionid
    }
}
impl crate::CompareWithPrimaryKey for DispatchInterconnection1PrimaryKey {
    type PrimaryKey = DispatchInterconnection1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.from_regionid == key.from_regionid
            && self.intervention == key.intervention
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
            && self.to_regionid == key.to_regionid
    }
}
impl crate::PrimaryKey for DispatchInterconnection1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchInterconnection1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "from_regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "to_regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "dispatchinterval",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "irlf",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "meteredmwflow",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "from_region_mw_losses",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "to_region_mw_losses",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut from_regionid_array = Vec::new();
        let mut to_regionid_array = Vec::new();
        let mut dispatchinterval_array = Vec::new();
        let mut irlf_array = Vec::new();
        let mut mwflow_array = Vec::new();
        let mut meteredmwflow_array = Vec::new();
        let mut from_region_mw_losses_array = Vec::new();
        let mut to_region_mw_losses_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
            intervention_array.push({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
            from_regionid_array.push(row.from_regionid);
            to_regionid_array.push(row.to_regionid);
            dispatchinterval_array.push(row.dispatchinterval.start().timestamp_millis());
            irlf_array.push({
                row.irlf.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow_array.push({
                row.mwflow.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            meteredmwflow_array.push({
                row.meteredmwflow.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            from_region_mw_losses_array.push({
                row.from_region_mw_losses.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            to_region_mw_losses_array.push({
                row.to_region_mw_losses.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    from_regionid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    to_regionid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(dispatchinterval_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(irlf_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(meteredmwflow_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(from_region_mw_losses_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(to_region_mw_losses_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCH_LOCAL_PRICE
///  _Sets out local pricing offsets associated with each DUID connection point for each dispatch period. Note that from 2014 Mid year release only records with non-zero Local_Price_Adjustment values are issued_
///
/// * Data Set Name: Dispatch
/// * File Name: Local Price
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * DUID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchLocalPrice1 {
    /// Market date time starting at 04:05
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: String,
    /// Aggregate Constraint contribution cost of this unit: Sum(MarginalValue x Factor) for all relevant Constraints
    pub local_price_adjustment: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DispatchLocalPrice1 {
    type PrimaryKey = DispatchLocalPrice1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("LOCAL_PRICE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DispatchLocalPrice1PrimaryKey {
        DispatchLocalPrice1PrimaryKey {
            duid: self.duid.clone(),
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "dispatch_local_price_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DispatchLocalPrice1 {
    type Row = DispatchLocalPrice1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchLocalPrice1 {
    type PrimaryKey = DispatchLocalPrice1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchLocalPrice1PrimaryKey {
    pub duid: String,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DispatchLocalPrice1PrimaryKey {
    type Row = DispatchLocalPrice1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchLocalPrice1PrimaryKey {
    type PrimaryKey = DispatchLocalPrice1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for DispatchLocalPrice1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchLocalPrice1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "local_price_adjustment",
                arrow2::datatypes::DataType::Decimal(10, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "locally_constrained",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut local_price_adjustment_array = Vec::new();
        let mut locally_constrained_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            duid_array.push(row.duid);
            local_price_adjustment_array.push({
                row.local_price_adjustment.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            locally_constrained_array.push({
                row.locally_constrained.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(local_price_adjustment_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(locally_constrained_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCH_MNSPBIDTRK
///  _DISPATCH_MNSPBIDTRK shows the MNSP bid tracking, including the bid version used in each dispatch run for each MNSP Interconnector Link. The table identifies which bids from MNSP_DAYOFFER and MNSP_BIDOFFERPERIOD were applied._
///
/// * Data Set Name: Dispatch
/// * File Name: Mnspbidtrk
/// * Data Version: 1
///
/// # Description
///  DISPATCH_MNSPBIDTRK shows own details for participant as they occur, with all details until close of business yesterday being available to all participants after end of day. Source DISPATCH_MNSPBIDTRK potentially updates every 5 minutes. Volume 220, 000 per year
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * LINKID
/// * PARTICIPANTID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchMnspbidtrk1 {
    /// Market date starting at 04:05
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Participant that owns unit during effective record period
    pub participantid: String,
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    pub linkid: String,
    /// Offer date for bid
    #[serde(with = "crate::mms_datetime_opt")]
    pub offersettlementdate: Option<chrono::NaiveDateTime>,
    /// Time this bid was processed and loaded
    #[serde(with = "crate::mms_datetime_opt")]
    pub offereffectivedate: Option<chrono::NaiveDateTime>,
    /// VersionNo of the bid/offer used
    pub offerversionno: Option<rust_decimal::Decimal>,
    /// Record creation timestamp
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for DispatchMnspbidtrk1 {
    type PrimaryKey = DispatchMnspbidtrk1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("MNSPBIDTRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DispatchMnspbidtrk1PrimaryKey {
        DispatchMnspbidtrk1PrimaryKey {
            linkid: self.linkid.clone(),
            participantid: self.participantid.clone(),
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "dispatch_mnspbidtrk_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DispatchMnspbidtrk1 {
    type Row = DispatchMnspbidtrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.linkid == row.linkid
            && self.participantid == row.participantid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchMnspbidtrk1 {
    type PrimaryKey = DispatchMnspbidtrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid == key.linkid
            && self.participantid == key.participantid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchMnspbidtrk1PrimaryKey {
    pub linkid: String,
    pub participantid: String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DispatchMnspbidtrk1PrimaryKey {
    type Row = DispatchMnspbidtrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.linkid == row.linkid
            && self.participantid == row.participantid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchMnspbidtrk1PrimaryKey {
    type PrimaryKey = DispatchMnspbidtrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid == key.linkid
            && self.participantid == key.participantid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for DispatchMnspbidtrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchMnspbidtrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("linkid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "offersettlementdate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "offereffectivedate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "offerversionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut linkid_array = Vec::new();
        let mut offersettlementdate_array = Vec::new();
        let mut offereffectivedate_array = Vec::new();
        let mut offerversionno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            linkid_array.push(row.linkid);
            offersettlementdate_array
                .push(row.offersettlementdate.map(|val| val.timestamp_millis()));
            offereffectivedate_array.push(row.offereffectivedate.map(|val| val.timestamp_millis()));
            offerversionno_array.push({
                row.offerversionno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(linkid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(offersettlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(offereffectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(offerversionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCH_MR_SCHEDULE_TRK
///  _DISPATCH_MR_SCHEDULE_TRK records the Mandatory Restrictions Acceptance Schedule applied to this dispatch interval for this region.<br>DISPATCH_MR_SCHEDULE_TRK is populated by the Dispatch process and records the MR Offer Stack applied in each dispatch interval. DISPATCH_MR_SCHEDULE_TRK is used by Settlements to calculate payments according to the correct MR offer stack.<br>_
///
/// * Data Set Name: Dispatch
/// * File Name: Mr Schedule Trk
/// * Data Version: 1
///
/// # Description
///  DISPATCH_MR_SCHEDULE_TRK  data is public to all participants. Source DISPATCH_MR_SCHEDULE_TRK updates are ad hoc. Volume 2 rows per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * REGIONID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchMrScheduleTrk1 {
    /// Date Time of the Dispatch Interval
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Unique RegionID; Key reference to MR_Event_Schedule
    pub regionid: String,
    /// Mandatory Restriction date; Key reference to MR_Event_Schedule table
    #[serde(with = "crate::mms_datetime_opt")]
    pub mr_date: Option<chrono::NaiveDateTime>,
    /// Date Time the MR  acceptance stack was created; Key reference to MR_Event_Schedule table
    #[serde(with = "crate::mms_datetime_opt")]
    pub version_datetime: Option<chrono::NaiveDateTime>,
    /// Date and  time the record was last inserted/modified
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for DispatchMrScheduleTrk1 {
    type PrimaryKey = DispatchMrScheduleTrk1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("MR_SCHEDULE_TRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DispatchMrScheduleTrk1PrimaryKey {
        DispatchMrScheduleTrk1PrimaryKey {
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "dispatch_mr_schedule_trk_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DispatchMrScheduleTrk1 {
    type Row = DispatchMrScheduleTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.regionid == row.regionid && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchMrScheduleTrk1 {
    type PrimaryKey = DispatchMrScheduleTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchMrScheduleTrk1PrimaryKey {
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DispatchMrScheduleTrk1PrimaryKey {
    type Row = DispatchMrScheduleTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.regionid == row.regionid && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchMrScheduleTrk1PrimaryKey {
    type PrimaryKey = DispatchMrScheduleTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for DispatchMrScheduleTrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchMrScheduleTrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("mr_date", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "version_datetime",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut mr_date_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            regionid_array.push(row.regionid);
            mr_date_array.push(row.mr_date.map(|val| val.timestamp_millis()));
            version_datetime_array.push(row.version_datetime.map(|val| val.timestamp_millis()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mr_date_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(version_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCH_PRICE_REVISION
///  _An audit trail of price changes on the DISPATCHPRICE table (i.e. for 5 minute dispatch prices for energy and FCAS)._
///
/// * Data Set Name: Priceload
/// * File Name: Price Revision
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * INTERVENTION
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PriceloadPriceRevision1 {
    /// Market date and time starting at 04:05
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no; always 1
    pub runno: rust_decimal::Decimal,
    /// Manual intervention flag; always 0
    pub intervention: rust_decimal::Decimal,
    /// Affected Region Identifier
    pub regionid: String,
    /// Affected Bid Type Identifier
    pub bidtype: String,
    /// Version No of price revision for this settlement date
    pub versionno: i64,
    /// New RRP in DISPATCHPRICE table
    pub rrp_new: Option<rust_decimal::Decimal>,
    /// Old RRP from DISPATCHPRICE table
    pub rrp_old: Option<rust_decimal::Decimal>,
    /// The datetime the record was last changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for PriceloadPriceRevision1 {
    type PrimaryKey = PriceloadPriceRevision1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PRICELOAD".into(),
            table_name: Some("PRICE_REVISION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> PriceloadPriceRevision1PrimaryKey {
        PriceloadPriceRevision1PrimaryKey {
            bidtype: self.bidtype.clone(),
            intervention: self.intervention,
            regionid: self.regionid.clone(),
            runno: self.runno,
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "priceload_price_revision_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for PriceloadPriceRevision1 {
    type Row = PriceloadPriceRevision1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.intervention == row.intervention
            && self.regionid == row.regionid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for PriceloadPriceRevision1 {
    type PrimaryKey = PriceloadPriceRevision1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.intervention == key.intervention
            && self.regionid == key.regionid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PriceloadPriceRevision1PrimaryKey {
    pub bidtype: String,
    pub intervention: rust_decimal::Decimal,
    pub regionid: String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: i64,
}
impl crate::CompareWithRow for PriceloadPriceRevision1PrimaryKey {
    type Row = PriceloadPriceRevision1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.intervention == row.intervention
            && self.regionid == row.regionid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for PriceloadPriceRevision1PrimaryKey {
    type PrimaryKey = PriceloadPriceRevision1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.intervention == key.intervention
            && self.regionid == key.regionid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for PriceloadPriceRevision1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PriceloadPriceRevision1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("bidtype", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "rrp_new",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp_old",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut bidtype_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut rrp_new_array = Vec::new();
        let mut rrp_old_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
            intervention_array.push({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
            regionid_array.push(row.regionid);
            bidtype_array.push(row.bidtype);
            versionno_array.push(row.versionno);
            rrp_new_array.push({
                row.rrp_new.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp_old_array.push({
                row.rrp_old.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bidtype_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(versionno_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp_new_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp_old_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCH_UNIT_CONFORMANCE
///  _DISPATCH_UNIT_CONFORMANCE details the conformance of a scheduled units operation with respect to a cleared target on dispatch interval basis.<br>Data is confidential_
///
/// * Data Set Name: Dispatch
/// * File Name: Unit Conformance
/// * Data Version: 1
///
/// # Description
///  DISPATCH_UNIT_CONFORMANCE data is confidential. Source DISPATCH_UNIT_CONFORMANCE shows data for every 5 minutes for all scheduled units Volume Rows per day: 288 per scheduled unit
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * DUID
/// * INTERVAL_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchUnitConformance1 {
    /// Dispatch Interval that the conformance data applies to
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Dispatchable Unit Identifier
    pub duid: String,
    /// Dispatch Target - MW
    pub totalcleared: Option<rust_decimal::Decimal>,
    /// Unit output measured at the conclusion of the dispatch interval - MW (MWB)
    pub actualmw: Option<rust_decimal::Decimal>,
    /// Rate of Change in direction of error MW per hour
    pub roc: Option<rust_decimal::Decimal>,
    /// Offered unit capacity - MW (MWO)
    pub availability: Option<rust_decimal::Decimal>,
    /// Lower Regulation FCAS enabled - MW (FCL)
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation FCAS enabled - MW (FCR)
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Calculated small trigger error limit in MW
    pub striglm: Option<rust_decimal::Decimal>,
    /// Calculated large trigger error limit in MW
    pub ltriglm: Option<rust_decimal::Decimal>,
    /// Calculated actual error
    pub mwerror: Option<rust_decimal::Decimal>,
    /// Max of mwerror while that unit was not in a normal state
    pub max_mwerror: Option<rust_decimal::Decimal>,
    /// Large trigger error count. Reset when mwerror changes sign
    pub lecount: Option<i64>,
    /// Small trigger error count.  Reset when mwerror changes sign
    pub secount: Option<i64>,
    /// Unit conformance status.<br>NORMAL<br>OFF-TARGET<br>NOT-RESPONDING<br>NC-PENDING<br>NON-CONFORMING<br>SUSPENDED
    pub status: Option<String>,
    /// Participant action required in response to current STATUS
    pub participant_status_action: Option<String>,
    /// conformance operating mode<br>MANUAL<br>AUTO
    pub operating_mode: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for DispatchUnitConformance1 {
    type PrimaryKey = DispatchUnitConformance1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("UNIT_CONFORMANCE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DispatchUnitConformance1PrimaryKey {
        DispatchUnitConformance1PrimaryKey {
            duid: self.duid.clone(),
            interval_datetime: self.interval_datetime,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "dispatch_unit_conformance_v1".to_string()
    }
}
impl crate::CompareWithRow for DispatchUnitConformance1 {
    type Row = DispatchUnitConformance1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.interval_datetime == row.interval_datetime
    }
}
impl crate::CompareWithPrimaryKey for DispatchUnitConformance1 {
    type PrimaryKey = DispatchUnitConformance1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.interval_datetime == key.interval_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchUnitConformance1PrimaryKey {
    pub duid: String,
    pub interval_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DispatchUnitConformance1PrimaryKey {
    type Row = DispatchUnitConformance1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.interval_datetime == row.interval_datetime
    }
}
impl crate::CompareWithPrimaryKey for DispatchUnitConformance1PrimaryKey {
    type PrimaryKey = DispatchUnitConformance1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.interval_datetime == key.interval_datetime
    }
}
impl crate::PrimaryKey for DispatchUnitConformance1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchUnitConformance1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "interval_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "totalcleared",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "actualmw",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new("roc", arrow2::datatypes::DataType::Decimal(16, 6), true),
            arrow2::datatypes::Field::new(
                "availability",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerreg",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raisereg",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "striglm",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "ltriglm",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwerror",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "max_mwerror",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new("lecount", arrow2::datatypes::DataType::Int64, true),
            arrow2::datatypes::Field::new("secount", arrow2::datatypes::DataType::Int64, true),
            arrow2::datatypes::Field::new("status", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "participant_status_action",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "operating_mode",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut interval_datetime_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut totalcleared_array = Vec::new();
        let mut actualmw_array = Vec::new();
        let mut roc_array = Vec::new();
        let mut availability_array = Vec::new();
        let mut lowerreg_array = Vec::new();
        let mut raisereg_array = Vec::new();
        let mut striglm_array = Vec::new();
        let mut ltriglm_array = Vec::new();
        let mut mwerror_array = Vec::new();
        let mut max_mwerror_array = Vec::new();
        let mut lecount_array = Vec::new();
        let mut secount_array = Vec::new();
        let mut status_array = Vec::new();
        let mut participant_status_action_array = Vec::new();
        let mut operating_mode_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            interval_datetime_array.push(row.interval_datetime.timestamp_millis());
            duid_array.push(row.duid);
            totalcleared_array.push({
                row.totalcleared.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            actualmw_array.push({
                row.actualmw.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            roc_array.push({
                row.roc.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            availability_array.push({
                row.availability.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lowerreg_array.push({
                row.lowerreg.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            raisereg_array.push({
                row.raisereg.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            striglm_array.push({
                row.striglm.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            ltriglm_array.push({
                row.ltriglm.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            mwerror_array.push({
                row.mwerror.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            max_mwerror_array.push({
                row.max_mwerror.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lecount_array.push(row.lecount);
            secount_array.push(row.secount);
            status_array.push(row.status);
            participant_status_action_array.push(row.participant_status_action);
            operating_mode_array.push(row.operating_mode);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(interval_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalcleared_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(actualmw_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(roc_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(availability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerreg_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raisereg_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(striglm_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ltriglm_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwerror_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(max_mwerror_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lecount_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(secount_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(status_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    participant_status_action_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(operating_mode_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DISPATCH_UNIT_SCADA
///  _Dispatchable unit MW from SCADA at the start of the dispatch interval. The table includes all scheduled and semi-scheduled (and non-scheduled units where SCADA is available)_
///
/// * Data Set Name: Dispatch
/// * File Name: Unit Scada
/// * Data Version: 1
///
/// # Description
///  DISPATCH_UNIT_SCADA data  is public data, and is available to all participants. Source DISPATCH_UNIT_SCADA shows data for every 5 minutes for all scheduled units Volume Rows per day: 288 per each scheduled, semi-scheduled (and non-scheduled unit where SCADA is available)
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DUID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchUnitScada1 {
    /// Date Time of the Dispatch Interval
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatchable Unit Identifier
    pub duid: String,
    /// Instantaneous MW reading from SCADA at the start of the Dispatch interval
    pub scadavalue: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DispatchUnitScada1 {
    type PrimaryKey = DispatchUnitScada1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("UNIT_SCADA".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DispatchUnitScada1PrimaryKey {
        DispatchUnitScada1PrimaryKey {
            duid: self.duid.clone(),
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "dispatch_unit_scada_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DispatchUnitScada1 {
    type Row = DispatchUnitScada1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchUnitScada1 {
    type PrimaryKey = DispatchUnitScada1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchUnitScada1PrimaryKey {
    pub duid: String,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DispatchUnitScada1PrimaryKey {
    type Row = DispatchUnitScada1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchUnitScada1PrimaryKey {
    type PrimaryKey = DispatchUnitScada1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for DispatchUnitScada1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchUnitScada1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "scadavalue",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut scadavalue_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            duid_array.push(row.duid);
            scadavalue_array.push({
                row.scadavalue.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(scadavalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## INTERMITTENT_FORECAST_TRK
///  _Uniquely tracks which Intermittent Generation forecast was used for the DUID in which Dispatch run_
///
/// * Data Set Name: Dispatch
/// * File Name: Intermittent Forecast Trk
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * DUID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchIntermittentForecastTrk1 {
    /// DateTime of the Dispatch run (dispatch interval ending)
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Tracks to INTERMITTENT_DS_RUN.DUID
    pub duid: String,
    /// Tracks to INTERMITTENT_DS_RUN.ORIGIN, except when the forecast used is either SCADA or FCST or Last Target
    pub origin: Option<String>,
    /// Tracks to INTERMITTENT_DS_RUN.FORECAST_PRIORITY, except for -1 which denotes SCADA or FCST, and 0 which denotes Last Target
    pub forecast_priority: Option<rust_decimal::Decimal>,
    /// Tracks to INTERMITTENT_DS_RUN.OFFERDATETIME
    #[serde(with = "crate::mms_datetime_opt")]
    pub offerdatetime: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for DispatchIntermittentForecastTrk1 {
    type PrimaryKey = DispatchIntermittentForecastTrk1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("INTERMITTENT_FORECAST_TRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DispatchIntermittentForecastTrk1PrimaryKey {
        DispatchIntermittentForecastTrk1PrimaryKey {
            duid: self.duid.clone(),
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "dispatch_intermittent_forecast_trk_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DispatchIntermittentForecastTrk1 {
    type Row = DispatchIntermittentForecastTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchIntermittentForecastTrk1 {
    type PrimaryKey = DispatchIntermittentForecastTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchIntermittentForecastTrk1PrimaryKey {
    pub duid: String,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DispatchIntermittentForecastTrk1PrimaryKey {
    type Row = DispatchIntermittentForecastTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchIntermittentForecastTrk1PrimaryKey {
    type PrimaryKey = DispatchIntermittentForecastTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for DispatchIntermittentForecastTrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchIntermittentForecastTrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("origin", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "forecast_priority",
                arrow2::datatypes::DataType::Decimal(10, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "offerdatetime",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut origin_array = Vec::new();
        let mut forecast_priority_array = Vec::new();
        let mut offerdatetime_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            duid_array.push(row.duid);
            origin_array.push(row.origin);
            forecast_priority_array.push({
                row.forecast_priority.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            offerdatetime_array.push(row.offerdatetime.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(origin_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(forecast_priority_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(offerdatetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## NEGATIVE_RESIDUE
///  _Shows the inputs provided to the Negative Residue Constraints in the Dispatch horizon_
///
/// * Data Set Name: Dispatch
/// * File Name: Negative Residue
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DIRECTIONAL_INTERCONNECTORID
/// * NRM_DATETIME
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DispatchNegativeResidue1 {
    /// Dispatch Interval
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// The time that residue information is processed
    #[serde(with = "crate::mms_datetime")]
    pub nrm_datetime: chrono::NaiveDateTime,
    /// Negative residue related direction interconnector id
    pub directional_interconnectorid: String,
    /// Is 1 if negative residue process is on, else is 0
    pub nrm_activated_flag: Option<rust_decimal::Decimal>,
    /// Negative residue triggering amount
    pub cumul_negresidue_amount: Option<rust_decimal::Decimal>,
    /// Previous trading interval cumulative negative residue amount
    pub cumul_negresidue_prev_ti: Option<rust_decimal::Decimal>,
    /// Current trading interval negative residue amount
    pub negresidue_current_ti: Option<rust_decimal::Decimal>,
    /// The cumulative negative residue for the next trading interval (PD)
    pub negresidue_pd_next_ti: Option<rust_decimal::Decimal>,
    /// SubjectToReview, Indeterminate, Accepted or Rejected
    pub price_revision: Option<String>,
    /// Predispatch sequence number
    pub predispatchseqno: Option<String>,
    /// The starting DI when NRM event is active
    #[serde(with = "crate::mms_datetime_opt")]
    pub event_activated_di: Option<chrono::NaiveDateTime>,
    /// The finishing DI when NRM event stops being active.
    #[serde(with = "crate::mms_datetime_opt")]
    pub event_deactivated_di: Option<chrono::NaiveDateTime>,
    /// Count of the number of DIs not binding by this constraint
    pub di_notbinding_count: Option<rust_decimal::Decimal>,
    /// Count of the number of DIs violated by this constraint
    pub di_violated_count: Option<rust_decimal::Decimal>,
    /// 1 if constraint is blocked, else 0
    pub nrmconstraint_blocked_flag: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DispatchNegativeResidue1 {
    type PrimaryKey = DispatchNegativeResidue1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DISPATCH".into(),
            table_name: Some("NEGATIVE_RESIDUE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DispatchNegativeResidue1PrimaryKey {
        DispatchNegativeResidue1PrimaryKey {
            directional_interconnectorid: self.directional_interconnectorid.clone(),
            nrm_datetime: self.nrm_datetime,
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "dispatch_negative_residue_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DispatchNegativeResidue1 {
    type Row = DispatchNegativeResidue1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.directional_interconnectorid == row.directional_interconnectorid
            && self.nrm_datetime == row.nrm_datetime
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchNegativeResidue1 {
    type PrimaryKey = DispatchNegativeResidue1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.directional_interconnectorid == key.directional_interconnectorid
            && self.nrm_datetime == key.nrm_datetime
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispatchNegativeResidue1PrimaryKey {
    pub directional_interconnectorid: String,
    pub nrm_datetime: chrono::NaiveDateTime,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DispatchNegativeResidue1PrimaryKey {
    type Row = DispatchNegativeResidue1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.directional_interconnectorid == row.directional_interconnectorid
            && self.nrm_datetime == row.nrm_datetime
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for DispatchNegativeResidue1PrimaryKey {
    type PrimaryKey = DispatchNegativeResidue1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.directional_interconnectorid == key.directional_interconnectorid
            && self.nrm_datetime == key.nrm_datetime
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for DispatchNegativeResidue1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DispatchNegativeResidue1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "nrm_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "directional_interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "nrm_activated_flag",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "cumul_negresidue_amount",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "cumul_negresidue_prev_ti",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "negresidue_current_ti",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "negresidue_pd_next_ti",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "price_revision",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "predispatchseqno",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "event_activated_di",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "event_deactivated_di",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "di_notbinding_count",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "di_violated_count",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "nrmconstraint_blocked_flag",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut nrm_datetime_array = Vec::new();
        let mut directional_interconnectorid_array = Vec::new();
        let mut nrm_activated_flag_array = Vec::new();
        let mut cumul_negresidue_amount_array = Vec::new();
        let mut cumul_negresidue_prev_ti_array = Vec::new();
        let mut negresidue_current_ti_array = Vec::new();
        let mut negresidue_pd_next_ti_array = Vec::new();
        let mut price_revision_array = Vec::new();
        let mut predispatchseqno_array = Vec::new();
        let mut event_activated_di_array = Vec::new();
        let mut event_deactivated_di_array = Vec::new();
        let mut di_notbinding_count_array = Vec::new();
        let mut di_violated_count_array = Vec::new();
        let mut nrmconstraint_blocked_flag_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            nrm_datetime_array.push(row.nrm_datetime.timestamp_millis());
            directional_interconnectorid_array.push(row.directional_interconnectorid);
            nrm_activated_flag_array.push({
                row.nrm_activated_flag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            cumul_negresidue_amount_array.push({
                row.cumul_negresidue_amount.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            cumul_negresidue_prev_ti_array.push({
                row.cumul_negresidue_prev_ti.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            negresidue_current_ti_array.push({
                row.negresidue_current_ti.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            negresidue_pd_next_ti_array.push({
                row.negresidue_pd_next_ti.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            price_revision_array.push(row.price_revision);
            predispatchseqno_array.push(row.predispatchseqno);
            event_activated_di_array.push(row.event_activated_di.map(|val| val.timestamp_millis()));
            event_deactivated_di_array
                .push(row.event_deactivated_di.map(|val| val.timestamp_millis()));
            di_notbinding_count_array.push({
                row.di_notbinding_count.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            di_violated_count_array.push({
                row.di_violated_count.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            nrmconstraint_blocked_flag_array.push({
                row.nrmconstraint_blocked_flag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(nrm_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    directional_interconnectorid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(nrm_activated_flag_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cumul_negresidue_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cumul_negresidue_prev_ti_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(negresidue_current_ti_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(negresidue_pd_next_ti_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(price_revision_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    predispatchseqno_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(event_activated_di_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(event_deactivated_di_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(di_notbinding_count_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(di_violated_count_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(nrmconstraint_blocked_flag_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
