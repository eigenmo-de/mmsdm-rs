/// # Summary
///
/// ## ANCILLARY_RECOVERY_SPLIT
///  _ANCILLARY_RECOVERY_SPLIT holds the actual customer portion for each service and payment type. A single EFFECTIVEDATE/VERSIONNO combination applies to all services (i.e. the latest EFFECTIVEDATE/VERSIONNO is not retrieved for a single service, but applies to a data set)._
///
/// * Data Set Name: Settlement Config
/// * File Name: Ancillary Recovery Split
/// * Data Version: 1
///
/// # Description
///  ANCILLARY_RECOVERY_SPLIT is public data, and is available to all participants. Source This table is updated infrequently.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PAYMENTTYPE
/// * SERVICE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigAncillaryRecoverySplit1 {
    /// Calendar settlement date record becomes effective.
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number of the record for the given date.
    pub versionno: rust_decimal::Decimal,
    /// Ancillary service name (e.g. AGC, FCASCOMP)
    pub service: String,
    /// A payment type associated with the service (can be ENABLING, AVAILABILITY, USAGE, or COMPENSATION).
    pub paymenttype: String,
    /// The percentage value of the recovery funded by market customers.
    pub customer_portion: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigAncillaryRecoverySplit1 {
    type PrimaryKey = SettlementConfigAncillaryRecoverySplit1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("ANCILLARY_RECOVERY_SPLIT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementConfigAncillaryRecoverySplit1PrimaryKey {
        SettlementConfigAncillaryRecoverySplit1PrimaryKey {
            effectivedate: self.effectivedate,
            paymenttype: self.paymenttype.clone(),
            service: self.service.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "settlement_config_ancillary_recovery_split_v1".to_string()
    }
}
impl crate::CompareWithRow for SettlementConfigAncillaryRecoverySplit1 {
    type Row = SettlementConfigAncillaryRecoverySplit1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.paymenttype == row.paymenttype
            && self.service == row.service
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigAncillaryRecoverySplit1 {
    type PrimaryKey = SettlementConfigAncillaryRecoverySplit1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.paymenttype == key.paymenttype
            && self.service == key.service
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementConfigAncillaryRecoverySplit1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub paymenttype: String,
    pub service: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for SettlementConfigAncillaryRecoverySplit1PrimaryKey {
    type Row = SettlementConfigAncillaryRecoverySplit1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.paymenttype == row.paymenttype
            && self.service == row.service
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigAncillaryRecoverySplit1PrimaryKey {
    type PrimaryKey = SettlementConfigAncillaryRecoverySplit1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.paymenttype == key.paymenttype
            && self.service == key.service
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for SettlementConfigAncillaryRecoverySplit1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SettlementConfigAncillaryRecoverySplit1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("service", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "paymenttype",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "customer_portion",
                arrow2::datatypes::DataType::Decimal(8, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut service_array = Vec::new();
        let mut paymenttype_array = Vec::new();
        let mut customer_portion_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            service_array.push(row.service);
            paymenttype_array.push(row.paymenttype);
            customer_portion_array.push({
                row.customer_portion.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(service_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    paymenttype_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(customer_portion_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MARKETFEE
///  _MARKETFEE sets out fee type and period for each market fee._
///
/// * Data Set Name: Settlement Config
/// * File Name: Marketfee
/// * Data Version: 1
///
/// # Description
///  MARKETFEE data is public, so is available to all participants. Source MARKETFEE updates when fees change.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * MARKETFEEID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketfee1 {
    /// Identifier for Market Fee
    pub marketfeeid: String,
    /// Period type - PERIOD, DAILY, WEEKLY
    pub marketfeeperiod: Option<String>,
    /// Type - MW or $
    pub marketfeetype: Option<String>,
    /// Description of market fee
    pub description: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub gl_tcode: Option<String>,
    /// &nbsp;
    pub gl_financialcode: Option<String>,
    /// &nbsp;
    pub fee_class: Option<String>,
}
impl crate::GetTable for SettlementConfigMarketfee1 {
    type PrimaryKey = SettlementConfigMarketfee1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("MARKETFEE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementConfigMarketfee1PrimaryKey {
        SettlementConfigMarketfee1PrimaryKey {
            marketfeeid: self.marketfeeid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "settlement_config_marketfee_v1".to_string()
    }
}
impl crate::CompareWithRow for SettlementConfigMarketfee1 {
    type Row = SettlementConfigMarketfee1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.marketfeeid == row.marketfeeid
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigMarketfee1 {
    type PrimaryKey = SettlementConfigMarketfee1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.marketfeeid == key.marketfeeid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementConfigMarketfee1PrimaryKey {
    pub marketfeeid: String,
}
impl crate::CompareWithRow for SettlementConfigMarketfee1PrimaryKey {
    type Row = SettlementConfigMarketfee1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.marketfeeid == row.marketfeeid
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigMarketfee1PrimaryKey {
    type PrimaryKey = SettlementConfigMarketfee1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.marketfeeid == key.marketfeeid
    }
}
impl crate::PrimaryKey for SettlementConfigMarketfee1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SettlementConfigMarketfee1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "marketfeeid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "marketfeeperiod",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "marketfeetype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("gl_tcode", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "gl_financialcode",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "fee_class",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut marketfeeid_array = Vec::new();
        let mut marketfeeperiod_array = Vec::new();
        let mut marketfeetype_array = Vec::new();
        let mut description_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut gl_tcode_array = Vec::new();
        let mut gl_financialcode_array = Vec::new();
        let mut fee_class_array = Vec::new();
        for (_, row) in partition {
            marketfeeid_array.push(row.marketfeeid);
            marketfeeperiod_array.push(row.marketfeeperiod);
            marketfeetype_array.push(row.marketfeetype);
            description_array.push(row.description);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            gl_tcode_array.push(row.gl_tcode);
            gl_financialcode_array.push(row.gl_financialcode);
            fee_class_array.push(row.fee_class);
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    marketfeeid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(marketfeeperiod_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(marketfeetype_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(gl_tcode_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    gl_financialcode_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(fee_class_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MARKETFEEDATA
///  _MARKETFEEDATA sets out actual fee rates, as adjusted from time to time._
///
/// * Data Set Name: Settlement Config
/// * File Name: Marketfeedata
/// * Data Version: 1
///
/// # Description
///  MARKETFEEDATA is public data, and is available to all participants. Source MARKETFEEDATA updates whenever fee rates change.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEID
/// * MARKETFEEVERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketfeedata1 {
    /// Identifier for Market Fee
    pub marketfeeid: String,
    /// Version of fees for this id
    pub marketfeeversionno: rust_decimal::Decimal,
    /// Date on which this data becomes effective
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Market fee rate/MWh, a dollar amount
    pub marketfeevalue: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigMarketfeedata1 {
    type PrimaryKey = SettlementConfigMarketfeedata1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("MARKETFEEDATA".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementConfigMarketfeedata1PrimaryKey {
        SettlementConfigMarketfeedata1PrimaryKey {
            effectivedate: self.effectivedate,
            marketfeeid: self.marketfeeid.clone(),
            marketfeeversionno: self.marketfeeversionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "settlement_config_marketfeedata_v1".to_string()
    }
}
impl crate::CompareWithRow for SettlementConfigMarketfeedata1 {
    type Row = SettlementConfigMarketfeedata1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeid == row.marketfeeid
            && self.marketfeeversionno == row.marketfeeversionno
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigMarketfeedata1 {
    type PrimaryKey = SettlementConfigMarketfeedata1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.marketfeeid == key.marketfeeid
            && self.marketfeeversionno == key.marketfeeversionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementConfigMarketfeedata1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub marketfeeid: String,
    pub marketfeeversionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for SettlementConfigMarketfeedata1PrimaryKey {
    type Row = SettlementConfigMarketfeedata1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeid == row.marketfeeid
            && self.marketfeeversionno == row.marketfeeversionno
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigMarketfeedata1PrimaryKey {
    type PrimaryKey = SettlementConfigMarketfeedata1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.marketfeeid == key.marketfeeid
            && self.marketfeeversionno == key.marketfeeversionno
    }
}
impl crate::PrimaryKey for SettlementConfigMarketfeedata1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SettlementConfigMarketfeedata1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "marketfeeid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "marketfeeversionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "marketfeevalue",
                arrow2::datatypes::DataType::Decimal(22, 8),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut marketfeeid_array = Vec::new();
        let mut marketfeeversionno_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut marketfeevalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            marketfeeid_array.push(row.marketfeeid);
            marketfeeversionno_array.push({
                let mut val = row.marketfeeversionno;
                val.rescale(0);
                val.mantissa()
            });
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            marketfeevalue_array.push({
                row.marketfeevalue.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    marketfeeid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(marketfeeversionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marketfeevalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MARKETFEETRK
///  _MARKETFEETRK sets out versions of each market fee used and its effective date._
///
/// * Data Set Name: Settlement Config
/// * File Name: Marketfeetrk
/// * Data Version: 1
///
/// # Description
///  MARKETFEETRK data is public, so is available to all participants. Source MARKETFEETRK updated infrequently, when new annual rates must be inserted. Volume One record inserted per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEVERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketfeetrk1 {
    /// Version of fees for this ID
    pub marketfeeversionno: rust_decimal::Decimal,
    /// Effective Date of Market notice
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// User authorising record
    pub authorisedby: Option<String>,
    /// Date record authorised
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigMarketfeetrk1 {
    type PrimaryKey = SettlementConfigMarketfeetrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("MARKETFEETRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementConfigMarketfeetrk1PrimaryKey {
        SettlementConfigMarketfeetrk1PrimaryKey {
            effectivedate: self.effectivedate,
            marketfeeversionno: self.marketfeeversionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "settlement_config_marketfeetrk_v1".to_string()
    }
}
impl crate::CompareWithRow for SettlementConfigMarketfeetrk1 {
    type Row = SettlementConfigMarketfeetrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate && self.marketfeeversionno == row.marketfeeversionno
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigMarketfeetrk1 {
    type PrimaryKey = SettlementConfigMarketfeetrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.marketfeeversionno == key.marketfeeversionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementConfigMarketfeetrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub marketfeeversionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for SettlementConfigMarketfeetrk1PrimaryKey {
    type Row = SettlementConfigMarketfeetrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate && self.marketfeeversionno == row.marketfeeversionno
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigMarketfeetrk1PrimaryKey {
    type PrimaryKey = SettlementConfigMarketfeetrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.marketfeeversionno == key.marketfeeversionno
    }
}
impl crate::PrimaryKey for SettlementConfigMarketfeetrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SettlementConfigMarketfeetrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "marketfeeversionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut marketfeeversionno_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            marketfeeversionno_array.push({
                let mut val = row.marketfeeversionno;
                val.rescale(0);
                val.mantissa()
            });
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp_millis()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(marketfeeversionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MARKET_FEE_CAT_EXCL
///  _Market fee exclusions for participant categories. _
///
/// * Data Set Name: Settlement Config
/// * File Name: Market Fee Cat Excl
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEID
/// * PARTICIPANT_CATEGORYID
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketFeeCatExcl1 {
    /// The excluded market fee
    pub marketfeeid: String,
    /// The date the exclusion is effective from
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// The version information for this record
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// Participant category to be excluded from this market fee
    pub participant_categoryid: String,
}
impl crate::GetTable for SettlementConfigMarketFeeCatExcl1 {
    type PrimaryKey = SettlementConfigMarketFeeCatExcl1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("MARKET_FEE_CAT_EXCL".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementConfigMarketFeeCatExcl1PrimaryKey {
        SettlementConfigMarketFeeCatExcl1PrimaryKey {
            effectivedate: self.effectivedate,
            marketfeeid: self.marketfeeid.clone(),
            participant_categoryid: self.participant_categoryid.clone(),
            version_datetime: self.version_datetime,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "settlement_config_market_fee_cat_excl_v1".to_string()
    }
}
impl crate::CompareWithRow for SettlementConfigMarketFeeCatExcl1 {
    type Row = SettlementConfigMarketFeeCatExcl1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeid == row.marketfeeid
            && self.participant_categoryid == row.participant_categoryid
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigMarketFeeCatExcl1 {
    type PrimaryKey = SettlementConfigMarketFeeCatExcl1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.marketfeeid == key.marketfeeid
            && self.participant_categoryid == key.participant_categoryid
            && self.version_datetime == key.version_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementConfigMarketFeeCatExcl1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub marketfeeid: String,
    pub participant_categoryid: String,
    pub version_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for SettlementConfigMarketFeeCatExcl1PrimaryKey {
    type Row = SettlementConfigMarketFeeCatExcl1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeid == row.marketfeeid
            && self.participant_categoryid == row.participant_categoryid
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigMarketFeeCatExcl1PrimaryKey {
    type PrimaryKey = SettlementConfigMarketFeeCatExcl1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.marketfeeid == key.marketfeeid
            && self.participant_categoryid == key.participant_categoryid
            && self.version_datetime == key.version_datetime
    }
}
impl crate::PrimaryKey for SettlementConfigMarketFeeCatExcl1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SettlementConfigMarketFeeCatExcl1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "marketfeeid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "version_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participant_categoryid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut marketfeeid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut participant_categoryid_array = Vec::new();
        for (_, row) in partition {
            marketfeeid_array.push(row.marketfeeid);
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            version_datetime_array.push(row.version_datetime.timestamp_millis());
            participant_categoryid_array.push(row.participant_categoryid);
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    marketfeeid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(version_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participant_categoryid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MARKET_FEE_CAT_EXCL_TRK
///  _Tracking table for market fee exclusions for participant categories._
///
/// * Data Set Name: Settlement Config
/// * File Name: Market Fee Cat Excl Trk
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEID
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketFeeCatExclTrk1 {
    /// The excluded market fee
    pub marketfeeid: String,
    /// The date the exclusion is effective from
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// The version information for this record
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// Last date and time the record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigMarketFeeCatExclTrk1 {
    type PrimaryKey = SettlementConfigMarketFeeCatExclTrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("MARKET_FEE_CAT_EXCL_TRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementConfigMarketFeeCatExclTrk1PrimaryKey {
        SettlementConfigMarketFeeCatExclTrk1PrimaryKey {
            effectivedate: self.effectivedate,
            marketfeeid: self.marketfeeid.clone(),
            version_datetime: self.version_datetime,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "settlement_config_market_fee_cat_excl_trk_v1".to_string()
    }
}
impl crate::CompareWithRow for SettlementConfigMarketFeeCatExclTrk1 {
    type Row = SettlementConfigMarketFeeCatExclTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeid == row.marketfeeid
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigMarketFeeCatExclTrk1 {
    type PrimaryKey = SettlementConfigMarketFeeCatExclTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.marketfeeid == key.marketfeeid
            && self.version_datetime == key.version_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementConfigMarketFeeCatExclTrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub marketfeeid: String,
    pub version_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for SettlementConfigMarketFeeCatExclTrk1PrimaryKey {
    type Row = SettlementConfigMarketFeeCatExclTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeid == row.marketfeeid
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigMarketFeeCatExclTrk1PrimaryKey {
    type PrimaryKey = SettlementConfigMarketFeeCatExclTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.marketfeeid == key.marketfeeid
            && self.version_datetime == key.version_datetime
    }
}
impl crate::PrimaryKey for SettlementConfigMarketFeeCatExclTrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SettlementConfigMarketFeeCatExclTrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "marketfeeid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "version_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut marketfeeid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            marketfeeid_array.push(row.marketfeeid);
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            version_datetime_array.push(row.version_datetime.timestamp_millis());
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    marketfeeid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(version_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MARKET_FEE_EXCLUSION
///  _MARKET_FEE_EXCLUSION shows the list of market fees from which a participant is excluded from funding after a particular settlement date._
///
/// * Data Set Name: Settlement Config
/// * File Name: Market Fee Exclusion
/// * Data Version: 1
///
/// # Description
///  MARKET_FEE_EXCLUSION data is confidential to the relevant participant. Source MARKET_FEE_EXCLUSION updates only on change of participant configuration.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketFeeExclusion1 {
    /// Unique participant identifier
    pub participantid: String,
    /// Date on which this data becomes effective
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of fees for this ID
    pub versionno: rust_decimal::Decimal,
    /// Identifier for Market Fee
    pub marketfeeid: String,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigMarketFeeExclusion1 {
    type PrimaryKey = SettlementConfigMarketFeeExclusion1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("MARKET_FEE_EXCLUSION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementConfigMarketFeeExclusion1PrimaryKey {
        SettlementConfigMarketFeeExclusion1PrimaryKey {
            effectivedate: self.effectivedate,
            marketfeeid: self.marketfeeid.clone(),
            participantid: self.participantid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "settlement_config_market_fee_exclusion_v1".to_string()
    }
}
impl crate::CompareWithRow for SettlementConfigMarketFeeExclusion1 {
    type Row = SettlementConfigMarketFeeExclusion1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeid == row.marketfeeid
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigMarketFeeExclusion1 {
    type PrimaryKey = SettlementConfigMarketFeeExclusion1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.marketfeeid == key.marketfeeid
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementConfigMarketFeeExclusion1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub marketfeeid: String,
    pub participantid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for SettlementConfigMarketFeeExclusion1PrimaryKey {
    type Row = SettlementConfigMarketFeeExclusion1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeid == row.marketfeeid
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigMarketFeeExclusion1PrimaryKey {
    type PrimaryKey = SettlementConfigMarketFeeExclusion1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.marketfeeid == key.marketfeeid
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for SettlementConfigMarketFeeExclusion1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SettlementConfigMarketFeeExclusion1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "marketfeeid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut participantid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut marketfeeid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            participantid_array.push(row.participantid);
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            marketfeeid_array.push(row.marketfeeid);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    marketfeeid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## MARKET_FEE_EXCLUSIONTRK
///  _MARKET_FEE_EXCLUSIONTRK shows authorisation details of participant market fee exclusion data sets._
///
/// * Data Set Name: Settlement Config
/// * File Name: Market Fee Exclusion Trk
/// * Data Version: 1
///
/// # Description
///  MARKET_FEE_EXCLUSIONTRK is confidential to the participant. Source MARKET_FEE_EXCLUSIONTRK updates only on change of participant configuration.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigMarketFeeExclusionTrk1 {
    /// Unique participant identifier
    pub participantid: String,
    /// Date on which this data becomes effective
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version of fees for this ID
    pub versionno: rust_decimal::Decimal,
    /// User authorising record
    pub authorisedby: Option<String>,
    /// Date record authorised
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigMarketFeeExclusionTrk1 {
    type PrimaryKey = SettlementConfigMarketFeeExclusionTrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("MARKET_FEE_EXCLUSION_TRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementConfigMarketFeeExclusionTrk1PrimaryKey {
        SettlementConfigMarketFeeExclusionTrk1PrimaryKey {
            effectivedate: self.effectivedate,
            participantid: self.participantid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "settlement_config_market_fee_exclusion_trk_v1".to_string()
    }
}
impl crate::CompareWithRow for SettlementConfigMarketFeeExclusionTrk1 {
    type Row = SettlementConfigMarketFeeExclusionTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigMarketFeeExclusionTrk1 {
    type PrimaryKey = SettlementConfigMarketFeeExclusionTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementConfigMarketFeeExclusionTrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for SettlementConfigMarketFeeExclusionTrk1PrimaryKey {
    type Row = SettlementConfigMarketFeeExclusionTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigMarketFeeExclusionTrk1PrimaryKey {
    type PrimaryKey = SettlementConfigMarketFeeExclusionTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for SettlementConfigMarketFeeExclusionTrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SettlementConfigMarketFeeExclusionTrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut participantid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            participantid_array.push(row.participantid);
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp_millis()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## PARTICIPANT_BANDFEE_ALLOC
///  _PARTICIPANT_BANDFEE_ALLOC shows the market fee for each Participant/Participant Category over time._
///
/// * Data Set Name: Settlement Config
/// * File Name: Participant Bandfee Alloc
/// * Data Version: 1
///
/// # Description
///  Source This view updates only on change of participant configuration.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * MARKETFEEID
/// * PARTICIPANTCATEGORYID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigParticipantBandfeeAlloc1 {
    /// Unique participant identifier
    pub participantid: String,
    /// Identifier for Market Fee
    pub marketfeeid: String,
    /// Date on which this data becomes effective.
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Period identifier
    pub versionno: rust_decimal::Decimal,
    /// The participant category that the market fee recovery amount pertains to.
    pub participantcategoryid: String,
    /// The value of this market fee
    pub marketfeevalue: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigParticipantBandfeeAlloc1 {
    type PrimaryKey = SettlementConfigParticipantBandfeeAlloc1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("PARTICIPANT_BANDFEE_ALLOC".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementConfigParticipantBandfeeAlloc1PrimaryKey {
        SettlementConfigParticipantBandfeeAlloc1PrimaryKey {
            effectivedate: self.effectivedate,
            marketfeeid: self.marketfeeid.clone(),
            participantcategoryid: self.participantcategoryid.clone(),
            participantid: self.participantid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "settlement_config_participant_bandfee_alloc_v1".to_string()
    }
}
impl crate::CompareWithRow for SettlementConfigParticipantBandfeeAlloc1 {
    type Row = SettlementConfigParticipantBandfeeAlloc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeid == row.marketfeeid
            && self.participantcategoryid == row.participantcategoryid
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigParticipantBandfeeAlloc1 {
    type PrimaryKey = SettlementConfigParticipantBandfeeAlloc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.marketfeeid == key.marketfeeid
            && self.participantcategoryid == key.participantcategoryid
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementConfigParticipantBandfeeAlloc1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub marketfeeid: String,
    pub participantcategoryid: String,
    pub participantid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for SettlementConfigParticipantBandfeeAlloc1PrimaryKey {
    type Row = SettlementConfigParticipantBandfeeAlloc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.marketfeeid == row.marketfeeid
            && self.participantcategoryid == row.participantcategoryid
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigParticipantBandfeeAlloc1PrimaryKey {
    type PrimaryKey = SettlementConfigParticipantBandfeeAlloc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.marketfeeid == key.marketfeeid
            && self.participantcategoryid == key.participantcategoryid
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for SettlementConfigParticipantBandfeeAlloc1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SettlementConfigParticipantBandfeeAlloc1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "marketfeeid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantcategoryid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "marketfeevalue",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut participantid_array = Vec::new();
        let mut marketfeeid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantcategoryid_array = Vec::new();
        let mut marketfeevalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            participantid_array.push(row.participantid);
            marketfeeid_array.push(row.marketfeeid);
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            participantcategoryid_array.push(row.participantcategoryid);
            marketfeevalue_array.push({
                row.marketfeevalue.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    marketfeeid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantcategoryid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marketfeevalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## REALLOCATION
///  _The REALLOCATION table shows the financial transactions agreed between two participants that are settled through the AEMO pool settlements process._
///
/// * Data Set Name: Setcfg
/// * File Name: Reallocation
/// * Data Version: 2
///
/// # Description
///  Note The column REALLOCATION_TYPE can be used in conjunction with CREDITPARTICIPANT or DEBITPARTICIPANT to determine who submitted a reallocation.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * REALLOCATIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SetcfgReallocation2 {
    /// Reallocation identifier
    pub reallocationid: String,
    /// The participant to be credited for the reallocation
    pub creditparticipantid: Option<String>,
    /// The participant to be debited for the reallocation
    pub debitparticipantid: Option<String>,
    /// Region identifier, being the spot price reference node for this reallocation
    pub regionid: Option<String>,
    /// $, (Quantity) Mwh, SWAP, CAP or FLOOR
    pub agreementtype: Option<String>,
    /// Optional reference detail for credit participant
    pub creditreference: Option<String>,
    /// Optional reference detail for debit participant
    pub debitreference: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// First day of the Reallocation contract
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Last day of the Reallocation contract
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Reallocation state. One of SUBMITTED, AUTHORISED, CANCELLED.
    pub current_stepid: Option<String>,
    /// The day type profile for which the reallocation applies over the start and end date range. Valid entries are BUSINESS, NON_BUSINESS or FLAT.
    pub daytype: Option<String>,
    /// Denotes a Credit or Debit reallocation with a value of "C" or "D" respectively
    pub reallocation_type: Option<String>,
    /// Unique ID of the calendar for which data is requested
    pub calendarid: Option<String>,
    /// The length of settlement intervals (in minutes) in the reallocation profile
    pub intervallength: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SetcfgReallocation2 {
    type PrimaryKey = SetcfgReallocation2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETCFG".into(),
            table_name: Some("REALLOCATION".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> SetcfgReallocation2PrimaryKey {
        SetcfgReallocation2PrimaryKey {
            reallocationid: self.reallocationid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "setcfg_reallocation_v2".to_string()
    }
}
impl crate::CompareWithRow for SetcfgReallocation2 {
    type Row = SetcfgReallocation2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.reallocationid == row.reallocationid
    }
}
impl crate::CompareWithPrimaryKey for SetcfgReallocation2 {
    type PrimaryKey = SetcfgReallocation2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.reallocationid == key.reallocationid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SetcfgReallocation2PrimaryKey {
    pub reallocationid: String,
}
impl crate::CompareWithRow for SetcfgReallocation2PrimaryKey {
    type Row = SetcfgReallocation2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.reallocationid == row.reallocationid
    }
}
impl crate::CompareWithPrimaryKey for SetcfgReallocation2PrimaryKey {
    type PrimaryKey = SetcfgReallocation2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.reallocationid == key.reallocationid
    }
}
impl crate::PrimaryKey for SetcfgReallocation2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SetcfgReallocation2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "reallocationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "creditparticipantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "debitparticipantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "agreementtype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "creditreference",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "debitreference",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "current_stepid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("daytype", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "reallocation_type",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "calendarid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "intervallength",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut reallocationid_array = Vec::new();
        let mut creditparticipantid_array = Vec::new();
        let mut debitparticipantid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut agreementtype_array = Vec::new();
        let mut creditreference_array = Vec::new();
        let mut debitreference_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut current_stepid_array = Vec::new();
        let mut daytype_array = Vec::new();
        let mut reallocation_type_array = Vec::new();
        let mut calendarid_array = Vec::new();
        let mut intervallength_array = Vec::new();
        for (_, row) in partition {
            reallocationid_array.push(row.reallocationid);
            creditparticipantid_array.push(row.creditparticipantid);
            debitparticipantid_array.push(row.debitparticipantid);
            regionid_array.push(row.regionid);
            agreementtype_array.push(row.agreementtype);
            creditreference_array.push(row.creditreference);
            debitreference_array.push(row.debitreference);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            startdate_array.push(row.startdate.map(|val| val.timestamp_millis()));
            enddate_array.push(row.enddate.map(|val| val.timestamp_millis()));
            current_stepid_array.push(row.current_stepid);
            daytype_array.push(row.daytype);
            reallocation_type_array.push(row.reallocation_type);
            calendarid_array.push(row.calendarid);
            intervallength_array.push({
                row.intervallength.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    reallocationid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    creditparticipantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    debitparticipantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(agreementtype_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(creditreference_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(debitreference_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enddate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(current_stepid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(daytype_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    reallocation_type_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(calendarid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(intervallength_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## REALLOCATIONINTERVAL
///  _30-minute or (5-minute for 5MS) data comprising a single reallocation transaction._
///
/// * Data Set Name: Setcfg
/// * File Name: Reallocationinterval
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * PERIODID
/// * REALLOCATIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SetcfgReallocationinterval1 {
    /// Reallocation identifier
    pub reallocationid: String,
    /// Trading Interval
    pub periodid: i64,
    /// Reallocation value in the units of the agreement type
    pub value: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Nominated Reallocation Price, only used in agreement types of SWAP, CAP and FLOOR, being the contract strike price in $/MWh
    pub nrp: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for SetcfgReallocationinterval1 {
    type PrimaryKey = SetcfgReallocationinterval1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETCFG".into(),
            table_name: Some("REALLOCATIONINTERVAL".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SetcfgReallocationinterval1PrimaryKey {
        SetcfgReallocationinterval1PrimaryKey {
            periodid: self.periodid,
            reallocationid: self.reallocationid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "setcfg_reallocationinterval_v1".to_string()
    }
}
impl crate::CompareWithRow for SetcfgReallocationinterval1 {
    type Row = SetcfgReallocationinterval1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.periodid == row.periodid && self.reallocationid == row.reallocationid
    }
}
impl crate::CompareWithPrimaryKey for SetcfgReallocationinterval1 {
    type PrimaryKey = SetcfgReallocationinterval1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.reallocationid == key.reallocationid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SetcfgReallocationinterval1PrimaryKey {
    pub periodid: i64,
    pub reallocationid: String,
}
impl crate::CompareWithRow for SetcfgReallocationinterval1PrimaryKey {
    type Row = SetcfgReallocationinterval1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.periodid == row.periodid && self.reallocationid == row.reallocationid
    }
}
impl crate::CompareWithPrimaryKey for SetcfgReallocationinterval1PrimaryKey {
    type PrimaryKey = SetcfgReallocationinterval1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid && self.reallocationid == key.reallocationid
    }
}
impl crate::PrimaryKey for SetcfgReallocationinterval1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SetcfgReallocationinterval1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "reallocationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "value",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("nrp", arrow2::datatypes::DataType::Decimal(15, 5), true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut reallocationid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut value_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut nrp_array = Vec::new();
        for (_, row) in partition {
            reallocationid_array.push(row.reallocationid);
            periodid_array.push(row.periodid);
            value_array.push({
                row.value.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            nrp_array.push({
                row.nrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    reallocationid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(value_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(nrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETCFG_PARTICIPANT_MPF
///  _SETCFG_PARTICIPANT_MPF shows the Market Participation Factors (MPF) for each participant for each connection point. The MPF values are used to determine recovery amounts for regulation FCAS._
///
/// * Data Set Name: Settlement Config
/// * File Name: Setcfg Participant Mpf
/// * Data Version: 1
///
/// # Description
///  SETCFG_PARTICIPANT_MPF data is available to all participants. Volume Approximately 20,000 records per year
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONNECTIONPOINTID
/// * EFFECTIVEDATE
/// * PARTICIPANTCATEGORYID
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigSetcfgParticipantMpf1 {
    /// Participant identifier
    pub participantid: String,
    /// Effective date of the MPF data
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number of the MPF data
    pub versionno: rust_decimal::Decimal,
    /// Participant Category
    pub participantcategoryid: String,
    /// Connection point identifier
    pub connectionpointid: String,
    /// Market Participation Factor
    pub mpf: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigSetcfgParticipantMpf1 {
    type PrimaryKey = SettlementConfigSetcfgParticipantMpf1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("SETCFG_PARTICIPANT_MPF".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementConfigSetcfgParticipantMpf1PrimaryKey {
        SettlementConfigSetcfgParticipantMpf1PrimaryKey {
            connectionpointid: self.connectionpointid.clone(),
            effectivedate: self.effectivedate,
            participantcategoryid: self.participantcategoryid.clone(),
            participantid: self.participantid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "settlement_config_setcfg_participant_mpf_v1".to_string()
    }
}
impl crate::CompareWithRow for SettlementConfigSetcfgParticipantMpf1 {
    type Row = SettlementConfigSetcfgParticipantMpf1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.connectionpointid == row.connectionpointid
            && self.effectivedate == row.effectivedate
            && self.participantcategoryid == row.participantcategoryid
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigSetcfgParticipantMpf1 {
    type PrimaryKey = SettlementConfigSetcfgParticipantMpf1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid == key.connectionpointid
            && self.effectivedate == key.effectivedate
            && self.participantcategoryid == key.participantcategoryid
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementConfigSetcfgParticipantMpf1PrimaryKey {
    pub connectionpointid: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub participantcategoryid: String,
    pub participantid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for SettlementConfigSetcfgParticipantMpf1PrimaryKey {
    type Row = SettlementConfigSetcfgParticipantMpf1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.connectionpointid == row.connectionpointid
            && self.effectivedate == row.effectivedate
            && self.participantcategoryid == row.participantcategoryid
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigSetcfgParticipantMpf1PrimaryKey {
    type PrimaryKey = SettlementConfigSetcfgParticipantMpf1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid == key.connectionpointid
            && self.effectivedate == key.effectivedate
            && self.participantcategoryid == key.participantcategoryid
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for SettlementConfigSetcfgParticipantMpf1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SettlementConfigSetcfgParticipantMpf1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantcategoryid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("mpf", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut participantid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantcategoryid_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut mpf_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            participantid_array.push(row.participantid);
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            participantcategoryid_array.push(row.participantcategoryid);
            connectionpointid_array.push(row.connectionpointid);
            mpf_array.push({
                row.mpf.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantcategoryid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    connectionpointid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mpf_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETCFG_PARTICIPANT_MPFTRK
///  _SETCFG_PARTICIPANT_MPFTRK is the tracking table for Market Participation Factors (MPF) data stored in the SETCFG_PARTICIPANT_MPF table for each participant._
///
/// * Data Set Name: Settlement Config
/// * File Name: Setcfg Participant Mpftrk
/// * Data Version: 1
///
/// # Description
///  SETCFG_PARTICIPANT_MPFTRK data is public, so is available to all participants. Volume Approximately 2,000 records per year
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PARTICIPANTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementConfigSetcfgParticipantMpftrk1 {
    /// Participant identifier
    pub participantid: String,
    /// Effective date of the MPF data
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number of the MPF data
    pub versionno: rust_decimal::Decimal,
    /// Authorising user
    pub authorisedby: Option<String>,
    /// Authorised date and time
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementConfigSetcfgParticipantMpftrk1 {
    type PrimaryKey = SettlementConfigSetcfgParticipantMpftrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENT_CONFIG".into(),
            table_name: Some("SETCFG_PARTICIPANT_MPFTRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementConfigSetcfgParticipantMpftrk1PrimaryKey {
        SettlementConfigSetcfgParticipantMpftrk1PrimaryKey {
            effectivedate: self.effectivedate,
            participantid: self.participantid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "settlement_config_setcfg_participant_mpftrk_v1".to_string()
    }
}
impl crate::CompareWithRow for SettlementConfigSetcfgParticipantMpftrk1 {
    type Row = SettlementConfigSetcfgParticipantMpftrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigSetcfgParticipantMpftrk1 {
    type PrimaryKey = SettlementConfigSetcfgParticipantMpftrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementConfigSetcfgParticipantMpftrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub participantid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for SettlementConfigSetcfgParticipantMpftrk1PrimaryKey {
    type Row = SettlementConfigSetcfgParticipantMpftrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.participantid == row.participantid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SettlementConfigSetcfgParticipantMpftrk1PrimaryKey {
    type PrimaryKey = SettlementConfigSetcfgParticipantMpftrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.participantid == key.participantid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for SettlementConfigSetcfgParticipantMpftrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SettlementConfigSetcfgParticipantMpftrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut participantid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            participantid_array.push(row.participantid);
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp_millis()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETCFG_WDRRR_CALENDAR
///  _Wholesale Demand Response Reimbursement Rate Calendar_
///
/// * Data Set Name: Settlements Config
/// * File Name: Wdrrr Calendar
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * REGIONID
/// * VERSION_DATETIME
/// * WDRRRPERIOD
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsConfigWdrrrCalendar1 {
    /// Unique identifier for the period to which the WDRRR applies. For quarter-based periods, this will be equal to YYYY[Q]NN, for example,2020Q3 for 2020 Quarter 3.
    pub wdrrrperiod: String,
    /// Unique Identifier for the region id
    pub regionid: String,
    /// The Version Date time of the latest changes.
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// Start Date of Period (Inclusive).
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// End Date of Period (Inclusive).
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Last changed date for the record.
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsConfigWdrrrCalendar1 {
    type PrimaryKey = SettlementsConfigWdrrrCalendar1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENTS_CONFIG".into(),
            table_name: Some("WDRRR_CALENDAR".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementsConfigWdrrrCalendar1PrimaryKey {
        SettlementsConfigWdrrrCalendar1PrimaryKey {
            regionid: self.regionid.clone(),
            version_datetime: self.version_datetime,
            wdrrrperiod: self.wdrrrperiod.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "settlements_config_wdrrr_calendar_v1".to_string()
    }
}
impl crate::CompareWithRow for SettlementsConfigWdrrrCalendar1 {
    type Row = SettlementsConfigWdrrrCalendar1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.regionid == row.regionid
            && self.version_datetime == row.version_datetime
            && self.wdrrrperiod == row.wdrrrperiod
    }
}
impl crate::CompareWithPrimaryKey for SettlementsConfigWdrrrCalendar1 {
    type PrimaryKey = SettlementsConfigWdrrrCalendar1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid
            && self.version_datetime == key.version_datetime
            && self.wdrrrperiod == key.wdrrrperiod
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsConfigWdrrrCalendar1PrimaryKey {
    pub regionid: String,
    pub version_datetime: chrono::NaiveDateTime,
    pub wdrrrperiod: String,
}
impl crate::CompareWithRow for SettlementsConfigWdrrrCalendar1PrimaryKey {
    type Row = SettlementsConfigWdrrrCalendar1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.regionid == row.regionid
            && self.version_datetime == row.version_datetime
            && self.wdrrrperiod == row.wdrrrperiod
    }
}
impl crate::CompareWithPrimaryKey for SettlementsConfigWdrrrCalendar1PrimaryKey {
    type PrimaryKey = SettlementsConfigWdrrrCalendar1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid
            && self.version_datetime == key.version_datetime
            && self.wdrrrperiod == key.wdrrrperiod
    }
}
impl crate::PrimaryKey for SettlementsConfigWdrrrCalendar1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SettlementsConfigWdrrrCalendar1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "wdrrrperiod",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "version_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut wdrrrperiod_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            wdrrrperiod_array.push(row.wdrrrperiod);
            regionid_array.push(row.regionid);
            version_datetime_array.push(row.version_datetime.timestamp_millis());
            startdate_array.push(row.startdate.map(|val| val.timestamp_millis()));
            enddate_array.push(row.enddate.map(|val| val.timestamp_millis()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    wdrrrperiod_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(version_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enddate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETCFG_WDR_REIMBURSE_RATE
///  _Settlements WDR transactions_
///
/// * Data Set Name: Settlements Config
/// * File Name: Wdr Reimburse Rate
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * REGIONID
/// * VERSION_DATETIME
/// * WDRRRPERIOD
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsConfigWdrReimburseRate1 {
    /// Unique identifier for the period to which the WDRRR applies. For quarter-based periods, this will be equal to YYYY[Q]NN, e.g. 2020Q3 for 2020 Quarter 3.
    pub wdrrrperiod: String,
    /// Unique identifier for the region
    pub regionid: String,
    /// The Version Date time of the latest changes.
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// WDRRR value for the period and region ($/MWh)
    pub wdrrr: Option<rust_decimal::Decimal>,
    /// A flag to indicate that the WDRRR value is FIRM for the period and region, i.e. it is based on a complete set of firm prices from dispatch. Possible Values are 1 and 0
    pub isfirm: Option<rust_decimal::Decimal>,
    /// Last changed date for the record
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SettlementsConfigWdrReimburseRate1 {
    type PrimaryKey = SettlementsConfigWdrReimburseRate1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SETTLEMENTS_CONFIG".into(),
            table_name: Some("WDR_REIMBURSE_RATE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementsConfigWdrReimburseRate1PrimaryKey {
        SettlementsConfigWdrReimburseRate1PrimaryKey {
            regionid: self.regionid.clone(),
            version_datetime: self.version_datetime,
            wdrrrperiod: self.wdrrrperiod.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "settlements_config_wdr_reimburse_rate_v1".to_string()
    }
}
impl crate::CompareWithRow for SettlementsConfigWdrReimburseRate1 {
    type Row = SettlementsConfigWdrReimburseRate1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.regionid == row.regionid
            && self.version_datetime == row.version_datetime
            && self.wdrrrperiod == row.wdrrrperiod
    }
}
impl crate::CompareWithPrimaryKey for SettlementsConfigWdrReimburseRate1 {
    type PrimaryKey = SettlementsConfigWdrReimburseRate1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid
            && self.version_datetime == key.version_datetime
            && self.wdrrrperiod == key.wdrrrperiod
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsConfigWdrReimburseRate1PrimaryKey {
    pub regionid: String,
    pub version_datetime: chrono::NaiveDateTime,
    pub wdrrrperiod: String,
}
impl crate::CompareWithRow for SettlementsConfigWdrReimburseRate1PrimaryKey {
    type Row = SettlementsConfigWdrReimburseRate1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.regionid == row.regionid
            && self.version_datetime == row.version_datetime
            && self.wdrrrperiod == row.wdrrrperiod
    }
}
impl crate::CompareWithPrimaryKey for SettlementsConfigWdrReimburseRate1PrimaryKey {
    type PrimaryKey = SettlementsConfigWdrReimburseRate1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid
            && self.version_datetime == key.version_datetime
            && self.wdrrrperiod == key.wdrrrperiod
    }
}
impl crate::PrimaryKey for SettlementsConfigWdrReimburseRate1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SettlementsConfigWdrReimburseRate1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "wdrrrperiod",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "version_datetime",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "wdrrr",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "isfirm",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut wdrrrperiod_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut wdrrr_array = Vec::new();
        let mut isfirm_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            wdrrrperiod_array.push(row.wdrrrperiod);
            regionid_array.push(row.regionid);
            version_datetime_array.push(row.version_datetime.timestamp_millis());
            wdrrr_array.push({
                row.wdrrr.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            isfirm_array.push({
                row.isfirm.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    wdrrrperiod_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(version_datetime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(wdrrr_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(isfirm_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
