/// # Summary
///
/// ## AUCTION
///  _AUCTION holds auction details. AUCTION is new in March 2003 to support SRA Inter-Temporal Linking._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction
/// * Data Version: 1
///
/// # Description
///  AUCTION is public data, and is available to all participants. Source Static. Volume 4 records per year
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * AUCTIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuction1 {
    /// Unique id for each auction date
    pub auctionid: String,
    /// Auction date
    #[serde(with = "crate::mms_datetime_opt")]
    pub auctiondate: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    #[serde(with = "crate::mms_datetime_opt")]
    pub notifydate: Option<chrono::NaiveDateTime>,
    /// Open date for bidding
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Close date for bidding
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Description of an auction
    pub description: Option<String>,
    /// &nbsp;
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub authorisedby: Option<String>,
    /// &nbsp;
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionConfigAuction1 {
    type PrimaryKey = IrauctionConfigAuction1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_CONFIG".into(),
            table_name: Some("AUCTION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionConfigAuction1PrimaryKey {
        IrauctionConfigAuction1PrimaryKey {
            auctionid: self.auctionid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_config_auction_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionConfigAuction1 {
    type Row = IrauctionConfigAuction1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.auctionid == row.auctionid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionConfigAuction1 {
    type PrimaryKey = IrauctionConfigAuction1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionConfigAuction1PrimaryKey {
    pub auctionid: String,
}
impl crate::CompareWithRow for IrauctionConfigAuction1PrimaryKey {
    type Row = IrauctionConfigAuction1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.auctionid == row.auctionid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionConfigAuction1PrimaryKey {
    type PrimaryKey = IrauctionConfigAuction1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid
    }
}
impl crate::PrimaryKey for IrauctionConfigAuction1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionConfigAuction1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "auctionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("auctiondate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("notifydate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
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
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut auctionid_array = Vec::new();
        let mut auctiondate_array = Vec::new();
        let mut notifydate_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut description_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            auctionid_array.push(row.auctionid);
            auctiondate_array.push(row.auctiondate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            notifydate_array.push(row.notifydate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            startdate_array.push(row.startdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            enddate_array.push(row.enddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            description_array.push(row.description);
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
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(auctionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(auctiondate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(notifydate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)),
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
/// ## AUCTION_CALENDAR
///  _AUCTION_CALENDAR holds the definitions of each auction quarter in a contract year. AUCTION_CALENDAR supports the Settlement Residue Auction._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Calendar
/// * Data Version: 2
///
/// # Description
///  AUCTION_CALENDAR is public data, and is available to all participants. Source Updates are usually quarterly by the SRA team. Volume AUCTION_CALENDAR shows a maximum of 16 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * QUARTER
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionCalendar2 {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// First day of SRA Contract Quarter expressed as Date
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Last day of SRA Contract Quarter expressed as Date
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Default notification date
    #[serde(with = "crate::mms_datetime_opt")]
    pub notifydate: Option<chrono::NaiveDateTime>,
    /// Date for payment by Participant
    #[serde(with = "crate::mms_datetime_opt")]
    pub paymentdate: Option<chrono::NaiveDateTime>,
    /// Date of reconciliation for the quarter
    #[serde(with = "crate::mms_datetime_opt")]
    pub reconciliationdate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The date the Prelim Purchase Statement is generated
    #[serde(with = "crate::mms_datetime_opt")]
    pub prelimpurchasestmtdate: Option<chrono::NaiveDateTime>,
    /// The date the Prelim Proceeds Statement is generated
    #[serde(with = "crate::mms_datetime_opt")]
    pub prelimproceedsstmtdate: Option<chrono::NaiveDateTime>,
    /// The date the Final Purchase Statement is generated
    #[serde(with = "crate::mms_datetime_opt")]
    pub finalpurchasestmtdate: Option<chrono::NaiveDateTime>,
    /// The date the Final Proceeds Statement is generated
    #[serde(with = "crate::mms_datetime_opt")]
    pub finalproceedsstmtdate: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionConfigAuctionCalendar2 {
    type PrimaryKey = IrauctionConfigAuctionCalendar2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_CONFIG".into(),
            table_name: Some("AUCTION_CALENDAR".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> IrauctionConfigAuctionCalendar2PrimaryKey {
        IrauctionConfigAuctionCalendar2PrimaryKey {
            contractyear: self.contractyear,
            quarter: self.quarter,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_config_auction_calendar_v2".to_string()
    }
}
impl crate::CompareWithRow for IrauctionConfigAuctionCalendar2 {
    type Row = IrauctionConfigAuctionCalendar2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear && self.quarter == row.quarter
    }
}
impl crate::CompareWithPrimaryKey for IrauctionConfigAuctionCalendar2 {
    type PrimaryKey = IrauctionConfigAuctionCalendar2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.quarter == key.quarter
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionConfigAuctionCalendar2PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub quarter: rust_decimal::Decimal,
}
impl crate::CompareWithRow for IrauctionConfigAuctionCalendar2PrimaryKey {
    type Row = IrauctionConfigAuctionCalendar2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear && self.quarter == row.quarter
    }
}
impl crate::CompareWithPrimaryKey for IrauctionConfigAuctionCalendar2PrimaryKey {
    type PrimaryKey = IrauctionConfigAuctionCalendar2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.quarter == key.quarter
    }
}
impl crate::PrimaryKey for IrauctionConfigAuctionCalendar2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionConfigAuctionCalendar2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "quarter",
                arrow2::datatypes::DataType::Decimal(1, 0),
                false,
            ),
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("notifydate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("paymentdate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "reconciliationdate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "prelimpurchasestmtdate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "prelimproceedsstmtdate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "finalpurchasestmtdate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "finalproceedsstmtdate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractyear_array = Vec::new();
        let mut quarter_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut notifydate_array = Vec::new();
        let mut paymentdate_array = Vec::new();
        let mut reconciliationdate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut prelimpurchasestmtdate_array = Vec::new();
        let mut prelimproceedsstmtdate_array = Vec::new();
        let mut finalpurchasestmtdate_array = Vec::new();
        let mut finalproceedsstmtdate_array = Vec::new();
        for (_, row) in partition {
            contractyear_array.push({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
            quarter_array.push({
                let mut val = row.quarter;
                val.rescale(0);
                val.mantissa()
            });
            startdate_array.push(row.startdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            enddate_array.push(row.enddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            notifydate_array.push(row.notifydate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            paymentdate_array.push(row.paymentdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            reconciliationdate_array.push(row.reconciliationdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            prelimpurchasestmtdate_array.push(row.prelimpurchasestmtdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            prelimproceedsstmtdate_array.push(row.prelimproceedsstmtdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            finalpurchasestmtdate_array.push(row.finalpurchasestmtdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            finalproceedsstmtdate_array.push(row.finalproceedsstmtdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(contractyear_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(quarter_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(notifydate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(paymentdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(reconciliationdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(prelimpurchasestmtdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(prelimproceedsstmtdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(finalpurchasestmtdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(finalproceedsstmtdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## AUCTION_IC_ALLOCATIONS
///  _AUCTION_IC_ALLOCATIONS supports the Settlement Residue Auction by providing the basis for setting up contracts for individual tranches. AUCTION_IC_ALLOCATIONS shows the default definitions for the total number of units and proportion applicable to each directional interconnector for a specified auction quarter._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Ic Allocations
/// * Data Version: 2
///
/// # Description
///  AUCTION_IC_ALLOCATIONS is public data, and is available to all participants. Source Updates are usually quarterly as auctions are held from Settlement Residue Auction team's SRIS interface. Volume AUCTION_IC_ALLOCATIONS contains a maximum of 100 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * QUARTER
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionIcAllocations2 {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Version of data for other key data - a higher version for same key data takes precedence
    pub versionno: rust_decimal::Decimal,
    /// Contracted Interconnector Identifier
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Number of units on the interconnector
    pub maximumunits: Option<rust_decimal::Decimal>,
    /// Percentage of the total residue for each Unit
    pub proportion: Option<rust_decimal::Decimal>,
    /// Daily auction fee
    pub auctionfee: Option<rust_decimal::Decimal>,
    /// Authorisation date
    #[serde(with = "crate::mms_datetime_opt")]
    pub changedate: Option<chrono::NaiveDateTime>,
    /// Name of person authorising this data set
    pub changedby: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Fees for Cancelled Units.
    pub auctionfee_sales: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IrauctionConfigAuctionIcAllocations2 {
    type PrimaryKey = IrauctionConfigAuctionIcAllocations2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_CONFIG".into(),
            table_name: Some("AUCTION_IC_ALLOCATIONS".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> IrauctionConfigAuctionIcAllocations2PrimaryKey {
        IrauctionConfigAuctionIcAllocations2PrimaryKey {
            contractyear: self.contractyear,
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            quarter: self.quarter,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_config_auction_ic_allocations_v2".to_string()
    }
}
impl crate::CompareWithRow for IrauctionConfigAuctionIcAllocations2 {
    type Row = IrauctionConfigAuctionIcAllocations2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.quarter == row.quarter
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionConfigAuctionIcAllocations2 {
    type PrimaryKey = IrauctionConfigAuctionIcAllocations2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.quarter == key.quarter
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionConfigAuctionIcAllocations2PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub quarter: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for IrauctionConfigAuctionIcAllocations2PrimaryKey {
    type Row = IrauctionConfigAuctionIcAllocations2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.quarter == row.quarter
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionConfigAuctionIcAllocations2PrimaryKey {
    type PrimaryKey = IrauctionConfigAuctionIcAllocations2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.quarter == key.quarter
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for IrauctionConfigAuctionIcAllocations2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionConfigAuctionIcAllocations2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "quarter",
                arrow2::datatypes::DataType::Decimal(1, 0),
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
                false,
            ),
            arrow2::datatypes::Field::new(
                "fromregionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "maximumunits",
                arrow2::datatypes::DataType::Decimal(5, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "proportion",
                arrow2::datatypes::DataType::Decimal(8, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "auctionfee",
                arrow2::datatypes::DataType::Decimal(17, 5),
                true,
            ),
            arrow2::datatypes::Field::new("changedate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "changedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "auctionfee_sales",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractyear_array = Vec::new();
        let mut quarter_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut maximumunits_array = Vec::new();
        let mut proportion_array = Vec::new();
        let mut auctionfee_array = Vec::new();
        let mut changedate_array = Vec::new();
        let mut changedby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut auctionfee_sales_array = Vec::new();
        for (_, row) in partition {
            contractyear_array.push({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
            quarter_array.push({
                let mut val = row.quarter;
                val.rescale(0);
                val.mantissa()
            });
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            maximumunits_array.push({
                row.maximumunits.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            proportion_array.push({
                row.proportion.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            auctionfee_array.push({
                row.auctionfee.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            changedate_array.push(row.changedate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            changedby_array.push(row.changedby);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            auctionfee_sales_array.push({
                row.auctionfee_sales.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(contractyear_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(quarter_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    fromregionid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maximumunits_array)
                        .to(arrow2::datatypes::DataType::Decimal(5, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(proportion_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(auctionfee_array)
                        .to(arrow2::datatypes::DataType::Decimal(17, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(changedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(changedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(auctionfee_sales_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## AUCTION_REVENUE_ESTIMATE
///  _AUCTION_REVENUE_ESTIMATE supports the Settlement Residue Auction, by holding the evaluator’s estimates of revenue for each month of a given quarter.<br>Since reserve prices are no longer applicable from the end of 2001, zero is used as a default to avoid rewriting the system._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Revenue Estimate
/// * Data Version: 1
///
/// # Description
///  AUCTION_REVENUE_ESTIMATE is public data, and is available to all participants. Source Updates are quarterly from SRA team via SRIS interface Volume AUCTION_REVENUE_ESTIMATE contains a maximum of 300 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * MONTHNO
/// * QUARTER
/// * VALUATIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionRevenueEstimate1 {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Identifier of the estimator
    pub valuationid: String,
    /// Version of data for other key data - a higher version for same key data will take precedence
    pub versionno: rust_decimal::Decimal,
    /// Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Month number within quarter (1..3)
    pub monthno: rust_decimal::Decimal,
    /// First day of month as date
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Last day of month as date
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Estimated Revenue
    pub revenue: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionConfigAuctionRevenueEstimate1 {
    type PrimaryKey = IrauctionConfigAuctionRevenueEstimate1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_CONFIG".into(),
            table_name: Some("AUCTION_REVENUE_ESTIMATE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionConfigAuctionRevenueEstimate1PrimaryKey {
        IrauctionConfigAuctionRevenueEstimate1PrimaryKey {
            contractyear: self.contractyear,
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            monthno: self.monthno,
            quarter: self.quarter,
            valuationid: self.valuationid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_config_auction_revenue_estimate_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionConfigAuctionRevenueEstimate1 {
    type Row = IrauctionConfigAuctionRevenueEstimate1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.monthno == row.monthno
            && self.quarter == row.quarter
            && self.valuationid == row.valuationid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionConfigAuctionRevenueEstimate1 {
    type PrimaryKey = IrauctionConfigAuctionRevenueEstimate1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.monthno == key.monthno
            && self.quarter == key.quarter
            && self.valuationid == key.valuationid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionConfigAuctionRevenueEstimate1PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub monthno: rust_decimal::Decimal,
    pub quarter: rust_decimal::Decimal,
    pub valuationid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for IrauctionConfigAuctionRevenueEstimate1PrimaryKey {
    type Row = IrauctionConfigAuctionRevenueEstimate1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.monthno == row.monthno
            && self.quarter == row.quarter
            && self.valuationid == row.valuationid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionConfigAuctionRevenueEstimate1PrimaryKey {
    type PrimaryKey = IrauctionConfigAuctionRevenueEstimate1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.monthno == key.monthno
            && self.quarter == key.quarter
            && self.valuationid == key.valuationid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for IrauctionConfigAuctionRevenueEstimate1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionConfigAuctionRevenueEstimate1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "quarter",
                arrow2::datatypes::DataType::Decimal(1, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "valuationid",
                arrow2::datatypes::DataType::LargeUtf8,
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
                false,
            ),
            arrow2::datatypes::Field::new(
                "fromregionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "monthno",
                arrow2::datatypes::DataType::Decimal(1, 0),
                false,
            ),
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "revenue",
                arrow2::datatypes::DataType::Decimal(17, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractyear_array = Vec::new();
        let mut quarter_array = Vec::new();
        let mut valuationid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut monthno_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut revenue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            contractyear_array.push({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
            quarter_array.push({
                let mut val = row.quarter;
                val.rescale(0);
                val.mantissa()
            });
            valuationid_array.push(row.valuationid);
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            monthno_array.push({
                let mut val = row.monthno;
                val.rescale(0);
                val.mantissa()
            });
            startdate_array.push(row.startdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            enddate_array.push(row.enddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            revenue_array.push({
                row.revenue.map(|mut val| {
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
                    arrow2::array::PrimitiveArray::from_slice(contractyear_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(quarter_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    valuationid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    fromregionid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(monthno_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(revenue_array)
                        .to(arrow2::datatypes::DataType::Decimal(17, 5)),
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
/// ## AUCTION_REVENUE_TRACK
///  _AUCTION_REVENUE_TRACK supports the Settlement Residue Auction, by holding the tracking information for each evaluator’s estimates for a given quarter. The status field is dynamic and is used for selection of estimates to be published._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Revenue Track
/// * Data Version: 1
///
/// # Description
///  AUCTION_REVENUE_TRACK is public data, and is available to all participants. Source Updates are quarterly after SRA team updates SRIS interface. Volume AUCTION_REVENUE_TRACK contains a maximum of 100 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * QUARTER
/// * VALUATIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionRevenueTrack1 {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Identifier of the estimator
    pub valuationid: String,
    /// Version of data for other key data - a higher version for same key data takes precedence
    pub versionno: rust_decimal::Decimal,
    /// Date from which the record change is applicable
    #[serde(with = "crate::mms_datetime_opt")]
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// Internal use
    pub status: Option<String>,
    /// Reference to methodology document
    pub documentref: Option<String>,
    /// Date of authorisation for this record
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Name of person authorising this record
    pub authorisedby: Option<String>,
    /// Date and time this record was last changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionConfigAuctionRevenueTrack1 {
    type PrimaryKey = IrauctionConfigAuctionRevenueTrack1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_CONFIG".into(),
            table_name: Some("AUCTION_REVENUE_TRACK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionConfigAuctionRevenueTrack1PrimaryKey {
        IrauctionConfigAuctionRevenueTrack1PrimaryKey {
            contractyear: self.contractyear,
            quarter: self.quarter,
            valuationid: self.valuationid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_config_auction_revenue_track_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionConfigAuctionRevenueTrack1 {
    type Row = IrauctionConfigAuctionRevenueTrack1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear
            && self.quarter == row.quarter
            && self.valuationid == row.valuationid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionConfigAuctionRevenueTrack1 {
    type PrimaryKey = IrauctionConfigAuctionRevenueTrack1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear
            && self.quarter == key.quarter
            && self.valuationid == key.valuationid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionConfigAuctionRevenueTrack1PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub quarter: rust_decimal::Decimal,
    pub valuationid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for IrauctionConfigAuctionRevenueTrack1PrimaryKey {
    type Row = IrauctionConfigAuctionRevenueTrack1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear
            && self.quarter == row.quarter
            && self.valuationid == row.valuationid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionConfigAuctionRevenueTrack1PrimaryKey {
    type PrimaryKey = IrauctionConfigAuctionRevenueTrack1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear
            && self.quarter == key.quarter
            && self.valuationid == key.valuationid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for IrauctionConfigAuctionRevenueTrack1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionConfigAuctionRevenueTrack1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "quarter",
                arrow2::datatypes::DataType::Decimal(1, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "valuationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new("status", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "documentref",
                arrow2::datatypes::DataType::LargeUtf8,
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
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractyear_array = Vec::new();
        let mut quarter_array = Vec::new();
        let mut valuationid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut status_array = Vec::new();
        let mut documentref_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            contractyear_array.push({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
            quarter_array.push({
                let mut val = row.quarter;
                val.rescale(0);
                val.mantissa()
            });
            valuationid_array.push(row.valuationid);
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            effectivedate_array.push(row.effectivedate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            status_array.push(row.status);
            documentref_array.push(row.documentref);
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
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(contractyear_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(quarter_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    valuationid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(status_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(documentref_array)),
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
/// ## AUCTION_RP_ESTIMATE
///  _AUCTION_RP_ESTIMATE supports the Settlement Residue Auction, by holding the evaluator’s estimates of revenue prices for a given quarter.<br>Since reserve prices are no longer applicable from the end of 2001, zero is used as a default to avoid rewriting the system._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Rp Estimate
/// * Data Version: 1
///
/// # Description
///  AUCTION_RP_ESTIMATE is public data, and is available to all participants. Source Updates are quarterly by SRA team via SRIS interface. Volume This view contains a maximum of 100 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * QUARTER
/// * VALUATIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionRpEstimate1 {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Identifier of the estimator
    pub valuationid: String,
    /// Version of data for other key data - a higher version for same key data takes precedence
    pub versionno: rust_decimal::Decimal,
    /// Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Estimate of reserve price
    pub rpestimate: Option<rust_decimal::Decimal>,
    /// Last date and time record was changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionConfigAuctionRpEstimate1 {
    type PrimaryKey = IrauctionConfigAuctionRpEstimate1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_CONFIG".into(),
            table_name: Some("AUCTION_RP_ESTIMATE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionConfigAuctionRpEstimate1PrimaryKey {
        IrauctionConfigAuctionRpEstimate1PrimaryKey {
            contractyear: self.contractyear,
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            quarter: self.quarter,
            valuationid: self.valuationid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_config_auction_rp_estimate_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionConfigAuctionRpEstimate1 {
    type Row = IrauctionConfigAuctionRpEstimate1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.quarter == row.quarter
            && self.valuationid == row.valuationid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionConfigAuctionRpEstimate1 {
    type PrimaryKey = IrauctionConfigAuctionRpEstimate1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.quarter == key.quarter
            && self.valuationid == key.valuationid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionConfigAuctionRpEstimate1PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub quarter: rust_decimal::Decimal,
    pub valuationid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for IrauctionConfigAuctionRpEstimate1PrimaryKey {
    type Row = IrauctionConfigAuctionRpEstimate1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.quarter == row.quarter
            && self.valuationid == row.valuationid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionConfigAuctionRpEstimate1PrimaryKey {
    type PrimaryKey = IrauctionConfigAuctionRpEstimate1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.quarter == key.quarter
            && self.valuationid == key.valuationid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for IrauctionConfigAuctionRpEstimate1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionConfigAuctionRpEstimate1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "quarter",
                arrow2::datatypes::DataType::Decimal(1, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "valuationid",
                arrow2::datatypes::DataType::LargeUtf8,
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
                false,
            ),
            arrow2::datatypes::Field::new(
                "fromregionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "rpestimate",
                arrow2::datatypes::DataType::Decimal(17, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractyear_array = Vec::new();
        let mut quarter_array = Vec::new();
        let mut valuationid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut rpestimate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            contractyear_array.push({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
            quarter_array.push({
                let mut val = row.quarter;
                val.rescale(0);
                val.mantissa()
            });
            valuationid_array.push(row.valuationid);
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            rpestimate_array.push({
                row.rpestimate.map(|mut val| {
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
                    arrow2::array::PrimitiveArray::from_slice(contractyear_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(quarter_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    valuationid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    fromregionid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rpestimate_array)
                        .to(arrow2::datatypes::DataType::Decimal(17, 5)),
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
/// ## AUCTION_TRANCHE
///  _AUCTION_TRANCHE supports the Settlement Residue Auction, by holding the default definitions for the percentage number of units allocated and dates applicable to each tranche for a specified auction quarter. This information provides the basis for setting up contracts for individual tranches._
///
/// * Data Set Name: Irauction Config
/// * File Name: Auction Tranche
/// * Data Version: 1
///
/// # Description
///  AUCTION_TRANCHE is public data, and is available to all participants. Source Updates are quarterly from SRA team via SRIS interface. Volume AUCTION_TRANCHE contains a maximum of 100 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * QUARTER
/// * TRANCHE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionConfigAuctionTranche1 {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Version of data for other key data - a higher version for same key data will take precedence
    pub versionno: rust_decimal::Decimal,
    /// Label identifying the arbitrary segmented share of the Interconnector flow
    pub tranche: rust_decimal::Decimal,
    /// Default date of the auction
    #[serde(with = "crate::mms_datetime_opt")]
    pub auctiondate: Option<chrono::NaiveDateTime>,
    /// Default date participants notified of details
    #[serde(with = "crate::mms_datetime_opt")]
    pub notifydate: Option<chrono::NaiveDateTime>,
    /// Percentage of units allocated to the tranche
    pub unitallocation: Option<rust_decimal::Decimal>,
    /// Date of changing this record
    #[serde(with = "crate::mms_datetime_opt")]
    pub changedate: Option<chrono::NaiveDateTime>,
    /// Name of person who changed this record
    pub changedby: Option<String>,
    /// Date and time record was last changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionConfigAuctionTranche1 {
    type PrimaryKey = IrauctionConfigAuctionTranche1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_CONFIG".into(),
            table_name: Some("AUCTION_TRANCHE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionConfigAuctionTranche1PrimaryKey {
        IrauctionConfigAuctionTranche1PrimaryKey {
            contractyear: self.contractyear,
            quarter: self.quarter,
            tranche: self.tranche,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_config_auction_tranche_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionConfigAuctionTranche1 {
    type Row = IrauctionConfigAuctionTranche1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear
            && self.quarter == row.quarter
            && self.tranche == row.tranche
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionConfigAuctionTranche1 {
    type PrimaryKey = IrauctionConfigAuctionTranche1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear
            && self.quarter == key.quarter
            && self.tranche == key.tranche
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionConfigAuctionTranche1PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub quarter: rust_decimal::Decimal,
    pub tranche: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for IrauctionConfigAuctionTranche1PrimaryKey {
    type Row = IrauctionConfigAuctionTranche1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear
            && self.quarter == row.quarter
            && self.tranche == row.tranche
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionConfigAuctionTranche1PrimaryKey {
    type PrimaryKey = IrauctionConfigAuctionTranche1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear
            && self.quarter == key.quarter
            && self.tranche == key.tranche
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for IrauctionConfigAuctionTranche1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionConfigAuctionTranche1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "quarter",
                arrow2::datatypes::DataType::Decimal(1, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "tranche",
                arrow2::datatypes::DataType::Decimal(2, 0),
                false,
            ),
            arrow2::datatypes::Field::new("auctiondate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("notifydate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "unitallocation",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new("changedate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "changedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractyear_array = Vec::new();
        let mut quarter_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut tranche_array = Vec::new();
        let mut auctiondate_array = Vec::new();
        let mut notifydate_array = Vec::new();
        let mut unitallocation_array = Vec::new();
        let mut changedate_array = Vec::new();
        let mut changedby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            contractyear_array.push({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
            quarter_array.push({
                let mut val = row.quarter;
                val.rescale(0);
                val.mantissa()
            });
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            tranche_array.push({
                let mut val = row.tranche;
                val.rescale(0);
                val.mantissa()
            });
            auctiondate_array.push(row.auctiondate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            notifydate_array.push(row.notifydate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            unitallocation_array.push({
                row.unitallocation.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            changedate_array.push(row.changedate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            changedby_array.push(row.changedby);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(contractyear_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(quarter_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(tranche_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(auctiondate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(notifydate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(unitallocation_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(changedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(changedby_array)),
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
/// ## RESIDUECONTRACTPAYMENTS
///  _RESIDUECONTRACTPAYMENTS shows Settlement Residue Auction payment Participant notifications._
///
/// * Data Set Name: Settlement Config
/// * File Name: Residuecontractpayments
/// * Data Version: 1
///
/// # Description
///  RESIDUECONTRACTPAYMENTS data is confidential to the relevant participant.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigResiduecontractpayments1 {
    /// SRA Contract ID
    pub contractid: String,
    /// Participant Identifier
    pub participantid: String,
    /// Date and time this record was last changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigResiduecontractpayments1 {
    type PrimaryKey = SettlementConfigResiduecontractpayments1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("RESIDUECONTRACTPAYMENTS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementConfigResiduecontractpayments1PrimaryKey {
        SettlementConfigResiduecontractpayments1PrimaryKey {
            contractid: self.contractid.clone(),
            participantid: self.participantid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "settlement_config_residuecontractpayments_v1".to_string()
    }
}
impl crate::CompareWithRow for SettlementConfigResiduecontractpayments1 {
    type Row = SettlementConfigResiduecontractpayments1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigResiduecontractpayments1 {
    type PrimaryKey = SettlementConfigResiduecontractpayments1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.participantid == key.participantid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SettlementConfigResiduecontractpayments1PrimaryKey {
    pub contractid: String,
    pub participantid: String,
}
impl crate::CompareWithRow for SettlementConfigResiduecontractpayments1PrimaryKey {
    type Row = SettlementConfigResiduecontractpayments1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigResiduecontractpayments1PrimaryKey {
    type PrimaryKey = SettlementConfigResiduecontractpayments1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.participantid == key.participantid
    }
}
impl crate::PrimaryKey for SettlementConfigResiduecontractpayments1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SettlementConfigResiduecontractpayments1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractid",
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
        let mut contractid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            contractid_array.push(row.contractid);
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
                    contractid_array,
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
/// ## RESIDUEFILETRK
///  _RESIDUEFILETRK records all Settlement Residue Auction offers submitted by participants._
///
/// * Data Set Name: Irauction Bids
/// * File Name: File Trk
/// * Data Version: 1
///
/// # Description
///  RESIDUEFILETRK data is confidential to each participant Source RESIDUEFILETRK updates are ad hoc from participants Volume Assuming quarterly contracts RESIDUEFILETRK contains a maximum of 5,000 records per annum. Each bid file can contain many bids for each auction. Participants can input multiple bids (with the last acknowledged file being used in the auction).
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * LOADDATE
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionBidsFileTrk1 {
    /// SRA ContractID
    pub contractid: Option<String>,
    /// Participant Identifier
    pub participantid: String,
    /// Date-Time SRA offer was loaded
    #[serde(with = "crate::mms_datetime")]
    pub loaddate: chrono::NaiveDateTime,
    /// SRA offer file name
    pub filename: Option<String>,
    /// SRA acknowledgment file name
    pub ackfilename: Option<String>,
    /// Load status [SUCCESSFUL/CORRUPT]
    pub status: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Unique id for each auction date. (new in March 2003 to support SRA Inter-Temporal Linking)
    pub auctionid: String,
}
impl crate::GetTable for IrauctionBidsFileTrk1 {
    type PrimaryKey = IrauctionBidsFileTrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_BIDS".into(),
            table_name: Some("FILE_TRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionBidsFileTrk1PrimaryKey {
        IrauctionBidsFileTrk1PrimaryKey {
            auctionid: self.auctionid.clone(),
            loaddate: self.loaddate,
            participantid: self.participantid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_bids_file_trk_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionBidsFileTrk1 {
    type Row = IrauctionBidsFileTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.auctionid == row.auctionid
            && self.loaddate == row.loaddate
            && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionBidsFileTrk1 {
    type PrimaryKey = IrauctionBidsFileTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid
            && self.loaddate == key.loaddate
            && self.participantid == key.participantid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionBidsFileTrk1PrimaryKey {
    pub auctionid: String,
    pub loaddate: chrono::NaiveDateTime,
    pub participantid: String,
}
impl crate::CompareWithRow for IrauctionBidsFileTrk1PrimaryKey {
    type Row = IrauctionBidsFileTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.auctionid == row.auctionid
            && self.loaddate == row.loaddate
            && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionBidsFileTrk1PrimaryKey {
    type PrimaryKey = IrauctionBidsFileTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid
            && self.loaddate == key.loaddate
            && self.participantid == key.participantid
    }
}
impl crate::PrimaryKey for IrauctionBidsFileTrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionBidsFileTrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("loaddate", arrow2::datatypes::DataType::Date32, false),
            arrow2::datatypes::Field::new("filename", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "ackfilename",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("status", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "auctionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut loaddate_array = Vec::new();
        let mut filename_array = Vec::new();
        let mut ackfilename_array = Vec::new();
        let mut status_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut auctionid_array = Vec::new();
        for (_, row) in partition {
            contractid_array.push(row.contractid);
            participantid_array.push(row.participantid);
            loaddate_array.push(
                i32::try_from(
                    (row.loaddate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            filename_array.push(row.filename);
            ackfilename_array.push(row.ackfilename);
            status_array.push(row.status);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            auctionid_array.push(row.auctionid);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(contractid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(loaddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(filename_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(ackfilename_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(status_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(auctionid_array)),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## RESIDUE_BID_TRK
///  _RESIDUE_BID_TRK supports the Settlement Residue Auction, by detailing which bid was used for which SRA Contract run._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Bid Trk
/// * Data Version: 1
///
/// # Description
///  Source RESIDUE_BID_TRK updates are usually quarterly from participants before an Auction. RESIDUE_BID_TRK data is confidential to the relevant participant. RESIDUE_BID_TRK excludes contracts and versions without a valid publication date (i.e invalid bids are ignored). Volume Assuming monthly contracts, RESIDUE_BID_TRK shows a maximum of 500 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueBidTrk1 {
    /// SRA Contract unique identifier
    pub contractid: Option<String>,
    /// Version of Bid used
    pub versionno: rust_decimal::Decimal,
    /// Identifier of participant
    pub participantid: String,
    /// Date and time bid used
    #[serde(with = "crate::mms_datetime_opt")]
    pub bidloaddate: Option<chrono::NaiveDateTime>,
    /// Date and time this record was last changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Unique id for each auction date. (new in March 2003 to support SRA Inter-Temporal Linking)
    pub auctionid: String,
}
impl crate::GetTable for IrauctionResidueBidTrk1 {
    type PrimaryKey = IrauctionResidueBidTrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("RESIDUE_BID_TRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionResidueBidTrk1PrimaryKey {
        IrauctionResidueBidTrk1PrimaryKey {
            auctionid: self.auctionid.clone(),
            participantid: self.participantid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_residue_bid_trk_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionResidueBidTrk1 {
    type Row = IrauctionResidueBidTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.auctionid == row.auctionid
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionResidueBidTrk1 {
    type PrimaryKey = IrauctionResidueBidTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResidueBidTrk1PrimaryKey {
    pub auctionid: String,
    pub participantid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for IrauctionResidueBidTrk1PrimaryKey {
    type Row = IrauctionResidueBidTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.auctionid == row.auctionid
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionResidueBidTrk1PrimaryKey {
    type PrimaryKey = IrauctionResidueBidTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for IrauctionResidueBidTrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionResidueBidTrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
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
            arrow2::datatypes::Field::new("bidloaddate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "auctionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut bidloaddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut auctionid_array = Vec::new();
        for (_, row) in partition {
            contractid_array.push(row.contractid);
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            bidloaddate_array.push(row.bidloaddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            auctionid_array.push(row.auctionid);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(contractid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bidloaddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(auctionid_array)),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## RESIDUE_CONTRACTS
///  _RESIDUE_CONTRACTS supports the Settlement Residue Auction, by holding the contract details for each period for which a residue contract will be offered._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Contracts
/// * Data Version: 1
///
/// # Description
///  RESIDUE_CONTRACTS data is public, so is available to all participants. Source RESIDUE_CONTRACTS updates are quarterly by AEMO. Volume Assuming quarterly contracts, RESIDUE_CONTRACTS contains a maximum of 50 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * QUARTER
/// * TRANCHE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueContracts1 {
    /// SRA Contracted Year
    pub contractyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// Label identifying the arbitrary segmented share of the Interconnector flow
    pub tranche: rust_decimal::Decimal,
    /// Unique identifier for each SRA Contract as specified by AEMO
    pub contractid: Option<String>,
    /// SRA Quarter start date
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// SRA Quarter end date
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Open date of bidding, calculated as RNOTIFYDATE business days before the auction date
    #[serde(with = "crate::mms_datetime_opt")]
    pub notifydate: Option<chrono::NaiveDateTime>,
    /// Close date of bidding, calculated as RAUCDATE business days before the contract start date
    #[serde(with = "crate::mms_datetime_opt")]
    pub auctiondate: Option<chrono::NaiveDateTime>,
    /// Identifies methodology used
    pub calcmethod: Option<String>,
    /// Authorisation date for this record
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Name of authorising officer or process
    pub authorisedby: Option<String>,
    /// Date notification posted
    #[serde(with = "crate::mms_datetime_opt")]
    pub notifypostdate: Option<chrono::NaiveDateTime>,
    /// Name of notifying person
    pub notifyby: Option<String>,
    /// Date of publishing the auction results
    #[serde(with = "crate::mms_datetime_opt")]
    pub postdate: Option<chrono::NaiveDateTime>,
    /// Name of publishing officer or process
    pub postedby: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Description of Contract
    pub description: Option<String>,
    /// Unique id for each auction date (new in March 2003 to support SRA Inter-Temporal Linking)
    pub auctionid: Option<String>,
}
impl crate::GetTable for IrauctionResidueContracts1 {
    type PrimaryKey = IrauctionResidueContracts1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("RESIDUE_CONTRACTS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionResidueContracts1PrimaryKey {
        IrauctionResidueContracts1PrimaryKey {
            contractyear: self.contractyear,
            quarter: self.quarter,
            tranche: self.tranche,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_residue_contracts_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionResidueContracts1 {
    type Row = IrauctionResidueContracts1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear
            && self.quarter == row.quarter
            && self.tranche == row.tranche
    }
}
impl crate::CompareWithPrimaryKey for IrauctionResidueContracts1 {
    type PrimaryKey = IrauctionResidueContracts1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear
            && self.quarter == key.quarter
            && self.tranche == key.tranche
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResidueContracts1PrimaryKey {
    pub contractyear: rust_decimal::Decimal,
    pub quarter: rust_decimal::Decimal,
    pub tranche: rust_decimal::Decimal,
}
impl crate::CompareWithRow for IrauctionResidueContracts1PrimaryKey {
    type Row = IrauctionResidueContracts1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear
            && self.quarter == row.quarter
            && self.tranche == row.tranche
    }
}
impl crate::CompareWithPrimaryKey for IrauctionResidueContracts1PrimaryKey {
    type PrimaryKey = IrauctionResidueContracts1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear
            && self.quarter == key.quarter
            && self.tranche == key.tranche
    }
}
impl crate::PrimaryKey for IrauctionResidueContracts1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionResidueContracts1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "quarter",
                arrow2::datatypes::DataType::Decimal(1, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "tranche",
                arrow2::datatypes::DataType::Decimal(2, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("notifydate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("auctiondate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "calcmethod",
                arrow2::datatypes::DataType::LargeUtf8,
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
            arrow2::datatypes::Field::new(
                "notifypostdate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new("notifyby", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("postdate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("postedby", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "auctionid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractyear_array = Vec::new();
        let mut quarter_array = Vec::new();
        let mut tranche_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut notifydate_array = Vec::new();
        let mut auctiondate_array = Vec::new();
        let mut calcmethod_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut notifypostdate_array = Vec::new();
        let mut notifyby_array = Vec::new();
        let mut postdate_array = Vec::new();
        let mut postedby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut description_array = Vec::new();
        let mut auctionid_array = Vec::new();
        for (_, row) in partition {
            contractyear_array.push({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
            quarter_array.push({
                let mut val = row.quarter;
                val.rescale(0);
                val.mantissa()
            });
            tranche_array.push({
                let mut val = row.tranche;
                val.rescale(0);
                val.mantissa()
            });
            contractid_array.push(row.contractid);
            startdate_array.push(row.startdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            enddate_array.push(row.enddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            notifydate_array.push(row.notifydate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            auctiondate_array.push(row.auctiondate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            calcmethod_array.push(row.calcmethod);
            authoriseddate_array.push(row.authoriseddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            authorisedby_array.push(row.authorisedby);
            notifypostdate_array.push(row.notifypostdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            notifyby_array.push(row.notifyby);
            postdate_array.push(row.postdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            postedby_array.push(row.postedby);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            description_array.push(row.description);
            auctionid_array.push(row.auctionid);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(contractyear_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(quarter_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(tranche_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(contractid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(notifydate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(auctiondate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(calcmethod_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(notifypostdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(notifyby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(postdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(postedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(auctionid_array)),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## RESIDUE_CON_DATA
///  _RESIDUE_CON_DATA supports the Settlement Residue Auction, by holding for each participant the confidential data from the auction. RESIDUE_CON_DATA joins to RESIDUE_PUBLIC_DATA and RESIDUE_TRK._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Con Data
/// * Data Version: 2
///
/// # Description
///  Source RESIDUE_CON_DATA refreshes whenever a Settlement Residue Auction is run (i.e. quarterly). RESIDUE_CON_DATA data is confidential to the relevant participant. RESIDUE_CON_DATA excludes contracts and versions without a valid publication date (i.e invalid bids are ignored). Volume RESIDUE_CON_DATA shows a maximum of 6000 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueConData2 {
    /// SRA Contract unique identifier
    pub contractid: String,
    /// Contract run version
    pub versionno: rust_decimal::Decimal,
    /// Identifier of Contracted Participant
    pub participantid: String,
    /// Identifier of Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Units purchased on the directional interconnector (i.e. Contracted quantity)
    pub unitspurchased: Option<rust_decimal::Decimal>,
    /// Payment due (i.e. total purchase price)
    pub linkpayment: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The number of cancelled Units for all Auction Participants.
    pub secondary_units_sold: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IrauctionResidueConData2 {
    type PrimaryKey = IrauctionResidueConData2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("RESIDUE_CON_DATA".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> IrauctionResidueConData2PrimaryKey {
        IrauctionResidueConData2PrimaryKey {
            contractid: self.contractid.clone(),
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            participantid: self.participantid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_residue_con_data_v2".to_string()
    }
}
impl crate::CompareWithRow for IrauctionResidueConData2 {
    type Row = IrauctionResidueConData2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionResidueConData2 {
    type PrimaryKey = IrauctionResidueConData2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResidueConData2PrimaryKey {
    pub contractid: String,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub participantid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for IrauctionResidueConData2PrimaryKey {
    type Row = IrauctionResidueConData2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionResidueConData2PrimaryKey {
    type PrimaryKey = IrauctionResidueConData2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for IrauctionResidueConData2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionResidueConData2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
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
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "fromregionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "unitspurchased",
                arrow2::datatypes::DataType::Decimal(17, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "linkpayment",
                arrow2::datatypes::DataType::Decimal(17, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "secondary_units_sold",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut unitspurchased_array = Vec::new();
        let mut linkpayment_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut secondary_units_sold_array = Vec::new();
        for (_, row) in partition {
            contractid_array.push(row.contractid);
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            unitspurchased_array.push({
                row.unitspurchased.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            linkpayment_array.push({
                row.linkpayment.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            secondary_units_sold_array.push({
                row.secondary_units_sold.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    contractid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    fromregionid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(unitspurchased_array)
                        .to(arrow2::datatypes::DataType::Decimal(17, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(linkpayment_array)
                        .to(arrow2::datatypes::DataType::Decimal(17, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(secondary_units_sold_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## RESIDUE_CON_ESTIMATES_TRK
///  _RESIDUE_CON_ESTIMATES_TRK supports the Settlement Residue Auction, by holding the tracking details of the estimates used to generate the reserve price for each contract._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Con Estimates Trk
/// * Data Version: 1
///
/// # Description
///  Source RESIDUE_CON_ESTIMATES_TRK updates are quarterly by SRA team. Volume Assuming monthly contracts, RESIDUE_CON_ESTIMATES_TRK shows a maximum of 50 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * CONTRACTYEAR
/// * QUARTER
/// * VALUATIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueConEstimatesTrk1 {
    /// SRA Contract unique identifier
    pub contractid: String,
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Contract Quarter
    pub quarter: rust_decimal::Decimal,
    /// Identifier of the estimator
    pub valuationid: String,
    /// Version of a record, as nominated by the participant
    pub versionno: Option<rust_decimal::Decimal>,
    /// Date and time this record was changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionResidueConEstimatesTrk1 {
    type PrimaryKey = IrauctionResidueConEstimatesTrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("RESIDUE_CON_ESTIMATES_TRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionResidueConEstimatesTrk1PrimaryKey {
        IrauctionResidueConEstimatesTrk1PrimaryKey {
            contractid: self.contractid.clone(),
            contractyear: self.contractyear,
            quarter: self.quarter,
            valuationid: self.valuationid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_residue_con_estimates_trk_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionResidueConEstimatesTrk1 {
    type Row = IrauctionResidueConEstimatesTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.contractyear == row.contractyear
            && self.quarter == row.quarter
            && self.valuationid == row.valuationid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionResidueConEstimatesTrk1 {
    type PrimaryKey = IrauctionResidueConEstimatesTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.contractyear == key.contractyear
            && self.quarter == key.quarter
            && self.valuationid == key.valuationid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResidueConEstimatesTrk1PrimaryKey {
    pub contractid: String,
    pub contractyear: rust_decimal::Decimal,
    pub quarter: rust_decimal::Decimal,
    pub valuationid: String,
}
impl crate::CompareWithRow for IrauctionResidueConEstimatesTrk1PrimaryKey {
    type Row = IrauctionResidueConEstimatesTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.contractyear == row.contractyear
            && self.quarter == row.quarter
            && self.valuationid == row.valuationid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionResidueConEstimatesTrk1PrimaryKey {
    type PrimaryKey = IrauctionResidueConEstimatesTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.contractyear == key.contractyear
            && self.quarter == key.quarter
            && self.valuationid == key.valuationid
    }
}
impl crate::PrimaryKey for IrauctionResidueConEstimatesTrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionResidueConEstimatesTrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "quarter",
                arrow2::datatypes::DataType::Decimal(1, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "valuationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractid_array = Vec::new();
        let mut contractyear_array = Vec::new();
        let mut quarter_array = Vec::new();
        let mut valuationid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            contractid_array.push(row.contractid);
            contractyear_array.push({
                let mut val = row.contractyear;
                val.rescale(0);
                val.mantissa()
            });
            quarter_array.push({
                let mut val = row.quarter;
                val.rescale(0);
                val.mantissa()
            });
            valuationid_array.push(row.valuationid);
            versionno_array.push({
                row.versionno.map(|mut val| {
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
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    contractid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(contractyear_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(quarter_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    valuationid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(versionno_array)
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
/// ## RESIDUE_CON_FUNDS
///  _RESIDUE_CON_FUNDS supports the Settlement Residue Auction, by holding the fund details for each contract._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Con Funds
/// * Data Version: 1
///
/// # Description
///  RESIDUE_CON_FUNDS data is public, so is available to all participants. Source RESIDUE_CON_FUNDS updates are quarterly from SRA team via SRIS interface. Volume Assuming quarterly contracts, RESIDUE_CON_FUNDS contains a maximum of 600 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueConFunds1 {
    /// SRA Contract unique identifier as specified by AEMO
    pub contractid: String,
    /// Identifier for the Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Actual number of units allocated based on the auction default percentage for the tranche and the total number of units to be auctioned for this quarter
    pub defaultunits: Option<rust_decimal::Decimal>,
    /// Units reallocated from the previous tranche of this quarter
    pub rolloverunits: Option<rust_decimal::Decimal>,
    /// Units reallocated from the previous tranche of this quarter because they were not taken up by the participant
    pub reallocatedunits: Option<rust_decimal::Decimal>,
    /// Total units offered for Contract
    pub unitsoffered: Option<rust_decimal::Decimal>,
    /// Average reserve price calculated from the selected estimates
    pub meanreserveprice: Option<rust_decimal::Decimal>,
    /// Scaling factor for regional Frequency control Ancillary Service requirement
    pub scalefactor: Option<rust_decimal::Decimal>,
    /// Actual reserve price
    pub actualreserveprice: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionResidueConFunds1 {
    type PrimaryKey = IrauctionResidueConFunds1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("RESIDUE_CON_FUNDS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionResidueConFunds1PrimaryKey {
        IrauctionResidueConFunds1PrimaryKey {
            contractid: self.contractid.clone(),
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_residue_con_funds_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionResidueConFunds1 {
    type Row = IrauctionResidueConFunds1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionResidueConFunds1 {
    type PrimaryKey = IrauctionResidueConFunds1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResidueConFunds1PrimaryKey {
    pub contractid: String,
    pub fromregionid: String,
    pub interconnectorid: String,
}
impl crate::CompareWithRow for IrauctionResidueConFunds1PrimaryKey {
    type Row = IrauctionResidueConFunds1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionResidueConFunds1PrimaryKey {
    type PrimaryKey = IrauctionResidueConFunds1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
    }
}
impl crate::PrimaryKey for IrauctionResidueConFunds1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionResidueConFunds1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "fromregionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "defaultunits",
                arrow2::datatypes::DataType::Decimal(5, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rolloverunits",
                arrow2::datatypes::DataType::Decimal(5, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "reallocatedunits",
                arrow2::datatypes::DataType::Decimal(5, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "unitsoffered",
                arrow2::datatypes::DataType::Decimal(5, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "meanreserveprice",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "scalefactor",
                arrow2::datatypes::DataType::Decimal(8, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "actualreserveprice",
                arrow2::datatypes::DataType::Decimal(9, 2),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractid_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut defaultunits_array = Vec::new();
        let mut rolloverunits_array = Vec::new();
        let mut reallocatedunits_array = Vec::new();
        let mut unitsoffered_array = Vec::new();
        let mut meanreserveprice_array = Vec::new();
        let mut scalefactor_array = Vec::new();
        let mut actualreserveprice_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            contractid_array.push(row.contractid);
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            defaultunits_array.push({
                row.defaultunits.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            rolloverunits_array.push({
                row.rolloverunits.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            reallocatedunits_array.push({
                row.reallocatedunits.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            unitsoffered_array.push({
                row.unitsoffered.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            meanreserveprice_array.push({
                row.meanreserveprice.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            scalefactor_array.push({
                row.scalefactor.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            actualreserveprice_array.push({
                row.actualreserveprice.map(|mut val| {
                    val.rescale(2);
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
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    contractid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    fromregionid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(defaultunits_array)
                        .to(arrow2::datatypes::DataType::Decimal(5, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rolloverunits_array)
                        .to(arrow2::datatypes::DataType::Decimal(5, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(reallocatedunits_array)
                        .to(arrow2::datatypes::DataType::Decimal(5, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(unitsoffered_array)
                        .to(arrow2::datatypes::DataType::Decimal(5, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(meanreserveprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(scalefactor_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(actualreserveprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 2)),
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
/// ## RESIDUE_FUNDS_BID
///  _RESIDUE_FUNDS_BID supports the Settlement Residue Auction, by showing the fund details for each SRA bid by each Participant._
///
/// * Data Set Name: Irauction Bids
/// * File Name: Funds Bid
/// * Data Version: 1
///
/// # Description
///  Source Participant's bid file. RESIDUE_FUNDS_BID data is confidential to the relevant participant. RESIDUE_FUNDS_BID shows a maximum of 30,000 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * LOADDATE
/// * OPTIONID
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionBidsFundsBid1 {
    /// SRA Contract identifier
    pub contractid: String,
    /// Participant identifier
    pub participantid: String,
    /// Date and time the batcher loaded the SRA offer
    #[serde(with = "crate::mms_datetime")]
    pub loaddate: chrono::NaiveDateTime,
    /// Unique option identifier (1..20)
    pub optionid: rust_decimal::Decimal,
    /// Interconnector Identifier
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Quantity of units bid for
    pub units: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionBidsFundsBid1 {
    type PrimaryKey = IrauctionBidsFundsBid1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_BIDS".into(),
            table_name: Some("FUNDS_BID".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionBidsFundsBid1PrimaryKey {
        IrauctionBidsFundsBid1PrimaryKey {
            contractid: self.contractid.clone(),
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            loaddate: self.loaddate,
            optionid: self.optionid,
            participantid: self.participantid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_bids_funds_bid_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionBidsFundsBid1 {
    type Row = IrauctionBidsFundsBid1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.loaddate == row.loaddate
            && self.optionid == row.optionid
            && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionBidsFundsBid1 {
    type PrimaryKey = IrauctionBidsFundsBid1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.loaddate == key.loaddate
            && self.optionid == key.optionid
            && self.participantid == key.participantid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionBidsFundsBid1PrimaryKey {
    pub contractid: String,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub loaddate: chrono::NaiveDateTime,
    pub optionid: rust_decimal::Decimal,
    pub participantid: String,
}
impl crate::CompareWithRow for IrauctionBidsFundsBid1PrimaryKey {
    type Row = IrauctionBidsFundsBid1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.loaddate == row.loaddate
            && self.optionid == row.optionid
            && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionBidsFundsBid1PrimaryKey {
    type PrimaryKey = IrauctionBidsFundsBid1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.loaddate == key.loaddate
            && self.optionid == key.optionid
            && self.participantid == key.participantid
    }
}
impl crate::PrimaryKey for IrauctionBidsFundsBid1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionBidsFundsBid1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("loaddate", arrow2::datatypes::DataType::Date32, false),
            arrow2::datatypes::Field::new(
                "optionid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "fromregionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "units",
                arrow2::datatypes::DataType::Decimal(5, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut loaddate_array = Vec::new();
        let mut optionid_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut units_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            contractid_array.push(row.contractid);
            participantid_array.push(row.participantid);
            loaddate_array.push(
                i32::try_from(
                    (row.loaddate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            optionid_array.push({
                let mut val = row.optionid;
                val.rescale(0);
                val.mantissa()
            });
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            units_array.push({
                row.units.map(|mut val| {
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
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    contractid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(loaddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(optionid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    fromregionid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(units_array)
                        .to(arrow2::datatypes::DataType::Decimal(5, 0)),
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
/// ## RESIDUE_PRICE_BID
///  _RESIDUE_PRICE_BID supports the Settlement Residue Auction, holding the unit and bid price details for each participant._
///
/// * Data Set Name: Irauction Bids
/// * File Name: Price Bid
/// * Data Version: 1
///
/// # Description
///  Source The participant's own bid file RESIDUE_PRICE_BID data is confidential to the relevant participant. The public version of the data is available to all auction participants post the associated auction date in RESIDUE_PRICE_FUNDS_BID. Volume RESIDUE_PRICE_BID shows a maximum of 10,000 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * LOADDATE
/// * OPTIONID
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionBidsPriceBid1 {
    /// Not to be used. Unique id for each SRA contract (specified by AEMO)
    pub contractid: Option<String>,
    /// Participant identifier
    pub participantid: String,
    /// Date and Time the batcher loaded the bid
    #[serde(with = "crate::mms_datetime")]
    pub loaddate: chrono::NaiveDateTime,
    /// Unique option (bid) identifier (1..800)
    pub optionid: rust_decimal::Decimal,
    /// Price offered for each unit
    pub bidprice: Option<rust_decimal::Decimal>,
    /// Date and time this record was last changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Unique id for each auction date (new in March 2003 to support SRA Inter-Temporal Linking)
    pub auctionid: String,
}
impl crate::GetTable for IrauctionBidsPriceBid1 {
    type PrimaryKey = IrauctionBidsPriceBid1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION_BIDS".into(),
            table_name: Some("PRICE_BID".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionBidsPriceBid1PrimaryKey {
        IrauctionBidsPriceBid1PrimaryKey {
            auctionid: self.auctionid.clone(),
            loaddate: self.loaddate,
            optionid: self.optionid,
            participantid: self.participantid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_bids_price_bid_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionBidsPriceBid1 {
    type Row = IrauctionBidsPriceBid1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.auctionid == row.auctionid
            && self.loaddate == row.loaddate
            && self.optionid == row.optionid
            && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionBidsPriceBid1 {
    type PrimaryKey = IrauctionBidsPriceBid1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid
            && self.loaddate == key.loaddate
            && self.optionid == key.optionid
            && self.participantid == key.participantid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionBidsPriceBid1PrimaryKey {
    pub auctionid: String,
    pub loaddate: chrono::NaiveDateTime,
    pub optionid: rust_decimal::Decimal,
    pub participantid: String,
}
impl crate::CompareWithRow for IrauctionBidsPriceBid1PrimaryKey {
    type Row = IrauctionBidsPriceBid1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.auctionid == row.auctionid
            && self.loaddate == row.loaddate
            && self.optionid == row.optionid
            && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionBidsPriceBid1PrimaryKey {
    type PrimaryKey = IrauctionBidsPriceBid1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid
            && self.loaddate == key.loaddate
            && self.optionid == key.optionid
            && self.participantid == key.participantid
    }
}
impl crate::PrimaryKey for IrauctionBidsPriceBid1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionBidsPriceBid1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("loaddate", arrow2::datatypes::DataType::Date32, false),
            arrow2::datatypes::Field::new(
                "optionid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "bidprice",
                arrow2::datatypes::DataType::Decimal(17, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "auctionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut loaddate_array = Vec::new();
        let mut optionid_array = Vec::new();
        let mut bidprice_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut auctionid_array = Vec::new();
        for (_, row) in partition {
            contractid_array.push(row.contractid);
            participantid_array.push(row.participantid);
            loaddate_array.push(
                i32::try_from(
                    (row.loaddate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            optionid_array.push({
                let mut val = row.optionid;
                val.rescale(0);
                val.mantissa()
            });
            bidprice_array.push({
                row.bidprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            auctionid_array.push(row.auctionid);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(contractid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(loaddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(optionid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bidprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(17, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(auctionid_array)),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## RESIDUE_PRICE_FUNDS_BID
///  _RESIDUE_PRICE_FUNDS_BIDshows the bids producing the auction outcome, without exposing participant-specific details. RESIDUE_PRICE_FUNDS_BID is new in March 2003 to support SRA Inter-Temporal Linking._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Price Funds Bid
/// * Data Version: 1
///
/// # Description
///  RESIDUE_PRICE_FUNDS_BID data is public. The data is available to all auction participants post the associated auction date. Volume The volume is very dependent on the number of active bids. An indication is about 250,000 per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * LINKEDBIDFLAG
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResiduePriceFundsBid1 {
    /// Unique id for each contract specified by AEMO
    pub contractid: String,
    /// Unique interconnector identifier
    pub interconnectorid: String,
    /// Unique region identifier
    pub fromregionid: String,
    /// Quantity of units bid
    pub units: Option<rust_decimal::Decimal>,
    /// Price bid for each unit
    pub bidprice: Option<rust_decimal::Decimal>,
    /// A unique option id, with respect to the auction, created to show which bid elements are linked.
    pub linkedbidflag: rust_decimal::Decimal,
    /// Unique id for each auction date
    pub auctionid: String,
    /// Date and time this record was last changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionResiduePriceFundsBid1 {
    type PrimaryKey = IrauctionResiduePriceFundsBid1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("RESIDUE_PRICE_FUNDS_BID".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionResiduePriceFundsBid1PrimaryKey {
        IrauctionResiduePriceFundsBid1PrimaryKey {
            auctionid: self.auctionid.clone(),
            contractid: self.contractid.clone(),
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            linkedbidflag: self.linkedbidflag,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_residue_price_funds_bid_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionResiduePriceFundsBid1 {
    type Row = IrauctionResiduePriceFundsBid1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.auctionid == row.auctionid
            && self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.linkedbidflag == row.linkedbidflag
    }
}
impl crate::CompareWithPrimaryKey for IrauctionResiduePriceFundsBid1 {
    type PrimaryKey = IrauctionResiduePriceFundsBid1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid
            && self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.linkedbidflag == key.linkedbidflag
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResiduePriceFundsBid1PrimaryKey {
    pub auctionid: String,
    pub contractid: String,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub linkedbidflag: rust_decimal::Decimal,
}
impl crate::CompareWithRow for IrauctionResiduePriceFundsBid1PrimaryKey {
    type Row = IrauctionResiduePriceFundsBid1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.auctionid == row.auctionid
            && self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.linkedbidflag == row.linkedbidflag
    }
}
impl crate::CompareWithPrimaryKey for IrauctionResiduePriceFundsBid1PrimaryKey {
    type PrimaryKey = IrauctionResiduePriceFundsBid1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid
            && self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.linkedbidflag == key.linkedbidflag
    }
}
impl crate::PrimaryKey for IrauctionResiduePriceFundsBid1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionResiduePriceFundsBid1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "fromregionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "units",
                arrow2::datatypes::DataType::Decimal(5, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bidprice",
                arrow2::datatypes::DataType::Decimal(17, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "linkedbidflag",
                arrow2::datatypes::DataType::Decimal(6, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "auctionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractid_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut units_array = Vec::new();
        let mut bidprice_array = Vec::new();
        let mut linkedbidflag_array = Vec::new();
        let mut auctionid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            contractid_array.push(row.contractid);
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            units_array.push({
                row.units.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            bidprice_array.push({
                row.bidprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            linkedbidflag_array.push({
                let mut val = row.linkedbidflag;
                val.rescale(0);
                val.mantissa()
            });
            auctionid_array.push(row.auctionid);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    contractid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    fromregionid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(units_array)
                        .to(arrow2::datatypes::DataType::Decimal(5, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bidprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(17, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(linkedbidflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(auctionid_array)),
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
/// ## RESIDUE_PUBLIC_DATA
///  _RESIDUE_PUBLIC_DATA shows the public auction results.<br>RESIDUE_PUBLIC_DATA supports the Settlement Residue Auction, by holding the public details of the auction for a given contract. RESIDUE_PUBLIC_DATA joins to RESIDUE_CON_DATA and RESIDUE.<br>_
///
/// * Data Set Name: Irauction
/// * File Name: Residue Public Data
/// * Data Version: 1
///
/// # Description
///  RESIDUE_PUBLIC_DATA excludes contracts and versions without a valid publication date (i.e. invalid bids are ignored).  The data is available to all auction participants post the associated auction date. Source RESIDUE_PUBLIC_DATA updates are quarterly from NEMMCO. Volume RESIDUE_PUBLIC_DATA shows a maximum of 120 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResiduePublicData1 {
    /// Unique id for each contract to be specified by AEMO
    pub contractid: String,
    /// Version Number
    pub versionno: rust_decimal::Decimal,
    /// Unique interconnector identifier
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Total units offered for auction
    pub unitsoffered: Option<rust_decimal::Decimal>,
    /// Units Sold (modified format and usage in March 2003 to support SRA Inter-Temporal Linking)
    pub unitssold: Option<rust_decimal::Decimal>,
    /// Clearing price
    pub clearingprice: Option<rust_decimal::Decimal>,
    /// Reserve price
    pub reserveprice: Option<rust_decimal::Decimal>,
    /// Date and time this record was last changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionResiduePublicData1 {
    type PrimaryKey = IrauctionResiduePublicData1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("RESIDUE_PUBLIC_DATA".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionResiduePublicData1PrimaryKey {
        IrauctionResiduePublicData1PrimaryKey {
            contractid: self.contractid.clone(),
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_residue_public_data_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionResiduePublicData1 {
    type Row = IrauctionResiduePublicData1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionResiduePublicData1 {
    type PrimaryKey = IrauctionResiduePublicData1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResiduePublicData1PrimaryKey {
    pub contractid: String,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for IrauctionResiduePublicData1PrimaryKey {
    type Row = IrauctionResiduePublicData1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionResiduePublicData1PrimaryKey {
    type PrimaryKey = IrauctionResiduePublicData1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for IrauctionResiduePublicData1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionResiduePublicData1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
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
                false,
            ),
            arrow2::datatypes::Field::new(
                "fromregionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "unitsoffered",
                arrow2::datatypes::DataType::Decimal(5, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "unitssold",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "clearingprice",
                arrow2::datatypes::DataType::Decimal(17, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "reserveprice",
                arrow2::datatypes::DataType::Decimal(17, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut unitsoffered_array = Vec::new();
        let mut unitssold_array = Vec::new();
        let mut clearingprice_array = Vec::new();
        let mut reserveprice_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            contractid_array.push(row.contractid);
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            unitsoffered_array.push({
                row.unitsoffered.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            unitssold_array.push({
                row.unitssold.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            clearingprice_array.push({
                row.clearingprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            reserveprice_array.push({
                row.reserveprice.map(|mut val| {
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
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    contractid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    fromregionid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(unitsoffered_array)
                        .to(arrow2::datatypes::DataType::Decimal(5, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(unitssold_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(clearingprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(17, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(reserveprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(17, 5)),
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
/// ## RESIDUE_TRK
///  _RESIDUE_TRK supports the Settlement Residue Auction, by showing the tracking records for different residue auction runs. RESIDUE_TRK joins to RESIDUE_PUBLIC_DATA and RESIDUE_CON_DATA._
///
/// * Data Set Name: Irauction
/// * File Name: Residue Trk
/// * Data Version: 1
///
/// # Description
///  Source RESIDUE_TRK updates whenever Settlement Residue Auctions are run and the results published (i.e. quarterly). The RESIDUE_TRK data is available to all participants post the associated auction date. Volume Assuming quarterly contracts, RESIDUE_TRK shows a maximum of 50 records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionResidueTrk1 {
    /// SRA Contract identifier
    pub contractid: Option<String>,
    /// Contract run version
    pub versionno: rust_decimal::Decimal,
    /// Date auction results determined
    #[serde(with = "crate::mms_datetime_opt")]
    pub rundate: Option<chrono::NaiveDateTime>,
    /// Date results published
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorising officer or process
    pub authorisedby: Option<String>,
    /// Date the run is authorised
    #[serde(with = "crate::mms_datetime_opt")]
    pub postdate: Option<chrono::NaiveDateTime>,
    /// Name of authorising officer or process
    pub postedby: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Load status [SUCCESSFUL/CORRUPT]
    pub status: Option<String>,
    /// Unique id for each auction date. (new in March 2003 to support SRA Inter-Temporal Linking)
    pub auctionid: String,
}
impl crate::GetTable for IrauctionResidueTrk1 {
    type PrimaryKey = IrauctionResidueTrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("RESIDUE_TRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionResidueTrk1PrimaryKey {
        IrauctionResidueTrk1PrimaryKey {
            auctionid: self.auctionid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_residue_trk_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionResidueTrk1 {
    type Row = IrauctionResidueTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.auctionid == row.auctionid && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionResidueTrk1 {
    type PrimaryKey = IrauctionResidueTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionResidueTrk1PrimaryKey {
    pub auctionid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for IrauctionResidueTrk1PrimaryKey {
    type Row = IrauctionResidueTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.auctionid == row.auctionid && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionResidueTrk1PrimaryKey {
    type PrimaryKey = IrauctionResidueTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for IrauctionResidueTrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionResidueTrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("rundate", arrow2::datatypes::DataType::Date32, true),
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
            arrow2::datatypes::Field::new("postdate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("postedby", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("status", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "auctionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut rundate_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut postdate_array = Vec::new();
        let mut postedby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut status_array = Vec::new();
        let mut auctionid_array = Vec::new();
        for (_, row) in partition {
            contractid_array.push(row.contractid);
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            rundate_array.push(row.rundate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            authoriseddate_array.push(row.authoriseddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            authorisedby_array.push(row.authorisedby);
            postdate_array.push(row.postdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            postedby_array.push(row.postedby);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            status_array.push(row.status);
            auctionid_array.push(row.auctionid);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(contractid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rundate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(postdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(postedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(status_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(auctionid_array)),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SRA_CASH_SECURITY
///  _Records the Cash Security details provided by an SRA Auction Participant as collateral to cover their Trading Position in the SRA market_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Cash Security
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CASH_SECURITY_ID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraCashSecurity1 {
    /// Unique identifier for the cash security.
    pub cash_security_id: String,
    /// Unique identifier for the auction participant lodging the cash security.
    pub participantid: Option<String>,
    /// Date AEMO received the Cash Security deposit
    #[serde(with = "crate::mms_datetime_opt")]
    pub provision_date: Option<chrono::NaiveDateTime>,
    /// Dollar amount of the cash security.
    pub cash_amount: Option<rust_decimal::Decimal>,
    /// The interest account ID for calculating the interest payment
    pub interest_acct_id: Option<String>,
    /// Authorised date
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Date the entire Cash Security amount was returned to the Auction Participant
    #[serde(with = "crate::mms_datetime_opt")]
    pub finalreturndate: Option<chrono::NaiveDateTime>,
    /// Returned Dollar amount of the Cash Security.
    pub cash_security_returned: Option<rust_decimal::Decimal>,
    /// Cash Security deleted date. For valid records, DeletionDate will be Null.
    #[serde(with = "crate::mms_datetime_opt")]
    pub deletiondate: Option<chrono::NaiveDateTime>,
    /// The date and time this record was last modified
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionSraCashSecurity1 {
    type PrimaryKey = IrauctionSraCashSecurity1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_CASH_SECURITY".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionSraCashSecurity1PrimaryKey {
        IrauctionSraCashSecurity1PrimaryKey {
            cash_security_id: self.cash_security_id.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_sra_cash_security_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionSraCashSecurity1 {
    type Row = IrauctionSraCashSecurity1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.cash_security_id == row.cash_security_id
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraCashSecurity1 {
    type PrimaryKey = IrauctionSraCashSecurity1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.cash_security_id == key.cash_security_id
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraCashSecurity1PrimaryKey {
    pub cash_security_id: String,
}
impl crate::CompareWithRow for IrauctionSraCashSecurity1PrimaryKey {
    type Row = IrauctionSraCashSecurity1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.cash_security_id == row.cash_security_id
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraCashSecurity1PrimaryKey {
    type PrimaryKey = IrauctionSraCashSecurity1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.cash_security_id == key.cash_security_id
    }
}
impl crate::PrimaryKey for IrauctionSraCashSecurity1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionSraCashSecurity1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "cash_security_id",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "provision_date",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "cash_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "interest_acct_id",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "finalreturndate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "cash_security_returned",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "deletiondate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut cash_security_id_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut provision_date_array = Vec::new();
        let mut cash_amount_array = Vec::new();
        let mut interest_acct_id_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut finalreturndate_array = Vec::new();
        let mut cash_security_returned_array = Vec::new();
        let mut deletiondate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            cash_security_id_array.push(row.cash_security_id);
            participantid_array.push(row.participantid);
            provision_date_array.push(row.provision_date.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            cash_amount_array.push({
                row.cash_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            interest_acct_id_array.push(row.interest_acct_id);
            authoriseddate_array.push(row.authoriseddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            finalreturndate_array.push(row.finalreturndate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            cash_security_returned_array.push({
                row.cash_security_returned.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            deletiondate_array.push(row.deletiondate.map(|val| {
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
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    cash_security_id_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(participantid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(provision_date_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cash_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    interest_acct_id_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(finalreturndate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cash_security_returned_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(deletiondate_array)
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
/// ## SRA_FINANCIAL_AUCPAY_DETAIL
///  _Records details of the SRA financial auction payment_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Aucpay Detail
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucpayDetail1 {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// Unique  participant identifier
    pub participantid: String,
    /// The identifier for the Directional Interconnector
    pub interconnectorid: String,
    /// The source Region identifier for the Directional Interconnector
    pub fromregionid: String,
    /// The SRA contract identifier
    pub contractid: String,
    /// The Maximum Units Available for purchase in the Auction
    pub maximum_units: Option<rust_decimal::Decimal>,
    /// The total number of Allocated Units in the Auction, including Cancelled Units by an Auction Participant
    pub units_sold: Option<rust_decimal::Decimal>,
    /// The total number of units unpaid for in the auction
    pub shortfall_units: Option<rust_decimal::Decimal>,
    /// The reserve price of the auction
    pub reserve_price: Option<rust_decimal::Decimal>,
    /// The Market Clearing Price of the Auction
    pub clearing_price: Option<rust_decimal::Decimal>,
    /// The payment amount owed by AEMO before shortfall
    pub payment_amount: Option<rust_decimal::Decimal>,
    /// The shortfall amount
    pub shortfall_amount: Option<rust_decimal::Decimal>,
    /// The percentage of the auction proceeds allocated to the TNSP on the directional link
    pub allocation: Option<rust_decimal::Decimal>,
    /// The payment amount owed by AEMO, including shortfall
    pub net_payment_amount: Option<rust_decimal::Decimal>,
    /// The date and time this record was last modified
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionSraFinancialAucpayDetail1 {
    type PrimaryKey = IrauctionSraFinancialAucpayDetail1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_FINANCIAL_AUCPAY_DETAIL".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionSraFinancialAucpayDetail1PrimaryKey {
        IrauctionSraFinancialAucpayDetail1PrimaryKey {
            contractid: self.contractid.clone(),
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            participantid: self.participantid.clone(),
            sra_quarter: self.sra_quarter,
            sra_runno: self.sra_runno,
            sra_year: self.sra_year,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_sra_financial_aucpay_detail_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionSraFinancialAucpayDetail1 {
    type Row = IrauctionSraFinancialAucpayDetail1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid
            && self.sra_quarter == row.sra_quarter
            && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraFinancialAucpayDetail1 {
    type PrimaryKey = IrauctionSraFinancialAucpayDetail1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.sra_quarter == key.sra_quarter
            && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraFinancialAucpayDetail1PrimaryKey {
    pub contractid: String,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub participantid: String,
    pub sra_quarter: i64,
    pub sra_runno: i64,
    pub sra_year: i64,
}
impl crate::CompareWithRow for IrauctionSraFinancialAucpayDetail1PrimaryKey {
    type Row = IrauctionSraFinancialAucpayDetail1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid
            && self.sra_quarter == row.sra_quarter
            && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraFinancialAucpayDetail1PrimaryKey {
    type PrimaryKey = IrauctionSraFinancialAucpayDetail1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.sra_quarter == key.sra_quarter
            && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
impl crate::PrimaryKey for IrauctionSraFinancialAucpayDetail1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionSraFinancialAucpayDetail1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("sra_year", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("sra_quarter", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("sra_runno", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "fromregionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "maximum_units",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "units_sold",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "shortfall_units",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "reserve_price",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "clearing_price",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "payment_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "shortfall_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "allocation",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "net_payment_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut sra_year_array = Vec::new();
        let mut sra_quarter_array = Vec::new();
        let mut sra_runno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut maximum_units_array = Vec::new();
        let mut units_sold_array = Vec::new();
        let mut shortfall_units_array = Vec::new();
        let mut reserve_price_array = Vec::new();
        let mut clearing_price_array = Vec::new();
        let mut payment_amount_array = Vec::new();
        let mut shortfall_amount_array = Vec::new();
        let mut allocation_array = Vec::new();
        let mut net_payment_amount_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            sra_year_array.push(row.sra_year);
            sra_quarter_array.push(row.sra_quarter);
            sra_runno_array.push(row.sra_runno);
            participantid_array.push(row.participantid);
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            contractid_array.push(row.contractid);
            maximum_units_array.push({
                row.maximum_units.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            units_sold_array.push({
                row.units_sold.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            shortfall_units_array.push({
                row.shortfall_units.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            reserve_price_array.push({
                row.reserve_price.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            clearing_price_array.push({
                row.clearing_price.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            payment_amount_array.push({
                row.payment_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            shortfall_amount_array.push({
                row.shortfall_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            allocation_array.push({
                row.allocation.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            net_payment_amount_array.push({
                row.net_payment_amount.map(|mut val| {
                    val.rescale(8);
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
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_year_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_quarter_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_runno_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    fromregionid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    contractid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(maximum_units_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(units_sold_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(shortfall_units_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(reserve_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(clearing_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(payment_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(shortfall_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(allocation_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(net_payment_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
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
/// ## SRA_FINANCIAL_AUCPAY_SUM
///  _Records a summary of the Auction payment amount_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Aucpay Sum
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucpaySum1 {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// Unique participant identifier
    pub participantid: String,
    /// The total auction proceeds allocated to the TNSP
    pub gross_proceeds_amount: Option<rust_decimal::Decimal>,
    /// The total auction proceeds allocated to all TNSPs in the SRA quarter
    pub total_gross_proceeds_amount: Option<rust_decimal::Decimal>,
    /// The shortfall amount for in the SRA Quarter for the TNSP
    pub shortfall_amount: Option<rust_decimal::Decimal>,
    /// The total shortfall amount for in the SRA Quarter for all TNSPs
    pub total_shortfall_amount: Option<rust_decimal::Decimal>,
    /// The net payment amount owed by AEMO to the TNSP
    pub net_payment_amount: Option<rust_decimal::Decimal>,
    /// The date and time this record was last modified
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionSraFinancialAucpaySum1 {
    type PrimaryKey = IrauctionSraFinancialAucpaySum1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_FINANCIAL_AUCPAY_SUM".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionSraFinancialAucpaySum1PrimaryKey {
        IrauctionSraFinancialAucpaySum1PrimaryKey {
            participantid: self.participantid.clone(),
            sra_quarter: self.sra_quarter,
            sra_runno: self.sra_runno,
            sra_year: self.sra_year,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_sra_financial_aucpay_sum_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionSraFinancialAucpaySum1 {
    type Row = IrauctionSraFinancialAucpaySum1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.sra_quarter == row.sra_quarter
            && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraFinancialAucpaySum1 {
    type PrimaryKey = IrauctionSraFinancialAucpaySum1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.sra_quarter == key.sra_quarter
            && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraFinancialAucpaySum1PrimaryKey {
    pub participantid: String,
    pub sra_quarter: i64,
    pub sra_runno: i64,
    pub sra_year: i64,
}
impl crate::CompareWithRow for IrauctionSraFinancialAucpaySum1PrimaryKey {
    type Row = IrauctionSraFinancialAucpaySum1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.sra_quarter == row.sra_quarter
            && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraFinancialAucpaySum1PrimaryKey {
    type PrimaryKey = IrauctionSraFinancialAucpaySum1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.sra_quarter == key.sra_quarter
            && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
impl crate::PrimaryKey for IrauctionSraFinancialAucpaySum1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionSraFinancialAucpaySum1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("sra_year", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("sra_quarter", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("sra_runno", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "gross_proceeds_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "total_gross_proceeds_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "shortfall_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "total_shortfall_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "net_payment_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut sra_year_array = Vec::new();
        let mut sra_quarter_array = Vec::new();
        let mut sra_runno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut gross_proceeds_amount_array = Vec::new();
        let mut total_gross_proceeds_amount_array = Vec::new();
        let mut shortfall_amount_array = Vec::new();
        let mut total_shortfall_amount_array = Vec::new();
        let mut net_payment_amount_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            sra_year_array.push(row.sra_year);
            sra_quarter_array.push(row.sra_quarter);
            sra_runno_array.push(row.sra_runno);
            participantid_array.push(row.participantid);
            gross_proceeds_amount_array.push({
                row.gross_proceeds_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            total_gross_proceeds_amount_array.push({
                row.total_gross_proceeds_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            shortfall_amount_array.push({
                row.shortfall_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            total_shortfall_amount_array.push({
                row.total_shortfall_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            net_payment_amount_array.push({
                row.net_payment_amount.map(|mut val| {
                    val.rescale(8);
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
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_year_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_quarter_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_runno_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(gross_proceeds_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(total_gross_proceeds_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(shortfall_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(total_shortfall_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(net_payment_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
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
/// ## SRA_FINANCIAL_AUC_MARDETAIL
///  _This table stores details of the margins returned to the participants._
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Auc Mardetail
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CASH_SECURITY_ID
/// * PARTICIPANTID
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucMardetail1 {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// The participant identifier.
    pub participantid: String,
    /// Unique identifier for the cash security.
    pub cash_security_id: String,
    /// The amount returned to the Auction participant from this cash security.
    pub returned_amount: Option<rust_decimal::Decimal>,
    /// The amount of interest applicable to the returned amount.
    pub returned_interest: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IrauctionSraFinancialAucMardetail1 {
    type PrimaryKey = IrauctionSraFinancialAucMardetail1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_FINANCIAL_AUC_MARDETAIL".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionSraFinancialAucMardetail1PrimaryKey {
        IrauctionSraFinancialAucMardetail1PrimaryKey {
            cash_security_id: self.cash_security_id.clone(),
            participantid: self.participantid.clone(),
            sra_quarter: self.sra_quarter,
            sra_runno: self.sra_runno,
            sra_year: self.sra_year,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_sra_financial_auc_mardetail_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionSraFinancialAucMardetail1 {
    type Row = IrauctionSraFinancialAucMardetail1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.cash_security_id == row.cash_security_id
            && self.participantid == row.participantid
            && self.sra_quarter == row.sra_quarter
            && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraFinancialAucMardetail1 {
    type PrimaryKey = IrauctionSraFinancialAucMardetail1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.cash_security_id == key.cash_security_id
            && self.participantid == key.participantid
            && self.sra_quarter == key.sra_quarter
            && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraFinancialAucMardetail1PrimaryKey {
    pub cash_security_id: String,
    pub participantid: String,
    pub sra_quarter: i64,
    pub sra_runno: i64,
    pub sra_year: i64,
}
impl crate::CompareWithRow for IrauctionSraFinancialAucMardetail1PrimaryKey {
    type Row = IrauctionSraFinancialAucMardetail1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.cash_security_id == row.cash_security_id
            && self.participantid == row.participantid
            && self.sra_quarter == row.sra_quarter
            && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraFinancialAucMardetail1PrimaryKey {
    type PrimaryKey = IrauctionSraFinancialAucMardetail1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.cash_security_id == key.cash_security_id
            && self.participantid == key.participantid
            && self.sra_quarter == key.sra_quarter
            && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
impl crate::PrimaryKey for IrauctionSraFinancialAucMardetail1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionSraFinancialAucMardetail1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("sra_year", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("sra_quarter", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("sra_runno", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "cash_security_id",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "returned_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "returned_interest",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut sra_year_array = Vec::new();
        let mut sra_quarter_array = Vec::new();
        let mut sra_runno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut cash_security_id_array = Vec::new();
        let mut returned_amount_array = Vec::new();
        let mut returned_interest_array = Vec::new();
        for (_, row) in partition {
            sra_year_array.push(row.sra_year);
            sra_quarter_array.push(row.sra_quarter);
            sra_runno_array.push(row.sra_runno);
            participantid_array.push(row.participantid);
            cash_security_id_array.push(row.cash_security_id);
            returned_amount_array.push({
                row.returned_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            returned_interest_array.push({
                row.returned_interest.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_year_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_quarter_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_runno_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    cash_security_id_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(returned_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(returned_interest_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SRA_FINANCIAL_AUC_MARGIN
///  _Records the amount of Cash Security required to be held by an Auction Participant after settlement_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Auc Margin
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucMargin1 {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// Unique  participant identifier.
    pub participantid: String,
    /// Total cash security held by the participant.
    pub total_cash_security: Option<rust_decimal::Decimal>,
    /// The amount of trading  cash security required to be held by the participant after settlement.
    pub required_margin: Option<rust_decimal::Decimal>,
    /// The amount of cash security returned to the participant.
    pub returned_margin: Option<rust_decimal::Decimal>,
    /// The amount of interest applicable to returned cash security amounts.
    pub returned_margin_interest: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IrauctionSraFinancialAucMargin1 {
    type PrimaryKey = IrauctionSraFinancialAucMargin1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_FINANCIAL_AUC_MARGIN".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionSraFinancialAucMargin1PrimaryKey {
        IrauctionSraFinancialAucMargin1PrimaryKey {
            participantid: self.participantid.clone(),
            sra_quarter: self.sra_quarter,
            sra_runno: self.sra_runno,
            sra_year: self.sra_year,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_sra_financial_auc_margin_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionSraFinancialAucMargin1 {
    type Row = IrauctionSraFinancialAucMargin1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.sra_quarter == row.sra_quarter
            && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraFinancialAucMargin1 {
    type PrimaryKey = IrauctionSraFinancialAucMargin1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.sra_quarter == key.sra_quarter
            && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraFinancialAucMargin1PrimaryKey {
    pub participantid: String,
    pub sra_quarter: i64,
    pub sra_runno: i64,
    pub sra_year: i64,
}
impl crate::CompareWithRow for IrauctionSraFinancialAucMargin1PrimaryKey {
    type Row = IrauctionSraFinancialAucMargin1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.sra_quarter == row.sra_quarter
            && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraFinancialAucMargin1PrimaryKey {
    type PrimaryKey = IrauctionSraFinancialAucMargin1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.sra_quarter == key.sra_quarter
            && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
impl crate::PrimaryKey for IrauctionSraFinancialAucMargin1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionSraFinancialAucMargin1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("sra_year", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("sra_quarter", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("sra_runno", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "total_cash_security",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "required_margin",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "returned_margin",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "returned_margin_interest",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut sra_year_array = Vec::new();
        let mut sra_quarter_array = Vec::new();
        let mut sra_runno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut total_cash_security_array = Vec::new();
        let mut required_margin_array = Vec::new();
        let mut returned_margin_array = Vec::new();
        let mut returned_margin_interest_array = Vec::new();
        for (_, row) in partition {
            sra_year_array.push(row.sra_year);
            sra_quarter_array.push(row.sra_quarter);
            sra_runno_array.push(row.sra_runno);
            participantid_array.push(row.participantid);
            total_cash_security_array.push({
                row.total_cash_security.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            required_margin_array.push({
                row.required_margin.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            returned_margin_array.push({
                row.returned_margin.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            returned_margin_interest_array.push({
                row.returned_margin_interest.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_year_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_quarter_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_runno_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(total_cash_security_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(required_margin_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(returned_margin_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(returned_margin_interest_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SRA_FINANCIAL_AUC_RECEIPTS
///  _Records details of the Cancelled Units and their value for the Auction Participant_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Auc Receipts
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialAucReceipts1 {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// Unique participant identifier
    pub participantid: String,
    /// The identifier for the Directional Interconnector
    pub interconnectorid: String,
    /// The source region identifier for the Directional Interconnector
    pub fromregionid: String,
    /// The SRA contract identifier
    pub contractid: String,
    /// The number of units purchased
    pub units_purchased: Option<rust_decimal::Decimal>,
    /// The clearing price of the auction
    pub clearing_price: Option<rust_decimal::Decimal>,
    /// The payment amount owed to AEMO
    pub receipt_amount: Option<rust_decimal::Decimal>,
    /// The last changed date for the record
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Dollar value of Cancelled Units in the Auction for the Auction Participant
    pub proceeds_amount: Option<rust_decimal::Decimal>,
    /// Units cancelled in the auction by the Auction  participant.
    pub units_sold: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IrauctionSraFinancialAucReceipts1 {
    type PrimaryKey = IrauctionSraFinancialAucReceipts1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_FINANCIAL_AUC_RECEIPTS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionSraFinancialAucReceipts1PrimaryKey {
        IrauctionSraFinancialAucReceipts1PrimaryKey {
            contractid: self.contractid.clone(),
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            participantid: self.participantid.clone(),
            sra_quarter: self.sra_quarter,
            sra_runno: self.sra_runno,
            sra_year: self.sra_year,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_sra_financial_auc_receipts_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionSraFinancialAucReceipts1 {
    type Row = IrauctionSraFinancialAucReceipts1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid
            && self.sra_quarter == row.sra_quarter
            && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraFinancialAucReceipts1 {
    type PrimaryKey = IrauctionSraFinancialAucReceipts1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.sra_quarter == key.sra_quarter
            && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraFinancialAucReceipts1PrimaryKey {
    pub contractid: String,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub participantid: String,
    pub sra_quarter: i64,
    pub sra_runno: i64,
    pub sra_year: i64,
}
impl crate::CompareWithRow for IrauctionSraFinancialAucReceipts1PrimaryKey {
    type Row = IrauctionSraFinancialAucReceipts1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid
            && self.sra_quarter == row.sra_quarter
            && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraFinancialAucReceipts1PrimaryKey {
    type PrimaryKey = IrauctionSraFinancialAucReceipts1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.sra_quarter == key.sra_quarter
            && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
impl crate::PrimaryKey for IrauctionSraFinancialAucReceipts1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionSraFinancialAucReceipts1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("sra_year", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("sra_quarter", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("sra_runno", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "fromregionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "units_purchased",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "clearing_price",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "receipt_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "proceeds_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "units_sold",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut sra_year_array = Vec::new();
        let mut sra_quarter_array = Vec::new();
        let mut sra_runno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut units_purchased_array = Vec::new();
        let mut clearing_price_array = Vec::new();
        let mut receipt_amount_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut proceeds_amount_array = Vec::new();
        let mut units_sold_array = Vec::new();
        for (_, row) in partition {
            sra_year_array.push(row.sra_year);
            sra_quarter_array.push(row.sra_quarter);
            sra_runno_array.push(row.sra_runno);
            participantid_array.push(row.participantid);
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            contractid_array.push(row.contractid);
            units_purchased_array.push({
                row.units_purchased.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            clearing_price_array.push({
                row.clearing_price.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            receipt_amount_array.push({
                row.receipt_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            proceeds_amount_array.push({
                row.proceeds_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            units_sold_array.push({
                row.units_sold.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_year_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_quarter_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_runno_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    fromregionid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    contractid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(units_purchased_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(clearing_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(receipt_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(proceeds_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(units_sold_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SRA_FINANCIAL_RUNTRK
///  _Records details of the settlement process for the cancellation and purchase of SRA Auction Units_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Financial Runtrk
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * SRA_QUARTER
/// * SRA_RUNNO
/// * SRA_YEAR
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraFinancialRuntrk1 {
    /// Year of the Tranche
    pub sra_year: i64,
    /// Relevant Quarter of the Tranche
    pub sra_quarter: i64,
    /// SRA Run Number
    pub sra_runno: i64,
    /// The type of SRA run
    pub runtype: Option<String>,
    /// The date and time the run was triggered
    #[serde(with = "crate::mms_datetime_opt")]
    pub rundate: Option<chrono::NaiveDateTime>,
    /// The date/time the run was posted
    #[serde(with = "crate::mms_datetime_opt")]
    pub posteddate: Option<chrono::NaiveDateTime>,
    /// Version number of the interest component used in the payments run
    pub interest_versionno: Option<i64>,
    /// Version number of the makeup component used in the makeup run
    pub makeup_versionno: Option<i64>,
    /// The date and time this record was last modified
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionSraFinancialRuntrk1 {
    type PrimaryKey = IrauctionSraFinancialRuntrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_FINANCIAL_RUNTRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionSraFinancialRuntrk1PrimaryKey {
        IrauctionSraFinancialRuntrk1PrimaryKey {
            sra_quarter: self.sra_quarter,
            sra_runno: self.sra_runno,
            sra_year: self.sra_year,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_sra_financial_runtrk_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionSraFinancialRuntrk1 {
    type Row = IrauctionSraFinancialRuntrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.sra_quarter == row.sra_quarter
            && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraFinancialRuntrk1 {
    type PrimaryKey = IrauctionSraFinancialRuntrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.sra_quarter == key.sra_quarter
            && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraFinancialRuntrk1PrimaryKey {
    pub sra_quarter: i64,
    pub sra_runno: i64,
    pub sra_year: i64,
}
impl crate::CompareWithRow for IrauctionSraFinancialRuntrk1PrimaryKey {
    type Row = IrauctionSraFinancialRuntrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.sra_quarter == row.sra_quarter
            && self.sra_runno == row.sra_runno
            && self.sra_year == row.sra_year
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraFinancialRuntrk1PrimaryKey {
    type PrimaryKey = IrauctionSraFinancialRuntrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.sra_quarter == key.sra_quarter
            && self.sra_runno == key.sra_runno
            && self.sra_year == key.sra_year
    }
}
impl crate::PrimaryKey for IrauctionSraFinancialRuntrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionSraFinancialRuntrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("sra_year", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("sra_quarter", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("sra_runno", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("runtype", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("rundate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("posteddate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "interest_versionno",
                arrow2::datatypes::DataType::Int64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "makeup_versionno",
                arrow2::datatypes::DataType::Int64,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut sra_year_array = Vec::new();
        let mut sra_quarter_array = Vec::new();
        let mut sra_runno_array = Vec::new();
        let mut runtype_array = Vec::new();
        let mut rundate_array = Vec::new();
        let mut posteddate_array = Vec::new();
        let mut interest_versionno_array = Vec::new();
        let mut makeup_versionno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            sra_year_array.push(row.sra_year);
            sra_quarter_array.push(row.sra_quarter);
            sra_runno_array.push(row.sra_runno);
            runtype_array.push(row.runtype);
            rundate_array.push(row.rundate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            posteddate_array.push(row.posteddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            interest_versionno_array.push(row.interest_versionno);
            makeup_versionno_array.push(row.makeup_versionno);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_year_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_quarter_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_runno_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(runtype_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rundate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(posteddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(
                    interest_versionno_array,
                )),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(makeup_versionno_array)),
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
/// ## SRA_OFFER_PRODUCT
///  _Holds the Product details for each Offer File submitted by each SRA Auction Participant._
///
/// * Data Set Name: Irauction
/// * File Name: Sra Offer Product
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * LOADDATE
/// * OPTIONID
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraOfferProduct1 {
    /// Unique ID for each Auction date
    pub auctionid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// The date and time the system loaded the SRA Offer File
    #[serde(with = "crate::mms_datetime")]
    pub loaddate: chrono::NaiveDateTime,
    /// Unique Product identifier (1 - 2000)
    pub optionid: i64,
    /// Unique Directional Interconnector identifier
    pub interconnectorid: Option<String>,
    /// The source Region identifier for the Directional Interconnector
    pub fromregionid: Option<String>,
    /// The Offer quantity for this Product
    pub offer_quantity: Option<i64>,
    /// The Offer price for this Product
    pub offer_price: Option<rust_decimal::Decimal>,
    /// Tranche identifier
    pub trancheid: Option<String>,
    /// The date and time this record was last modified
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionSraOfferProduct1 {
    type PrimaryKey = IrauctionSraOfferProduct1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_OFFER_PRODUCT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionSraOfferProduct1PrimaryKey {
        IrauctionSraOfferProduct1PrimaryKey {
            auctionid: self.auctionid.clone(),
            loaddate: self.loaddate,
            optionid: self.optionid,
            participantid: self.participantid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_sra_offer_product_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionSraOfferProduct1 {
    type Row = IrauctionSraOfferProduct1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.auctionid == row.auctionid
            && self.loaddate == row.loaddate
            && self.optionid == row.optionid
            && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraOfferProduct1 {
    type PrimaryKey = IrauctionSraOfferProduct1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid
            && self.loaddate == key.loaddate
            && self.optionid == key.optionid
            && self.participantid == key.participantid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraOfferProduct1PrimaryKey {
    pub auctionid: String,
    pub loaddate: chrono::NaiveDateTime,
    pub optionid: i64,
    pub participantid: String,
}
impl crate::CompareWithRow for IrauctionSraOfferProduct1PrimaryKey {
    type Row = IrauctionSraOfferProduct1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.auctionid == row.auctionid
            && self.loaddate == row.loaddate
            && self.optionid == row.optionid
            && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraOfferProduct1PrimaryKey {
    type PrimaryKey = IrauctionSraOfferProduct1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid
            && self.loaddate == key.loaddate
            && self.optionid == key.optionid
            && self.participantid == key.participantid
    }
}
impl crate::PrimaryKey for IrauctionSraOfferProduct1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionSraOfferProduct1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "auctionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("loaddate", arrow2::datatypes::DataType::Date32, false),
            arrow2::datatypes::Field::new("optionid", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "fromregionid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "offer_quantity",
                arrow2::datatypes::DataType::Int64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "offer_price",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "trancheid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut auctionid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut loaddate_array = Vec::new();
        let mut optionid_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut offer_quantity_array = Vec::new();
        let mut offer_price_array = Vec::new();
        let mut trancheid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            auctionid_array.push(row.auctionid);
            participantid_array.push(row.participantid);
            loaddate_array.push(
                i32::try_from(
                    (row.loaddate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            optionid_array.push(row.optionid);
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            offer_quantity_array.push(row.offer_quantity);
            offer_price_array.push({
                row.offer_price.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            trancheid_array.push(row.trancheid);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(auctionid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(loaddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(optionid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(fromregionid_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(offer_quantity_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(offer_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(trancheid_array)),
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
/// ## SRA_OFFER_PROFILE
///  _Holds the data of an SRA Auction Participant Offer Submission._
///
/// * Data Set Name: Irauction
/// * File Name: Sra Offer Profile
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * AUCTIONID
/// * LOADDATE
/// * PARTICIPANTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraOfferProfile1 {
    /// Unique ID for each Auction date
    pub auctionid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// The date and time the system loaded the SRA Offer File
    #[serde(with = "crate::mms_datetime")]
    pub loaddate: chrono::NaiveDateTime,
    /// SRA Offer File name
    pub filename: Option<String>,
    /// SRA acknowledgment file name
    pub ackfilename: Option<String>,
    /// Transaction ID used for tracking
    pub transactionid: Option<String>,
    /// The date and time this record was last modified
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionSraOfferProfile1 {
    type PrimaryKey = IrauctionSraOfferProfile1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_OFFER_PROFILE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionSraOfferProfile1PrimaryKey {
        IrauctionSraOfferProfile1PrimaryKey {
            auctionid: self.auctionid.clone(),
            loaddate: self.loaddate,
            participantid: self.participantid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_sra_offer_profile_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionSraOfferProfile1 {
    type Row = IrauctionSraOfferProfile1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.auctionid == row.auctionid
            && self.loaddate == row.loaddate
            && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraOfferProfile1 {
    type PrimaryKey = IrauctionSraOfferProfile1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid
            && self.loaddate == key.loaddate
            && self.participantid == key.participantid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraOfferProfile1PrimaryKey {
    pub auctionid: String,
    pub loaddate: chrono::NaiveDateTime,
    pub participantid: String,
}
impl crate::CompareWithRow for IrauctionSraOfferProfile1PrimaryKey {
    type Row = IrauctionSraOfferProfile1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.auctionid == row.auctionid
            && self.loaddate == row.loaddate
            && self.participantid == row.participantid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraOfferProfile1PrimaryKey {
    type PrimaryKey = IrauctionSraOfferProfile1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.auctionid == key.auctionid
            && self.loaddate == key.loaddate
            && self.participantid == key.participantid
    }
}
impl crate::PrimaryKey for IrauctionSraOfferProfile1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionSraOfferProfile1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "auctionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("loaddate", arrow2::datatypes::DataType::Date32, false),
            arrow2::datatypes::Field::new("filename", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "ackfilename",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "transactionid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut auctionid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut loaddate_array = Vec::new();
        let mut filename_array = Vec::new();
        let mut ackfilename_array = Vec::new();
        let mut transactionid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            auctionid_array.push(row.auctionid);
            participantid_array.push(row.participantid);
            loaddate_array.push(
                i32::try_from(
                    (row.loaddate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            filename_array.push(row.filename);
            ackfilename_array.push(row.ackfilename);
            transactionid_array.push(row.transactionid);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(auctionid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(loaddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(filename_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(ackfilename_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(transactionid_array)),
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
/// ## SRA_PRUDENTIAL_CASH_SECURITY
///  _Records the Cash Security details provided by an SRA Auction Participant as collateral to cover their Trading Position in the SRA market_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Prudential Cash Security
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CASH_SECURITY_ID
/// * PARTICIPANTID
/// * PRUDENTIAL_DATE
/// * PRUDENTIAL_RUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraPrudentialCashSecurity1 {
    /// The prudential date of the run.
    #[serde(with = "crate::mms_datetime")]
    pub prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date
    pub prudential_runno: i64,
    /// Unique participant identifier for the Auction Participant lodging the Cash Security
    pub participantid: String,
    /// Unique identifier for the cash security.
    pub cash_security_id: String,
    /// Remaining Cash Security deposit available
    pub cash_security_amount: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IrauctionSraPrudentialCashSecurity1 {
    type PrimaryKey = IrauctionSraPrudentialCashSecurity1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_PRUDENTIAL_CASH_SECURITY".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionSraPrudentialCashSecurity1PrimaryKey {
        IrauctionSraPrudentialCashSecurity1PrimaryKey {
            cash_security_id: self.cash_security_id.clone(),
            participantid: self.participantid.clone(),
            prudential_date: self.prudential_date,
            prudential_runno: self.prudential_runno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_sra_prudential_cash_security_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionSraPrudentialCashSecurity1 {
    type Row = IrauctionSraPrudentialCashSecurity1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.cash_security_id == row.cash_security_id
            && self.participantid == row.participantid
            && self.prudential_date == row.prudential_date
            && self.prudential_runno == row.prudential_runno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraPrudentialCashSecurity1 {
    type PrimaryKey = IrauctionSraPrudentialCashSecurity1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.cash_security_id == key.cash_security_id
            && self.participantid == key.participantid
            && self.prudential_date == key.prudential_date
            && self.prudential_runno == key.prudential_runno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraPrudentialCashSecurity1PrimaryKey {
    pub cash_security_id: String,
    pub participantid: String,
    pub prudential_date: chrono::NaiveDateTime,
    pub prudential_runno: i64,
}
impl crate::CompareWithRow for IrauctionSraPrudentialCashSecurity1PrimaryKey {
    type Row = IrauctionSraPrudentialCashSecurity1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.cash_security_id == row.cash_security_id
            && self.participantid == row.participantid
            && self.prudential_date == row.prudential_date
            && self.prudential_runno == row.prudential_runno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraPrudentialCashSecurity1PrimaryKey {
    type PrimaryKey = IrauctionSraPrudentialCashSecurity1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.cash_security_id == key.cash_security_id
            && self.participantid == key.participantid
            && self.prudential_date == key.prudential_date
            && self.prudential_runno == key.prudential_runno
    }
}
impl crate::PrimaryKey for IrauctionSraPrudentialCashSecurity1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionSraPrudentialCashSecurity1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "prudential_date",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "prudential_runno",
                arrow2::datatypes::DataType::Int64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "cash_security_id",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "cash_security_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut prudential_date_array = Vec::new();
        let mut prudential_runno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut cash_security_id_array = Vec::new();
        let mut cash_security_amount_array = Vec::new();
        for (_, row) in partition {
            prudential_date_array.push(
                i32::try_from(
                    (row.prudential_date.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            prudential_runno_array.push(row.prudential_runno);
            participantid_array.push(row.participantid);
            cash_security_id_array.push(row.cash_security_id);
            cash_security_amount_array.push({
                row.cash_security_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(prudential_date_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(
                    prudential_runno_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    cash_security_id_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cash_security_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SRA_PRUDENTIAL_COMP_POSITION
///  _The prudential position of each company at the date and time of a specific prudential run_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Prudential Comp Position
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * PRUDENTIAL_DATE
/// * PRUDENTIAL_RUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraPrudentialCompPosition1 {
    /// The prudential date of the run.
    #[serde(with = "crate::mms_datetime")]
    pub prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date
    pub prudential_runno: i64,
    /// Unique participant identifier
    pub participantid: String,
    /// The Trading Limit of the company at the time of the prudential run
    pub trading_limit: Option<rust_decimal::Decimal>,
    /// Current Prudential Exposure of the Auction Participant including Offers
    pub prudential_exposure_amount: Option<rust_decimal::Decimal>,
    /// The amount of Trading Margin available to the Auction Participant to trade (including Offered Units and Cancelled Units). Equal to TRADING_LIMIT minus PRUDENTIAL_EXPOSURE_AMOUNT.
    pub trading_margin: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IrauctionSraPrudentialCompPosition1 {
    type PrimaryKey = IrauctionSraPrudentialCompPosition1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_PRUDENTIAL_COMP_POSITION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionSraPrudentialCompPosition1PrimaryKey {
        IrauctionSraPrudentialCompPosition1PrimaryKey {
            participantid: self.participantid.clone(),
            prudential_date: self.prudential_date,
            prudential_runno: self.prudential_runno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_sra_prudential_comp_position_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionSraPrudentialCompPosition1 {
    type Row = IrauctionSraPrudentialCompPosition1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.prudential_date == row.prudential_date
            && self.prudential_runno == row.prudential_runno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraPrudentialCompPosition1 {
    type PrimaryKey = IrauctionSraPrudentialCompPosition1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.prudential_date == key.prudential_date
            && self.prudential_runno == key.prudential_runno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraPrudentialCompPosition1PrimaryKey {
    pub participantid: String,
    pub prudential_date: chrono::NaiveDateTime,
    pub prudential_runno: i64,
}
impl crate::CompareWithRow for IrauctionSraPrudentialCompPosition1PrimaryKey {
    type Row = IrauctionSraPrudentialCompPosition1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.prudential_date == row.prudential_date
            && self.prudential_runno == row.prudential_runno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraPrudentialCompPosition1PrimaryKey {
    type PrimaryKey = IrauctionSraPrudentialCompPosition1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.prudential_date == key.prudential_date
            && self.prudential_runno == key.prudential_runno
    }
}
impl crate::PrimaryKey for IrauctionSraPrudentialCompPosition1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionSraPrudentialCompPosition1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "prudential_date",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "prudential_runno",
                arrow2::datatypes::DataType::Int64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "trading_limit",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "prudential_exposure_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "trading_margin",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut prudential_date_array = Vec::new();
        let mut prudential_runno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut trading_limit_array = Vec::new();
        let mut prudential_exposure_amount_array = Vec::new();
        let mut trading_margin_array = Vec::new();
        for (_, row) in partition {
            prudential_date_array.push(
                i32::try_from(
                    (row.prudential_date.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            prudential_runno_array.push(row.prudential_runno);
            participantid_array.push(row.participantid);
            trading_limit_array.push({
                row.trading_limit.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            prudential_exposure_amount_array.push({
                row.prudential_exposure_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            trading_margin_array.push({
                row.trading_margin.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(prudential_date_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(
                    prudential_runno_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(trading_limit_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(prudential_exposure_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(trading_margin_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SRA_PRUDENTIAL_EXPOSURE
///  _Records details of the Prudential Exposure of an SRA Auction Participant_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Prudential Exposure
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * PRUDENTIAL_DATE
/// * PRUDENTIAL_RUNNO
/// * SRA_QUARTER
/// * SRA_YEAR
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraPrudentialExposure1 {
    /// The prudential date of the run.
    #[serde(with = "crate::mms_datetime")]
    pub prudential_date: chrono::NaiveDateTime,
    /// The run number for the prudential date.
    pub prudential_runno: i64,
    /// Unique participant identifier
    pub participantid: String,
    /// AEMO Contract Year number starting the week beginning 1 January
    pub sra_year: i64,
    /// Contract Relevant Quarter
    pub sra_quarter: i64,
    /// The identifier for the Directional Interconnector
    pub interconnectorid: String,
    /// The source Region identifier for the Directional Interconnector
    pub fromregionid: String,
    /// The largest Tranche where the Unit was either sold or offered. Used in the calculation of the Average Purchase Price for the Trading Position of the Product. The most recent Tranche where Units were cancelled or offered (if the Offer is below the Average Purchase Price)
    pub max_tranche: Option<i64>,
    /// Unique identifier for the Auction having the Offer. Has a null value when no Offer is made for the Relevant Quarter
    pub auctionid: Option<String>,
    /// Timestamp of the Offer File submitted by the Auction Participant. Has a null value when no Offer is made for the Relevant Quarter
    #[serde(with = "crate::mms_datetime_opt")]
    pub offer_submissiontime: Option<chrono::NaiveDateTime>,
    /// Calculated Average Purchase Price for the Product
    pub average_purchase_price: Option<rust_decimal::Decimal>,
    /// Calculated average cancellation price for product.
    pub average_cancellation_price: Option<rust_decimal::Decimal>,
    /// Calculated cancellation volume for product.
    pub cancellation_volume: Option<rust_decimal::Decimal>,
    /// Calculated trading position for product.
    pub trading_position: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for IrauctionSraPrudentialExposure1 {
    type PrimaryKey = IrauctionSraPrudentialExposure1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_PRUDENTIAL_EXPOSURE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionSraPrudentialExposure1PrimaryKey {
        IrauctionSraPrudentialExposure1PrimaryKey {
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            participantid: self.participantid.clone(),
            prudential_date: self.prudential_date,
            prudential_runno: self.prudential_runno,
            sra_quarter: self.sra_quarter,
            sra_year: self.sra_year,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_sra_prudential_exposure_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionSraPrudentialExposure1 {
    type Row = IrauctionSraPrudentialExposure1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid
            && self.prudential_date == row.prudential_date
            && self.prudential_runno == row.prudential_runno
            && self.sra_quarter == row.sra_quarter
            && self.sra_year == row.sra_year
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraPrudentialExposure1 {
    type PrimaryKey = IrauctionSraPrudentialExposure1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.prudential_date == key.prudential_date
            && self.prudential_runno == key.prudential_runno
            && self.sra_quarter == key.sra_quarter
            && self.sra_year == key.sra_year
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraPrudentialExposure1PrimaryKey {
    pub fromregionid: String,
    pub interconnectorid: String,
    pub participantid: String,
    pub prudential_date: chrono::NaiveDateTime,
    pub prudential_runno: i64,
    pub sra_quarter: i64,
    pub sra_year: i64,
}
impl crate::CompareWithRow for IrauctionSraPrudentialExposure1PrimaryKey {
    type Row = IrauctionSraPrudentialExposure1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid
            && self.prudential_date == row.prudential_date
            && self.prudential_runno == row.prudential_runno
            && self.sra_quarter == row.sra_quarter
            && self.sra_year == row.sra_year
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraPrudentialExposure1PrimaryKey {
    type PrimaryKey = IrauctionSraPrudentialExposure1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.prudential_date == key.prudential_date
            && self.prudential_runno == key.prudential_runno
            && self.sra_quarter == key.sra_quarter
            && self.sra_year == key.sra_year
    }
}
impl crate::PrimaryKey for IrauctionSraPrudentialExposure1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionSraPrudentialExposure1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "prudential_date",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "prudential_runno",
                arrow2::datatypes::DataType::Int64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("sra_year", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("sra_quarter", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "fromregionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("max_tranche", arrow2::datatypes::DataType::Int64, true),
            arrow2::datatypes::Field::new(
                "auctionid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "offer_submissiontime",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "average_purchase_price",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "average_cancellation_price",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "cancellation_volume",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "trading_position",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut prudential_date_array = Vec::new();
        let mut prudential_runno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut sra_year_array = Vec::new();
        let mut sra_quarter_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut max_tranche_array = Vec::new();
        let mut auctionid_array = Vec::new();
        let mut offer_submissiontime_array = Vec::new();
        let mut average_purchase_price_array = Vec::new();
        let mut average_cancellation_price_array = Vec::new();
        let mut cancellation_volume_array = Vec::new();
        let mut trading_position_array = Vec::new();
        for (_, row) in partition {
            prudential_date_array.push(
                i32::try_from(
                    (row.prudential_date.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            prudential_runno_array.push(row.prudential_runno);
            participantid_array.push(row.participantid);
            sra_year_array.push(row.sra_year);
            sra_quarter_array.push(row.sra_quarter);
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            max_tranche_array.push(row.max_tranche);
            auctionid_array.push(row.auctionid);
            offer_submissiontime_array.push(row.offer_submissiontime.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            average_purchase_price_array.push({
                row.average_purchase_price.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            average_cancellation_price_array.push({
                row.average_cancellation_price.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            cancellation_volume_array.push({
                row.cancellation_volume.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            trading_position_array.push({
                row.trading_position.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(prudential_date_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(
                    prudential_runno_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_year_array)),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(sra_quarter_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    fromregionid_array,
                )),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(max_tranche_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(auctionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(offer_submissiontime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(average_purchase_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(average_cancellation_price_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cancellation_volume_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(trading_position_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SRA_PRUDENTIAL_RUN
///  _Records the prudential run details for each prudential date_
///
/// * Data Set Name: Irauction
/// * File Name: Sra Prudential Run
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * PRUDENTIAL_DATE
/// * PRUDENTIAL_RUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionSraPrudentialRun1 {
    /// The prudential date of the run.
    #[serde(with = "crate::mms_datetime")]
    pub prudential_date: chrono::NaiveDateTime,
    /// The prudential run number for the run
    pub prudential_runno: i64,
}
impl crate::GetTable for IrauctionSraPrudentialRun1 {
    type PrimaryKey = IrauctionSraPrudentialRun1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("SRA_PRUDENTIAL_RUN".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionSraPrudentialRun1PrimaryKey {
        IrauctionSraPrudentialRun1PrimaryKey {
            prudential_date: self.prudential_date,
            prudential_runno: self.prudential_runno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_sra_prudential_run_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionSraPrudentialRun1 {
    type Row = IrauctionSraPrudentialRun1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.prudential_date == row.prudential_date && self.prudential_runno == row.prudential_runno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraPrudentialRun1 {
    type PrimaryKey = IrauctionSraPrudentialRun1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.prudential_date == key.prudential_date && self.prudential_runno == key.prudential_runno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionSraPrudentialRun1PrimaryKey {
    pub prudential_date: chrono::NaiveDateTime,
    pub prudential_runno: i64,
}
impl crate::CompareWithRow for IrauctionSraPrudentialRun1PrimaryKey {
    type Row = IrauctionSraPrudentialRun1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.prudential_date == row.prudential_date && self.prudential_runno == row.prudential_runno
    }
}
impl crate::CompareWithPrimaryKey for IrauctionSraPrudentialRun1PrimaryKey {
    type PrimaryKey = IrauctionSraPrudentialRun1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.prudential_date == key.prudential_date && self.prudential_runno == key.prudential_runno
    }
}
impl crate::PrimaryKey for IrauctionSraPrudentialRun1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionSraPrudentialRun1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "prudential_date",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "prudential_runno",
                arrow2::datatypes::DataType::Int64,
                false,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut prudential_date_array = Vec::new();
        let mut prudential_runno_array = Vec::new();
        for (_, row) in partition {
            prudential_date_array.push(
                i32::try_from(
                    (row.prudential_date.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            prudential_runno_array.push(row.prudential_runno);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(prudential_date_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(
                    prudential_runno_array,
                )),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## VALUATIONID
///  _VALUATIONID shows the identifiers and descriptions of the valuers submitting estimates of upcoming settlement residues. VALUATIONID supports the Settlement Residue Auction._
///
/// * Data Set Name: Irauction
/// * File Name: Valuationid
/// * Data Version: 1
///
/// # Description
///  VALUATIONID is public data, and is available to all participants. Source VALUATIONID updates are quarterly from the Settlement Residues Information System [SRIS]. Volume VALUATIONID shows up to five (5) records. Updates are rare.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * VALUATIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct IrauctionValuationid1 {
    /// Identifier of the estimator
    pub valuationid: String,
    /// Full name of estimator
    pub description: Option<String>,
    /// Timestamp of record creation or modification
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for IrauctionValuationid1 {
    type PrimaryKey = IrauctionValuationid1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "IRAUCTION".into(),
            table_name: Some("VALUATIONID".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> IrauctionValuationid1PrimaryKey {
        IrauctionValuationid1PrimaryKey {
            valuationid: self.valuationid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "irauction_valuationid_v1".to_string()
    }
}
impl crate::CompareWithRow for IrauctionValuationid1 {
    type Row = IrauctionValuationid1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.valuationid == row.valuationid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionValuationid1 {
    type PrimaryKey = IrauctionValuationid1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.valuationid == key.valuationid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IrauctionValuationid1PrimaryKey {
    pub valuationid: String,
}
impl crate::CompareWithRow for IrauctionValuationid1PrimaryKey {
    type Row = IrauctionValuationid1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.valuationid == row.valuationid
    }
}
impl crate::CompareWithPrimaryKey for IrauctionValuationid1PrimaryKey {
    type PrimaryKey = IrauctionValuationid1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.valuationid == key.valuationid
    }
}
impl crate::PrimaryKey for IrauctionValuationid1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for IrauctionValuationid1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "valuationid",
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
        let mut valuationid_array = Vec::new();
        let mut description_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            valuationid_array.push(row.valuationid);
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
                    valuationid_array,
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
