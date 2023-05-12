#[allow(unused_imports)]
use chrono::Datelike as _;
/// # Summary
///
/// ## BIDTYPES
///  _BIDTYPES, together with the associated tracking data in BIDTYPESTRK, define a set of ancillary services with bidding parameters from a given date.<br>BIDTYPES is static data describing each type of bid quantity, the number of applicable bands, how many days ahead a price lock down becomes effective and the validation rule that applies.<br>_
///
/// * Data Set Name: Market Config
/// * File Name: Bidtypes
/// * Data Version: 1
///
/// # Description
///  BIDTYPES  is public to participants Source BIDTYPES updates when the static data relating to an ancillary service type is modified. Volume Expect modifications to be rare. Allow for approximately 20 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigBidtypes1 {
    /// Bid Type Identifier
    pub bidtype: String,
    /// Market date starting at 04:30 inclusive
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Record version number
    pub versionno: rust_decimal::Decimal,
    /// Description of this Bid Type
    pub description: Option<String>,
    /// Number of active bands (1 to 10)
    pub numberofbands: Option<rust_decimal::Decimal>,
    /// Number of days prior to the Market Day when prices are locked from 12:30pm
    pub numdaysaheadpricelocked: Option<rust_decimal::Decimal>,
    /// ENERGY or AS validation rules to apply.
    pub validationrule: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Alias for this BIDTYPE used in the SPD Solver
    pub spdalias: Option<String>,
}
impl mmsdm_core::GetTable for MarketConfigBidtypes1 {
    type PrimaryKey = MarketConfigBidtypes1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MARKET_CONFIG".into(),
            table_name: Some("BIDTYPES".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MarketConfigBidtypes1PrimaryKey {
        MarketConfigBidtypes1PrimaryKey {
            bidtype: self.bidtype.clone(),
            effectivedate: self.effectivedate,
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.effectivedate.year(),
            month: num_traits::FromPrimitive::from_u32(self.effectivedate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "market_config_bidtypes_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MarketConfigBidtypes1PrimaryKey {
    pub bidtype: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigBidtypes1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MarketConfigBidtypes1 {
    type Row = MarketConfigBidtypes1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigBidtypes1 {
    type PrimaryKey = MarketConfigBidtypes1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for MarketConfigBidtypes1PrimaryKey {
    type Row = MarketConfigBidtypes1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype && self.effectivedate == row.effectivedate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigBidtypes1PrimaryKey {
    type PrimaryKey = MarketConfigBidtypes1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.effectivedate == key.effectivedate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigBidtypes1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("bidtype",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("description",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("numberofbands",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("numdaysaheadpricelocked",
                arrow2::datatypes::DataType::Decimal(2, 0), true),
                arrow2::datatypes::Field::new("validationrule",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("spdalias",
                arrow2::datatypes::DataType::LargeUtf8, true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut bidtype_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut description_array = Vec::new();
        let mut numberofbands_array = Vec::new();
        let mut numdaysaheadpricelocked_array = Vec::new();
        let mut validationrule_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut spdalias_array = Vec::new();
        for row in partition {
            bidtype_array.push(row.bidtype);
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            description_array.push(row.description);
            numberofbands_array
                .push({
                    row.numberofbands
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            numdaysaheadpricelocked_array
                .push({
                    row.numdaysaheadpricelocked
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            validationrule_array.push(row.validationrule);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            spdalias_array.push(row.spdalias);
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(bidtype_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(description_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(numberofbands_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(numdaysaheadpricelocked_array)
                    .to(arrow2::datatypes::DataType::Decimal(2, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(validationrule_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(spdalias_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BIDTYPESTRK
///  _BIDTYPESTRK, together with the associated data in BIDTYPES, define a set of ancillary services with bidding parameters from a given date._
///
/// * Data Set Name: Market Config
/// * File Name: Bidtypestrk
/// * Data Version: 1
///
/// # Description
///  BIDTYPESTRK is public to participants Source BIDTYPESTRK updates when the static data relating to an ancillary service type is modified. Volume Expect modifications to be rare. Allow for approximately 20 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigBidtypestrk1 {
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
impl mmsdm_core::GetTable for MarketConfigBidtypestrk1 {
    type PrimaryKey = MarketConfigBidtypestrk1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MARKET_CONFIG".into(),
            table_name: Some("BIDTYPESTRK".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MarketConfigBidtypestrk1PrimaryKey {
        MarketConfigBidtypestrk1PrimaryKey {
            effectivedate: self.effectivedate,
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.effectivedate.year(),
            month: num_traits::FromPrimitive::from_u32(self.effectivedate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "market_config_bidtypestrk_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MarketConfigBidtypestrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigBidtypestrk1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MarketConfigBidtypestrk1 {
    type Row = MarketConfigBidtypestrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigBidtypestrk1 {
    type PrimaryKey = MarketConfigBidtypestrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for MarketConfigBidtypestrk1PrimaryKey {
    type Row = MarketConfigBidtypestrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigBidtypestrk1PrimaryKey {
    type PrimaryKey = MarketConfigBidtypestrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigBidtypestrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
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
        let mut versionno_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            authorisedby_array.push(row.authorisedby);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
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
/// ## INTERCONNECTOR
///  _INTERCONNECTOR sets out valid identifiers for each interconnector._
///
/// * Data Set Name: Market Config
/// * File Name: Interconnector
/// * Data Version: 1
///
/// # Description
///  INTERCONNECTOR is public data, available to all participants. Source INTERCONNECTOR changes infrequently, usually annually.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERCONNECTORID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigInterconnector1 {
    /// Unique Id of this interconnector
    pub interconnectorid: String,
    /// Starting region of the interconnect
    pub regionfrom: Option<String>,
    /// Not used
    pub rsoid: Option<String>,
    /// Ending region of the interconnect
    pub regionto: Option<String>,
    /// Description of interconnector
    pub description: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MarketConfigInterconnector1 {
    type PrimaryKey = MarketConfigInterconnector1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MARKET_CONFIG".into(),
            table_name: Some("INTERCONNECTOR".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MarketConfigInterconnector1PrimaryKey {
        MarketConfigInterconnector1PrimaryKey {
            interconnectorid: self.interconnectorid.clone(),
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "market_config_interconnector_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MarketConfigInterconnector1PrimaryKey {
    pub interconnectorid: String,
}
impl mmsdm_core::PrimaryKey for MarketConfigInterconnector1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MarketConfigInterconnector1 {
    type Row = MarketConfigInterconnector1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interconnectorid == row.interconnectorid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigInterconnector1 {
    type PrimaryKey = MarketConfigInterconnector1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
    }
}
impl mmsdm_core::CompareWithRow for MarketConfigInterconnector1PrimaryKey {
    type Row = MarketConfigInterconnector1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interconnectorid == row.interconnectorid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigInterconnector1PrimaryKey {
    type PrimaryKey = MarketConfigInterconnector1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigInterconnector1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("regionfrom",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("rsoid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("regionto",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("description",
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
        let mut interconnectorid_array = Vec::new();
        let mut regionfrom_array = Vec::new();
        let mut rsoid_array = Vec::new();
        let mut regionto_array = Vec::new();
        let mut description_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            interconnectorid_array.push(row.interconnectorid);
            regionfrom_array.push(row.regionfrom);
            rsoid_array.push(row.rsoid);
            regionto_array.push(row.regionto);
            description_array.push(row.description);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(interconnectorid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(regionfrom_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(rsoid_array)) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(regionto_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >, std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(description_array)) as std::sync::Arc < dyn
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
/// ## INTERCONNECTORALLOC
///  _INTERCONNECTORALLOC shows allocations of interconnector residues to Network Service Providers._
///
/// * Data Set Name: Market Config
/// * File Name: Interconnectoralloc
/// * Data Version: 1
///
/// # Description
///  INTERCONNECTORALLOC data is confidential to the relevant participant. Source INTERCONNECTORALLOC changes infrequently, typically annually.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigInterconnectoralloc1 {
    /// Effective Date of Allocation Details
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version No in respect to effective date
    pub versionno: rust_decimal::Decimal,
    /// Interconnector identifier
    pub interconnectorid: String,
    /// Region Identifier
    pub regionid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Allocation % / 100
    pub allocation: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MarketConfigInterconnectoralloc1 {
    type PrimaryKey = MarketConfigInterconnectoralloc1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MARKET_CONFIG".into(),
            table_name: Some("INTERCONNECTORALLOC".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MarketConfigInterconnectoralloc1PrimaryKey {
        MarketConfigInterconnectoralloc1PrimaryKey {
            effectivedate: self.effectivedate,
            interconnectorid: self.interconnectorid.clone(),
            participantid: self.participantid.clone(),
            regionid: self.regionid.clone(),
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.effectivedate.year(),
            month: num_traits::FromPrimitive::from_u32(self.effectivedate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "market_config_interconnectoralloc_v1_{}_{}", self.partition_suffix().year,
            self.partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MarketConfigInterconnectoralloc1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub interconnectorid: String,
    pub participantid: String,
    pub regionid: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigInterconnectoralloc1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MarketConfigInterconnectoralloc1 {
    type Row = MarketConfigInterconnectoralloc1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigInterconnectoralloc1 {
    type PrimaryKey = MarketConfigInterconnectoralloc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for MarketConfigInterconnectoralloc1PrimaryKey {
    type Row = MarketConfigInterconnectoralloc1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigInterconnectoralloc1PrimaryKey {
    type PrimaryKey = MarketConfigInterconnectoralloc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigInterconnectoralloc1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(5, 0), false),
                arrow2::datatypes::Field::new("interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("allocation",
                arrow2::datatypes::DataType::Decimal(12, 5), true),
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
        let mut versionno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut allocation_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            interconnectorid_array.push(row.interconnectorid);
            regionid_array.push(row.regionid);
            participantid_array.push(row.participantid);
            allocation_array
                .push({
                    row.allocation
                        .map(|mut val| {
                            val.rescale(5);
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
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(5, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(interconnectorid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(allocation_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 5))) as std::sync::Arc <
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
/// ## INTERCONNECTORCONSTRAINT
///  _INTERCONNECTORCONSTRAINT sets out Interconnector limit data used as defaults in dispatch, predispatch and STPASA and used by SPD in calculating flows. INTERCONNECTORCONSTRAINT includes an additional field to restrict an interconnector from support transfer of FCAS._
///
/// * Data Set Name: Market Config
/// * File Name: Interconnectorconstraint
/// * Data Version: 1
///
/// # Description
///  INTERCONNECTORCONSTRAINT is public data, available to all participants. Source INTERCONNECTORCONSTRAINT changes infrequently, typically annually.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * INTERCONNECTORID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigInterconnectorconstraint1 {
    /// SPD Factor
    pub reserveoverallloadfactor: Option<rust_decimal::Decimal>,
    /// Loss share attributable to from region
    pub fromregionlossshare: Option<rust_decimal::Decimal>,
    /// Date that this limit is effective from
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version for this date
    pub versionno: rust_decimal::Decimal,
    /// Unique Id of this interconnector
    pub interconnectorid: String,
    /// Limit of energy flowing into the RegionFrom
    pub maxmwin: Option<rust_decimal::Decimal>,
    /// Limit of energy flowing out of the Region
    pub maxmwout: Option<rust_decimal::Decimal>,
    /// Constant Loss factor
    pub lossconstant: Option<rust_decimal::Decimal>,
    /// Linear coefficient of loss factor calculation
    pub lossflowcoefficient: Option<rust_decimal::Decimal>,
    /// Identifies the EMS entity that represents the interconnector flow
    pub emsmeasurand: Option<String>,
    /// User authorising record
    pub authorisedby: Option<String>,
    /// Date record authorised
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Not used
    pub dynamicrhs: Option<String>,
    /// Interconnector import limit
    pub importlimit: Option<rust_decimal::Decimal>,
    /// Interconnector export limit
    pub exportlimit: Option<rust_decimal::Decimal>,
    /// SPD Factor
    pub outagederationfactor: Option<rust_decimal::Decimal>,
    /// Factor for non-physical losses rerun
    pub nonphysicallossfactor: Option<rust_decimal::Decimal>,
    /// Interconnector overload for 60 sec
    pub overloadfactor60sec: Option<rust_decimal::Decimal>,
    /// Interconnector overload for 6 sec
    pub overloadfactor6sec: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag to indicate that the interconnector cannot support FCAS Transfers
    pub fcassupportunavailable: Option<rust_decimal::Decimal>,
    /// Interconnector type - Currently either "REGULATED" or "MNSP"
    pub ictype: Option<String>,
}
impl mmsdm_core::GetTable for MarketConfigInterconnectorconstraint1 {
    type PrimaryKey = MarketConfigInterconnectorconstraint1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MARKET_CONFIG".into(),
            table_name: Some("INTERCONNECTORCONSTRAINT".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MarketConfigInterconnectorconstraint1PrimaryKey {
        MarketConfigInterconnectorconstraint1PrimaryKey {
            effectivedate: self.effectivedate,
            interconnectorid: self.interconnectorid.clone(),
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.effectivedate.year(),
            month: num_traits::FromPrimitive::from_u32(self.effectivedate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "market_config_interconnectorconstraint_v1_{}_{}", self.partition_suffix()
            .year, self.partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MarketConfigInterconnectorconstraint1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub interconnectorid: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigInterconnectorconstraint1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MarketConfigInterconnectorconstraint1 {
    type Row = MarketConfigInterconnectorconstraint1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid == row.interconnectorid
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigInterconnectorconstraint1 {
    type PrimaryKey = MarketConfigInterconnectorconstraint1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid == key.interconnectorid
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for MarketConfigInterconnectorconstraint1PrimaryKey {
    type Row = MarketConfigInterconnectorconstraint1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid == row.interconnectorid
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for MarketConfigInterconnectorconstraint1PrimaryKey {
    type PrimaryKey = MarketConfigInterconnectorconstraint1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid == key.interconnectorid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigInterconnectorconstraint1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("reserveoverallloadfactor",
                arrow2::datatypes::DataType::Decimal(5, 2), true),
                arrow2::datatypes::Field::new("fromregionlossshare",
                arrow2::datatypes::DataType::Decimal(5, 2), true),
                arrow2::datatypes::Field::new("effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("maxmwin",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("maxmwout",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lossconstant",
                arrow2::datatypes::DataType::Decimal(15, 6), true),
                arrow2::datatypes::Field::new("lossflowcoefficient",
                arrow2::datatypes::DataType::Decimal(27, 17), true),
                arrow2::datatypes::Field::new("emsmeasurand",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("authorisedby",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("authoriseddate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("dynamicrhs",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("importlimit",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("exportlimit",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("outagederationfactor",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("nonphysicallossfactor",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("overloadfactor60sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("overloadfactor6sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("fcassupportunavailable",
                arrow2::datatypes::DataType::Decimal(1, 0), true),
                arrow2::datatypes::Field::new("ictype",
                arrow2::datatypes::DataType::LargeUtf8, true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut reserveoverallloadfactor_array = Vec::new();
        let mut fromregionlossshare_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut maxmwin_array = Vec::new();
        let mut maxmwout_array = Vec::new();
        let mut lossconstant_array = Vec::new();
        let mut lossflowcoefficient_array = Vec::new();
        let mut emsmeasurand_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut dynamicrhs_array = Vec::new();
        let mut importlimit_array = Vec::new();
        let mut exportlimit_array = Vec::new();
        let mut outagederationfactor_array = Vec::new();
        let mut nonphysicallossfactor_array = Vec::new();
        let mut overloadfactor60sec_array = Vec::new();
        let mut overloadfactor6sec_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut fcassupportunavailable_array = Vec::new();
        let mut ictype_array = Vec::new();
        for row in partition {
            reserveoverallloadfactor_array
                .push({
                    row.reserveoverallloadfactor
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            fromregionlossshare_array
                .push({
                    row.fromregionlossshare
                        .map(|mut val| {
                            val.rescale(2);
                            val.mantissa()
                        })
                });
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            interconnectorid_array.push(row.interconnectorid);
            maxmwin_array
                .push({
                    row.maxmwin
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            maxmwout_array
                .push({
                    row.maxmwout
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lossconstant_array
                .push({
                    row.lossconstant
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            lossflowcoefficient_array
                .push({
                    row.lossflowcoefficient
                        .map(|mut val| {
                            val.rescale(17);
                            val.mantissa()
                        })
                });
            emsmeasurand_array.push(row.emsmeasurand);
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            dynamicrhs_array.push(row.dynamicrhs);
            importlimit_array
                .push({
                    row.importlimit
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            exportlimit_array
                .push({
                    row.exportlimit
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            outagederationfactor_array
                .push({
                    row.outagederationfactor
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            nonphysicallossfactor_array
                .push({
                    row.nonphysicallossfactor
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            overloadfactor60sec_array
                .push({
                    row.overloadfactor60sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            overloadfactor6sec_array
                .push({
                    row.overloadfactor6sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            fcassupportunavailable_array
                .push({
                    row.fcassupportunavailable
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            ictype_array.push(row.ictype);
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(reserveoverallloadfactor_array)
                    .to(arrow2::datatypes::DataType::Decimal(5, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(fromregionlossshare_array)
                    .to(arrow2::datatypes::DataType::Decimal(5, 2))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(interconnectorid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxmwin_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(maxmwout_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lossconstant_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lossflowcoefficient_array)
                    .to(arrow2::datatypes::DataType::Decimal(27, 17))) as std::sync::Arc
                    < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(emsmeasurand_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(authorisedby_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(dynamicrhs_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(importlimit_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(exportlimit_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(outagederationfactor_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(nonphysicallossfactor_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(overloadfactor60sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(overloadfactor6sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(fcassupportunavailable_array)
                    .to(arrow2::datatypes::DataType::Decimal(1, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(ictype_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## INTRAREGIONALLOC
///  _INTRAREGIONALLOC shows allocations of intra-regional residues to participants._
///
/// * Data Set Name: Market Config
/// * File Name: Intraregionalloc
/// * Data Version: 1
///
/// # Description
///  INTRAREGIONALLOC data is confidential to the relevant participant. Source The data in INTRAREGIONALLOC changes infrequently.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PARTICIPANTID
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigIntraregionalloc1 {
    /// Effective Date of Allocation Details
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version No in respect to effective date
    pub versionno: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Allocation Percent / 100
    pub allocation: Option<rust_decimal::Decimal>,
    /// Last changed date/time
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MarketConfigIntraregionalloc1 {
    type PrimaryKey = MarketConfigIntraregionalloc1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MARKET_CONFIG".into(),
            table_name: Some("INTRAREGIONALLOC".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MarketConfigIntraregionalloc1PrimaryKey {
        MarketConfigIntraregionalloc1PrimaryKey {
            effectivedate: self.effectivedate,
            participantid: self.participantid.clone(),
            regionid: self.regionid.clone(),
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.effectivedate.year(),
            month: num_traits::FromPrimitive::from_u32(self.effectivedate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "market_config_intraregionalloc_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MarketConfigIntraregionalloc1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: String,
    pub regionid: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigIntraregionalloc1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MarketConfigIntraregionalloc1 {
    type Row = MarketConfigIntraregionalloc1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigIntraregionalloc1 {
    type PrimaryKey = MarketConfigIntraregionalloc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for MarketConfigIntraregionalloc1PrimaryKey {
    type Row = MarketConfigIntraregionalloc1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigIntraregionalloc1PrimaryKey {
    type PrimaryKey = MarketConfigIntraregionalloc1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigIntraregionalloc1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(5, 0), false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("allocation",
                arrow2::datatypes::DataType::Decimal(12, 5), true),
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
        let mut versionno_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut allocation_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            regionid_array.push(row.regionid);
            participantid_array.push(row.participantid);
            allocation_array
                .push({
                    row.allocation
                        .map(|mut val| {
                            val.rescale(5);
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
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(5, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(allocation_array)
                    .to(arrow2::datatypes::DataType::Decimal(12, 5))) as std::sync::Arc <
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
/// ## LOSSFACTORMODEL
///  _LOSSFACTORMODEL sets out the demand coefficients for each interconnector, used by LP Solver modelling of interconnector flows._
///
/// * Data Set Name: Market Config
/// * File Name: Lossfactormodel
/// * Data Version: 1
///
/// # Description
///  LOSSFACTORMODEL is public data, so is available to all participants. Source LOSSFACTORMODEL only changes annually, when there is a change in the interconnector.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * INTERCONNECTORID
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigLossfactormodel1 {
    /// Calendar date data set is effective
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number within effective date of the status proposed
    pub versionno: rust_decimal::Decimal,
    /// The unique identifier for the interconnector.
    pub interconnectorid: String,
    /// The unique region identifier for a connection point of the interconnector
    pub regionid: String,
    /// The coefficient applied to the region demand in the calculation of the interconnector loss factor
    pub demandcoefficient: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MarketConfigLossfactormodel1 {
    type PrimaryKey = MarketConfigLossfactormodel1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MARKET_CONFIG".into(),
            table_name: Some("LOSSFACTORMODEL".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MarketConfigLossfactormodel1PrimaryKey {
        MarketConfigLossfactormodel1PrimaryKey {
            effectivedate: self.effectivedate,
            interconnectorid: self.interconnectorid.clone(),
            regionid: self.regionid.clone(),
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.effectivedate.year(),
            month: num_traits::FromPrimitive::from_u32(self.effectivedate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "market_config_lossfactormodel_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MarketConfigLossfactormodel1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub interconnectorid: String,
    pub regionid: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigLossfactormodel1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MarketConfigLossfactormodel1 {
    type Row = MarketConfigLossfactormodel1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid == row.interconnectorid
            && self.regionid == row.regionid && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigLossfactormodel1 {
    type PrimaryKey = MarketConfigLossfactormodel1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid == key.interconnectorid
            && self.regionid == key.regionid && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for MarketConfigLossfactormodel1PrimaryKey {
    type Row = MarketConfigLossfactormodel1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid == row.interconnectorid
            && self.regionid == row.regionid && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigLossfactormodel1PrimaryKey {
    type PrimaryKey = MarketConfigLossfactormodel1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid == key.interconnectorid
            && self.regionid == key.regionid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigLossfactormodel1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("demandcoefficient",
                arrow2::datatypes::DataType::Decimal(27, 17), true),
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
        let mut versionno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut demandcoefficient_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            interconnectorid_array.push(row.interconnectorid);
            regionid_array.push(row.regionid);
            demandcoefficient_array
                .push({
                    row.demandcoefficient
                        .map(|mut val| {
                            val.rescale(17);
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
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(interconnectorid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(demandcoefficient_array)
                    .to(arrow2::datatypes::DataType::Decimal(27, 17))) as std::sync::Arc
                    < dyn arrow2::array::Array >,
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
/// ## LOSSMODEL
///  _LOSSMODEL sets out segment breakpoints in loss model for each interconnector, used by LP Solver modelling of interconnector flows._
///
/// * Data Set Name: Market Config
/// * File Name: Lossmodel
/// * Data Version: 1
///
/// # Description
///  LOSSMODEL data is public, so is available to all participants. Source LOSSMODEL only changes annually, when there is a change in the interconnector.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * INTERCONNECTORID
/// * LOSSSEGMENT
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigLossmodel1 {
    /// Calendar date data set is effective
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number within effective date
    pub versionno: rust_decimal::Decimal,
    /// Interconnector identifier
    pub interconnectorid: String,
    /// Not used
    pub periodid: Option<String>,
    /// Segment Identifier (1 to 80 at present)
    pub losssegment: rust_decimal::Decimal,
    /// MW Value for segment
    pub mwbreakpoint: Option<rust_decimal::Decimal>,
    /// Not used
    pub lossfactor: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MarketConfigLossmodel1 {
    type PrimaryKey = MarketConfigLossmodel1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MARKET_CONFIG".into(),
            table_name: Some("LOSSMODEL".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MarketConfigLossmodel1PrimaryKey {
        MarketConfigLossmodel1PrimaryKey {
            effectivedate: self.effectivedate,
            interconnectorid: self.interconnectorid.clone(),
            losssegment: self.losssegment,
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.effectivedate.year(),
            month: num_traits::FromPrimitive::from_u32(self.effectivedate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "market_config_lossmodel_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MarketConfigLossmodel1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub interconnectorid: String,
    pub losssegment: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigLossmodel1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MarketConfigLossmodel1 {
    type Row = MarketConfigLossmodel1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid == row.interconnectorid
            && self.losssegment == row.losssegment && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigLossmodel1 {
    type PrimaryKey = MarketConfigLossmodel1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid == key.interconnectorid
            && self.losssegment == key.losssegment && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for MarketConfigLossmodel1PrimaryKey {
    type Row = MarketConfigLossmodel1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.interconnectorid == row.interconnectorid
            && self.losssegment == row.losssegment && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigLossmodel1PrimaryKey {
    type PrimaryKey = MarketConfigLossmodel1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.interconnectorid == key.interconnectorid
            && self.losssegment == key.losssegment && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigLossmodel1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("losssegment",
                arrow2::datatypes::DataType::Decimal(6, 0), false),
                arrow2::datatypes::Field::new("mwbreakpoint",
                arrow2::datatypes::DataType::Decimal(6, 0), true),
                arrow2::datatypes::Field::new("lossfactor",
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
        let mut versionno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut losssegment_array = Vec::new();
        let mut mwbreakpoint_array = Vec::new();
        let mut lossfactor_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            interconnectorid_array.push(row.interconnectorid);
            periodid_array.push(row.periodid);
            losssegment_array
                .push({
                    let mut val = row.losssegment;
                    val.rescale(0);
                    val.mantissa()
                });
            mwbreakpoint_array
                .push({
                    row.mwbreakpoint
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lossfactor_array
                .push({
                    row.lossfactor
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
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(interconnectorid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(periodid_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(losssegment_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(mwbreakpoint_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lossfactor_array)
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
/// ## MARKET_PRICE_THRESHOLDS
///  _MARKET_PRICE_THRESHOLDS sets out the market cap , floor and administered price thresholds applying to the electricity market_
///
/// * Data Set Name: Market Config
/// * File Name: Market Price Thresholds
/// * Data Version: 1
///
/// # Description
///  MARKET_PRICE_THRESHOLDS data is public, so is available to all participants. Source MARKET_PRICE_THRESHOLDS only changes when a change is made to a market price threshold. This table changes infrequently.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigMarketPriceThresholds1 {
    /// Calendar date that this record becomes effective
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// version no for the effective date
    pub versionno: rust_decimal::Decimal,
    /// value of lost load if total supply falls short of demand after load management then involuntary load
    pub voll: Option<rust_decimal::Decimal>,
    /// The floor price that the spot market price will not fall below.
    pub marketpricefloor: Option<rust_decimal::Decimal>,
    /// Threshold value beyond which Aggregate Prices per Region over 336 Trade Intervals (Energy), or 2016 Dispatch Intervals (FCAS), will result in an Administered Price declaration
    pub administered_price_threshold: Option<rust_decimal::Decimal>,
    /// date data authorised
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// user authorising
    pub authorisedby: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MarketConfigMarketPriceThresholds1 {
    type PrimaryKey = MarketConfigMarketPriceThresholds1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MARKET_CONFIG".into(),
            table_name: Some("MARKET_PRICE_THRESHOLDS".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MarketConfigMarketPriceThresholds1PrimaryKey {
        MarketConfigMarketPriceThresholds1PrimaryKey {
            effectivedate: self.effectivedate,
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.effectivedate.year(),
            month: num_traits::FromPrimitive::from_u32(self.effectivedate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "market_config_market_price_thresholds_v1_{}_{}", self.partition_suffix()
            .year, self.partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MarketConfigMarketPriceThresholds1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigMarketPriceThresholds1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MarketConfigMarketPriceThresholds1 {
    type Row = MarketConfigMarketPriceThresholds1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigMarketPriceThresholds1 {
    type PrimaryKey = MarketConfigMarketPriceThresholds1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for MarketConfigMarketPriceThresholds1PrimaryKey {
    type Row = MarketConfigMarketPriceThresholds1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigMarketPriceThresholds1PrimaryKey {
    type PrimaryKey = MarketConfigMarketPriceThresholds1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigMarketPriceThresholds1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("voll",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("marketpricefloor",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("administered_price_threshold",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
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
        let mut versionno_array = Vec::new();
        let mut voll_array = Vec::new();
        let mut marketpricefloor_array = Vec::new();
        let mut administered_price_threshold_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            voll_array
                .push({
                    row.voll
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            marketpricefloor_array
                .push({
                    row.marketpricefloor
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            administered_price_threshold_array
                .push({
                    row.administered_price_threshold
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            authorisedby_array.push(row.authorisedby);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(voll_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(marketpricefloor_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(administered_price_threshold_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
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
/// ## REGION
///  _REGION sets out valid region IDs._
///
/// * Data Set Name: Market Config
/// * File Name: Region
/// * Data Version: 1
///
/// # Description
///  REGION data is public, so is available to all participants. Source REGION updates if a change is ever made to a region. This table is static data and is likely to change very infrequently.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigRegion1 {
    /// Differentiates this region from all other regions
    pub regionid: String,
    /// Full description of region
    pub description: Option<String>,
    /// Status of the region e.g. working, inactive, archive.
    pub regionstatus: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MarketConfigRegion1 {
    type PrimaryKey = MarketConfigRegion1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MARKET_CONFIG".into(),
            table_name: Some("REGION".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MarketConfigRegion1PrimaryKey {
        MarketConfigRegion1PrimaryKey {
            regionid: self.regionid.clone(),
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "market_config_region_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MarketConfigRegion1PrimaryKey {
    pub regionid: String,
}
impl mmsdm_core::PrimaryKey for MarketConfigRegion1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MarketConfigRegion1 {
    type Row = MarketConfigRegion1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.regionid == row.regionid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigRegion1 {
    type PrimaryKey = MarketConfigRegion1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid
    }
}
impl mmsdm_core::CompareWithRow for MarketConfigRegion1PrimaryKey {
    type Row = MarketConfigRegion1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.regionid == row.regionid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigRegion1PrimaryKey {
    type PrimaryKey = MarketConfigRegion1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigRegion1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("description",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("regionstatus",
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
        let mut regionid_array = Vec::new();
        let mut description_array = Vec::new();
        let mut regionstatus_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            regionid_array.push(row.regionid);
            description_array.push(row.description);
            regionstatus_array.push(row.regionstatus);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(description_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(regionstatus_array)) as std::sync::Arc < dyn
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
/// ## REGIONSTANDINGDATA
///  _REGIONSTANDINGDATA sets out standing region data including the region reference node._
///
/// * Data Set Name: Market Config
/// * File Name: Regionstandingdata
/// * Data Version: 1
///
/// # Description
///  REGIONSTANDINGDATA data is public, so is available to all participants. Source REGIONSTANDINGDATA only changes when a change is made to a region. This table changes infrequently.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigRegionstandingdata1 {
    /// Effective date of this record, only the latest date applies
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version No of the standing data that should be effective on this date
    pub versionno: rust_decimal::Decimal,
    /// Differentiates this region from all other regions
    pub regionid: String,
    /// the unique identifier of the participant with responsibility for the region.
    pub rsoid: Option<String>,
    /// unique id of a connection point, being the reference point for this region
    pub regionalreferencepointid: Option<String>,
    /// Period identifier of the peak trading period of this connection point
    pub peaktradingperiod: Option<rust_decimal::Decimal>,
    /// Date record authorised
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorising record
    pub authorisedby: Option<String>,
    /// Scaling factor for regional FCAS requirement
    pub scalingfactor: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MarketConfigRegionstandingdata1 {
    type PrimaryKey = MarketConfigRegionstandingdata1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MARKET_CONFIG".into(),
            table_name: Some("REGIONSTANDINGDATA".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MarketConfigRegionstandingdata1PrimaryKey {
        MarketConfigRegionstandingdata1PrimaryKey {
            effectivedate: self.effectivedate,
            regionid: self.regionid.clone(),
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.effectivedate.year(),
            month: num_traits::FromPrimitive::from_u32(self.effectivedate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "market_config_regionstandingdata_v1_{}_{}", self.partition_suffix().year,
            self.partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MarketConfigRegionstandingdata1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub regionid: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigRegionstandingdata1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MarketConfigRegionstandingdata1 {
    type Row = MarketConfigRegionstandingdata1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate && self.regionid == row.regionid
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigRegionstandingdata1 {
    type PrimaryKey = MarketConfigRegionstandingdata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for MarketConfigRegionstandingdata1PrimaryKey {
    type Row = MarketConfigRegionstandingdata1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate && self.regionid == row.regionid
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigRegionstandingdata1PrimaryKey {
    type PrimaryKey = MarketConfigRegionstandingdata1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigRegionstandingdata1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("rsoid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("regionalreferencepointid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("peaktradingperiod",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
                arrow2::datatypes::Field::new("authoriseddate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("authorisedby",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("scalingfactor",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
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
        let mut versionno_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut rsoid_array = Vec::new();
        let mut regionalreferencepointid_array = Vec::new();
        let mut peaktradingperiod_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut scalingfactor_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            regionid_array.push(row.regionid);
            rsoid_array.push(row.rsoid);
            regionalreferencepointid_array.push(row.regionalreferencepointid);
            peaktradingperiod_array
                .push({
                    row.peaktradingperiod
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            authorisedby_array.push(row.authorisedby);
            scalingfactor_array
                .push({
                    row.scalingfactor
                        .map(|mut val| {
                            val.rescale(5);
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
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(rsoid_array)) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(regionalreferencepointid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(peaktradingperiod_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(authorisedby_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(scalingfactor_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
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
/// ## TRANSMISSIONLOSSFACTOR
///  _TRANSMISSIONLOSSFACTOR shows the Transmission Loss factors applied at each connection point._
///
/// * Data Set Name: Market Config
/// * File Name: Transmissionlossfactor
/// * Data Version: 2
///
/// # Description
///  TRANSMISSIONLOSSFACTOR is public data, and is available to all participants. Source TRANSMISSIONLOSSFACTOR updates when new connection points are created or loss factors change.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONNECTIONPOINTID
/// * EFFECTIVEDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MarketConfigTransmissionlossfactor2 {
    /// Transmission Loss Factor
    pub transmissionlossfactor: rust_decimal::Decimal,
    /// Effective date of record
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of record for given effective date
    pub versionno: rust_decimal::Decimal,
    /// Connection Point ID
    pub connectionpointid: String,
    /// &nbsp;
    pub regionid: Option<String>,
    /// Record creation timestamp
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Secondary transmission loss factor applied in settlements for generator purchases.
    pub secondary_tlf: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for MarketConfigTransmissionlossfactor2 {
    type PrimaryKey = MarketConfigTransmissionlossfactor2PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "MARKET_CONFIG".into(),
            table_name: Some("TRANSMISSIONLOSSFACTOR".into()),
            version: 2,
        }
    }
    fn primary_key(&self) -> MarketConfigTransmissionlossfactor2PrimaryKey {
        MarketConfigTransmissionlossfactor2PrimaryKey {
            connectionpointid: self.connectionpointid.clone(),
            effectivedate: self.effectivedate,
            versionno: self.versionno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.effectivedate.year(),
            month: num_traits::FromPrimitive::from_u32(self.effectivedate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "market_config_transmissionlossfactor_v2_{}_{}", self.partition_suffix()
            .year, self.partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MarketConfigTransmissionlossfactor2PrimaryKey {
    pub connectionpointid: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for MarketConfigTransmissionlossfactor2PrimaryKey {}
impl mmsdm_core::CompareWithRow for MarketConfigTransmissionlossfactor2 {
    type Row = MarketConfigTransmissionlossfactor2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.connectionpointid == row.connectionpointid
            && self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MarketConfigTransmissionlossfactor2 {
    type PrimaryKey = MarketConfigTransmissionlossfactor2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid == key.connectionpointid
            && self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for MarketConfigTransmissionlossfactor2PrimaryKey {
    type Row = MarketConfigTransmissionlossfactor2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.connectionpointid == row.connectionpointid
            && self.effectivedate == row.effectivedate && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey
for MarketConfigTransmissionlossfactor2PrimaryKey {
    type PrimaryKey = MarketConfigTransmissionlossfactor2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid == key.connectionpointid
            && self.effectivedate == key.effectivedate && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MarketConfigTransmissionlossfactor2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("transmissionlossfactor",
                arrow2::datatypes::DataType::Decimal(15, 5), false),
                arrow2::datatypes::Field::new("effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("versionno",
                arrow2::datatypes::DataType::Decimal(22, 0), false),
                arrow2::datatypes::Field::new("connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("secondary_tlf",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut transmissionlossfactor_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut secondary_tlf_array = Vec::new();
        for row in partition {
            transmissionlossfactor_array
                .push({
                    let mut val = row.transmissionlossfactor;
                    val.rescale(5);
                    val.mantissa()
                });
            effectivedate_array.push(row.effectivedate.timestamp());
            versionno_array
                .push({
                    let mut val = row.versionno;
                    val.rescale(0);
                    val.mantissa()
                });
            connectionpointid_array.push(row.connectionpointid);
            regionid_array.push(row.regionid);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            secondary_tlf_array
                .push({
                    row.secondary_tlf
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(transmissionlossfactor_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array)
                    .to(arrow2::datatypes::DataType::Decimal(22, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(connectionpointid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(regionid_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(secondary_tlf_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
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
        (Some("BIDTYPES"), version) if version <= 1_i32 => {
            let d: Vec<MarketConfigBidtypes1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMarketConfigBidtypes1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("BIDTYPESTRK"), version) if version <= 1_i32 => {
            let d: Vec<MarketConfigBidtypestrk1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMarketConfigBidtypestrk1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("INTERCONNECTOR"), version) if version <= 1_i32 => {
            let d: Vec<MarketConfigInterconnector1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMarketConfigInterconnector1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("INTERCONNECTORALLOC"), version) if version <= 1_i32 => {
            let d: Vec<MarketConfigInterconnectoralloc1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMarketConfigInterconnectoralloc1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("INTERCONNECTORCONSTRAINT"), version) if version <= 1_i32 => {
            let d: Vec<MarketConfigInterconnectorconstraint1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMarketConfigInterconnectorconstraint1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("INTRAREGIONALLOC"), version) if version <= 1_i32 => {
            let d: Vec<MarketConfigIntraregionalloc1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMarketConfigIntraregionalloc1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("LOSSFACTORMODEL"), version) if version <= 1_i32 => {
            let d: Vec<MarketConfigLossfactormodel1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMarketConfigLossfactormodel1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("LOSSMODEL"), version) if version <= 1_i32 => {
            let d: Vec<MarketConfigLossmodel1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMarketConfigLossmodel1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("MARKET_PRICE_THRESHOLDS"), version) if version <= 1_i32 => {
            let d: Vec<MarketConfigMarketPriceThresholds1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMarketConfigMarketPriceThresholds1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("REGION"), version) if version <= 1_i32 => {
            let d: Vec<MarketConfigRegion1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMarketConfigRegion1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("REGIONSTANDINGDATA"), version) if version <= 1_i32 => {
            let d: Vec<MarketConfigRegionstandingdata1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMarketConfigRegionstandingdata1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("TRANSMISSIONLOSSFACTOR"), version) if version <= 2_i32 => {
            let d: Vec<MarketConfigTransmissionlossfactor2> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMarketConfigTransmissionlossfactor2 @P1, @P2",
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
