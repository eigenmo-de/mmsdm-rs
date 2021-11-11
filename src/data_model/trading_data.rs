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
    #[serde(with = "crate::mms_datetime")]
    pub perioddate: chrono::NaiveDateTime,
    /// Region Identifier
    pub regionid: String,
    /// The 30-minute interval period, 1 to 48
    pub periodid: rust_decimal::Decimal,
    /// Regional reference price for this period
    pub rrp: Option<rust_decimal::Decimal>,
    /// Result of Manifestly Incorrect Inputs Price Status and OCD_Status - either "FIRM" or "NOT FIRM". Only FIRM if the Dispatch Interval is resolved for both MII and OCD
    pub price_confidence: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for TradingAverageprice301 {
    type PrimaryKey = TradingAverageprice301PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "TRADING".into(),
            table_name: Some("AVERAGEPRICE30".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> TradingAverageprice301PrimaryKey {
        TradingAverageprice301PrimaryKey {
            perioddate: self.perioddate.clone(),
            regionid: self.regionid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "trading_averageprice30_v1".to_string()
    }
}
impl crate::CompareWithRow for TradingAverageprice301 {
    type Row = TradingAverageprice301;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.perioddate == row.perioddate && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for TradingAverageprice301 {
    type PrimaryKey = TradingAverageprice301PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.perioddate == key.perioddate && self.regionid == key.regionid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TradingAverageprice301PrimaryKey {
    pub perioddate: chrono::NaiveDateTime,
    pub regionid: String,
}
impl crate::CompareWithRow for TradingAverageprice301PrimaryKey {
    type Row = TradingAverageprice301;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.perioddate == row.perioddate && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for TradingAverageprice301PrimaryKey {
    type PrimaryKey = TradingAverageprice301PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.perioddate == key.perioddate && self.regionid == key.regionid
    }
}
impl crate::PrimaryKey for TradingAverageprice301PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for TradingAverageprice301 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("perioddate", arrow2::datatypes::DataType::Date32, false),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("rrp", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new(
                "price_confidence",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut perioddate_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut rrp_array = Vec::new();
        let mut price_confidence_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            perioddate_array.push(
                i32::try_from(
                    (row.perioddate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            regionid_array.push(row.regionid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            rrp_array.push({
                row.rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            price_confidence_array.push(row.price_confidence);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(perioddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    price_confidence_array,
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
    #[serde(with = "crate::mms_datetime")]
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
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for TradingInterconnectorres2 {
    type PrimaryKey = TradingInterconnectorres2PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "TRADING".into(),
            table_name: Some("INTERCONNECTORRES".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> TradingInterconnectorres2PrimaryKey {
        TradingInterconnectorres2PrimaryKey {
            interconnectorid: self.interconnectorid.clone(),
            periodid: self.periodid.clone(),
            runno: self.runno.clone(),
            settlementdate: self.settlementdate.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "trading_interconnectorres_v2_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for TradingInterconnectorres2 {
    type Row = TradingInterconnectorres2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interconnectorid == row.interconnectorid
            && self.periodid == row.periodid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for TradingInterconnectorres2 {
    type PrimaryKey = TradingInterconnectorres2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
            && self.periodid == key.periodid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TradingInterconnectorres2PrimaryKey {
    pub interconnectorid: String,
    pub periodid: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for TradingInterconnectorres2PrimaryKey {
    type Row = TradingInterconnectorres2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interconnectorid == row.interconnectorid
            && self.periodid == row.periodid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for TradingInterconnectorres2PrimaryKey {
    type PrimaryKey = TradingInterconnectorres2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
            && self.periodid == key.periodid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for TradingInterconnectorres2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for TradingInterconnectorres2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
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
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut meteredmwflow_array = Vec::new();
        let mut mwflow_array = Vec::new();
        let mut mwlosses_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(
                i32::try_from(
                    (row.settlementdate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
            interconnectorid_array.push(row.interconnectorid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
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
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
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
    #[serde(with = "crate::mms_datetime")]
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
    #[serde(with = "crate::mms_datetime_opt")]
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
impl crate::GetTable for TradingPrice2 {
    type PrimaryKey = TradingPrice2PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "TRADING".into(),
            table_name: Some("PRICE".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> TradingPrice2PrimaryKey {
        TradingPrice2PrimaryKey {
            periodid: self.periodid.clone(),
            regionid: self.regionid.clone(),
            runno: self.runno.clone(),
            settlementdate: self.settlementdate.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        (
            chrono::Datelike::year(&self.settlementdate),
            num_traits::FromPrimitive::from_u32(chrono::Datelike::month(&self.settlementdate))
                .unwrap(),
        )
    }

    fn partition_name(&self) -> String {
        format!(
            "trading_price_v2_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for TradingPrice2 {
    type Row = TradingPrice2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for TradingPrice2 {
    type PrimaryKey = TradingPrice2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TradingPrice2PrimaryKey {
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for TradingPrice2PrimaryKey {
    type Row = TradingPrice2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for TradingPrice2PrimaryKey {
    type PrimaryKey = TradingPrice2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for TradingPrice2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for TradingPrice2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("rrp", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new("eep", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new(
                "invalidflag",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("rop", arrow2::datatypes::DataType::Decimal(15, 5), true),
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
                "price_status",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
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
        for (_, row) in partition {
            settlementdate_array.push(
                i32::try_from(
                    (row.settlementdate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
            regionid_array.push(row.regionid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            rrp_array.push({
                row.rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            eep_array.push({
                row.eep.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            invalidflag_array.push(row.invalidflag);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            rop_array.push({
                row.rop.map(|mut val| {
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
            price_status_array.push(row.price_status);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(eep_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(invalidflag_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rop_array)
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
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(price_status_array)),
            ],
        )
        .map_err(Into::into)
    }
}
