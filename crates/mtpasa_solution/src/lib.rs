/// # Summary
///
/// ## MTPASA_CASERESULT
///  _MTPASA solution header table_
///
/// * Data Set Name: Mtpasa
/// * File Name: Caseresult
/// * Data Version: 1
///
/// # Description
///  MTPASA_CASERESULT is public data. Holds one Record for entire solution
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * RUN_DATETIME
/// * RUN_NO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaCaseresult1 {
    /// Date processing of the run begins.
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Version of PLEXOS used
    pub plexos_version: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MtpasaCaseresult1 {
    type PrimaryKey = MtpasaCaseresult1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("CASERESULT".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MtpasaCaseresult1PrimaryKey {
        MtpasaCaseresult1PrimaryKey {
            run_datetime: self.run_datetime,
            run_no: self.run_no,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mtpasa_caseresult_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MtpasaCaseresult1PrimaryKey {
    pub run_datetime: chrono::NaiveDateTime,
    pub run_no: i64,
}
impl mmsdm_core::PrimaryKey for MtpasaCaseresult1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MtpasaCaseresult1 {
    type Row = MtpasaCaseresult1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.run_datetime == row.run_datetime && self.run_no == row.run_no
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaCaseresult1 {
    type PrimaryKey = MtpasaCaseresult1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime && self.run_no == key.run_no
    }
}
impl mmsdm_core::CompareWithRow for MtpasaCaseresult1PrimaryKey {
    type Row = MtpasaCaseresult1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.run_datetime == row.run_datetime && self.run_no == row.run_no
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaCaseresult1PrimaryKey {
    type PrimaryKey = MtpasaCaseresult1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime && self.run_no == key.run_no
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaCaseresult1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("run_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("run_no",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("plexos_version",
                arrow2::datatypes::DataType::LargeUtf8, true),
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
        let mut run_datetime_array = Vec::new();
        let mut run_no_array = Vec::new();
        let mut plexos_version_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            run_datetime_array.push(row.run_datetime.timestamp());
            run_no_array.push(row.run_no);
            plexos_version_array.push(row.plexos_version);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_no_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(plexos_version_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
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
/// ## MTPASA_CONSTRAINTRESULT
///  _Constraint results for Binding or Violating Constraints_
///
/// * Data Set Name: Mtpasa
/// * File Name: Constraintresult
/// * Data Version: 1
///
/// # Description
///  MTPASA_CONSTRAINTRESULT is public data. &nbsp;
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * DAY
/// * DEMAND_POE_TYPE
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaConstraintresult1 {
    /// Date processing of the run begins.
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: String,
    /// Demand POE type used. Value is POE10
    pub demand_poe_type: String,
    /// Day this result is for
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub day: chrono::NaiveDateTime,
    /// The unique identifier for the constraint. Only binding or violating constraints are reported
    pub constraintid: String,
    /// The effective date of the constraint used
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// The version of the constraint used
    pub versionno: Option<rust_decimal::Decimal>,
    /// Half hourly period reported, selected as period of maximum NEM scheduled demand (calculated as maximum of scheduled demands, averaged across iterations and reference years)
    pub periodid: Option<rust_decimal::Decimal>,
    /// Proportion of a constraint binding, across iterations and reference years
    pub probabilityofbinding: Option<rust_decimal::Decimal>,
    /// Proportion of a constraint violating, across iterations and reference years
    pub probabilityofviolation: Option<rust_decimal::Decimal>,
    /// The 90th percentile violation degree for this constraint, across iterations and reference years (MW)
    pub constraintviolation90: Option<rust_decimal::Decimal>,
    /// The 50th percentile violation degree for this constraint, across iterations and reference years (MW)
    pub constraintviolation50: Option<rust_decimal::Decimal>,
    /// The 10th percentile violation degree for this constraint, across iterations and reference years (MW)
    pub constraintviolation10: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MtpasaConstraintresult1 {
    type PrimaryKey = MtpasaConstraintresult1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("CONSTRAINTRESULT".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MtpasaConstraintresult1PrimaryKey {
        MtpasaConstraintresult1PrimaryKey {
            constraintid: self.constraintid.clone(),
            day: self.day,
            demand_poe_type: self.demand_poe_type.clone(),
            run_datetime: self.run_datetime,
            run_no: self.run_no,
            runtype: self.runtype.clone(),
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mtpasa_constraintresult_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MtpasaConstraintresult1PrimaryKey {
    pub constraintid: String,
    pub day: chrono::NaiveDateTime,
    pub demand_poe_type: String,
    pub run_datetime: chrono::NaiveDateTime,
    pub run_no: i64,
    pub runtype: String,
}
impl mmsdm_core::PrimaryKey for MtpasaConstraintresult1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MtpasaConstraintresult1 {
    type Row = MtpasaConstraintresult1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid && self.day == row.day
            && self.demand_poe_type == row.demand_poe_type
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaConstraintresult1 {
    type PrimaryKey = MtpasaConstraintresult1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.day == key.day
            && self.demand_poe_type == key.demand_poe_type
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
    }
}
impl mmsdm_core::CompareWithRow for MtpasaConstraintresult1PrimaryKey {
    type Row = MtpasaConstraintresult1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid && self.day == row.day
            && self.demand_poe_type == row.demand_poe_type
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaConstraintresult1PrimaryKey {
    type PrimaryKey = MtpasaConstraintresult1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid && self.day == key.day
            && self.demand_poe_type == key.demand_poe_type
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaConstraintresult1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("run_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("run_no",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("runtype",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("demand_poe_type",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("day",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("constraintid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("probabilityofbinding",
                arrow2::datatypes::DataType::Decimal(8, 5), true),
                arrow2::datatypes::Field::new("probabilityofviolation",
                arrow2::datatypes::DataType::Decimal(8, 5), true),
                arrow2::datatypes::Field::new("constraintviolation90",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("constraintviolation50",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("constraintviolation10",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
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
        let mut run_datetime_array = Vec::new();
        let mut run_no_array = Vec::new();
        let mut runtype_array = Vec::new();
        let mut demand_poe_type_array = Vec::new();
        let mut day_array = Vec::new();
        let mut constraintid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut probabilityofbinding_array = Vec::new();
        let mut probabilityofviolation_array = Vec::new();
        let mut constraintviolation90_array = Vec::new();
        let mut constraintviolation50_array = Vec::new();
        let mut constraintviolation10_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            run_datetime_array.push(row.run_datetime.timestamp());
            run_no_array.push(row.run_no);
            runtype_array.push(row.runtype);
            demand_poe_type_array.push(row.demand_poe_type);
            day_array.push(row.day.timestamp());
            constraintid_array.push(row.constraintid);
            effectivedate_array.push(row.effectivedate.map(|val| val.timestamp()));
            versionno_array
                .push({
                    row.versionno
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            periodid_array
                .push({
                    row.periodid
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            probabilityofbinding_array
                .push({
                    row.probabilityofbinding
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            probabilityofviolation_array
                .push({
                    row.probabilityofviolation
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            constraintviolation90_array
                .push({
                    row.constraintviolation90
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            constraintviolation50_array
                .push({
                    row.constraintviolation50
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            constraintviolation10_array
                .push({
                    row.constraintviolation10
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_no_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(runtype_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(demand_poe_type_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(day_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(constraintid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(effectivedate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(probabilityofbinding_array)
                    .to(arrow2::datatypes::DataType::Decimal(8, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(probabilityofviolation_array)
                    .to(arrow2::datatypes::DataType::Decimal(8, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(constraintviolation90_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(constraintviolation50_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(constraintviolation10_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
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
/// ## MTPASA_CONSTRAINTSUMMARY
///  _Constraint Summary results over aggregation periods_
///
/// * Data Set Name: Mtpasa
/// * File Name: Constraintsummary
/// * Data Version: 1
///
/// # Description
///  MTPASA_CONSTRAINTSUMMARY is public data.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * AGGREGATION_PERIOD
/// * CONSTRAINTID
/// * DAY
/// * DEMAND_POE_TYPE
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaConstraintsummary1 {
    /// Date processing of the run begins.
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: String,
    /// Demand POE type used. Value is POE10
    pub demand_poe_type: String,
    /// Day this result is for
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub day: chrono::NaiveDateTime,
    /// The unique identifier for the constraint. Only binding or violating constraints are reported
    pub constraintid: String,
    /// The effective date of the constraint used
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// The version of the constraintID
    pub versionno: Option<rust_decimal::Decimal>,
    /// Period data is aggregated over. Values are PEAK, SHOULDER, OFFPEAK. PEAK = 14:00-19:59, SHOULDER = 07:00-13:59 and 20:00-21:59, OFFPEAK = 22:00-06:59
    pub aggregation_period: String,
    /// Constraint hours binding or violating for period, averaged across iterations and reference years
    pub constrainthoursbinding: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MtpasaConstraintsummary1 {
    type PrimaryKey = MtpasaConstraintsummary1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("CONSTRAINTSUMMARY".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MtpasaConstraintsummary1PrimaryKey {
        MtpasaConstraintsummary1PrimaryKey {
            aggregation_period: self.aggregation_period.clone(),
            constraintid: self.constraintid.clone(),
            day: self.day,
            demand_poe_type: self.demand_poe_type.clone(),
            run_datetime: self.run_datetime,
            run_no: self.run_no,
            runtype: self.runtype.clone(),
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mtpasa_constraintsummary_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MtpasaConstraintsummary1PrimaryKey {
    pub aggregation_period: String,
    pub constraintid: String,
    pub day: chrono::NaiveDateTime,
    pub demand_poe_type: String,
    pub run_datetime: chrono::NaiveDateTime,
    pub run_no: i64,
    pub runtype: String,
}
impl mmsdm_core::PrimaryKey for MtpasaConstraintsummary1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MtpasaConstraintsummary1 {
    type Row = MtpasaConstraintsummary1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.aggregation_period == row.aggregation_period
            && self.constraintid == row.constraintid && self.day == row.day
            && self.demand_poe_type == row.demand_poe_type
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaConstraintsummary1 {
    type PrimaryKey = MtpasaConstraintsummary1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.aggregation_period == key.aggregation_period
            && self.constraintid == key.constraintid && self.day == key.day
            && self.demand_poe_type == key.demand_poe_type
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
    }
}
impl mmsdm_core::CompareWithRow for MtpasaConstraintsummary1PrimaryKey {
    type Row = MtpasaConstraintsummary1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.aggregation_period == row.aggregation_period
            && self.constraintid == row.constraintid && self.day == row.day
            && self.demand_poe_type == row.demand_poe_type
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaConstraintsummary1PrimaryKey {
    type PrimaryKey = MtpasaConstraintsummary1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.aggregation_period == key.aggregation_period
            && self.constraintid == key.constraintid && self.day == key.day
            && self.demand_poe_type == key.demand_poe_type
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaConstraintsummary1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("run_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("run_no",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("runtype",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("demand_poe_type",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("day",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("constraintid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("aggregation_period",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("constrainthoursbinding",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
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
        let mut run_datetime_array = Vec::new();
        let mut run_no_array = Vec::new();
        let mut runtype_array = Vec::new();
        let mut demand_poe_type_array = Vec::new();
        let mut day_array = Vec::new();
        let mut constraintid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut aggregation_period_array = Vec::new();
        let mut constrainthoursbinding_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            run_datetime_array.push(row.run_datetime.timestamp());
            run_no_array.push(row.run_no);
            runtype_array.push(row.runtype);
            demand_poe_type_array.push(row.demand_poe_type);
            day_array.push(row.day.timestamp());
            constraintid_array.push(row.constraintid);
            effectivedate_array.push(row.effectivedate.map(|val| val.timestamp()));
            versionno_array
                .push({
                    row.versionno
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            aggregation_period_array.push(row.aggregation_period);
            constrainthoursbinding_array
                .push({
                    row.constrainthoursbinding
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_no_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(runtype_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(demand_poe_type_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(day_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(constraintid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(effectivedate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(aggregation_period_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(constrainthoursbinding_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
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
/// ## MTPASA_DUIDAVAILABILITY
///  _Offered PASA Availability of the scheduled generator DUID for each day over the Medium Term PASA period. The data in this table is input data to the MT PASA process it is not part of the MTPASA solution. The availability does not reflect any energy limitations in the MT PASA offers_
///
/// * Data Set Name: Mtpasa
/// * File Name: Duidavailability
/// * Data Version: 2
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DAY
/// * DUID
/// * PUBLISH_DATETIME
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaDuidavailability2 {
    /// Date Time the report was published.
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub publish_datetime: chrono::NaiveDateTime,
    /// Date on which the PASA availability of DUID applies.
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub day: chrono::NaiveDateTime,
    /// NEM Region.
    pub regionid: String,
    /// NEM DUID.
    pub duid: String,
    /// Offered PASA Availability of Scheduled generator DUID for the day.
    pub pasaavailability: Option<rust_decimal::Decimal>,
    /// Date Time of the latest offer used for DUID for this date.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub latest_offer_datetime: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Status of a reported capacity value (e.g. 1 for Yes, 0 for No)
    pub carryoverstatus: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for MtpasaDuidavailability2 {
    type PrimaryKey = MtpasaDuidavailability2PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("DUIDAVAILABILITY".into()),
            version: 2,
        }
    }
    fn primary_key(&self) -> MtpasaDuidavailability2PrimaryKey {
        MtpasaDuidavailability2PrimaryKey {
            day: self.day,
            duid: self.duid.clone(),
            publish_datetime: self.publish_datetime,
            regionid: self.regionid.clone(),
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mtpasa_duidavailability_v2".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MtpasaDuidavailability2PrimaryKey {
    pub day: chrono::NaiveDateTime,
    pub duid: String,
    pub publish_datetime: chrono::NaiveDateTime,
    pub regionid: String,
}
impl mmsdm_core::PrimaryKey for MtpasaDuidavailability2PrimaryKey {}
impl mmsdm_core::CompareWithRow for MtpasaDuidavailability2 {
    type Row = MtpasaDuidavailability2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.day == row.day && self.duid == row.duid
            && self.publish_datetime == row.publish_datetime
            && self.regionid == row.regionid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaDuidavailability2 {
    type PrimaryKey = MtpasaDuidavailability2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.duid == key.duid
            && self.publish_datetime == key.publish_datetime
            && self.regionid == key.regionid
    }
}
impl mmsdm_core::CompareWithRow for MtpasaDuidavailability2PrimaryKey {
    type Row = MtpasaDuidavailability2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.day == row.day && self.duid == row.duid
            && self.publish_datetime == row.publish_datetime
            && self.regionid == row.regionid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaDuidavailability2PrimaryKey {
    type PrimaryKey = MtpasaDuidavailability2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.duid == key.duid
            && self.publish_datetime == key.publish_datetime
            && self.regionid == key.regionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaDuidavailability2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("publish_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("day",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("duid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("pasaavailability",
                arrow2::datatypes::DataType::Decimal(12, 0), true),
                arrow2::datatypes::Field::new("latest_offer_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("carryoverstatus",
                arrow2::datatypes::DataType::Decimal(1, 0), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut publish_datetime_array = Vec::new();
        let mut day_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut pasaavailability_array = Vec::new();
        let mut latest_offer_datetime_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut carryoverstatus_array = Vec::new();
        for row in partition {
            publish_datetime_array.push(row.publish_datetime.timestamp());
            day_array.push(row.day.timestamp());
            regionid_array.push(row.regionid);
            duid_array.push(row.duid);
            pasaavailability_array
                .push({
                    row.pasaavailability
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            latest_offer_datetime_array
                .push(row.latest_offer_datetime.map(|val| val.timestamp()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            carryoverstatus_array
                .push({
                    row.carryoverstatus
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(publish_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(day_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(duid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(pasaavailability_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(latest_offer_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(carryoverstatus_array)
                    .to(arrow2::datatypes::DataType::Decimal(1, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MTPASA_INTERCONNECTORRESULT
///  _Interconnector results for interval of max demand per day_
///
/// * Data Set Name: Mtpasa
/// * File Name: Interconnectorresult
/// * Data Version: 1
///
/// # Description
///  MTPASA_INTERCONNECTORRESULT is public data.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DAY
/// * DEMAND_POE_TYPE
/// * INTERCONNECTORID
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaInterconnectorresult1 {
    /// Date processing of the run begins.
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: String,
    /// Demand POE type used. Value is POE10
    pub demand_poe_type: String,
    /// Day this result is for
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub day: chrono::NaiveDateTime,
    /// The unique identifier for the interconnector
    pub interconnectorid: String,
    /// Half hourly period reported, selected as period of maximum NEM scheduled demand (calculated as maximum of scheduled demands, averaged across iterations and reference years)
    pub periodid: Option<rust_decimal::Decimal>,
    /// The 90th percentile for flows, across iterations and reference years. Positive values indicate exporting, negative values indicate importing (MW)
    pub flow90: Option<rust_decimal::Decimal>,
    /// The 50th percentile for flows, across iterations and reference years. Positive values indicate exporting, negative values indicate importing (MW)
    pub flow50: Option<rust_decimal::Decimal>,
    /// The 10th percentile for flows, across iterations and reference years. Positive values indicate exporting, negative values indicate importing (MW)
    pub flow10: Option<rust_decimal::Decimal>,
    /// Proportion of iterations and reference years with interconnector constrained when exporting
    pub probabilityofbindingexport: Option<rust_decimal::Decimal>,
    /// Proportion of iterations and reference years with interconnector constrained when importing
    pub probabilityofbindingimport: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of exporting energy on the basis of invoked constraints and static interconnector export limit, averaged across iterations and reference years
    pub calculatedexportlimit: Option<rust_decimal::Decimal>,
    /// Calculated Interconnector limit of importing energy on the basis of invoked constraints and static interconnector import limit, averaged across iterations and reference years
    pub calculatedimportlimit: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MtpasaInterconnectorresult1 {
    type PrimaryKey = MtpasaInterconnectorresult1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("INTERCONNECTORRESULT".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MtpasaInterconnectorresult1PrimaryKey {
        MtpasaInterconnectorresult1PrimaryKey {
            day: self.day,
            demand_poe_type: self.demand_poe_type.clone(),
            interconnectorid: self.interconnectorid.clone(),
            run_datetime: self.run_datetime,
            run_no: self.run_no,
            runtype: self.runtype.clone(),
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mtpasa_interconnectorresult_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MtpasaInterconnectorresult1PrimaryKey {
    pub day: chrono::NaiveDateTime,
    pub demand_poe_type: String,
    pub interconnectorid: String,
    pub run_datetime: chrono::NaiveDateTime,
    pub run_no: i64,
    pub runtype: String,
}
impl mmsdm_core::PrimaryKey for MtpasaInterconnectorresult1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MtpasaInterconnectorresult1 {
    type Row = MtpasaInterconnectorresult1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.day == row.day && self.demand_poe_type == row.demand_poe_type
            && self.interconnectorid == row.interconnectorid
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaInterconnectorresult1 {
    type PrimaryKey = MtpasaInterconnectorresult1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.demand_poe_type == key.demand_poe_type
            && self.interconnectorid == key.interconnectorid
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
    }
}
impl mmsdm_core::CompareWithRow for MtpasaInterconnectorresult1PrimaryKey {
    type Row = MtpasaInterconnectorresult1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.day == row.day && self.demand_poe_type == row.demand_poe_type
            && self.interconnectorid == row.interconnectorid
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaInterconnectorresult1PrimaryKey {
    type PrimaryKey = MtpasaInterconnectorresult1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.demand_poe_type == key.demand_poe_type
            && self.interconnectorid == key.interconnectorid
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaInterconnectorresult1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("run_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("run_no",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("runtype",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("demand_poe_type",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("day",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("flow90",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("flow50",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("flow10",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("probabilityofbindingexport",
                arrow2::datatypes::DataType::Decimal(8, 5), true),
                arrow2::datatypes::Field::new("probabilityofbindingimport",
                arrow2::datatypes::DataType::Decimal(8, 5), true),
                arrow2::datatypes::Field::new("calculatedexportlimit",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("calculatedimportlimit",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
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
        let mut run_datetime_array = Vec::new();
        let mut run_no_array = Vec::new();
        let mut runtype_array = Vec::new();
        let mut demand_poe_type_array = Vec::new();
        let mut day_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut flow90_array = Vec::new();
        let mut flow50_array = Vec::new();
        let mut flow10_array = Vec::new();
        let mut probabilityofbindingexport_array = Vec::new();
        let mut probabilityofbindingimport_array = Vec::new();
        let mut calculatedexportlimit_array = Vec::new();
        let mut calculatedimportlimit_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            run_datetime_array.push(row.run_datetime.timestamp());
            run_no_array.push(row.run_no);
            runtype_array.push(row.runtype);
            demand_poe_type_array.push(row.demand_poe_type);
            day_array.push(row.day.timestamp());
            interconnectorid_array.push(row.interconnectorid);
            periodid_array
                .push({
                    row.periodid
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            flow90_array
                .push({
                    row.flow90
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            flow50_array
                .push({
                    row.flow50
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            flow10_array
                .push({
                    row.flow10
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            probabilityofbindingexport_array
                .push({
                    row.probabilityofbindingexport
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            probabilityofbindingimport_array
                .push({
                    row.probabilityofbindingimport
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            calculatedexportlimit_array
                .push({
                    row.calculatedexportlimit
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            calculatedimportlimit_array
                .push({
                    row.calculatedimportlimit
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_no_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(runtype_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(demand_poe_type_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(day_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(interconnectorid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(flow90_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(flow50_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(flow10_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(probabilityofbindingexport_array)
                    .to(arrow2::datatypes::DataType::Decimal(8, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(probabilityofbindingimport_array)
                    .to(arrow2::datatypes::DataType::Decimal(8, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(calculatedexportlimit_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(calculatedimportlimit_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
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
/// ## MTPASA_LOLPRESULT
///  _Results for Loss of Load Probability (LOLP) run per day_
///
/// * Data Set Name: Mtpasa
/// * File Name: Lolpresult
/// * Data Version: 1
///
/// # Description
///  MTPASA_LOLPRESULT is public data.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DAY
/// * REGIONID
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaLolpresult1 {
    /// Date processing of the run begins.
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always LOLP
    pub runtype: String,
    /// Day this result is for
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub day: chrono::NaiveDateTime,
    /// The unique region identifier
    pub regionid: String,
    /// The half hourly interval period with the highest LOLP, or highest region demand if LOLP = 0 for all intervals (1..48)
    pub worst_interval_periodid: Option<rust_decimal::Decimal>,
    /// The Abstract Operational Demand for the worst interval in this region (MW)
    pub worst_interval_demand: Option<rust_decimal::Decimal>,
    /// The half hourly aggregate intermittent generation for the worst interval in this region (MW)
    pub worst_interval_intgen: Option<rust_decimal::Decimal>,
    /// The half hourly aggregate demand side participation for the worst interval period in this region (MW)
    pub worst_interval_dsp: Option<rust_decimal::Decimal>,
    /// Loss of Load Probability for the worst interval in this region
    pub lossofloadprobability: Option<rust_decimal::Decimal>,
    /// Loss of Load Magnitude for the worst interval in this region. Values are LOW, MEDIUM, HIGH
    pub lossofloadmagnitude: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MtpasaLolpresult1 {
    type PrimaryKey = MtpasaLolpresult1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("LOLPRESULT".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MtpasaLolpresult1PrimaryKey {
        MtpasaLolpresult1PrimaryKey {
            day: self.day,
            regionid: self.regionid.clone(),
            run_datetime: self.run_datetime,
            run_no: self.run_no,
            runtype: self.runtype.clone(),
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mtpasa_lolpresult_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MtpasaLolpresult1PrimaryKey {
    pub day: chrono::NaiveDateTime,
    pub regionid: String,
    pub run_datetime: chrono::NaiveDateTime,
    pub run_no: i64,
    pub runtype: String,
}
impl mmsdm_core::PrimaryKey for MtpasaLolpresult1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MtpasaLolpresult1 {
    type Row = MtpasaLolpresult1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.day == row.day && self.regionid == row.regionid
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaLolpresult1 {
    type PrimaryKey = MtpasaLolpresult1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
    }
}
impl mmsdm_core::CompareWithRow for MtpasaLolpresult1PrimaryKey {
    type Row = MtpasaLolpresult1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.day == row.day && self.regionid == row.regionid
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaLolpresult1PrimaryKey {
    type PrimaryKey = MtpasaLolpresult1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaLolpresult1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("run_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("run_no",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("runtype",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("day",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("worst_interval_periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("worst_interval_demand",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("worst_interval_intgen",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("worst_interval_dsp",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("lossofloadprobability",
                arrow2::datatypes::DataType::Decimal(8, 5), true),
                arrow2::datatypes::Field::new("lossofloadmagnitude",
                arrow2::datatypes::DataType::LargeUtf8, true),
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
        let mut run_datetime_array = Vec::new();
        let mut run_no_array = Vec::new();
        let mut runtype_array = Vec::new();
        let mut day_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut worst_interval_periodid_array = Vec::new();
        let mut worst_interval_demand_array = Vec::new();
        let mut worst_interval_intgen_array = Vec::new();
        let mut worst_interval_dsp_array = Vec::new();
        let mut lossofloadprobability_array = Vec::new();
        let mut lossofloadmagnitude_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            run_datetime_array.push(row.run_datetime.timestamp());
            run_no_array.push(row.run_no);
            runtype_array.push(row.runtype);
            day_array.push(row.day.timestamp());
            regionid_array.push(row.regionid);
            worst_interval_periodid_array
                .push({
                    row.worst_interval_periodid
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            worst_interval_demand_array
                .push({
                    row.worst_interval_demand
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            worst_interval_intgen_array
                .push({
                    row.worst_interval_intgen
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            worst_interval_dsp_array
                .push({
                    row.worst_interval_dsp
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            lossofloadprobability_array
                .push({
                    row.lossofloadprobability
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lossofloadmagnitude_array.push(row.lossofloadmagnitude);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_no_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(runtype_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(day_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(worst_interval_periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(worst_interval_demand_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(worst_interval_intgen_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(worst_interval_dsp_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lossofloadprobability_array)
                    .to(arrow2::datatypes::DataType::Decimal(8, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(lossofloadmagnitude_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
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
/// ## MTPASA_REGIONAVAILABILITY
///  _Stores the Region-aggregate offered PASA Availability of scheduled generators for each day over the Medium Term PASA period. The data in this table is an aggregate of input data to the MT PASA process it is not part of the MTPASA solution. The aggregate availability does not reflect any energy limitations in the MT PASA offers._
///
/// * Data Set Name: Mtpasa
/// * File Name: Regionavailability
/// * Data Version: 4
///
/// # Description
///  MTPASA_REGIONAVAILABILITY is public data.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DAY
/// * PUBLISH_DATETIME
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaRegionavailability4 {
    /// Date Time the report was published.
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub publish_datetime: chrono::NaiveDateTime,
    /// Date on which the aggregation applies.
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub day: chrono::NaiveDateTime,
    /// NEM Region.
    pub regionid: String,
    /// Aggregate of the offered PASA Availability for all Scheduled generators in this region.
    pub pasaavailability_scheduled: Option<rust_decimal::Decimal>,
    /// Date Time of the latest offer used in the aggregation for this region and date.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub latest_offer_datetime: Option<chrono::NaiveDateTime>,
    /// Region energy unconstrained MW capacity
    pub energyunconstrainedcapacity: Option<rust_decimal::Decimal>,
    /// Region energy constrained MW capacity
    pub energyconstrainedcapacity: Option<rust_decimal::Decimal>,
    /// Allowance made for non-scheduled generation in the demand forecast (MW)
    pub nonscheduledgeneration: Option<rust_decimal::Decimal>,
    /// 10% probability demand (ex non-scheduled demand)
    pub demand10: Option<rust_decimal::Decimal>,
    /// 50% probability demand (ex non-scheduled demand)
    pub demand50: Option<rust_decimal::Decimal>,
    /// Total weekly operational as generated consumption (POE 10)
    pub energyreqdemand10: Option<rust_decimal::Decimal>,
    /// Total weekly operational as generated consumption (POE 50)
    pub energyreqdemand50: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Minimum of the Operational Load as Generated (OPGEN) peaks that occur in all ref years for the P10 traces (MW).
    pub demand10min: Option<rust_decimal::Decimal>,
    /// Maximum of the Operational Load as Generated (OPGEN) peaks that occur in all ref years for the P10 traces (MW).
    pub demand10max: Option<rust_decimal::Decimal>,
    /// Minimum of the Operational Load as Generated (OPGEN) peaks that occur in all ref years for the P50 traces (MW).
    pub demand50min: Option<rust_decimal::Decimal>,
    /// Maximum of the Operational Load as Generated (OPGEN) peaks that occur in all ref years for the P50 traces (MW).
    pub demand50max: Option<rust_decimal::Decimal>,
    /// Split of the CARRYOVER component of aggregate capacity vs the currently reported capacity.
    pub carryovercapacity: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for MtpasaRegionavailability4 {
    type PrimaryKey = MtpasaRegionavailability4PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("REGIONAVAILABILITY".into()),
            version: 4,
        }
    }
    fn primary_key(&self) -> MtpasaRegionavailability4PrimaryKey {
        MtpasaRegionavailability4PrimaryKey {
            day: self.day,
            publish_datetime: self.publish_datetime,
            regionid: self.regionid.clone(),
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mtpasa_regionavailability_v4".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MtpasaRegionavailability4PrimaryKey {
    pub day: chrono::NaiveDateTime,
    pub publish_datetime: chrono::NaiveDateTime,
    pub regionid: String,
}
impl mmsdm_core::PrimaryKey for MtpasaRegionavailability4PrimaryKey {}
impl mmsdm_core::CompareWithRow for MtpasaRegionavailability4 {
    type Row = MtpasaRegionavailability4;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.day == row.day && self.publish_datetime == row.publish_datetime
            && self.regionid == row.regionid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaRegionavailability4 {
    type PrimaryKey = MtpasaRegionavailability4PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.publish_datetime == key.publish_datetime
            && self.regionid == key.regionid
    }
}
impl mmsdm_core::CompareWithRow for MtpasaRegionavailability4PrimaryKey {
    type Row = MtpasaRegionavailability4;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.day == row.day && self.publish_datetime == row.publish_datetime
            && self.regionid == row.regionid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaRegionavailability4PrimaryKey {
    type PrimaryKey = MtpasaRegionavailability4PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.publish_datetime == key.publish_datetime
            && self.regionid == key.regionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaRegionavailability4 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("publish_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("day",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("pasaavailability_scheduled",
                arrow2::datatypes::DataType::Decimal(12, 0), true),
                arrow2::datatypes::Field::new("latest_offer_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true),
                arrow2::datatypes::Field::new("energyunconstrainedcapacity",
                arrow2::datatypes::DataType::Decimal(12, 0), true),
                arrow2::datatypes::Field::new("energyconstrainedcapacity",
                arrow2::datatypes::DataType::Decimal(12, 0), true),
                arrow2::datatypes::Field::new("nonscheduledgeneration",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("demand10",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("demand50",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("energyreqdemand10",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("energyreqdemand50",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("demand10min",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("demand10max",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("demand50min",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("demand50max",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("carryovercapacity",
                arrow2::datatypes::DataType::Decimal(12, 0), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut publish_datetime_array = Vec::new();
        let mut day_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut pasaavailability_scheduled_array = Vec::new();
        let mut latest_offer_datetime_array = Vec::new();
        let mut energyunconstrainedcapacity_array = Vec::new();
        let mut energyconstrainedcapacity_array = Vec::new();
        let mut nonscheduledgeneration_array = Vec::new();
        let mut demand10_array = Vec::new();
        let mut demand50_array = Vec::new();
        let mut energyreqdemand10_array = Vec::new();
        let mut energyreqdemand50_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut demand10min_array = Vec::new();
        let mut demand10max_array = Vec::new();
        let mut demand50min_array = Vec::new();
        let mut demand50max_array = Vec::new();
        let mut carryovercapacity_array = Vec::new();
        for row in partition {
            publish_datetime_array.push(row.publish_datetime.timestamp());
            day_array.push(row.day.timestamp());
            regionid_array.push(row.regionid);
            pasaavailability_scheduled_array
                .push({
                    row.pasaavailability_scheduled
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            latest_offer_datetime_array
                .push(row.latest_offer_datetime.map(|val| val.timestamp()));
            energyunconstrainedcapacity_array
                .push({
                    row.energyunconstrainedcapacity
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            energyconstrainedcapacity_array
                .push({
                    row.energyconstrainedcapacity
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            nonscheduledgeneration_array
                .push({
                    row.nonscheduledgeneration
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            demand10_array
                .push({
                    row.demand10
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            demand50_array
                .push({
                    row.demand50
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            energyreqdemand10_array
                .push({
                    row.energyreqdemand10
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            energyreqdemand50_array
                .push({
                    row.energyreqdemand50
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            demand10min_array
                .push({
                    row.demand10min
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            demand10max_array
                .push({
                    row.demand10max
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            demand50min_array
                .push({
                    row.demand50min
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            demand50max_array
                .push({
                    row.demand50max
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            carryovercapacity_array
                .push({
                    row.carryovercapacity
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(publish_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(day_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(pasaavailability_scheduled_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(latest_offer_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(energyunconstrainedcapacity_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(energyconstrainedcapacity_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(nonscheduledgeneration_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demand10_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demand50_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(energyreqdemand10_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(energyreqdemand50_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demand10min_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demand10max_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demand50min_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demand50max_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(carryovercapacity_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MTPASA_REGIONAVAIL_TRK
///  _The tracking table to assist in versioning of the region-aggregate offered PASA Availability data published to the MTPASA_REGIONAVAILABILITY table._
///
/// * Data Set Name: Mtpasa
/// * File Name: Regionavailtrk
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * PUBLISH_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaRegionavailtrk1 {
    /// Date Time the report was published.
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub publish_datetime: chrono::NaiveDateTime,
    /// First date of the report inclusive.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Last date of the report inclusive.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Date Time of the latest offer used in the report.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub latest_offer_datetime: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MtpasaRegionavailtrk1 {
    type PrimaryKey = MtpasaRegionavailtrk1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("REGIONAVAILTRK".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MtpasaRegionavailtrk1PrimaryKey {
        MtpasaRegionavailtrk1PrimaryKey {
            publish_datetime: self.publish_datetime,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mtpasa_regionavailtrk_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MtpasaRegionavailtrk1PrimaryKey {
    pub publish_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MtpasaRegionavailtrk1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MtpasaRegionavailtrk1 {
    type Row = MtpasaRegionavailtrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.publish_datetime == row.publish_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaRegionavailtrk1 {
    type PrimaryKey = MtpasaRegionavailtrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.publish_datetime == key.publish_datetime
    }
}
impl mmsdm_core::CompareWithRow for MtpasaRegionavailtrk1PrimaryKey {
    type Row = MtpasaRegionavailtrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.publish_datetime == row.publish_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaRegionavailtrk1PrimaryKey {
    type PrimaryKey = MtpasaRegionavailtrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.publish_datetime == key.publish_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaRegionavailtrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("publish_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("startdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("enddate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("latest_offer_datetime",
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
        let mut publish_datetime_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut latest_offer_datetime_array = Vec::new();
        for row in partition {
            publish_datetime_array.push(row.publish_datetime.timestamp());
            startdate_array.push(row.startdate.map(|val| val.timestamp()));
            enddate_array.push(row.enddate.map(|val| val.timestamp()));
            latest_offer_datetime_array
                .push(row.latest_offer_datetime.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(publish_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(startdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(enddate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(latest_offer_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MTPASA_REGIONITERATION
///  _Region results for Unserved Energy (USE)_
///
/// * Data Set Name: Mtpasa
/// * File Name: Regioniteration
/// * Data Version: 1
///
/// # Description
///  MTPASA_REGIONITERATION is public data.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * AGGREGATION_PERIOD
/// * DEMAND_POE_TYPE
/// * PERIOD_ENDING
/// * REGIONID
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
/// * USE_ITERATION_ID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaRegioniteration1 {
    /// Date processing of the run begins.
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: String,
    /// Demand POE type used. Value is POE10 or POE50
    pub demand_poe_type: String,
    /// Period data is aggregated over. Values are YEAR
    pub aggregation_period: String,
    /// Datetime of day at end of period (i.e. last day of year reported)
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub period_ending: chrono::NaiveDateTime,
    /// The unique region identifier
    pub regionid: String,
    /// Iteration ID, only produced for iterations showing unserved energy&gt;0
    pub use_iteration_id: i64,
    /// Number of half hours showing unserved energy over year, for iteration
    pub use_iteration_event_number: Option<rust_decimal::Decimal>,
    /// Average unserved energy event size for iteration over year (MW)
    pub use_iteration_event_average: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MtpasaRegioniteration1 {
    type PrimaryKey = MtpasaRegioniteration1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("REGIONITERATION".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MtpasaRegioniteration1PrimaryKey {
        MtpasaRegioniteration1PrimaryKey {
            aggregation_period: self.aggregation_period.clone(),
            demand_poe_type: self.demand_poe_type.clone(),
            period_ending: self.period_ending,
            regionid: self.regionid.clone(),
            run_datetime: self.run_datetime,
            run_no: self.run_no,
            runtype: self.runtype.clone(),
            use_iteration_id: self.use_iteration_id,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mtpasa_regioniteration_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MtpasaRegioniteration1PrimaryKey {
    pub aggregation_period: String,
    pub demand_poe_type: String,
    pub period_ending: chrono::NaiveDateTime,
    pub regionid: String,
    pub run_datetime: chrono::NaiveDateTime,
    pub run_no: i64,
    pub runtype: String,
    pub use_iteration_id: i64,
}
impl mmsdm_core::PrimaryKey for MtpasaRegioniteration1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MtpasaRegioniteration1 {
    type Row = MtpasaRegioniteration1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.aggregation_period == row.aggregation_period
            && self.demand_poe_type == row.demand_poe_type
            && self.period_ending == row.period_ending && self.regionid == row.regionid
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype
            && self.use_iteration_id == row.use_iteration_id
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaRegioniteration1 {
    type PrimaryKey = MtpasaRegioniteration1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.aggregation_period == key.aggregation_period
            && self.demand_poe_type == key.demand_poe_type
            && self.period_ending == key.period_ending && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
            && self.use_iteration_id == key.use_iteration_id
    }
}
impl mmsdm_core::CompareWithRow for MtpasaRegioniteration1PrimaryKey {
    type Row = MtpasaRegioniteration1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.aggregation_period == row.aggregation_period
            && self.demand_poe_type == row.demand_poe_type
            && self.period_ending == row.period_ending && self.regionid == row.regionid
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype
            && self.use_iteration_id == row.use_iteration_id
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaRegioniteration1PrimaryKey {
    type PrimaryKey = MtpasaRegioniteration1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.aggregation_period == key.aggregation_period
            && self.demand_poe_type == key.demand_poe_type
            && self.period_ending == key.period_ending && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
            && self.use_iteration_id == key.use_iteration_id
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaRegioniteration1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("run_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("run_no",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("runtype",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("demand_poe_type",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("aggregation_period",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("period_ending",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("use_iteration_id",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("use_iteration_event_number",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_iteration_event_average",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
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
        let mut run_datetime_array = Vec::new();
        let mut run_no_array = Vec::new();
        let mut runtype_array = Vec::new();
        let mut demand_poe_type_array = Vec::new();
        let mut aggregation_period_array = Vec::new();
        let mut period_ending_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut use_iteration_id_array = Vec::new();
        let mut use_iteration_event_number_array = Vec::new();
        let mut use_iteration_event_average_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            run_datetime_array.push(row.run_datetime.timestamp());
            run_no_array.push(row.run_no);
            runtype_array.push(row.runtype);
            demand_poe_type_array.push(row.demand_poe_type);
            aggregation_period_array.push(row.aggregation_period);
            period_ending_array.push(row.period_ending.timestamp());
            regionid_array.push(row.regionid);
            use_iteration_id_array.push(row.use_iteration_id);
            use_iteration_event_number_array
                .push({
                    row.use_iteration_event_number
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_iteration_event_average_array
                .push({
                    row.use_iteration_event_average
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_no_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(runtype_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(demand_poe_type_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(aggregation_period_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(period_ending_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(use_iteration_id_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_iteration_event_number_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_iteration_event_average_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
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
/// ## MTPASA_REGIONRESULT
///  _Region results for interval of max demand per day._
///
/// * Data Set Name: Mtpasa
/// * File Name: Regionresult
/// * Data Version: 2
///
/// # Description
///  MTPASA_REGIONRESULT is public data.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DAY
/// * DEMAND_POE_TYPE
/// * REGIONID
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaRegionresult2 {
    /// Date processing of the run begins.
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: String,
    /// Demand POE type used. Value is POE10
    pub demand_poe_type: String,
    /// Day this result is for
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub day: chrono::NaiveDateTime,
    /// The unique region identifier
    pub regionid: String,
    /// Half hourly period reported, selected as period of maximum NEM scheduled demand (calculated as maximum of scheduled demands, averaged across iterations and reference years)
    pub periodid: Option<rust_decimal::Decimal>,
    /// Demand value from selected half hourly interval (MW)
    pub demand: Option<rust_decimal::Decimal>,
    /// The total installed capacity of all generation (MW)
    pub aggregateinstalledcapacity: Option<rust_decimal::Decimal>,
    /// Total number of iterations and reference years performed
    pub numberofiterations: Option<rust_decimal::Decimal>,
    /// Number of iterations and reference years with unserved energy&gt;0
    pub use_numberofiterations: Option<rust_decimal::Decimal>,
    /// Maximum unserved energy, across iterations and reference years (MW)
    pub use_max: Option<rust_decimal::Decimal>,
    /// Upper quartile unserved energy, across iterations and reference years (MW)
    pub use_upperquartile: Option<rust_decimal::Decimal>,
    /// Median unserved energy, across iterations and reference years (MW)
    pub use_median: Option<rust_decimal::Decimal>,
    /// Lower quartile unserved energy, across iterations and reference years (MW)
    pub use_lowerquartile: Option<rust_decimal::Decimal>,
    /// Minimum unserved energy, across iterations and reference years (MW)
    pub use_min: Option<rust_decimal::Decimal>,
    /// Average unserved energy, across iterations and reference years (MW)
    pub use_average: Option<rust_decimal::Decimal>,
    /// Average unserved energy event size, across iterations and reference years (MW)
    pub use_event_average: Option<rust_decimal::Decimal>,
    /// The 90th percentile for scheduled generation across iterations and reference years (MW)
    pub totalscheduledgen90: Option<rust_decimal::Decimal>,
    /// The 50th percentile for scheduled generation across iterations and reference years (MW)
    pub totalscheduledgen50: Option<rust_decimal::Decimal>,
    /// The 10th percentile for scheduled generation across iterations and reference years (MW)
    pub totalscheduledgen10: Option<rust_decimal::Decimal>,
    /// The 90th percentile for intermittent generation, across iterations and reference years (MW)
    pub totalintermittentgen90: Option<rust_decimal::Decimal>,
    /// The 50th percentile for intermittent generation, across iterations and reference years (MW)
    pub totalintermittentgen50: Option<rust_decimal::Decimal>,
    /// The 10th percentile for intermittent generation, across iterations and reference years (MW)
    pub totalintermittentgen10: Option<rust_decimal::Decimal>,
    /// The 90th percentile for demand side participation, across iterations and reference years (MW)
    pub demandsideparticipation90: Option<rust_decimal::Decimal>,
    /// The 50th percentile for demand side participation, across iterations and reference years (MW)
    pub demandsideparticipation50: Option<rust_decimal::Decimal>,
    /// The 10th percentile for demand side participation, across iterations and reference years (MW)
    pub demandsideparticipation10: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The 90% percentile for semi-scheduled generation across iterations and reference years (MW)
    pub totalsemischedulegen90: Option<rust_decimal::Decimal>,
    /// The 50% percentile for semi-scheduled generation across iterations and reference years (MW)
    pub totalsemischedulegen50: Option<rust_decimal::Decimal>,
    /// The 10% percentile for semi-scheduled generation across iterations and reference years (MW)
    pub totalsemischedulegen10: Option<rust_decimal::Decimal>,
    /// Minimum available capacity, across iterations and reference years (MW).
    pub totalavailablegenmin: Option<rust_decimal::Decimal>,
    /// The 10% percentile for available capacity, across iterations and reference years (MW).
    pub totalavailablegen10: Option<rust_decimal::Decimal>,
    /// The 50% percentile for available capacity, across iterations and reference years (MW).
    pub totalavailablegen50: Option<rust_decimal::Decimal>,
    /// The 90% percentile for available capacity, across iterations and reference years (MW).
    pub totalavailablegen90: Option<rust_decimal::Decimal>,
    /// Maximum available capacity, across iterations and reference years (MW).
    pub totalavailablegenmax: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for MtpasaRegionresult2 {
    type PrimaryKey = MtpasaRegionresult2PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("REGIONRESULT".into()),
            version: 2,
        }
    }
    fn primary_key(&self) -> MtpasaRegionresult2PrimaryKey {
        MtpasaRegionresult2PrimaryKey {
            day: self.day,
            demand_poe_type: self.demand_poe_type.clone(),
            regionid: self.regionid.clone(),
            run_datetime: self.run_datetime,
            run_no: self.run_no,
            runtype: self.runtype.clone(),
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mtpasa_regionresult_v2".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MtpasaRegionresult2PrimaryKey {
    pub day: chrono::NaiveDateTime,
    pub demand_poe_type: String,
    pub regionid: String,
    pub run_datetime: chrono::NaiveDateTime,
    pub run_no: i64,
    pub runtype: String,
}
impl mmsdm_core::PrimaryKey for MtpasaRegionresult2PrimaryKey {}
impl mmsdm_core::CompareWithRow for MtpasaRegionresult2 {
    type Row = MtpasaRegionresult2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.day == row.day && self.demand_poe_type == row.demand_poe_type
            && self.regionid == row.regionid && self.run_datetime == row.run_datetime
            && self.run_no == row.run_no && self.runtype == row.runtype
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaRegionresult2 {
    type PrimaryKey = MtpasaRegionresult2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.demand_poe_type == key.demand_poe_type
            && self.regionid == key.regionid && self.run_datetime == key.run_datetime
            && self.run_no == key.run_no && self.runtype == key.runtype
    }
}
impl mmsdm_core::CompareWithRow for MtpasaRegionresult2PrimaryKey {
    type Row = MtpasaRegionresult2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.day == row.day && self.demand_poe_type == row.demand_poe_type
            && self.regionid == row.regionid && self.run_datetime == row.run_datetime
            && self.run_no == row.run_no && self.runtype == row.runtype
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaRegionresult2PrimaryKey {
    type PrimaryKey = MtpasaRegionresult2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day == key.day && self.demand_poe_type == key.demand_poe_type
            && self.regionid == key.regionid && self.run_datetime == key.run_datetime
            && self.run_no == key.run_no && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaRegionresult2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("run_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("run_no",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("runtype",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("demand_poe_type",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("day",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("demand",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("aggregateinstalledcapacity",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("numberofiterations",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_numberofiterations",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_max",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_upperquartile",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_median",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_lowerquartile",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_min",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_average",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_event_average",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("totalscheduledgen90",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("totalscheduledgen50",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("totalscheduledgen10",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("totalintermittentgen90",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("totalintermittentgen50",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("totalintermittentgen10",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("demandsideparticipation90",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("demandsideparticipation50",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("demandsideparticipation10",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("totalsemischedulegen90",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("totalsemischedulegen50",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("totalsemischedulegen10",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("totalavailablegenmin",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("totalavailablegen10",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("totalavailablegen50",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("totalavailablegen90",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("totalavailablegenmax",
                arrow2::datatypes::DataType::Decimal(12, 2), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut run_datetime_array = Vec::new();
        let mut run_no_array = Vec::new();
        let mut runtype_array = Vec::new();
        let mut demand_poe_type_array = Vec::new();
        let mut day_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut demand_array = Vec::new();
        let mut aggregateinstalledcapacity_array = Vec::new();
        let mut numberofiterations_array = Vec::new();
        let mut use_numberofiterations_array = Vec::new();
        let mut use_max_array = Vec::new();
        let mut use_upperquartile_array = Vec::new();
        let mut use_median_array = Vec::new();
        let mut use_lowerquartile_array = Vec::new();
        let mut use_min_array = Vec::new();
        let mut use_average_array = Vec::new();
        let mut use_event_average_array = Vec::new();
        let mut totalscheduledgen90_array = Vec::new();
        let mut totalscheduledgen50_array = Vec::new();
        let mut totalscheduledgen10_array = Vec::new();
        let mut totalintermittentgen90_array = Vec::new();
        let mut totalintermittentgen50_array = Vec::new();
        let mut totalintermittentgen10_array = Vec::new();
        let mut demandsideparticipation90_array = Vec::new();
        let mut demandsideparticipation50_array = Vec::new();
        let mut demandsideparticipation10_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut totalsemischedulegen90_array = Vec::new();
        let mut totalsemischedulegen50_array = Vec::new();
        let mut totalsemischedulegen10_array = Vec::new();
        let mut totalavailablegenmin_array = Vec::new();
        let mut totalavailablegen10_array = Vec::new();
        let mut totalavailablegen50_array = Vec::new();
        let mut totalavailablegen90_array = Vec::new();
        let mut totalavailablegenmax_array = Vec::new();
        for row in partition {
            run_datetime_array.push(row.run_datetime.timestamp());
            run_no_array.push(row.run_no);
            runtype_array.push(row.runtype);
            demand_poe_type_array.push(row.demand_poe_type);
            day_array.push(row.day.timestamp());
            regionid_array.push(row.regionid);
            periodid_array
                .push({
                    row.periodid
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            demand_array
                .push({
                    row.demand
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            aggregateinstalledcapacity_array
                .push({
                    row.aggregateinstalledcapacity
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            numberofiterations_array
                .push({
                    row.numberofiterations
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_numberofiterations_array
                .push({
                    row.use_numberofiterations
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_max_array
                .push({
                    row.use_max
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_upperquartile_array
                .push({
                    row.use_upperquartile
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_median_array
                .push({
                    row.use_median
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_lowerquartile_array
                .push({
                    row.use_lowerquartile
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_min_array
                .push({
                    row.use_min
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_average_array
                .push({
                    row.use_average
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_event_average_array
                .push({
                    row.use_event_average
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            totalscheduledgen90_array
                .push({
                    row.totalscheduledgen90
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            totalscheduledgen50_array
                .push({
                    row.totalscheduledgen50
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            totalscheduledgen10_array
                .push({
                    row.totalscheduledgen10
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            totalintermittentgen90_array
                .push({
                    row.totalintermittentgen90
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            totalintermittentgen50_array
                .push({
                    row.totalintermittentgen50
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            totalintermittentgen10_array
                .push({
                    row.totalintermittentgen10
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            demandsideparticipation90_array
                .push({
                    row.demandsideparticipation90
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            demandsideparticipation50_array
                .push({
                    row.demandsideparticipation50
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            demandsideparticipation10_array
                .push({
                    row.demandsideparticipation10
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            totalsemischedulegen90_array
                .push({
                    row.totalsemischedulegen90
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            totalsemischedulegen50_array
                .push({
                    row.totalsemischedulegen50
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            totalsemischedulegen10_array
                .push({
                    row.totalsemischedulegen10
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            totalavailablegenmin_array
                .push({
                    row.totalavailablegenmin
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            totalavailablegen10_array
                .push({
                    row.totalavailablegen10
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            totalavailablegen50_array
                .push({
                    row.totalavailablegen50
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            totalavailablegen90_array
                .push({
                    row.totalavailablegen90
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            totalavailablegenmax_array
                .push({
                    row.totalavailablegenmax
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_no_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(runtype_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(demand_poe_type_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(day_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demand_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(aggregateinstalledcapacity_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(numberofiterations_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_numberofiterations_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_max_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_upperquartile_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_median_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_lowerquartile_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_min_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_average_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_event_average_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalscheduledgen90_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalscheduledgen50_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalscheduledgen10_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalintermittentgen90_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalintermittentgen50_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalintermittentgen10_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demandsideparticipation90_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demandsideparticipation50_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demandsideparticipation10_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalsemischedulegen90_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalsemischedulegen50_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalsemischedulegen10_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalavailablegenmin_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalavailablegen10_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalavailablegen50_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalavailablegen90_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalavailablegenmax_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MTPASA_REGIONSUMMARY
///  _Region Results summary over aggregation periods._
///
/// * Data Set Name: Mtpasa
/// * File Name: Regionsummary
/// * Data Version: 1
///
/// # Description
///  MTPASA_REGIONSUMMARY is public data.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * AGGREGATION_PERIOD
/// * DEMAND_POE_TYPE
/// * PERIOD_ENDING
/// * REGIONID
/// * RUN_DATETIME
/// * RUN_NO
/// * RUNTYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaRegionsummary1 {
    /// Date processing of the run begins.
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Unique run id.
    pub run_no: i64,
    /// Type of run. Always RELIABILITY
    pub runtype: String,
    /// Demand POE type used. Value are POE10, POE50
    pub demand_poe_type: String,
    /// Period data is aggregated over. Values are YEAR, MONTH
    pub aggregation_period: String,
    /// Datetime of day at end of period (i.e. last day of month or year reported)
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub period_ending: chrono::NaiveDateTime,
    /// The unique region identifier
    pub regionid: String,
    /// Native demand calculated from Operational As Generated trace supplied by Energy Forecasting
    pub nativedemand: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 10th percentile of iterations and reference years (MWh)
    pub use_percentile10: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 20th percentile of iterations and reference years (MWh)
    pub use_percentile20: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 30th percentile of iterations and reference years (MWh)
    pub use_percentile30: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 40th percentile of iterations and reference years (MWh)
    pub use_percentile40: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 50th percentile of iterations and reference years (MWh)
    pub use_percentile50: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 60th percentile of iterations and reference years (MWh)
    pub use_percentile60: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 70th percentile of iterations and reference years (MWh)
    pub use_percentile70: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 80th percentile of iterations and reference years (MWh)
    pub use_percentile80: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 90th percentile of iterations and reference years (MWh)
    pub use_percentile90: Option<rust_decimal::Decimal>,
    /// Unserved energy period amount at the 100th percentile of iterations and reference years (MWh)
    pub use_percentile100: Option<rust_decimal::Decimal>,
    /// Average period unserved energy across iterations and reference years (MWh)
    pub use_average: Option<rust_decimal::Decimal>,
    /// Total number of iterations and reference years performed
    pub numberofiterations: Option<rust_decimal::Decimal>,
    /// Number of iterations and reference years showing unserved energy
    pub use_numberofiterations: Option<rust_decimal::Decimal>,
    /// Maximum unserved energy event size across all half hourly intervals and iterations and reference years that have unserved energy&gt;0 (MW)
    pub use_event_max: Option<rust_decimal::Decimal>,
    /// Upper quartile unserved energy event size across all half hourly intervals and iterations and reference years that have unserved energy&gt;0 (MW)
    pub use_event_upperquartile: Option<rust_decimal::Decimal>,
    /// Median unserved energy event size across all half hourly intervals and iterations and reference years that have unserved energy&gt;0 (MW)
    pub use_event_median: Option<rust_decimal::Decimal>,
    /// Lower quartile unserved energy event size across all half hourly intervals and iterations and reference years that have unserved energy&gt;0 (MW)
    pub use_event_lowerquartile: Option<rust_decimal::Decimal>,
    /// Minimum unserved energy event size across all half hourly intervals and iterations and reference years that have unserved energy&gt;0 (MW)
    pub use_event_min: Option<rust_decimal::Decimal>,
    /// Fixed Values of 0.696 for 50 POE and 0.304 for 10 POE.
    pub weight: Option<rust_decimal::Decimal>,
    /// Weighted average USE per region = (USE_AVERAGE_POE10/NATIVE_DEMAND_POE_10*WEIGHT_POE_10 + USE_AVERAGE_POE50/NATIVE_DEMAND_POE_50*WEIGHT_POE_50)*100
    pub use_weighted_avg: Option<rust_decimal::Decimal>,
    /// LRC Condition reported (Value=1) if USE_WEIGHTED_AVG &gt;= 0.002% otherwise (Value=0)
    pub lrc: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MtpasaRegionsummary1 {
    type PrimaryKey = MtpasaRegionsummary1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("REGIONSUMMARY".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MtpasaRegionsummary1PrimaryKey {
        MtpasaRegionsummary1PrimaryKey {
            aggregation_period: self.aggregation_period.clone(),
            demand_poe_type: self.demand_poe_type.clone(),
            period_ending: self.period_ending,
            regionid: self.regionid.clone(),
            run_datetime: self.run_datetime,
            run_no: self.run_no,
            runtype: self.runtype.clone(),
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mtpasa_regionsummary_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MtpasaRegionsummary1PrimaryKey {
    pub aggregation_period: String,
    pub demand_poe_type: String,
    pub period_ending: chrono::NaiveDateTime,
    pub regionid: String,
    pub run_datetime: chrono::NaiveDateTime,
    pub run_no: i64,
    pub runtype: String,
}
impl mmsdm_core::PrimaryKey for MtpasaRegionsummary1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MtpasaRegionsummary1 {
    type Row = MtpasaRegionsummary1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.aggregation_period == row.aggregation_period
            && self.demand_poe_type == row.demand_poe_type
            && self.period_ending == row.period_ending && self.regionid == row.regionid
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaRegionsummary1 {
    type PrimaryKey = MtpasaRegionsummary1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.aggregation_period == key.aggregation_period
            && self.demand_poe_type == key.demand_poe_type
            && self.period_ending == key.period_ending && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
    }
}
impl mmsdm_core::CompareWithRow for MtpasaRegionsummary1PrimaryKey {
    type Row = MtpasaRegionsummary1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.aggregation_period == row.aggregation_period
            && self.demand_poe_type == row.demand_poe_type
            && self.period_ending == row.period_ending && self.regionid == row.regionid
            && self.run_datetime == row.run_datetime && self.run_no == row.run_no
            && self.runtype == row.runtype
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaRegionsummary1PrimaryKey {
    type PrimaryKey = MtpasaRegionsummary1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.aggregation_period == key.aggregation_period
            && self.demand_poe_type == key.demand_poe_type
            && self.period_ending == key.period_ending && self.regionid == key.regionid
            && self.run_datetime == key.run_datetime && self.run_no == key.run_no
            && self.runtype == key.runtype
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaRegionsummary1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("run_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("run_no",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("runtype",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("demand_poe_type",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("aggregation_period",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("period_ending",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("nativedemand",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_percentile10",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_percentile20",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_percentile30",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_percentile40",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_percentile50",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_percentile60",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_percentile70",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_percentile80",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_percentile90",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_percentile100",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_average",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("numberofiterations",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_numberofiterations",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_event_max",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_event_upperquartile",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_event_median",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_event_lowerquartile",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("use_event_min",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
                arrow2::datatypes::Field::new("weight",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("use_weighted_avg",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("lrc",
                arrow2::datatypes::DataType::Decimal(12, 2), true),
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
        let mut run_datetime_array = Vec::new();
        let mut run_no_array = Vec::new();
        let mut runtype_array = Vec::new();
        let mut demand_poe_type_array = Vec::new();
        let mut aggregation_period_array = Vec::new();
        let mut period_ending_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut nativedemand_array = Vec::new();
        let mut use_percentile10_array = Vec::new();
        let mut use_percentile20_array = Vec::new();
        let mut use_percentile30_array = Vec::new();
        let mut use_percentile40_array = Vec::new();
        let mut use_percentile50_array = Vec::new();
        let mut use_percentile60_array = Vec::new();
        let mut use_percentile70_array = Vec::new();
        let mut use_percentile80_array = Vec::new();
        let mut use_percentile90_array = Vec::new();
        let mut use_percentile100_array = Vec::new();
        let mut use_average_array = Vec::new();
        let mut numberofiterations_array = Vec::new();
        let mut use_numberofiterations_array = Vec::new();
        let mut use_event_max_array = Vec::new();
        let mut use_event_upperquartile_array = Vec::new();
        let mut use_event_median_array = Vec::new();
        let mut use_event_lowerquartile_array = Vec::new();
        let mut use_event_min_array = Vec::new();
        let mut weight_array = Vec::new();
        let mut use_weighted_avg_array = Vec::new();
        let mut lrc_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            run_datetime_array.push(row.run_datetime.timestamp());
            run_no_array.push(row.run_no);
            runtype_array.push(row.runtype);
            demand_poe_type_array.push(row.demand_poe_type);
            aggregation_period_array.push(row.aggregation_period);
            period_ending_array.push(row.period_ending.timestamp());
            regionid_array.push(row.regionid);
            nativedemand_array
                .push({
                    row.nativedemand
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_percentile10_array
                .push({
                    row.use_percentile10
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_percentile20_array
                .push({
                    row.use_percentile20
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_percentile30_array
                .push({
                    row.use_percentile30
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_percentile40_array
                .push({
                    row.use_percentile40
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_percentile50_array
                .push({
                    row.use_percentile50
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_percentile60_array
                .push({
                    row.use_percentile60
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_percentile70_array
                .push({
                    row.use_percentile70
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_percentile80_array
                .push({
                    row.use_percentile80
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_percentile90_array
                .push({
                    row.use_percentile90
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_percentile100_array
                .push({
                    row.use_percentile100
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_average_array
                .push({
                    row.use_average
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            numberofiterations_array
                .push({
                    row.numberofiterations
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_numberofiterations_array
                .push({
                    row.use_numberofiterations
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_event_max_array
                .push({
                    row.use_event_max
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_event_upperquartile_array
                .push({
                    row.use_event_upperquartile
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_event_median_array
                .push({
                    row.use_event_median
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_event_lowerquartile_array
                .push({
                    row.use_event_lowerquartile
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            use_event_min_array
                .push({
                    row.use_event_min
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            weight_array
                .push({
                    row.weight
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            use_weighted_avg_array
                .push({
                    row.use_weighted_avg
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            lrc_array
                .push({
                    row.lrc
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_no_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(runtype_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(demand_poe_type_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(aggregation_period_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(period_ending_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(nativedemand_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_percentile10_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_percentile20_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_percentile30_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_percentile40_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_percentile50_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_percentile60_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_percentile70_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_percentile80_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_percentile90_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_percentile100_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_average_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(numberofiterations_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_numberofiterations_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_event_max_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_event_upperquartile_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_event_median_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_event_lowerquartile_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_event_min_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(weight_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(use_weighted_avg_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lrc_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
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
        (Some("CASERESULT"), version) if version <= 1_i32 => {
            let d: Vec<MtpasaCaseresult1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMtpasaCaseresult1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("CONSTRAINTRESULT"), version) if version <= 1_i32 => {
            let d: Vec<MtpasaConstraintresult1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMtpasaConstraintresult1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("CONSTRAINTSUMMARY"), version) if version <= 1_i32 => {
            let d: Vec<MtpasaConstraintsummary1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMtpasaConstraintsummary1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("DUIDAVAILABILITY"), version) if version <= 2_i32 => {
            let d: Vec<MtpasaDuidavailability2> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMtpasaDuidavailability2 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("INTERCONNECTORRESULT"), version) if version <= 1_i32 => {
            let d: Vec<MtpasaInterconnectorresult1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMtpasaInterconnectorresult1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("LOLPRESULT"), version) if version <= 1_i32 => {
            let d: Vec<MtpasaLolpresult1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMtpasaLolpresult1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("REGIONAVAILABILITY"), version) if version <= 4_i32 => {
            let d: Vec<MtpasaRegionavailability4> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMtpasaRegionavailability4 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("REGIONAVAILTRK"), version) if version <= 1_i32 => {
            let d: Vec<MtpasaRegionavailtrk1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMtpasaRegionavailtrk1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("REGIONITERATION"), version) if version <= 1_i32 => {
            let d: Vec<MtpasaRegioniteration1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMtpasaRegioniteration1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("REGIONRESULT"), version) if version <= 2_i32 => {
            let d: Vec<MtpasaRegionresult2> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMtpasaRegionresult2 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("REGIONSUMMARY"), version) if version <= 1_i32 => {
            let d: Vec<MtpasaRegionsummary1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMtpasaRegionsummary1 @P1, @P2",
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
