use chrono::Datelike as _;
/// # Summary
///
/// ## AVERAGEPRICE30
///  _Reflects the 30-minute average price (the pre-5MS trading price)._
///
/// * Data Set Name: Trading
/// * File Name: Averageprice30
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * PERIODDATE
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TradingAverageprice301 {
    /// 30-minute interval period, 1 to 48 from the start of the calendar day
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub perioddate: chrono::NaiveDateTime,
    /// Region Identifier
    pub regionid: String,
    /// The 30-minute interval period, 1 to 48
    pub periodid: rust_decimal::Decimal,
    /// Regional reference price for this period
    pub rrp: Option<rust_decimal::Decimal>,
    /// Result of Manifestly Incorrect Inputs Price Status and OCD_Status - either "FIRM" or "NOT FIRM". Only FIRM if the Dispatch Interval is resolved for both MII and OCD
    pub price_confidence: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for TradingAverageprice301 {
    type PrimaryKey = TradingAverageprice301PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "TRADING".into(),
            table_name: Some("AVERAGEPRICE30".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> TradingAverageprice301PrimaryKey {
        TradingAverageprice301PrimaryKey {
            perioddate: self.perioddate,
            regionid: self.regionid.clone(),
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "trading_averageprice30_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct TradingAverageprice301PrimaryKey {
    pub perioddate: chrono::NaiveDateTime,
    pub regionid: String,
}
impl mmsdm_core::PrimaryKey for TradingAverageprice301PrimaryKey {}
impl mmsdm_core::CompareWithRow for TradingAverageprice301 {
    type Row = TradingAverageprice301;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.perioddate == row.perioddate && self.regionid == row.regionid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for TradingAverageprice301 {
    type PrimaryKey = TradingAverageprice301PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.perioddate == key.perioddate && self.regionid == key.regionid
    }
}
impl mmsdm_core::CompareWithRow for TradingAverageprice301PrimaryKey {
    type Row = TradingAverageprice301;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.perioddate == row.perioddate && self.regionid == row.regionid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for TradingAverageprice301PrimaryKey {
    type PrimaryKey = TradingAverageprice301PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.perioddate == key.perioddate && self.regionid == key.regionid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for TradingAverageprice301 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("perioddate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("rrp",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("price_confidence",
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
        let mut perioddate_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut rrp_array = Vec::new();
        let mut price_confidence_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            perioddate_array.push(row.perioddate.timestamp());
            regionid_array.push(row.regionid);
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            rrp_array
                .push({
                    row.rrp
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            price_confidence_array.push(row.price_confidence);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(perioddate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rrp_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(price_confidence_array)) as std::sync::Arc < dyn
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
/// ## TRADINGINTERCONNECT
///  _TRADINGINTERCONNECT shows the Interconnector flows for the 5 minutes Trading Interval.<br>Prior to 5 Minute Settlements, this was the average of the six 5 minute dispatch intervals within the 30 minute period.<br>_
///
/// * Data Set Name: Trading
/// * File Name: Interconnectorres
/// * Data Version: 2
///
/// # Description
///  TRADINGINTERCONNECT is public data, and is available to all participants. Source TRADINGINTERCONNECT is updated half hourly.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERCONNECTORID
/// * PERIODID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TradingInterconnectorres2 {
    /// Date that this data applies to
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Dispatch run no.
    pub runno: rust_decimal::Decimal,
    /// Interconnector identifier
    pub interconnectorid: String,
    /// Period number where 1 represents the trading interval ending at 00:05 AEST
    pub periodid: rust_decimal::Decimal,
    /// Average of the metered MW flow from the start of each dispatch interval.
    pub meteredmwflow: Option<rust_decimal::Decimal>,
    /// Calculated MW Flow from SPD
    pub mwflow: Option<rust_decimal::Decimal>,
    /// MW losses at calculated MW flow
    pub mwlosses: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for TradingInterconnectorres2 {
    type PrimaryKey = TradingInterconnectorres2PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "TRADING".into(),
            table_name: Some("INTERCONNECTORRES".into()),
            version: 2,
        }
    }
    fn primary_key(&self) -> TradingInterconnectorres2PrimaryKey {
        TradingInterconnectorres2PrimaryKey {
            interconnectorid: self.interconnectorid.clone(),
            periodid: self.periodid,
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "trading_interconnectorres_v2_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct TradingInterconnectorres2PrimaryKey {
    pub interconnectorid: String,
    pub periodid: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for TradingInterconnectorres2PrimaryKey {}
impl mmsdm_core::CompareWithRow for TradingInterconnectorres2 {
    type Row = TradingInterconnectorres2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interconnectorid == row.interconnectorid && self.periodid == row.periodid
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for TradingInterconnectorres2 {
    type PrimaryKey = TradingInterconnectorres2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid && self.periodid == key.periodid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for TradingInterconnectorres2PrimaryKey {
    type Row = TradingInterconnectorres2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interconnectorid == row.interconnectorid && self.periodid == row.periodid
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for TradingInterconnectorres2PrimaryKey {
    type PrimaryKey = TradingInterconnectorres2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid && self.periodid == key.periodid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for TradingInterconnectorres2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("runno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("meteredmwflow",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("mwflow",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("mwlosses",
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
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut meteredmwflow_array = Vec::new();
        let mut mwflow_array = Vec::new();
        let mut mwlosses_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            runno_array
                .push({
                    let mut val = row.runno;
                    val.rescale(0);
                    val.mantissa()
                });
            interconnectorid_array.push(row.interconnectorid);
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            meteredmwflow_array
                .push({
                    row.meteredmwflow
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            mwflow_array
                .push({
                    row.mwflow
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            mwlosses_array
                .push({
                    row.mwlosses
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(runno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(interconnectorid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(meteredmwflow_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(mwflow_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(mwlosses_array)
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
/// ## TRADINGPRICE
///  _TRADINGPRICE sets out 5 minutes spot market price, including fields to handle the Ancillary Services functionality. If prices are adjusted, the final price is recorded in the regional reference price (RRP) field with price before adjustment recorded in the regional original price (ROP) field.<br>Prior to 5 Minute Settlements, this was half-hourly spot market values, which was calculated as the average of the six 5 minute dispatch intervals within the 30 minute period._
///
/// * Data Set Name: Trading
/// * File Name: Price
/// * Data Version: 2
///
/// # Description
///  TRADINGPRICE data is public, so is available to all participants. Source TRADINGPRICE updates every 30 minutes. Notes INVALIDFLAG The INVALIDFLAG field is used to indicate whether the Trading interval price has been adjusted after the trading interval was completed. On a very restricted set of events, the market rules allow a dispatch price (5 min) to be adjusted on the next business day, and, when this occurs, the corresponding trading interval price for that region is also adjusted and marked as adjusted with INVALIDFLAG of 'A'. The INVALIDFLAG = 'Y' only applies to historical periods when not all six of the 5-minute dispatch intervals were run in the trading interval. System changes implemented on 30 September 2001 mean this situation no longer occurs since missing dispatch intervals are automatically populated from a previous interval. If the INVALIDFLAG field = '0', the price was not adjusted and all six dispatch intervals are present. Prices There is no field in the TRADINGPRICE table (or the MMS data model anywhere) telling you that the price is provisional or final. The only reliable method is to ensure that the trading date is at least 2 business days old.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * PERIODID
/// * REGIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TradingPrice2 {
    /// Date that this data applies to
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Run No
    pub runno: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Period number where 1 represents the trading interval ending at 00:05 AEST
    pub periodid: rust_decimal::Decimal,
    /// Regional reference price for this dispatch period
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess energy price where negative average
    pub eep: Option<rust_decimal::Decimal>,
    /// Indicates when the Trading interval price has been adjusted after the trading interval was completed
    pub invalidflag: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Regional Original Price. The price before any adjustments were made
    pub rop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raise6secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    pub raise6secrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raise60secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    pub raise60secrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raise5minrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    pub raise5minrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub raiseregrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    pub raiseregrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lower6secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    pub lower6secrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lower60secrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    pub lower60secrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lower5minrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    pub lower5minrop: Option<rust_decimal::Decimal>,
    /// Regional reference price for this dispatch period
    pub lowerregrrp: Option<rust_decimal::Decimal>,
    /// Original regional price - prior to APC or VoLL overrides applied
    pub lowerregrop: Option<rust_decimal::Decimal>,
    /// Status of regional prices for this dispatch interval "NOT FIRM" or "FIRM"
    pub price_status: Option<String>,
}
impl mmsdm_core::GetTable for TradingPrice2 {
    type PrimaryKey = TradingPrice2PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "TRADING".into(),
            table_name: Some("PRICE".into()),
            version: 2,
        }
    }
    fn primary_key(&self) -> TradingPrice2PrimaryKey {
        TradingPrice2PrimaryKey {
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.settlementdate.year(),
            month: num_traits::FromPrimitive::from_u32(self.settlementdate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "trading_price_v2_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct TradingPrice2PrimaryKey {
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for TradingPrice2PrimaryKey {}
impl mmsdm_core::CompareWithRow for TradingPrice2 {
    type Row = TradingPrice2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.periodid == row.periodid && self.regionid == row.regionid
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for TradingPrice2 {
    type PrimaryKey = TradingPrice2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid == key.regionid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for TradingPrice2PrimaryKey {
    type Row = TradingPrice2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.periodid == row.periodid && self.regionid == row.regionid
            && self.runno == row.runno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for TradingPrice2PrimaryKey {
    type PrimaryKey = TradingPrice2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.regionid == key.regionid
            && self.runno == key.runno && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for TradingPrice2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("runno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("rrp",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("eep",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("invalidflag",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("rop",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise6secrrp",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise6secrop",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60secrrp",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60secrop",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise5minrrp",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise5minrop",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raiseregrrp",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raiseregrop",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6secrrp",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6secrop",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60secrrp",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60secrop",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower5minrrp",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower5minrop",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lowerregrrp",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lowerregrop",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("price_status",
                arrow2::datatypes::DataType::LargeUtf8, true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut rrp_array = Vec::new();
        let mut eep_array = Vec::new();
        let mut invalidflag_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut rop_array = Vec::new();
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
        let mut price_status_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            runno_array
                .push({
                    let mut val = row.runno;
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
            rrp_array
                .push({
                    row.rrp
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            eep_array
                .push({
                    row.eep
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            invalidflag_array.push(row.invalidflag);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            rop_array
                .push({
                    row.rop
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise6secrrp_array
                .push({
                    row.raise6secrrp
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise6secrop_array
                .push({
                    row.raise6secrop
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60secrrp_array
                .push({
                    row.raise60secrrp
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60secrop_array
                .push({
                    row.raise60secrop
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise5minrrp_array
                .push({
                    row.raise5minrrp
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise5minrop_array
                .push({
                    row.raise5minrop
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raiseregrrp_array
                .push({
                    row.raiseregrrp
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raiseregrop_array
                .push({
                    row.raiseregrop
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6secrrp_array
                .push({
                    row.lower6secrrp
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6secrop_array
                .push({
                    row.lower6secrop
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60secrrp_array
                .push({
                    row.lower60secrrp
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60secrop_array
                .push({
                    row.lower60secrop
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower5minrrp_array
                .push({
                    row.lower5minrrp
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower5minrop_array
                .push({
                    row.lower5minrop
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lowerregrrp_array
                .push({
                    row.lowerregrrp
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lowerregrop_array
                .push({
                    row.lowerregrop
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            price_status_array.push(row.price_status);
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(runno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rrp_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(eep_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(invalidflag_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rop_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6secrrp_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6secrop_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60secrrp_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60secrop_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5minrrp_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5minrop_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raiseregrrp_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raiseregrop_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6secrrp_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6secrop_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60secrrp_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60secrop_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5minrrp_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5minrop_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowerregrrp_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowerregrop_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(price_status_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
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
        (Some("AVERAGEPRICE30"), version) if version <= 1_i32 => {
            let d: Vec<TradingAverageprice301> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertTradingAverageprice301 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("INTERCONNECTORRES"), version) if version <= 2_i32 => {
            let d: Vec<TradingInterconnectorres2> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertTradingInterconnectorres2 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("PRICE"), version) if version <= 2_i32 => {
            let d: Vec<TradingPrice2> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertTradingPrice2 @P1, @P2",
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
