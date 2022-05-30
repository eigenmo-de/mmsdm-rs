/// # Summary
/// 
/// ## PDPASA_CASESOLUTION
///  _The top-level table identifying a PDPASA case, reporting options applied in the case and summary results_
/// 
/// * Data Set Name: Pdpasa
/// * File Name: Casesolution
/// * Data Version: 3
/// 
/// # Description
///  PDPASA_CASESOLUTION is public data. Source PDPASA_CASESOLUTION is updated each PDPASA run (i.e. half-hourly). Volume Rows per day: 48 Mb per month: &lt;1
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PdpasaCasesolution3 {
    /// Case identifier by the time the case was run
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Version of the PASA solver used to solve this case
    pub pasaversion: Option<String>,
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
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Specifies the Probability of Exceedence (POE) demand forecast for Reliability LRC assessment (0 if no assessment, 10 for 10%, 50 for 50%, 90 for 90%)
    pub reliabilitylrcdemandoption: Option<rust_decimal::Decimal>,
    /// Specifies the Probability of Exceedence (POE) demand forecast for outage LRC assessment (0 if no assessment, 10 for 10%, 50 for 50%, 90 for 90%)
    pub outagelrcdemandoption: Option<rust_decimal::Decimal>,
    /// Specifies the Probability of Exceedence (POE) demand forecast for LOR assessment (0 if no assessment, 10 for 10%, 50 for 50%, 90 for 90%)
    pub lordemandoption: Option<rust_decimal::Decimal>,
    /// Generation Availability to be used in Reliability LRC run (either PASA or MARKET)
    pub reliabilitylrccapacityoption: Option<String>,
    /// Generation Availability to be used in Outage LRC run (either PASA or MARKET)
    pub outagelrccapacityoption: Option<String>,
    /// Generation Availability to be used in LOR run (either PASA or MARKET)
    pub lorcapacityoption: Option<String>,
    /// UIGF POE forecast availability used for this option
    pub loruigf_option: Option<rust_decimal::Decimal>,
    /// UIGF POE forecast availability used for this option
    pub reliability_lrcuigf_option: Option<rust_decimal::Decimal>,
    /// UIGF POE forecast availability used for this option
    pub outage_lrcuigf_option: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for PdpasaCasesolution3 {
    type PrimaryKey = PdpasaCasesolution3PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PDPASA".into(),
            table_name: Some("CASESOLUTION".into()),
            version: 3,
        }
    }

    fn primary_key(&self) -> PdpasaCasesolution3PrimaryKey {
        PdpasaCasesolution3PrimaryKey {
            run_datetime: self.run_datetime
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "pdpasa_casesolution_v3".to_string()
    }
}
impl mmsdm_core::CompareWithRow for PdpasaCasesolution3 {
    type Row = PdpasaCasesolution3;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PdpasaCasesolution3 {
    type PrimaryKey = PdpasaCasesolution3PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PdpasaCasesolution3PrimaryKey {
    pub run_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::CompareWithRow for PdpasaCasesolution3PrimaryKey {
    type Row = PdpasaCasesolution3;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.run_datetime == row.run_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PdpasaCasesolution3PrimaryKey {
    type PrimaryKey = PdpasaCasesolution3PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime
    }
}
impl mmsdm_core::PrimaryKey for PdpasaCasesolution3PrimaryKey {
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PdpasaCasesolution3 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("run_datetime", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("pasaversion", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("reservecondition", arrow2::datatypes::DataType::Decimal(1,0), true),
            arrow2::datatypes::Field::new("lorcondition", arrow2::datatypes::DataType::Decimal(1,0), true),
            arrow2::datatypes::Field::new("capacityobjfunction", arrow2::datatypes::DataType::Decimal(12,3), true),
            arrow2::datatypes::Field::new("capacityoption", arrow2::datatypes::DataType::Decimal(12,3), true),
            arrow2::datatypes::Field::new("maxsurplusreserveoption", arrow2::datatypes::DataType::Decimal(12,3), true),
            arrow2::datatypes::Field::new("maxsparecapacityoption", arrow2::datatypes::DataType::Decimal(12,3), true),
            arrow2::datatypes::Field::new("interconnectorflowpenalty", arrow2::datatypes::DataType::Decimal(12,3), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("reliabilitylrcdemandoption", arrow2::datatypes::DataType::Decimal(12,3), true),
            arrow2::datatypes::Field::new("outagelrcdemandoption", arrow2::datatypes::DataType::Decimal(12,3), true),
            arrow2::datatypes::Field::new("lordemandoption", arrow2::datatypes::DataType::Decimal(12,3), true),
            arrow2::datatypes::Field::new("reliabilitylrccapacityoption", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("outagelrccapacityoption", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lorcapacityoption", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("loruigf_option", arrow2::datatypes::DataType::Decimal(3,0), true),
            arrow2::datatypes::Field::new("reliability_lrcuigf_option", arrow2::datatypes::DataType::Decimal(3,0), true),
            arrow2::datatypes::Field::new("outage_lrcuigf_option", arrow2::datatypes::DataType::Decimal(3,0), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut run_datetime_array = Vec::new();
        let mut pasaversion_array = Vec::new();
        let mut reservecondition_array = Vec::new();
        let mut lorcondition_array = Vec::new();
        let mut capacityobjfunction_array = Vec::new();
        let mut capacityoption_array = Vec::new();
        let mut maxsurplusreserveoption_array = Vec::new();
        let mut maxsparecapacityoption_array = Vec::new();
        let mut interconnectorflowpenalty_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut reliabilitylrcdemandoption_array = Vec::new();
        let mut outagelrcdemandoption_array = Vec::new();
        let mut lordemandoption_array = Vec::new();
        let mut reliabilitylrccapacityoption_array = Vec::new();
        let mut outagelrccapacityoption_array = Vec::new();
        let mut lorcapacityoption_array = Vec::new();
        let mut loruigf_option_array = Vec::new();
        let mut reliability_lrcuigf_option_array = Vec::new();
        let mut outage_lrcuigf_option_array = Vec::new();
        for row in partition {
            run_datetime_array.push(row.run_datetime.timestamp());
            pasaversion_array.push(row.pasaversion);
            reservecondition_array.push({
                        row.reservecondition.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            lorcondition_array.push({
                        row.lorcondition.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            capacityobjfunction_array.push({
                        row.capacityobjfunction.map(|mut val| {
                            val.rescale(3);
                            val.mantissa()
                        })
                    });
            capacityoption_array.push({
                        row.capacityoption.map(|mut val| {
                            val.rescale(3);
                            val.mantissa()
                        })
                    });
            maxsurplusreserveoption_array.push({
                        row.maxsurplusreserveoption.map(|mut val| {
                            val.rescale(3);
                            val.mantissa()
                        })
                    });
            maxsparecapacityoption_array.push({
                        row.maxsparecapacityoption.map(|mut val| {
                            val.rescale(3);
                            val.mantissa()
                        })
                    });
            interconnectorflowpenalty_array.push({
                        row.interconnectorflowpenalty.map(|mut val| {
                            val.rescale(3);
                            val.mantissa()
                        })
                    });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            reliabilitylrcdemandoption_array.push({
                        row.reliabilitylrcdemandoption.map(|mut val| {
                            val.rescale(3);
                            val.mantissa()
                        })
                    });
            outagelrcdemandoption_array.push({
                        row.outagelrcdemandoption.map(|mut val| {
                            val.rescale(3);
                            val.mantissa()
                        })
                    });
            lordemandoption_array.push({
                        row.lordemandoption.map(|mut val| {
                            val.rescale(3);
                            val.mantissa()
                        })
                    });
            reliabilitylrccapacityoption_array.push(row.reliabilitylrccapacityoption);
            outagelrccapacityoption_array.push(row.outagelrccapacityoption);
            lorcapacityoption_array.push(row.lorcapacityoption);
            loruigf_option_array.push({
                        row.loruigf_option.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            reliability_lrcuigf_option_array.push({
                        row.reliability_lrcuigf_option.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            outage_lrcuigf_option_array.push({
                        row.outage_lrcuigf_option.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_datetime_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(pasaversion_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(reservecondition_array).to(arrow2::datatypes::DataType::Decimal(1,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lorcondition_array).to(arrow2::datatypes::DataType::Decimal(1,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capacityobjfunction_array).to(arrow2::datatypes::DataType::Decimal(12,3))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capacityoption_array).to(arrow2::datatypes::DataType::Decimal(12,3))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxsurplusreserveoption_array).to(arrow2::datatypes::DataType::Decimal(12,3))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxsparecapacityoption_array).to(arrow2::datatypes::DataType::Decimal(12,3))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(interconnectorflowpenalty_array).to(arrow2::datatypes::DataType::Decimal(12,3))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(reliabilitylrcdemandoption_array).to(arrow2::datatypes::DataType::Decimal(12,3))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(outagelrcdemandoption_array).to(arrow2::datatypes::DataType::Decimal(12,3))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lordemandoption_array).to(arrow2::datatypes::DataType::Decimal(12,3))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(reliabilitylrccapacityoption_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(outagelrccapacityoption_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(lorcapacityoption_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(loruigf_option_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(reliability_lrcuigf_option_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(outage_lrcuigf_option_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## PDPASA_CONSTRAINTSOLUTION
///  _PDPASA_CONSTRAINTSOLUTION shows binding and violated constraint results from the capacity evaluation, including the RHS value._
/// 
/// * Data Set Name: Pdpasa
/// * File Name: Constraintsolution
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
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
/// * RUNTYPE
/// * STUDYREGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PdpasaConstraintsolution1 {
    /// Unique Timestamp Identifier for this study
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// The unique identifier for the interval within this study
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Constraint identifier (synonymous with GenConID)
    pub constraintid: String,
    /// The RHS value in the capacity evaluation.
    pub capacityrhs: Option<rust_decimal::Decimal>,
    /// Capacity adequacy assessment marginal value, 0 if not binding
    pub capacitymarginalvalue: Option<rust_decimal::Decimal>,
    /// Capacity adequacy assessment violation degree for generic constraint; 0 if not violating
    pub capacityviolationdegree: Option<rust_decimal::Decimal>,
    /// Last changed date of this record
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Type of run. Values are RELIABILITY_LRC, OUTAGE_LRC and LOR.
    pub runtype: String,
    /// Primary Region for LP Solve (or MARKET if none).
    pub studyregionid: String,
}
impl mmsdm_core::GetTable for PdpasaConstraintsolution1 {
    type PrimaryKey = PdpasaConstraintsolution1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PDPASA".into(),
            table_name: Some("CONSTRAINTSOLUTION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> PdpasaConstraintsolution1PrimaryKey {
        PdpasaConstraintsolution1PrimaryKey {
            constraintid: self.constraintid.clone(),
            interval_datetime: self.interval_datetime,
            run_datetime: self.run_datetime,
            runtype: self.runtype.clone(),
            studyregionid: self.studyregionid.clone()
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "pdpasa_constraintsolution_v1".to_string()
    }
}
impl mmsdm_core::CompareWithRow for PdpasaConstraintsolution1 {
    type Row = PdpasaConstraintsolution1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid
        && self.interval_datetime == row.interval_datetime
        && self.run_datetime == row.run_datetime
        && self.runtype == row.runtype
        && self.studyregionid == row.studyregionid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PdpasaConstraintsolution1 {
    type PrimaryKey = PdpasaConstraintsolution1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
        && self.interval_datetime == key.interval_datetime
        && self.run_datetime == key.run_datetime
        && self.runtype == key.runtype
        && self.studyregionid == key.studyregionid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PdpasaConstraintsolution1PrimaryKey {
    pub constraintid: String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
    pub runtype: String,
    pub studyregionid: String,
}
impl mmsdm_core::CompareWithRow for PdpasaConstraintsolution1PrimaryKey {
    type Row = PdpasaConstraintsolution1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid
        && self.interval_datetime == row.interval_datetime
        && self.run_datetime == row.run_datetime
        && self.runtype == row.runtype
        && self.studyregionid == row.studyregionid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PdpasaConstraintsolution1PrimaryKey {
    type PrimaryKey = PdpasaConstraintsolution1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
        && self.interval_datetime == key.interval_datetime
        && self.run_datetime == key.run_datetime
        && self.runtype == key.runtype
        && self.studyregionid == key.studyregionid
    }
}
impl mmsdm_core::PrimaryKey for PdpasaConstraintsolution1PrimaryKey {
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PdpasaConstraintsolution1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("run_datetime", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("interval_datetime", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("constraintid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("capacityrhs", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("capacitymarginalvalue", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("capacityviolationdegree", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("runtype", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("studyregionid", arrow2::datatypes::DataType::LargeUtf8, false)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut run_datetime_array = Vec::new();
        let mut interval_datetime_array = Vec::new();
        let mut constraintid_array = Vec::new();
        let mut capacityrhs_array = Vec::new();
        let mut capacitymarginalvalue_array = Vec::new();
        let mut capacityviolationdegree_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut runtype_array = Vec::new();
        let mut studyregionid_array = Vec::new();
        for row in partition {
            run_datetime_array.push(row.run_datetime.timestamp());
            interval_datetime_array.push(row.interval_datetime.timestamp());
            constraintid_array.push(row.constraintid);
            capacityrhs_array.push({
                        row.capacityrhs.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            capacitymarginalvalue_array.push({
                        row.capacitymarginalvalue.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            capacityviolationdegree_array.push({
                        row.capacityviolationdegree.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            runtype_array.push(row.runtype);
            studyregionid_array.push(row.studyregionid);
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_datetime_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(interval_datetime_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(constraintid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capacityrhs_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capacitymarginalvalue_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capacityviolationdegree_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(runtype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(studyregionid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## PDPASA_INTERCONNECTORSOLN
///  _PDPASA_INTERCONNECTORSOLN shows the results of the capacity evaluation for Interconnectors, including the calculated limits for the interval._
/// 
/// * Data Set Name: Pdpasa
/// * File Name: Interconnectorsoln
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
/// * RUNTYPE
/// * STUDYREGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PdpasaInterconnectorsoln1 {
    /// Unique Timestamp Identifier for this study
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// The unique identifier for the interval within this study
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Interconnector Identifier
    pub interconnectorid: String,
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
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Type of run. Values are RELIABILITY_LRC, OUTAGE_LRC and LOR.
    pub runtype: String,
    /// ID of the constraint that sets the Interconnector Export Limit
    pub exportlimitconstraintid: Option<String>,
    /// ID of the constraint that sets the Interconnector Import Limit
    pub importlimitconstraintid: Option<String>,
    /// Primary Region for LP Solve (or MARKET if none).
    pub studyregionid: String,
}
impl mmsdm_core::GetTable for PdpasaInterconnectorsoln1 {
    type PrimaryKey = PdpasaInterconnectorsoln1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PDPASA".into(),
            table_name: Some("INTERCONNECTORSOLN".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> PdpasaInterconnectorsoln1PrimaryKey {
        PdpasaInterconnectorsoln1PrimaryKey {
            interconnectorid: self.interconnectorid.clone(),
            interval_datetime: self.interval_datetime,
            run_datetime: self.run_datetime,
            runtype: self.runtype.clone(),
            studyregionid: self.studyregionid.clone()
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "pdpasa_interconnectorsoln_v1".to_string()
    }
}
impl mmsdm_core::CompareWithRow for PdpasaInterconnectorsoln1 {
    type Row = PdpasaInterconnectorsoln1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interconnectorid == row.interconnectorid
        && self.interval_datetime == row.interval_datetime
        && self.run_datetime == row.run_datetime
        && self.runtype == row.runtype
        && self.studyregionid == row.studyregionid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PdpasaInterconnectorsoln1 {
    type PrimaryKey = PdpasaInterconnectorsoln1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
        && self.interval_datetime == key.interval_datetime
        && self.run_datetime == key.run_datetime
        && self.runtype == key.runtype
        && self.studyregionid == key.studyregionid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PdpasaInterconnectorsoln1PrimaryKey {
    pub interconnectorid: String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
    pub runtype: String,
    pub studyregionid: String,
}
impl mmsdm_core::CompareWithRow for PdpasaInterconnectorsoln1PrimaryKey {
    type Row = PdpasaInterconnectorsoln1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interconnectorid == row.interconnectorid
        && self.interval_datetime == row.interval_datetime
        && self.run_datetime == row.run_datetime
        && self.runtype == row.runtype
        && self.studyregionid == row.studyregionid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PdpasaInterconnectorsoln1PrimaryKey {
    type PrimaryKey = PdpasaInterconnectorsoln1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
        && self.interval_datetime == key.interval_datetime
        && self.run_datetime == key.run_datetime
        && self.runtype == key.runtype
        && self.studyregionid == key.studyregionid
    }
}
impl mmsdm_core::PrimaryKey for PdpasaInterconnectorsoln1PrimaryKey {
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PdpasaInterconnectorsoln1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("run_datetime", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("interval_datetime", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("interconnectorid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("capacitymwflow", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("capacitymarginalvalue", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("capacityviolationdegree", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("calculatedexportlimit", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("calculatedimportlimit", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("runtype", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("exportlimitconstraintid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("importlimitconstraintid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("studyregionid", arrow2::datatypes::DataType::LargeUtf8, false)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut run_datetime_array = Vec::new();
        let mut interval_datetime_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut capacitymwflow_array = Vec::new();
        let mut capacitymarginalvalue_array = Vec::new();
        let mut capacityviolationdegree_array = Vec::new();
        let mut calculatedexportlimit_array = Vec::new();
        let mut calculatedimportlimit_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut runtype_array = Vec::new();
        let mut exportlimitconstraintid_array = Vec::new();
        let mut importlimitconstraintid_array = Vec::new();
        let mut studyregionid_array = Vec::new();
        for row in partition {
            run_datetime_array.push(row.run_datetime.timestamp());
            interval_datetime_array.push(row.interval_datetime.timestamp());
            interconnectorid_array.push(row.interconnectorid);
            capacitymwflow_array.push({
                        row.capacitymwflow.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            capacitymarginalvalue_array.push({
                        row.capacitymarginalvalue.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            capacityviolationdegree_array.push({
                        row.capacityviolationdegree.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            calculatedexportlimit_array.push({
                        row.calculatedexportlimit.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            calculatedimportlimit_array.push({
                        row.calculatedimportlimit.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            runtype_array.push(row.runtype);
            exportlimitconstraintid_array.push(row.exportlimitconstraintid);
            importlimitconstraintid_array.push(row.importlimitconstraintid);
            studyregionid_array.push(row.studyregionid);
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_datetime_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(interval_datetime_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(interconnectorid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capacitymwflow_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capacitymarginalvalue_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capacityviolationdegree_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(calculatedexportlimit_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(calculatedimportlimit_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(runtype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(exportlimitconstraintid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(importlimitconstraintid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(studyregionid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## PDPASA_REGIONSOLUTION
///  _The PDPASA region solution data_
/// 
/// * Data Set Name: Pdpasa
/// * File Name: Regionsolution
/// * Data Version: 7
/// 
/// # Description
///  PDPASA_REGIONSOLUTION is public so is available to all participants. Source PDPASA_REGIONSOLUTION is updated each PDPASA run (i.e. half-hourly). Volume Rows per day: 32000 Notes LRC Determination SURPLUSRESERVE is the surplus reserve in a region based on meeting the demand plus the reserve requirement in all regions simultaneously. Note that any surplus above the network restrictions and system reserve requirements is reported in the region it is generated, thus a surplus of zero can mean that a region is importing to meet a requirement or that it has exported all surplus to meet an adjacent region’s requirement. &nbsp; The PASA processes also calculate a regionally optimised surplus called the Maximum LRC Surplus (MAXSURPLUSRESERVE) being a figure on how much generation could be brought to this region subject to meeting requirements in other regions. &nbsp; LOR Determination MAXSPARECAPACITY is a regionally optimised figure representing the surplus generation able to be brought to a region subject to meeting the demand in all other regions. &nbsp; Participants are directed to the first half hour of the Predispatch PASA (PDPASA) reports as NEMMCO's latest reserve determination for a given half hour.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * INTERVAL_DATETIME
/// * REGIONID
/// * RUN_DATETIME
/// * RUNTYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct PdpasaRegionsolution7 {
    /// Case identifier by the time the case was run
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// End date time of the interval
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region identifier
    pub regionid: String,
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
    /// Sum of  MAXAVAIL quantities offered by all Scheduled Generators in a given Region for a given PERIODID.
    pub aggregatecapacityavailable: Option<rust_decimal::Decimal>,
    /// Sum of  MAXAVAIL quantities bid by of all Scheduled Loads in a given Region for a given PERIODID.
    pub aggregatescheduledload: Option<rust_decimal::Decimal>,
    /// Date time the record was created or modified changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Sum of  PASAAVAILABILITY quantities offered by all Scheduled Generators in a given Region for a given PERIODID.
    pub aggregatepasaavailability: Option<rust_decimal::Decimal>,
    /// Type of run. Values are RELIABILITY_LRC, OUTAGE_LRC and LOR.
    pub runtype: String,
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
    /// Aggregate Regional UIGF availability
    pub semi_scheduled_capacity: Option<rust_decimal::Decimal>,
    /// Aggregate Regional UIGF availability for LOR
    pub lor_semi_scheduled_capacity: Option<rust_decimal::Decimal>,
    /// Largest Credible Risk. MW value for highest credible contingency 
    pub lcr: Option<rust_decimal::Decimal>,
    /// Two Largest Creditable Risks. MW value for highest two credible contingencies.
    pub lcr2: Option<rust_decimal::Decimal>,
    /// Forecasting Uncertainty Measure. MW value of reserve calculated as defined in the Reserve Level Declaration Guidelines
    pub fum: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW) where the primary fuel source is solar
    pub ss_solar_uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Unconstrained Intermittent Generation Forecast of Semi-scheduled generation (MW) where the primary fuel source is wind
    pub ss_wind_uigf: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-scheduled UIGF availability where the primary fuel source is solar
    pub ss_solar_capacity: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-scheduled UIGF availability where the primary fuel source is wind
    pub ss_wind_capacity: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-scheduled cleared MW where the primary fuel source is solar and StudyRegion = Region
    pub ss_solar_cleared: Option<rust_decimal::Decimal>,
    /// Regional aggregated Semi-scheduled cleared MW where the primary fuel source is wind and StudyRegion = Region
    pub ss_wind_cleared: Option<rust_decimal::Decimal>,
    /// Regional aggregated Wholesale Demand Response (WDR) availability in MW.
    pub wdr_available: Option<rust_decimal::Decimal>,
    /// Regional aggregated Wholesale Demand Response (WDR) PASA availability in MW.
    pub wdr_pasaavailable: Option<rust_decimal::Decimal>,
    /// Regional aggregated Wholesale Demand Response (WDR) capacity in MW.
    pub wdr_capacity: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for PdpasaRegionsolution7 {
    type PrimaryKey = PdpasaRegionsolution7PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PDPASA".into(),
            table_name: Some("REGIONSOLUTION".into()),
            version: 7,
        }
    }

    fn primary_key(&self) -> PdpasaRegionsolution7PrimaryKey {
        PdpasaRegionsolution7PrimaryKey {
            interval_datetime: self.interval_datetime,
            regionid: self.regionid.clone(),
            run_datetime: self.run_datetime,
            runtype: self.runtype.clone()
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "pdpasa_regionsolution_v7".to_string()
    }
}
impl mmsdm_core::CompareWithRow for PdpasaRegionsolution7 {
    type Row = PdpasaRegionsolution7;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interval_datetime == row.interval_datetime
        && self.regionid == row.regionid
        && self.run_datetime == row.run_datetime
        && self.runtype == row.runtype
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PdpasaRegionsolution7 {
    type PrimaryKey = PdpasaRegionsolution7PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
        && self.regionid == key.regionid
        && self.run_datetime == key.run_datetime
        && self.runtype == key.runtype
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct PdpasaRegionsolution7PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: String,
    pub run_datetime: chrono::NaiveDateTime,
    pub runtype: String,
}
impl mmsdm_core::CompareWithRow for PdpasaRegionsolution7PrimaryKey {
    type Row = PdpasaRegionsolution7;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interval_datetime == row.interval_datetime
        && self.regionid == row.regionid
        && self.run_datetime == row.run_datetime
        && self.runtype == row.runtype
    }
}
impl mmsdm_core::CompareWithPrimaryKey for PdpasaRegionsolution7PrimaryKey {
    type PrimaryKey = PdpasaRegionsolution7PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
        && self.regionid == key.regionid
        && self.run_datetime == key.run_datetime
        && self.runtype == key.runtype
    }
}
impl mmsdm_core::PrimaryKey for PdpasaRegionsolution7PrimaryKey {
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for PdpasaRegionsolution7 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("run_datetime", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("interval_datetime", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("demand10", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("demand50", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("demand90", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("reservereq", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("capacityreq", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("energyreqdemand50", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("unconstrainedcapacity", arrow2::datatypes::DataType::Decimal(12,0), true),
            arrow2::datatypes::Field::new("constrainedcapacity", arrow2::datatypes::DataType::Decimal(12,0), true),
            arrow2::datatypes::Field::new("netinterchangeunderscarcity", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("surpluscapacity", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("surplusreserve", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("reservecondition", arrow2::datatypes::DataType::Decimal(1,0), true),
            arrow2::datatypes::Field::new("maxsurplusreserve", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("maxsparecapacity", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("lorcondition", arrow2::datatypes::DataType::Decimal(1,0), true),
            arrow2::datatypes::Field::new("aggregatecapacityavailable", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("aggregatescheduledload", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("aggregatepasaavailability", arrow2::datatypes::DataType::Decimal(12,0), true),
            arrow2::datatypes::Field::new("runtype", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("energyreqdemand10", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("calculatedlor1level", arrow2::datatypes::DataType::Decimal(16,6), true),
            arrow2::datatypes::Field::new("calculatedlor2level", arrow2::datatypes::DataType::Decimal(16,6), true),
            arrow2::datatypes::Field::new("msrnetinterchangeunderscarcity", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("lornetinterchangeunderscarcity", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("totalintermittentgeneration", arrow2::datatypes::DataType::Decimal(15,5), true),
            arrow2::datatypes::Field::new("demand_and_nonschedgen", arrow2::datatypes::DataType::Decimal(15,5), true),
            arrow2::datatypes::Field::new("uigf", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("semi_scheduled_capacity", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("lor_semi_scheduled_capacity", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("lcr", arrow2::datatypes::DataType::Decimal(16,6), true),
            arrow2::datatypes::Field::new("lcr2", arrow2::datatypes::DataType::Decimal(16,6), true),
            arrow2::datatypes::Field::new("fum", arrow2::datatypes::DataType::Decimal(16,6), true),
            arrow2::datatypes::Field::new("ss_solar_uigf", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("ss_wind_uigf", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("ss_solar_capacity", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("ss_wind_capacity", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("ss_solar_cleared", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("ss_wind_cleared", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("wdr_available", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("wdr_pasaavailable", arrow2::datatypes::DataType::Decimal(12,2), true),
            arrow2::datatypes::Field::new("wdr_capacity", arrow2::datatypes::DataType::Decimal(12,2), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut run_datetime_array = Vec::new();
        let mut interval_datetime_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut demand10_array = Vec::new();
        let mut demand50_array = Vec::new();
        let mut demand90_array = Vec::new();
        let mut reservereq_array = Vec::new();
        let mut capacityreq_array = Vec::new();
        let mut energyreqdemand50_array = Vec::new();
        let mut unconstrainedcapacity_array = Vec::new();
        let mut constrainedcapacity_array = Vec::new();
        let mut netinterchangeunderscarcity_array = Vec::new();
        let mut surpluscapacity_array = Vec::new();
        let mut surplusreserve_array = Vec::new();
        let mut reservecondition_array = Vec::new();
        let mut maxsurplusreserve_array = Vec::new();
        let mut maxsparecapacity_array = Vec::new();
        let mut lorcondition_array = Vec::new();
        let mut aggregatecapacityavailable_array = Vec::new();
        let mut aggregatescheduledload_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut aggregatepasaavailability_array = Vec::new();
        let mut runtype_array = Vec::new();
        let mut energyreqdemand10_array = Vec::new();
        let mut calculatedlor1level_array = Vec::new();
        let mut calculatedlor2level_array = Vec::new();
        let mut msrnetinterchangeunderscarcity_array = Vec::new();
        let mut lornetinterchangeunderscarcity_array = Vec::new();
        let mut totalintermittentgeneration_array = Vec::new();
        let mut demand_and_nonschedgen_array = Vec::new();
        let mut uigf_array = Vec::new();
        let mut semi_scheduled_capacity_array = Vec::new();
        let mut lor_semi_scheduled_capacity_array = Vec::new();
        let mut lcr_array = Vec::new();
        let mut lcr2_array = Vec::new();
        let mut fum_array = Vec::new();
        let mut ss_solar_uigf_array = Vec::new();
        let mut ss_wind_uigf_array = Vec::new();
        let mut ss_solar_capacity_array = Vec::new();
        let mut ss_wind_capacity_array = Vec::new();
        let mut ss_solar_cleared_array = Vec::new();
        let mut ss_wind_cleared_array = Vec::new();
        let mut wdr_available_array = Vec::new();
        let mut wdr_pasaavailable_array = Vec::new();
        let mut wdr_capacity_array = Vec::new();
        for row in partition {
            run_datetime_array.push(row.run_datetime.timestamp());
            interval_datetime_array.push(row.interval_datetime.timestamp());
            regionid_array.push(row.regionid);
            demand10_array.push({
                        row.demand10.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            demand50_array.push({
                        row.demand50.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            demand90_array.push({
                        row.demand90.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            reservereq_array.push({
                        row.reservereq.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            capacityreq_array.push({
                        row.capacityreq.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            energyreqdemand50_array.push({
                        row.energyreqdemand50.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            unconstrainedcapacity_array.push({
                        row.unconstrainedcapacity.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            constrainedcapacity_array.push({
                        row.constrainedcapacity.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            netinterchangeunderscarcity_array.push({
                        row.netinterchangeunderscarcity.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            surpluscapacity_array.push({
                        row.surpluscapacity.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            surplusreserve_array.push({
                        row.surplusreserve.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            reservecondition_array.push({
                        row.reservecondition.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            maxsurplusreserve_array.push({
                        row.maxsurplusreserve.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            maxsparecapacity_array.push({
                        row.maxsparecapacity.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            lorcondition_array.push({
                        row.lorcondition.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            aggregatecapacityavailable_array.push({
                        row.aggregatecapacityavailable.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            aggregatescheduledload_array.push({
                        row.aggregatescheduledload.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            aggregatepasaavailability_array.push({
                        row.aggregatepasaavailability.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            runtype_array.push(row.runtype);
            energyreqdemand10_array.push({
                        row.energyreqdemand10.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            calculatedlor1level_array.push({
                        row.calculatedlor1level.map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                    });
            calculatedlor2level_array.push({
                        row.calculatedlor2level.map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                    });
            msrnetinterchangeunderscarcity_array.push({
                        row.msrnetinterchangeunderscarcity.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            lornetinterchangeunderscarcity_array.push({
                        row.lornetinterchangeunderscarcity.map(|mut val| {
                            val.rescale(2);
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
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            semi_scheduled_capacity_array.push({
                        row.semi_scheduled_capacity.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            lor_semi_scheduled_capacity_array.push({
                        row.lor_semi_scheduled_capacity.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            lcr_array.push({
                        row.lcr.map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                    });
            lcr2_array.push({
                        row.lcr2.map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                    });
            fum_array.push({
                        row.fum.map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                    });
            ss_solar_uigf_array.push({
                        row.ss_solar_uigf.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            ss_wind_uigf_array.push({
                        row.ss_wind_uigf.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            ss_solar_capacity_array.push({
                        row.ss_solar_capacity.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            ss_wind_capacity_array.push({
                        row.ss_wind_capacity.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            ss_solar_cleared_array.push({
                        row.ss_solar_cleared.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            ss_wind_cleared_array.push({
                        row.ss_wind_cleared.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            wdr_available_array.push({
                        row.wdr_available.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            wdr_pasaavailable_array.push({
                        row.wdr_pasaavailable.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            wdr_capacity_array.push({
                        row.wdr_capacity.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(run_datetime_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(interval_datetime_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demand10_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demand50_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demand90_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(reservereq_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(capacityreq_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(energyreqdemand50_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(unconstrainedcapacity_array).to(arrow2::datatypes::DataType::Decimal(12,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(constrainedcapacity_array).to(arrow2::datatypes::DataType::Decimal(12,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(netinterchangeunderscarcity_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(surpluscapacity_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(surplusreserve_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(reservecondition_array).to(arrow2::datatypes::DataType::Decimal(1,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxsurplusreserve_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxsparecapacity_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lorcondition_array).to(arrow2::datatypes::DataType::Decimal(1,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(aggregatecapacityavailable_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(aggregatescheduledload_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(aggregatepasaavailability_array).to(arrow2::datatypes::DataType::Decimal(12,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(runtype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(energyreqdemand10_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(calculatedlor1level_array).to(arrow2::datatypes::DataType::Decimal(16,6))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(calculatedlor2level_array).to(arrow2::datatypes::DataType::Decimal(16,6))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(msrnetinterchangeunderscarcity_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lornetinterchangeunderscarcity_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalintermittentgeneration_array).to(arrow2::datatypes::DataType::Decimal(15,5))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demand_and_nonschedgen_array).to(arrow2::datatypes::DataType::Decimal(15,5))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(uigf_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(semi_scheduled_capacity_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lor_semi_scheduled_capacity_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lcr_array).to(arrow2::datatypes::DataType::Decimal(16,6))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lcr2_array).to(arrow2::datatypes::DataType::Decimal(16,6))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(fum_array).to(arrow2::datatypes::DataType::Decimal(16,6))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(ss_solar_uigf_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(ss_wind_uigf_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(ss_solar_capacity_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(ss_wind_capacity_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(ss_solar_cleared_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(ss_wind_cleared_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(wdr_available_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(wdr_pasaavailable_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(wdr_capacity_array).to(arrow2::datatypes::DataType::Decimal(12,2))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
