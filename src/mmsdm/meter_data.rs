/// Data Set Name: Meterdata
/// File Name: Aggregate Reads
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterdataAggregateReads1 {
    /// Case Identifier
    case_id: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Connection Point ID
    connectionpointid: String,
    /// The meter type for the read, one of: CUSTOMER; GENERATOR; EMBEDDED_GENERATOR
    meter_type: String,
    /// The financially responsible market participantid
    frmp: String,
    /// The local retailer at the connection point id
    lr: String,
    /// The settlement interval identifier
    periodid: rust_decimal::Decimal,
    /// The import(pool-centric) value for the meter read (MWh)
    importvalue: rust_decimal::Decimal,
    /// The export(pool-centric) value for the meter read (MWh)
    exportvalue: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MeterdataAggregateReads1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "METERDATA".into(),
                        table_name: "AGGREGATE_READS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Meterdata
/// File Name: Trk
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterdataTrk1 {
    /// Case Identifier
    case_id: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    aggregate_reads_load_datetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    individual_reads_load_datetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    enddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MeterdataTrk1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "METERDATA".into(),
                        table_name: "TRK".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Meterdata
/// File Name: Individual Reads
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterdataIndividualReads1 {
    /// Case Identifier
    case_id: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// The National Metering Identifier (NMI)
    meter_id: String,
    /// The National Metering Identifier (NMI) data stream
    meter_id_suffix: String,
    /// The financially responsible market participantid
    frmp: String,
    /// The local retailer at the connection point id
    lr: String,
    /// The settlement interval identifier
    periodid: rust_decimal::Decimal,
    /// Connection Point ID
    connectionpointid: String,
    /// The meter type for the read, one of: CUSTOMER; GENERATOR; EMBEDDED_GENERATOR
    meter_type: String,
    /// The import(pool-centric) value for the meter read (MWh)
    importvalue: rust_decimal::Decimal,
    /// The export(pool-centric) value for the meter read (MWh)
    exportvalue: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MeterdataIndividualReads1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "METERDATA".into(),
                        table_name: "INDIVIDUAL_READS".into(),
                        version: 1,
                    }
                    
    }
}
/// Data Set Name: Meterdata
/// File Name: Interconnector
/// Data Version: 1
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterdataInterconnector1 {
    /// Case Identifier
    case_id: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime")]
    settlementdate: chrono::NaiveDateTime,
    /// Interconnector Identifier
    interconnectorid: String,
    /// The settlement interval identifier
    periodid: rust_decimal::Decimal,
    /// The import direction value for the meter read (MWh)
    importvalue: Option<rust_decimal::Decimal>,
    /// The export direction value for the meter read (MWh)
    exportvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable<MeterdataInterconnector1> for crate::AemoFile {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "METERDATA".into(),
                        table_name: "INTERCONNECTOR".into(),
                        version: 1,
                    }
                    
    }
}
