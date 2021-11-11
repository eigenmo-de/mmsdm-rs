/// # Summary
///
/// ## METERDATA_AGGREGATE_READS
///  _Publishes aggregated metering data associated with a wholesale connection point for a given CASE_ID_
///
/// * Data Set Name: Meterdata
/// * File Name: Aggregate Reads
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CASE_ID
/// * CONNECTIONPOINTID
/// * FRMP
/// * LR
/// * METER_TYPE
/// * PERIODID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterdataAggregateReads1 {
    /// Case Identifier
    pub case_id: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Connection Point ID
    pub connectionpointid: String,
    /// The meter type for the read, one of: CUSTOMER; GENERATOR; EMBEDDED_GENERATOR
    pub meter_type: String,
    /// The financially responsible market participantid
    pub frmp: String,
    /// The local retailer at the connection point id
    pub lr: String,
    /// Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// The import(pool-centric) value for the meter read (MWh)
    pub importvalue: rust_decimal::Decimal,
    /// The export(pool-centric) value for the meter read (MWh)
    pub exportvalue: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MeterdataAggregateReads1 {
    type PrimaryKey = MeterdataAggregateReads1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "METERDATA".into(),
            table_name: Some("AGGREGATE_READS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> MeterdataAggregateReads1PrimaryKey {
        MeterdataAggregateReads1PrimaryKey {
            case_id: self.case_id.clone(),
            connectionpointid: self.connectionpointid.clone(),
            frmp: self.frmp.clone(),
            lr: self.lr.clone(),
            meter_type: self.meter_type.clone(),
            periodid: self.periodid.clone(),
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
            "meterdata_aggregate_reads_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for MeterdataAggregateReads1 {
    type Row = MeterdataAggregateReads1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.case_id == row.case_id
            && self.connectionpointid == row.connectionpointid
            && self.frmp == row.frmp
            && self.lr == row.lr
            && self.meter_type == row.meter_type
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for MeterdataAggregateReads1 {
    type PrimaryKey = MeterdataAggregateReads1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id
            && self.connectionpointid == key.connectionpointid
            && self.frmp == key.frmp
            && self.lr == key.lr
            && self.meter_type == key.meter_type
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeterdataAggregateReads1PrimaryKey {
    pub case_id: rust_decimal::Decimal,
    pub connectionpointid: String,
    pub frmp: String,
    pub lr: String,
    pub meter_type: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for MeterdataAggregateReads1PrimaryKey {
    type Row = MeterdataAggregateReads1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.case_id == row.case_id
            && self.connectionpointid == row.connectionpointid
            && self.frmp == row.frmp
            && self.lr == row.lr
            && self.meter_type == row.meter_type
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for MeterdataAggregateReads1PrimaryKey {
    type PrimaryKey = MeterdataAggregateReads1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id
            && self.connectionpointid == key.connectionpointid
            && self.frmp == key.frmp
            && self.lr == key.lr
            && self.meter_type == key.meter_type
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for MeterdataAggregateReads1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for MeterdataAggregateReads1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "case_id",
                arrow2::datatypes::DataType::Decimal(15, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "meter_type",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("frmp", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("lr", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "importvalue",
                arrow2::datatypes::DataType::Decimal(18, 8),
                false,
            ),
            arrow2::datatypes::Field::new(
                "exportvalue",
                arrow2::datatypes::DataType::Decimal(18, 8),
                false,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut case_id_array = Vec::new();
        let mut settlementdate_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut meter_type_array = Vec::new();
        let mut frmp_array = Vec::new();
        let mut lr_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut importvalue_array = Vec::new();
        let mut exportvalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            case_id_array.push({
                let mut val = row.case_id;
                val.rescale(0);
                val.mantissa()
            });
            settlementdate_array.push(
                i32::try_from(
                    (row.settlementdate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            connectionpointid_array.push(row.connectionpointid);
            meter_type_array.push(row.meter_type);
            frmp_array.push(row.frmp);
            lr_array.push(row.lr);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            importvalue_array.push({
                let mut val = row.importvalue;
                val.rescale(8);
                val.mantissa()
            });
            exportvalue_array.push({
                let mut val = row.exportvalue;
                val.rescale(8);
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
                    arrow2::array::PrimitiveArray::from_slice(case_id_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    connectionpointid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    meter_type_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(frmp_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(lr_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(importvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(exportvalue_array)
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
/// ## METERDATA_INDIVIDUAL_READS
///  _Publishes metering data associated with individual metering points for a given CASE_ID_
///
/// * Data Set Name: Meterdata
/// * File Name: Individual Reads
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CASE_ID
/// * METER_ID
/// * METER_ID_SUFFIX
/// * PERIODID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterdataIndividualReads1 {
    /// Case Identifier
    pub case_id: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// The National Metering Identifier (NMI)
    pub meter_id: String,
    /// The National Metering Identifier (NMI) data stream
    pub meter_id_suffix: String,
    /// The financially responsible market participantid
    pub frmp: String,
    /// The local retailer at the connection point id
    pub lr: String,
    /// Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Connection Point ID
    pub connectionpointid: String,
    /// The meter type for the read, one of: CUSTOMER; GENERATOR; EMBEDDED_GENERATOR
    pub meter_type: String,
    /// The import(pool-centric) value for the meter read (MWh)
    pub importvalue: rust_decimal::Decimal,
    /// The export(pool-centric) value for the meter read (MWh)
    pub exportvalue: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MeterdataIndividualReads1 {
    type PrimaryKey = MeterdataIndividualReads1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "METERDATA".into(),
            table_name: Some("INDIVIDUAL_READS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> MeterdataIndividualReads1PrimaryKey {
        MeterdataIndividualReads1PrimaryKey {
            case_id: self.case_id.clone(),
            meter_id: self.meter_id.clone(),
            meter_id_suffix: self.meter_id_suffix.clone(),
            periodid: self.periodid.clone(),
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
            "meterdata_individual_reads_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for MeterdataIndividualReads1 {
    type Row = MeterdataIndividualReads1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.case_id == row.case_id
            && self.meter_id == row.meter_id
            && self.meter_id_suffix == row.meter_id_suffix
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for MeterdataIndividualReads1 {
    type PrimaryKey = MeterdataIndividualReads1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id
            && self.meter_id == key.meter_id
            && self.meter_id_suffix == key.meter_id_suffix
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeterdataIndividualReads1PrimaryKey {
    pub case_id: rust_decimal::Decimal,
    pub meter_id: String,
    pub meter_id_suffix: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for MeterdataIndividualReads1PrimaryKey {
    type Row = MeterdataIndividualReads1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.case_id == row.case_id
            && self.meter_id == row.meter_id
            && self.meter_id_suffix == row.meter_id_suffix
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for MeterdataIndividualReads1PrimaryKey {
    type PrimaryKey = MeterdataIndividualReads1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id
            && self.meter_id == key.meter_id
            && self.meter_id_suffix == key.meter_id_suffix
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for MeterdataIndividualReads1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for MeterdataIndividualReads1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "case_id",
                arrow2::datatypes::DataType::Decimal(15, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "meter_id",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "meter_id_suffix",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("frmp", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("lr", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "meter_type",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "importvalue",
                arrow2::datatypes::DataType::Decimal(18, 8),
                false,
            ),
            arrow2::datatypes::Field::new(
                "exportvalue",
                arrow2::datatypes::DataType::Decimal(18, 8),
                false,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut case_id_array = Vec::new();
        let mut settlementdate_array = Vec::new();
        let mut meter_id_array = Vec::new();
        let mut meter_id_suffix_array = Vec::new();
        let mut frmp_array = Vec::new();
        let mut lr_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut meter_type_array = Vec::new();
        let mut importvalue_array = Vec::new();
        let mut exportvalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            case_id_array.push({
                let mut val = row.case_id;
                val.rescale(0);
                val.mantissa()
            });
            settlementdate_array.push(
                i32::try_from(
                    (row.settlementdate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            meter_id_array.push(row.meter_id);
            meter_id_suffix_array.push(row.meter_id_suffix);
            frmp_array.push(row.frmp);
            lr_array.push(row.lr);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            connectionpointid_array.push(row.connectionpointid);
            meter_type_array.push(row.meter_type);
            importvalue_array.push({
                let mut val = row.importvalue;
                val.rescale(8);
                val.mantissa()
            });
            exportvalue_array.push({
                let mut val = row.exportvalue;
                val.rescale(8);
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
                    arrow2::array::PrimitiveArray::from_slice(case_id_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(meter_id_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    meter_id_suffix_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(frmp_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(lr_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    connectionpointid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    meter_type_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(importvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(exportvalue_array)
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
/// ## METERDATA_INTERCONNECTOR
///  _Publishes metering data associated with wholesale interconnectors for a given CASE_ID_
///
/// * Data Set Name: Meterdata
/// * File Name: Interconnector
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CASE_ID
/// * INTERCONNECTORID
/// * PERIODID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterdataInterconnector1 {
    /// Case Identifier
    pub case_id: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Interconnector Identifier
    pub interconnectorid: String,
    /// Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// The import direction value for the meter read (MWh)
    pub importvalue: Option<rust_decimal::Decimal>,
    /// The export direction value for the meter read (MWh)
    pub exportvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MeterdataInterconnector1 {
    type PrimaryKey = MeterdataInterconnector1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "METERDATA".into(),
            table_name: Some("INTERCONNECTOR".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> MeterdataInterconnector1PrimaryKey {
        MeterdataInterconnector1PrimaryKey {
            case_id: self.case_id.clone(),
            interconnectorid: self.interconnectorid.clone(),
            periodid: self.periodid.clone(),
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
            "meterdata_interconnector_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for MeterdataInterconnector1 {
    type Row = MeterdataInterconnector1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.case_id == row.case_id
            && self.interconnectorid == row.interconnectorid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for MeterdataInterconnector1 {
    type PrimaryKey = MeterdataInterconnector1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id
            && self.interconnectorid == key.interconnectorid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeterdataInterconnector1PrimaryKey {
    pub case_id: rust_decimal::Decimal,
    pub interconnectorid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for MeterdataInterconnector1PrimaryKey {
    type Row = MeterdataInterconnector1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.case_id == row.case_id
            && self.interconnectorid == row.interconnectorid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl crate::CompareWithPrimaryKey for MeterdataInterconnector1PrimaryKey {
    type PrimaryKey = MeterdataInterconnector1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id
            && self.interconnectorid == key.interconnectorid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
impl crate::PrimaryKey for MeterdataInterconnector1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for MeterdataInterconnector1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "case_id",
                arrow2::datatypes::DataType::Decimal(15, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date32,
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
                "importvalue",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "exportvalue",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut case_id_array = Vec::new();
        let mut settlementdate_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut importvalue_array = Vec::new();
        let mut exportvalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            case_id_array.push({
                let mut val = row.case_id;
                val.rescale(0);
                val.mantissa()
            });
            settlementdate_array.push(
                i32::try_from(
                    (row.settlementdate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            interconnectorid_array.push(row.interconnectorid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            importvalue_array.push({
                row.importvalue.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            exportvalue_array.push({
                row.exportvalue.map(|mut val| {
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
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(case_id_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(importvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(exportvalue_array)
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
