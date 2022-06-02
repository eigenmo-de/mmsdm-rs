/// # Summary
/// 
/// ## OFFERAGCDATA
///  _OFFERAGCDATA shows availability reoffers of Automatic Generation Control. _
/// 
/// * Data Set Name: Asoffer
/// * File Name: Offeragcdata
/// * Data Version: 1
/// 
/// # Description
///  OFFERAGCDATA data is confidential to the relevant participant. Source OFFERAGCDATA updates as reoffers submitted.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTID
/// * EFFECTIVEDATE
/// * PERIODID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferagcdata1 {
    /// Contract Identifier
    pub contractid: String,
    /// Market date of offer
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of record
    pub versionno: rust_decimal::Decimal,
    /// Availability flag (0 or 1)
    pub availability: Option<rust_decimal::Decimal>,
    /// Upper control limit. This is used by SPD.
    pub upperlimit: Option<rust_decimal::Decimal>,
    /// Lower control limit MW. This is used by SPD.
    pub lowerlimit: Option<rust_decimal::Decimal>,
    /// Authorised date
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorised by
    pub authorisedby: Option<String>,
    /// Name of reoffer file
    pub filename: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Market day trading interval number
    pub periodid: rust_decimal::Decimal,
    /// AGC Ramp Rate Up. This is used by SPD.
    pub agcup: Option<rust_decimal::Decimal>,
    /// AGC Ramp Rate Down. This is used by SPD.
    pub agcdown: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for AsofferOfferagcdata1 {
    type PrimaryKey = AsofferOfferagcdata1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "ASOFFER".into(),
            table_name: Some("OFFERAGCDATA".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> AsofferOfferagcdata1PrimaryKey {
        AsofferOfferagcdata1PrimaryKey {
            contractid: self.contractid.clone(),
            effectivedate: self.effectivedate,
            periodid: self.periodid,
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "asoffer_offeragcdata_v1".to_string()
    }
}
impl mmsdm_core::CompareWithRow for AsofferOfferagcdata1 {
    type Row = AsofferOfferagcdata1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
        && self.effectivedate == row.effectivedate
        && self.periodid == row.periodid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for AsofferOfferagcdata1 {
    type PrimaryKey = AsofferOfferagcdata1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
        && self.effectivedate == key.effectivedate
        && self.periodid == key.periodid
        && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct AsofferOfferagcdata1PrimaryKey {
    pub contractid: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::CompareWithRow for AsofferOfferagcdata1PrimaryKey {
    type Row = AsofferOfferagcdata1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
        && self.effectivedate == row.effectivedate
        && self.periodid == row.periodid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for AsofferOfferagcdata1PrimaryKey {
    type PrimaryKey = AsofferOfferagcdata1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
        && self.effectivedate == key.effectivedate
        && self.periodid == key.periodid
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::PrimaryKey for AsofferOfferagcdata1PrimaryKey {
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for AsofferOfferagcdata1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("contractid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("availability", arrow2::datatypes::DataType::Decimal(4,0), true),
            arrow2::datatypes::Field::new("upperlimit", arrow2::datatypes::DataType::Decimal(4,0), true),
            arrow2::datatypes::Field::new("lowerlimit", arrow2::datatypes::DataType::Decimal(4,0), true),
            arrow2::datatypes::Field::new("authoriseddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("authorisedby", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("filename", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("agcup", arrow2::datatypes::DataType::Decimal(3,0), true),
            arrow2::datatypes::Field::new("agcdown", arrow2::datatypes::DataType::Decimal(3,0), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut contractid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut availability_array = Vec::new();
        let mut upperlimit_array = Vec::new();
        let mut lowerlimit_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut filename_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut agcup_array = Vec::new();
        let mut agcdown_array = Vec::new();
        for row in partition {
            contractid_array.push(row.contractid);
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            availability_array.push({
                        row.availability.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            upperlimit_array.push({
                        row.upperlimit.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            lowerlimit_array.push({
                        row.lowerlimit.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            authorisedby_array.push(row.authorisedby);
            filename_array.push(row.filename);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            periodid_array.push({
                        let mut val = row.periodid;
                        val.rescale(0);
                        val.mantissa()
                    });
            agcup_array.push({
                        row.agcup.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            agcdown_array.push({
                        row.agcdown.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(contractid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(availability_array).to(arrow2::datatypes::DataType::Decimal(4,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(upperlimit_array).to(arrow2::datatypes::DataType::Decimal(4,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowerlimit_array).to(arrow2::datatypes::DataType::Decimal(4,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(filename_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(agcup_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(agcdown_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## OFFERASTRK
///  _OFFERASTRK tracks successfully acknowledged ancillary service reoffers._
/// 
/// * Data Set Name: Asoffer
/// * File Name: Offerastrk
/// * Data Version: 1
/// 
/// # Description
///  OFFERASTRK data is confidential to the relevant participant. Source OFFERASTRK is updated as offers are successfully acknowledged.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferastrk1 {
    /// Market day starting at 4:00 am
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of the offer for that date
    pub versionno: rust_decimal::Decimal,
    /// Participant ID
    pub participantid: String,
    /// Submitted file name.
    pub filename: Option<String>,
    /// Last changed date and time.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for AsofferOfferastrk1 {
    type PrimaryKey = AsofferOfferastrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "ASOFFER".into(),
            table_name: Some("OFFERASTRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> AsofferOfferastrk1PrimaryKey {
        AsofferOfferastrk1PrimaryKey {
            effectivedate: self.effectivedate,
            participantid: self.participantid.clone(),
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "asoffer_offerastrk_v1".to_string()
    }
}
impl mmsdm_core::CompareWithRow for AsofferOfferastrk1 {
    type Row = AsofferOfferastrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.participantid == row.participantid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for AsofferOfferastrk1 {
    type PrimaryKey = AsofferOfferastrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.participantid == key.participantid
        && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct AsofferOfferastrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::CompareWithRow for AsofferOfferastrk1PrimaryKey {
    type Row = AsofferOfferastrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.participantid == row.participantid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for AsofferOfferastrk1PrimaryKey {
    type PrimaryKey = AsofferOfferastrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.participantid == key.participantid
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::PrimaryKey for AsofferOfferastrk1PrimaryKey {
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for AsofferOfferastrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("participantid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("filename", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut filename_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            participantid_array.push(row.participantid);
            filename_array.push(row.filename);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(participantid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(filename_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## OFFERLSHEDDATA
///  _OFFERLSHEDDATA shows reoffers of load shed including available load shed quantity._
/// 
/// * Data Set Name: Asoffer
/// * File Name: Offerlsheddata
/// * Data Version: 1
/// 
/// # Description
///  OFFERLSHEDDATA data is confidential to the relevant participant. Source OFFERLSHEDDATA updates as reoffers process.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTID
/// * EFFECTIVEDATE
/// * PERIODID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferlsheddata1 {
    /// Contract identifier
    pub contractid: String,
    /// Market date of reoffer
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version No of reoffer
    pub versionno: rust_decimal::Decimal,
    /// Available load
    pub availableload: Option<rust_decimal::Decimal>,
    /// Authorised date
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorised by
    pub authorisedby: Option<String>,
    /// Name of reoffer file
    pub filename: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Market day trading interval number
    pub periodid: rust_decimal::Decimal,
}
impl mmsdm_core::GetTable for AsofferOfferlsheddata1 {
    type PrimaryKey = AsofferOfferlsheddata1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "ASOFFER".into(),
            table_name: Some("OFFERLSHEDDATA".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> AsofferOfferlsheddata1PrimaryKey {
        AsofferOfferlsheddata1PrimaryKey {
            contractid: self.contractid.clone(),
            effectivedate: self.effectivedate,
            periodid: self.periodid,
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "asoffer_offerlsheddata_v1".to_string()
    }
}
impl mmsdm_core::CompareWithRow for AsofferOfferlsheddata1 {
    type Row = AsofferOfferlsheddata1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
        && self.effectivedate == row.effectivedate
        && self.periodid == row.periodid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for AsofferOfferlsheddata1 {
    type PrimaryKey = AsofferOfferlsheddata1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
        && self.effectivedate == key.effectivedate
        && self.periodid == key.periodid
        && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct AsofferOfferlsheddata1PrimaryKey {
    pub contractid: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::CompareWithRow for AsofferOfferlsheddata1PrimaryKey {
    type Row = AsofferOfferlsheddata1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
        && self.effectivedate == row.effectivedate
        && self.periodid == row.periodid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for AsofferOfferlsheddata1PrimaryKey {
    type PrimaryKey = AsofferOfferlsheddata1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
        && self.effectivedate == key.effectivedate
        && self.periodid == key.periodid
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::PrimaryKey for AsofferOfferlsheddata1PrimaryKey {
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for AsofferOfferlsheddata1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("contractid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("availableload", arrow2::datatypes::DataType::Decimal(4,0), true),
            arrow2::datatypes::Field::new("authoriseddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("authorisedby", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("filename", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::Decimal(3,0), false)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut contractid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut availableload_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut filename_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut periodid_array = Vec::new();
        for row in partition {
            contractid_array.push(row.contractid);
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            availableload_array.push({
                        row.availableload.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            authorisedby_array.push(row.authorisedby);
            filename_array.push(row.filename);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            periodid_array.push({
                        let mut val = row.periodid;
                        val.rescale(0);
                        val.mantissa()
                    });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(contractid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(availableload_array).to(arrow2::datatypes::DataType::Decimal(4,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(filename_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## OFFERRESTARTDATA
///  _OFFERRESTARTDATA sets out reoffers of system restart availability._
/// 
/// * Data Set Name: Asoffer
/// * File Name: Offerrestartdata
/// * Data Version: 1
/// 
/// # Description
///  OFFERRESTARTDATA data is confidential to the relevant participant. Source OFFERRESTARTDATA updates as reoffers process.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTID
/// * OFFERDATE
/// * PERIODID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferrestartdata1 {
    /// Contract identifier
    pub contractid: String,
    /// Effective date of contract
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub offerdate: chrono::NaiveDateTime,
    /// Version No of contract
    pub versionno: rust_decimal::Decimal,
    /// Available load
    pub availability: Option<String>,
    /// Authorised date
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorised by
    pub authorisedby: Option<String>,
    /// Name of reoffer file
    pub filename: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Market day trading interval number
    pub periodid: rust_decimal::Decimal,
}
impl mmsdm_core::GetTable for AsofferOfferrestartdata1 {
    type PrimaryKey = AsofferOfferrestartdata1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "ASOFFER".into(),
            table_name: Some("OFFERRESTARTDATA".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> AsofferOfferrestartdata1PrimaryKey {
        AsofferOfferrestartdata1PrimaryKey {
            contractid: self.contractid.clone(),
            offerdate: self.offerdate,
            periodid: self.periodid,
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "asoffer_offerrestartdata_v1".to_string()
    }
}
impl mmsdm_core::CompareWithRow for AsofferOfferrestartdata1 {
    type Row = AsofferOfferrestartdata1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
        && self.offerdate == row.offerdate
        && self.periodid == row.periodid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for AsofferOfferrestartdata1 {
    type PrimaryKey = AsofferOfferrestartdata1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
        && self.offerdate == key.offerdate
        && self.periodid == key.periodid
        && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct AsofferOfferrestartdata1PrimaryKey {
    pub contractid: String,
    pub offerdate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::CompareWithRow for AsofferOfferrestartdata1PrimaryKey {
    type Row = AsofferOfferrestartdata1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
        && self.offerdate == row.offerdate
        && self.periodid == row.periodid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for AsofferOfferrestartdata1PrimaryKey {
    type PrimaryKey = AsofferOfferrestartdata1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
        && self.offerdate == key.offerdate
        && self.periodid == key.periodid
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::PrimaryKey for AsofferOfferrestartdata1PrimaryKey {
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for AsofferOfferrestartdata1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("contractid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("offerdate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("availability", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("authoriseddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("authorisedby", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("filename", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::Decimal(3,0), false)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut contractid_array = Vec::new();
        let mut offerdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut availability_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut filename_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut periodid_array = Vec::new();
        for row in partition {
            contractid_array.push(row.contractid);
            offerdate_array.push(row.offerdate.timestamp());
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            availability_array.push(row.availability);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            authorisedby_array.push(row.authorisedby);
            filename_array.push(row.filename);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            periodid_array.push({
                        let mut val = row.periodid;
                        val.rescale(0);
                        val.mantissa()
                    });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(contractid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(offerdate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(availability_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(filename_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## OFFERRPOWERDATA
///  _OFFERRPOWERDATA shows reoffers of reactive power capability and settlement measurements._
/// 
/// * Data Set Name: Asoffer
/// * File Name: Offerrpowerdata
/// * Data Version: 1
/// 
/// # Description
///  OFFERRPOWERDATA data is confidential to the relevant participant. Source OFFERRPOWERDATA updates as reoffers process. 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * CONTRACTID
/// * EFFECTIVEDATE
/// * PERIODID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AsofferOfferrpowerdata1 {
    /// Contract Version No.
    pub contractid: String,
    /// Contract Version No.
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version No. of Re-Offer
    pub versionno: rust_decimal::Decimal,
    /// Market trading interval
    pub periodid: rust_decimal::Decimal,
    /// Availability of service
    pub availability: Option<rust_decimal::Decimal>,
    /// Reactive Power Absorption Capability (MVar)
    pub mta: Option<rust_decimal::Decimal>,
    /// Reactive Power Generation Capability (MVar)
    pub mtg: Option<rust_decimal::Decimal>,
    /// Date Contract was Authorised
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User Name
    pub authorisedby: Option<String>,
    /// File name of Re-Offer file
    pub filename: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for AsofferOfferrpowerdata1 {
    type PrimaryKey = AsofferOfferrpowerdata1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "ASOFFER".into(),
            table_name: Some("OFFERRPOWERDATA".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> AsofferOfferrpowerdata1PrimaryKey {
        AsofferOfferrpowerdata1PrimaryKey {
            contractid: self.contractid.clone(),
            effectivedate: self.effectivedate,
            periodid: self.periodid,
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "asoffer_offerrpowerdata_v1".to_string()
    }
}
impl mmsdm_core::CompareWithRow for AsofferOfferrpowerdata1 {
    type Row = AsofferOfferrpowerdata1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
        && self.effectivedate == row.effectivedate
        && self.periodid == row.periodid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for AsofferOfferrpowerdata1 {
    type PrimaryKey = AsofferOfferrpowerdata1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
        && self.effectivedate == key.effectivedate
        && self.periodid == key.periodid
        && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct AsofferOfferrpowerdata1PrimaryKey {
    pub contractid: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::CompareWithRow for AsofferOfferrpowerdata1PrimaryKey {
    type Row = AsofferOfferrpowerdata1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
        && self.effectivedate == row.effectivedate
        && self.periodid == row.periodid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for AsofferOfferrpowerdata1PrimaryKey {
    type PrimaryKey = AsofferOfferrpowerdata1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
        && self.effectivedate == key.effectivedate
        && self.periodid == key.periodid
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::PrimaryKey for AsofferOfferrpowerdata1PrimaryKey {
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for AsofferOfferrpowerdata1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("contractid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("availability", arrow2::datatypes::DataType::Decimal(3,0), true),
            arrow2::datatypes::Field::new("mta", arrow2::datatypes::DataType::Decimal(6,0), true),
            arrow2::datatypes::Field::new("mtg", arrow2::datatypes::DataType::Decimal(6,0), true),
            arrow2::datatypes::Field::new("authoriseddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("authorisedby", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("filename", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut contractid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut availability_array = Vec::new();
        let mut mta_array = Vec::new();
        let mut mtg_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut filename_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractid_array.push(row.contractid);
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            periodid_array.push({
                        let mut val = row.periodid;
                        val.rescale(0);
                        val.mantissa()
                    });
            availability_array.push({
                        row.availability.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            mta_array.push({
                        row.mta.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            mtg_array.push({
                        row.mtg.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            authorisedby_array.push(row.authorisedby);
            filename_array.push(row.filename);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(contractid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(availability_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(mta_array).to(arrow2::datatypes::DataType::Decimal(6,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(mtg_array).to(arrow2::datatypes::DataType::Decimal(6,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(filename_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
