/// # Summary
///
/// ## MTPASA_RESERVELIMIT
///  _MT PASA input table defining a MT PASA Reserve Requirement within a single set. An MT PASA Reserve Requirement can span more than one region._
///
/// * Data Set Name: Mtpasa
/// * File Name: Reservelimit
/// * Data Version: 1
///
/// # Description
///  Source MTPASA_RESERVELIMIT is updated on an ad hoc basis when a new Reserve Requirement is published. Volume ~20 rows per year
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * RESERVELIMITID
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaReservelimit1 {
    /// Trade date when the set of reserve requirements become effective
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Timestamp when the set of reserve requirements become effective
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// MT PASA Reserve Requirement identifier
    pub reservelimitid: String,
    /// Description of this Reserve Requirement
    pub description: Option<String>,
    /// Right hand side value for this Reserve requirement
    pub rhs: Option<rust_decimal::Decimal>,
    /// Timestamp the record was last modified.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MtpasaReservelimit1 {
    type PrimaryKey = MtpasaReservelimit1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("RESERVELIMIT".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MtpasaReservelimit1PrimaryKey {
        MtpasaReservelimit1PrimaryKey {
            effectivedate: self.effectivedate,
            reservelimitid: self.reservelimitid.clone(),
            version_datetime: self.version_datetime,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mtpasa_reservelimit_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MtpasaReservelimit1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub reservelimitid: String,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MtpasaReservelimit1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MtpasaReservelimit1 {
    type Row = MtpasaReservelimit1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.reservelimitid == row.reservelimitid
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaReservelimit1 {
    type PrimaryKey = MtpasaReservelimit1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.reservelimitid == key.reservelimitid
            && self.version_datetime == key.version_datetime
    }
}
impl mmsdm_core::CompareWithRow for MtpasaReservelimit1PrimaryKey {
    type Row = MtpasaReservelimit1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.reservelimitid == row.reservelimitid
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaReservelimit1PrimaryKey {
    type PrimaryKey = MtpasaReservelimit1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.reservelimitid == key.reservelimitid
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaReservelimit1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("version_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("reservelimitid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("description",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("rhs",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
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
        let mut effectivedate_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut reservelimitid_array = Vec::new();
        let mut description_array = Vec::new();
        let mut rhs_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            version_datetime_array.push(row.version_datetime.timestamp());
            reservelimitid_array.push(row.reservelimitid);
            description_array.push(row.description);
            rhs_array
                .push({
                    row.rhs
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(version_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(reservelimitid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(description_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rhs_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
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
/// ## MTPASA_RESERVELIMIT_REGION
///  _MT PASA input table to define the regions that are part of a single MT PASA Reserve Requirement_
///
/// * Data Set Name: Mtpasa
/// * File Name: Reservelimit Region
/// * Data Version: 1
///
/// # Description
///  Source MTPASA_RESERVELIMIT_REGION is updated on an ad hoc basis when a new Reserve Requirement is published. Volume ~50 rows per year
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * REGIONID
/// * RESERVELIMITID
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaReservelimitRegion1 {
    /// Trade date when the set of reserve requirements become effective
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Timestamp when the set of reserve requirements become effective
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// MT PASA Reserve requirement identifier
    pub reservelimitid: String,
    /// Region ID - identifier of a NEM region included in this requirement
    pub regionid: String,
    /// Coefficient for the region in this reserve requirement
    pub coef: Option<rust_decimal::Decimal>,
    /// Timestamp the record was last modified
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MtpasaReservelimitRegion1 {
    type PrimaryKey = MtpasaReservelimitRegion1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("RESERVELIMIT_REGION".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MtpasaReservelimitRegion1PrimaryKey {
        MtpasaReservelimitRegion1PrimaryKey {
            effectivedate: self.effectivedate,
            regionid: self.regionid.clone(),
            reservelimitid: self.reservelimitid.clone(),
            version_datetime: self.version_datetime,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mtpasa_reservelimit_region_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MtpasaReservelimitRegion1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub regionid: String,
    pub reservelimitid: String,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MtpasaReservelimitRegion1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MtpasaReservelimitRegion1 {
    type Row = MtpasaReservelimitRegion1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate && self.regionid == row.regionid
            && self.reservelimitid == row.reservelimitid
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaReservelimitRegion1 {
    type PrimaryKey = MtpasaReservelimitRegion1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.regionid == key.regionid
            && self.reservelimitid == key.reservelimitid
            && self.version_datetime == key.version_datetime
    }
}
impl mmsdm_core::CompareWithRow for MtpasaReservelimitRegion1PrimaryKey {
    type Row = MtpasaReservelimitRegion1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate && self.regionid == row.regionid
            && self.reservelimitid == row.reservelimitid
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaReservelimitRegion1PrimaryKey {
    type PrimaryKey = MtpasaReservelimitRegion1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.regionid == key.regionid
            && self.reservelimitid == key.reservelimitid
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaReservelimitRegion1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("version_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("reservelimitid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("coef",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
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
        let mut effectivedate_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut reservelimitid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut coef_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            version_datetime_array.push(row.version_datetime.timestamp());
            reservelimitid_array.push(row.reservelimitid);
            regionid_array.push(row.regionid);
            coef_array
                .push({
                    row.coef
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(version_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(reservelimitid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(coef_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
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
/// ## MTPASA_RESERVELIMIT_SET
///  _MT PASA input table defining a set of MT PASA Reserve Requirements. Note only one set can be active on a given date._
///
/// * Data Set Name: Mtpasa
/// * File Name: Reservelimit Set
/// * Data Version: 1
///
/// # Description
///  Source MTPASA_RESERVELIMIT_SET is updated on an ad hoc basis when a new Reserve Requirement is published. Volume ~2 rows per year
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MtpasaReservelimitSet1 {
    /// Trade date when the set of reserve requirements become effective
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Timestamp when the set of reserve requirements become effective
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// MT PASA LRC Reserve Requirement Set Identifier
    pub reservelimit_set_id: Option<String>,
    /// Description of this set of Reserve Requirements
    pub description: Option<String>,
    /// Date the requirement set was authorised
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorising this requirement set
    pub authorisedby: Option<String>,
    /// Timestamp the record was last modified
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MtpasaReservelimitSet1 {
    type PrimaryKey = MtpasaReservelimitSet1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MTPASA".into(),
            table_name: Some("RESERVELIMIT_SET".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MtpasaReservelimitSet1PrimaryKey {
        MtpasaReservelimitSet1PrimaryKey {
            effectivedate: self.effectivedate,
            version_datetime: self.version_datetime,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "mtpasa_reservelimit_set_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MtpasaReservelimitSet1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub version_datetime: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MtpasaReservelimitSet1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MtpasaReservelimitSet1 {
    type Row = MtpasaReservelimitSet1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaReservelimitSet1 {
    type PrimaryKey = MtpasaReservelimitSet1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.version_datetime == key.version_datetime
    }
}
impl mmsdm_core::CompareWithRow for MtpasaReservelimitSet1PrimaryKey {
    type Row = MtpasaReservelimitSet1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.version_datetime == row.version_datetime
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MtpasaReservelimitSet1PrimaryKey {
    type PrimaryKey = MtpasaReservelimitSet1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.version_datetime == key.version_datetime
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MtpasaReservelimitSet1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("version_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("reservelimit_set_id",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("description",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("authoriseddate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("authorisedby",
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
        let mut effectivedate_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut reservelimit_set_id_array = Vec::new();
        let mut description_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            version_datetime_array.push(row.version_datetime.timestamp());
            reservelimit_set_id_array.push(row.reservelimit_set_id);
            description_array.push(row.description);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            authorisedby_array.push(row.authorisedby);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(version_datetime_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(reservelimit_set_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(description_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(authorisedby_array)) as std::sync::Arc < dyn
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
/// ## RESERVE
///  _RESERVE sets out specific reserve requirements for dispatch, predispatch and STPASA, for each half-hour interval by region. Updates show as new versions for a date.<br>_
///
/// * Data Set Name: Reserve Data
/// * File Name: Reserve
/// * Data Version: 1
///
/// # Description
///  Two fields specify Frequency Controlled Ancillary Services requirements for the regulation ancillary services. Another two fields specify the Lack of Reserve levels to be applied in the ST PASA solver.  Change Notice 324 (for the FCAS Constraint enhancements project) means that Dispatch no longer utilises the static FCAS requirements defined in the DELTAMW and RESERVE tables. These tables are replaced with constraint data as a source of FCAS requirements. RESERVE data is public, so is available to all participants. Source RESERVE updates as AEMO updates forecasts, daily.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ReserveDataReserve1 {
    /// Market date starting at 04:00am
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Version No of record for this date, the version of the file loaded to produce these reserve figures
    pub versionno: rust_decimal::Decimal,
    /// Differentiates this region from all other regions
    pub regionid: String,
    /// Market Trading Interval
    pub periodid: rust_decimal::Decimal,
    /// Lower 5 minute reserve requirement
    pub lower5min: Option<rust_decimal::Decimal>,
    /// Lower 60 second reserve requirement
    pub lower60sec: Option<rust_decimal::Decimal>,
    /// Lower 6 second reserve requirement
    pub lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 5 minute reserve requirement
    pub raise5min: Option<rust_decimal::Decimal>,
    /// Raise 60 second reserve requirement
    pub raise60sec: Option<rust_decimal::Decimal>,
    /// Raise 6 second reserve requirement
    pub raise6sec: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// PASA reserve requirement
    pub pasareserve: Option<rust_decimal::Decimal>,
    /// PASA Load rejection reserve requirement
    pub loadrejectionreservereq: Option<rust_decimal::Decimal>,
    /// Raise Regulation reserve requirement
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Lower Regulation reserve requirement
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// PASA Lack of Reserve 1 Level
    pub lor1level: Option<rust_decimal::Decimal>,
    /// PASA Lack of Reserve 1 Level
    pub lor2level: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for ReserveDataReserve1 {
    type PrimaryKey = ReserveDataReserve1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "RESERVE_DATA".into(),
            table_name: Some("RESERVE".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> ReserveDataReserve1PrimaryKey {
        ReserveDataReserve1PrimaryKey {
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::Datelike::month(&self.settlementdate),
                )
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "reserve_data_reserve_v1_{}_{}", chrono::Datelike::year(& self
            .settlementdate), chrono::Datelike::month(& self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ReserveDataReserve1PrimaryKey {
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for ReserveDataReserve1PrimaryKey {}
impl mmsdm_core::CompareWithRow for ReserveDataReserve1 {
    type Row = ReserveDataReserve1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.periodid == row.periodid && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ReserveDataReserve1 {
    type PrimaryKey = ReserveDataReserve1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for ReserveDataReserve1PrimaryKey {
    type Row = ReserveDataReserve1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.periodid == row.periodid && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for ReserveDataReserve1PrimaryKey {
    type PrimaryKey = ReserveDataReserve1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for ReserveDataReserve1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(2, 0), false),
                arrow2::datatypes::Field::new("lower5min",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("lower60sec",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("lower6sec",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("raise5min",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("raise60sec",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("raise6sec",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("pasareserve",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("loadrejectionreservereq",
                arrow2::datatypes::DataType::Decimal(10, 0), true),
                arrow2::datatypes::Field::new("raisereg",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("lowerreg",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("lor1level",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("lor2level",
                arrow2::datatypes::DataType::Decimal(6, 0), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut lower5min_array = Vec::new();
        let mut lower60sec_array = Vec::new();
        let mut lower6sec_array = Vec::new();
        let mut raise5min_array = Vec::new();
        let mut raise60sec_array = Vec::new();
        let mut raise6sec_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut pasareserve_array = Vec::new();
        let mut loadrejectionreservereq_array = Vec::new();
        let mut raisereg_array = Vec::new();
        let mut lowerreg_array = Vec::new();
        let mut lor1level_array = Vec::new();
        let mut lor2level_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            regionid_array.push(row.regionid);
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            lower5min_array
                .push({
                    row.lower5min
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lower60sec_array
                .push({
                    row.lower60sec
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lower6sec_array
                .push({
                    row.lower6sec
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            raise5min_array
                .push({
                    row.raise5min
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            raise60sec_array
                .push({
                    row.raise60sec
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            raise6sec_array
                .push({
                    row.raise6sec
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            pasareserve_array
                .push({
                    row.pasareserve
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            loadrejectionreservereq_array
                .push({
                    row.loadrejectionreservereq
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            raisereg_array
                .push({
                    row.raisereg
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lowerreg_array
                .push({
                    row.lowerreg
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lor1level_array
                .push({
                    row.lor1level
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lor2level_array
                .push({
                    row.lor2level
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(2, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5min_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5min_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(pasareserve_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(loadrejectionreservereq_array)
                    .to(arrow2::datatypes::DataType::Decimal(10, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raisereg_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowerreg_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lor1level_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lor2level_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
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
        (Some("RESERVELIMIT"), version) if version <= 1_i32 => {
            let d: Vec<MtpasaReservelimit1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMtpasaReservelimit1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("RESERVELIMIT_REGION"), version) if version <= 1_i32 => {
            let d: Vec<MtpasaReservelimitRegion1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMtpasaReservelimitRegion1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("RESERVELIMIT_SET"), version) if version <= 1_i32 => {
            let d: Vec<MtpasaReservelimitSet1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMtpasaReservelimitSet1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("RESERVE"), version) if version <= 1_i32 => {
            let d: Vec<ReserveDataReserve1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertReserveDataReserve1 @P1, @P2",
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
