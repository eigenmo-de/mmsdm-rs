/// # Summary
///
/// ## PREDISPATCHBLOCKEDCONSTRAINT
///  _PREDISPATCH Blocked Constraints lists any constraints that were blocked in a Predispatch run. If no constraints are blocked, there will be no rows for that predispatch run._
///
/// * Data Set Name: Predispatch
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
/// * PREDISPATCHSEQNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchBlockedConstraints1 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: crate::TradingPeriod,
    /// Generic Constraint identifier (synonymous with GenConID)
    pub constraintid: String,
}
impl crate::GetTable for PredispatchBlockedConstraints1 {
    type PrimaryKey = PredispatchBlockedConstraints1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: Some("BLOCKED_CONSTRAINTS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> PredispatchBlockedConstraints1PrimaryKey {
        PredispatchBlockedConstraints1PrimaryKey {
            constraintid: self.constraintid.clone(),
            predispatchseqno: self.predispatchseqno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "predispatch_blocked_constraints_v1".to_string()
    }
}
impl crate::CompareWithRow for PredispatchBlockedConstraints1 {
    type Row = PredispatchBlockedConstraints1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid && self.predispatchseqno == row.predispatchseqno
    }
}
impl crate::CompareWithPrimaryKey for PredispatchBlockedConstraints1 {
    type PrimaryKey = PredispatchBlockedConstraints1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.predispatchseqno == key.predispatchseqno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PredispatchBlockedConstraints1PrimaryKey {
    pub constraintid: String,
    pub predispatchseqno: crate::TradingPeriod,
}
impl crate::CompareWithRow for PredispatchBlockedConstraints1PrimaryKey {
    type Row = PredispatchBlockedConstraints1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid && self.predispatchseqno == row.predispatchseqno
    }
}
impl crate::CompareWithPrimaryKey for PredispatchBlockedConstraints1PrimaryKey {
    type PrimaryKey = PredispatchBlockedConstraints1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.predispatchseqno == key.predispatchseqno
    }
}
impl crate::PrimaryKey for PredispatchBlockedConstraints1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PredispatchBlockedConstraints1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "predispatchseqno",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "constraintid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut predispatchseqno_array = Vec::new();
        let mut constraintid_array = Vec::new();
        for (_, row) in partition {
            predispatchseqno_array.push(row.predispatchseqno.start().timestamp_millis());
            constraintid_array.push(row.constraintid);
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(predispatchseqno_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    constraintid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PREDISPATCHCASESOLUTION
///  _PREDISPATCHCASESOLUTION provides information relating to the complete predispatch run. The fields provide an overview of the dispatch run results allowing immediate identification of conditions such as energy or FCAS deficiencies._
///
/// * Data Set Name: Predispatch
/// * File Name: Case Solution
/// * Data Version: 1
///
/// # Description
///  PREDISPATCHCASESOLUTION data is public, so is available to all participants. Source PREDISPATCHCASESOLUTION updates every half-hour. Volume Approximately 48 records per day.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * PREDISPATCHSEQNO
/// * RUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchCaseSolution1 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: crate::TradingPeriod,
    /// Predispatch run no, normally 1.
    pub runno: rust_decimal::Decimal,
    /// If non-zero indicated one of the following conditions: 1 = Supply Scarcity, Excess generation or constraint violations, -X = Model failure
    pub solutionstatus: Option<rust_decimal::Decimal>,
    /// Current version of SPD
    pub spdversion: Option<String>,
    /// Non-Physical Losses algorithm invoked during this run
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
    /// Total of Energy Constrained unit offer violations.
    pub totalenergyconstrviolation: Option<rust_decimal::Decimal>,
    /// Total of unit summated offer band violations
    pub totalenergyofferviolation: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag to indicate if this Pre-Dispatch case includes an intervention pricing run: 0 = case does not include an intervention pricing run, 1 = case does include an intervention pricing run. This field has a default value of 0 and is not nullable
    pub intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for PredispatchCaseSolution1 {
    type PrimaryKey = PredispatchCaseSolution1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: Some("CASE_SOLUTION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> PredispatchCaseSolution1PrimaryKey {
        PredispatchCaseSolution1PrimaryKey {
            predispatchseqno: self.predispatchseqno,
            runno: self.runno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "predispatch_case_solution_v1".to_string()
    }
}
impl crate::CompareWithRow for PredispatchCaseSolution1 {
    type Row = PredispatchCaseSolution1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.predispatchseqno == row.predispatchseqno && self.runno == row.runno
    }
}
impl crate::CompareWithPrimaryKey for PredispatchCaseSolution1 {
    type PrimaryKey = PredispatchCaseSolution1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.predispatchseqno == key.predispatchseqno && self.runno == key.runno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PredispatchCaseSolution1PrimaryKey {
    pub predispatchseqno: crate::TradingPeriod,
    pub runno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for PredispatchCaseSolution1PrimaryKey {
    type Row = PredispatchCaseSolution1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.predispatchseqno == row.predispatchseqno && self.runno == row.runno
    }
}
impl crate::CompareWithPrimaryKey for PredispatchCaseSolution1PrimaryKey {
    type PrimaryKey = PredispatchCaseSolution1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.predispatchseqno == key.predispatchseqno && self.runno == key.runno
    }
}
impl crate::PrimaryKey for PredispatchCaseSolution1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PredispatchCaseSolution1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "predispatchseqno",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
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
                "totalenergyconstrviolation",
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
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut predispatchseqno_array = Vec::new();
        let mut runno_array = Vec::new();
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
        let mut totalenergyconstrviolation_array = Vec::new();
        let mut totalenergyofferviolation_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut intervention_array = Vec::new();
        for (_, row) in partition {
            predispatchseqno_array.push(row.predispatchseqno.start().timestamp_millis());
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
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
            totalenergyconstrviolation_array.push({
                row.totalenergyconstrviolation.map(|mut val| {
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
            intervention_array.push({
                row.intervention.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(predispatchseqno_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(solutionstatus_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(spdversion_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(nonphysicallosses_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalobjective_array)
                        .to(arrow2::datatypes::DataType::Decimal(27, 10)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalareagenviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalinterconnectorviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalgenericviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalramprateviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalunitmwcapacityviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(total5minviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalregviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(total6secviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(total60secviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalasprofileviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalenergyconstrviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalenergyofferviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PREDISPATCHCONSTRAINT
///  _PREDISPATCHCONSTRAINT sets out constraints that are binding in each predispatch run and interconnector constraints (whether binding or not). Only binding and interconnector constraints are reported. Binding contracts have marginal value greater than $0. Interconnector constraints are listed so RHS values can be reported for ST PASA.<br>Constraint solutions only report fixed loading /MR constraints on the next day.<br>_
///
/// * Data Set Name: Predispatch
/// * File Name: Constraint Solution
/// * Data Version: 5
///
/// # Description
///  PREDISPATCHCONSTRAINT data is confidential on the day of creation, and public to all participants after the end of the market day. Source PREDISPATCHCONSTRAINT updates with every thirty-minute predispatch run. Note The PERIODID columns in tables PREDISPATCHCONSTRAINT and PREDISPATCH_FCAS_REQ have no consistent relationship with the other PERIODID values in the other tables in the PRE-DISPATCH package (such as PREDISPATCHPRICE). AEMO and many Participants appreciate the data model is inconsistent, but the cost of changing existing systems has been judged as being unjustifiable. An additional field DATETIME was added to allow joins between these data sets.
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchConstraintSolution5 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: crate::TradingPeriod,
    /// SPD Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Generic constraint identifier
    pub constraintid: String,
    /// Unique period identifier, in the format yyyymmddpp. The period (pp) is 01 to 48, with 01 corresponding to the half-hour ending at 04:30am.
    pub periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
    /// RHS value used.
    pub rhs: Option<rust_decimal::Decimal>,
    /// Marginal value of violated constraint
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Degree of constraint violation
    pub violationdegree: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Period date and time
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
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
impl crate::GetTable for PredispatchConstraintSolution5 {
    type PrimaryKey = PredispatchConstraintSolution5PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: Some("CONSTRAINT_SOLUTION".into()),
            version: 5,
        }
    }

    fn primary_key(&self) -> PredispatchConstraintSolution5PrimaryKey {
        PredispatchConstraintSolution5PrimaryKey {
            constraintid: self.constraintid.clone(),
            datetime: self.datetime,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "predispatch_constraint_solution_v5".to_string()
    }
}
impl crate::CompareWithRow for PredispatchConstraintSolution5 {
    type Row = PredispatchConstraintSolution5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid && self.datetime == row.datetime
    }
}
impl crate::CompareWithPrimaryKey for PredispatchConstraintSolution5 {
    type PrimaryKey = PredispatchConstraintSolution5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.datetime == key.datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PredispatchConstraintSolution5PrimaryKey {
    pub constraintid: String,
    pub datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for PredispatchConstraintSolution5PrimaryKey {
    type Row = PredispatchConstraintSolution5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid && self.datetime == row.datetime
    }
}
impl crate::CompareWithPrimaryKey for PredispatchConstraintSolution5PrimaryKey {
    type PrimaryKey = PredispatchConstraintSolution5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.datetime == key.datetime
    }
}
impl crate::PrimaryKey for PredispatchConstraintSolution5PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PredispatchConstraintSolution5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "predispatchseqno",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "constraintid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
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
            arrow2::datatypes::Field::new("datetime", arrow2::datatypes::DataType::Date64, false),
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

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut predispatchseqno_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut constraintid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut rhs_array = Vec::new();
        let mut marginalvalue_array = Vec::new();
        let mut violationdegree_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut datetime_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut genconid_effectivedate_array = Vec::new();
        let mut genconid_versionno_array = Vec::new();
        let mut lhs_array = Vec::new();
        for (_, row) in partition {
            predispatchseqno_array.push(row.predispatchseqno.start().timestamp_millis());
            runno_array.push({
                row.runno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            constraintid_array.push(row.constraintid);
            periodid_array.push(row.periodid);
            intervention_array.push({
                row.intervention.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
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
            datetime_array.push(row.datetime.timestamp_millis());
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

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(predispatchseqno_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    constraintid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(periodid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rhs_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginalvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(violationdegree_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(duid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(genconid_effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(genconid_versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lhs_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PREDISPATCHINTERCONNECTORRES
///  _PREDISPATCHINTERCONNECTORRES records Interconnector flows and losses for the periods calculated in each predispatch run. Only binding and interconnector constraints are reported.<br>Some fields are for the Frequency Controlled Ancillary Services export and import limits and extra reporting of the generic constraint setting the energy import and export limits.<br>_
///
/// * Data Set Name: Predispatch
/// * File Name: Interconnector Soln
/// * Data Version: 3
///
/// # Description
///  Source PREDISPATCHINTERCONNECTORRES updates with every thirty-minute predispatch run. Note MW losses can be negative depending on the flow. The definition of direction of flow for an interconnector is that positive flow starts from the FROMREGION in INTERCONNECTOR.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DATETIME
/// * INTERCONNECTORID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchInterconnectorSoln3 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: crate::TradingPeriod,
    /// SPD Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Interconnector identifier
    pub interconnectorid: String,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
    /// Metered MW Flow from EMS. For periods subsequent to the first period of a Pre-Dispatch run, this value represents the cleared target for the previous period of that Pre-Dispatch run.
    pub meteredmwflow: Option<rust_decimal::Decimal>,
    /// Calculated MW Flow
    pub mwflow: Option<rust_decimal::Decimal>,
    /// Calculated MW Losses
    pub mwlosses: Option<rust_decimal::Decimal>,
    /// $ Marginal value of interconnector constraint from SPD
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Degree of violation of interconnector constraint in MW
    pub violationdegree: Option<rust_decimal::Decimal>,
    /// Last changed.
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Period date and time
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
    /// Calculated export limit.
    pub exportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit.
    pub importlimit: Option<rust_decimal::Decimal>,
    /// Marginal loss factor. Use this to adjust bids between reports.
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
impl crate::GetTable for PredispatchInterconnectorSoln3 {
    type PrimaryKey = PredispatchInterconnectorSoln3PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: Some("INTERCONNECTOR_SOLN".into()),
            version: 3,
        }
    }

    fn primary_key(&self) -> PredispatchInterconnectorSoln3PrimaryKey {
        PredispatchInterconnectorSoln3PrimaryKey {
            datetime: self.datetime,
            interconnectorid: self.interconnectorid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "predispatch_interconnector_soln_v3".to_string()
    }
}
impl crate::CompareWithRow for PredispatchInterconnectorSoln3 {
    type Row = PredispatchInterconnectorSoln3;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.datetime == row.datetime && self.interconnectorid == row.interconnectorid
    }
}
impl crate::CompareWithPrimaryKey for PredispatchInterconnectorSoln3 {
    type PrimaryKey = PredispatchInterconnectorSoln3PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.interconnectorid == key.interconnectorid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PredispatchInterconnectorSoln3PrimaryKey {
    pub datetime: chrono::NaiveDateTime,
    pub interconnectorid: String,
}
impl crate::CompareWithRow for PredispatchInterconnectorSoln3PrimaryKey {
    type Row = PredispatchInterconnectorSoln3;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.datetime == row.datetime && self.interconnectorid == row.interconnectorid
    }
}
impl crate::CompareWithPrimaryKey for PredispatchInterconnectorSoln3PrimaryKey {
    type PrimaryKey = PredispatchInterconnectorSoln3PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.interconnectorid == key.interconnectorid
    }
}
impl crate::PrimaryKey for PredispatchInterconnectorSoln3PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PredispatchInterconnectorSoln3 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "predispatchseqno",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
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
            arrow2::datatypes::Field::new("datetime", arrow2::datatypes::DataType::Date64, false),
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

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut predispatchseqno_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut meteredmwflow_array = Vec::new();
        let mut mwflow_array = Vec::new();
        let mut mwlosses_array = Vec::new();
        let mut marginalvalue_array = Vec::new();
        let mut violationdegree_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut datetime_array = Vec::new();
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
            predispatchseqno_array.push(row.predispatchseqno.start().timestamp_millis());
            runno_array.push({
                row.runno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            interconnectorid_array.push(row.interconnectorid);
            periodid_array.push(row.periodid);
            intervention_array.push({
                row.intervention.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
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
            datetime_array.push(row.datetime.timestamp_millis());
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

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(predispatchseqno_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(periodid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(meteredmwflow_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwlosses_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginalvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(violationdegree_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(exportlimit_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(importlimit_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginalloss_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(exportgenconid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(importgenconid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(fcasexportlimit_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(fcasimportlimit_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(local_price_adjustment_export_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(locally_constrained_export_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(local_price_adjustment_import_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(locally_constrained_import_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PREDISPATCHINTERSENSITIVITIES
///  _PREDISPATCHINTERSENSITIVITIES sets out the sensitivity flows for each interconnector by period._
///
/// * Data Set Name: Predispatch
/// * File Name: Interconnectr Sens
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DATETIME
/// * INTERCONNECTORID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchInterconnectrSens1 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: crate::TradingPeriod,
    /// LP Solver Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Unique interconnector identifier
    pub interconnectorid: String,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
    /// Period date and time
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
    /// Flag to indicate if the sensitivity run contains an active intervention constraint: 0 = No, 1 = Yes
    pub intervention_active: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 1
    pub mwflow1: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 2
    pub mwflow2: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 3
    pub mwflow3: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 4
    pub mwflow4: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 5
    pub mwflow5: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 6
    pub mwflow6: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 7
    pub mwflow7: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 8
    pub mwflow8: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 9
    pub mwflow9: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 10
    pub mwflow10: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 11
    pub mwflow11: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 12
    pub mwflow12: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 13
    pub mwflow13: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 14
    pub mwflow14: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 15
    pub mwflow15: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 16
    pub mwflow16: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 17
    pub mwflow17: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 18
    pub mwflow18: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 19
    pub mwflow19: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 20
    pub mwflow20: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 21
    pub mwflow21: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 22
    pub mwflow22: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 23
    pub mwflow23: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 24
    pub mwflow24: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 25
    pub mwflow25: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 26
    pub mwflow26: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 27
    pub mwflow27: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 28
    pub mwflow28: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 29
    pub mwflow29: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 30
    pub mwflow30: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 31
    pub mwflow31: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 32
    pub mwflow32: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 33
    pub mwflow33: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 34
    pub mwflow34: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 35
    pub mwflow35: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 36
    pub mwflow36: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 37
    pub mwflow37: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 38
    pub mwflow38: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 39
    pub mwflow39: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 40
    pub mwflow40: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 41
    pub mwflow41: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 42
    pub mwflow42: Option<rust_decimal::Decimal>,
    /// MW flow for given Interconnector for scenario 43
    pub mwflow43: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for PredispatchInterconnectrSens1 {
    type PrimaryKey = PredispatchInterconnectrSens1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: Some("INTERCONNECTR_SENS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> PredispatchInterconnectrSens1PrimaryKey {
        PredispatchInterconnectrSens1PrimaryKey {
            datetime: self.datetime,
            interconnectorid: self.interconnectorid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "predispatch_interconnectr_sens_v1".to_string()
    }
}
impl crate::CompareWithRow for PredispatchInterconnectrSens1 {
    type Row = PredispatchInterconnectrSens1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.datetime == row.datetime && self.interconnectorid == row.interconnectorid
    }
}
impl crate::CompareWithPrimaryKey for PredispatchInterconnectrSens1 {
    type PrimaryKey = PredispatchInterconnectrSens1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.interconnectorid == key.interconnectorid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PredispatchInterconnectrSens1PrimaryKey {
    pub datetime: chrono::NaiveDateTime,
    pub interconnectorid: String,
}
impl crate::CompareWithRow for PredispatchInterconnectrSens1PrimaryKey {
    type Row = PredispatchInterconnectrSens1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.datetime == row.datetime && self.interconnectorid == row.interconnectorid
    }
}
impl crate::CompareWithPrimaryKey for PredispatchInterconnectrSens1PrimaryKey {
    type PrimaryKey = PredispatchInterconnectrSens1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.interconnectorid == key.interconnectorid
    }
}
impl crate::PrimaryKey for PredispatchInterconnectrSens1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PredispatchInterconnectrSens1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "predispatchseqno",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
            arrow2::datatypes::Field::new("datetime", arrow2::datatypes::DataType::Date64, false),
            arrow2::datatypes::Field::new(
                "intervention_active",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow1",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow2",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow3",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow4",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow5",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow6",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow7",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow8",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow9",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow10",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow11",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow12",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow13",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow14",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow15",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow16",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow17",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow18",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow19",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow20",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow21",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow22",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow23",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow24",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow25",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow26",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow27",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow28",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow29",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow30",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow31",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow32",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow33",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow34",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow35",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow36",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow37",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow38",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow39",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow40",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow41",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow42",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mwflow43",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut predispatchseqno_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut datetime_array = Vec::new();
        let mut intervention_active_array = Vec::new();
        let mut mwflow1_array = Vec::new();
        let mut mwflow2_array = Vec::new();
        let mut mwflow3_array = Vec::new();
        let mut mwflow4_array = Vec::new();
        let mut mwflow5_array = Vec::new();
        let mut mwflow6_array = Vec::new();
        let mut mwflow7_array = Vec::new();
        let mut mwflow8_array = Vec::new();
        let mut mwflow9_array = Vec::new();
        let mut mwflow10_array = Vec::new();
        let mut mwflow11_array = Vec::new();
        let mut mwflow12_array = Vec::new();
        let mut mwflow13_array = Vec::new();
        let mut mwflow14_array = Vec::new();
        let mut mwflow15_array = Vec::new();
        let mut mwflow16_array = Vec::new();
        let mut mwflow17_array = Vec::new();
        let mut mwflow18_array = Vec::new();
        let mut mwflow19_array = Vec::new();
        let mut mwflow20_array = Vec::new();
        let mut mwflow21_array = Vec::new();
        let mut mwflow22_array = Vec::new();
        let mut mwflow23_array = Vec::new();
        let mut mwflow24_array = Vec::new();
        let mut mwflow25_array = Vec::new();
        let mut mwflow26_array = Vec::new();
        let mut mwflow27_array = Vec::new();
        let mut mwflow28_array = Vec::new();
        let mut mwflow29_array = Vec::new();
        let mut mwflow30_array = Vec::new();
        let mut mwflow31_array = Vec::new();
        let mut mwflow32_array = Vec::new();
        let mut mwflow33_array = Vec::new();
        let mut mwflow34_array = Vec::new();
        let mut mwflow35_array = Vec::new();
        let mut mwflow36_array = Vec::new();
        let mut mwflow37_array = Vec::new();
        let mut mwflow38_array = Vec::new();
        let mut mwflow39_array = Vec::new();
        let mut mwflow40_array = Vec::new();
        let mut mwflow41_array = Vec::new();
        let mut mwflow42_array = Vec::new();
        let mut mwflow43_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            predispatchseqno_array.push(row.predispatchseqno.start().timestamp_millis());
            runno_array.push({
                row.runno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            interconnectorid_array.push(row.interconnectorid);
            periodid_array.push(row.periodid);
            intervention_array.push({
                row.intervention.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            datetime_array.push(row.datetime.timestamp_millis());
            intervention_active_array.push({
                row.intervention_active.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            mwflow1_array.push({
                row.mwflow1.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow2_array.push({
                row.mwflow2.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow3_array.push({
                row.mwflow3.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow4_array.push({
                row.mwflow4.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow5_array.push({
                row.mwflow5.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow6_array.push({
                row.mwflow6.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow7_array.push({
                row.mwflow7.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow8_array.push({
                row.mwflow8.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow9_array.push({
                row.mwflow9.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow10_array.push({
                row.mwflow10.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow11_array.push({
                row.mwflow11.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow12_array.push({
                row.mwflow12.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow13_array.push({
                row.mwflow13.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow14_array.push({
                row.mwflow14.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow15_array.push({
                row.mwflow15.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow16_array.push({
                row.mwflow16.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow17_array.push({
                row.mwflow17.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow18_array.push({
                row.mwflow18.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow19_array.push({
                row.mwflow19.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow20_array.push({
                row.mwflow20.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow21_array.push({
                row.mwflow21.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow22_array.push({
                row.mwflow22.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow23_array.push({
                row.mwflow23.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow24_array.push({
                row.mwflow24.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow25_array.push({
                row.mwflow25.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow26_array.push({
                row.mwflow26.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow27_array.push({
                row.mwflow27.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow28_array.push({
                row.mwflow28.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow29_array.push({
                row.mwflow29.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow30_array.push({
                row.mwflow30.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow31_array.push({
                row.mwflow31.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow32_array.push({
                row.mwflow32.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow33_array.push({
                row.mwflow33.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow34_array.push({
                row.mwflow34.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow35_array.push({
                row.mwflow35.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow36_array.push({
                row.mwflow36.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow37_array.push({
                row.mwflow37.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow38_array.push({
                row.mwflow38.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow39_array.push({
                row.mwflow39.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow40_array.push({
                row.mwflow40.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow41_array.push({
                row.mwflow41.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow42_array.push({
                row.mwflow42.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mwflow43_array.push({
                row.mwflow43.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(predispatchseqno_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(periodid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_active_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow1_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow2_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow3_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow4_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow5_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow6_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow7_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow8_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow9_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow10_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow11_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow12_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow13_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow14_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow15_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow16_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow17_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow18_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow19_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow20_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow21_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow22_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow23_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow24_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow25_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow26_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow27_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow28_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow29_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow30_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow31_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow32_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow33_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow34_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow35_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow36_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow37_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow38_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow39_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow40_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow41_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow42_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow43_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PREDISPATCHLOAD
///  _PREDISPATCHLOAD shows pre-dispatch targets for each dispatchable unit, including additional fields to handle the Ancillary Services functionality. No record is written where a unit is not dispatched. PREDISPATCHLOAD shows all the results for each period._
///
/// * Data Set Name: Predispatch
/// * File Name: Unit Solution
/// * Data Version: 2
///
/// # Description
///  Source Own (confidential) data updates every thirty minutes, with whole market data for the day before available as part of next day market data. Note ** A flag exists for each ancillary service type such that a unit trapped or stranded in one or more service type can be immediately identified. The flag is defined using the low 3 bits as follows: Flag Name Bit Description Enabled 0 The unit is enabled to provide this ancillary service type. Trapped 1 The unit is enabled to provide this ancillary service type, however the profile for this service type is causing the unit to be trapped in the energy market. Stranded 2 The unit is bid available to provide this ancillary service type, however, the unit is operating in the energy market outside of the profile for this service type and is stranded from providing this service. Interpretation of the bit-flags as a number gives the following possibilities (i.e. other combinations are not possible): Numeric Value Bit (2,1,0) Meaning 0 000 Not stranded, not trapped, not enabled. 1 001 Not stranded, not trapped, is enabled. 3 011 Not stranded, is trapped, is enabled. 4 100 Is stranded, not trapped, not enabled. For example, testing for availability can be done by checking for odd (=available) or even (=unavailable) number (e.g.  mod(flag,2)  results in 0 for unavailable and 1 for available). *** "Actual FCAS availability" is determined in a post-processing step based on the energy target (TotalCleared) and bid FCAS trapezium for that interval. However, if the unit is outside the bid FCAS trapezium at the start of the interval (InitialMW), the "Actual FCAS availability" is set to zero. For regulation services, the trapezium is the most restrictive of the bid/SCADA trapezium values.
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * DATETIME
/// * DUID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchUnitSolution2 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: crate::TradingPeriod,
    /// SPD Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Dispatchable unit identifier for fast start
    pub duid: String,
    /// Not used
    pub tradetype: Option<rust_decimal::Decimal>,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
    /// Connection point identifier
    pub connectionpointid: Option<String>,
    /// AGC Status from EMS
    pub agcstatus: Option<rust_decimal::Decimal>,
    /// Dispatch mode of unit for fast start (1-4)
    pub dispatchmode: Option<rust_decimal::Decimal>,
    /// Initial MW at start of first period. For periods subsequent to the first period of a Pre-Dispatch run, this value represents the cleared target for the previous period of that Pre-Dispatch run.
    pub initialmw: Option<rust_decimal::Decimal>,
    /// Target MW at end of period
    pub totalcleared: Option<rust_decimal::Decimal>,
    /// Lower 5 min MW target in period
    pub lower5min: Option<rust_decimal::Decimal>,
    /// Lower 60 sec MW target in period
    pub lower60sec: Option<rust_decimal::Decimal>,
    /// Lower 6 sec MW target in period
    pub lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 5 min MW target in period
    pub raise5min: Option<rust_decimal::Decimal>,
    /// Raise 60 sec MW target in period
    pub raise60sec: Option<rust_decimal::Decimal>,
    /// Raise 6 sec MW target in period
    pub raise6sec: Option<rust_decimal::Decimal>,
    /// Ramp down rate in period in MW/minute
    pub rampdownrate: Option<rust_decimal::Decimal>,
    /// Ramp up rate in period in MW/minute
    pub rampuprate: Option<rust_decimal::Decimal>,
    /// Not used in Pre-Dispatch
    pub downepf: Option<rust_decimal::Decimal>,
    /// Not used in Pre-Dispatch
    pub upepf: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 5 min from LP Solver
    pub marginal5minvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 60 seconds from LP Solver
    pub marginal60secvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for 6 seconds from LP Solver
    pub marginal6secvalue: Option<rust_decimal::Decimal>,
    /// Marginal $ value for energy from LP Solver
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
    /// Period date and time
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
    /// Lower Regulation reserve target
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve target
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Bid energy availability
    pub availability: Option<rust_decimal::Decimal>,
    /// Raise 6sec status flag
    pub raise6secflags: Option<rust_decimal::Decimal>,
    /// Raise 60sec status flag
    pub raise60secflags: Option<rust_decimal::Decimal>,
    /// Raise 5min status flag
    pub raise5minflags: Option<rust_decimal::Decimal>,
    /// Raise reg status flag
    pub raiseregflags: Option<rust_decimal::Decimal>,
    /// Lower 6sec status flag
    pub lower6secflags: Option<rust_decimal::Decimal>,
    /// Lower 60sec status flag
    pub lower60secflags: Option<rust_decimal::Decimal>,
    /// Lower 5min status flag  
    pub lower5minflags: Option<rust_decimal::Decimal>,
    /// Lower Reg status flag  
    pub lowerregflags: Option<rust_decimal::Decimal>,
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
}
impl crate::GetTable for PredispatchUnitSolution2 {
    type PrimaryKey = PredispatchUnitSolution2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: Some("UNIT_SOLUTION".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> PredispatchUnitSolution2PrimaryKey {
        PredispatchUnitSolution2PrimaryKey {
            datetime: self.datetime,
            duid: self.duid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "predispatch_unit_solution_v2".to_string()
    }
}
impl crate::CompareWithRow for PredispatchUnitSolution2 {
    type Row = PredispatchUnitSolution2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.datetime == row.datetime && self.duid == row.duid
    }
}
impl crate::CompareWithPrimaryKey for PredispatchUnitSolution2 {
    type PrimaryKey = PredispatchUnitSolution2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.duid == key.duid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PredispatchUnitSolution2PrimaryKey {
    pub datetime: chrono::NaiveDateTime,
    pub duid: String,
}
impl crate::CompareWithRow for PredispatchUnitSolution2PrimaryKey {
    type Row = PredispatchUnitSolution2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.datetime == row.datetime && self.duid == row.duid
    }
}
impl crate::CompareWithPrimaryKey for PredispatchUnitSolution2PrimaryKey {
    type PrimaryKey = PredispatchUnitSolution2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.duid == key.duid
    }
}
impl crate::PrimaryKey for PredispatchUnitSolution2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PredispatchUnitSolution2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "predispatchseqno",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "tradetype",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "agcstatus",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "dispatchmode",
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
            arrow2::datatypes::Field::new("datetime", arrow2::datatypes::DataType::Date64, false),
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
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut predispatchseqno_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut tradetype_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut agcstatus_array = Vec::new();
        let mut dispatchmode_array = Vec::new();
        let mut initialmw_array = Vec::new();
        let mut totalcleared_array = Vec::new();
        let mut lower5min_array = Vec::new();
        let mut lower60sec_array = Vec::new();
        let mut lower6sec_array = Vec::new();
        let mut raise5min_array = Vec::new();
        let mut raise60sec_array = Vec::new();
        let mut raise6sec_array = Vec::new();
        let mut rampdownrate_array = Vec::new();
        let mut rampuprate_array = Vec::new();
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
        let mut datetime_array = Vec::new();
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
        let mut raise6secactualavailability_array = Vec::new();
        let mut raise60secactualavailability_array = Vec::new();
        let mut raise5minactualavailability_array = Vec::new();
        let mut raiseregactualavailability_array = Vec::new();
        let mut lower6secactualavailability_array = Vec::new();
        let mut lower60secactualavailability_array = Vec::new();
        let mut lower5minactualavailability_array = Vec::new();
        let mut lowerregactualavailability_array = Vec::new();
        let mut semidispatchcap_array = Vec::new();
        for (_, row) in partition {
            predispatchseqno_array.push(row.predispatchseqno.start().timestamp_millis());
            runno_array.push({
                row.runno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            duid_array.push(row.duid);
            tradetype_array.push({
                row.tradetype.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            periodid_array.push(row.periodid);
            intervention_array.push({
                row.intervention.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            connectionpointid_array.push(row.connectionpointid);
            agcstatus_array.push({
                row.agcstatus.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            dispatchmode_array.push({
                row.dispatchmode.map(|mut val| {
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
            datetime_array.push(row.datetime.timestamp_millis());
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
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(predispatchseqno_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(tradetype_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(periodid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    connectionpointid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(agcstatus_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(dispatchmode_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(initialmw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalcleared_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5min_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60sec_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6sec_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5min_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60sec_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6sec_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rampdownrate_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rampuprate_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(downepf_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(upepf_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginal5minvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginal60secvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginal6secvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginalvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(violation5mindegree_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(violation60secdegree_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(violation6secdegree_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(violationdegree_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerreg_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raisereg_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(availability_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secflags_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secflags_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minflags_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregflags_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secflags_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secflags_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minflags_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregflags_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(semidispatchcap_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PREDISPATCHOFFERTRK
///  _PREDISPATCHOFFERTRK is for the ancillary service bid tracking of predispatch processing. PREDISPATCHOFFERTRK identifies which bids from BIDDAYOFFER and BIDOFFERPERIOD were applied for a given unit and ancillary service for each predispatch run._
///
/// * Data Set Name: Predispatch
/// * File Name: Offertrk
/// * Data Version: 1
///
/// # Description
///  Source PREDISPATCHOFFERTRK updates every 30 minutes. The data is confidential to each participant until the next trading day.  Volume Approximately 45,000 records per day.
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DUID
/// * PERIODID
/// * PREDISPATCHSEQNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchOffertrk1 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: crate::TradingPeriod,
    /// Dispatchable Unit identifier
    pub duid: String,
    /// Bid type Identifier - the ancillary service to which the bid applies
    pub bidtype: String,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: String,
    /// Settlement date of bid applied
    #[serde(with = "crate::mms_datetime_opt")]
    pub bidsettlementdate: Option<chrono::NaiveDateTime>,
    /// Time this bid was processed and loaded
    #[serde(with = "crate::mms_datetime_opt")]
    pub bidofferdate: Option<chrono::NaiveDateTime>,
    /// Period date and time
    #[serde(with = "crate::mms_datetime_opt")]
    pub datetime: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for PredispatchOffertrk1 {
    type PrimaryKey = PredispatchOffertrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: Some("OFFERTRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> PredispatchOffertrk1PrimaryKey {
        PredispatchOffertrk1PrimaryKey {
            bidtype: self.bidtype.clone(),
            duid: self.duid.clone(),
            periodid: self.periodid.clone(),
            predispatchseqno: self.predispatchseqno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "predispatch_offertrk_v1".to_string()
    }
}
impl crate::CompareWithRow for PredispatchOffertrk1 {
    type Row = PredispatchOffertrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.duid == row.duid
            && self.periodid == row.periodid
            && self.predispatchseqno == row.predispatchseqno
    }
}
impl crate::CompareWithPrimaryKey for PredispatchOffertrk1 {
    type PrimaryKey = PredispatchOffertrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.duid == key.duid
            && self.periodid == key.periodid
            && self.predispatchseqno == key.predispatchseqno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PredispatchOffertrk1PrimaryKey {
    pub bidtype: String,
    pub duid: String,
    pub periodid: String,
    pub predispatchseqno: crate::TradingPeriod,
}
impl crate::CompareWithRow for PredispatchOffertrk1PrimaryKey {
    type Row = PredispatchOffertrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.duid == row.duid
            && self.periodid == row.periodid
            && self.predispatchseqno == row.predispatchseqno
    }
}
impl crate::CompareWithPrimaryKey for PredispatchOffertrk1PrimaryKey {
    type PrimaryKey = PredispatchOffertrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.duid == key.duid
            && self.periodid == key.periodid
            && self.predispatchseqno == key.predispatchseqno
    }
}
impl crate::PrimaryKey for PredispatchOffertrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PredispatchOffertrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "predispatchseqno",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("bidtype", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
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
            arrow2::datatypes::Field::new("datetime", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut predispatchseqno_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut bidtype_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut bidsettlementdate_array = Vec::new();
        let mut bidofferdate_array = Vec::new();
        let mut datetime_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            predispatchseqno_array.push(row.predispatchseqno.start().timestamp_millis());
            duid_array.push(row.duid);
            bidtype_array.push(row.bidtype);
            periodid_array.push(row.periodid);
            bidsettlementdate_array.push(row.bidsettlementdate.map(|val| val.timestamp_millis()));
            bidofferdate_array.push(row.bidofferdate.map(|val| val.timestamp_millis()));
            datetime_array.push(row.datetime.map(|val| val.timestamp_millis()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(predispatchseqno_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bidtype_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(periodid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bidsettlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bidofferdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PREDISPATCHPRICE
///  _PREDISPATCHPRICE records predispatch prices for each region by period for each predispatch run, including fields to handle the Ancillary Services functionality._
///
/// * Data Set Name: Predispatch
/// * File Name: Region Prices
/// * Data Version: 1
///
/// # Description
///  PREDISPATCHPRICE data is public, so is available to all participants. Source PREDISPATCHPRICE updates with every thirty-minute predispatch run.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DATETIME
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchRegionPrices1 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: crate::TradingPeriod,
    /// LP Solver Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Unique region identifier
    pub regionid: String,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess energy price
    pub eep: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp1: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep1: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp2: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep2: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp3: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep3: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp4: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep4: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp5: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep5: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp6: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep6: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp7: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep7: Option<rust_decimal::Decimal>,
    /// Not used
    pub rrp8: Option<rust_decimal::Decimal>,
    /// Not used
    pub eep8: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Period date and time
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
    /// Regional reference price for this dispatch period
    pub raise6secrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raise60secrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raise5minrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raiseregrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lower6secrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lower60secrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lower5minrrp: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lowerregrrp: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for PredispatchRegionPrices1 {
    type PrimaryKey = PredispatchRegionPrices1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: Some("REGION_PRICES".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> PredispatchRegionPrices1PrimaryKey {
        PredispatchRegionPrices1PrimaryKey {
            datetime: self.datetime,
            regionid: self.regionid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "predispatch_region_prices_v1".to_string()
    }
}
impl crate::CompareWithRow for PredispatchRegionPrices1 {
    type Row = PredispatchRegionPrices1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.datetime == row.datetime && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for PredispatchRegionPrices1 {
    type PrimaryKey = PredispatchRegionPrices1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.regionid == key.regionid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PredispatchRegionPrices1PrimaryKey {
    pub datetime: chrono::NaiveDateTime,
    pub regionid: String,
}
impl crate::CompareWithRow for PredispatchRegionPrices1PrimaryKey {
    type Row = PredispatchRegionPrices1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.datetime == row.datetime && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for PredispatchRegionPrices1PrimaryKey {
    type PrimaryKey = PredispatchRegionPrices1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.regionid == key.regionid
    }
}
impl crate::PrimaryKey for PredispatchRegionPrices1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PredispatchRegionPrices1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "predispatchseqno",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
            arrow2::datatypes::Field::new("rrp", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new("eep", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new(
                "rrp1",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "eep1",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp2",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "eep2",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp3",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "eep3",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp4",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "eep4",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp5",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "eep5",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp6",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "eep6",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp7",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "eep7",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp8",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "eep8",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("datetime", arrow2::datatypes::DataType::Date64, false),
            arrow2::datatypes::Field::new(
                "raise6secrrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secrrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minrrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raiseregrrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secrrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secrrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minrrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerregrrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut predispatchseqno_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut rrp_array = Vec::new();
        let mut eep_array = Vec::new();
        let mut rrp1_array = Vec::new();
        let mut eep1_array = Vec::new();
        let mut rrp2_array = Vec::new();
        let mut eep2_array = Vec::new();
        let mut rrp3_array = Vec::new();
        let mut eep3_array = Vec::new();
        let mut rrp4_array = Vec::new();
        let mut eep4_array = Vec::new();
        let mut rrp5_array = Vec::new();
        let mut eep5_array = Vec::new();
        let mut rrp6_array = Vec::new();
        let mut eep6_array = Vec::new();
        let mut rrp7_array = Vec::new();
        let mut eep7_array = Vec::new();
        let mut rrp8_array = Vec::new();
        let mut eep8_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut datetime_array = Vec::new();
        let mut raise6secrrp_array = Vec::new();
        let mut raise60secrrp_array = Vec::new();
        let mut raise5minrrp_array = Vec::new();
        let mut raiseregrrp_array = Vec::new();
        let mut lower6secrrp_array = Vec::new();
        let mut lower60secrrp_array = Vec::new();
        let mut lower5minrrp_array = Vec::new();
        let mut lowerregrrp_array = Vec::new();
        for (_, row) in partition {
            predispatchseqno_array.push(row.predispatchseqno.start().timestamp_millis());
            runno_array.push({
                row.runno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            regionid_array.push(row.regionid);
            periodid_array.push(row.periodid);
            intervention_array.push({
                row.intervention.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
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
            rrp1_array.push({
                row.rrp1.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            eep1_array.push({
                row.eep1.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp2_array.push({
                row.rrp2.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            eep2_array.push({
                row.eep2.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp3_array.push({
                row.rrp3.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            eep3_array.push({
                row.eep3.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp4_array.push({
                row.rrp4.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            eep4_array.push({
                row.eep4.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp5_array.push({
                row.rrp5.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            eep5_array.push({
                row.eep5.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp6_array.push({
                row.rrp6.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            eep6_array.push({
                row.eep6.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp7_array.push({
                row.rrp7.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            eep7_array.push({
                row.eep7.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp8_array.push({
                row.rrp8.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            eep8_array.push({
                row.eep8.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            datetime_array.push(row.datetime.timestamp_millis());
            raise6secrrp_array.push({
                row.raise6secrrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise60secrrp_array.push({
                row.raise60secrrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raise5minrrp_array.push({
                row.raise5minrrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            raiseregrrp_array.push({
                row.raiseregrrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower6secrrp_array.push({
                row.lower6secrrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower60secrrp_array.push({
                row.lower60secrrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower5minrrp_array.push({
                row.lower5minrrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lowerregrrp_array.push({
                row.lowerregrrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(predispatchseqno_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(periodid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(eep_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp1_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(eep1_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp2_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(eep2_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp3_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(eep3_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp4_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(eep4_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp5_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(eep5_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp6_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(eep6_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp7_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(eep7_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp8_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(eep8_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PREDISPATCHPRICESENSITIVITIES
///  _PREDISPATCHPRICESENSITIVITIES sets out the sensitivity prices for each region by period._
///
/// * Data Set Name: Predispatch
/// * File Name: Pricesensitivities
/// * Data Version: 1
///
/// # Description
///  Source The plan is to provide this data every half-hour.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DATETIME
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchPricesensitivities1 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: crate::TradingPeriod,
    /// LP Solver Predispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Unique region identifier
    pub regionid: String,
    /// PERIODID is just a period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period.
    pub periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 1
    pub rrpeep1: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 2
    pub rrpeep2: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 3
    pub rrpeep3: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 4
    pub rrpeep4: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 5
    pub rrpeep5: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 6
    pub rrpeep6: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 7
    pub rrpeep7: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 8
    pub rrpeep8: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 9
    pub rrpeep9: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 10
    pub rrpeep10: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 11
    pub rrpeep11: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 12
    pub rrpeep12: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 13
    pub rrpeep13: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 14
    pub rrpeep14: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 15
    pub rrpeep15: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 16
    pub rrpeep16: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 17
    pub rrpeep17: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 18
    pub rrpeep18: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 19
    pub rrpeep19: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 20
    pub rrpeep20: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 21
    pub rrpeep21: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 22
    pub rrpeep22: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 23
    pub rrpeep23: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 24
    pub rrpeep24: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 25
    pub rrpeep25: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 26
    pub rrpeep26: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 27
    pub rrpeep27: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 28
    pub rrpeep28: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Period date and time
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
    /// Regional Energy Price for scenario 29
    pub rrpeep29: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 30
    pub rrpeep30: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 31
    pub rrpeep31: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 32
    pub rrpeep32: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 33
    pub rrpeep33: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 34
    pub rrpeep34: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 35
    pub rrpeep35: Option<rust_decimal::Decimal>,
    /// Flag to indicate if the sensitivity run contains an active intervention constraint: 0 = No, 1 = Yes
    pub intervention_active: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 36
    pub rrpeep36: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 37
    pub rrpeep37: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 38
    pub rrpeep38: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 39
    pub rrpeep39: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 40
    pub rrpeep40: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 41
    pub rrpeep41: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 42
    pub rrpeep42: Option<rust_decimal::Decimal>,
    /// Regional Energy Price for scenario 43
    pub rrpeep43: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for PredispatchPricesensitivities1 {
    type PrimaryKey = PredispatchPricesensitivities1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: Some("PRICESENSITIVITIES".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> PredispatchPricesensitivities1PrimaryKey {
        PredispatchPricesensitivities1PrimaryKey {
            datetime: self.datetime,
            regionid: self.regionid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "predispatch_pricesensitivities_v1".to_string()
    }
}
impl crate::CompareWithRow for PredispatchPricesensitivities1 {
    type Row = PredispatchPricesensitivities1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.datetime == row.datetime && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for PredispatchPricesensitivities1 {
    type PrimaryKey = PredispatchPricesensitivities1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.regionid == key.regionid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PredispatchPricesensitivities1PrimaryKey {
    pub datetime: chrono::NaiveDateTime,
    pub regionid: String,
}
impl crate::CompareWithRow for PredispatchPricesensitivities1PrimaryKey {
    type Row = PredispatchPricesensitivities1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.datetime == row.datetime && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for PredispatchPricesensitivities1PrimaryKey {
    type PrimaryKey = PredispatchPricesensitivities1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.regionid == key.regionid
    }
}
impl crate::PrimaryKey for PredispatchPricesensitivities1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PredispatchPricesensitivities1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "predispatchseqno",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep1",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep2",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep3",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep4",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep5",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep6",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep7",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep8",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep9",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep10",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep11",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep12",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep13",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep14",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep15",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep16",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep17",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep18",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep19",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep20",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep21",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep22",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep23",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep24",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep25",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep26",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep27",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep28",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("datetime", arrow2::datatypes::DataType::Date64, false),
            arrow2::datatypes::Field::new(
                "rrpeep29",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep30",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep31",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep32",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep33",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep34",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep35",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "intervention_active",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep36",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep37",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep38",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep39",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep40",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep41",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep42",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrpeep43",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut predispatchseqno_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut rrpeep1_array = Vec::new();
        let mut rrpeep2_array = Vec::new();
        let mut rrpeep3_array = Vec::new();
        let mut rrpeep4_array = Vec::new();
        let mut rrpeep5_array = Vec::new();
        let mut rrpeep6_array = Vec::new();
        let mut rrpeep7_array = Vec::new();
        let mut rrpeep8_array = Vec::new();
        let mut rrpeep9_array = Vec::new();
        let mut rrpeep10_array = Vec::new();
        let mut rrpeep11_array = Vec::new();
        let mut rrpeep12_array = Vec::new();
        let mut rrpeep13_array = Vec::new();
        let mut rrpeep14_array = Vec::new();
        let mut rrpeep15_array = Vec::new();
        let mut rrpeep16_array = Vec::new();
        let mut rrpeep17_array = Vec::new();
        let mut rrpeep18_array = Vec::new();
        let mut rrpeep19_array = Vec::new();
        let mut rrpeep20_array = Vec::new();
        let mut rrpeep21_array = Vec::new();
        let mut rrpeep22_array = Vec::new();
        let mut rrpeep23_array = Vec::new();
        let mut rrpeep24_array = Vec::new();
        let mut rrpeep25_array = Vec::new();
        let mut rrpeep26_array = Vec::new();
        let mut rrpeep27_array = Vec::new();
        let mut rrpeep28_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut datetime_array = Vec::new();
        let mut rrpeep29_array = Vec::new();
        let mut rrpeep30_array = Vec::new();
        let mut rrpeep31_array = Vec::new();
        let mut rrpeep32_array = Vec::new();
        let mut rrpeep33_array = Vec::new();
        let mut rrpeep34_array = Vec::new();
        let mut rrpeep35_array = Vec::new();
        let mut intervention_active_array = Vec::new();
        let mut rrpeep36_array = Vec::new();
        let mut rrpeep37_array = Vec::new();
        let mut rrpeep38_array = Vec::new();
        let mut rrpeep39_array = Vec::new();
        let mut rrpeep40_array = Vec::new();
        let mut rrpeep41_array = Vec::new();
        let mut rrpeep42_array = Vec::new();
        let mut rrpeep43_array = Vec::new();
        for (_, row) in partition {
            predispatchseqno_array.push(row.predispatchseqno.start().timestamp_millis());
            runno_array.push({
                row.runno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            regionid_array.push(row.regionid);
            periodid_array.push(row.periodid);
            intervention_array.push({
                row.intervention.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            rrpeep1_array.push({
                row.rrpeep1.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep2_array.push({
                row.rrpeep2.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep3_array.push({
                row.rrpeep3.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep4_array.push({
                row.rrpeep4.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep5_array.push({
                row.rrpeep5.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep6_array.push({
                row.rrpeep6.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep7_array.push({
                row.rrpeep7.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep8_array.push({
                row.rrpeep8.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep9_array.push({
                row.rrpeep9.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep10_array.push({
                row.rrpeep10.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep11_array.push({
                row.rrpeep11.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep12_array.push({
                row.rrpeep12.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep13_array.push({
                row.rrpeep13.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep14_array.push({
                row.rrpeep14.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep15_array.push({
                row.rrpeep15.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep16_array.push({
                row.rrpeep16.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep17_array.push({
                row.rrpeep17.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep18_array.push({
                row.rrpeep18.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep19_array.push({
                row.rrpeep19.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep20_array.push({
                row.rrpeep20.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep21_array.push({
                row.rrpeep21.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep22_array.push({
                row.rrpeep22.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep23_array.push({
                row.rrpeep23.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep24_array.push({
                row.rrpeep24.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep25_array.push({
                row.rrpeep25.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep26_array.push({
                row.rrpeep26.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep27_array.push({
                row.rrpeep27.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep28_array.push({
                row.rrpeep28.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            datetime_array.push(row.datetime.timestamp_millis());
            rrpeep29_array.push({
                row.rrpeep29.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep30_array.push({
                row.rrpeep30.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep31_array.push({
                row.rrpeep31.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep32_array.push({
                row.rrpeep32.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep33_array.push({
                row.rrpeep33.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep34_array.push({
                row.rrpeep34.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep35_array.push({
                row.rrpeep35.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            intervention_active_array.push({
                row.intervention_active.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            rrpeep36_array.push({
                row.rrpeep36.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep37_array.push({
                row.rrpeep37.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep38_array.push({
                row.rrpeep38.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep39_array.push({
                row.rrpeep39.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep40_array.push({
                row.rrpeep40.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep41_array.push({
                row.rrpeep41.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep42_array.push({
                row.rrpeep42.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrpeep43_array.push({
                row.rrpeep43.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(predispatchseqno_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(periodid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep1_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep2_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep3_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep4_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep5_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep6_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep7_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep8_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep9_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep10_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep11_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep12_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep13_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep14_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep15_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep16_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep17_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep18_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep19_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep20_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep21_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep22_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep23_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep24_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep25_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep26_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep27_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep28_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep29_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep30_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep31_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep32_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep33_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep34_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep35_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_active_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep36_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep37_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep38_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep39_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep40_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep41_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep42_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrpeep43_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PREDISPATCHREGIONSUM
///  _PREDISPATCHREGIONSUM sets out the overall regional Pre-Dispatch results for base case details (excluding price). _
///
/// * Data Set Name: Predispatch
/// * File Name: Region Solution
/// * Data Version: 6
///
/// # Description
///  PREDISPATCHREGIONSUM includes the forecast demand (total demand) and Frequency Control Ancillary Services (FCAS) requirements (specifically, for the Raise Regulation and Lower Regulation Ancillary Services plus improvements to demand calculations). PREDISPATCHREGIONSUM updates each half-hour with the latest Pre-Dispatch details for the remaining period. Regional demand can be calculated as total demand plus dispatchable load (i.e. Regional demand = Total Demand + Dispatchable Load) Source PREDISPATCHREGIONSUM updates every thirty minutes. Note *** "Actual FCAS availability" is determined in a post-processing step based on the energy target (TotalCleared) and bid FCAS trapezium for that interval. However, if the unit is outside the bid FCAS trapezium at the start of the interval (InitialMW), the "Actual FCAS availability" is set to zero. For regulation services, the trapezium is the most restrictive of the bid/SCADA trapezium values. From 16 February 2006, the old reserve values are no longer populated (i.e. are null), being LORSurplus and LRCSurplus. For more details on the changes to Reporting of Reserve Condition Data, refer to AEMO Communication 2042. For the best available indicator of reserve condition in each of the regions of the NEM for each trading interval, refer to the latest run of the Pre-Dispatch PASA (see table PDPASA_REGIONSOLUTION).
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DATETIME
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchRegionSolution6 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: crate::TradingPeriod,
    /// LP Solver Pre-Dispatch run no, typically 1. It increments if the case is re-run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Unique region identifier
    pub regionid: String,
    /// PERIODID is just a period count, starting from 1 for each Pre-Dispatch run. Use DATETIME to determine half hour period.
    pub periodid: Option<String>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event that there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
    /// Total demand in MW for period (less normally on loads)
    pub totaldemand: Option<rust_decimal::Decimal>,
    /// Aggregate generation bid available in region
    pub availablegeneration: Option<rust_decimal::Decimal>,
    /// Aggregate load bid available in region
    pub availableload: Option<rust_decimal::Decimal>,
    /// Delta MW value only
    pub demandforecast: Option<rust_decimal::Decimal>,
    /// Generation dispatched in period
    pub dispatchablegeneration: Option<rust_decimal::Decimal>,
    /// Load dispatched in period
    pub dispatchableload: Option<rust_decimal::Decimal>,
    /// Net interconnector flow from the regional reference node
    pub netinterchange: Option<rust_decimal::Decimal>,
    /// Excess generation in period / Deficit generation if VOLL
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
    /// Not used since Dec 2003.  Raise 5 min MW imported
    pub raise5minimport: Option<rust_decimal::Decimal>,
    /// Raise 5 min local dispatch
    pub raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Local price of raise 5 min
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
    /// Period date and time
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Period expressed as Date/Time
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
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
    /// generation availability taking into account daily energy constraints
    pub decavailability: Option<rust_decimal::Decimal>,
    /// Not used after Feb 2006. Total short term generation capacity reserve used in assessing lack of reserve condition
    pub lorsurplus: Option<rust_decimal::Decimal>,
    /// Not used after Feb 2006. Total short term generation capacity reserve above the stated low reserve condition requirement
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
impl crate::GetTable for PredispatchRegionSolution6 {
    type PrimaryKey = PredispatchRegionSolution6PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: Some("REGION_SOLUTION".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> PredispatchRegionSolution6PrimaryKey {
        PredispatchRegionSolution6PrimaryKey {
            datetime: self.datetime,
            regionid: self.regionid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "predispatch_region_solution_v6".to_string()
    }
}
impl crate::CompareWithRow for PredispatchRegionSolution6 {
    type Row = PredispatchRegionSolution6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.datetime == row.datetime && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for PredispatchRegionSolution6 {
    type PrimaryKey = PredispatchRegionSolution6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.regionid == key.regionid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PredispatchRegionSolution6PrimaryKey {
    pub datetime: chrono::NaiveDateTime,
    pub regionid: String,
}
impl crate::CompareWithRow for PredispatchRegionSolution6PrimaryKey {
    type Row = PredispatchRegionSolution6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.datetime == row.datetime && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for PredispatchRegionSolution6PrimaryKey {
    type PrimaryKey = PredispatchRegionSolution6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.regionid == key.regionid
    }
}
impl crate::PrimaryKey for PredispatchRegionSolution6PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PredispatchRegionSolution6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "predispatchseqno",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
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
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("datetime", arrow2::datatypes::DataType::Date64, false),
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
                "decavailability",
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

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut predispatchseqno_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut periodid_array = Vec::new();
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
        let mut lastchanged_array = Vec::new();
        let mut datetime_array = Vec::new();
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
        let mut decavailability_array = Vec::new();
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
            predispatchseqno_array.push(row.predispatchseqno.start().timestamp_millis());
            runno_array.push({
                row.runno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            regionid_array.push(row.regionid);
            periodid_array.push(row.periodid);
            intervention_array.push({
                row.intervention.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
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
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            datetime_array.push(row.datetime.timestamp_millis());
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
            decavailability_array.push({
                row.decavailability.map(|mut val| {
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

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(predispatchseqno_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(periodid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totaldemand_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(availablegeneration_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(availableload_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(demandforecast_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(dispatchablegeneration_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(dispatchableload_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(netinterchange_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(excessgeneration_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5mindispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minimport_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minlocaldispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minlocalprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minlocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minsupplyprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secdispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secimport_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60seclocaldispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60seclocalprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60seclocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secsupplyprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secdispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secimport_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6seclocaldispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6seclocalprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6seclocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secsupplyprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5mindispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minimport_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minlocaldispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minlocalprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minlocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minsupplyprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secdispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secimport_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60seclocaldispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60seclocalprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60seclocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secsupplyprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secdispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secimport_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6seclocaldispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6seclocalprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6seclocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secsupplyprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(initialsupply_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(clearedsupply_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregimport_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerreglocaldispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerreglocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregimport_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raisereglocaldispatch_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raisereglocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minlocalviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raisereglocalviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60seclocalviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6seclocalviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minlocalviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerreglocalviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60seclocalviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6seclocalviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregactualavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(decavailability_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lorsurplus_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lrcsurplus_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalintermittentgeneration_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(demand_and_nonschedgen_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(uigf_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(semischedule_clearedmw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(semischedule_compliancemw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ss_solar_uigf_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ss_wind_uigf_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ss_solar_clearedmw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ss_wind_clearedmw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ss_solar_compliancemw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ss_wind_compliancemw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(wdr_initialmw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(wdr_available_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(wdr_dispatched_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PREDISPATCHSCENARIODEMAND
///  _PREDISPATCHSCENARIODEMAND defines the demand offsets that are applied for each of the predispatch sensitivity scenarios._
///
/// * Data Set Name: Predispatch
/// * File Name: Scenario Demand
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * REGIONID
/// * SCENARIO
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchScenarioDemand1 {
    /// The effective date of this set of scenarios
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// The version of this set of scenarios
    pub versionno: i64,
    /// The scenario identifier.
    pub scenario: i64,
    /// The region to which to apply the deltaMW for this SCENARIO.
    pub regionid: String,
    /// The MW offset that is applied for this scenario
    pub deltamw: Option<i64>,
}
impl crate::GetTable for PredispatchScenarioDemand1 {
    type PrimaryKey = PredispatchScenarioDemand1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: Some("SCENARIO_DEMAND".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> PredispatchScenarioDemand1PrimaryKey {
        PredispatchScenarioDemand1PrimaryKey {
            effectivedate: self.effectivedate,
            regionid: self.regionid.clone(),
            scenario: self.scenario,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "predispatch_scenario_demand_v1".to_string()
    }
}
impl crate::CompareWithRow for PredispatchScenarioDemand1 {
    type Row = PredispatchScenarioDemand1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.regionid == row.regionid
            && self.scenario == row.scenario
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for PredispatchScenarioDemand1 {
    type PrimaryKey = PredispatchScenarioDemand1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.regionid == key.regionid
            && self.scenario == key.scenario
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PredispatchScenarioDemand1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub regionid: String,
    pub scenario: i64,
    pub versionno: i64,
}
impl crate::CompareWithRow for PredispatchScenarioDemand1PrimaryKey {
    type Row = PredispatchScenarioDemand1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.regionid == row.regionid
            && self.scenario == row.scenario
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for PredispatchScenarioDemand1PrimaryKey {
    type PrimaryKey = PredispatchScenarioDemand1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.regionid == key.regionid
            && self.scenario == key.scenario
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for PredispatchScenarioDemand1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PredispatchScenarioDemand1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("scenario", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("deltamw", arrow2::datatypes::DataType::Int64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut scenario_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut deltamw_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            versionno_array.push(row.versionno);
            scenario_array.push(row.scenario);
            regionid_array.push(row.regionid);
            deltamw_array.push(row.deltamw);
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(scenario_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(deltamw_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PREDISPATCHSCENARIODEMANDTRK
///  _Tracks the predispatch scenario offset updates across time_
///
/// * Data Set Name: Predispatch
/// * File Name: Scenario Demand Trk
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchScenarioDemandTrk1 {
    /// The effective date of this set of scenarios
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// The version of this set of scenarios
    pub versionno: i64,
    /// The user that authorised the scenario update
    pub authorisedby: Option<String>,
    /// The datetime that the scenario update was authorised
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// The datetime that the record was last changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for PredispatchScenarioDemandTrk1 {
    type PrimaryKey = PredispatchScenarioDemandTrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: Some("SCENARIO_DEMAND_TRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> PredispatchScenarioDemandTrk1PrimaryKey {
        PredispatchScenarioDemandTrk1PrimaryKey {
            effectivedate: self.effectivedate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "predispatch_scenario_demand_trk_v1".to_string()
    }
}
impl crate::CompareWithRow for PredispatchScenarioDemandTrk1 {
    type Row = PredispatchScenarioDemandTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for PredispatchScenarioDemandTrk1 {
    type PrimaryKey = PredispatchScenarioDemandTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PredispatchScenarioDemandTrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: i64,
}
impl crate::CompareWithRow for PredispatchScenarioDemandTrk1PrimaryKey {
    type Row = PredispatchScenarioDemandTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for PredispatchScenarioDemandTrk1PrimaryKey {
    type PrimaryKey = PredispatchScenarioDemandTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for PredispatchScenarioDemandTrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PredispatchScenarioDemandTrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            versionno_array.push(row.versionno);
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp_millis()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PREDISPATCH_FCAS_REQ
///  _PREDISPATCH_FCAS_REQ shows Predispatch Constraint tracking for Regional FCAS Requirements._
///
/// * Data Set Name: Predispatch
/// * File Name: Regionfcasrequirement
/// * Data Version: 2
///
/// # Description
///  Source PREDISPATCH_FCAS_REQ updates with each pre-dispatch run (half hourly) Volume Approximately 2,000 rows per day. Note The PERIODID columns in tables PREDISPATCHCONSTRAINT and PREDISPATCH_FCAS_REQ have no consistent relationship with the other PERIODID values in the other tables in the PRE-DISPATCH package (such as PREDISPATCHPRICE). AEMO and many Participants appreciate the data model is inconsistent, but the cost of changing existing systems has been judged as being unjustifiable. An additional field DATETIME was added to allow joins between these data sets.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * DATETIME
/// * GENCONID
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchRegionfcasrequirement2 {
    /// PreDispatch Sequence number
    pub predispatchseqno: Option<String>,
    /// Case Run number
    pub runno: Option<rust_decimal::Decimal>,
    /// Intervention Flag
    pub intervention: Option<rust_decimal::Decimal>,
    /// Unique period identifier, in the format yyyymmddpp. The period (pp) is 01 to 48, with 01 corresponding to the half-hour ending at 04:30am.
    pub periodid: Option<String>,
    /// Generic Constraint ID - Join to table GenConData
    pub genconid: String,
    /// Region ID
    pub regionid: String,
    /// Bid Type Identifier
    pub bidtype: String,
    /// Generic Constraint EffectiveDate - Join to table GenConData
    #[serde(with = "crate::mms_datetime_opt")]
    pub genconeffectivedate: Option<chrono::NaiveDateTime>,
    /// Generic Constraint Version number - Join to table GenConData
    pub genconversionno: Option<rust_decimal::Decimal>,
    /// Marginal Value of generic constraint
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Date and Time of trading interval
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
    /// Last date and time record changed
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
impl crate::GetTable for PredispatchRegionfcasrequirement2 {
    type PrimaryKey = PredispatchRegionfcasrequirement2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: Some("REGIONFCASREQUIREMENT".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> PredispatchRegionfcasrequirement2PrimaryKey {
        PredispatchRegionfcasrequirement2PrimaryKey {
            bidtype: self.bidtype.clone(),
            datetime: self.datetime,
            genconid: self.genconid.clone(),
            regionid: self.regionid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "predispatch_regionfcasrequirement_v2".to_string()
    }
}
impl crate::CompareWithRow for PredispatchRegionfcasrequirement2 {
    type Row = PredispatchRegionfcasrequirement2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.datetime == row.datetime
            && self.genconid == row.genconid
            && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for PredispatchRegionfcasrequirement2 {
    type PrimaryKey = PredispatchRegionfcasrequirement2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.datetime == key.datetime
            && self.genconid == key.genconid
            && self.regionid == key.regionid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PredispatchRegionfcasrequirement2PrimaryKey {
    pub bidtype: String,
    pub datetime: chrono::NaiveDateTime,
    pub genconid: String,
    pub regionid: String,
}
impl crate::CompareWithRow for PredispatchRegionfcasrequirement2PrimaryKey {
    type Row = PredispatchRegionfcasrequirement2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.datetime == row.datetime
            && self.genconid == row.genconid
            && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for PredispatchRegionfcasrequirement2PrimaryKey {
    type PrimaryKey = PredispatchRegionfcasrequirement2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.datetime == key.datetime
            && self.genconid == key.genconid
            && self.regionid == key.regionid
    }
}
impl crate::PrimaryKey for PredispatchRegionfcasrequirement2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PredispatchRegionfcasrequirement2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "predispatchseqno",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::LargeUtf8, true),
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
            arrow2::datatypes::Field::new("datetime", arrow2::datatypes::DataType::Date64, false),
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

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut predispatchseqno_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut genconid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut bidtype_array = Vec::new();
        let mut genconeffectivedate_array = Vec::new();
        let mut genconversionno_array = Vec::new();
        let mut marginalvalue_array = Vec::new();
        let mut datetime_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut base_cost_array = Vec::new();
        let mut adjusted_cost_array = Vec::new();
        let mut estimated_cmpf_array = Vec::new();
        let mut estimated_crmpf_array = Vec::new();
        let mut recovery_factor_cmpf_array = Vec::new();
        let mut recovery_factor_crmpf_array = Vec::new();
        for (_, row) in partition {
            predispatchseqno_array.push(row.predispatchseqno);
            runno_array.push({
                row.runno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            intervention_array.push({
                row.intervention.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            periodid_array.push(row.periodid);
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
            datetime_array.push(row.datetime.timestamp_millis());
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

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    predispatchseqno_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(periodid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(genconid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bidtype_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(genconeffectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(genconversionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marginalvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(base_cost_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(adjusted_cost_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(estimated_cmpf_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(estimated_crmpf_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(recovery_factor_cmpf_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(recovery_factor_crmpf_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PREDISPATCH_LOCAL_PRICE
///  _Sets out local pricing offsets associated with each DUID connection point for each dispatch period_
///
/// * Data Set Name: Predispatch
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
/// * DATETIME
/// * DUID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchLocalPrice1 {
    /// Unique identifier of predispatch run in the form YYYYMMDDPP with 01 at 04:30
    pub predispatchseqno: crate::TradingPeriod,
    /// The unique identifier for the interval within this study
    #[serde(with = "crate::mms_datetime")]
    pub datetime: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: String,
    /// A period count, starting from 1 for each predispatch run. Use DATETIME to determine half hour period
    pub periodid: Option<String>,
    /// Aggregate Constraint contribution cost of this unit: Sum(MarginalValue x Factor) for all relevant Constraints
    pub local_price_adjustment: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for PredispatchLocalPrice1 {
    type PrimaryKey = PredispatchLocalPrice1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: Some("LOCAL_PRICE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> PredispatchLocalPrice1PrimaryKey {
        PredispatchLocalPrice1PrimaryKey {
            datetime: self.datetime,
            duid: self.duid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "predispatch_local_price_v1".to_string()
    }
}
impl crate::CompareWithRow for PredispatchLocalPrice1 {
    type Row = PredispatchLocalPrice1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.datetime == row.datetime && self.duid == row.duid
    }
}
impl crate::CompareWithPrimaryKey for PredispatchLocalPrice1 {
    type PrimaryKey = PredispatchLocalPrice1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.duid == key.duid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PredispatchLocalPrice1PrimaryKey {
    pub datetime: chrono::NaiveDateTime,
    pub duid: String,
}
impl crate::CompareWithRow for PredispatchLocalPrice1PrimaryKey {
    type Row = PredispatchLocalPrice1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.datetime == row.datetime && self.duid == row.duid
    }
}
impl crate::CompareWithPrimaryKey for PredispatchLocalPrice1PrimaryKey {
    type PrimaryKey = PredispatchLocalPrice1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.datetime == key.datetime && self.duid == key.duid
    }
}
impl crate::PrimaryKey for PredispatchLocalPrice1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PredispatchLocalPrice1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "predispatchseqno",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new("datetime", arrow2::datatypes::DataType::Date64, false),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::LargeUtf8, true),
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
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut predispatchseqno_array = Vec::new();
        let mut datetime_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut local_price_adjustment_array = Vec::new();
        let mut locally_constrained_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            predispatchseqno_array.push(row.predispatchseqno.start().timestamp_millis());
            datetime_array.push(row.datetime.timestamp_millis());
            duid_array.push(row.duid);
            periodid_array.push(row.periodid);
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
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(predispatchseqno_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(periodid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(local_price_adjustment_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(locally_constrained_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PREDISPATCH_MNSPBIDTRK
///  _PREDISPATCH_MNSPBIDTRK shows the MNSP bid tracking, including the bid version used in each predispatch run for each MNSP Interconnector Link. PREDISPATCH_MNSPBIDTRK shows the audit trail of the bid used for each predispatch run._
///
/// * Data Set Name: Predispatch
/// * File Name: Mnspbidtrk
/// * Data Version: 1
///
/// # Description
///  Source Own (confidential) data updates every predispatch run. All bids are available to all participants as part of next day market data. Volume 1, 700, 000 per year
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * LINKID
/// * PERIODID
/// * PREDISPATCHSEQNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PredispatchMnspbidtrk1 {
    /// Predispatch run identifier
    pub predispatchseqno: String,
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    pub linkid: String,
    /// Trading Interval number
    pub periodid: String,
    /// Participant Identifier
    pub participantid: Option<String>,
    /// Market Date from which bid is active
    #[serde(with = "crate::mms_datetime_opt")]
    pub settlementdate: Option<chrono::NaiveDateTime>,
    /// Time this bid was processed and loaded
    #[serde(with = "crate::mms_datetime_opt")]
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// Version No. for given offer date and settlement date used
    pub versionno: Option<rust_decimal::Decimal>,
    /// Period expressed as Date/Time
    #[serde(with = "crate::mms_datetime_opt")]
    pub datetime: Option<chrono::NaiveDateTime>,
    /// Record creation timestamp
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for PredispatchMnspbidtrk1 {
    type PrimaryKey = PredispatchMnspbidtrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PREDISPATCH".into(),
            table_name: Some("MNSPBIDTRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> PredispatchMnspbidtrk1PrimaryKey {
        PredispatchMnspbidtrk1PrimaryKey {
            linkid: self.linkid.clone(),
            periodid: self.periodid.clone(),
            predispatchseqno: self.predispatchseqno.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "predispatch_mnspbidtrk_v1".to_string()
    }
}
impl crate::CompareWithRow for PredispatchMnspbidtrk1 {
    type Row = PredispatchMnspbidtrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.linkid == row.linkid
            && self.periodid == row.periodid
            && self.predispatchseqno == row.predispatchseqno
    }
}
impl crate::CompareWithPrimaryKey for PredispatchMnspbidtrk1 {
    type PrimaryKey = PredispatchMnspbidtrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid == key.linkid
            && self.periodid == key.periodid
            && self.predispatchseqno == key.predispatchseqno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PredispatchMnspbidtrk1PrimaryKey {
    pub linkid: String,
    pub periodid: String,
    pub predispatchseqno: String,
}
impl crate::CompareWithRow for PredispatchMnspbidtrk1PrimaryKey {
    type Row = PredispatchMnspbidtrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.linkid == row.linkid
            && self.periodid == row.periodid
            && self.predispatchseqno == row.predispatchseqno
    }
}
impl crate::CompareWithPrimaryKey for PredispatchMnspbidtrk1PrimaryKey {
    type PrimaryKey = PredispatchMnspbidtrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.linkid == key.linkid
            && self.periodid == key.periodid
            && self.predispatchseqno == key.predispatchseqno
    }
}
impl crate::PrimaryKey for PredispatchMnspbidtrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for PredispatchMnspbidtrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "predispatchseqno",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("linkid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new("offerdate", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new("datetime", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut predispatchseqno_array = Vec::new();
        let mut linkid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut settlementdate_array = Vec::new();
        let mut offerdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut datetime_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            predispatchseqno_array.push(row.predispatchseqno);
            linkid_array.push(row.linkid);
            periodid_array.push(row.periodid);
            participantid_array.push(row.participantid);
            settlementdate_array.push(row.settlementdate.map(|val| val.timestamp_millis()));
            offerdate_array.push(row.offerdate.map(|val| val.timestamp_millis()));
            versionno_array.push({
                row.versionno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            datetime_array.push(row.datetime.map(|val| val.timestamp_millis()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    predispatchseqno_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(linkid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(periodid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(participantid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(offerdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
