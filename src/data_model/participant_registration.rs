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
    #[serde(with = "crate::mms_datetime")]
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationBidduiddetails1 {
    type PrimaryKey = ParticipantRegistrationBidduiddetails1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
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
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_bidduiddetails_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationBidduiddetails1 {
    type Row = ParticipantRegistrationBidduiddetails1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.duid == row.duid
            && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationBidduiddetails1 {
    type PrimaryKey = ParticipantRegistrationBidduiddetails1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.duid == key.duid
            && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationBidduiddetails1PrimaryKey {
    pub bidtype: String,
    pub duid: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ParticipantRegistrationBidduiddetails1PrimaryKey {
    type Row = ParticipantRegistrationBidduiddetails1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.duid == row.duid
            && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationBidduiddetails1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationBidduiddetails1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.duid == key.duid
            && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for ParticipantRegistrationBidduiddetails1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationBidduiddetails1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("bidtype", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "maxcapacity",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "minenablementlevel",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "maxenablementlevel",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "maxlowerangle",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "maxupperangle",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
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
        for (_, row) in partition {
            duid_array.push(row.duid);
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
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
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bidtype_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maxcapacity_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(minenablementlevel_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maxenablementlevel_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maxlowerangle_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maxupperangle_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
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
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Record version number
    pub versionno: rust_decimal::Decimal,
    /// Date of record authorisation. A NULL value indicates the record is not authorised.
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User that authorised record. A NULL value indicates the record is not authorised.
    pub authorisedby: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationBidduiddetailstrk1 {
    type PrimaryKey = ParticipantRegistrationBidduiddetailstrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("BIDDUIDDETAILSTRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationBidduiddetailstrk1PrimaryKey {
        ParticipantRegistrationBidduiddetailstrk1PrimaryKey {
            duid: self.duid.clone(),
            effectivedate: self.effectivedate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_bidduiddetailstrk_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationBidduiddetailstrk1 {
    type Row = ParticipantRegistrationBidduiddetailstrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationBidduiddetailstrk1 {
    type PrimaryKey = ParticipantRegistrationBidduiddetailstrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationBidduiddetailstrk1PrimaryKey {
    pub duid: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ParticipantRegistrationBidduiddetailstrk1PrimaryKey {
    type Row = ParticipantRegistrationBidduiddetailstrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationBidduiddetailstrk1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationBidduiddetailstrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for ParticipantRegistrationBidduiddetailstrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationBidduiddetailstrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut duid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            duid_array.push(row.duid);
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            authoriseddate_array.push(row.authoriseddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            authorisedby_array.push(row.authorisedby);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationDispatchableunit1 {
    type PrimaryKey = ParticipantRegistrationDispatchableunit1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("DISPATCHABLEUNIT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationDispatchableunit1PrimaryKey {
        ParticipantRegistrationDispatchableunit1PrimaryKey {
            duid: self.duid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_dispatchableunit_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationDispatchableunit1 {
    type Row = ParticipantRegistrationDispatchableunit1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationDispatchableunit1 {
    type PrimaryKey = ParticipantRegistrationDispatchableunit1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationDispatchableunit1PrimaryKey {
    pub duid: String,
}
impl crate::CompareWithRow for ParticipantRegistrationDispatchableunit1PrimaryKey {
    type Row = ParticipantRegistrationDispatchableunit1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationDispatchableunit1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationDispatchableunit1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
    }
}
impl crate::PrimaryKey for ParticipantRegistrationDispatchableunit1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationDispatchableunit1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("duname", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("unittype", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut duid_array = Vec::new();
        let mut duname_array = Vec::new();
        let mut unittype_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            duid_array.push(row.duid);
            duname_array.push(row.duname);
            unittype_array.push(row.unittype);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(duname_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(unittype_array)),
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
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of record
    pub versionno: rust_decimal::Decimal,
    /// Dispatchable Unit identifier
    pub duid: String,
    /// Physical unit identifier
    pub gensetid: String,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationDualloc1 {
    type PrimaryKey = ParticipantRegistrationDualloc1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
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
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_dualloc_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationDualloc1 {
    type Row = ParticipantRegistrationDualloc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.effectivedate == row.effectivedate
            && self.gensetid == row.gensetid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationDualloc1 {
    type PrimaryKey = ParticipantRegistrationDualloc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.effectivedate == key.effectivedate
            && self.gensetid == key.gensetid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationDualloc1PrimaryKey {
    pub duid: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub gensetid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ParticipantRegistrationDualloc1PrimaryKey {
    type Row = ParticipantRegistrationDualloc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.effectivedate == row.effectivedate
            && self.gensetid == row.gensetid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationDualloc1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationDualloc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.effectivedate == key.effectivedate
            && self.gensetid == key.gensetid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for ParticipantRegistrationDualloc1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationDualloc1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "gensetid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut gensetid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            duid_array.push(row.duid);
            gensetid_array.push(row.gensetid);
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
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(gensetid_array)),
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
/// ## DUDETAIL
///  _DUDETAIL sets out a records specific details for each unit including start type and whether normally on or off load. Much of this data is information only and is not used in dispatch or settlements._
///
/// * Data Set Name: Participant Registration
/// * File Name: Dudetail
/// * Data Version: 3
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
pub struct ParticipantRegistrationDudetail3 {
    /// Effective calendar date of record
    #[serde(with = "crate::mms_datetime")]
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
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
impl crate::GetTable for ParticipantRegistrationDudetail3 {
    type PrimaryKey = ParticipantRegistrationDudetail3PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("DUDETAIL".into()),
            version: 3,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationDudetail3PrimaryKey {
        ParticipantRegistrationDudetail3PrimaryKey {
            duid: self.duid.clone(),
            effectivedate: self.effectivedate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_dudetail_v3".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationDudetail3 {
    type Row = ParticipantRegistrationDudetail3;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationDudetail3 {
    type PrimaryKey = ParticipantRegistrationDudetail3PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationDudetail3PrimaryKey {
    pub duid: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ParticipantRegistrationDudetail3PrimaryKey {
    type Row = ParticipantRegistrationDudetail3;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationDudetail3PrimaryKey {
    type PrimaryKey = ParticipantRegistrationDudetail3PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for ParticipantRegistrationDudetail3PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationDudetail3 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "voltlevel",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "registeredcapacity",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "agccapability",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "dispatchtype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "maxcapacity",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "starttype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "normallyonflag",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "physicaldetailsflag",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "spinningreserveflag",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "intermittentflag",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "semi_schedule_flag",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "maxrateofchangeup",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "maxrateofchangedown",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "dispatchsubtype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
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
        for (_, row) in partition {
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
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
            authoriseddate_array.push(row.authoriseddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
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

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    connectionpointid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(voltlevel_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(registeredcapacity_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(agccapability_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(dispatchtype_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maxcapacity_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(starttype_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(normallyonflag_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    physicaldetailsflag_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    spinningreserveflag_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    intermittentflag_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    semi_schedule_flag_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maxrateofchangeup_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maxrateofchangedown_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(dispatchsubtype_array)),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DUDETAILSUMMARY
///  _DUDETAILSUMMARY sets out a single summary unit table so reducing the need for participants to use the various dispatchable unit detail and owner tables to establish generating unit specific details._
///
/// * Data Set Name: Participant Registration
/// * File Name: Dudetailsummary
/// * Data Version: 4
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
pub struct ParticipantRegistrationDudetailsummary4 {
    /// Dispatchable Unit Identifier
    pub duid: String,
    /// Start date for effective record
    #[serde(with = "crate::mms_datetime")]
    pub start_date: chrono::NaiveDateTime,
    /// End date for effective record
    #[serde(with = "crate::mms_datetime")]
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
    #[serde(with = "crate::mms_datetime_opt")]
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
impl crate::GetTable for ParticipantRegistrationDudetailsummary4 {
    type PrimaryKey = ParticipantRegistrationDudetailsummary4PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("DUDETAILSUMMARY".into()),
            version: 4,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationDudetailsummary4PrimaryKey {
        ParticipantRegistrationDudetailsummary4PrimaryKey {
            duid: self.duid.clone(),
            start_date: self.start_date,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_dudetailsummary_v4".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationDudetailsummary4 {
    type Row = ParticipantRegistrationDudetailsummary4;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.start_date == row.start_date
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationDudetailsummary4 {
    type PrimaryKey = ParticipantRegistrationDudetailsummary4PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.start_date == key.start_date
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationDudetailsummary4PrimaryKey {
    pub duid: String,
    pub start_date: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for ParticipantRegistrationDudetailsummary4PrimaryKey {
    type Row = ParticipantRegistrationDudetailsummary4;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.start_date == row.start_date
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationDudetailsummary4PrimaryKey {
    type PrimaryKey = ParticipantRegistrationDudetailsummary4PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.start_date == key.start_date
    }
}
impl crate::PrimaryKey for ParticipantRegistrationDudetailsummary4PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationDudetailsummary4 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("start_date", arrow2::datatypes::DataType::Date32, false),
            arrow2::datatypes::Field::new("end_date", arrow2::datatypes::DataType::Date32, false),
            arrow2::datatypes::Field::new(
                "dispatchtype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "stationid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "transmissionlossfactor",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "starttype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "distributionlossfactor",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "minimum_energy_price",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "maximum_energy_price",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "schedule_type",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "min_ramp_rate_up",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "min_ramp_rate_down",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "max_ramp_rate_up",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "max_ramp_rate_down",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "is_aggregated",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "dispatchsubtype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
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
        for (_, row) in partition {
            duid_array.push(row.duid);
            start_date_array.push(
                i32::try_from(
                    (row.start_date.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            end_date_array.push(
                i32::try_from(
                    (row.end_date.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            dispatchtype_array.push(row.dispatchtype);
            connectionpointid_array.push(row.connectionpointid);
            regionid_array.push(row.regionid);
            stationid_array.push(row.stationid);
            participantid_array.push(row.participantid);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
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

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(start_date_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(end_date_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(dispatchtype_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    connectionpointid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(regionid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(stationid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(participantid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(transmissionlossfactor_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(starttype_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(distributionlossfactor_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(minimum_energy_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maximum_energy_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(schedule_type_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(min_ramp_rate_up_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(min_ramp_rate_down_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(max_ramp_rate_up_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(max_ramp_rate_down_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(is_aggregated_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(dispatchsubtype_array)),
            ],
        )
        .map_err(Into::into)
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
    #[serde(with = "crate::mms_datetime")]
    pub applydate: chrono::NaiveDateTime,
    /// Version no of the record for the given effective date
    pub versionno: rust_decimal::Decimal,
    /// AEMO user authorising
    pub authorisedby: Option<String>,
    /// Date authorised
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Not used
    #[serde(with = "crate::mms_datetime_opt")]
    pub comdate: Option<chrono::NaiveDateTime>,
    /// Not used
    #[serde(with = "crate::mms_datetime_opt")]
    pub decomdate: Option<chrono::NaiveDateTime>,
    /// Not used
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Not used
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationGenmeter1 {
    type PrimaryKey = ParticipantRegistrationGenmeter1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("GENMETER".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationGenmeter1PrimaryKey {
        ParticipantRegistrationGenmeter1PrimaryKey {
            applydate: self.applydate,
            meterid: self.meterid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_genmeter_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationGenmeter1 {
    type Row = ParticipantRegistrationGenmeter1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.applydate == row.applydate
            && self.meterid == row.meterid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationGenmeter1 {
    type PrimaryKey = ParticipantRegistrationGenmeter1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.applydate == key.applydate
            && self.meterid == key.meterid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationGenmeter1PrimaryKey {
    pub applydate: chrono::NaiveDateTime,
    pub meterid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ParticipantRegistrationGenmeter1PrimaryKey {
    type Row = ParticipantRegistrationGenmeter1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.applydate == row.applydate
            && self.meterid == row.meterid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationGenmeter1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationGenmeter1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.applydate == key.applydate
            && self.meterid == key.meterid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for ParticipantRegistrationGenmeter1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationGenmeter1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("meterid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("gensetid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "stationid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "metertype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "meterclass",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "voltagelevel",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new("applydate", arrow2::datatypes::DataType::Date32, false),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new("comdate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("decomdate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
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
        for (_, row) in partition {
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
            applydate_array.push(
                i32::try_from(
                    (row.applydate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            comdate_array.push(row.comdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            decomdate_array.push(row.decomdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            enddate_array.push(row.enddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            startdate_array.push(row.startdate.map(|val| {
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
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(meterid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(gensetid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    connectionpointid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(stationid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(metertype_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(meterclass_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(voltagelevel_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(applydate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(comdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(decomdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startdate_array)
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The emissions factor for the generating unit, as calculated by Settlements staff members
    pub co2e_emissions_factor: Option<rust_decimal::Decimal>,
    /// The energy source for the generating unit, as used in the calculation of the CO2-e emissions factor.  Distinct from the Energy Source for a generating unit published as part of the Registration Master List
    pub co2e_energy_source: Option<String>,
    /// An indicator as to the source of the emission factor used in the calculation of the index. The applicable values for this field would be NTNDP which indicates the emission factor is quoted from the National Transmission Network Development Plan or Estimated to indicate the emission factor has been calculated using an internal AEMO procedure based upon the Department of Climate Change and Energy Efficiency NGA factors
    pub co2e_data_source: Option<String>,
}
impl crate::GetTable for ParticipantRegistrationGenunits2 {
    type PrimaryKey = ParticipantRegistrationGenunits2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("GENUNITS".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationGenunits2PrimaryKey {
        ParticipantRegistrationGenunits2PrimaryKey {
            gensetid: self.gensetid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_genunits_v2".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationGenunits2 {
    type Row = ParticipantRegistrationGenunits2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.gensetid == row.gensetid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationGenunits2 {
    type PrimaryKey = ParticipantRegistrationGenunits2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.gensetid == key.gensetid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationGenunits2PrimaryKey {
    pub gensetid: String,
}
impl crate::CompareWithRow for ParticipantRegistrationGenunits2PrimaryKey {
    type Row = ParticipantRegistrationGenunits2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.gensetid == row.gensetid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationGenunits2PrimaryKey {
    type PrimaryKey = ParticipantRegistrationGenunits2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.gensetid == key.gensetid
    }
}
impl crate::PrimaryKey for ParticipantRegistrationGenunits2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationGenunits2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "gensetid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "stationid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "setlossfactor",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "cdindicator",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("agcflag", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "spinningflag",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "voltlevel",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "registeredcapacity",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "dispatchtype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "starttype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "mktgeneratorind",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "normalstatus",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "maxcapacity",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "gensettype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "gensetname",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "co2e_emissions_factor",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "co2e_energy_source",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "co2e_data_source",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
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
        for (_, row) in partition {
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
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            co2e_emissions_factor_array.push({
                row.co2e_emissions_factor.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            co2e_energy_source_array.push(row.co2e_energy_source);
            co2e_data_source_array.push(row.co2e_data_source);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(gensetid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(stationid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(setlossfactor_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(cdindicator_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(agcflag_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(spinningflag_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(voltlevel_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(registeredcapacity_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(dispatchtype_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(starttype_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(mktgeneratorind_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(normalstatus_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maxcapacity_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(gensettype_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(gensetname_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(co2e_emissions_factor_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    co2e_energy_source_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    co2e_data_source_array,
                )),
            ],
        )
        .map_err(Into::into)
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
    #[serde(with = "crate::mms_datetime")]
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationGenunitsUnit1 {
    type PrimaryKey = ParticipantRegistrationGenunitsUnit1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
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
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_genunits_unit_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationGenunitsUnit1 {
    type Row = ParticipantRegistrationGenunitsUnit1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.gensetid == row.gensetid
            && self.unit_grouping_label == row.unit_grouping_label
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationGenunitsUnit1 {
    type PrimaryKey = ParticipantRegistrationGenunitsUnit1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.gensetid == key.gensetid
            && self.unit_grouping_label == key.unit_grouping_label
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationGenunitsUnit1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub gensetid: String,
    pub unit_grouping_label: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ParticipantRegistrationGenunitsUnit1PrimaryKey {
    type Row = ParticipantRegistrationGenunitsUnit1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.gensetid == row.gensetid
            && self.unit_grouping_label == row.unit_grouping_label
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationGenunitsUnit1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationGenunitsUnit1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.gensetid == key.gensetid
            && self.unit_grouping_label == key.unit_grouping_label
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for ParticipantRegistrationGenunitsUnit1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationGenunitsUnit1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "gensetid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(6, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "unit_grouping_label",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "unit_count",
                arrow2::datatypes::DataType::Decimal(10, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "unit_size",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "unit_max_size",
                arrow2::datatypes::DataType::Decimal(8, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "aggregation_flag",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut gensetid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut unit_grouping_label_array = Vec::new();
        let mut unit_count_array = Vec::new();
        let mut unit_size_array = Vec::new();
        let mut unit_max_size_array = Vec::new();
        let mut aggregation_flag_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            gensetid_array.push(row.gensetid);
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
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
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(gensetid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    unit_grouping_label_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(unit_count_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(unit_size_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(unit_max_size_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(aggregation_flag_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
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
    #[serde(with = "crate::mms_datetime")]
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorising officer
    pub authorisedby: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Transmission Loss Factor for Link "From Region" end
    pub from_region_tlf: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor for Link at "To Region" end
    pub to_region_tlf: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for ParticipantRegistrationMnspInterconnector2 {
    type PrimaryKey = ParticipantRegistrationMnspInterconnector2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("MNSP_INTERCONNECTOR".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationMnspInterconnector2PrimaryKey {
        ParticipantRegistrationMnspInterconnector2PrimaryKey {
            effectivedate: self.effectivedate,
            linkid: self.linkid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_mnsp_interconnector_v2".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationMnspInterconnector2 {
    type Row = ParticipantRegistrationMnspInterconnector2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.linkid == row.linkid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationMnspInterconnector2 {
    type PrimaryKey = ParticipantRegistrationMnspInterconnector2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.linkid == key.linkid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationMnspInterconnector2PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub linkid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ParticipantRegistrationMnspInterconnector2PrimaryKey {
    type Row = ParticipantRegistrationMnspInterconnector2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.linkid == row.linkid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationMnspInterconnector2PrimaryKey {
    type PrimaryKey = ParticipantRegistrationMnspInterconnector2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.linkid == key.linkid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for ParticipantRegistrationMnspInterconnector2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationMnspInterconnector2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("linkid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "fromregion",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("toregion", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "maxcapacity",
                arrow2::datatypes::DataType::Decimal(5, 0),
                true,
            ),
            arrow2::datatypes::Field::new("tlf", arrow2::datatypes::DataType::Decimal(12, 7), true),
            arrow2::datatypes::Field::new(
                "lhsfactor",
                arrow2::datatypes::DataType::Decimal(12, 7),
                true,
            ),
            arrow2::datatypes::Field::new(
                "meterflowconstant",
                arrow2::datatypes::DataType::Decimal(12, 7),
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "from_region_tlf",
                arrow2::datatypes::DataType::Decimal(12, 7),
                true,
            ),
            arrow2::datatypes::Field::new(
                "to_region_tlf",
                arrow2::datatypes::DataType::Decimal(12, 7),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
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
        for (_, row) in partition {
            linkid_array.push(row.linkid);
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
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
            authoriseddate_array.push(row.authoriseddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            authorisedby_array.push(row.authorisedby);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
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

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(linkid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(fromregion_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(toregion_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maxcapacity_array)
                        .to(arrow2::datatypes::DataType::Decimal(5, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(tlf_array)
                        .to(arrow2::datatypes::DataType::Decimal(12, 7)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lhsfactor_array)
                        .to(arrow2::datatypes::DataType::Decimal(12, 7)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(meterflowconstant_array)
                        .to(arrow2::datatypes::DataType::Decimal(12, 7)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(from_region_tlf_array)
                        .to(arrow2::datatypes::DataType::Decimal(12, 7)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(to_region_tlf_array)
                        .to(arrow2::datatypes::DataType::Decimal(12, 7)),
                ),
            ],
        )
        .map_err(Into::into)
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
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of data for other key data - a higher version for same key data takes precedence
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationMnspParticipant1 {
    type PrimaryKey = ParticipantRegistrationMnspParticipant1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
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
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_mnsp_participant_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationMnspParticipant1 {
    type Row = ParticipantRegistrationMnspParticipant1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationMnspParticipant1 {
    type PrimaryKey = ParticipantRegistrationMnspParticipant1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationMnspParticipant1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub interconnectorid: String,
    pub participantid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ParticipantRegistrationMnspParticipant1PrimaryKey {
    type Row = ParticipantRegistrationMnspParticipant1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationMnspParticipant1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationMnspParticipant1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for ParticipantRegistrationMnspParticipant1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationMnspParticipant1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut interconnectorid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            interconnectorid_array.push(row.interconnectorid);
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationParticipant1 {
    type PrimaryKey = ParticipantRegistrationParticipant1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("PARTICIPANT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationParticipant1PrimaryKey {
        ParticipantRegistrationParticipant1PrimaryKey {
            participantid: self.participantid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_participant_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationParticipant1 {
    type Row = ParticipantRegistrationParticipant1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationParticipant1 {
    type PrimaryKey = ParticipantRegistrationParticipant1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationParticipant1PrimaryKey {
    pub participantid: String,
}
impl crate::CompareWithRow for ParticipantRegistrationParticipant1PrimaryKey {
    type Row = ParticipantRegistrationParticipant1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationParticipant1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipant1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
    }
}
impl crate::PrimaryKey for ParticipantRegistrationParticipant1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationParticipant1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantclassid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("name", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("acn", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "primarybusiness",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut participantid_array = Vec::new();
        let mut participantclassid_array = Vec::new();
        let mut name_array = Vec::new();
        let mut description_array = Vec::new();
        let mut acn_array = Vec::new();
        let mut primarybusiness_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            participantid_array.push(row.participantid);
            participantclassid_array.push(row.participantclassid);
            name_array.push(row.name);
            description_array.push(row.description);
            acn_array.push(row.acn);
            primarybusiness_array.push(row.primarybusiness);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    participantclassid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(name_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(acn_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(primarybusiness_array)),
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Date record authorised
    #[serde(with = "crate::mms_datetime_opt")]
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Australian Business Number
    pub abn: Option<String>,
}
impl crate::GetTable for ParticipantRegistrationParticipantaccount1 {
    type PrimaryKey = ParticipantRegistrationParticipantaccount1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("PARTICIPANTACCOUNT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationParticipantaccount1PrimaryKey {
        ParticipantRegistrationParticipantaccount1PrimaryKey {
            participantid: self.participantid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_participantaccount_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationParticipantaccount1 {
    type Row = ParticipantRegistrationParticipantaccount1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationParticipantaccount1 {
    type PrimaryKey = ParticipantRegistrationParticipantaccount1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationParticipantaccount1PrimaryKey {
    pub participantid: String,
}
impl crate::CompareWithRow for ParticipantRegistrationParticipantaccount1PrimaryKey {
    type Row = ParticipantRegistrationParticipantaccount1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationParticipantaccount1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipantaccount1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
    }
}
impl crate::PrimaryKey for ParticipantRegistrationParticipantaccount1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationParticipantaccount1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "accountname",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "accountnumber",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("bankname", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "banknumber",
                arrow2::datatypes::DataType::Decimal(10, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "branchname",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "branchnumber",
                arrow2::datatypes::DataType::Decimal(10, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bsbnumber",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "nemmcocreditaccountnumber",
                arrow2::datatypes::DataType::Decimal(10, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "nemmcodebitaccountnumber",
                arrow2::datatypes::DataType::Decimal(10, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("abn", arrow2::datatypes::DataType::LargeUtf8, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
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
        for (_, row) in partition {
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
            authoriseddate_array.push(row.authoriseddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            effectivedate_array.push(row.effectivedate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            abn_array.push(row.abn);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(accountname_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(accountnumber_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(bankname_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(banknumber_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(branchname_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(branchnumber_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(bsbnumber_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(nemmcocreditaccountnumber_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(nemmcodebitaccountnumber_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(abn_array)),
            ],
        )
        .map_err(Into::into)
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationParticipantcategory1 {
    type PrimaryKey = ParticipantRegistrationParticipantcategory1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("PARTICIPANTCATEGORY".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationParticipantcategory1PrimaryKey {
        ParticipantRegistrationParticipantcategory1PrimaryKey {
            participantcategoryid: self.participantcategoryid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_participantcategory_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationParticipantcategory1 {
    type Row = ParticipantRegistrationParticipantcategory1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantcategoryid == row.participantcategoryid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationParticipantcategory1 {
    type PrimaryKey = ParticipantRegistrationParticipantcategory1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantcategoryid == key.participantcategoryid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationParticipantcategory1PrimaryKey {
    pub participantcategoryid: String,
}
impl crate::CompareWithRow for ParticipantRegistrationParticipantcategory1PrimaryKey {
    type Row = ParticipantRegistrationParticipantcategory1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantcategoryid == row.participantcategoryid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationParticipantcategory1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipantcategory1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantcategoryid == key.participantcategoryid
    }
}
impl crate::PrimaryKey for ParticipantRegistrationParticipantcategory1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationParticipantcategory1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "participantcategoryid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut participantcategoryid_array = Vec::new();
        let mut description_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            participantcategoryid_array.push(row.participantcategoryid);
            description_array.push(row.description);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantcategoryid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)),
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationParticipantcategoryalloc1 {
    type PrimaryKey = ParticipantRegistrationParticipantcategoryalloc1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("PARTICIPANTCATEGORYALLOC".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {
        ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {
            participantcategoryid: self.participantcategoryid.clone(),
            participantid: self.participantid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_participantcategoryalloc_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationParticipantcategoryalloc1 {
    type Row = ParticipantRegistrationParticipantcategoryalloc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantcategoryid == row.participantcategoryid
            && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationParticipantcategoryalloc1 {
    type PrimaryKey = ParticipantRegistrationParticipantcategoryalloc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantcategoryid == key.participantcategoryid
            && self.participantid == key.participantid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {
    pub participantcategoryid: String,
    pub participantid: String,
}
impl crate::CompareWithRow for ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {
    type Row = ParticipantRegistrationParticipantcategoryalloc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantcategoryid == row.participantcategoryid
            && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipantcategoryalloc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantcategoryid == key.participantcategoryid
            && self.participantid == key.participantid
    }
}
impl crate::PrimaryKey for ParticipantRegistrationParticipantcategoryalloc1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationParticipantcategoryalloc1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "participantcategoryid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut participantcategoryid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            participantcategoryid_array.push(row.participantcategoryid);
            participantid_array.push(row.participantid);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantcategoryid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationParticipantclass1 {
    type PrimaryKey = ParticipantRegistrationParticipantclass1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("PARTICIPANTCLASS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationParticipantclass1PrimaryKey {
        ParticipantRegistrationParticipantclass1PrimaryKey {
            participantclassid: self.participantclassid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_participantclass_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationParticipantclass1 {
    type Row = ParticipantRegistrationParticipantclass1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantclassid == row.participantclassid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationParticipantclass1 {
    type PrimaryKey = ParticipantRegistrationParticipantclass1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantclassid == key.participantclassid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationParticipantclass1PrimaryKey {
    pub participantclassid: String,
}
impl crate::CompareWithRow for ParticipantRegistrationParticipantclass1PrimaryKey {
    type Row = ParticipantRegistrationParticipantclass1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantclassid == row.participantclassid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationParticipantclass1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipantclass1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantclassid == key.participantclassid
    }
}
impl crate::PrimaryKey for ParticipantRegistrationParticipantclass1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationParticipantclass1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "participantclassid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut participantclassid_array = Vec::new();
        let mut description_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            participantclassid_array.push(row.participantclassid);
            description_array.push(row.description);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantclassid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)),
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
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// &nbsp;
    pub participantid: String,
    /// &nbsp;
    pub creditlimit: Option<rust_decimal::Decimal>,
    /// &nbsp;
    pub authorisedby: Option<String>,
    /// &nbsp;
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationParticipantcreditdetail1 {
    type PrimaryKey = ParticipantRegistrationParticipantcreditdetail1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("PARTICIPANTCREDITDETAIL".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationParticipantcreditdetail1PrimaryKey {
        ParticipantRegistrationParticipantcreditdetail1PrimaryKey {
            effectivedate: self.effectivedate,
            participantid: self.participantid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_participantcreditdetail_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationParticipantcreditdetail1 {
    type Row = ParticipantRegistrationParticipantcreditdetail1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationParticipantcreditdetail1 {
    type PrimaryKey = ParticipantRegistrationParticipantcreditdetail1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.participantid == key.participantid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationParticipantcreditdetail1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: String,
}
impl crate::CompareWithRow for ParticipantRegistrationParticipantcreditdetail1PrimaryKey {
    type Row = ParticipantRegistrationParticipantcreditdetail1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationParticipantcreditdetail1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationParticipantcreditdetail1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.participantid == key.participantid
    }
}
impl crate::PrimaryKey for ParticipantRegistrationParticipantcreditdetail1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationParticipantcreditdetail1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "creditlimit",
                arrow2::datatypes::DataType::Decimal(10, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
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
        let mut participantid_array = Vec::new();
        let mut creditlimit_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            participantid_array.push(row.participantid);
            creditlimit_array.push({
                row.creditlimit.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            authorisedby_array.push(row.authorisedby);
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
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(creditlimit_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
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
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Station Identifier
    pub stationid: String,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationStadualloc1 {
    type PrimaryKey = ParticipantRegistrationStadualloc1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
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
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_stadualloc_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationStadualloc1 {
    type Row = ParticipantRegistrationStadualloc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.effectivedate == row.effectivedate
            && self.stationid == row.stationid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationStadualloc1 {
    type PrimaryKey = ParticipantRegistrationStadualloc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.effectivedate == key.effectivedate
            && self.stationid == key.stationid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationStadualloc1PrimaryKey {
    pub duid: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub stationid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ParticipantRegistrationStadualloc1PrimaryKey {
    type Row = ParticipantRegistrationStadualloc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.effectivedate == row.effectivedate
            && self.stationid == row.stationid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationStadualloc1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationStadualloc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.effectivedate == key.effectivedate
            && self.stationid == key.stationid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for ParticipantRegistrationStadualloc1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationStadualloc1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "stationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut duid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut stationid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            duid_array.push(row.duid);
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            stationid_array.push(row.stationid);
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(stationid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Not used. Do not use as the Connection Point Identifier for station load
    pub connectionpointid: Option<String>,
}
impl crate::GetTable for ParticipantRegistrationStation1 {
    type PrimaryKey = ParticipantRegistrationStation1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("STATION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationStation1PrimaryKey {
        ParticipantRegistrationStation1PrimaryKey {
            stationid: self.stationid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_station_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationStation1 {
    type Row = ParticipantRegistrationStation1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.stationid == row.stationid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationStation1 {
    type PrimaryKey = ParticipantRegistrationStation1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.stationid == key.stationid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationStation1PrimaryKey {
    pub stationid: String,
}
impl crate::CompareWithRow for ParticipantRegistrationStation1PrimaryKey {
    type Row = ParticipantRegistrationStation1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.stationid == row.stationid
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationStation1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationStation1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.stationid == key.stationid
    }
}
impl crate::PrimaryKey for ParticipantRegistrationStation1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationStation1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "stationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "stationname",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("address1", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("address2", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("address3", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("address4", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("city", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("state", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("postcode", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
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
        for (_, row) in partition {
            stationid_array.push(row.stationid);
            stationname_array.push(row.stationname);
            address1_array.push(row.address1);
            address2_array.push(row.address2);
            address3_array.push(row.address3);
            address4_array.push(row.address4);
            city_array.push(row.city);
            state_array.push(row.state);
            postcode_array.push(row.postcode);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            connectionpointid_array.push(row.connectionpointid);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(stationid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(stationname_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(address1_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(address2_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(address3_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(address4_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(city_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(state_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(postcode_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    connectionpointid_array,
                )),
            ],
        )
        .map_err(Into::into)
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
    #[serde(with = "crate::mms_datetime")]
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationStationoperatingstatus1 {
    type PrimaryKey = ParticipantRegistrationStationoperatingstatus1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("STATIONOPERATINGSTATUS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationStationoperatingstatus1PrimaryKey {
        ParticipantRegistrationStationoperatingstatus1PrimaryKey {
            effectivedate: self.effectivedate,
            stationid: self.stationid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_stationoperatingstatus_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationStationoperatingstatus1 {
    type Row = ParticipantRegistrationStationoperatingstatus1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.stationid == row.stationid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationStationoperatingstatus1 {
    type PrimaryKey = ParticipantRegistrationStationoperatingstatus1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.stationid == key.stationid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationStationoperatingstatus1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub stationid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ParticipantRegistrationStationoperatingstatus1PrimaryKey {
    type Row = ParticipantRegistrationStationoperatingstatus1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.stationid == row.stationid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationStationoperatingstatus1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationStationoperatingstatus1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.stationid == key.stationid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for ParticipantRegistrationStationoperatingstatus1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationStationoperatingstatus1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "stationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("status", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
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
        let mut stationid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut status_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            stationid_array.push(row.stationid);
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            status_array.push(row.status);
            authorisedby_array.push(row.authorisedby);
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
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(stationid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(status_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
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
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Unique participant identifier
    pub participantid: String,
    /// Station Identifier
    pub stationid: String,
    /// Version no of record within the effective date
    pub versionno: rust_decimal::Decimal,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationStationowner1 {
    type PrimaryKey = ParticipantRegistrationStationowner1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
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
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_stationowner_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationStationowner1 {
    type Row = ParticipantRegistrationStationowner1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid
            && self.stationid == row.stationid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationStationowner1 {
    type PrimaryKey = ParticipantRegistrationStationowner1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid
            && self.stationid == key.stationid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationStationowner1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: String,
    pub stationid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ParticipantRegistrationStationowner1PrimaryKey {
    type Row = ParticipantRegistrationStationowner1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid
            && self.stationid == row.stationid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationStationowner1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationStationowner1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid
            && self.stationid == key.stationid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for ParticipantRegistrationStationowner1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationStationowner1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "stationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut effectivedate_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut stationid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            participantid_array.push(row.participantid);
            stationid_array.push(row.stationid);
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
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
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(stationid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
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
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Unique participant identifier
    pub participantid: String,
    /// Version no of record within the effective date
    pub versionno: rust_decimal::Decimal,
    /// User authorising record
    pub authorisedby: Option<String>,
    /// Date record authorised
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ParticipantRegistrationStationownertrk1 {
    type PrimaryKey = ParticipantRegistrationStationownertrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "PARTICIPANT_REGISTRATION".into(),
            table_name: Some("STATIONOWNERTRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ParticipantRegistrationStationownertrk1PrimaryKey {
        ParticipantRegistrationStationownertrk1PrimaryKey {
            effectivedate: self.effectivedate,
            participantid: self.participantid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "participant_registration_stationownertrk_v1".to_string()
    }
}
impl crate::CompareWithRow for ParticipantRegistrationStationownertrk1 {
    type Row = ParticipantRegistrationStationownertrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationStationownertrk1 {
    type PrimaryKey = ParticipantRegistrationStationownertrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticipantRegistrationStationownertrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ParticipantRegistrationStationownertrk1PrimaryKey {
    type Row = ParticipantRegistrationStationownertrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ParticipantRegistrationStationownertrk1PrimaryKey {
    type PrimaryKey = ParticipantRegistrationStationownertrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for ParticipantRegistrationStationownertrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ParticipantRegistrationStationownertrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
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
        let mut participantid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            participantid_array.push(row.participantid);
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            authorisedby_array.push(row.authorisedby);
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
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
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
