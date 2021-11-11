/// # Summary
///
/// ## P5MIN_BLOCKEDCONSTRAINT
///  _P5MIN Blocked Constraints lists any constraints that were blocked in a P5MIN run. If no constraints are blocked, there will be no rows for that 5 minute predispatch run._
///
/// * Data Set Name: P5min
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
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minBlockedConstraints1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Generic Constraint identifier (synonymous with GenConID)
    pub constraintid: String,
}
impl crate::GetTable for P5minBlockedConstraints1 {
    type PrimaryKey = P5minBlockedConstraints1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: Some("BLOCKED_CONSTRAINTS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> P5minBlockedConstraints1PrimaryKey {
        P5minBlockedConstraints1PrimaryKey {
            constraintid: self.constraintid.clone(),
            run_datetime: self.run_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "p5min_blocked_constraints_v1".to_string()
    }
}
impl crate::CompareWithRow for P5minBlockedConstraints1 {
    type Row = P5minBlockedConstraints1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minBlockedConstraints1 {
    type PrimaryKey = P5minBlockedConstraints1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.run_datetime == key.run_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minBlockedConstraints1PrimaryKey {
    pub constraintid: String,
    pub run_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for P5minBlockedConstraints1PrimaryKey {
    type Row = P5minBlockedConstraints1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minBlockedConstraints1PrimaryKey {
    type PrimaryKey = P5minBlockedConstraints1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.run_datetime == key.run_datetime
    }
}
impl crate::PrimaryKey for P5minBlockedConstraints1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for P5minBlockedConstraints1 {
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
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut run_datetime_array = Vec::new();
        let mut constraintid_array = Vec::new();
        for (_, row) in partition {
            run_datetime_array.push(
                i32::try_from(
                    (row.run_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            constraintid_array.push(row.constraintid);
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
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## P5MIN_CASESOLUTION
///  _The five-minute predispatch (P5Min) is a MMS system providing projected dispatch for 12 Dispatch cycles (one hour). The 5-minute Predispatch cycle runs every 5-minutes to produce a dispatch and pricing schedule to a 5-minute resolution covering the next hour, a total of twelve periods.<br>P5MIN_CASESOLUTION shows one record containing results pertaining to the entire solution.<br>_
///
/// * Data Set Name: P5min
/// * File Name: Casesolution
/// * Data Version: 2
///
/// # Description
///  P5MIN_CASESOLUTION data is public, so is available to all participants. Source P5MIN_CASESOLUTION updates every 5 minutes. Volume Rows per day: 288
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minCasesolution2 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Date and Time of first interval in study
    pub startinterval_datetime: Option<String>,
    /// The Objective function from the LP
    pub totalobjective: Option<rust_decimal::Decimal>,
    /// Flag to indicate non-physical losses occurred in this study
    pub nonphysicallosses: Option<rust_decimal::Decimal>,
    /// Sum of Regional Energy balance violations
    pub totalareagenviolation: Option<rust_decimal::Decimal>,
    /// Sum of Interconnector violations of standing data limits
    pub totalinterconnectorviolation: Option<rust_decimal::Decimal>,
    /// Sum of Generic Constraint violations
    pub totalgenericviolation: Option<rust_decimal::Decimal>,
    /// Sum of Unit Ramp Rate violations
    pub totalramprateviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit capacity violations
    pub totalunitmwcapacityviolation: Option<rust_decimal::Decimal>,
    /// Sum of regional 5 min FCAS violations
    pub total5minviolation: Option<rust_decimal::Decimal>,
    /// Sum of regional regulation FCAS violations
    pub totalregviolation: Option<rust_decimal::Decimal>,
    /// Sum of regional 6 sec FCAS violations
    pub total6secviolation: Option<rust_decimal::Decimal>,
    /// Sum of regional 60 sec FCAS violations
    pub total60secviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit energy constrained violations
    pub totalenergyconstrviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit offer violations
    pub totalenergyofferviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit FCAS profile offer violations
    pub totalasprofileviolation: Option<rust_decimal::Decimal>,
    /// Sum of unit Fast start profile violations
    pub totalfaststartviolation: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag to indicate if this Predispatch case includes an intervention pricing run: 0 = case does not include an intervention pricing run, 1 = case does include an intervention pricing run. This field has a default value of 0 and is not nullable
    pub intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for P5minCasesolution2 {
    type PrimaryKey = P5minCasesolution2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: Some("CASESOLUTION".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> P5minCasesolution2PrimaryKey {
        P5minCasesolution2PrimaryKey {
            run_datetime: self.run_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "p5min_casesolution_v2".to_string()
    }
}
impl crate::CompareWithRow for P5minCasesolution2 {
    type Row = P5minCasesolution2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minCasesolution2 {
    type PrimaryKey = P5minCasesolution2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minCasesolution2PrimaryKey {
    pub run_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for P5minCasesolution2PrimaryKey {
    type Row = P5minCasesolution2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minCasesolution2PrimaryKey {
    type PrimaryKey = P5minCasesolution2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
    }
}
impl crate::PrimaryKey for P5minCasesolution2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for P5minCasesolution2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "run_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "startinterval_datetime",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "totalobjective",
                arrow2::datatypes::DataType::Decimal(27, 10),
                true,
            ),
            arrow2::datatypes::Field::new(
                "nonphysicallosses",
                arrow2::datatypes::DataType::Decimal(1, 0),
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
                "totalenergyconstrviolation",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "totalenergyofferviolation",
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
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut run_datetime_array = Vec::new();
        let mut startinterval_datetime_array = Vec::new();
        let mut totalobjective_array = Vec::new();
        let mut nonphysicallosses_array = Vec::new();
        let mut totalareagenviolation_array = Vec::new();
        let mut totalinterconnectorviolation_array = Vec::new();
        let mut totalgenericviolation_array = Vec::new();
        let mut totalramprateviolation_array = Vec::new();
        let mut totalunitmwcapacityviolation_array = Vec::new();
        let mut total5minviolation_array = Vec::new();
        let mut totalregviolation_array = Vec::new();
        let mut total6secviolation_array = Vec::new();
        let mut total60secviolation_array = Vec::new();
        let mut totalenergyconstrviolation_array = Vec::new();
        let mut totalenergyofferviolation_array = Vec::new();
        let mut totalasprofileviolation_array = Vec::new();
        let mut totalfaststartviolation_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut intervention_array = Vec::new();
        for (_, row) in partition {
            run_datetime_array.push(
                i32::try_from(
                    (row.run_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            startinterval_datetime_array.push(row.startinterval_datetime);
            totalobjective_array.push({
                row.totalobjective.map(|mut val| {
                    val.rescale(10);
                    val.mantissa()
                })
            });
            nonphysicallosses_array.push({
                row.nonphysicallosses.map(|mut val| {
                    val.rescale(0);
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
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            intervention_array.push({
                row.intervention.map(|mut val| {
                    val.rescale(0);
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
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    startinterval_datetime_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalobjective_array)
                        .to(arrow2::datatypes::DataType::Decimal(27, 10)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(nonphysicallosses_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
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
                    arrow2::array::PrimitiveArray::from(totalenergyconstrviolation_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalenergyofferviolation_array)
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
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## P5MIN_CONSTRAINTSOLUTION
///  _The Five-Minute Pre-Dispatch (P5Min) is a MMS system providing projected dispatch for 12 Dispatch cycles (one hour). The Five-Minute Pre-dispatch cycle runs every 5-minutes to produce a dispatch and pricing schedule to a 5-minute resolution covering the next hour, a total of twelve periods.<br>P5MIN_CONSTRAINTSOLUTION shows binding and violated constraint results from the capacity evaluation, including the RHS value.<br>_
///
/// * Data Set Name: P5min
/// * File Name: Constraintsolution
/// * Data Version: 6
///
/// # Description
///  P5MIN_CONSTRAINTSOLUTION is public data, so is available to all participants. Source P5MIN_CONSTRAINTSOLUTION updates every five minutes. Volume Rows per day: ~2.3 million
///
/// # Notes
///  * (Visibility) Data in this table is: Private &amp; Public
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minConstraintsolution6 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Constraint identifier (synonymous with GenConID)
    pub constraintid: String,
    /// Right Hand Side value in the capacity evaluation
    pub rhs: Option<rust_decimal::Decimal>,
    /// Marginal cost of constraint (&gt;0 if binding)
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Amount of Violation (&gt;0 if  violating)
    pub violationdegree: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// DUID to which the Constraint is confidential. Null denotes non-confidential
    pub duid: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub genconid_effectivedate: Option<chrono::NaiveDateTime>,
    /// Version number of the Generic Constraint (ConstraintID). This field is used to track the version of this generic constraint applied in this dispatch interval
    pub genconid_versionno: Option<rust_decimal::Decimal>,
    /// Aggregation of the constraints LHS term solution values
    pub lhs: Option<rust_decimal::Decimal>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run(INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0)
    pub intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for P5minConstraintsolution6 {
    type PrimaryKey = P5minConstraintsolution6PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: Some("CONSTRAINTSOLUTION".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> P5minConstraintsolution6PrimaryKey {
        P5minConstraintsolution6PrimaryKey {
            constraintid: self.constraintid.clone(),
            interval_datetime: self.interval_datetime.clone(),
            run_datetime: self.run_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "p5min_constraintsolution_v6".to_string()
    }
}
impl crate::CompareWithRow for P5minConstraintsolution6 {
    type Row = P5minConstraintsolution6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minConstraintsolution6 {
    type PrimaryKey = P5minConstraintsolution6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minConstraintsolution6PrimaryKey {
    pub constraintid: String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for P5minConstraintsolution6PrimaryKey {
    type Row = P5minConstraintsolution6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minConstraintsolution6PrimaryKey {
    type PrimaryKey = P5minConstraintsolution6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
impl crate::PrimaryKey for P5minConstraintsolution6PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for P5minConstraintsolution6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "run_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interval_datetime",
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
            arrow2::datatypes::Field::new(
                "violationdegree",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "genconid_effectivedate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "genconid_versionno",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lhs", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut run_datetime_array = Vec::new();
        let mut interval_datetime_array = Vec::new();
        let mut constraintid_array = Vec::new();
        let mut rhs_array = Vec::new();
        let mut marginalvalue_array = Vec::new();
        let mut violationdegree_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut genconid_effectivedate_array = Vec::new();
        let mut genconid_versionno_array = Vec::new();
        let mut lhs_array = Vec::new();
        let mut intervention_array = Vec::new();
        for (_, row) in partition {
            run_datetime_array.push(
                i32::try_from(
                    (row.run_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            interval_datetime_array.push(
                i32::try_from(
                    (row.interval_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
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
            violationdegree_array.push({
                row.violationdegree.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            duid_array.push(row.duid);
            genconid_effectivedate_array.push(row.genconid_effectivedate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
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
            intervention_array.push({
                row.intervention.map(|mut val| {
                    val.rescale(0);
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
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(interval_datetime_array)
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
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(violationdegree_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(genconid_effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(genconid_versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lhs_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## P5MIN_INTERCONNECTORSOLN
///  _The five-minute predispatch (P5Min) is a MMS system providing projected dispatch for 12 Dispatch cycles (one hour). The 5-minute Predispatch cycle runs every 5-minutes to produce a dispatch and pricing schedule to a 5-minute resolution covering the next hour, a total of twelve periods.<br>P5MIN_INTERCONNECTORSOLN sets out the results of the capacity evaluation for Interconnectors, including the calculated limits for the interval.<br>_
///
/// * Data Set Name: P5min
/// * File Name: Interconnectorsoln
/// * Data Version: 4
///
/// # Description
///  P5MIN_INTERCONNECTORSOLN is public data, so is available to all participants. Source P5MIN_INTERCONNECTORSOLN updates every 5 minutes. Volume Rows per day: 1440 Based on 200 interconnector/binding constraints per interval
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERCONNECTORID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minInterconnectorsoln4 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Interconnector identifier
    pub interconnectorid: String,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// SCADA MW Flow measured at Run start. For periods subsequent to the first period of a P5MIN run, this value represents the cleared target for the previous period of that P5MIN run.
    pub meteredmwflow: Option<rust_decimal::Decimal>,
    /// Cleared Interconnector loading level (MW)
    pub mwflow: Option<rust_decimal::Decimal>,
    /// Interconnector Losses at cleared flow
    pub mwlosses: Option<rust_decimal::Decimal>,
    /// Marginal cost of Interconnector standing data limits (if binding)
    pub marginalvalue: Option<rust_decimal::Decimal>,
    /// Violation of Interconnector standing data limits
    pub violationdegree: Option<rust_decimal::Decimal>,
    /// Flag indicating MNSP registration
    pub mnsp: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of exporting energy on the basis of invoked constraints and static interconnector export limit
    pub exportlimit: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of importing energy on the basis of invoked constraints and static interconnector import limit. Note unlike the input interconnector import limit this is a directional quantity and should be defined with respect to the interconnector flow.
    pub importlimit: Option<rust_decimal::Decimal>,
    /// Marginal loss factor at the cleared flow
    pub marginalloss: Option<rust_decimal::Decimal>,
    /// Generic Constraint setting the export limit
    pub exportgenconid: Option<String>,
    /// Generic Constraint setting the import limit
    pub importgenconid: Option<String>,
    /// Calculated export limit applying to energy + Frequency Controlled Ancillary Services.
    pub fcasexportlimit: Option<rust_decimal::Decimal>,
    /// Calculated import limit applying to energy + Frequency Controlled Ancillary Services.
    pub fcasimportlimit: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Export (Factor &gt;= 0)
    pub local_price_adjustment_export: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Export: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained_export: Option<rust_decimal::Decimal>,
    /// Aggregate Constraint contribution cost of this Interconnector: Sum(MarginalValue x Factor) for all relevant Constraints, for Import (Factor &gt;= 0)
    pub local_price_adjustment_import: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment_Import: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained_import: Option<rust_decimal::Decimal>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0)
    pub intervention: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for P5minInterconnectorsoln4 {
    type PrimaryKey = P5minInterconnectorsoln4PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: Some("INTERCONNECTORSOLN".into()),
            version: 4,
        }
    }

    fn primary_key(&self) -> P5minInterconnectorsoln4PrimaryKey {
        P5minInterconnectorsoln4PrimaryKey {
            interconnectorid: self.interconnectorid.clone(),
            interval_datetime: self.interval_datetime.clone(),
            run_datetime: self.run_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "p5min_interconnectorsoln_v4".to_string()
    }
}
impl crate::CompareWithRow for P5minInterconnectorsoln4 {
    type Row = P5minInterconnectorsoln4;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interconnectorid == row.interconnectorid
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minInterconnectorsoln4 {
    type PrimaryKey = P5minInterconnectorsoln4PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minInterconnectorsoln4PrimaryKey {
    pub interconnectorid: String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for P5minInterconnectorsoln4PrimaryKey {
    type Row = P5minInterconnectorsoln4;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interconnectorid == row.interconnectorid
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minInterconnectorsoln4PrimaryKey {
    type PrimaryKey = P5minInterconnectorsoln4PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
impl crate::PrimaryKey for P5minInterconnectorsoln4PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for P5minInterconnectorsoln4 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "run_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interval_datetime",
                arrow2::datatypes::DataType::Date32,
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
            arrow2::datatypes::Field::new("mnsp", arrow2::datatypes::DataType::Decimal(1, 0), true),
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
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
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
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut run_datetime_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut interval_datetime_array = Vec::new();
        let mut meteredmwflow_array = Vec::new();
        let mut mwflow_array = Vec::new();
        let mut mwlosses_array = Vec::new();
        let mut marginalvalue_array = Vec::new();
        let mut violationdegree_array = Vec::new();
        let mut mnsp_array = Vec::new();
        let mut exportlimit_array = Vec::new();
        let mut importlimit_array = Vec::new();
        let mut marginalloss_array = Vec::new();
        let mut exportgenconid_array = Vec::new();
        let mut importgenconid_array = Vec::new();
        let mut fcasexportlimit_array = Vec::new();
        let mut fcasimportlimit_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut local_price_adjustment_export_array = Vec::new();
        let mut locally_constrained_export_array = Vec::new();
        let mut local_price_adjustment_import_array = Vec::new();
        let mut locally_constrained_import_array = Vec::new();
        let mut intervention_array = Vec::new();
        for (_, row) in partition {
            run_datetime_array.push(
                i32::try_from(
                    (row.run_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            interconnectorid_array.push(row.interconnectorid);
            interval_datetime_array.push(
                i32::try_from(
                    (row.interval_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
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
            mnsp_array.push({
                row.mnsp.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
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
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
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
            intervention_array.push({
                row.intervention.map(|mut val| {
                    val.rescale(0);
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
                    interconnectorid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(interval_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
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
                    arrow2::array::PrimitiveArray::from(mnsp_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
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
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
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
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## P5MIN_INTERSENSITIVITIES
///  _Price Sensitivies for 5MinPD solution. New solution every 5 minutes. Current Scenarios defined in P5MIN_SCENARIODEMANDTRK/P5MIN_SCENARIODEMAND_
///
/// * Data Set Name: P5min
/// * File Name: Intersensitivities
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERCONNECTORID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minIntersensitivities1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Interconnector identifier
    pub interconnectorid: String,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0)
    pub intervention: rust_decimal::Decimal,
    /// Flag to indicate if the sensitivity run contains an active intervention constraint: 0 = No, 1 = Yes
    pub intervention_active: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow1 = MW flow for given Interconnector for Scenario 1
    pub mwflow1: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow2 = MW flow for given Interconnector for Scenario 2
    pub mwflow2: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow3 = MW flow for given Interconnector for Scenario 3
    pub mwflow3: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow4 = MW flow for given Interconnector for Scenario 4
    pub mwflow4: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow5 = MW flow for given Interconnector for Scenario 5
    pub mwflow5: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow6 = MW flow for given Interconnector for Scenario 6
    pub mwflow6: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow7 = MW flow for given Interconnector for Scenario 7
    pub mwflow7: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow8 = MW flow for given Interconnector for Scenario 8
    pub mwflow8: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow9 = MW flow for given Interconnector for Scenario 9
    pub mwflow9: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow10 = MW flow for given Interconnector for Scenario 10
    pub mwflow10: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow11 = MW flow for given Interconnector for Scenario 11
    pub mwflow11: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow12 = MW flow for given Interconnector for Scenario 12
    pub mwflow12: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow13 = MW flow for given Interconnector for Scenario 13
    pub mwflow13: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow14 = MW flow for given Interconnector for Scenario 14
    pub mwflow14: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow15 = MW flow for given Interconnector for Scenario 15
    pub mwflow15: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow16 = MW flow for given Interconnector for Scenario 16
    pub mwflow16: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow17 = MW flow for given Interconnector for Scenario 17
    pub mwflow17: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow18 = MW flow for given Interconnector for Scenario 18
    pub mwflow18: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow19 = MW flow for given Interconnector for Scenario 19
    pub mwflow19: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow20 = MW flow for given Interconnector for Scenario 20
    pub mwflow20: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow21 = MW flow for given Interconnector for Scenario 21
    pub mwflow21: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow22 = MW flow for given Interconnector for Scenario 22
    pub mwflow22: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow23 = MW flow for given Interconnector for Scenario 23
    pub mwflow23: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow24 = MW flow for given Interconnector for Scenario 24
    pub mwflow24: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow25 = MW flow for given Interconnector for Scenario 25
    pub mwflow25: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow26 = MW flow for given Interconnector for Scenario 26
    pub mwflow26: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow27 = MW flow for given Interconnector for Scenario 27
    pub mwflow27: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow28 = MW flow for given Interconnector for Scenario 28
    pub mwflow28: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow29 = MW flow for given Interconnector for Scenario 29
    pub mwflow29: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow30 = MW flow for given Interconnector for Scenario 30
    pub mwflow30: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow31 = MW flow for given Interconnector for Scenario 31
    pub mwflow31: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow32 = MW flow for given Interconnector for Scenario 32
    pub mwflow32: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow33 = MW flow for given Interconnector for Scenario 33
    pub mwflow33: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow34 = MW flow for given Interconnector for Scenario 34
    pub mwflow34: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow35 = MW flow for given Interconnector for Scenario 35
    pub mwflow35: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow36 = MW flow for given Interconnector for Scenario 36
    pub mwflow36: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow37 = MW flow for given Interconnector for Scenario 37
    pub mwflow37: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow38 = MW flow for given Interconnector for Scenario 38
    pub mwflow38: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow39 = MW flow for given Interconnector for Scenario 39
    pub mwflow39: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow40 = MW flow for given Interconnector for Scenario 40
    pub mwflow40: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow41 = MW flow for given Interconnector for Scenario 41
    pub mwflow41: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow42 = MW flow for given Interconnector for Scenario 42
    pub mwflow42: Option<rust_decimal::Decimal>,
    /// MW Flow value. Flow43 = MW flow for given Interconnector for Scenario 43
    pub mwflow43: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for P5minIntersensitivities1 {
    type PrimaryKey = P5minIntersensitivities1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: Some("INTERSENSITIVITIES".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> P5minIntersensitivities1PrimaryKey {
        P5minIntersensitivities1PrimaryKey {
            interconnectorid: self.interconnectorid.clone(),
            interval_datetime: self.interval_datetime.clone(),
            run_datetime: self.run_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "p5min_intersensitivities_v1".to_string()
    }
}
impl crate::CompareWithRow for P5minIntersensitivities1 {
    type Row = P5minIntersensitivities1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interconnectorid == row.interconnectorid
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minIntersensitivities1 {
    type PrimaryKey = P5minIntersensitivities1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minIntersensitivities1PrimaryKey {
    pub interconnectorid: String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for P5minIntersensitivities1PrimaryKey {
    type Row = P5minIntersensitivities1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interconnectorid == row.interconnectorid
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minIntersensitivities1PrimaryKey {
    type PrimaryKey = P5minIntersensitivities1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
impl crate::PrimaryKey for P5minIntersensitivities1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for P5minIntersensitivities1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "run_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interval_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(1, 0),
                false,
            ),
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
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut run_datetime_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut interval_datetime_array = Vec::new();
        let mut intervention_array = Vec::new();
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
            run_datetime_array.push(
                i32::try_from(
                    (row.run_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            interconnectorid_array.push(row.interconnectorid);
            interval_datetime_array.push(
                i32::try_from(
                    (row.interval_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            intervention_array.push({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
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
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(run_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(interval_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_active_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow1_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow2_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow3_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow4_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow5_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow6_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow7_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow8_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow9_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow10_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow11_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow12_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow13_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow14_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow15_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow16_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow17_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow18_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow19_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow20_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow21_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow22_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow23_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow24_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow25_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow26_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow27_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow28_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow29_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow30_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow31_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow32_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow33_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow34_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow35_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow36_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow37_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow38_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow39_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow40_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow41_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow42_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow43_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## P5MIN_LOCAL_PRICE
///  _Sets out local pricing offsets associated with each DUID connection point for each dispatch period_
///
/// * Data Set Name: P5min
/// * File Name: Local Price
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DUID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minLocalPrice1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: String,
    /// Aggregate Constraint contribution cost of this unit: Sum(MarginalValue x Factor) for all relevant Constraints
    pub local_price_adjustment: Option<rust_decimal::Decimal>,
    /// Key for Local_Price_Adjustment: 2 = at least one Outage Constraint; 1 = at least 1 System Normal Constraint (and no Outage Constraint); 0 = No System Normal or Outage Constraints
    pub locally_constrained: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for P5minLocalPrice1 {
    type PrimaryKey = P5minLocalPrice1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: Some("LOCAL_PRICE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> P5minLocalPrice1PrimaryKey {
        P5minLocalPrice1PrimaryKey {
            duid: self.duid.clone(),
            interval_datetime: self.interval_datetime.clone(),
            run_datetime: self.run_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "p5min_local_price_v1".to_string()
    }
}
impl crate::CompareWithRow for P5minLocalPrice1 {
    type Row = P5minLocalPrice1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minLocalPrice1 {
    type PrimaryKey = P5minLocalPrice1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minLocalPrice1PrimaryKey {
    pub duid: String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for P5minLocalPrice1PrimaryKey {
    type Row = P5minLocalPrice1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minLocalPrice1PrimaryKey {
    type PrimaryKey = P5minLocalPrice1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
impl crate::PrimaryKey for P5minLocalPrice1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for P5minLocalPrice1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "run_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interval_datetime",
                arrow2::datatypes::DataType::Date32,
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
        let mut run_datetime_array = Vec::new();
        let mut interval_datetime_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut local_price_adjustment_array = Vec::new();
        let mut locally_constrained_array = Vec::new();
        for (_, row) in partition {
            run_datetime_array.push(
                i32::try_from(
                    (row.run_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            interval_datetime_array.push(
                i32::try_from(
                    (row.interval_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
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
                    arrow2::array::PrimitiveArray::from_slice(run_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(interval_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
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
/// ## P5MIN_PRICESENSITIVITIES
///  _Price Sensitivies for 5MinPD solution. New solution every 5 minutes. Current Scenarios defined in P5MIN_SCENARIODEMANDTRK/P5MIN_SCENARIODEMAND_
///
/// * Data Set Name: P5min
/// * File Name: Pricesensitivities
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minPricesensitivities1 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Region
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Whether an Intervention constraint was defined in this run
    pub intervention: rust_decimal::Decimal,
    /// Flag to indicate if the sensitivity run contains an active intervention constraint: 0 = No, 1 = Yes
    pub intervention_active: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 1
    pub rrp1: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 2
    pub rrp2: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 3
    pub rrp3: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 4
    pub rrp4: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 5
    pub rrp5: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 6
    pub rrp6: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 7
    pub rrp7: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 8
    pub rrp8: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 9
    pub rrp9: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 10
    pub rrp10: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 11
    pub rrp11: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 12
    pub rrp12: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 13
    pub rrp13: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 14
    pub rrp14: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 15
    pub rrp15: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 16
    pub rrp16: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 17
    pub rrp17: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 18
    pub rrp18: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 19
    pub rrp19: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 20
    pub rrp20: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 21
    pub rrp21: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 22
    pub rrp22: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 23
    pub rrp23: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 24
    pub rrp24: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 25
    pub rrp25: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 26
    pub rrp26: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 27
    pub rrp27: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 28
    pub rrp28: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 29
    pub rrp29: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 30
    pub rrp30: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 31
    pub rrp31: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 32
    pub rrp32: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 33
    pub rrp33: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 34
    pub rrp34: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 35
    pub rrp35: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 36
    pub rrp36: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 37
    pub rrp37: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 38
    pub rrp38: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 39
    pub rrp39: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 40
    pub rrp40: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 41
    pub rrp41: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 42
    pub rrp42: Option<rust_decimal::Decimal>,
    /// Regional Reference price for scenario 43
    pub rrp43: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for P5minPricesensitivities1 {
    type PrimaryKey = P5minPricesensitivities1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: Some("PRICESENSITIVITIES".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> P5minPricesensitivities1PrimaryKey {
        P5minPricesensitivities1PrimaryKey {
            interval_datetime: self.interval_datetime.clone(),
            regionid: self.regionid.clone(),
            run_datetime: self.run_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "p5min_pricesensitivities_v1".to_string()
    }
}
impl crate::CompareWithRow for P5minPricesensitivities1 {
    type Row = P5minPricesensitivities1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minPricesensitivities1 {
    type PrimaryKey = P5minPricesensitivities1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minPricesensitivities1PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: String,
    pub run_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for P5minPricesensitivities1PrimaryKey {
    type Row = P5minPricesensitivities1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minPricesensitivities1PrimaryKey {
    type PrimaryKey = P5minPricesensitivities1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime
    }
}
impl crate::PrimaryKey for P5minPricesensitivities1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for P5minPricesensitivities1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "run_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interval_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(1, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "intervention_active",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp1",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp2",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp3",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp4",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp5",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp6",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp7",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp8",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp9",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp10",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp11",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp12",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp13",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp14",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp15",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp16",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp17",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp18",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp19",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp20",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp21",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp22",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp23",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp24",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp25",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp26",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp27",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp28",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp29",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp30",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp31",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp32",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp33",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp34",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp35",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp36",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp37",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp38",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp39",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp40",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp41",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp42",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rrp43",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut run_datetime_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut interval_datetime_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut intervention_active_array = Vec::new();
        let mut rrp1_array = Vec::new();
        let mut rrp2_array = Vec::new();
        let mut rrp3_array = Vec::new();
        let mut rrp4_array = Vec::new();
        let mut rrp5_array = Vec::new();
        let mut rrp6_array = Vec::new();
        let mut rrp7_array = Vec::new();
        let mut rrp8_array = Vec::new();
        let mut rrp9_array = Vec::new();
        let mut rrp10_array = Vec::new();
        let mut rrp11_array = Vec::new();
        let mut rrp12_array = Vec::new();
        let mut rrp13_array = Vec::new();
        let mut rrp14_array = Vec::new();
        let mut rrp15_array = Vec::new();
        let mut rrp16_array = Vec::new();
        let mut rrp17_array = Vec::new();
        let mut rrp18_array = Vec::new();
        let mut rrp19_array = Vec::new();
        let mut rrp20_array = Vec::new();
        let mut rrp21_array = Vec::new();
        let mut rrp22_array = Vec::new();
        let mut rrp23_array = Vec::new();
        let mut rrp24_array = Vec::new();
        let mut rrp25_array = Vec::new();
        let mut rrp26_array = Vec::new();
        let mut rrp27_array = Vec::new();
        let mut rrp28_array = Vec::new();
        let mut rrp29_array = Vec::new();
        let mut rrp30_array = Vec::new();
        let mut rrp31_array = Vec::new();
        let mut rrp32_array = Vec::new();
        let mut rrp33_array = Vec::new();
        let mut rrp34_array = Vec::new();
        let mut rrp35_array = Vec::new();
        let mut rrp36_array = Vec::new();
        let mut rrp37_array = Vec::new();
        let mut rrp38_array = Vec::new();
        let mut rrp39_array = Vec::new();
        let mut rrp40_array = Vec::new();
        let mut rrp41_array = Vec::new();
        let mut rrp42_array = Vec::new();
        let mut rrp43_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            run_datetime_array.push(
                i32::try_from(
                    (row.run_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            regionid_array.push(row.regionid);
            interval_datetime_array.push(
                i32::try_from(
                    (row.interval_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            intervention_array.push({
                let mut val = row.intervention;
                val.rescale(0);
                val.mantissa()
            });
            intervention_active_array.push({
                row.intervention_active.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            rrp1_array.push({
                row.rrp1.map(|mut val| {
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
            rrp3_array.push({
                row.rrp3.map(|mut val| {
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
            rrp5_array.push({
                row.rrp5.map(|mut val| {
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
            rrp7_array.push({
                row.rrp7.map(|mut val| {
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
            rrp9_array.push({
                row.rrp9.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp10_array.push({
                row.rrp10.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp11_array.push({
                row.rrp11.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp12_array.push({
                row.rrp12.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp13_array.push({
                row.rrp13.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp14_array.push({
                row.rrp14.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp15_array.push({
                row.rrp15.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp16_array.push({
                row.rrp16.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp17_array.push({
                row.rrp17.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp18_array.push({
                row.rrp18.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp19_array.push({
                row.rrp19.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp20_array.push({
                row.rrp20.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp21_array.push({
                row.rrp21.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp22_array.push({
                row.rrp22.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp23_array.push({
                row.rrp23.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp24_array.push({
                row.rrp24.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp25_array.push({
                row.rrp25.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp26_array.push({
                row.rrp26.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp27_array.push({
                row.rrp27.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp28_array.push({
                row.rrp28.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp29_array.push({
                row.rrp29.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp30_array.push({
                row.rrp30.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp31_array.push({
                row.rrp31.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp32_array.push({
                row.rrp32.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp33_array.push({
                row.rrp33.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp34_array.push({
                row.rrp34.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp35_array.push({
                row.rrp35.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp36_array.push({
                row.rrp36.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp37_array.push({
                row.rrp37.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp38_array.push({
                row.rrp38.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp39_array.push({
                row.rrp39.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp40_array.push({
                row.rrp40.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp41_array.push({
                row.rrp41.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp42_array.push({
                row.rrp42.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp43_array.push({
                row.rrp43.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(run_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(interval_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_active_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp1_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp2_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp3_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp4_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp5_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp6_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp7_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp8_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp9_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp10_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp11_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp12_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp13_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp14_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp15_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp16_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp17_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp18_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp19_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp20_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp21_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp22_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp23_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp24_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp25_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp26_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp27_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp28_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp29_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp30_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp31_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp32_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp33_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp34_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp35_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp36_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp37_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp38_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp39_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp40_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp41_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp42_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp43_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## P5MIN_REGIONSOLUTION
///  _The five-minute predispatch (P5Min) is a MMS system providing projected dispatch for 12 Dispatch cycles (one hour). The 5-minute Predispatch cycle runs every 5-minutes to produce a dispatch and pricing schedule to a 5-minute resolution covering the next hour, a total of twelve periods.<br>P5MIN_REGIONSOLUTION shows the results of the regional capacity, maximum surplus reserve and maximum spare capacity evaluations for each period of the study.<br>_
///
/// * Data Set Name: P5min
/// * File Name: Regionsolution
/// * Data Version: 6
///
/// # Description
///  P5MIN_REGIONSOLUTION is public data, so is available to all participants. Source P5MIN_REGIONSOLUTION updates every 5 minutes. Volume Rows per day: 1440
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minRegionsolution6 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region Identifier
    pub regionid: String,
    /// Region Reference Price (Energy)
    pub rrp: Option<rust_decimal::Decimal>,
    /// Region Override Price (Energy)
    pub rop: Option<rust_decimal::Decimal>,
    /// Total Energy Imbalance (MW)
    pub excessgeneration: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Raise6Sec)
    pub raise6secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Raise6Sec)
    pub raise6secrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Raise60Sec)
    pub raise60secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Raise60Sec)
    pub raise60secrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Raise5Min)
    pub raise5minrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Raise5Min)
    pub raise5minrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (RaiseReg)
    pub raiseregrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (RaiseReg)
    pub raiseregrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Lower6Sec)
    pub lower6secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Lower6Sec)
    pub lower6secrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Lower60Sec)
    pub lower60secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Lower60Sec)
    pub lower60secrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (Lower5Min)
    pub lower5minrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (Lower5Min)
    pub lower5minrop: Option<rust_decimal::Decimal>,
    /// Region Reference Price (LowerReg)
    pub lowerregrrp: Option<rust_decimal::Decimal>,
    /// Original regional price (LowerReg)
    pub lowerregrop: Option<rust_decimal::Decimal>,
    /// Regional Demand - NB NOT net of Interconnector flows or Loads
    pub totaldemand: Option<rust_decimal::Decimal>,
    /// Regional Available generation
    pub availablegeneration: Option<rust_decimal::Decimal>,
    /// Regional Available Load
    pub availableload: Option<rust_decimal::Decimal>,
    /// Predicted change in regional demand for this interval
    pub demandforecast: Option<rust_decimal::Decimal>,
    /// Regional Generation Dispatched
    pub dispatchablegeneration: Option<rust_decimal::Decimal>,
    /// Regional Load Dispatched
    pub dispatchableload: Option<rust_decimal::Decimal>,
    /// Net interconnector Flows
    pub netinterchange: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW dispatch
    pub lower5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min MW imported
    pub lower5minimport: Option<rust_decimal::Decimal>,
    /// Lower 5 min local dispatch
    pub lower5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min local requirement
    pub lower5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 5 min total requirement
    pub lower5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW dispatch
    pub lower60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec MW imported
    pub lower60secimport: Option<rust_decimal::Decimal>,
    /// Lower 60 sec local dispatch
    pub lower60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec local requirement
    pub lower60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 60 sec total requirement
    pub lower60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW dispatch
    pub lower6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec MW imported
    pub lower6secimport: Option<rust_decimal::Decimal>,
    /// Lower 6 sec local dispatch
    pub lower6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec local requirement
    pub lower6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower 6 sec total requirement
    pub lower6secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Total Raise 5 min MW dispatch
    pub raise5mindispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min MW imported
    pub raise5minimport: Option<rust_decimal::Decimal>,
    /// Raise 5 min local dispatch
    pub raise5minlocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min local requirement
    pub raise5minlocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 5 min total requirement
    pub raise5minreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW dispatch
    pub raise60secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec MW imported
    pub raise60secimport: Option<rust_decimal::Decimal>,
    /// Raise 50 sec local dispatch
    pub raise60seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec local requirement
    pub raise60seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 60 sec total requirement
    pub raise60secreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec MW dispatch
    pub raise6secdispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec MW imported
    pub raise6secimport: Option<rust_decimal::Decimal>,
    /// Raise 6 sec local dispatch
    pub raise6seclocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec local requirement
    pub raise6seclocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise 6 sec total requirement
    pub raise6secreq: Option<rust_decimal::Decimal>,
    /// Aggregate dispatch error applied
    pub aggregatedispatcherror: Option<rust_decimal::Decimal>,
    /// Sum of initial generation and import for region
    pub initialsupply: Option<rust_decimal::Decimal>,
    /// Sum of cleared generation and import for region
    pub clearedsupply: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation MW imported
    pub lowerregimport: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Total Lower Regulation dispatch
    pub lowerregdispatch: Option<rust_decimal::Decimal>,
    /// Lower Regulation local dispatch
    pub lowerreglocaldispatch: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation local requirement
    pub lowerreglocalreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Lower Regulation total requirement
    pub lowerregreq: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Raise Regulation MW imported
    pub raiseregimport: Option<rust_decimal::Decimal>,
    /// Not used since Dec 2003. Total Raise Regulation dispatch
    pub raiseregdispatch: Option<rust_decimal::Decimal>,
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
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
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run (INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
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
impl crate::GetTable for P5minRegionsolution6 {
    type PrimaryKey = P5minRegionsolution6PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: Some("REGIONSOLUTION".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> P5minRegionsolution6PrimaryKey {
        P5minRegionsolution6PrimaryKey {
            interval_datetime: self.interval_datetime.clone(),
            regionid: self.regionid.clone(),
            run_datetime: self.run_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "p5min_regionsolution_v6".to_string()
    }
}
impl crate::CompareWithRow for P5minRegionsolution6 {
    type Row = P5minRegionsolution6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minRegionsolution6 {
    type PrimaryKey = P5minRegionsolution6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minRegionsolution6PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: String,
    pub run_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for P5minRegionsolution6PrimaryKey {
    type Row = P5minRegionsolution6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minRegionsolution6PrimaryKey {
    type PrimaryKey = P5minRegionsolution6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime
    }
}
impl crate::PrimaryKey for P5minRegionsolution6PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for P5minRegionsolution6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "run_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interval_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("rrp", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new("rop", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new(
                "excessgeneration",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
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
                "lower5minlocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minreq",
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
                "lower60seclocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secreq",
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
                "lower6seclocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secreq",
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
                "raise5minlocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minreq",
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
                "raise60seclocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secreq",
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
                "raise6seclocalreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6secreq",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "aggregatedispatcherror",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
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
                "lowerregdispatch",
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
                "raiseregdispatch",
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
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
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
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
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
        let mut run_datetime_array = Vec::new();
        let mut interval_datetime_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut rrp_array = Vec::new();
        let mut rop_array = Vec::new();
        let mut excessgeneration_array = Vec::new();
        let mut raise6secrrp_array = Vec::new();
        let mut raise6secrop_array = Vec::new();
        let mut raise60secrrp_array = Vec::new();
        let mut raise60secrop_array = Vec::new();
        let mut raise5minrrp_array = Vec::new();
        let mut raise5minrop_array = Vec::new();
        let mut raiseregrrp_array = Vec::new();
        let mut raiseregrop_array = Vec::new();
        let mut lower6secrrp_array = Vec::new();
        let mut lower6secrop_array = Vec::new();
        let mut lower60secrrp_array = Vec::new();
        let mut lower60secrop_array = Vec::new();
        let mut lower5minrrp_array = Vec::new();
        let mut lower5minrop_array = Vec::new();
        let mut lowerregrrp_array = Vec::new();
        let mut lowerregrop_array = Vec::new();
        let mut totaldemand_array = Vec::new();
        let mut availablegeneration_array = Vec::new();
        let mut availableload_array = Vec::new();
        let mut demandforecast_array = Vec::new();
        let mut dispatchablegeneration_array = Vec::new();
        let mut dispatchableload_array = Vec::new();
        let mut netinterchange_array = Vec::new();
        let mut lower5mindispatch_array = Vec::new();
        let mut lower5minimport_array = Vec::new();
        let mut lower5minlocaldispatch_array = Vec::new();
        let mut lower5minlocalreq_array = Vec::new();
        let mut lower5minreq_array = Vec::new();
        let mut lower60secdispatch_array = Vec::new();
        let mut lower60secimport_array = Vec::new();
        let mut lower60seclocaldispatch_array = Vec::new();
        let mut lower60seclocalreq_array = Vec::new();
        let mut lower60secreq_array = Vec::new();
        let mut lower6secdispatch_array = Vec::new();
        let mut lower6secimport_array = Vec::new();
        let mut lower6seclocaldispatch_array = Vec::new();
        let mut lower6seclocalreq_array = Vec::new();
        let mut lower6secreq_array = Vec::new();
        let mut raise5mindispatch_array = Vec::new();
        let mut raise5minimport_array = Vec::new();
        let mut raise5minlocaldispatch_array = Vec::new();
        let mut raise5minlocalreq_array = Vec::new();
        let mut raise5minreq_array = Vec::new();
        let mut raise60secdispatch_array = Vec::new();
        let mut raise60secimport_array = Vec::new();
        let mut raise60seclocaldispatch_array = Vec::new();
        let mut raise60seclocalreq_array = Vec::new();
        let mut raise60secreq_array = Vec::new();
        let mut raise6secdispatch_array = Vec::new();
        let mut raise6secimport_array = Vec::new();
        let mut raise6seclocaldispatch_array = Vec::new();
        let mut raise6seclocalreq_array = Vec::new();
        let mut raise6secreq_array = Vec::new();
        let mut aggregatedispatcherror_array = Vec::new();
        let mut initialsupply_array = Vec::new();
        let mut clearedsupply_array = Vec::new();
        let mut lowerregimport_array = Vec::new();
        let mut lowerregdispatch_array = Vec::new();
        let mut lowerreglocaldispatch_array = Vec::new();
        let mut lowerreglocalreq_array = Vec::new();
        let mut lowerregreq_array = Vec::new();
        let mut raiseregimport_array = Vec::new();
        let mut raiseregdispatch_array = Vec::new();
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
        let mut lastchanged_array = Vec::new();
        let mut totalintermittentgeneration_array = Vec::new();
        let mut demand_and_nonschedgen_array = Vec::new();
        let mut uigf_array = Vec::new();
        let mut semischedule_clearedmw_array = Vec::new();
        let mut semischedule_compliancemw_array = Vec::new();
        let mut intervention_array = Vec::new();
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
            run_datetime_array.push(
                i32::try_from(
                    (row.run_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            interval_datetime_array.push(
                i32::try_from(
                    (row.interval_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            regionid_array.push(row.regionid);
            rrp_array.push({
                row.rrp.map(|mut val| {
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
            excessgeneration_array.push({
                row.excessgeneration.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
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
            lower5minlocalreq_array.push({
                row.lower5minlocalreq.map(|mut val| {
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
            lower60seclocalreq_array.push({
                row.lower60seclocalreq.map(|mut val| {
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
            lower6seclocalreq_array.push({
                row.lower6seclocalreq.map(|mut val| {
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
            raise5minlocalreq_array.push({
                row.raise5minlocalreq.map(|mut val| {
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
            raise60seclocalreq_array.push({
                row.raise60seclocalreq.map(|mut val| {
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
            raise6seclocalreq_array.push({
                row.raise6seclocalreq.map(|mut val| {
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
            aggregatedispatcherror_array.push({
                row.aggregatedispatcherror.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
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
            lowerregdispatch_array.push({
                row.lowerregdispatch.map(|mut val| {
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
            raiseregdispatch_array.push({
                row.raiseregdispatch.map(|mut val| {
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
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
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
            intervention_array.push({
                row.intervention.map(|mut val| {
                    val.rescale(0);
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
                    arrow2::array::PrimitiveArray::from_slice(run_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(interval_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rop_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(excessgeneration_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
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
                    arrow2::array::PrimitiveArray::from(raise60secrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secrop_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
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
                    arrow2::array::PrimitiveArray::from(raiseregrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregrop_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
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
                    arrow2::array::PrimitiveArray::from(lower60secrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secrop_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
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
                    arrow2::array::PrimitiveArray::from(lowerregrrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregrop_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
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
                    arrow2::array::PrimitiveArray::from(lower5minlocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minreq_array)
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
                    arrow2::array::PrimitiveArray::from(lower60seclocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secreq_array)
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
                    arrow2::array::PrimitiveArray::from(lower6seclocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secreq_array)
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
                    arrow2::array::PrimitiveArray::from(raise5minlocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minreq_array)
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
                    arrow2::array::PrimitiveArray::from(raise60seclocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secreq_array)
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
                    arrow2::array::PrimitiveArray::from(raise6seclocalreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secreq_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(aggregatedispatcherror_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
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
                    arrow2::array::PrimitiveArray::from(lowerregdispatch_array)
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
                    arrow2::array::PrimitiveArray::from(raiseregdispatch_array)
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
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
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
                    arrow2::array::PrimitiveArray::from(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
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
/// ## P5MIN_SCENARIODEMAND
///  _The P5Min scenario MW offsets_
///
/// * Data Set Name: P5min
/// * File Name: Scenariodemand
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
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minScenariodemand1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// The scenario identifier
    pub scenario: rust_decimal::Decimal,
    /// The region to which to apply the deltaMW for this SCENARIO
    pub regionid: String,
    /// The MW offset to apply to region total demand for this SCENARIO
    pub deltamw: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for P5minScenariodemand1 {
    type PrimaryKey = P5minScenariodemand1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: Some("SCENARIODEMAND".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> P5minScenariodemand1PrimaryKey {
        P5minScenariodemand1PrimaryKey {
            effectivedate: self.effectivedate.clone(),
            regionid: self.regionid.clone(),
            scenario: self.scenario.clone(),
            version_datetime: self.version_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "p5min_scenariodemand_v1".to_string()
    }
}
impl crate::CompareWithRow for P5minScenariodemand1 {
    type Row = P5minScenariodemand1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.regionid == row.regionid
            && self.scenario == row.scenario
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minScenariodemand1 {
    type PrimaryKey = P5minScenariodemand1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.regionid == key.regionid
            && self.scenario == key.scenario
            && self.version_datetime == key.version_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minScenariodemand1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub regionid: String,
    pub scenario: rust_decimal::Decimal,
    pub version_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for P5minScenariodemand1PrimaryKey {
    type Row = P5minScenariodemand1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.regionid == row.regionid
            && self.scenario == row.scenario
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minScenariodemand1PrimaryKey {
    type PrimaryKey = P5minScenariodemand1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.regionid == key.regionid
            && self.scenario == key.scenario
            && self.version_datetime == key.version_datetime
    }
}
impl crate::PrimaryKey for P5minScenariodemand1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for P5minScenariodemand1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "version_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "scenario",
                arrow2::datatypes::DataType::Decimal(2, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "deltamw",
                arrow2::datatypes::DataType::Decimal(4, 0),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut effectivedate_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut scenario_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut deltamw_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            version_datetime_array.push(
                i32::try_from(
                    (row.version_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            scenario_array.push({
                let mut val = row.scenario;
                val.rescale(0);
                val.mantissa()
            });
            regionid_array.push(row.regionid);
            deltamw_array.push({
                row.deltamw.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(version_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(scenario_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(deltamw_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## P5MIN_SCENARIODEMANDTRK
///  _Tracks the 5Min scenario offset updates across time_
///
/// * Data Set Name: P5min
/// * File Name: Scenariodemandtrk
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
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minScenariodemandtrk1 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for P5minScenariodemandtrk1 {
    type PrimaryKey = P5minScenariodemandtrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: Some("SCENARIODEMANDTRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> P5minScenariodemandtrk1PrimaryKey {
        P5minScenariodemandtrk1PrimaryKey {
            effectivedate: self.effectivedate.clone(),
            version_datetime: self.version_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "p5min_scenariodemandtrk_v1".to_string()
    }
}
impl crate::CompareWithRow for P5minScenariodemandtrk1 {
    type Row = P5minScenariodemandtrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minScenariodemandtrk1 {
    type PrimaryKey = P5minScenariodemandtrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.version_datetime == key.version_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minScenariodemandtrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub version_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for P5minScenariodemandtrk1PrimaryKey {
    type Row = P5minScenariodemandtrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minScenariodemandtrk1PrimaryKey {
    type PrimaryKey = P5minScenariodemandtrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.version_datetime == key.version_datetime
    }
}
impl crate::PrimaryKey for P5minScenariodemandtrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for P5minScenariodemandtrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "version_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut effectivedate_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            version_datetime_array.push(
                i32::try_from(
                    (row.version_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            authoriseddate_array.push(row.authoriseddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(version_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## P5MIN_UNITSOLUTION
///  _The five-minute predispatch (P5Min) is a MMS system providing projected dispatch for 12 Dispatch cycles (one hour). The 5-minute Predispatch cycle runs every 5-minutes to produce a dispatch and pricing schedule to a 5-minute resolution covering the next hour, a total of twelve periods.<br>P5MIN_UNITSOLUTION shows the Unit results from the capacity evaluations for each period of the study.<br>_
///
/// * Data Set Name: P5min
/// * File Name: Unitsolution
/// * Data Version: 3
///
/// # Description
///  P5MIN_UNITSOLUTION data is confidential, so shows own details for participant. Source P5MIN_UNITSOLUTION updates every 5 minutes for all units, even zero targets. Volume Rows per day: 57600 Based on 200 units per Interval Note A bitwise flag exists for each ancillary service type such that a unit trapped or stranded in one or more service type can be immediately identified. The SPD Formulation document details the logic determining whether a unit is "trapped" or "stranded". The flag is defined as follows: Flagged Condition Bit Description Field value FCAS profile active 0 The bid profile for this service has been activated such that the unit is available to be cleared to provide this ancillary service type. 1 or 3 Trapped 1 The unit is enabled to provide this ancillary service type, however the profile for this service type is causing the unit to be trapped in the energy market. 3 Stranded 2 The unit is bid available to provide this ancillary service type, however, the unit is operating in the energy market outside of the profile for this service type and is stranded from providing this service. 4
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * DUID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct P5minUnitsolution3 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Dispatchable unit identifier
    pub duid: String,
    /// Connection point identifier for DUID
    pub connectionpointid: Option<String>,
    /// Generator or Load
    pub tradetype: Option<rust_decimal::Decimal>,
    /// AGC Status from EMS: 1 = on, 0 = off
    pub agcstatus: Option<rust_decimal::Decimal>,
    /// Initial MW at start of period. For periods subsequent to the first period of a P5MIN run, this value represents the cleared target for the previous period of that P5MIN run.
    pub initialmw: Option<rust_decimal::Decimal>,
    /// Target MW for end of period
    pub totalcleared: Option<rust_decimal::Decimal>,
    /// Ramp down rate (lessor of bid or telemetered rate).
    pub rampdownrate: Option<rust_decimal::Decimal>,
    /// Ramp up rate (lessor of bid or telemetered rate).
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
    /// Lower Regulation reserve target
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve target
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Energy Availability (MW)
    pub availability: Option<rust_decimal::Decimal>,
    /// Raise 6sec status flag
    pub raise6secflags: Option<rust_decimal::Decimal>,
    /// Raise 60sec status flag  
    pub raise60secflags: Option<rust_decimal::Decimal>,
    /// Raise 5min status flag  
    pub raise5minflags: Option<rust_decimal::Decimal>,
    /// Raise Reg status flag  
    pub raiseregflags: Option<rust_decimal::Decimal>,
    /// Lower 6sec status flag
    pub lower6secflags: Option<rust_decimal::Decimal>,
    /// Lower 60sec status flag  
    pub lower60secflags: Option<rust_decimal::Decimal>,
    /// Lower 5min status flag  
    pub lower5minflags: Option<rust_decimal::Decimal>,
    /// Lower Reg status flag  
    pub lowerregflags: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Boolean representation flagging if the Target is Capped
    pub semidispatchcap: Option<rust_decimal::Decimal>,
    /// Flag to indicate if this result set was sourced from the pricing run (INTERVENTION=0) or the physical run(INTERVENTION=1). In the event there is not intervention in the market, both pricing and physical runs correspond to INTERVENTION=0
    pub intervention: Option<rust_decimal::Decimal>,
    /// Minutes for which the unit has been in the current DISPATCHMODE. From NEMDE TRADERSOLUTION element FSTARGETMODETIME attribute.
    pub dispatchmodetime: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for P5minUnitsolution3 {
    type PrimaryKey = P5minUnitsolution3PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "P5MIN".into(),
            table_name: Some("UNITSOLUTION".into()),
            version: 3,
        }
    }

    fn primary_key(&self) -> P5minUnitsolution3PrimaryKey {
        P5minUnitsolution3PrimaryKey {
            duid: self.duid.clone(),
            interval_datetime: self.interval_datetime.clone(),
            run_datetime: self.run_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "p5min_unitsolution_v3".to_string()
    }
}
impl crate::CompareWithRow for P5minUnitsolution3 {
    type Row = P5minUnitsolution3;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minUnitsolution3 {
    type PrimaryKey = P5minUnitsolution3PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct P5minUnitsolution3PrimaryKey {
    pub duid: String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for P5minUnitsolution3PrimaryKey {
    type Row = P5minUnitsolution3;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for P5minUnitsolution3PrimaryKey {
    type PrimaryKey = P5minUnitsolution3PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
impl crate::PrimaryKey for P5minUnitsolution3PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for P5minUnitsolution3 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "run_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interval_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "tradetype",
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
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "semidispatchcap",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::Decimal(2, 0),
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
        let mut run_datetime_array = Vec::new();
        let mut interval_datetime_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut tradetype_array = Vec::new();
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
        let mut lastchanged_array = Vec::new();
        let mut semidispatchcap_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut dispatchmodetime_array = Vec::new();
        for (_, row) in partition {
            run_datetime_array.push(
                i32::try_from(
                    (row.run_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            interval_datetime_array.push(
                i32::try_from(
                    (row.interval_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            duid_array.push(row.duid);
            connectionpointid_array.push(row.connectionpointid);
            tradetype_array.push({
                row.tradetype.map(|mut val| {
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
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            semidispatchcap_array.push({
                row.semidispatchcap.map(|mut val| {
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
                    arrow2::array::PrimitiveArray::from_slice(run_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(interval_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    connectionpointid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(tradetype_array)
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
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(semidispatchcap_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervention_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
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
