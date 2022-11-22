use chrono::Datelike as _;
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
    /// Settlement date within the case
    #[serde(with = "mmsdm_core::mms_datetime")]
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
    /// Last changed date for the record
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MeterdataAggregateReads1 {
    type PrimaryKey = MeterdataAggregateReads1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "METERDATA".into(),
            table_name: Some("AGGREGATE_READS".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MeterdataAggregateReads1PrimaryKey {
        MeterdataAggregateReads1PrimaryKey {
            case_id: self.case_id,
            connectionpointid: self.connectionpointid.clone(),
            frmp: self.frmp.clone(),
            lr: self.lr.clone(),
            meter_type: self.meter_type.clone(),
            periodid: self.periodid,
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
            "meterdata_aggregate_reads_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MeterdataAggregateReads1PrimaryKey {
    pub case_id: rust_decimal::Decimal,
    pub connectionpointid: String,
    pub frmp: String,
    pub lr: String,
    pub meter_type: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MeterdataAggregateReads1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MeterdataAggregateReads1 {
    type Row = MeterdataAggregateReads1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.case_id == row.case_id && self.connectionpointid == row.connectionpointid
            && self.frmp == row.frmp && self.lr == row.lr
            && self.meter_type == row.meter_type && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterdataAggregateReads1 {
    type PrimaryKey = MeterdataAggregateReads1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.connectionpointid == key.connectionpointid
            && self.frmp == key.frmp && self.lr == key.lr
            && self.meter_type == key.meter_type && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for MeterdataAggregateReads1PrimaryKey {
    type Row = MeterdataAggregateReads1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.case_id == row.case_id && self.connectionpointid == row.connectionpointid
            && self.frmp == row.frmp && self.lr == row.lr
            && self.meter_type == row.meter_type && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterdataAggregateReads1PrimaryKey {
    type PrimaryKey = MeterdataAggregateReads1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.connectionpointid == key.connectionpointid
            && self.frmp == key.frmp && self.lr == key.lr
            && self.meter_type == key.meter_type && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MeterdataAggregateReads1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("case_id",
                arrow2::datatypes::DataType::Decimal(15, 0), false),
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("meter_type",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("frmp",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("lr",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("importvalue",
                arrow2::datatypes::DataType::Decimal(18, 8), false),
                arrow2::datatypes::Field::new("exportvalue",
                arrow2::datatypes::DataType::Decimal(18, 8), false),
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
        for row in partition {
            case_id_array
                .push({
                    let mut val = row.case_id;
                    val.rescale(0);
                    val.mantissa()
                });
            settlementdate_array.push(row.settlementdate.timestamp());
            connectionpointid_array.push(row.connectionpointid);
            meter_type_array.push(row.meter_type);
            frmp_array.push(row.frmp);
            lr_array.push(row.lr);
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            importvalue_array
                .push({
                    let mut val = row.importvalue;
                    val.rescale(8);
                    val.mantissa()
                });
            exportvalue_array
                .push({
                    let mut val = row.exportvalue;
                    val.rescale(8);
                    val.mantissa()
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(case_id_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(connectionpointid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(meter_type_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(frmp_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(lr_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(importvalue_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(exportvalue_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
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
    /// Settlement date within the case
    #[serde(with = "mmsdm_core::mms_datetime")]
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
    /// Last changed date for the record
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MeterdataIndividualReads1 {
    type PrimaryKey = MeterdataIndividualReads1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "METERDATA".into(),
            table_name: Some("INDIVIDUAL_READS".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MeterdataIndividualReads1PrimaryKey {
        MeterdataIndividualReads1PrimaryKey {
            case_id: self.case_id,
            meter_id: self.meter_id.clone(),
            meter_id_suffix: self.meter_id_suffix.clone(),
            periodid: self.periodid,
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
            "meterdata_individual_reads_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MeterdataIndividualReads1PrimaryKey {
    pub case_id: rust_decimal::Decimal,
    pub meter_id: String,
    pub meter_id_suffix: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MeterdataIndividualReads1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MeterdataIndividualReads1 {
    type Row = MeterdataIndividualReads1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.case_id == row.case_id && self.meter_id == row.meter_id
            && self.meter_id_suffix == row.meter_id_suffix
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterdataIndividualReads1 {
    type PrimaryKey = MeterdataIndividualReads1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.meter_id == key.meter_id
            && self.meter_id_suffix == key.meter_id_suffix
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for MeterdataIndividualReads1PrimaryKey {
    type Row = MeterdataIndividualReads1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.case_id == row.case_id && self.meter_id == row.meter_id
            && self.meter_id_suffix == row.meter_id_suffix
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterdataIndividualReads1PrimaryKey {
    type PrimaryKey = MeterdataIndividualReads1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.meter_id == key.meter_id
            && self.meter_id_suffix == key.meter_id_suffix
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MeterdataIndividualReads1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("case_id",
                arrow2::datatypes::DataType::Decimal(15, 0), false),
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("meter_id",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("meter_id_suffix",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("frmp",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("lr",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("meter_type",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("importvalue",
                arrow2::datatypes::DataType::Decimal(18, 8), false),
                arrow2::datatypes::Field::new("exportvalue",
                arrow2::datatypes::DataType::Decimal(18, 8), false),
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
        for row in partition {
            case_id_array
                .push({
                    let mut val = row.case_id;
                    val.rescale(0);
                    val.mantissa()
                });
            settlementdate_array.push(row.settlementdate.timestamp());
            meter_id_array.push(row.meter_id);
            meter_id_suffix_array.push(row.meter_id_suffix);
            frmp_array.push(row.frmp);
            lr_array.push(row.lr);
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            connectionpointid_array.push(row.connectionpointid);
            meter_type_array.push(row.meter_type);
            importvalue_array
                .push({
                    let mut val = row.importvalue;
                    val.rescale(8);
                    val.mantissa()
                });
            exportvalue_array
                .push({
                    let mut val = row.exportvalue;
                    val.rescale(8);
                    val.mantissa()
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(case_id_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(meter_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(meter_id_suffix_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(frmp_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(lr_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(connectionpointid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(meter_type_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(importvalue_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(exportvalue_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
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
    /// Settlement date within the case
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Interconnector Identifier
    pub interconnectorid: String,
    /// Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// The import direction value for the meter read (MWh)
    pub importvalue: Option<rust_decimal::Decimal>,
    /// The export direction value for the meter read (MWh)
    pub exportvalue: Option<rust_decimal::Decimal>,
    /// Last changed date for the record
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for MeterdataInterconnector1 {
    type PrimaryKey = MeterdataInterconnector1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "METERDATA".into(),
            table_name: Some("INTERCONNECTOR".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MeterdataInterconnector1PrimaryKey {
        MeterdataInterconnector1PrimaryKey {
            case_id: self.case_id,
            interconnectorid: self.interconnectorid.clone(),
            periodid: self.periodid,
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
            "meterdata_interconnector_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MeterdataInterconnector1PrimaryKey {
    pub case_id: rust_decimal::Decimal,
    pub interconnectorid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MeterdataInterconnector1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MeterdataInterconnector1 {
    type Row = MeterdataInterconnector1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.case_id == row.case_id && self.interconnectorid == row.interconnectorid
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterdataInterconnector1 {
    type PrimaryKey = MeterdataInterconnector1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.interconnectorid == key.interconnectorid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for MeterdataInterconnector1PrimaryKey {
    type Row = MeterdataInterconnector1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.case_id == row.case_id && self.interconnectorid == row.interconnectorid
            && self.periodid == row.periodid && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterdataInterconnector1PrimaryKey {
    type PrimaryKey = MeterdataInterconnector1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.interconnectorid == key.interconnectorid
            && self.periodid == key.periodid && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MeterdataInterconnector1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("case_id",
                arrow2::datatypes::DataType::Decimal(15, 0), false),
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("importvalue",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("exportvalue",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
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
        let mut case_id_array = Vec::new();
        let mut settlementdate_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut importvalue_array = Vec::new();
        let mut exportvalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            case_id_array
                .push({
                    let mut val = row.case_id;
                    val.rescale(0);
                    val.mantissa()
                });
            settlementdate_array.push(row.settlementdate.timestamp());
            interconnectorid_array.push(row.interconnectorid);
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            importvalue_array
                .push({
                    row.importvalue
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            exportvalue_array
                .push({
                    row.exportvalue
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(case_id_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(interconnectorid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(importvalue_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(exportvalue_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
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
/// ## METERDATA_WDR_READS
///  _Metering Data WDR Readings_
///
/// * Data Set Name: Meterdata
/// * File Name: Wdr Reads
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
/// * MARKET_ID
/// * METER_ID
/// * PERIODID
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterdataWdrReads1 {
    /// Unique identifier for the market to which this metering record applies.  Always equal to NEM in the current system.
    pub market_id: String,
    /// Unique identifier for the metering case.
    pub case_id: rust_decimal::Decimal,
    /// The settlement date for the metering record
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Unique identifier for the meter to which the metering record applies
    pub meter_id: String,
    /// Unique identifier for the transmission node to which this meter belongs on the settlement date
    pub tni: Option<String>,
    /// Unique identifier for the participant acting as the FRMP for this NMI on the settlement date
    pub frmp: Option<String>,
    /// Unique identifier for the participant acting as the DRSP for this NMI on the settlement date
    pub drsp: Option<String>,
    /// Trading interval identifier, with Period 1 being the first TI for the calendar day, i.e interval ending 00:05.
    pub periodid: rust_decimal::Decimal,
    /// Metered quantity Import in MWh for the NMI in the trading interval.  A negative value indicates net consumption, while a positive value indicates net generation
    pub meteredquantityimport: Option<rust_decimal::Decimal>,
    /// Metered quantity Export in MWh for the NMI in the trading interval.  A negative value indicates net consumption, while a positive value indicates net generation
    pub meteredquantityexport: Option<rust_decimal::Decimal>,
    /// Baseline quantity in MWh for the NMI in the trading interval.  A negative value indicates net consumption, while a positive value indicates the net generation
    pub baselinequantity: Option<rust_decimal::Decimal>,
    /// Quality flag for the meter read.  Where multiple datastreams exist against the NMI with different quality flags for each read, the lowest quality flag will be published against the NMI for the interval.
    pub qualityflag: Option<String>,
    /// A value of TRUE (indicated by 1) for this column indicates that financial settlement of WDR transactions for this NMI should not proceed for the settlement date and trading interval. Possible values are 1 and 0.
    pub isnoncompliant: Option<rust_decimal::Decimal>,
    /// A reference to the baseline run that produced the baseline quantity for this NMI and interval
    pub baselinecalculationid: Option<String>,
}
impl mmsdm_core::GetTable for MeterdataWdrReads1 {
    type PrimaryKey = MeterdataWdrReads1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "METERDATA".into(),
            table_name: Some("WDR_READS".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> MeterdataWdrReads1PrimaryKey {
        MeterdataWdrReads1PrimaryKey {
            case_id: self.case_id,
            market_id: self.market_id.clone(),
            meter_id: self.meter_id.clone(),
            periodid: self.periodid,
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
            "meterdata_wdr_reads_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct MeterdataWdrReads1PrimaryKey {
    pub case_id: rust_decimal::Decimal,
    pub market_id: String,
    pub meter_id: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for MeterdataWdrReads1PrimaryKey {}
impl mmsdm_core::CompareWithRow for MeterdataWdrReads1 {
    type Row = MeterdataWdrReads1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.case_id == row.case_id && self.market_id == row.market_id
            && self.meter_id == row.meter_id && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterdataWdrReads1 {
    type PrimaryKey = MeterdataWdrReads1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.market_id == key.market_id
            && self.meter_id == key.meter_id && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for MeterdataWdrReads1PrimaryKey {
    type Row = MeterdataWdrReads1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.case_id == row.case_id && self.market_id == row.market_id
            && self.meter_id == row.meter_id && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for MeterdataWdrReads1PrimaryKey {
    type PrimaryKey = MeterdataWdrReads1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.case_id == key.case_id && self.market_id == key.market_id
            && self.meter_id == key.meter_id && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for MeterdataWdrReads1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("market_id",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("case_id",
                arrow2::datatypes::DataType::Decimal(15, 0), false),
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("meter_id",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("tni",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("frmp",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("drsp",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("periodid",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("meteredquantityimport",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("meteredquantityexport",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("baselinequantity",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("qualityflag",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("isnoncompliant",
                arrow2::datatypes::DataType::Decimal(1, 0), true),
                arrow2::datatypes::Field::new("baselinecalculationid",
                arrow2::datatypes::DataType::LargeUtf8, true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut market_id_array = Vec::new();
        let mut case_id_array = Vec::new();
        let mut settlementdate_array = Vec::new();
        let mut meter_id_array = Vec::new();
        let mut tni_array = Vec::new();
        let mut frmp_array = Vec::new();
        let mut drsp_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut meteredquantityimport_array = Vec::new();
        let mut meteredquantityexport_array = Vec::new();
        let mut baselinequantity_array = Vec::new();
        let mut qualityflag_array = Vec::new();
        let mut isnoncompliant_array = Vec::new();
        let mut baselinecalculationid_array = Vec::new();
        for row in partition {
            market_id_array.push(row.market_id);
            case_id_array
                .push({
                    let mut val = row.case_id;
                    val.rescale(0);
                    val.mantissa()
                });
            settlementdate_array.push(row.settlementdate.timestamp());
            meter_id_array.push(row.meter_id);
            tni_array.push(row.tni);
            frmp_array.push(row.frmp);
            drsp_array.push(row.drsp);
            periodid_array
                .push({
                    let mut val = row.periodid;
                    val.rescale(0);
                    val.mantissa()
                });
            meteredquantityimport_array
                .push({
                    row.meteredquantityimport
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            meteredquantityexport_array
                .push({
                    row.meteredquantityexport
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            baselinequantity_array
                .push({
                    row.baselinequantity
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            qualityflag_array.push(row.qualityflag);
            isnoncompliant_array
                .push({
                    row.isnoncompliant
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            baselinecalculationid_array.push(row.baselinecalculationid);
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(market_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(case_id_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(meter_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(tni_array)) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(frmp_array)) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(drsp_array)) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(meteredquantityimport_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(meteredquantityexport_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(baselinequantity_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(qualityflag_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(isnoncompliant_array)
                    .to(arrow2::datatypes::DataType::Decimal(1, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(baselinecalculationid_array)) as std::sync::Arc < dyn
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
        (Some("AGGREGATE_READS"), version) if version <= 1_i32 => {
            let d: Vec<MeterdataAggregateReads1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMeterdataAggregateReads1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("INDIVIDUAL_READS"), version) if version <= 1_i32 => {
            let d: Vec<MeterdataIndividualReads1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMeterdataIndividualReads1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("INTERCONNECTOR"), version) if version <= 1_i32 => {
            let d: Vec<MeterdataInterconnector1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMeterdataInterconnector1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("WDR_READS"), version) if version <= 1_i32 => {
            let d: Vec<MeterdataWdrReads1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertMeterdataWdrReads1 @P1, @P2",
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
