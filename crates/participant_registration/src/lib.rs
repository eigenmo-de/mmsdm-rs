/// # Summary
/// 
/// ## BIDDUIDDETAILS
///  _BIDDUIDDETAILS and the associated tracking object BIDDUIDDETAILSTRK define the registration data for each ancillary service a dispatchable unit is registered to provide. The registration data is required to validate a dispatchable unit bid submitted for that ancillary service._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Bidduiddetails
/// * Data Version: 1
/// 
/// # Description
///  BIDDUIDDETAILS data is public to participants. Source BIDDUIDDETAILS updates as dispatchable unit registration details are modified. Volume Approximately 1000 records per year.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * BIDTYPE
/// * DUID
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationBidduiddetails1 {
    /// Dispatchable unit identifier
    pub duid: String,
    /// Market date starting at 04:30 inclusive
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Record version number
    pub versionno: rust_decimal::Decimal,
    /// Bid Type Identifier
    pub bidtype: String,
    /// Maximum Capacity of this DUID for this BIDTYPE
    pub maxcapacity: Option<rust_decimal::Decimal>,
    /// Minimum Energy Output (MW) at which this ancillary service becomes available (AS Only)
    pub minenablementlevel: Option<rust_decimal::Decimal>,
    /// Maximum Energy Output (MW) at which this ancillary service can be supplied (AS Only)
    pub maxenablementlevel: Option<rust_decimal::Decimal>,
    /// Maximum Angle at the lower end of the ancillary service profile (Degrees)
    pub maxlowerangle: Option<rust_decimal::Decimal>,
    /// Maximum Angle at the upper end of the ancillary service profile (Degrees)
    pub maxupperangle: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationBidduiddetails1 {
    type PrimaryKey = ParticipantRegistrationBidduiddetails1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("BIDDUIDDETAILS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationBidduiddetails1PrimaryKey {
        ParticipantRegistrationBidduiddetails1PrimaryKey {
            bidtype: self.bidtype.clone(),
            duid: self.duid.clone(),
            effectivedate: self.effectivedate,
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_bidduiddetails_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationBidduiddetails1PrimaryKey {
    pub bidtype: String,
    pub duid: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationBidduiddetails1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationBidduiddetails1 {
    type Row = ParticipantRegistrationBidduiddetails1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
        && self.duid == row.duid
        && self.effectivedate == row.effectivedate
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationBidduiddetails1 {
    type PrimaryKey = ParticipantRegistrationBidduiddetails1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
        && self.duid == key.duid
        && self.effectivedate == key.effectivedate
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationBidduiddetails1PrimaryKey {
    type Row = ParticipantRegistrationBidduiddetails1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
        && self.duid == row.duid
        && self.effectivedate == row.effectivedate
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationBidduiddetails1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationBidduiddetails1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
        && self.duid == key.duid
        && self.effectivedate == key.effectivedate
        && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationBidduiddetails1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("bidtype", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("maxcapacity", arrow2::datatypes::DataType::Decimal(22,0), true),
            arrow2::datatypes::Field::new("minenablementlevel", arrow2::datatypes::DataType::Decimal(22,0), true),
            arrow2::datatypes::Field::new("maxenablementlevel", arrow2::datatypes::DataType::Decimal(22,0), true),
            arrow2::datatypes::Field::new("maxlowerangle", arrow2::datatypes::DataType::Decimal(3,0), true),
            arrow2::datatypes::Field::new("maxupperangle", arrow2::datatypes::DataType::Decimal(3,0), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut duid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut bidtype_array = Vec::new();
        let mut maxcapacity_array = Vec::new();
        let mut minenablementlevel_array = Vec::new();
        let mut maxenablementlevel_array = Vec::new();
        let mut maxlowerangle_array = Vec::new();
        let mut maxupperangle_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            duid_array.push(row.duid);
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            bidtype_array.push(row.bidtype);
            maxcapacity_array.push({
                        row.maxcapacity.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            minenablementlevel_array.push({
                        row.minenablementlevel.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            maxenablementlevel_array.push({
                        row.maxenablementlevel.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            maxlowerangle_array.push({
                        row.maxlowerangle.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            maxupperangle_array.push({
                        row.maxupperangle.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bidtype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxcapacity_array).to(arrow2::datatypes::DataType::Decimal(22,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(minenablementlevel_array).to(arrow2::datatypes::DataType::Decimal(22,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxenablementlevel_array).to(arrow2::datatypes::DataType::Decimal(22,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxlowerangle_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxupperangle_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## BIDDUIDDETAILSTRK
///  _BIDDUIDDETAILSTRK shows the tracking for the associated object BIDDUIDDETAILS. Together, BIDDUIDDETAILSTRK and BIDDUIDDETAILS define the registration data for each ancillary service a dispatchable unit is registered to provide. The registration data is required to validate a dispatchable unit bid submitted for that ancillary service._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Bidduiddetailstrk
/// * Data Version: 1
/// 
/// # Description
///  BIDDUIDDETAILSTRK data is public to participants. Source BIDDUIDDETAILSTRK updates as dispatchable unit registration details are modified. Volume Approximately 200 records per year
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationBidduiddetailstrk1 {
    /// Dispatchable unit identifier
    pub duid: String,
    /// Market date starting at 04:30 inclusive
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Record version number
    pub versionno: rust_decimal::Decimal,
    /// Date of record authorisation. A NULL value indicates the record is not authorised.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User that authorised record. A NULL value indicates the record is not authorised.
    pub authorisedby: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationBidduiddetailstrk1 {
    type PrimaryKey = ParticipantRegistrationBidduiddetailstrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("BIDDUIDDETAILSTRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationBidduiddetailstrk1PrimaryKey {
        ParticipantRegistrationBidduiddetailstrk1PrimaryKey {
            duid: self.duid.clone(),
            effectivedate: self.effectivedate,
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_bidduiddetailstrk_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationBidduiddetailstrk1PrimaryKey {
    pub duid: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationBidduiddetailstrk1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationBidduiddetailstrk1 {
    type Row = ParticipantRegistrationBidduiddetailstrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
        && self.effectivedate == row.effectivedate
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationBidduiddetailstrk1 {
    type PrimaryKey = ParticipantRegistrationBidduiddetailstrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
        && self.effectivedate == key.effectivedate
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationBidduiddetailstrk1PrimaryKey {
    type Row = ParticipantRegistrationBidduiddetailstrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
        && self.effectivedate == row.effectivedate
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationBidduiddetailstrk1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationBidduiddetailstrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
        && self.effectivedate == key.effectivedate
        && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationBidduiddetailstrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("authoriseddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("authorisedby", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut duid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            duid_array.push(row.duid);
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            authorisedby_array.push(row.authorisedby);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## DISPATCHABLEUNIT
///  _DISPATCHABLEUNIT sets out the unit name and type of each dispatchable unit in the market._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Dispatchableunit
/// * Data Version: 1
/// 
/// # Description
///  DISPATCHABLEUNIT data is public data, and is available to all participants. Source DISPATCHABLEUNIT pdates as new units added or names changed.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DUID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationDispatchableunit1 {
    /// Dispatchable Unit Identifier
    pub duid: String,
    /// Dispatchable Unit full description
    pub duname: Option<String>,
    /// Generation or Load
    pub unittype: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationDispatchableunit1 {
    type PrimaryKey = ParticipantRegistrationDispatchableunit1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("DISPATCHABLEUNIT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationDispatchableunit1PrimaryKey {
        ParticipantRegistrationDispatchableunit1PrimaryKey {
            duid: self.duid.clone()
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_dispatchableunit_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationDispatchableunit1PrimaryKey {
    pub duid: String,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationDispatchableunit1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationDispatchableunit1 {
    type Row = ParticipantRegistrationDispatchableunit1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationDispatchableunit1 {
    type PrimaryKey = ParticipantRegistrationDispatchableunit1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationDispatchableunit1PrimaryKey {
    type Row = ParticipantRegistrationDispatchableunit1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationDispatchableunit1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationDispatchableunit1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationDispatchableunit1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("duname", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("unittype", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut duid_array = Vec::new();
        let mut duname_array = Vec::new();
        let mut unittype_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            duid_array.push(row.duid);
            duname_array.push(row.duname);
            unittype_array.push(row.unittype);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(duname_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(unittype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## DUALLOC
///  _DUALLOC cross references dispatch unit identifier to genset ID for each participant._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Dualloc
/// * Data Version: 1
/// 
/// # Description
///  Source DUALLOC updates where changed.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * EFFECTIVEDATE
/// * GENSETID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationDualloc1 {
    /// Effective calendar date of record
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of record
    pub versionno: rust_decimal::Decimal,
    /// Dispatchable Unit identifier
    pub duid: String,
    /// Physical unit identifier
    pub gensetid: String,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationDualloc1 {
    type PrimaryKey = ParticipantRegistrationDualloc1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("DUALLOC".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationDualloc1PrimaryKey {
        ParticipantRegistrationDualloc1PrimaryKey {
            duid: self.duid.clone(),
            effectivedate: self.effectivedate,
            gensetid: self.gensetid.clone(),
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_dualloc_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationDualloc1PrimaryKey {
    pub duid: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub gensetid: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationDualloc1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationDualloc1 {
    type Row = ParticipantRegistrationDualloc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
        && self.effectivedate == row.effectivedate
        && self.gensetid == row.gensetid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationDualloc1 {
    type PrimaryKey = ParticipantRegistrationDualloc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
        && self.effectivedate == key.effectivedate
        && self.gensetid == key.gensetid
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationDualloc1PrimaryKey {
    type Row = ParticipantRegistrationDualloc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
        && self.effectivedate == row.effectivedate
        && self.gensetid == row.gensetid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationDualloc1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationDualloc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
        && self.effectivedate == key.effectivedate
        && self.gensetid == key.gensetid
        && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationDualloc1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("gensetid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut gensetid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            duid_array.push(row.duid);
            gensetid_array.push(row.gensetid);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(gensetid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## DUDETAIL
///  _DUDETAIL sets out a records specific details for each unit including start type and whether normally on or off load. Much of this data is information only and is not used in dispatch or settlements._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Dudetail
/// * Data Version: 4
/// 
/// # Description
///  DUDETAIL is public data, and is available to all participants. Source DUDETAIL updates only when registration details change. Note To find the current set of details for selected dispatchable units, query the participant's local database as follows. Select du.* from dudetail du where (du.EFFECTIVEDATE, du.VERSIONNO) = ( select effectivedate, max(versionno) from dudetail where EFFECTIVEDATE = (select max(effectivedate) from  dudetail where EFFECTIVEDATE &lt;= sysdate and duid = du.duid and authoriseddate is not null) and duid = du.duid and authoriseddate is not null group by effectivedate ) and du.duid in ('UNIT1', 'UNIT2') ; The following notes apply to this SQL code: · This table is specific to dispatch units only. · If you wish to query details for a different date, substitute a date expression for "sysdate" in the "where EFFECTIVEDATE &lt;= sysdate" clause. · If you wish to list all the units, remove the line "and du.duid in ('UNIT1', 'UNIT2')" · The DUDETAIL table does not indicate if a unit is active;  this is done through ownership (STADUALLOC) by an active station owned by an active participant (STATIONOWNER ) · If you wish to query Station details refer to STATION, STATIONOWNER and STADUALLOC. · If you wish to look at connection point loss factors, refer to TRANSMISSIONLOSSFACTOR.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationDudetail4 {
    /// Effective calendar date of record
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Dispatchable Unit Identifier
    pub duid: String,
    /// version of Dispatchable Unit details for this effective date
    pub versionno: rust_decimal::Decimal,
    /// Country wide - Unique id of a connection point
    pub connectionpointid: Option<String>,
    /// Voltage Level
    pub voltlevel: Option<String>,
    /// Registered capacity for normal operations
    pub registeredcapacity: Option<rust_decimal::Decimal>,
    /// AGC Capability flag
    pub agccapability: Option<String>,
    /// Identifies LOAD or GENERATOR. This will likely expand to more generic models as new technology types are integrated into the NEM
    pub dispatchtype: Option<String>,
    /// Maximum Capacity as used for bid validation
    pub maxcapacity: Option<rust_decimal::Decimal>,
    /// Identify unit as Fast or Slow
    pub starttype: Option<String>,
    /// For a dispatchable load indicates that the load is normally on or off.
    pub normallyonflag: Option<String>,
    /// Indicates that the physical details for this unit are to be recorded
    pub physicaldetailsflag: Option<String>,
    /// Indicates spinning reserve capability
    pub spinningreserveflag: Option<String>,
    /// User authorising record
    pub authorisedby: Option<String>,
    /// Date record authorised
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Indicate whether a unit is intermittent (e.g. a wind farm)
    pub intermittentflag: Option<String>,
    /// Indicates if the DUID is a Semi-Scheduled Unit
    pub semi_schedule_flag: Option<String>,
    /// Maximum ramp up rate for Unit (Mw/min)
    pub maxrateofchangeup: Option<rust_decimal::Decimal>,
    /// Maximum ramp down rate for Unit (Mw/min)
    pub maxrateofchangedown: Option<rust_decimal::Decimal>,
    /// Additional information for DISPATCHTYPE. For DISPATCHTYPE = LOAD, subtype value is WDR for wholesale demand response units. For DISPATCHTYPE = LOAD, subtype value is NULL for Scheduled Loads. For DISPATCHTYPE = GENERATOR type, the subtype value is NULL. 
    pub dispatchsubtype: Option<String>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationDudetail4 {
    type PrimaryKey = ParticipantRegistrationDudetail4PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("DUDETAIL".into()),
            version: 4,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationDudetail4PrimaryKey {
        ParticipantRegistrationDudetail4PrimaryKey {
            duid: self.duid.clone(),
            effectivedate: self.effectivedate,
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_dudetail_v4".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationDudetail4PrimaryKey {
    pub duid: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationDudetail4PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationDudetail4 {
    type Row = ParticipantRegistrationDudetail4;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
        && self.effectivedate == row.effectivedate
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationDudetail4 {
    type PrimaryKey = ParticipantRegistrationDudetail4PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
        && self.effectivedate == key.effectivedate
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationDudetail4PrimaryKey {
    type Row = ParticipantRegistrationDudetail4;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
        && self.effectivedate == row.effectivedate
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationDudetail4PrimaryKey {
    type PrimaryKey = ParticipantRegistrationDudetail4PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
        && self.effectivedate == key.effectivedate
        && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationDudetail4 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("connectionpointid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("voltlevel", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("registeredcapacity", arrow2::datatypes::DataType::Decimal(6,0), true),
            arrow2::datatypes::Field::new("agccapability", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("dispatchtype", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("maxcapacity", arrow2::datatypes::DataType::Decimal(6,0), true),
            arrow2::datatypes::Field::new("starttype", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("normallyonflag", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("physicaldetailsflag", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("spinningreserveflag", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("authorisedby", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("authoriseddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("intermittentflag", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("semi_schedule_flag", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("maxrateofchangeup", arrow2::datatypes::DataType::Decimal(6,0), true),
            arrow2::datatypes::Field::new("maxrateofchangedown", arrow2::datatypes::DataType::Decimal(6,0), true),
            arrow2::datatypes::Field::new("dispatchsubtype", arrow2::datatypes::DataType::LargeUtf8, true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut effectivedate_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut voltlevel_array = Vec::new();
        let mut registeredcapacity_array = Vec::new();
        let mut agccapability_array = Vec::new();
        let mut dispatchtype_array = Vec::new();
        let mut maxcapacity_array = Vec::new();
        let mut starttype_array = Vec::new();
        let mut normallyonflag_array = Vec::new();
        let mut physicaldetailsflag_array = Vec::new();
        let mut spinningreserveflag_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut intermittentflag_array = Vec::new();
        let mut semi_schedule_flag_array = Vec::new();
        let mut maxrateofchangeup_array = Vec::new();
        let mut maxrateofchangedown_array = Vec::new();
        let mut dispatchsubtype_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            duid_array.push(row.duid);
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            connectionpointid_array.push(row.connectionpointid);
            voltlevel_array.push(row.voltlevel);
            registeredcapacity_array.push({
                        row.registeredcapacity.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            agccapability_array.push(row.agccapability);
            dispatchtype_array.push(row.dispatchtype);
            maxcapacity_array.push({
                        row.maxcapacity.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            starttype_array.push(row.starttype);
            normallyonflag_array.push(row.normallyonflag);
            physicaldetailsflag_array.push(row.physicaldetailsflag);
            spinningreserveflag_array.push(row.spinningreserveflag);
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            intermittentflag_array.push(row.intermittentflag);
            semi_schedule_flag_array.push(row.semi_schedule_flag);
            maxrateofchangeup_array.push({
                        row.maxrateofchangeup.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            maxrateofchangedown_array.push({
                        row.maxrateofchangedown.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            dispatchsubtype_array.push(row.dispatchsubtype);
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(connectionpointid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(voltlevel_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(registeredcapacity_array).to(arrow2::datatypes::DataType::Decimal(6,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(agccapability_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(dispatchtype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxcapacity_array).to(arrow2::datatypes::DataType::Decimal(6,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(starttype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(normallyonflag_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(physicaldetailsflag_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(spinningreserveflag_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(intermittentflag_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(semi_schedule_flag_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxrateofchangeup_array).to(arrow2::datatypes::DataType::Decimal(6,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxrateofchangedown_array).to(arrow2::datatypes::DataType::Decimal(6,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(dispatchsubtype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## DUDETAILSUMMARY
///  _DUDETAILSUMMARY sets out a single summary unit table so reducing the need for participants to use the various dispatchable unit detail and owner tables to establish generating unit specific details._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Dudetailsummary
/// * Data Version: 5
/// 
/// # Description
///  DUDETAILSUMMARY is a public table, and is available to all participants. Source DUDETAILSUMMARY updates only when registration details change.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * START_DATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationDudetailsummary5 {
    /// Dispatchable Unit Identifier
    pub duid: String,
    /// Start date for effective record
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub start_date: chrono::NaiveDateTime,
    /// End date for effective record
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub end_date: chrono::NaiveDateTime,
    /// Identifies LOAD or GENERATOR. This will likely expand to more generic models as new technology types are integrated into the NEM
    pub dispatchtype: Option<String>,
    /// Country wide - Unique id of a connection point
    pub connectionpointid: Option<String>,
    /// Region identifier that unit is in
    pub regionid: Option<String>,
    /// Station that unit is in
    pub stationid: Option<String>,
    /// Participant that owns unit during effective record period
    pub participantid: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The transmission level loss factor for currently assigned connection point
    pub transmissionlossfactor: Option<rust_decimal::Decimal>,
    /// Unit start type. At this time restricted to Fast, Slow or Non Dispatched 
    pub starttype: Option<String>,
    /// The distribution loss factor to the currently assigned connection point
    pub distributionlossfactor: Option<rust_decimal::Decimal>,
    /// Floored Offer/Bid Energy Price adjusted for TLF, DLF and MPF
    pub minimum_energy_price: Option<rust_decimal::Decimal>,
    /// Capped Offer/Bid Energy Price adjusted for TLF, DLF and VoLL
    pub maximum_energy_price: Option<rust_decimal::Decimal>,
    /// Scheduled status of the unit:<br>    'SCHEDULED'<br>    'NON-SCHEDULED'<br>    'SEMI-SCHEDULED'
    pub schedule_type: Option<String>,
    /// MW/Min. Calculated Minimum Ramp Rate Up value accepted for Energy Offers or Bids with explanation
    pub min_ramp_rate_up: Option<rust_decimal::Decimal>,
    /// MW/Min. Calculated Minimum Ramp Rate Down value accepted for Energy Offers or Bids with explanation
    pub min_ramp_rate_down: Option<rust_decimal::Decimal>,
    /// Maximum ramp up rate for Unit (Mw/min) - from DUDetail table
    pub max_ramp_rate_up: Option<rust_decimal::Decimal>,
    /// Maximum ramp down rate for Unit (Mw/min) - from DUDetail table
    pub max_ramp_rate_down: Option<rust_decimal::Decimal>,
    /// Whether the DUID is classified as an "Aggregated Unit" under the rules. This impacts the Minimum Ramp Rate calculation
    pub is_aggregated: Option<rust_decimal::Decimal>,
    /// Additional information for DISPATCHTYPE. For DISPATCHTYPE = LOAD, subtype value is WDR for wholesale demand response units For DISPATCHTYPE = LOAD, subtype value is NULL for Scheduled Loads. For DISPATCHTYPE = GENERATOR type, subtype value is NULL.
    pub dispatchsubtype: Option<String>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationDudetailsummary5 {
    type PrimaryKey = ParticipantRegistrationDudetailsummary5PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("DUDETAILSUMMARY".into()),
            version: 5,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationDudetailsummary5PrimaryKey {
        ParticipantRegistrationDudetailsummary5PrimaryKey {
            duid: self.duid.clone(),
            start_date: self.start_date
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_dudetailsummary_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationDudetailsummary5PrimaryKey {
    pub duid: String,
    pub start_date: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationDudetailsummary5PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationDudetailsummary5 {
    type Row = ParticipantRegistrationDudetailsummary5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
        && self.start_date == row.start_date
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationDudetailsummary5 {
    type PrimaryKey = ParticipantRegistrationDudetailsummary5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
        && self.start_date == key.start_date
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationDudetailsummary5PrimaryKey {
    type Row = ParticipantRegistrationDudetailsummary5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
        && self.start_date == row.start_date
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationDudetailsummary5PrimaryKey {
    type PrimaryKey = ParticipantRegistrationDudetailsummary5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
        && self.start_date == key.start_date
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationDudetailsummary5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("start_date", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("end_date", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("dispatchtype", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("connectionpointid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("stationid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("participantid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("transmissionlossfactor", arrow2::datatypes::DataType::Decimal(15,5), true),
            arrow2::datatypes::Field::new("starttype", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("distributionlossfactor", arrow2::datatypes::DataType::Decimal(15,5), true),
            arrow2::datatypes::Field::new("minimum_energy_price", arrow2::datatypes::DataType::Decimal(9,2), true),
            arrow2::datatypes::Field::new("maximum_energy_price", arrow2::datatypes::DataType::Decimal(9,2), true),
            arrow2::datatypes::Field::new("schedule_type", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("min_ramp_rate_up", arrow2::datatypes::DataType::Decimal(6,0), true),
            arrow2::datatypes::Field::new("min_ramp_rate_down", arrow2::datatypes::DataType::Decimal(6,0), true),
            arrow2::datatypes::Field::new("max_ramp_rate_up", arrow2::datatypes::DataType::Decimal(6,0), true),
            arrow2::datatypes::Field::new("max_ramp_rate_down", arrow2::datatypes::DataType::Decimal(6,0), true),
            arrow2::datatypes::Field::new("is_aggregated", arrow2::datatypes::DataType::Decimal(1,0), true),
            arrow2::datatypes::Field::new("dispatchsubtype", arrow2::datatypes::DataType::LargeUtf8, true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut duid_array = Vec::new();
        let mut start_date_array = Vec::new();
        let mut end_date_array = Vec::new();
        let mut dispatchtype_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut stationid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut transmissionlossfactor_array = Vec::new();
        let mut starttype_array = Vec::new();
        let mut distributionlossfactor_array = Vec::new();
        let mut minimum_energy_price_array = Vec::new();
        let mut maximum_energy_price_array = Vec::new();
        let mut schedule_type_array = Vec::new();
        let mut min_ramp_rate_up_array = Vec::new();
        let mut min_ramp_rate_down_array = Vec::new();
        let mut max_ramp_rate_up_array = Vec::new();
        let mut max_ramp_rate_down_array = Vec::new();
        let mut is_aggregated_array = Vec::new();
        let mut dispatchsubtype_array = Vec::new();
        for row in partition {
            duid_array.push(row.duid);
            start_date_array.push(row.start_date.timestamp());
            end_date_array.push(row.end_date.timestamp());
            dispatchtype_array.push(row.dispatchtype);
            connectionpointid_array.push(row.connectionpointid);
            regionid_array.push(row.regionid);
            stationid_array.push(row.stationid);
            participantid_array.push(row.participantid);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            transmissionlossfactor_array.push({
                        row.transmissionlossfactor.map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                    });
            starttype_array.push(row.starttype);
            distributionlossfactor_array.push({
                        row.distributionlossfactor.map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                    });
            minimum_energy_price_array.push({
                        row.minimum_energy_price.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            maximum_energy_price_array.push({
                        row.maximum_energy_price.map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                    });
            schedule_type_array.push(row.schedule_type);
            min_ramp_rate_up_array.push({
                        row.min_ramp_rate_up.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            min_ramp_rate_down_array.push({
                        row.min_ramp_rate_down.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            max_ramp_rate_up_array.push({
                        row.max_ramp_rate_up.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            max_ramp_rate_down_array.push({
                        row.max_ramp_rate_down.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            is_aggregated_array.push({
                        row.is_aggregated.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            dispatchsubtype_array.push(row.dispatchsubtype);
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(start_date_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(end_date_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(dispatchtype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(connectionpointid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(regionid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(stationid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(participantid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(transmissionlossfactor_array).to(arrow2::datatypes::DataType::Decimal(15,5))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(starttype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(distributionlossfactor_array).to(arrow2::datatypes::DataType::Decimal(15,5))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(minimum_energy_price_array).to(arrow2::datatypes::DataType::Decimal(9,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maximum_energy_price_array).to(arrow2::datatypes::DataType::Decimal(9,2))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(schedule_type_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(min_ramp_rate_up_array).to(arrow2::datatypes::DataType::Decimal(6,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(min_ramp_rate_down_array).to(arrow2::datatypes::DataType::Decimal(6,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(max_ramp_rate_up_array).to(arrow2::datatypes::DataType::Decimal(6,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(max_ramp_rate_down_array).to(arrow2::datatypes::DataType::Decimal(6,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(is_aggregated_array).to(arrow2::datatypes::DataType::Decimal(1,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(dispatchsubtype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## GENMETER
///  _GENMETER shows details of generator meter sets._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Genmeter
/// * Data Version: 1
/// 
/// # Description
///  GENMETER is a public table, and is available to all participants. Source GENMETER updates only when meter details change.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * APPLYDATE
/// * METERID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationGenmeter1 {
    /// Meter Id
    pub meterid: String,
    /// Generator Set ID
    pub gensetid: Option<String>,
    /// Not used
    pub connectionpointid: Option<String>,
    /// Station Identifier
    pub stationid: Option<String>,
    /// LOAD
    pub metertype: Option<String>,
    /// WATT or AUXILARY
    pub meterclass: Option<String>,
    /// Voltage
    pub voltagelevel: Option<rust_decimal::Decimal>,
    /// Application date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub applydate: chrono::NaiveDateTime,
    /// Version no of the record for the given effective date
    pub versionno: rust_decimal::Decimal,
    /// AEMO user authorising
    pub authorisedby: Option<String>,
    /// Date authorised
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Not used
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub comdate: Option<chrono::NaiveDateTime>,
    /// Not used
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub decomdate: Option<chrono::NaiveDateTime>,
    /// Not used
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Not used
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationGenmeter1 {
    type PrimaryKey = ParticipantRegistrationGenmeter1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("GENMETER".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationGenmeter1PrimaryKey {
        ParticipantRegistrationGenmeter1PrimaryKey {
            applydate: self.applydate,
            meterid: self.meterid.clone(),
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_genmeter_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationGenmeter1PrimaryKey {
    pub applydate: chrono::NaiveDateTime,
    pub meterid: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationGenmeter1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationGenmeter1 {
    type Row = ParticipantRegistrationGenmeter1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.applydate == row.applydate
        && self.meterid == row.meterid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationGenmeter1 {
    type PrimaryKey = ParticipantRegistrationGenmeter1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.applydate == key.applydate
        && self.meterid == key.meterid
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationGenmeter1PrimaryKey {
    type Row = ParticipantRegistrationGenmeter1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.applydate == row.applydate
        && self.meterid == row.meterid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationGenmeter1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationGenmeter1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.applydate == key.applydate
        && self.meterid == key.meterid
        && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationGenmeter1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("meterid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("gensetid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("connectionpointid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("stationid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("metertype", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("meterclass", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("voltagelevel", arrow2::datatypes::DataType::Decimal(6,0), true),
            arrow2::datatypes::Field::new("applydate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("authorisedby", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("authoriseddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("comdate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("decomdate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut meterid_array = Vec::new();
        let mut gensetid_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut stationid_array = Vec::new();
        let mut metertype_array = Vec::new();
        let mut meterclass_array = Vec::new();
        let mut voltagelevel_array = Vec::new();
        let mut applydate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut comdate_array = Vec::new();
        let mut decomdate_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            meterid_array.push(row.meterid);
            gensetid_array.push(row.gensetid);
            connectionpointid_array.push(row.connectionpointid);
            stationid_array.push(row.stationid);
            metertype_array.push(row.metertype);
            meterclass_array.push(row.meterclass);
            voltagelevel_array.push({
                        row.voltagelevel.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            applydate_array.push(row.applydate.timestamp());
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            comdate_array.push(row.comdate.map(|val| val.timestamp()));
            decomdate_array.push(row.decomdate.map(|val| val.timestamp()));
            enddate_array.push(row.enddate.map(|val| val.timestamp()));
            startdate_array.push(row.startdate.map(|val| val.timestamp()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(meterid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(gensetid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(connectionpointid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(stationid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(metertype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(meterclass_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(voltagelevel_array).to(arrow2::datatypes::DataType::Decimal(6,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(applydate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(comdate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(decomdate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(enddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(startdate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## GENUNITS
///  _GENUNITS shows Genset details for each physical unit with the relevant station.<br>_
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Genunits
/// * Data Version: 2
/// 
/// # Description
///  GENUNITS is a public table, and is available to all participants. Source GENUNITS updates whenever plant details change.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * GENSETID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationGenunits2 {
    /// Physical Unit identifier
    pub gensetid: String,
    /// Station Identifier
    pub stationid: Option<String>,
    /// Not used
    pub setlossfactor: Option<rust_decimal::Decimal>,
    /// Centrally dispatched Indicator
    pub cdindicator: Option<String>,
    /// AGC Available flag
    pub agcflag: Option<String>,
    /// Not used
    pub spinningflag: Option<String>,
    /// Voltage level
    pub voltlevel: Option<rust_decimal::Decimal>,
    /// Registered capacity
    pub registeredcapacity: Option<rust_decimal::Decimal>,
    /// Identifies LOAD or GENERATOR. This will likely expand to more generic models as new technology types are integrated into the NEM.
    pub dispatchtype: Option<String>,
    /// Fast / Slow / Not Dispatched
    pub starttype: Option<String>,
    /// Market Generator Indicator Flag
    pub mktgeneratorind: Option<String>,
    /// On / Off for load
    pub normalstatus: Option<String>,
    /// Maximum capacity
    pub maxcapacity: Option<rust_decimal::Decimal>,
    /// Genset type
    pub gensettype: Option<String>,
    /// Genset name
    pub gensetname: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The emissions factor for the generating unit, as calculated by Settlements staff members
    pub co2e_emissions_factor: Option<rust_decimal::Decimal>,
    /// The energy source for the generating unit, as used in the calculation of the CO2-e emissions factor.  Distinct from the Energy Source for a generating unit published as part of the Registration Master List
    pub co2e_energy_source: Option<String>,
    /// An indicator as to the source of the emission factor used in the calculation of the index. The applicable values for this field would be NTNDP which indicates the emission factor is quoted from the National Transmission Network Development Plan or Estimated to indicate the emission factor has been calculated using an internal AEMO procedure based upon the Department of Climate Change and Energy Efficiency NGA factors
    pub co2e_data_source: Option<String>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationGenunits2 {
    type PrimaryKey = ParticipantRegistrationGenunits2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("GENUNITS".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationGenunits2PrimaryKey {
        ParticipantRegistrationGenunits2PrimaryKey {
            gensetid: self.gensetid.clone()
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_genunits_v2".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationGenunits2PrimaryKey {
    pub gensetid: String,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationGenunits2PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationGenunits2 {
    type Row = ParticipantRegistrationGenunits2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.gensetid == row.gensetid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationGenunits2 {
    type PrimaryKey = ParticipantRegistrationGenunits2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.gensetid == key.gensetid
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationGenunits2PrimaryKey {
    type Row = ParticipantRegistrationGenunits2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.gensetid == row.gensetid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationGenunits2PrimaryKey {
    type PrimaryKey = ParticipantRegistrationGenunits2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.gensetid == key.gensetid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationGenunits2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("gensetid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("stationid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("setlossfactor", arrow2::datatypes::DataType::Decimal(16,6), true),
            arrow2::datatypes::Field::new("cdindicator", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("agcflag", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("spinningflag", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("voltlevel", arrow2::datatypes::DataType::Decimal(6,0), true),
            arrow2::datatypes::Field::new("registeredcapacity", arrow2::datatypes::DataType::Decimal(6,0), true),
            arrow2::datatypes::Field::new("dispatchtype", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("starttype", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("mktgeneratorind", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("normalstatus", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("maxcapacity", arrow2::datatypes::DataType::Decimal(6,0), true),
            arrow2::datatypes::Field::new("gensettype", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("gensetname", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("co2e_emissions_factor", arrow2::datatypes::DataType::Decimal(18,8), true),
            arrow2::datatypes::Field::new("co2e_energy_source", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("co2e_data_source", arrow2::datatypes::DataType::LargeUtf8, true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut gensetid_array = Vec::new();
        let mut stationid_array = Vec::new();
        let mut setlossfactor_array = Vec::new();
        let mut cdindicator_array = Vec::new();
        let mut agcflag_array = Vec::new();
        let mut spinningflag_array = Vec::new();
        let mut voltlevel_array = Vec::new();
        let mut registeredcapacity_array = Vec::new();
        let mut dispatchtype_array = Vec::new();
        let mut starttype_array = Vec::new();
        let mut mktgeneratorind_array = Vec::new();
        let mut normalstatus_array = Vec::new();
        let mut maxcapacity_array = Vec::new();
        let mut gensettype_array = Vec::new();
        let mut gensetname_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut co2e_emissions_factor_array = Vec::new();
        let mut co2e_energy_source_array = Vec::new();
        let mut co2e_data_source_array = Vec::new();
        for row in partition {
            gensetid_array.push(row.gensetid);
            stationid_array.push(row.stationid);
            setlossfactor_array.push({
                        row.setlossfactor.map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                    });
            cdindicator_array.push(row.cdindicator);
            agcflag_array.push(row.agcflag);
            spinningflag_array.push(row.spinningflag);
            voltlevel_array.push({
                        row.voltlevel.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            registeredcapacity_array.push({
                        row.registeredcapacity.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            dispatchtype_array.push(row.dispatchtype);
            starttype_array.push(row.starttype);
            mktgeneratorind_array.push(row.mktgeneratorind);
            normalstatus_array.push(row.normalstatus);
            maxcapacity_array.push({
                        row.maxcapacity.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            gensettype_array.push(row.gensettype);
            gensetname_array.push(row.gensetname);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            co2e_emissions_factor_array.push({
                        row.co2e_emissions_factor.map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                    });
            co2e_energy_source_array.push(row.co2e_energy_source);
            co2e_data_source_array.push(row.co2e_data_source);
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(gensetid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(stationid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(setlossfactor_array).to(arrow2::datatypes::DataType::Decimal(16,6))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(cdindicator_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(agcflag_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(spinningflag_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(voltlevel_array).to(arrow2::datatypes::DataType::Decimal(6,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(registeredcapacity_array).to(arrow2::datatypes::DataType::Decimal(6,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(dispatchtype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(starttype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(mktgeneratorind_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(normalstatus_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxcapacity_array).to(arrow2::datatypes::DataType::Decimal(6,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(gensettype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(gensetname_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(co2e_emissions_factor_array).to(arrow2::datatypes::DataType::Decimal(18,8))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(co2e_energy_source_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(co2e_data_source_array)) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## GENUNITS_UNIT
///  _Physical units within a Gen Unit Set_
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Genunits Unit
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
/// * GENSETID
/// * UNIT_GROUPING_LABEL
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationGenunitsUnit1 {
    /// System wide unique Generating Set ID
    pub gensetid: String,
    /// Effective Date of this detail record
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version with respect to the effective date
    pub versionno: rust_decimal::Decimal,
    /// Label of Physical Units within the station
    pub unit_grouping_label: String,
    /// Number of units in this Gen Unit grouping
    pub unit_count: Option<rust_decimal::Decimal>,
    /// Nameplate Capacity for each unit in this grouping
    pub unit_size: Option<rust_decimal::Decimal>,
    /// Maximum Capacity for each unit in this grouping
    pub unit_max_size: Option<rust_decimal::Decimal>,
    /// Indicator that Unit is part of an Aggregated Unit (at the DUID level)
    pub aggregation_flag: Option<rust_decimal::Decimal>,
    /// Date/Time when record was changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationGenunitsUnit1 {
    type PrimaryKey = ParticipantRegistrationGenunitsUnit1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("GENUNITS_UNIT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationGenunitsUnit1PrimaryKey {
        ParticipantRegistrationGenunitsUnit1PrimaryKey {
            effectivedate: self.effectivedate,
            gensetid: self.gensetid.clone(),
            unit_grouping_label: self.unit_grouping_label.clone(),
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_genunits_unit_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationGenunitsUnit1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub gensetid: String,
    pub unit_grouping_label: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationGenunitsUnit1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationGenunitsUnit1 {
    type Row = ParticipantRegistrationGenunitsUnit1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.gensetid == row.gensetid
        && self.unit_grouping_label == row.unit_grouping_label
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationGenunitsUnit1 {
    type PrimaryKey = ParticipantRegistrationGenunitsUnit1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.gensetid == key.gensetid
        && self.unit_grouping_label == key.unit_grouping_label
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationGenunitsUnit1PrimaryKey {
    type Row = ParticipantRegistrationGenunitsUnit1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.gensetid == row.gensetid
        && self.unit_grouping_label == row.unit_grouping_label
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationGenunitsUnit1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationGenunitsUnit1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.gensetid == key.gensetid
        && self.unit_grouping_label == key.unit_grouping_label
        && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationGenunitsUnit1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("gensetid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(6,0), false),
            arrow2::datatypes::Field::new("unit_grouping_label", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("unit_count", arrow2::datatypes::DataType::Decimal(10,0), true),
            arrow2::datatypes::Field::new("unit_size", arrow2::datatypes::DataType::Decimal(8,3), true),
            arrow2::datatypes::Field::new("unit_max_size", arrow2::datatypes::DataType::Decimal(8,3), true),
            arrow2::datatypes::Field::new("aggregation_flag", arrow2::datatypes::DataType::Decimal(1,0), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut gensetid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut unit_grouping_label_array = Vec::new();
        let mut unit_count_array = Vec::new();
        let mut unit_size_array = Vec::new();
        let mut unit_max_size_array = Vec::new();
        let mut aggregation_flag_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            gensetid_array.push(row.gensetid);
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            unit_grouping_label_array.push(row.unit_grouping_label);
            unit_count_array.push({
                        row.unit_count.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            unit_size_array.push({
                        row.unit_size.map(|mut val| {
                            val.rescale(3);
                            val.mantissa()
                        })
                    });
            unit_max_size_array.push({
                        row.unit_max_size.map(|mut val| {
                            val.rescale(3);
                            val.mantissa()
                        })
                    });
            aggregation_flag_array.push({
                        row.aggregation_flag.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(gensetid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(6,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(unit_grouping_label_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(unit_count_array).to(arrow2::datatypes::DataType::Decimal(10,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(unit_size_array).to(arrow2::datatypes::DataType::Decimal(8,3))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(unit_max_size_array).to(arrow2::datatypes::DataType::Decimal(8,3))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(aggregation_flag_array).to(arrow2::datatypes::DataType::Decimal(1,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## MNSP_INTERCONNECTOR
///  _MNSP_INTERCONNECTOR sets out attributes of each interconnector._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Mnsp Interconnector
/// * Data Version: 2
/// 
/// # Description
///  MNSP_INTERCONNECTOR data is public, so is available to all participants. Source MNSP_INTERCONNECTOR changes infrequently, typically annually. Volume Twice the number of MNSPs.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * LINKID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationMnspInterconnector2 {
    /// Identifier for each of the two MNSP Interconnector Links. Each link pertains to the direction from and to.
    pub linkid: String,
    /// Date when Interconnector becomes effective
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of data for other key data - a higher version for same key data will take precedence
    pub versionno: rust_decimal::Decimal,
    /// Interconnector Identifier
    pub interconnectorid: Option<String>,
    /// Nominated source region for Interconnector
    pub fromregion: Option<String>,
    /// Nominated destination region for Interconnector
    pub toregion: Option<String>,
    /// Maximum capacity
    pub maxcapacity: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor (redundant from May 2012)
    pub tlf: Option<rust_decimal::Decimal>,
    /// Factor applied to the LHS of constraint equations; set by AEMO
    pub lhsfactor: Option<rust_decimal::Decimal>,
    /// Obsolete; no longer applied. Ignore.
    pub meterflowconstant: Option<rust_decimal::Decimal>,
    /// Date of authorisation. Nominal date but required to enable Interconnector.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorising officer
    pub authorisedby: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Transmission Loss Factor for Link "From Region" end
    pub from_region_tlf: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor for Link at "To Region" end
    pub to_region_tlf: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationMnspInterconnector2 {
    type PrimaryKey = ParticipantRegistrationMnspInterconnector2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("MNSP_INTERCONNECTOR".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationMnspInterconnector2PrimaryKey {
        ParticipantRegistrationMnspInterconnector2PrimaryKey {
            effectivedate: self.effectivedate,
            linkid: self.linkid.clone(),
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_mnsp_interconnector_v2".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationMnspInterconnector2PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub linkid: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationMnspInterconnector2PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationMnspInterconnector2 {
    type Row = ParticipantRegistrationMnspInterconnector2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.linkid == row.linkid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationMnspInterconnector2 {
    type PrimaryKey = ParticipantRegistrationMnspInterconnector2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.linkid == key.linkid
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationMnspInterconnector2PrimaryKey {
    type Row = ParticipantRegistrationMnspInterconnector2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.linkid == row.linkid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationMnspInterconnector2PrimaryKey {
    type PrimaryKey = ParticipantRegistrationMnspInterconnector2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.linkid == key.linkid
        && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationMnspInterconnector2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("linkid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("interconnectorid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("fromregion", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("toregion", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("maxcapacity", arrow2::datatypes::DataType::Decimal(5,0), true),
            arrow2::datatypes::Field::new("tlf", arrow2::datatypes::DataType::Decimal(12,7), true),
            arrow2::datatypes::Field::new("lhsfactor", arrow2::datatypes::DataType::Decimal(12,7), true),
            arrow2::datatypes::Field::new("meterflowconstant", arrow2::datatypes::DataType::Decimal(12,7), true),
            arrow2::datatypes::Field::new("authoriseddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("authorisedby", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("from_region_tlf", arrow2::datatypes::DataType::Decimal(12,7), true),
            arrow2::datatypes::Field::new("to_region_tlf", arrow2::datatypes::DataType::Decimal(12,7), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut linkid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregion_array = Vec::new();
        let mut toregion_array = Vec::new();
        let mut maxcapacity_array = Vec::new();
        let mut tlf_array = Vec::new();
        let mut lhsfactor_array = Vec::new();
        let mut meterflowconstant_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut from_region_tlf_array = Vec::new();
        let mut to_region_tlf_array = Vec::new();
        for row in partition {
            linkid_array.push(row.linkid);
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            interconnectorid_array.push(row.interconnectorid);
            fromregion_array.push(row.fromregion);
            toregion_array.push(row.toregion);
            maxcapacity_array.push({
                        row.maxcapacity.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            tlf_array.push({
                        row.tlf.map(|mut val| {
                            val.rescale(7);
                            val.mantissa()
                        })
                    });
            lhsfactor_array.push({
                        row.lhsfactor.map(|mut val| {
                            val.rescale(7);
                            val.mantissa()
                        })
                    });
            meterflowconstant_array.push({
                        row.meterflowconstant.map(|mut val| {
                            val.rescale(7);
                            val.mantissa()
                        })
                    });
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            authorisedby_array.push(row.authorisedby);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            from_region_tlf_array.push({
                        row.from_region_tlf.map(|mut val| {
                            val.rescale(7);
                            val.mantissa()
                        })
                    });
            to_region_tlf_array.push({
                        row.to_region_tlf.map(|mut val| {
                            val.rescale(7);
                            val.mantissa()
                        })
                    });
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(linkid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(interconnectorid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(fromregion_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(toregion_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxcapacity_array).to(arrow2::datatypes::DataType::Decimal(5,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(tlf_array).to(arrow2::datatypes::DataType::Decimal(12,7))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lhsfactor_array).to(arrow2::datatypes::DataType::Decimal(12,7))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(meterflowconstant_array).to(arrow2::datatypes::DataType::Decimal(12,7))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(from_region_tlf_array).to(arrow2::datatypes::DataType::Decimal(12,7))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(to_region_tlf_array).to(arrow2::datatypes::DataType::Decimal(12,7))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## MNSP_PARTICIPANT
///  _MNSP_PARTICIPANT registers MNSP ownership._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Mnsp Participant
/// * Data Version: 1
/// 
/// # Description
///  MNSP_PARTICIPANT data is public, so is available to all participants. Source MNSP_PARTICIPANT updates infrequently, typically annually. Volume Number of MNSPs.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationMnspParticipant1 {
    /// Interconnector Identifier
    pub interconnectorid: String,
    /// Calendar date when Interconnector ownership becomes effective
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of data for other key data - a higher version for same key data takes precedence
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationMnspParticipant1 {
    type PrimaryKey = ParticipantRegistrationMnspParticipant1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("MNSP_PARTICIPANT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationMnspParticipant1PrimaryKey {
        ParticipantRegistrationMnspParticipant1PrimaryKey {
            effectivedate: self.effectivedate,
            interconnectorid: self.interconnectorid.clone(),
            participantid: self.participantid.clone(),
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_mnsp_participant_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationMnspParticipant1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub interconnectorid: String,
    pub participantid: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationMnspParticipant1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationMnspParticipant1 {
    type Row = ParticipantRegistrationMnspParticipant1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.interconnectorid == row.interconnectorid
        && self.participantid == row.participantid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationMnspParticipant1 {
    type PrimaryKey = ParticipantRegistrationMnspParticipant1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.interconnectorid == key.interconnectorid
        && self.participantid == key.participantid
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationMnspParticipant1PrimaryKey {
    type Row = ParticipantRegistrationMnspParticipant1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.interconnectorid == row.interconnectorid
        && self.participantid == row.participantid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationMnspParticipant1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationMnspParticipant1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.interconnectorid == key.interconnectorid
        && self.participantid == key.participantid
        && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationMnspParticipant1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("interconnectorid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("participantid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut interconnectorid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            interconnectorid_array.push(row.interconnectorid);
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            participantid_array.push(row.participantid);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(interconnectorid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(participantid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## PARTICIPANT
///  _PARTICIPANT sets out Participant ID, name and class for all participants._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Participant
/// * Data Version: 1
/// 
/// # Description
///  PARTICIPANT is public data, so is available to all participants. Source PARTICIPANT updates as new participants register or existing participants change details.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipant1 {
    /// Unique participant identifier
    pub participantid: String,
    /// Class of participant
    pub participantclassid: Option<String>,
    /// Full name of participant
    pub name: Option<String>,
    /// Not used
    pub description: Option<String>,
    /// Australian Company Number; Nine Numbers XXX-XXX-XXX
    pub acn: Option<String>,
    /// Identifies primary business activity of participant
    pub primarybusiness: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationParticipant1 {
    type PrimaryKey = ParticipantRegistrationParticipant1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("PARTICIPANT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationParticipant1PrimaryKey {
        ParticipantRegistrationParticipant1PrimaryKey {
            participantid: self.participantid.clone()
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_participant_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationParticipant1PrimaryKey {
    pub participantid: String,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationParticipant1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationParticipant1 {
    type Row = ParticipantRegistrationParticipant1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationParticipant1 {
    type PrimaryKey = ParticipantRegistrationParticipant1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationParticipant1PrimaryKey {
    type Row = ParticipantRegistrationParticipant1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationParticipant1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipant1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationParticipant1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("participantid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("participantclassid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("name", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("description", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("acn", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("primarybusiness", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut participantid_array = Vec::new();
        let mut participantclassid_array = Vec::new();
        let mut name_array = Vec::new();
        let mut description_array = Vec::new();
        let mut acn_array = Vec::new();
        let mut primarybusiness_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            participantid_array.push(row.participantid);
            participantclassid_array.push(row.participantclassid);
            name_array.push(row.name);
            description_array.push(row.description);
            acn_array.push(row.acn);
            primarybusiness_array.push(row.primarybusiness);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(participantid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(participantclassid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(name_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(acn_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(primarybusiness_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## PARTICIPANTACCOUNT
///  _PARTICIPANTACCOUNT shows financial details on participants._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Participantaccount
/// * Data Version: 1
/// 
/// # Description
///  PARTICIPANTACCOUNT data is confidential to the relevant participant. Source PARTICIPANTACCOUNT updates as new participants register or existing participants change details.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantaccount1 {
    /// Name of the account
    pub accountname: Option<String>,
    /// Unique participant identifier
    pub participantid: String,
    /// Account number
    pub accountnumber: Option<String>,
    /// Bank name
    pub bankname: Option<String>,
    /// Bank number
    pub banknumber: Option<rust_decimal::Decimal>,
    /// Branch name
    pub branchname: Option<String>,
    /// Branch number
    pub branchnumber: Option<rust_decimal::Decimal>,
    /// BSB number
    pub bsbnumber: Option<String>,
    /// AEMO credit account number
    pub nemmcocreditaccountnumber: Option<rust_decimal::Decimal>,
    /// AEMO debit account number
    pub nemmcodebitaccountnumber: Option<rust_decimal::Decimal>,
    /// User authorising record
    pub authorisedby: Option<String>,
    /// Authorised date
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Date record authorised
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Australian Business Number
    pub abn: Option<String>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationParticipantaccount1 {
    type PrimaryKey = ParticipantRegistrationParticipantaccount1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("PARTICIPANTACCOUNT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationParticipantaccount1PrimaryKey {
        ParticipantRegistrationParticipantaccount1PrimaryKey {
            participantid: self.participantid.clone()
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_participantaccount_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationParticipantaccount1PrimaryKey {
    pub participantid: String,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationParticipantaccount1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationParticipantaccount1 {
    type Row = ParticipantRegistrationParticipantaccount1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationParticipantaccount1 {
    type PrimaryKey = ParticipantRegistrationParticipantaccount1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationParticipantaccount1PrimaryKey {
    type Row = ParticipantRegistrationParticipantaccount1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationParticipantaccount1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipantaccount1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationParticipantaccount1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("accountname", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("participantid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("accountnumber", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("bankname", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("banknumber", arrow2::datatypes::DataType::Decimal(10,0), true),
            arrow2::datatypes::Field::new("branchname", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("branchnumber", arrow2::datatypes::DataType::Decimal(10,0), true),
            arrow2::datatypes::Field::new("bsbnumber", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("nemmcocreditaccountnumber", arrow2::datatypes::DataType::Decimal(10,0), true),
            arrow2::datatypes::Field::new("nemmcodebitaccountnumber", arrow2::datatypes::DataType::Decimal(10,0), true),
            arrow2::datatypes::Field::new("authorisedby", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("authoriseddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("abn", arrow2::datatypes::DataType::LargeUtf8, true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut accountname_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut accountnumber_array = Vec::new();
        let mut bankname_array = Vec::new();
        let mut banknumber_array = Vec::new();
        let mut branchname_array = Vec::new();
        let mut branchnumber_array = Vec::new();
        let mut bsbnumber_array = Vec::new();
        let mut nemmcocreditaccountnumber_array = Vec::new();
        let mut nemmcodebitaccountnumber_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut abn_array = Vec::new();
        for row in partition {
            accountname_array.push(row.accountname);
            participantid_array.push(row.participantid);
            accountnumber_array.push(row.accountnumber);
            bankname_array.push(row.bankname);
            banknumber_array.push({
                        row.banknumber.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            branchname_array.push(row.branchname);
            branchnumber_array.push({
                        row.branchnumber.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            bsbnumber_array.push(row.bsbnumber);
            nemmcocreditaccountnumber_array.push({
                        row.nemmcocreditaccountnumber.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            nemmcodebitaccountnumber_array.push({
                        row.nemmcodebitaccountnumber.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            effectivedate_array.push(row.effectivedate.map(|val| val.timestamp()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            abn_array.push(row.abn);
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(accountname_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(participantid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(accountnumber_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(bankname_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(banknumber_array).to(arrow2::datatypes::DataType::Decimal(10,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(branchname_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(branchnumber_array).to(arrow2::datatypes::DataType::Decimal(10,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(bsbnumber_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(nemmcocreditaccountnumber_array).to(arrow2::datatypes::DataType::Decimal(10,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(nemmcodebitaccountnumber_array).to(arrow2::datatypes::DataType::Decimal(10,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(abn_array)) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## PARTICIPANTCATEGORY
///  _PARTICIPANTCATEGORY sets out valid participant categories._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Participantcategory
/// * Data Version: 1
/// 
/// # Description
///  PARTICIPANTCATEGORY is public data, so is available to all participants. Source PARTICIPANTCATEGORY updates as categories change. PARTICIPANTCATEGORY changes infrequently.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTCATEGORYID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantcategory1 {
    /// Participant category identifier
    pub participantcategoryid: String,
    /// Category description
    pub description: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationParticipantcategory1 {
    type PrimaryKey = ParticipantRegistrationParticipantcategory1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("PARTICIPANTCATEGORY".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationParticipantcategory1PrimaryKey {
        ParticipantRegistrationParticipantcategory1PrimaryKey {
            participantcategoryid: self.participantcategoryid.clone()
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_participantcategory_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationParticipantcategory1PrimaryKey {
    pub participantcategoryid: String,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationParticipantcategory1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationParticipantcategory1 {
    type Row = ParticipantRegistrationParticipantcategory1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantcategoryid == row.participantcategoryid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationParticipantcategory1 {
    type PrimaryKey = ParticipantRegistrationParticipantcategory1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantcategoryid == key.participantcategoryid
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationParticipantcategory1PrimaryKey {
    type Row = ParticipantRegistrationParticipantcategory1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantcategoryid == row.participantcategoryid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationParticipantcategory1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipantcategory1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantcategoryid == key.participantcategoryid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationParticipantcategory1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("participantcategoryid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("description", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut participantcategoryid_array = Vec::new();
        let mut description_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            participantcategoryid_array.push(row.participantcategoryid);
            description_array.push(row.description);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(participantcategoryid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## PARTICIPANTCATEGORYALLOC
///  _PARTICIPANTCATEGORYALLOC sets out the assignment of participants to particular categories._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Participantcategoryalloc
/// * Data Version: 1
/// 
/// # Description
///  PARTICIPANTCATEGORYALLOC data is public, so is available to all participants. Source PARTICIPANTCATEGORYALLOC updates for new participants or when categories change. PARTICIPANTCATEGORYALLOC changes infrequently.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTCATEGORYID
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantcategoryalloc1 {
    /// Category unique identifier
    pub participantcategoryid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationParticipantcategoryalloc1 {
    type PrimaryKey = ParticipantRegistrationParticipantcategoryalloc1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("PARTICIPANTCATEGORYALLOC".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {
        ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {
            participantcategoryid: self.participantcategoryid.clone(),
            participantid: self.participantid.clone()
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_participantcategoryalloc_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {
    pub participantcategoryid: String,
    pub participantid: String,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationParticipantcategoryalloc1 {
    type Row = ParticipantRegistrationParticipantcategoryalloc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantcategoryid == row.participantcategoryid
        && self.participantid == row.participantid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationParticipantcategoryalloc1 {
    type PrimaryKey = ParticipantRegistrationParticipantcategoryalloc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantcategoryid == key.participantcategoryid
        && self.participantid == key.participantid
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {
    type Row = ParticipantRegistrationParticipantcategoryalloc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantcategoryid == row.participantcategoryid
        && self.participantid == row.participantid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipantcategoryalloc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantcategoryid == key.participantcategoryid
        && self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationParticipantcategoryalloc1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("participantcategoryid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("participantid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut participantcategoryid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            participantcategoryid_array.push(row.participantcategoryid);
            participantid_array.push(row.participantid);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(participantcategoryid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(participantid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## PARTICIPANTCLASS
///  _PARTICIPANTCLASS sets out valid participant classifications._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Participantclass
/// * Data Version: 1
/// 
/// # Description
///  PARTICIPANTCLASS data is public, so is available to all participants. Source PARTICIPANTCLASS updates only if classifications change. This table changes infrequently.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * PARTICIPANTCLASSID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantclass1 {
    /// Class of participant
    pub participantclassid: String,
    /// Description of participant class
    pub description: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationParticipantclass1 {
    type PrimaryKey = ParticipantRegistrationParticipantclass1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("PARTICIPANTCLASS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationParticipantclass1PrimaryKey {
        ParticipantRegistrationParticipantclass1PrimaryKey {
            participantclassid: self.participantclassid.clone()
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_participantclass_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationParticipantclass1PrimaryKey {
    pub participantclassid: String,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationParticipantclass1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationParticipantclass1 {
    type Row = ParticipantRegistrationParticipantclass1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantclassid == row.participantclassid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationParticipantclass1 {
    type PrimaryKey = ParticipantRegistrationParticipantclass1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantclassid == key.participantclassid
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationParticipantclass1PrimaryKey {
    type Row = ParticipantRegistrationParticipantclass1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantclassid == row.participantclassid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationParticipantclass1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipantclass1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantclassid == key.participantclassid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationParticipantclass1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("participantclassid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("description", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut participantclassid_array = Vec::new();
        let mut description_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            participantclassid_array.push(row.participantclassid);
            description_array.push(row.description);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(participantclassid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## PARTICIPANTCREDITDETAIL
///  _&nbsp; _
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Participantcreditdetail
/// * Data Version: 1
/// 
/// # Description
///  PARTICIPANTCREDITDETAIL data is confidential to each participant. Source PARTICIPANTCREDITDETAIL updates infrequently.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationParticipantcreditdetail1 {
    /// &nbsp; 
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// &nbsp; 
    pub participantid: String,
    /// &nbsp; 
    pub creditlimit: Option<rust_decimal::Decimal>,
    /// &nbsp; 
    pub authorisedby: Option<String>,
    /// &nbsp; 
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationParticipantcreditdetail1 {
    type PrimaryKey = ParticipantRegistrationParticipantcreditdetail1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("PARTICIPANTCREDITDETAIL".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationParticipantcreditdetail1PrimaryKey {
        ParticipantRegistrationParticipantcreditdetail1PrimaryKey {
            effectivedate: self.effectivedate,
            participantid: self.participantid.clone()
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_participantcreditdetail_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationParticipantcreditdetail1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: String,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationParticipantcreditdetail1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationParticipantcreditdetail1 {
    type Row = ParticipantRegistrationParticipantcreditdetail1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.participantid == row.participantid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationParticipantcreditdetail1 {
    type PrimaryKey = ParticipantRegistrationParticipantcreditdetail1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.participantid == key.participantid
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationParticipantcreditdetail1PrimaryKey {
    type Row = ParticipantRegistrationParticipantcreditdetail1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.participantid == row.participantid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationParticipantcreditdetail1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipantcreditdetail1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.participantid == key.participantid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationParticipantcreditdetail1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("participantid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("creditlimit", arrow2::datatypes::DataType::Decimal(10,0), true),
            arrow2::datatypes::Field::new("authorisedby", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("authoriseddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut effectivedate_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut creditlimit_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            participantid_array.push(row.participantid);
            creditlimit_array.push({
                        row.creditlimit.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(participantid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(creditlimit_array).to(arrow2::datatypes::DataType::Decimal(10,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## PMS_GROUP
///  _Entity table for group_
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Pms Group
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * GROUPID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationPmsGroup1 {
    /// Abstract identifier for the group
    pub groupid: rust_decimal::Decimal,
    /// Date record was created
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub createddate: Option<chrono::NaiveDateTime>,
    /// Date record was last changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationPmsGroup1 {
    type PrimaryKey = ParticipantRegistrationPmsGroup1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("PMS_GROUP".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationPmsGroup1PrimaryKey {
        ParticipantRegistrationPmsGroup1PrimaryKey {
            groupid: self.groupid
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_pms_group_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationPmsGroup1PrimaryKey {
    pub groupid: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationPmsGroup1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationPmsGroup1 {
    type Row = ParticipantRegistrationPmsGroup1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.groupid == row.groupid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationPmsGroup1 {
    type PrimaryKey = ParticipantRegistrationPmsGroup1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.groupid == key.groupid
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationPmsGroup1PrimaryKey {
    type Row = ParticipantRegistrationPmsGroup1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.groupid == row.groupid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationPmsGroup1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationPmsGroup1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.groupid == key.groupid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationPmsGroup1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("groupid", arrow2::datatypes::DataType::Decimal(20,0), false),
            arrow2::datatypes::Field::new("createddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut groupid_array = Vec::new();
        let mut createddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            groupid_array.push({
                        let mut val = row.groupid;
                        val.rescale(0);
                        val.mantissa()
                    });
            createddate_array.push(row.createddate.map(|val| val.timestamp()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(groupid_array).to(arrow2::datatypes::DataType::Decimal(20,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(createddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## PMS_GROUPNMI
///  _Describe the NMIs that a group uses to provide its service_
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Pms Groupnmi
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Private
/// 
/// # Primary Key Columns
/// 
/// * GROUPNMIID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationPmsGroupnmi1 {
    /// Record Identifier of the NMI within a Group. When data is updated, existing record identifier is terminated, and new record identifier(s) are allocated.
    pub groupnmiid: rust_decimal::Decimal,
    /// Group id of the Group which the NMI belongs in.
    pub groupid: Option<rust_decimal::Decimal>,
    /// Date for which this version is effective from
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub versionfrom: Option<chrono::NaiveDateTime>,
    /// Date for which this version is effective to. Will be set to current day plus one if it is the current active record or past date if the record has been superseded/ended.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub versionto: Option<chrono::NaiveDateTime>,
    /// Effective date of when this service started operation
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Date for which this version is effective to. Will be set to current day plus one if it is the current active record or past date if the record has been superseded/ended.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// National Meter Identifier linked to the group.
    pub nmi: Option<String>,
    /// Site name
    pub sitename: Option<String>,
    /// Specifies whether NMI is in a NERR aggregated premises (TRUE = 1/FALSE = 0)
    pub nerrgrouppremises: Option<rust_decimal::Decimal>,
    /// Baseline methodology to be used for the PoL and Baseline assessment of the NMI
    pub baselinemethodologyid: Option<String>,
    /// Maximum responsive component for the NMI
    pub mrc: Option<rust_decimal::Decimal>,
    /// Reason for the MRC
    pub mrcreason: Option<String>,
    /// Retail customer of the NMI
    pub retailcustomer: Option<String>,
    /// Indicates whether the NMI has been suspended from use. (TRUE = 1/FALSE = 0)
    pub suspended: Option<rust_decimal::Decimal>,
    /// Indicates whether the NMI is unavailable for use. (TRUE = 1/FALSE = 0)
    pub unavailable: Option<rust_decimal::Decimal>,
    /// Date which this record was approved
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub approveddate: Option<chrono::NaiveDateTime>,
    /// Date time which record was last changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationPmsGroupnmi1 {
    type PrimaryKey = ParticipantRegistrationPmsGroupnmi1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("PMS_GROUPNMI".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationPmsGroupnmi1PrimaryKey {
        ParticipantRegistrationPmsGroupnmi1PrimaryKey {
            groupnmiid: self.groupnmiid
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_pms_groupnmi_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationPmsGroupnmi1PrimaryKey {
    pub groupnmiid: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationPmsGroupnmi1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationPmsGroupnmi1 {
    type Row = ParticipantRegistrationPmsGroupnmi1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.groupnmiid == row.groupnmiid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationPmsGroupnmi1 {
    type PrimaryKey = ParticipantRegistrationPmsGroupnmi1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.groupnmiid == key.groupnmiid
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationPmsGroupnmi1PrimaryKey {
    type Row = ParticipantRegistrationPmsGroupnmi1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.groupnmiid == row.groupnmiid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationPmsGroupnmi1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationPmsGroupnmi1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.groupnmiid == key.groupnmiid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationPmsGroupnmi1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("groupnmiid", arrow2::datatypes::DataType::Decimal(20,0), false),
            arrow2::datatypes::Field::new("groupid", arrow2::datatypes::DataType::Decimal(20,0), true),
            arrow2::datatypes::Field::new("versionfrom", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("versionto", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("nmi", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("sitename", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("nerrgrouppremises", arrow2::datatypes::DataType::Decimal(1,0), true),
            arrow2::datatypes::Field::new("baselinemethodologyid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("mrc", arrow2::datatypes::DataType::Decimal(10,3), true),
            arrow2::datatypes::Field::new("mrcreason", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("retailcustomer", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("suspended", arrow2::datatypes::DataType::Decimal(1,0), true),
            arrow2::datatypes::Field::new("unavailable", arrow2::datatypes::DataType::Decimal(1,0), true),
            arrow2::datatypes::Field::new("approveddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut groupnmiid_array = Vec::new();
        let mut groupid_array = Vec::new();
        let mut versionfrom_array = Vec::new();
        let mut versionto_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut nmi_array = Vec::new();
        let mut sitename_array = Vec::new();
        let mut nerrgrouppremises_array = Vec::new();
        let mut baselinemethodologyid_array = Vec::new();
        let mut mrc_array = Vec::new();
        let mut mrcreason_array = Vec::new();
        let mut retailcustomer_array = Vec::new();
        let mut suspended_array = Vec::new();
        let mut unavailable_array = Vec::new();
        let mut approveddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            groupnmiid_array.push({
                        let mut val = row.groupnmiid;
                        val.rescale(0);
                        val.mantissa()
                    });
            groupid_array.push({
                        row.groupid.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            versionfrom_array.push(row.versionfrom.map(|val| val.timestamp()));
            versionto_array.push(row.versionto.map(|val| val.timestamp()));
            startdate_array.push(row.startdate.map(|val| val.timestamp()));
            enddate_array.push(row.enddate.map(|val| val.timestamp()));
            nmi_array.push(row.nmi);
            sitename_array.push(row.sitename);
            nerrgrouppremises_array.push({
                        row.nerrgrouppremises.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            baselinemethodologyid_array.push(row.baselinemethodologyid);
            mrc_array.push({
                        row.mrc.map(|mut val| {
                            val.rescale(3);
                            val.mantissa()
                        })
                    });
            mrcreason_array.push(row.mrcreason);
            retailcustomer_array.push(row.retailcustomer);
            suspended_array.push({
                        row.suspended.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            unavailable_array.push({
                        row.unavailable.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            approveddate_array.push(row.approveddate.map(|val| val.timestamp()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(groupnmiid_array).to(arrow2::datatypes::DataType::Decimal(20,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(groupid_array).to(arrow2::datatypes::DataType::Decimal(20,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(versionfrom_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(versionto_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(startdate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(enddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(nmi_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(sitename_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(nerrgrouppremises_array).to(arrow2::datatypes::DataType::Decimal(1,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(baselinemethodologyid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(mrc_array).to(arrow2::datatypes::DataType::Decimal(10,3))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(mrcreason_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(retailcustomer_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(suspended_array).to(arrow2::datatypes::DataType::Decimal(1,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(unavailable_array).to(arrow2::datatypes::DataType::Decimal(1,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(approveddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## PMS_GROUPSERVICE
///  _Describe the services a group provides and its relation to a market_
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Pms Groupservice
/// * Data Version: 1
/// 
/// 
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * GROUPSERVICEID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationPmsGroupservice1 {
    /// Record identifier of the Service allocated to the Group. When data is updated, existing record identifier is terminated, and new record identifier(s) are allocated.
    pub groupserviceid: rust_decimal::Decimal,
    /// Group id of the Group where the Service is attached to.
    pub groupid: Option<rust_decimal::Decimal>,
    /// Date for which this version is effective from.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub versionfrom: Option<chrono::NaiveDateTime>,
    /// Date for which this version is effective to. Will be set to max date 9999/12/31 23:59:59.999 until this version ends or a change to the version is required.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub versionto: Option<chrono::NaiveDateTime>,
    /// Effective date of when this service started operation
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Effective date of when this service ended operation. Will be set to max date 9999/12/31 23:59:59.999 until its service ends or a change to the service is required.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Market that this group is operating its service in. Will only be NEM initially.
    pub market: Option<String>,
    /// Service that this group is operating. Will be only be ENERGY initially
    pub servicetype: Option<String>,
    /// Describes the entity that is operating. Will only be WDRU initially.
    pub entitytype: Option<String>,
    /// Describe the entity ID in the market that it will be operating in. Will only contain the DUID of the group initially.
    pub entityid: Option<String>,
    /// Maximum responsive component for the service offering
    pub mrc: Option<rust_decimal::Decimal>,
    /// Reason for the MRC.
    pub mrcreason: Option<String>,
    /// Maximum ramp rate MW per minute of the service.
    pub maximumrampratepermin: Option<rust_decimal::Decimal>,
    /// Region the group is operating this service in One of NSW1, QLD1, VIC1, SA1 or TAS1
    pub region: Option<String>,
    /// Date which this record was approved
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub approveddate: Option<chrono::NaiveDateTime>,
    /// Date time which record was last changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationPmsGroupservice1 {
    type PrimaryKey = ParticipantRegistrationPmsGroupservice1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("PMS_GROUPSERVICE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationPmsGroupservice1PrimaryKey {
        ParticipantRegistrationPmsGroupservice1PrimaryKey {
            groupserviceid: self.groupserviceid
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_pms_groupservice_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationPmsGroupservice1PrimaryKey {
    pub groupserviceid: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationPmsGroupservice1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationPmsGroupservice1 {
    type Row = ParticipantRegistrationPmsGroupservice1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.groupserviceid == row.groupserviceid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationPmsGroupservice1 {
    type PrimaryKey = ParticipantRegistrationPmsGroupservice1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.groupserviceid == key.groupserviceid
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationPmsGroupservice1PrimaryKey {
    type Row = ParticipantRegistrationPmsGroupservice1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.groupserviceid == row.groupserviceid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationPmsGroupservice1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationPmsGroupservice1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.groupserviceid == key.groupserviceid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationPmsGroupservice1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("groupserviceid", arrow2::datatypes::DataType::Decimal(20,0), false),
            arrow2::datatypes::Field::new("groupid", arrow2::datatypes::DataType::Decimal(20,0), true),
            arrow2::datatypes::Field::new("versionfrom", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("versionto", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("market", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("servicetype", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("entitytype", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("entityid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("mrc", arrow2::datatypes::DataType::Decimal(10,3), true),
            arrow2::datatypes::Field::new("mrcreason", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("maximumrampratepermin", arrow2::datatypes::DataType::Decimal(10,0), true),
            arrow2::datatypes::Field::new("region", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("approveddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut groupserviceid_array = Vec::new();
        let mut groupid_array = Vec::new();
        let mut versionfrom_array = Vec::new();
        let mut versionto_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut market_array = Vec::new();
        let mut servicetype_array = Vec::new();
        let mut entitytype_array = Vec::new();
        let mut entityid_array = Vec::new();
        let mut mrc_array = Vec::new();
        let mut mrcreason_array = Vec::new();
        let mut maximumrampratepermin_array = Vec::new();
        let mut region_array = Vec::new();
        let mut approveddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            groupserviceid_array.push({
                        let mut val = row.groupserviceid;
                        val.rescale(0);
                        val.mantissa()
                    });
            groupid_array.push({
                        row.groupid.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            versionfrom_array.push(row.versionfrom.map(|val| val.timestamp()));
            versionto_array.push(row.versionto.map(|val| val.timestamp()));
            startdate_array.push(row.startdate.map(|val| val.timestamp()));
            enddate_array.push(row.enddate.map(|val| val.timestamp()));
            market_array.push(row.market);
            servicetype_array.push(row.servicetype);
            entitytype_array.push(row.entitytype);
            entityid_array.push(row.entityid);
            mrc_array.push({
                        row.mrc.map(|mut val| {
                            val.rescale(3);
                            val.mantissa()
                        })
                    });
            mrcreason_array.push(row.mrcreason);
            maximumrampratepermin_array.push({
                        row.maximumrampratepermin.map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                    });
            region_array.push(row.region);
            approveddate_array.push(row.approveddate.map(|val| val.timestamp()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(groupserviceid_array).to(arrow2::datatypes::DataType::Decimal(20,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(groupid_array).to(arrow2::datatypes::DataType::Decimal(20,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(versionfrom_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(versionto_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(startdate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(enddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(market_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(servicetype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(entitytype_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(entityid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(mrc_array).to(arrow2::datatypes::DataType::Decimal(10,3))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(mrcreason_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maximumrampratepermin_array).to(arrow2::datatypes::DataType::Decimal(10,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(region_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(approveddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## STADUALLOC
///  _STADUALLOC sets out details on the allocation of dispatchable units to particular sites or stations._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Stadualloc
/// * Data Version: 1
/// 
/// # Description
///  STADUALLOC is public data, and is available to all participants. Source STADUALLOC is updated whenever there is a station configuration change or new unit registration.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * DUID
/// * EFFECTIVEDATE
/// * STATIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStadualloc1 {
    /// Dispatchable Unit Identifier
    pub duid: String,
    /// Effective date of this record
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Station Identifier
    pub stationid: String,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationStadualloc1 {
    type PrimaryKey = ParticipantRegistrationStadualloc1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("STADUALLOC".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationStadualloc1PrimaryKey {
        ParticipantRegistrationStadualloc1PrimaryKey {
            duid: self.duid.clone(),
            effectivedate: self.effectivedate,
            stationid: self.stationid.clone(),
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_stadualloc_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationStadualloc1PrimaryKey {
    pub duid: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub stationid: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationStadualloc1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationStadualloc1 {
    type Row = ParticipantRegistrationStadualloc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
        && self.effectivedate == row.effectivedate
        && self.stationid == row.stationid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationStadualloc1 {
    type PrimaryKey = ParticipantRegistrationStadualloc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
        && self.effectivedate == key.effectivedate
        && self.stationid == key.stationid
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationStadualloc1PrimaryKey {
    type Row = ParticipantRegistrationStadualloc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
        && self.effectivedate == row.effectivedate
        && self.stationid == row.stationid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationStadualloc1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationStadualloc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
        && self.effectivedate == key.effectivedate
        && self.stationid == key.stationid
        && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationStadualloc1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("stationid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut duid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut stationid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            duid_array.push(row.duid);
            effectivedate_array.push(row.effectivedate.timestamp());
            stationid_array.push(row.stationid);
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(stationid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## STATION
///  _STATION sets out valid station identifiers._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Station
/// * Data Version: 1
/// 
/// # Description
///  STATION is public data, and is available to all participants. Source STATION updates whenever there is a station configuration change or new unit registration.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * STATIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStation1 {
    /// Station Identifier
    pub stationid: String,
    /// Full name of station
    pub stationname: Option<String>,
    /// Station Address
    pub address1: Option<String>,
    /// Station Address
    pub address2: Option<String>,
    /// Station Address
    pub address3: Option<String>,
    /// Station Address
    pub address4: Option<String>,
    /// City
    pub city: Option<String>,
    /// State of Australia
    pub state: Option<String>,
    /// Post Code
    pub postcode: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Not used. Do not use as the Connection Point Identifier for station load
    pub connectionpointid: Option<String>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationStation1 {
    type PrimaryKey = ParticipantRegistrationStation1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("STATION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationStation1PrimaryKey {
        ParticipantRegistrationStation1PrimaryKey {
            stationid: self.stationid.clone()
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_station_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationStation1PrimaryKey {
    pub stationid: String,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationStation1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationStation1 {
    type Row = ParticipantRegistrationStation1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.stationid == row.stationid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationStation1 {
    type PrimaryKey = ParticipantRegistrationStation1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.stationid == key.stationid
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationStation1PrimaryKey {
    type Row = ParticipantRegistrationStation1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.stationid == row.stationid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationStation1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationStation1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.stationid == key.stationid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationStation1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("stationid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("stationname", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("address1", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("address2", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("address3", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("address4", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("city", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("state", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("postcode", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("connectionpointid", arrow2::datatypes::DataType::LargeUtf8, true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut stationid_array = Vec::new();
        let mut stationname_array = Vec::new();
        let mut address1_array = Vec::new();
        let mut address2_array = Vec::new();
        let mut address3_array = Vec::new();
        let mut address4_array = Vec::new();
        let mut city_array = Vec::new();
        let mut state_array = Vec::new();
        let mut postcode_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        for row in partition {
            stationid_array.push(row.stationid);
            stationname_array.push(row.stationname);
            address1_array.push(row.address1);
            address2_array.push(row.address2);
            address3_array.push(row.address3);
            address4_array.push(row.address4);
            city_array.push(row.city);
            state_array.push(row.state);
            postcode_array.push(row.postcode);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            connectionpointid_array.push(row.connectionpointid);
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(stationid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(stationname_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(address1_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(address2_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(address3_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(address4_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(city_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(state_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(postcode_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(connectionpointid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## STATIONOPERATINGSTATUS
///  _STATIONOPERATINGSTATUS sets out the operating status of each station._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Stationoperatingstatus
/// * Data Version: 1
/// 
/// # Description
///  STATIONOWNER is public data, and is available to all participants. Source STATIONOWNER is updated whenever there is a change in the station owner or new units are registered.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * STATIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStationoperatingstatus1 {
    /// Effective date of this record
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Unique station identifier
    pub stationid: String,
    /// Version no of record within the effective date
    pub versionno: rust_decimal::Decimal,
    /// The operating status of this station, valid values are COMMISSIONED and DECOMMISSIONED
    pub status: Option<String>,
    /// User authorising record
    pub authorisedby: Option<String>,
    /// Date record authorised
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationStationoperatingstatus1 {
    type PrimaryKey = ParticipantRegistrationStationoperatingstatus1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("STATIONOPERATINGSTATUS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationStationoperatingstatus1PrimaryKey {
        ParticipantRegistrationStationoperatingstatus1PrimaryKey {
            effectivedate: self.effectivedate,
            stationid: self.stationid.clone(),
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_stationoperatingstatus_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationStationoperatingstatus1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub stationid: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationStationoperatingstatus1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationStationoperatingstatus1 {
    type Row = ParticipantRegistrationStationoperatingstatus1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.stationid == row.stationid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationStationoperatingstatus1 {
    type PrimaryKey = ParticipantRegistrationStationoperatingstatus1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.stationid == key.stationid
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationStationoperatingstatus1PrimaryKey {
    type Row = ParticipantRegistrationStationoperatingstatus1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.stationid == row.stationid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationStationoperatingstatus1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationStationoperatingstatus1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.stationid == key.stationid
        && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationStationoperatingstatus1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("stationid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("status", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("authorisedby", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("authoriseddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut effectivedate_array = Vec::new();
        let mut stationid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut status_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            stationid_array.push(row.stationid);
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            status_array.push(row.status);
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(stationid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(status_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## STATIONOWNER
///  _STATIONOWNER sets out the owner details of each station._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Stationowner
/// * Data Version: 1
/// 
/// # Description
///  STATIONOWNER is public data, and is available to all participants. Source STATIONOWNER is updated whenever there is a change in the station owner or new units are registered.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * PARTICIPANTID
/// * STATIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStationowner1 {
    /// Effective date of this record
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Unique participant identifier
    pub participantid: String,
    /// Station Identifier
    pub stationid: String,
    /// Version no of record within the effective date
    pub versionno: rust_decimal::Decimal,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationStationowner1 {
    type PrimaryKey = ParticipantRegistrationStationowner1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("STATIONOWNER".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationStationowner1PrimaryKey {
        ParticipantRegistrationStationowner1PrimaryKey {
            effectivedate: self.effectivedate,
            participantid: self.participantid.clone(),
            stationid: self.stationid.clone(),
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_stationowner_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationStationowner1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: String,
    pub stationid: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationStationowner1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationStationowner1 {
    type Row = ParticipantRegistrationStationowner1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.participantid == row.participantid
        && self.stationid == row.stationid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationStationowner1 {
    type PrimaryKey = ParticipantRegistrationStationowner1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.participantid == key.participantid
        && self.stationid == key.stationid
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationStationowner1PrimaryKey {
    type Row = ParticipantRegistrationStationowner1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.participantid == row.participantid
        && self.stationid == row.stationid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationStationowner1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationStationowner1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.participantid == key.participantid
        && self.stationid == key.stationid
        && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationStationowner1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("participantid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("stationid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut effectivedate_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut stationid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            participantid_array.push(row.participantid);
            stationid_array.push(row.stationid);
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(participantid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(stationid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
/// # Summary
/// 
/// ## STATIONOWNERTRK
///  _STATIONOWNERTRK shows the tracking for the associated object STATIONOWNER. Together, STATIONOWNERTRK and STATIONOWNER sets out the owner details of each station._
/// 
/// * Data Set Name: Participant Registration
/// * File Name: Stationownertrk
/// * Data Version: 1
/// 
/// # Description
///  STATIONOWNER is public data, and is available to all participants. Source STATIONOWNER is updated whenever there is a change in the station owner or new units are registered.
/// 
/// # Notes
///  * (Visibility) Data in this table is: Public
/// 
/// # Primary Key Columns
/// 
/// * EFFECTIVEDATE
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ParticipantRegistrationStationownertrk1 {
    /// Effective date of this record
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Unique participant identifier
    pub participantid: String,
    /// Version no of record within the effective date
    pub versionno: rust_decimal::Decimal,
    /// User authorising record
    pub authorisedby: Option<String>,
    /// Date record authorised
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for ParticipantRegistrationStationownertrk1 {
    type PrimaryKey = ParticipantRegistrationStationownertrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("STATIONOWNERTRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationStationownertrk1PrimaryKey {
        ParticipantRegistrationStationownertrk1PrimaryKey {
            effectivedate: self.effectivedate,
            participantid: self.participantid.clone(),
            versionno: self.versionno
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
    }

    fn partition_name(&self) -> String {
        "participant_registration_stationownertrk_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ParticipantRegistrationStationownertrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ParticipantRegistrationStationownertrk1PrimaryKey {
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationStationownertrk1 {
    type Row = ParticipantRegistrationStationownertrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.participantid == row.participantid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationStationownertrk1 {
    type PrimaryKey = ParticipantRegistrationStationownertrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.participantid == key.participantid
        && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for ParticipantRegistrationStationownertrk1PrimaryKey {
    type Row = ParticipantRegistrationStationownertrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
        && self.participantid == row.participantid
        && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ParticipantRegistrationStationownertrk1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationStationownertrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
        && self.participantid == key.participantid
        && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ParticipantRegistrationStationownertrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("effectivedate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), false),
            arrow2::datatypes::Field::new("participantid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Decimal(3,0), false),
            arrow2::datatypes::Field::new("authorisedby", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("authoriseddate", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None), true)
        ])
    }

    fn partition_to_chunk(partition: impl Iterator<Item=Self>) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut effectivedate_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            participantid_array.push(row.participantid);
            versionno_array.push({
                        let mut val = row.versionno;
                        val.rescale(0);
                        val.mantissa()
                    });
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
        //std::sync::Arc::new(Self::arrow_schema()),
        vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(participantid_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array).to(arrow2::datatypes::DataType::Decimal(3,0))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None))) as std::sync::Arc<dyn arrow2::array::Array>,
            ]
        ).map_err(Into::into)
    }
}
#[cfg(feature = "sql_server")]
pub async fn save<'a, S>(mms_file: &mut mmsdm_core::MmsFile<'a>, file_key: &mmsdm_core::FileKey, client: &mut tiberius::Client<S>, chunk_size: Option<usize>) -> mmsdm_core::Result<()>
where S: futures_util::AsyncRead + futures_util::AsyncWrite + Unpin + Send,
{
    match (
        file_key.table_name.as_deref(),
        file_key.version,
    ) {
        (Some("BIDDUIDDETAILS"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationBidduiddetails1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationBidduiddetails1 @P1, @P2", chunk_size).await?;
        }
        (Some("BIDDUIDDETAILSTRK"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationBidduiddetailstrk1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationBidduiddetailstrk1 @P1, @P2", chunk_size).await?;
        }
        (Some("DISPATCHABLEUNIT"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationDispatchableunit1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationDispatchableunit1 @P1, @P2", chunk_size).await?;
        }
        (Some("DUALLOC"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationDualloc1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationDualloc1 @P1, @P2", chunk_size).await?;
        }
        (Some("DUDETAIL"), version) if version <= 4_i32 => {
            let d: Vec<ParticipantRegistrationDudetail4> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationDudetail4 @P1, @P2", chunk_size).await?;
        }
        (Some("DUDETAILSUMMARY"), version) if version <= 5_i32 => {
            let d: Vec<ParticipantRegistrationDudetailsummary5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationDudetailsummary5 @P1, @P2", chunk_size).await?;
        }
        (Some("GENMETER"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationGenmeter1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationGenmeter1 @P1, @P2", chunk_size).await?;
        }
        (Some("GENUNITS"), version) if version <= 2_i32 => {
            let d: Vec<ParticipantRegistrationGenunits2> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationGenunits2 @P1, @P2", chunk_size).await?;
        }
        (Some("GENUNITS_UNIT"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationGenunitsUnit1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationGenunitsUnit1 @P1, @P2", chunk_size).await?;
        }
        (Some("MNSP_INTERCONNECTOR"), version) if version <= 2_i32 => {
            let d: Vec<ParticipantRegistrationMnspInterconnector2> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationMnspInterconnector2 @P1, @P2", chunk_size).await?;
        }
        (Some("MNSP_PARTICIPANT"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationMnspParticipant1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationMnspParticipant1 @P1, @P2", chunk_size).await?;
        }
        (Some("PARTICIPANT"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationParticipant1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipant1 @P1, @P2", chunk_size).await?;
        }
        (Some("PARTICIPANTACCOUNT"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationParticipantaccount1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantaccount1 @P1, @P2", chunk_size).await?;
        }
        (Some("PARTICIPANTCATEGORY"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationParticipantcategory1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantcategory1 @P1, @P2", chunk_size).await?;
        }
        (Some("PARTICIPANTCATEGORYALLOC"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationParticipantcategoryalloc1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantcategoryalloc1 @P1, @P2", chunk_size).await?;
        }
        (Some("PARTICIPANTCLASS"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationParticipantclass1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantclass1 @P1, @P2", chunk_size).await?;
        }
        (Some("PARTICIPANTCREDITDETAIL"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationParticipantcreditdetail1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantcreditdetail1 @P1, @P2", chunk_size).await?;
        }
        (Some("PMS_GROUP"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationPmsGroup1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationPmsGroup1 @P1, @P2", chunk_size).await?;
        }
        (Some("PMS_GROUPNMI"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationPmsGroupnmi1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationPmsGroupnmi1 @P1, @P2", chunk_size).await?;
        }
        (Some("PMS_GROUPSERVICE"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationPmsGroupservice1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationPmsGroupservice1 @P1, @P2", chunk_size).await?;
        }
        (Some("STADUALLOC"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationStadualloc1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationStadualloc1 @P1, @P2", chunk_size).await?;
        }
        (Some("STATION"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationStation1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationStation1 @P1, @P2", chunk_size).await?;
        }
        (Some("STATIONOPERATINGSTATUS"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationStationoperatingstatus1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationStationoperatingstatus1 @P1, @P2", chunk_size).await?;
        }
        (Some("STATIONOWNER"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationStationowner1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationStationowner1 @P1, @P2", chunk_size).await?;
        }
        (Some("STATIONOWNERTRK"), version) if version <= 1_i32 => {
            let d: Vec<ParticipantRegistrationStationownertrk1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(client, file_key, mms_file.header(), &d, "exec mmsdm_proc.InsertParticipantRegistrationStationownertrk1 @P1, @P2", chunk_size).await?;
        }
        _ => {
            log::error!("Unexpected file key {:?}", file_key);
        }
    }
    Ok(())
}
