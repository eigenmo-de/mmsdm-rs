/// # Summary
///
/// ## DEMANDOPERATIONALACTUAL
///  _Shows Actual Operational Demand for a particular date time interval._
///
/// * Data Set Name: Operational Demand
/// * File Name: Actual
/// * Data Version: 3
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OperationalDemandActual3 {
    /// Date time interval for operational demand value
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region identifier
    pub regionid: String,
    /// Average 30-minute measured operational demand MW value (unadjusted)
    pub operational_demand: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Adjustment value containing the estimated amount of activated RERT and involuntary load shedding that occurred as a result of a NER 4.8.9 instruction for load shedding from AEMO.
    pub operational_demand_adjustment: Option<rust_decimal::Decimal>,
    /// Estimated average 30-minute MW amount of Wholesale Demand Response that occurred
    pub wdr_estimate: Option<i64>,
}
impl crate::GetTable for OperationalDemandActual3 {
    type PrimaryKey = OperationalDemandActual3PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OPERATIONAL_DEMAND".into(),
            table_name: Some("ACTUAL".into()),
            version: 3,
        }
    }

    fn primary_key(&self) -> OperationalDemandActual3PrimaryKey {
        OperationalDemandActual3PrimaryKey {
            interval_datetime: self.interval_datetime,
            regionid: self.regionid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "operational_demand_actual_v3".to_string()
    }
}
impl crate::CompareWithRow for OperationalDemandActual3 {
    type Row = OperationalDemandActual3;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interval_datetime == row.interval_datetime && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for OperationalDemandActual3 {
    type PrimaryKey = OperationalDemandActual3PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime && self.regionid == key.regionid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct OperationalDemandActual3PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: String,
}
impl crate::CompareWithRow for OperationalDemandActual3PrimaryKey {
    type Row = OperationalDemandActual3;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interval_datetime == row.interval_datetime && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for OperationalDemandActual3PrimaryKey {
    type PrimaryKey = OperationalDemandActual3PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime && self.regionid == key.regionid
    }
}
impl crate::PrimaryKey for OperationalDemandActual3PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for OperationalDemandActual3 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "interval_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "operational_demand",
                arrow2::datatypes::DataType::Decimal(10, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "operational_demand_adjustment",
                arrow2::datatypes::DataType::Decimal(10, 0),
                true,
            ),
            arrow2::datatypes::Field::new("wdr_estimate", arrow2::datatypes::DataType::Int64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut interval_datetime_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut operational_demand_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut operational_demand_adjustment_array = Vec::new();
        let mut wdr_estimate_array = Vec::new();
        for (_, row) in partition {
            interval_datetime_array.push(row.interval_datetime.timestamp_millis());
            regionid_array.push(row.regionid);
            operational_demand_array.push({
                row.operational_demand.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            operational_demand_adjustment_array.push({
                row.operational_demand_adjustment.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            wdr_estimate_array.push(row.wdr_estimate);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(interval_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(operational_demand_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(operational_demand_adjustment_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(wdr_estimate_array)),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## DEMANDOPERATIONALFORECAST
///  _Shows Forecast Operational Demand for a particular date time interval._
///
/// * Data Set Name: Operational Demand
/// * File Name: Forecast
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OperationalDemandForecast1 {
    /// Forecast for a particular date time interval
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Region identifier
    pub regionid: String,
    /// Date time this forecast was produced
    #[serde(with = "crate::mms_datetime_opt")]
    pub load_date: Option<chrono::NaiveDateTime>,
    /// 10% probability of exceedance operational demand forecast value
    pub operational_demand_poe10: Option<rust_decimal::Decimal>,
    /// 50% probability of exceedance operational demand forecast value
    pub operational_demand_poe50: Option<rust_decimal::Decimal>,
    /// 90% probability of exceedance operational demand forecast value
    pub operational_demand_poe90: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for OperationalDemandForecast1 {
    type PrimaryKey = OperationalDemandForecast1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "OPERATIONAL_DEMAND".into(),
            table_name: Some("FORECAST".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> OperationalDemandForecast1PrimaryKey {
        OperationalDemandForecast1PrimaryKey {
            interval_datetime: self.interval_datetime,
            regionid: self.regionid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "operational_demand_forecast_v1".to_string()
    }
}
impl crate::CompareWithRow for OperationalDemandForecast1 {
    type Row = OperationalDemandForecast1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interval_datetime == row.interval_datetime && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for OperationalDemandForecast1 {
    type PrimaryKey = OperationalDemandForecast1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime && self.regionid == key.regionid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct OperationalDemandForecast1PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: String,
}
impl crate::CompareWithRow for OperationalDemandForecast1PrimaryKey {
    type Row = OperationalDemandForecast1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interval_datetime == row.interval_datetime && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for OperationalDemandForecast1PrimaryKey {
    type PrimaryKey = OperationalDemandForecast1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime && self.regionid == key.regionid
    }
}
impl crate::PrimaryKey for OperationalDemandForecast1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for OperationalDemandForecast1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "interval_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("load_date", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "operational_demand_poe10",
                arrow2::datatypes::DataType::Decimal(15, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "operational_demand_poe50",
                arrow2::datatypes::DataType::Decimal(15, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "operational_demand_poe90",
                arrow2::datatypes::DataType::Decimal(15, 2),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut interval_datetime_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut load_date_array = Vec::new();
        let mut operational_demand_poe10_array = Vec::new();
        let mut operational_demand_poe50_array = Vec::new();
        let mut operational_demand_poe90_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            interval_datetime_array.push(row.interval_datetime.timestamp_millis());
            regionid_array.push(row.regionid);
            load_date_array.push(row.load_date.map(|val| val.timestamp_millis()));
            operational_demand_poe10_array.push({
                row.operational_demand_poe10.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            operational_demand_poe50_array.push({
                row.operational_demand_poe50.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            operational_demand_poe90_array.push({
                row.operational_demand_poe90.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(interval_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(load_date_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(operational_demand_poe10_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(operational_demand_poe50_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(operational_demand_poe90_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## INTERMITTENT_CLUSTER_AVAIL
///  _A submission of expected plant availability for an intermittent generating unit cluster, by Trading Day and Trading Interval._
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Cluster Avail
/// * Data Version: 2
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * CLUSTERID
/// * DUID
/// * OFFERDATETIME
/// * PERIODID
/// * TRADINGDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentClusterAvail2 {
    /// The trading day to which the availability submission applies
    #[serde(with = "crate::mms_datetime")]
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: String,
    /// Date and Time when this cluster availability submission was loaded
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// Unique Cluster Identifier for this cluster within the DUID
    pub clusterid: String,
    /// Trading interval number (1…48) within this TRADINGDATE for which ELEMENTS_UNAVAILABLE applies
    pub periodid: rust_decimal::Decimal,
    /// Number of elements within this CLUSTERID (turbines for wind, or inverters for solar) that are not available for this TRADINGDATE and PERIODID (scheduled maintenance in AWEFS/ASEFS). Value between 0 and the registered Number of Cluster Elements.Value = 0 means no elements unavailable
    pub elements_unavailable: Option<rust_decimal::Decimal>,
    /// Number of elements within this CLUSTERID (turbines for wind, or inverters for solar) that are available for this TRADINGDATE and PERIODID (scheduled maintenance in AWEFS/ASEFS). Value between 0 and the registered Number of Cluster Elements. Value = 0 means no elements available
    pub elements_available: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DemandIntermittentClusterAvail2 {
    type PrimaryKey = DemandIntermittentClusterAvail2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("INTERMITTENT_CLUSTER_AVAIL".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> DemandIntermittentClusterAvail2PrimaryKey {
        DemandIntermittentClusterAvail2PrimaryKey {
            clusterid: self.clusterid.clone(),
            duid: self.duid.clone(),
            offerdatetime: self.offerdatetime,
            periodid: self.periodid,
            tradingdate: self.tradingdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "demand_intermittent_cluster_avail_v2".to_string()
    }
}
impl crate::CompareWithRow for DemandIntermittentClusterAvail2 {
    type Row = DemandIntermittentClusterAvail2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.clusterid == row.clusterid
            && self.duid == row.duid
            && self.offerdatetime == row.offerdatetime
            && self.periodid == row.periodid
            && self.tradingdate == row.tradingdate
    }
}
impl crate::CompareWithPrimaryKey for DemandIntermittentClusterAvail2 {
    type PrimaryKey = DemandIntermittentClusterAvail2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.clusterid == key.clusterid
            && self.duid == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.periodid == key.periodid
            && self.tradingdate == key.tradingdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct DemandIntermittentClusterAvail2PrimaryKey {
    pub clusterid: String,
    pub duid: String,
    pub offerdatetime: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub tradingdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DemandIntermittentClusterAvail2PrimaryKey {
    type Row = DemandIntermittentClusterAvail2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.clusterid == row.clusterid
            && self.duid == row.duid
            && self.offerdatetime == row.offerdatetime
            && self.periodid == row.periodid
            && self.tradingdate == row.tradingdate
    }
}
impl crate::CompareWithPrimaryKey for DemandIntermittentClusterAvail2PrimaryKey {
    type PrimaryKey = DemandIntermittentClusterAvail2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.clusterid == key.clusterid
            && self.duid == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.periodid == key.periodid
            && self.tradingdate == key.tradingdate
    }
}
impl crate::PrimaryKey for DemandIntermittentClusterAvail2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DemandIntermittentClusterAvail2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "tradingdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "offerdatetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "clusterid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "elements_unavailable",
                arrow2::datatypes::DataType::Decimal(5, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "elements_available",
                arrow2::datatypes::DataType::Decimal(5, 0),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut tradingdate_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut offerdatetime_array = Vec::new();
        let mut clusterid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut elements_unavailable_array = Vec::new();
        let mut elements_available_array = Vec::new();
        for (_, row) in partition {
            tradingdate_array.push(row.tradingdate.timestamp_millis());
            duid_array.push(row.duid);
            offerdatetime_array.push(row.offerdatetime.timestamp_millis());
            clusterid_array.push(row.clusterid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            elements_unavailable_array.push({
                row.elements_unavailable.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            elements_available_array.push({
                row.elements_available.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(tradingdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdatetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(clusterid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(elements_unavailable_array)
                        .to(arrow2::datatypes::DataType::Decimal(5, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(elements_available_array)
                        .to(arrow2::datatypes::DataType::Decimal(5, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## INTERMITTENT_CLUSTER_AVAIL_DAY
///  _Summary record for an availability submission for an intermittent generating unit cluster for a Trading Day._
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Cluster Avail Day
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * CLUSTERID
/// * DUID
/// * OFFERDATETIME
/// * TRADINGDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentClusterAvailDay1 {
    /// Trading Day for which this cluster availability submission applies
    #[serde(with = "crate::mms_datetime")]
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: String,
    /// Date and Time when this cluster availability submission was loaded
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// Unique Cluster Identifier for this cluster within the DUID
    pub clusterid: String,
}
impl crate::GetTable for DemandIntermittentClusterAvailDay1 {
    type PrimaryKey = DemandIntermittentClusterAvailDay1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("INTERMITTENT_CLUSTER_AVAIL_DAY".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DemandIntermittentClusterAvailDay1PrimaryKey {
        DemandIntermittentClusterAvailDay1PrimaryKey {
            clusterid: self.clusterid.clone(),
            duid: self.duid.clone(),
            offerdatetime: self.offerdatetime,
            tradingdate: self.tradingdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "demand_intermittent_cluster_avail_day_v1".to_string()
    }
}
impl crate::CompareWithRow for DemandIntermittentClusterAvailDay1 {
    type Row = DemandIntermittentClusterAvailDay1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.clusterid == row.clusterid
            && self.duid == row.duid
            && self.offerdatetime == row.offerdatetime
            && self.tradingdate == row.tradingdate
    }
}
impl crate::CompareWithPrimaryKey for DemandIntermittentClusterAvailDay1 {
    type PrimaryKey = DemandIntermittentClusterAvailDay1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.clusterid == key.clusterid
            && self.duid == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.tradingdate == key.tradingdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct DemandIntermittentClusterAvailDay1PrimaryKey {
    pub clusterid: String,
    pub duid: String,
    pub offerdatetime: chrono::NaiveDateTime,
    pub tradingdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DemandIntermittentClusterAvailDay1PrimaryKey {
    type Row = DemandIntermittentClusterAvailDay1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.clusterid == row.clusterid
            && self.duid == row.duid
            && self.offerdatetime == row.offerdatetime
            && self.tradingdate == row.tradingdate
    }
}
impl crate::CompareWithPrimaryKey for DemandIntermittentClusterAvailDay1PrimaryKey {
    type PrimaryKey = DemandIntermittentClusterAvailDay1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.clusterid == key.clusterid
            && self.duid == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.tradingdate == key.tradingdate
    }
}
impl crate::PrimaryKey for DemandIntermittentClusterAvailDay1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DemandIntermittentClusterAvailDay1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "tradingdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "offerdatetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "clusterid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut tradingdate_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut offerdatetime_array = Vec::new();
        let mut clusterid_array = Vec::new();
        for (_, row) in partition {
            tradingdate_array.push(row.tradingdate.timestamp_millis());
            duid_array.push(row.duid);
            offerdatetime_array.push(row.offerdatetime.timestamp_millis());
            clusterid_array.push(row.clusterid);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(tradingdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdatetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(clusterid_array)),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## INTERMITTENT_DS_PRED
///  _Unconstrained Intermittent Generation Forecasts (UIGF) for Dispatch_
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Ds Pred
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * DUID
/// * FORECAST_PRIORITY
/// * INTERVAL_DATETIME
/// * OFFERDATETIME
/// * ORIGIN
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentDsPred1 {
    /// Date and Time when the forecast applies (dispatch interval ending)<br>
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// DUID (or Area for non-scheduled) where this forecast applies
    pub duid: String,
    /// Date and Time when this forecast submission was loaded
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// Date and Time when the forecast applies (dispatch interval ending)
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Origin of this forecast (PARTICIPANTID, AWEFS/ASEFS, or another vendor)
    pub origin: String,
    /// Unsuppressed forecasts with higher priority values are used in Dispatch in preference to unsuppressed forecasts with lower priority values<br>
    pub forecast_priority: rust_decimal::Decimal,
    /// Forecast MW value for this interval_DateTime
    pub forecast_mean: Option<rust_decimal::Decimal>,
    /// Forecast 10% POE MW value for this interval_DateTime
    pub forecast_poe10: Option<rust_decimal::Decimal>,
    /// Forecast 50% POE MW value for this interval_DateTime. Used in Dispatch.
    pub forecast_poe50: Option<rust_decimal::Decimal>,
    /// Forecast 90% POE MW value for this interval_DateTime
    pub forecast_poe90: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DemandIntermittentDsPred1 {
    type PrimaryKey = DemandIntermittentDsPred1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("INTERMITTENT_DS_PRED".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DemandIntermittentDsPred1PrimaryKey {
        DemandIntermittentDsPred1PrimaryKey {
            duid: self.duid.clone(),
            forecast_priority: self.forecast_priority,
            interval_datetime: self.interval_datetime,
            offerdatetime: self.offerdatetime,
            origin: self.origin.clone(),
            run_datetime: self.run_datetime,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "demand_intermittent_ds_pred_v1".to_string()
    }
}
impl crate::CompareWithRow for DemandIntermittentDsPred1 {
    type Row = DemandIntermittentDsPred1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.forecast_priority == row.forecast_priority
            && self.interval_datetime == row.interval_datetime
            && self.offerdatetime == row.offerdatetime
            && self.origin == row.origin
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for DemandIntermittentDsPred1 {
    type PrimaryKey = DemandIntermittentDsPred1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.forecast_priority == key.forecast_priority
            && self.interval_datetime == key.interval_datetime
            && self.offerdatetime == key.offerdatetime
            && self.origin == key.origin
            && self.run_datetime == key.run_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct DemandIntermittentDsPred1PrimaryKey {
    pub duid: String,
    pub forecast_priority: rust_decimal::Decimal,
    pub interval_datetime: chrono::NaiveDateTime,
    pub offerdatetime: chrono::NaiveDateTime,
    pub origin: String,
    pub run_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DemandIntermittentDsPred1PrimaryKey {
    type Row = DemandIntermittentDsPred1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.forecast_priority == row.forecast_priority
            && self.interval_datetime == row.interval_datetime
            && self.offerdatetime == row.offerdatetime
            && self.origin == row.origin
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for DemandIntermittentDsPred1PrimaryKey {
    type PrimaryKey = DemandIntermittentDsPred1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.forecast_priority == key.forecast_priority
            && self.interval_datetime == key.interval_datetime
            && self.offerdatetime == key.offerdatetime
            && self.origin == key.origin
            && self.run_datetime == key.run_datetime
    }
}
impl crate::PrimaryKey for DemandIntermittentDsPred1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DemandIntermittentDsPred1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "run_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "offerdatetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interval_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("origin", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "forecast_priority",
                arrow2::datatypes::DataType::Decimal(10, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "forecast_mean",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "forecast_poe10",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "forecast_poe50",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "forecast_poe90",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut run_datetime_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut offerdatetime_array = Vec::new();
        let mut interval_datetime_array = Vec::new();
        let mut origin_array = Vec::new();
        let mut forecast_priority_array = Vec::new();
        let mut forecast_mean_array = Vec::new();
        let mut forecast_poe10_array = Vec::new();
        let mut forecast_poe50_array = Vec::new();
        let mut forecast_poe90_array = Vec::new();
        for (_, row) in partition {
            run_datetime_array.push(row.run_datetime.timestamp_millis());
            duid_array.push(row.duid);
            offerdatetime_array.push(row.offerdatetime.timestamp_millis());
            interval_datetime_array.push(row.interval_datetime.timestamp_millis());
            origin_array.push(row.origin);
            forecast_priority_array.push({
                let mut val = row.forecast_priority;
                val.rescale(0);
                val.mantissa()
            });
            forecast_mean_array.push({
                row.forecast_mean.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            forecast_poe10_array.push({
                row.forecast_poe10.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            forecast_poe50_array.push({
                row.forecast_poe50.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            forecast_poe90_array.push({
                row.forecast_poe90.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(run_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdatetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(interval_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(origin_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(forecast_priority_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(forecast_mean_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(forecast_poe10_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(forecast_poe50_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(forecast_poe90_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## INTERMITTENT_DS_RUN
///  _Unconstrained Intermittent Generation Forecasts (UIGF) for Dispatch._
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Ds Run
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * DUID
/// * FORECAST_PRIORITY
/// * OFFERDATETIME
/// * ORIGIN
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentDsRun1 {
    /// Date and Time where the forecast applies (dispatch interval ending)
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// DUID (or Area for non-scheduled) where this forecast applies
    pub duid: String,
    /// Date and Time when this forecast submission was loaded.
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// Origin of this forecast (PARTICIPANTID, AWEFS/ASEFS, or another vendor)
    pub origin: String,
    /// Unsuppressed forecasts with higher priority values are used in Dispatch in preference to unsuppressed forecasts with lower priority values.
    pub forecast_priority: rust_decimal::Decimal,
    /// Authorising officer of this forecast (applicable for participant forecasts only). This column is not made available to the public.
    pub authorisedby: Option<String>,
    /// Comments relating to the forecast. This column is not made available to the public.
    pub comments: Option<String>,
    /// Last date and time the record changed.
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Metadata relating to the forecast. This column is not made available to the public.
    pub model: Option<String>,
    /// Participant can document when the forecast was created
    #[serde(with = "crate::mms_datetime_opt")]
    pub participant_timestamp: Option<chrono::NaiveDateTime>,
    /// Was this forecast suppressed by AEMO? Suppressed = 1,Not suppressed =0<br>
    pub suppressed_aemo: Option<rust_decimal::Decimal>,
    /// Was this forecast suppressed by the participant? Suppressed submissions may not be used,  Suppressed = 1, Not suppressed =0<br>
    pub suppressed_participant: Option<rust_decimal::Decimal>,
    /// Uniquely identifies this interaction
    pub transaction_id: Option<String>,
}
impl crate::GetTable for DemandIntermittentDsRun1 {
    type PrimaryKey = DemandIntermittentDsRun1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("INTERMITTENT_DS_RUN".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DemandIntermittentDsRun1PrimaryKey {
        DemandIntermittentDsRun1PrimaryKey {
            duid: self.duid.clone(),
            forecast_priority: self.forecast_priority,
            offerdatetime: self.offerdatetime,
            origin: self.origin.clone(),
            run_datetime: self.run_datetime,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "demand_intermittent_ds_run_v1".to_string()
    }
}
impl crate::CompareWithRow for DemandIntermittentDsRun1 {
    type Row = DemandIntermittentDsRun1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.forecast_priority == row.forecast_priority
            && self.offerdatetime == row.offerdatetime
            && self.origin == row.origin
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for DemandIntermittentDsRun1 {
    type PrimaryKey = DemandIntermittentDsRun1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.forecast_priority == key.forecast_priority
            && self.offerdatetime == key.offerdatetime
            && self.origin == key.origin
            && self.run_datetime == key.run_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct DemandIntermittentDsRun1PrimaryKey {
    pub duid: String,
    pub forecast_priority: rust_decimal::Decimal,
    pub offerdatetime: chrono::NaiveDateTime,
    pub origin: String,
    pub run_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DemandIntermittentDsRun1PrimaryKey {
    type Row = DemandIntermittentDsRun1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.forecast_priority == row.forecast_priority
            && self.offerdatetime == row.offerdatetime
            && self.origin == row.origin
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for DemandIntermittentDsRun1PrimaryKey {
    type PrimaryKey = DemandIntermittentDsRun1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.forecast_priority == key.forecast_priority
            && self.offerdatetime == key.offerdatetime
            && self.origin == key.origin
            && self.run_datetime == key.run_datetime
    }
}
impl crate::PrimaryKey for DemandIntermittentDsRun1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DemandIntermittentDsRun1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "run_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "offerdatetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("origin", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "forecast_priority",
                arrow2::datatypes::DataType::Decimal(10, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("comments", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("model", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "participant_timestamp",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "suppressed_aemo",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "suppressed_participant",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "transaction_id",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut run_datetime_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut offerdatetime_array = Vec::new();
        let mut origin_array = Vec::new();
        let mut forecast_priority_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut comments_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut model_array = Vec::new();
        let mut participant_timestamp_array = Vec::new();
        let mut suppressed_aemo_array = Vec::new();
        let mut suppressed_participant_array = Vec::new();
        let mut transaction_id_array = Vec::new();
        for (_, row) in partition {
            run_datetime_array.push(row.run_datetime.timestamp_millis());
            duid_array.push(row.duid);
            offerdatetime_array.push(row.offerdatetime.timestamp_millis());
            origin_array.push(row.origin);
            forecast_priority_array.push({
                let mut val = row.forecast_priority;
                val.rescale(0);
                val.mantissa()
            });
            authorisedby_array.push(row.authorisedby);
            comments_array.push(row.comments);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            model_array.push(row.model);
            participant_timestamp_array
                .push(row.participant_timestamp.map(|val| val.timestamp_millis()));
            suppressed_aemo_array.push({
                row.suppressed_aemo.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            suppressed_participant_array.push({
                row.suppressed_participant.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            transaction_id_array.push(row.transaction_id);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(run_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdatetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(origin_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(forecast_priority_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(comments_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(model_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(participant_timestamp_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(suppressed_aemo_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(suppressed_participant_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(transaction_id_array)),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## INTERMITTENT_GEN_FCST
///  _Identifying record for a given forecast of an intermittent generation. This table is the version table for the INTERMITTENT_GEN_FCST_DATA table which stores the individual forecast values_
///
/// * Data Set Name: Forecast
/// * File Name: Intermittent Gen
/// * Data Version: 1
///
/// # Description
///  Source &nbsp; INTERMITTENT_GEN_FCST_DATA updates every 30 minutes when AEMO issues a new 30-minute forecast of intermittent generation out to 8 days ahead. Volume ~18,000 rows per generator per year
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * DUID
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForecastIntermittentGen1 {
    /// Date Time of forecast (AEST).
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Identifier of the intermittent generator.
    pub duid: String,
    /// Date Time (AEST) of the first half-hour interval being forecast.
    #[serde(with = "crate::mms_datetime")]
    pub start_interval_datetime: chrono::NaiveDateTime,
    /// Date Time (AEST) of the final half-hour interval being forecast.
    #[serde(with = "crate::mms_datetime")]
    pub end_interval_datetime: chrono::NaiveDateTime,
    /// Versioning information for resolution back to AEMO's wind generation forecasting system.
    pub versionno: Option<rust_decimal::Decimal>,
    /// Date Time record was created
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ForecastIntermittentGen1 {
    type PrimaryKey = ForecastIntermittentGen1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORECAST".into(),
            table_name: Some("INTERMITTENT_GEN".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ForecastIntermittentGen1PrimaryKey {
        ForecastIntermittentGen1PrimaryKey {
            duid: self.duid.clone(),
            run_datetime: self.run_datetime,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "forecast_intermittent_gen_v1".to_string()
    }
}
impl crate::CompareWithRow for ForecastIntermittentGen1 {
    type Row = ForecastIntermittentGen1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for ForecastIntermittentGen1 {
    type PrimaryKey = ForecastIntermittentGen1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.run_datetime == key.run_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ForecastIntermittentGen1PrimaryKey {
    pub duid: String,
    pub run_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for ForecastIntermittentGen1PrimaryKey {
    type Row = ForecastIntermittentGen1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for ForecastIntermittentGen1PrimaryKey {
    type PrimaryKey = ForecastIntermittentGen1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid && self.run_datetime == key.run_datetime
    }
}
impl crate::PrimaryKey for ForecastIntermittentGen1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ForecastIntermittentGen1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "run_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "start_interval_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "end_interval_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(10, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut run_datetime_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut start_interval_datetime_array = Vec::new();
        let mut end_interval_datetime_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            run_datetime_array.push(row.run_datetime.timestamp_millis());
            duid_array.push(row.duid);
            start_interval_datetime_array.push(row.start_interval_datetime.timestamp_millis());
            end_interval_datetime_array.push(row.end_interval_datetime.timestamp_millis());
            versionno_array.push({
                row.versionno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(run_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(start_interval_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(end_interval_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## INTERMITTENT_GEN_FCST_DATA
///  _Stores the forecast generation (MW) for each interval within a given forecast of an intermittent generator._
///
/// * Data Set Name: Forecast
/// * File Name: Intermittent Gen Data
/// * Data Version: 1
///
/// # Description
///  Source INTERMITTENT_GEN_FCST_DATA updates every 30 minutes when AEMO issues a new 30-minute forecast of wind generation out to 40 hours ahead. Volume ~1,500,000 rows per generator per year
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * DUID
/// * INTERVAL_DATETIME
/// * RUN_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForecastIntermittentGenData1 {
    /// Date Time of forecast (AEST).
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Identifier of the intermittent generator
    pub duid: String,
    /// Date Time (AEST) of the halfhour interval being forecast
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// The average forecast value in MW at the interval end
    pub powermean: Option<rust_decimal::Decimal>,
    /// 50% probability of exceedance forecast value in MW at the interval end
    pub powerpoe50: Option<rust_decimal::Decimal>,
    /// 10% probability of exceedance forecast value in MW at the interval end
    pub powerpoelow: Option<rust_decimal::Decimal>,
    /// 90% probability of exceedance forecast value in MW at the interval end
    pub powerpoehigh: Option<rust_decimal::Decimal>,
    /// Date Time record was created
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ForecastIntermittentGenData1 {
    type PrimaryKey = ForecastIntermittentGenData1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORECAST".into(),
            table_name: Some("INTERMITTENT_GEN_DATA".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ForecastIntermittentGenData1PrimaryKey {
        ForecastIntermittentGenData1PrimaryKey {
            duid: self.duid.clone(),
            interval_datetime: self.interval_datetime,
            run_datetime: self.run_datetime,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "forecast_intermittent_gen_data_v1".to_string()
    }
}
impl crate::CompareWithRow for ForecastIntermittentGenData1 {
    type Row = ForecastIntermittentGenData1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for ForecastIntermittentGenData1 {
    type PrimaryKey = ForecastIntermittentGenData1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ForecastIntermittentGenData1PrimaryKey {
    pub duid: String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub run_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for ForecastIntermittentGenData1PrimaryKey {
    type Row = ForecastIntermittentGenData1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.interval_datetime == row.interval_datetime
            && self.run_datetime == row.run_datetime
    }
}
impl crate::CompareWithPrimaryKey for ForecastIntermittentGenData1PrimaryKey {
    type PrimaryKey = ForecastIntermittentGenData1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.interval_datetime == key.interval_datetime
            && self.run_datetime == key.run_datetime
    }
}
impl crate::PrimaryKey for ForecastIntermittentGenData1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ForecastIntermittentGenData1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "run_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "interval_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "powermean",
                arrow2::datatypes::DataType::Decimal(9, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "powerpoe50",
                arrow2::datatypes::DataType::Decimal(9, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "powerpoelow",
                arrow2::datatypes::DataType::Decimal(9, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "powerpoehigh",
                arrow2::datatypes::DataType::Decimal(9, 3),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut run_datetime_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut interval_datetime_array = Vec::new();
        let mut powermean_array = Vec::new();
        let mut powerpoe50_array = Vec::new();
        let mut powerpoelow_array = Vec::new();
        let mut powerpoehigh_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            run_datetime_array.push(row.run_datetime.timestamp_millis());
            duid_array.push(row.duid);
            interval_datetime_array.push(row.interval_datetime.timestamp_millis());
            powermean_array.push({
                row.powermean.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            powerpoe50_array.push({
                row.powerpoe50.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            powerpoelow_array.push({
                row.powerpoelow.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            powerpoehigh_array.push({
                row.powerpoehigh.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(run_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(interval_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(powermean_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(powerpoe50_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(powerpoelow_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(powerpoehigh_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## INTERMITTENT_GEN_LIMIT
///  _A submission of Upper MW Limit for an intermittent generating unit, by Trading Day and Trading Interval_
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Gen Limit
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * DUID
/// * OFFERDATETIME
/// * PERIODID
/// * TRADINGDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentGenLimit1 {
    /// Trading Day for which this unit availability submission applies
    #[serde(with = "crate::mms_datetime")]
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: String,
    /// Date and Time when this unit availability submission was loaded
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// Trading interval number (1...48) within this TRADINGDATE for which UPPERMWLIMIT applies
    pub periodid: rust_decimal::Decimal,
    /// Maximum imposed MW limit (down regulation in AWEFS/ASEFS). Value between 0 and the registered DUID Maximum Capacity. Value = -1 means no limit applies.
    pub uppermwlimit: Option<i64>,
}
impl crate::GetTable for DemandIntermittentGenLimit1 {
    type PrimaryKey = DemandIntermittentGenLimit1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("INTERMITTENT_GEN_LIMIT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DemandIntermittentGenLimit1PrimaryKey {
        DemandIntermittentGenLimit1PrimaryKey {
            duid: self.duid.clone(),
            offerdatetime: self.offerdatetime,
            periodid: self.periodid,
            tradingdate: self.tradingdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "demand_intermittent_gen_limit_v1".to_string()
    }
}
impl crate::CompareWithRow for DemandIntermittentGenLimit1 {
    type Row = DemandIntermittentGenLimit1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.offerdatetime == row.offerdatetime
            && self.periodid == row.periodid
            && self.tradingdate == row.tradingdate
    }
}
impl crate::CompareWithPrimaryKey for DemandIntermittentGenLimit1 {
    type PrimaryKey = DemandIntermittentGenLimit1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.periodid == key.periodid
            && self.tradingdate == key.tradingdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct DemandIntermittentGenLimit1PrimaryKey {
    pub duid: String,
    pub offerdatetime: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub tradingdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DemandIntermittentGenLimit1PrimaryKey {
    type Row = DemandIntermittentGenLimit1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.offerdatetime == row.offerdatetime
            && self.periodid == row.periodid
            && self.tradingdate == row.tradingdate
    }
}
impl crate::CompareWithPrimaryKey for DemandIntermittentGenLimit1PrimaryKey {
    type PrimaryKey = DemandIntermittentGenLimit1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.periodid == key.periodid
            && self.tradingdate == key.tradingdate
    }
}
impl crate::PrimaryKey for DemandIntermittentGenLimit1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DemandIntermittentGenLimit1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "tradingdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "offerdatetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("uppermwlimit", arrow2::datatypes::DataType::Int64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut tradingdate_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut offerdatetime_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut uppermwlimit_array = Vec::new();
        for (_, row) in partition {
            tradingdate_array.push(row.tradingdate.timestamp_millis());
            duid_array.push(row.duid);
            offerdatetime_array.push(row.offerdatetime.timestamp_millis());
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            uppermwlimit_array.push(row.uppermwlimit);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(tradingdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdatetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(uppermwlimit_array)),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## INTERMITTENT_GEN_LIMIT_DAY
///  _Summary record for an Upper MW Limit submission for an intermittent generating unit for a Trading Day_
///
/// * Data Set Name: Demand
/// * File Name: Intermittent Gen Limit Day
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * DUID
/// * OFFERDATETIME
/// * TRADINGDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandIntermittentGenLimitDay1 {
    /// Trading Day for which this unit availability submission applies
    #[serde(with = "crate::mms_datetime")]
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: String,
    /// Date and Time when this unit availability submission was loaded
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// Unique participant identifier
    pub participantid: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// User entering the unit availability submission
    pub authorisedbyuser: Option<String>,
    /// Participant entering the unit availability submission
    pub authorisedbyparticipantid: Option<String>,
}
impl crate::GetTable for DemandIntermittentGenLimitDay1 {
    type PrimaryKey = DemandIntermittentGenLimitDay1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("INTERMITTENT_GEN_LIMIT_DAY".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DemandIntermittentGenLimitDay1PrimaryKey {
        DemandIntermittentGenLimitDay1PrimaryKey {
            duid: self.duid.clone(),
            offerdatetime: self.offerdatetime,
            tradingdate: self.tradingdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "demand_intermittent_gen_limit_day_v1".to_string()
    }
}
impl crate::CompareWithRow for DemandIntermittentGenLimitDay1 {
    type Row = DemandIntermittentGenLimitDay1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.offerdatetime == row.offerdatetime
            && self.tradingdate == row.tradingdate
    }
}
impl crate::CompareWithPrimaryKey for DemandIntermittentGenLimitDay1 {
    type PrimaryKey = DemandIntermittentGenLimitDay1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.tradingdate == key.tradingdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct DemandIntermittentGenLimitDay1PrimaryKey {
    pub duid: String,
    pub offerdatetime: chrono::NaiveDateTime,
    pub tradingdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DemandIntermittentGenLimitDay1PrimaryKey {
    type Row = DemandIntermittentGenLimitDay1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.offerdatetime == row.offerdatetime
            && self.tradingdate == row.tradingdate
    }
}
impl crate::CompareWithPrimaryKey for DemandIntermittentGenLimitDay1PrimaryKey {
    type PrimaryKey = DemandIntermittentGenLimitDay1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.tradingdate == key.tradingdate
    }
}
impl crate::PrimaryKey for DemandIntermittentGenLimitDay1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DemandIntermittentGenLimitDay1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "tradingdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "offerdatetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "authorisedbyuser",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authorisedbyparticipantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut tradingdate_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut offerdatetime_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut authorisedbyuser_array = Vec::new();
        let mut authorisedbyparticipantid_array = Vec::new();
        for (_, row) in partition {
            tradingdate_array.push(row.tradingdate.timestamp_millis());
            duid_array.push(row.duid);
            offerdatetime_array.push(row.offerdatetime.timestamp_millis());
            participantid_array.push(row.participantid);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            authorisedbyuser_array.push(row.authorisedbyuser);
            authorisedbyparticipantid_array.push(row.authorisedbyparticipantid);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(tradingdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdatetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(participantid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    authorisedbyuser_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    authorisedbyparticipantid_array,
                )),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MTPASA_INTERMITTENT_AVAIL
///  _A submission of expected plant availability for intermittent generators for use in MTPASA intermittent generation forecasts_
///
/// * Data Set Name: Demand
/// * File Name: Mtpasa Intermittent Avail
/// * Data Version: 2
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CLUSTERID
/// * DUID
/// * OFFERDATETIME
/// * TRADINGDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandMtpasaIntermittentAvail2 {
    /// Trading Day for which this cluster availability submission applies
    #[serde(with = "crate::mms_datetime")]
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: String,
    /// Date and Time when this cluster availability submission was loaded
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// Unique Cluster Identifier for this cluster within the DUID
    pub clusterid: String,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Number of elements within this CLUSTERID (turbines for wind, or inverters for solar) that are not available for this TRADINGDATE. Value between 0 and the registered Number of Cluster Elements.Value = 0 means no elements unavailable
    pub elements_unavailable: Option<rust_decimal::Decimal>,
    /// Number of elements within this CLUSTERID (turbines for wind, or inverters for solar) that are available for this TRADINGDATE. Value between 0 and the registered Number of Cluster Elements. Value = 0 means no elements available
    pub elements_available: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DemandMtpasaIntermittentAvail2 {
    type PrimaryKey = DemandMtpasaIntermittentAvail2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("MTPASA_INTERMITTENT_AVAIL".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> DemandMtpasaIntermittentAvail2PrimaryKey {
        DemandMtpasaIntermittentAvail2PrimaryKey {
            clusterid: self.clusterid.clone(),
            duid: self.duid.clone(),
            offerdatetime: self.offerdatetime,
            tradingdate: self.tradingdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "demand_mtpasa_intermittent_avail_v2".to_string()
    }
}
impl crate::CompareWithRow for DemandMtpasaIntermittentAvail2 {
    type Row = DemandMtpasaIntermittentAvail2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.clusterid == row.clusterid
            && self.duid == row.duid
            && self.offerdatetime == row.offerdatetime
            && self.tradingdate == row.tradingdate
    }
}
impl crate::CompareWithPrimaryKey for DemandMtpasaIntermittentAvail2 {
    type PrimaryKey = DemandMtpasaIntermittentAvail2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.clusterid == key.clusterid
            && self.duid == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.tradingdate == key.tradingdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct DemandMtpasaIntermittentAvail2PrimaryKey {
    pub clusterid: String,
    pub duid: String,
    pub offerdatetime: chrono::NaiveDateTime,
    pub tradingdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DemandMtpasaIntermittentAvail2PrimaryKey {
    type Row = DemandMtpasaIntermittentAvail2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.clusterid == row.clusterid
            && self.duid == row.duid
            && self.offerdatetime == row.offerdatetime
            && self.tradingdate == row.tradingdate
    }
}
impl crate::CompareWithPrimaryKey for DemandMtpasaIntermittentAvail2PrimaryKey {
    type PrimaryKey = DemandMtpasaIntermittentAvail2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.clusterid == key.clusterid
            && self.duid == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.tradingdate == key.tradingdate
    }
}
impl crate::PrimaryKey for DemandMtpasaIntermittentAvail2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DemandMtpasaIntermittentAvail2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "tradingdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "offerdatetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "clusterid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "elements_unavailable",
                arrow2::datatypes::DataType::Decimal(5, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "elements_available",
                arrow2::datatypes::DataType::Decimal(5, 0),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut tradingdate_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut offerdatetime_array = Vec::new();
        let mut clusterid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut elements_unavailable_array = Vec::new();
        let mut elements_available_array = Vec::new();
        for (_, row) in partition {
            tradingdate_array.push(row.tradingdate.timestamp_millis());
            duid_array.push(row.duid);
            offerdatetime_array.push(row.offerdatetime.timestamp_millis());
            clusterid_array.push(row.clusterid);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            elements_unavailable_array.push({
                row.elements_unavailable.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            elements_available_array.push({
                row.elements_available.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(tradingdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdatetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(clusterid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(elements_unavailable_array)
                        .to(arrow2::datatypes::DataType::Decimal(5, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(elements_available_array)
                        .to(arrow2::datatypes::DataType::Decimal(5, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MTPASA_INTERMITTENT_LIMIT
///  _A submission of expected maximum availability for intermittent generators for use in MTPASA intermittent generation<br>forecasts_
///
/// * Data Set Name: Demand
/// * File Name: Mtpasa Intermittent Limit
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * DUID
/// * OFFERDATETIME
/// * TRADINGDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandMtpasaIntermittentLimit1 {
    /// Trading Day for which this unit availability submission applies
    #[serde(with = "crate::mms_datetime")]
    pub tradingdate: chrono::NaiveDateTime,
    /// Unique Identifier of Dispatchable Unit
    pub duid: String,
    /// Date time file processed
    #[serde(with = "crate::mms_datetime")]
    pub offerdatetime: chrono::NaiveDateTime,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Maximum imposed MW limit. Value between 0 and the registered DUID Maximum Capacity.Value = -1 means no limit applies.
    pub uppermwlimit: Option<i64>,
    /// User entering the unit availability submission
    pub authorisedbyuser: Option<String>,
    /// Participant entering the unit availability submission
    pub authorisedbyparticipantid: Option<String>,
}
impl crate::GetTable for DemandMtpasaIntermittentLimit1 {
    type PrimaryKey = DemandMtpasaIntermittentLimit1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("MTPASA_INTERMITTENT_LIMIT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DemandMtpasaIntermittentLimit1PrimaryKey {
        DemandMtpasaIntermittentLimit1PrimaryKey {
            duid: self.duid.clone(),
            offerdatetime: self.offerdatetime,
            tradingdate: self.tradingdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "demand_mtpasa_intermittent_limit_v1".to_string()
    }
}
impl crate::CompareWithRow for DemandMtpasaIntermittentLimit1 {
    type Row = DemandMtpasaIntermittentLimit1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.offerdatetime == row.offerdatetime
            && self.tradingdate == row.tradingdate
    }
}
impl crate::CompareWithPrimaryKey for DemandMtpasaIntermittentLimit1 {
    type PrimaryKey = DemandMtpasaIntermittentLimit1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.tradingdate == key.tradingdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct DemandMtpasaIntermittentLimit1PrimaryKey {
    pub duid: String,
    pub offerdatetime: chrono::NaiveDateTime,
    pub tradingdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for DemandMtpasaIntermittentLimit1PrimaryKey {
    type Row = DemandMtpasaIntermittentLimit1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.offerdatetime == row.offerdatetime
            && self.tradingdate == row.tradingdate
    }
}
impl crate::CompareWithPrimaryKey for DemandMtpasaIntermittentLimit1PrimaryKey {
    type PrimaryKey = DemandMtpasaIntermittentLimit1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.offerdatetime == key.offerdatetime
            && self.tradingdate == key.tradingdate
    }
}
impl crate::PrimaryKey for DemandMtpasaIntermittentLimit1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DemandMtpasaIntermittentLimit1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "tradingdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "offerdatetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("uppermwlimit", arrow2::datatypes::DataType::Int64, true),
            arrow2::datatypes::Field::new(
                "authorisedbyuser",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authorisedbyparticipantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut tradingdate_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut offerdatetime_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut uppermwlimit_array = Vec::new();
        let mut authorisedbyuser_array = Vec::new();
        let mut authorisedbyparticipantid_array = Vec::new();
        for (_, row) in partition {
            tradingdate_array.push(row.tradingdate.timestamp_millis());
            duid_array.push(row.duid);
            offerdatetime_array.push(row.offerdatetime.timestamp_millis());
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            uppermwlimit_array.push(row.uppermwlimit);
            authorisedbyuser_array.push(row.authorisedbyuser);
            authorisedbyparticipantid_array.push(row.authorisedbyparticipantid);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(tradingdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdatetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(uppermwlimit_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    authorisedbyuser_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    authorisedbyparticipantid_array,
                )),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PERDEMAND
///  _PERDEMAND sets out the regional demands and MR schedule data for each half-hour period. PERDEMAND is a child table to RESDEMANDTRK._
///
/// * Data Set Name: Demand
/// * File Name: Period
/// * Data Version: 1
///
/// # Description
///  The RESDEMANDTRK and PERDEMAND tables have a parent/child relationship, and define forecast regional demands since market start. RESDEMANDTRK defines the existence and versioning information of a forecast for a specific region and trading date. PERDEMAND defines the numerical forecast values for each trading interval of a the trading day for that region. A complete trading day forecast for one region consists of one RESDEMANDTRK record and 48 PERDEMAND records. Source PERDEMAND updates whenever AEMO issues a new or revised forecast. ST PASA forecasts update seven days at a time. Predispatch updates one date. Volume 1296000 rows per year Note In the context of a mandatory restrictions event the forecast schedule (MW) of restrictions are reported through the RESDEMANDTRK and PERDEMAND tables using the new field PerDemand.MR_Schedule. The relationship between fields and mandatory restriction terms for the 50% probability of exceedence forecast are: · UnRestricted Profile  = ResDemand + MR_Schedule · Restricted Profile  = ResDemand
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * OFFERDATE
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandPeriod1 {
    /// Market date the forecast is made for. First date of the 7 days.
    #[serde(with = "crate::mms_datetime_opt")]
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// Market date of forecast up to 7 days ahead.
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Differentiates this region from all other regions
    pub regionid: String,
    /// Date record issued
    #[serde(with = "crate::mms_datetime")]
    pub offerdate: chrono::NaiveDateTime,
    /// Half hourly trading intervals from 04:30.
    pub periodid: rust_decimal::Decimal,
    /// The version of the RESDEMAND file for this date
    pub versionno: rust_decimal::Decimal,
    /// Base Demand forecast for period
    pub resdemand: Option<rust_decimal::Decimal>,
    /// Demand at 90% probability of exceedance
    pub demand90probability: Option<rust_decimal::Decimal>,
    /// Demand level for a 10% probability of exceedance
    pub demand10probability: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// MR_Schedule = Unrestricted Demand - POE
    pub mr_schedule: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for DemandPeriod1 {
    type PrimaryKey = DemandPeriod1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("PERIOD".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DemandPeriod1PrimaryKey {
        DemandPeriod1PrimaryKey {
            offerdate: self.offerdate,
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
            versionno: self.versionno,
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
            "demand_period_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for DemandPeriod1 {
    type Row = DemandPeriod1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.offerdate == row.offerdate
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for DemandPeriod1 {
    type PrimaryKey = DemandPeriod1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.offerdate == key.offerdate
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct DemandPeriod1PrimaryKey {
    pub offerdate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for DemandPeriod1PrimaryKey {
    type Row = DemandPeriod1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.offerdate == row.offerdate
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for DemandPeriod1PrimaryKey {
    type PrimaryKey = DemandPeriod1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.offerdate == key.offerdate
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for DemandPeriod1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DemandPeriod1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("offerdate", arrow2::datatypes::DataType::Date64, false),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "resdemand",
                arrow2::datatypes::DataType::Decimal(10, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "demand90probability",
                arrow2::datatypes::DataType::Decimal(10, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "demand10probability",
                arrow2::datatypes::DataType::Decimal(10, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "mr_schedule",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut effectivedate_array = Vec::new();
        let mut settlementdate_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut offerdate_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut resdemand_array = Vec::new();
        let mut demand90probability_array = Vec::new();
        let mut demand10probability_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut mr_schedule_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(row.effectivedate.map(|val| val.timestamp_millis()));
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            regionid_array.push(row.regionid);
            offerdate_array.push(row.offerdate.timestamp_millis());
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            resdemand_array.push({
                row.resdemand.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            demand90probability_array.push({
                row.demand90probability.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            demand10probability_array.push({
                row.demand10probability.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            mr_schedule_array.push({
                row.mr_schedule.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(resdemand_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(demand90probability_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(demand10probability_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mr_schedule_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## RESDEMANDTRK
///  _RESDEMANDTRK defines the existence and versioning information of a forecast for a specific region and trading date.<br>RESDEMANDTRK and PERDEMAND have a parent/child relationship, and are for defined forecast regional demands since market start. RESDEMANDTRK defines the existence and versioning information of a forecast for a specific region and trading date. PERDEMAND defines the numerical forecast values for each trading interval of a the trading day for that region. A complete trading day forecast for one region consists of one RESDEMANDTRK record and 48 PERDEMAND records.<br>_
///
/// * Data Set Name: Demand
/// * File Name: Trk
/// * Data Version: 1
///
/// # Description
///  RESDEMANDTRK data is public, so is available to all participants. Source RESDEMANDTRK updates are ad hoc. Volume 27000 rows per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * OFFERDATE
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct DemandTrk1 {
    /// Trading Date of the regional forecast
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: String,
    /// Date the forecast was created
    #[serde(with = "crate::mms_datetime")]
    pub offerdate: chrono::NaiveDateTime,
    /// Version of this forecast with respect to the Effectivedate and Offerdate
    pub versionno: rust_decimal::Decimal,
    /// Tracking purposes only
    pub filename: Option<String>,
    /// Date forecast authorised
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Identifier of authorising user
    pub authorisedby: Option<String>,
    /// Date and time the record was last modified
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for DemandTrk1 {
    type PrimaryKey = DemandTrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "DEMAND".into(),
            table_name: Some("TRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> DemandTrk1PrimaryKey {
        DemandTrk1PrimaryKey {
            effectivedate: self.effectivedate,
            offerdate: self.offerdate,
            regionid: self.regionid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "demand_trk_v1".to_string()
    }
}
impl crate::CompareWithRow for DemandTrk1 {
    type Row = DemandTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.offerdate == row.offerdate
            && self.regionid == row.regionid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for DemandTrk1 {
    type PrimaryKey = DemandTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.offerdate == key.offerdate
            && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct DemandTrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub offerdate: chrono::NaiveDateTime,
    pub regionid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for DemandTrk1PrimaryKey {
    type Row = DemandTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.offerdate == row.offerdate
            && self.regionid == row.regionid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for DemandTrk1PrimaryKey {
    type PrimaryKey = DemandTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.offerdate == key.offerdate
            && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for DemandTrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for DemandTrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("offerdate", arrow2::datatypes::DataType::Date64, false),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("filename", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut effectivedate_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut offerdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut filename_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            regionid_array.push(row.regionid);
            offerdate_array.push(row.offerdate.timestamp_millis());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            filename_array.push(row.filename);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp_millis()));
            authorisedby_array.push(row.authorisedby);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(offerdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(filename_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## ROOFTOP_PV_ACTUAL
///  _Estimate of regional Rooftop Solar actual generation for each half-hour interval in a day_
///
/// * Data Set Name: Rooftop
/// * File Name: Actual
/// * Data Version: 2
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
/// * TYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct RooftopActual2 {
    /// The forecast half-hour interval (time ending)
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// One of DAILY, MEASUREMENT or SATELLITE. DAILY- best quality estimated actuals, available day after. MEASUREMENT- best quality estimated actuals on day, delayed by 1 half hour. SATELLITE- estimated actuals using satellite imagery, delayed by 1 half hour.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Region identifier
    pub regionid: String,
    /// Estimated generation in MW at the interval end
    pub power: Option<rust_decimal::Decimal>,
    /// Quality indicator. Represents the quality of the estimate.
    pub qi: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for RooftopActual2 {
    type PrimaryKey = RooftopActual2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ROOFTOP".into(),
            table_name: Some("ACTUAL".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> RooftopActual2PrimaryKey {
        RooftopActual2PrimaryKey {
            interval_datetime: self.interval_datetime,
            regionid: self.regionid.clone(),
            r#type: self.r#type.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "rooftop_actual_v2".to_string()
    }
}
impl crate::CompareWithRow for RooftopActual2 {
    type Row = RooftopActual2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid
            && self.r#type == row.r#type
    }
}
impl crate::CompareWithPrimaryKey for RooftopActual2 {
    type PrimaryKey = RooftopActual2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid == key.regionid
            && self.r#type == key.r#type
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct RooftopActual2PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: String,
    pub r#type: String,
}
impl crate::CompareWithRow for RooftopActual2PrimaryKey {
    type Row = RooftopActual2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid
            && self.r#type == row.r#type
    }
}
impl crate::CompareWithPrimaryKey for RooftopActual2PrimaryKey {
    type PrimaryKey = RooftopActual2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid == key.regionid
            && self.r#type == key.r#type
    }
}
impl crate::PrimaryKey for RooftopActual2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for RooftopActual2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "interval_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("r#type", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "power",
                arrow2::datatypes::DataType::Decimal(12, 3),
                true,
            ),
            arrow2::datatypes::Field::new("qi", arrow2::datatypes::DataType::Decimal(2, 1), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut interval_datetime_array = Vec::new();
        let mut r#type_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut power_array = Vec::new();
        let mut qi_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            interval_datetime_array.push(row.interval_datetime.timestamp_millis());
            r#type_array.push(row.r#type);
            regionid_array.push(row.regionid);
            power_array.push({
                row.power.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            qi_array.push({
                row.qi.map(|mut val| {
                    val.rescale(1);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(interval_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(r#type_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(power_array)
                        .to(arrow2::datatypes::DataType::Decimal(12, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(qi_array)
                        .to(arrow2::datatypes::DataType::Decimal(2, 1)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## ROOFTOP_PV_FORECAST
///  _Regional forecasts of Rooftop Solar generation across the half-hour intervals over 8 days_
///
/// * Data Set Name: Rooftop
/// * File Name: Forecast
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERVAL_DATETIME
/// * REGIONID
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct RooftopForecast1 {
    /// Date time this forecast was produced
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// Region identifier
    pub regionid: String,
    /// The forecast half-hour interval (time ending)
    #[serde(with = "crate::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// The average forecast value in MW at the interval end
    pub powermean: Option<rust_decimal::Decimal>,
    /// 50% probability of exceedance forecast value in MW at the interval end
    pub powerpoe50: Option<rust_decimal::Decimal>,
    /// 10% probability of exceedance forecast value in MW at the interval end
    pub powerpoelow: Option<rust_decimal::Decimal>,
    /// 90% probability of exceedance forecast value in MW at the interval end
    pub powerpoehigh: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for RooftopForecast1 {
    type PrimaryKey = RooftopForecast1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ROOFTOP".into(),
            table_name: Some("FORECAST".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> RooftopForecast1PrimaryKey {
        RooftopForecast1PrimaryKey {
            interval_datetime: self.interval_datetime,
            regionid: self.regionid.clone(),
            version_datetime: self.version_datetime,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "rooftop_forecast_v1".to_string()
    }
}
impl crate::CompareWithRow for RooftopForecast1 {
    type Row = RooftopForecast1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for RooftopForecast1 {
    type PrimaryKey = RooftopForecast1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid == key.regionid
            && self.version_datetime == key.version_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct RooftopForecast1PrimaryKey {
    pub interval_datetime: chrono::NaiveDateTime,
    pub regionid: String,
    pub version_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for RooftopForecast1PrimaryKey {
    type Row = RooftopForecast1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interval_datetime == row.interval_datetime
            && self.regionid == row.regionid
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for RooftopForecast1PrimaryKey {
    type PrimaryKey = RooftopForecast1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interval_datetime == key.interval_datetime
            && self.regionid == key.regionid
            && self.version_datetime == key.version_datetime
    }
}
impl crate::PrimaryKey for RooftopForecast1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for RooftopForecast1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "version_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interval_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "powermean",
                arrow2::datatypes::DataType::Decimal(12, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "powerpoe50",
                arrow2::datatypes::DataType::Decimal(12, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "powerpoelow",
                arrow2::datatypes::DataType::Decimal(12, 3),
                true,
            ),
            arrow2::datatypes::Field::new(
                "powerpoehigh",
                arrow2::datatypes::DataType::Decimal(12, 3),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut version_datetime_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut interval_datetime_array = Vec::new();
        let mut powermean_array = Vec::new();
        let mut powerpoe50_array = Vec::new();
        let mut powerpoelow_array = Vec::new();
        let mut powerpoehigh_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            version_datetime_array.push(row.version_datetime.timestamp_millis());
            regionid_array.push(row.regionid);
            interval_datetime_array.push(row.interval_datetime.timestamp_millis());
            powermean_array.push({
                row.powermean.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            powerpoe50_array.push({
                row.powerpoe50.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            powerpoelow_array.push({
                row.powerpoelow.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            powerpoehigh_array.push({
                row.powerpoehigh.map(|mut val| {
                    val.rescale(3);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(version_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(interval_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(powermean_array)
                        .to(arrow2::datatypes::DataType::Decimal(12, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(powerpoe50_array)
                        .to(arrow2::datatypes::DataType::Decimal(12, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(powerpoelow_array)
                        .to(arrow2::datatypes::DataType::Decimal(12, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(powerpoehigh_array)
                        .to(arrow2::datatypes::DataType::Decimal(12, 3)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
