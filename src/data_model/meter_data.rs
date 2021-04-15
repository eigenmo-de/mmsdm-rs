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
    /// The settlement interval identifier
    pub periodid: rust_decimal::Decimal,
    /// The import(pool-centric) value for the meter read (MWh)
    pub importvalue: rust_decimal::Decimal,
    /// The export(pool-centric) value for the meter read (MWh)
    pub exportvalue: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MeterdataAggregateReads1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "METERDATA".into(),
                        table_name: Some("AGGREGATE_READS".into()),
                        version: 1,
                    }
                    
    }
}
/// # Summary
/// 
/// ## METERDATA_TRK
///  _Tracking table for the publication of wholesale settlement data associated with BILLING run_
/// 
/// * Data Set Name: Meterdata
/// * File Name: Trk
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MeterdataTrk1 {
    /// Case Identifier
    pub case_id: rust_decimal::Decimal,
    #[serde(with = "crate::mms_datetime_opt")]
    pub aggregate_reads_load_datetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub individual_reads_load_datetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MeterdataTrk1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "METERDATA".into(),
                        table_name: Some("TRK".into()),
                        version: 1,
                    }
                    
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
    /// The settlement interval identifier
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
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "METERDATA".into(),
                        table_name: Some("INDIVIDUAL_READS".into()),
                        version: 1,
                    }
                    
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
    /// The settlement interval identifier
    pub periodid: rust_decimal::Decimal,
    /// The import direction value for the meter read (MWh)
    pub importvalue: Option<rust_decimal::Decimal>,
    /// The export direction value for the meter read (MWh)
    pub exportvalue: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MeterdataInterconnector1 {
    fn get_file_key() -> crate::FileKey {

                    crate::FileKey {
                        data_set_name: "METERDATA".into(),
                        table_name: Some("INTERCONNECTOR".into()),
                        version: 1,
                    }
                    
    }
}
