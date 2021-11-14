/// # Summary
///
/// ## CONTRACTAGC
///  _CONTRACTAGC shows Automatic Generation Control (AGC) contract details for each dispatchable unit. There is a separate contract for each unit._
///
/// * Data Set Name: Ancilliary Services
/// * File Name: Contractagc
/// * Data Version: 1
///
/// # Description
///  CONTRACTAGC data is confidential to the relevant participant. Source CONTRACTAGC updates only where there is a contract variation.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AncilliaryServicesContractagc1 {
    /// Contract Identifier
    pub contractid: String,
    /// Contract Version No
    pub versionno: rust_decimal::Decimal,
    /// Starting Date of Contract
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// End date of contract
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Unique participant identifier
    pub participantid: Option<String>,
    /// Dispatchable Unit ID
    pub duid: Option<String>,
    /// Control Range Raise 5 Min MW
    pub crr: Option<rust_decimal::Decimal>,
    /// Control Range Lower 5 Min MW
    pub crl: Option<rust_decimal::Decimal>,
    /// Enabling Price in $
    pub rlprice: Option<rust_decimal::Decimal>,
    /// Compensation Cap in $
    pub ccprice: Option<rust_decimal::Decimal>,
    /// Block Size
    pub bs: Option<rust_decimal::Decimal>,
    /// User Name
    pub authorisedby: Option<String>,
    /// Date Contract was Authorised
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for AncilliaryServicesContractagc1 {
    type PrimaryKey = AncilliaryServicesContractagc1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ANCILLIARY_SERVICES".into(),
            table_name: Some("CONTRACTAGC".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> AncilliaryServicesContractagc1PrimaryKey {
        AncilliaryServicesContractagc1PrimaryKey {
            contractid: self.contractid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "ancilliary_services_contractagc_v1".to_string()
    }
}
impl crate::CompareWithRow for AncilliaryServicesContractagc1 {
    type Row = AncilliaryServicesContractagc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for AncilliaryServicesContractagc1 {
    type PrimaryKey = AncilliaryServicesContractagc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AncilliaryServicesContractagc1PrimaryKey {
    pub contractid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for AncilliaryServicesContractagc1PrimaryKey {
    type Row = AncilliaryServicesContractagc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for AncilliaryServicesContractagc1PrimaryKey {
    type PrimaryKey = AncilliaryServicesContractagc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for AncilliaryServicesContractagc1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for AncilliaryServicesContractagc1 {
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
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("crr", arrow2::datatypes::DataType::Decimal(4, 0), true),
            arrow2::datatypes::Field::new("crl", arrow2::datatypes::DataType::Decimal(4, 0), true),
            arrow2::datatypes::Field::new(
                "rlprice",
                arrow2::datatypes::DataType::Decimal(10, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "ccprice",
                arrow2::datatypes::DataType::Decimal(10, 2),
                true,
            ),
            arrow2::datatypes::Field::new("bs", arrow2::datatypes::DataType::Decimal(10, 2), true),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date32,
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
        let mut startdate_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut crr_array = Vec::new();
        let mut crl_array = Vec::new();
        let mut rlprice_array = Vec::new();
        let mut ccprice_array = Vec::new();
        let mut bs_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            contractid_array.push(row.contractid);
            versionno_array.push({
                let mut val = row.versionno;
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
            participantid_array.push(row.participantid);
            duid_array.push(row.duid);
            crr_array.push({
                row.crr.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            crl_array.push({
                row.crl.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            rlprice_array.push({
                row.rlprice.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            ccprice_array.push({
                row.ccprice.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            bs_array.push({
                row.bs.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| {
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
                    contractid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(participantid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(crr_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(crl_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rlprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ccprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bs_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
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
/// ## CONTRACTLOADSHED
///  _CONTRACTLOADSHED shows Governor contract details used in the settlement and dispatch of this service. Note: services are dispatched as 6 and 60 raise Frequency Control Ancillary Services (FCAS). Mandatory requirements and breakpoint details are not used for load shed._
///
/// * Data Set Name: Ancilliary Services
/// * File Name: Contractloadshed
/// * Data Version: 2
///
/// # Description
///  CONTRACTLOADSHED data is confidential to the relevant participant. Source CONTRACTLOADSHED updates only where there is a contract variation.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AncilliaryServicesContractloadshed2 {
    /// Contract Identifier
    pub contractid: String,
    /// Contract Version No.
    pub versionno: rust_decimal::Decimal,
    /// Starting Date of Contract
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Termination Date of Contract
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Unique participant identifier
    pub participantid: Option<String>,
    /// Dispatchable Unit ID
    pub duid: Option<String>,
    /// The load shed enabling price for this contract
    pub lseprice: Option<rust_decimal::Decimal>,
    /// Minimum Compensation price
    pub mcpprice: Option<rust_decimal::Decimal>,
    /// Price Tendered for Compensation per Trading interval - Not used since 13/12/1998
    pub tenderedprice: Option<rust_decimal::Decimal>,
    /// Load Shed Control Range
    pub lscr: Option<rust_decimal::Decimal>,
    /// SPD scaling factor for load shed vs dispatched, (1 = dispatched)
    pub ilscalingfactor: Option<rust_decimal::Decimal>,
    /// Not used
    pub lower60secbreakpoint: Option<rust_decimal::Decimal>,
    /// Not used
    pub lower60secmax: Option<rust_decimal::Decimal>,
    /// Not used
    pub lower6secbreakpoint: Option<rust_decimal::Decimal>,
    /// Not used
    pub lower6secmax: Option<rust_decimal::Decimal>,
    /// Not used
    pub raise60secbreakpoint: Option<rust_decimal::Decimal>,
    /// Not used
    pub raise60seccapacity: Option<rust_decimal::Decimal>,
    /// Maximum 60 second raise
    pub raise60secmax: Option<rust_decimal::Decimal>,
    /// Not used
    pub raise6secbreakpoint: Option<rust_decimal::Decimal>,
    /// Not used
    pub raise6seccapacity: Option<rust_decimal::Decimal>,
    /// Limit Equation Raise 6 Second Maximum MW
    pub raise6secmax: Option<rust_decimal::Decimal>,
    /// Not used
    pub price6secraisemandatory: Option<rust_decimal::Decimal>,
    /// Not used
    pub quant6secraisemandatory: Option<rust_decimal::Decimal>,
    /// Contract Price for 6 Second Raise
    pub price6secraisecontract: Option<rust_decimal::Decimal>,
    /// Contract Quantity for 6 Second Raise
    pub quant6secraisecontract: Option<rust_decimal::Decimal>,
    /// Not used
    pub price60secraisemandatory: Option<rust_decimal::Decimal>,
    /// Not used
    pub quant60secraisemandatory: Option<rust_decimal::Decimal>,
    /// Not used
    pub price60secraisecontract: Option<rust_decimal::Decimal>,
    /// Not used
    pub quant60secraisecontract: Option<rust_decimal::Decimal>,
    /// Not used
    pub price6seclowermandatory: Option<rust_decimal::Decimal>,
    /// Not used
    pub quant6seclowermandatory: Option<rust_decimal::Decimal>,
    /// Not used
    pub price6seclowercontract: Option<rust_decimal::Decimal>,
    /// Not used
    pub quant6seclowercontract: Option<rust_decimal::Decimal>,
    /// Not used
    pub price60seclowermandatory: Option<rust_decimal::Decimal>,
    /// Not used
    pub quant60seclowermandatory: Option<rust_decimal::Decimal>,
    /// Not used
    pub price60seclowercontract: Option<rust_decimal::Decimal>,
    /// Not used
    pub quant60seclowercontract: Option<rust_decimal::Decimal>,
    /// User Name
    pub authorisedby: Option<String>,
    /// Date Contract was Authorised
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The NMAS default payment amount
    pub default_testingpayment_amount: Option<rust_decimal::Decimal>,
    /// The NMAS Testing Service Start Date
    #[serde(with = "crate::mms_datetime_opt")]
    pub service_start_date: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for AncilliaryServicesContractloadshed2 {
    type PrimaryKey = AncilliaryServicesContractloadshed2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ANCILLIARY_SERVICES".into(),
            table_name: Some("CONTRACTLOADSHED".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> AncilliaryServicesContractloadshed2PrimaryKey {
        AncilliaryServicesContractloadshed2PrimaryKey {
            contractid: self.contractid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "ancilliary_services_contractloadshed_v2".to_string()
    }
}
impl crate::CompareWithRow for AncilliaryServicesContractloadshed2 {
    type Row = AncilliaryServicesContractloadshed2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for AncilliaryServicesContractloadshed2 {
    type PrimaryKey = AncilliaryServicesContractloadshed2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AncilliaryServicesContractloadshed2PrimaryKey {
    pub contractid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for AncilliaryServicesContractloadshed2PrimaryKey {
    type Row = AncilliaryServicesContractloadshed2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for AncilliaryServicesContractloadshed2PrimaryKey {
    type PrimaryKey = AncilliaryServicesContractloadshed2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for AncilliaryServicesContractloadshed2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for AncilliaryServicesContractloadshed2 {
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
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "lseprice",
                arrow2::datatypes::DataType::Decimal(6, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mcpprice",
                arrow2::datatypes::DataType::Decimal(12, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "tenderedprice",
                arrow2::datatypes::DataType::Decimal(6, 2),
                true,
            ),
            arrow2::datatypes::Field::new("lscr", arrow2::datatypes::DataType::Decimal(6, 2), true),
            arrow2::datatypes::Field::new(
                "ilscalingfactor",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secbreakpoint",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secmax",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secbreakpoint",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secmax",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secbreakpoint",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60seccapacity",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secmax",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6secbreakpoint",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6seccapacity",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6secmax",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "price6secraisemandatory",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "quant6secraisemandatory",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "price6secraisecontract",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "quant6secraisecontract",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "price60secraisemandatory",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "quant60secraisemandatory",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "price60secraisecontract",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "quant60secraisecontract",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "price6seclowermandatory",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "quant6seclowermandatory",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "price6seclowercontract",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "quant6seclowercontract",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "price60seclowermandatory",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "quant60seclowermandatory",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "price60seclowercontract",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "quant60seclowercontract",
                arrow2::datatypes::DataType::Decimal(9, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "default_testingpayment_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "service_start_date",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut lseprice_array = Vec::new();
        let mut mcpprice_array = Vec::new();
        let mut tenderedprice_array = Vec::new();
        let mut lscr_array = Vec::new();
        let mut ilscalingfactor_array = Vec::new();
        let mut lower60secbreakpoint_array = Vec::new();
        let mut lower60secmax_array = Vec::new();
        let mut lower6secbreakpoint_array = Vec::new();
        let mut lower6secmax_array = Vec::new();
        let mut raise60secbreakpoint_array = Vec::new();
        let mut raise60seccapacity_array = Vec::new();
        let mut raise60secmax_array = Vec::new();
        let mut raise6secbreakpoint_array = Vec::new();
        let mut raise6seccapacity_array = Vec::new();
        let mut raise6secmax_array = Vec::new();
        let mut price6secraisemandatory_array = Vec::new();
        let mut quant6secraisemandatory_array = Vec::new();
        let mut price6secraisecontract_array = Vec::new();
        let mut quant6secraisecontract_array = Vec::new();
        let mut price60secraisemandatory_array = Vec::new();
        let mut quant60secraisemandatory_array = Vec::new();
        let mut price60secraisecontract_array = Vec::new();
        let mut quant60secraisecontract_array = Vec::new();
        let mut price6seclowermandatory_array = Vec::new();
        let mut quant6seclowermandatory_array = Vec::new();
        let mut price6seclowercontract_array = Vec::new();
        let mut quant6seclowercontract_array = Vec::new();
        let mut price60seclowermandatory_array = Vec::new();
        let mut quant60seclowermandatory_array = Vec::new();
        let mut price60seclowercontract_array = Vec::new();
        let mut quant60seclowercontract_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut default_testingpayment_amount_array = Vec::new();
        let mut service_start_date_array = Vec::new();
        for (_, row) in partition {
            contractid_array.push(row.contractid);
            versionno_array.push({
                let mut val = row.versionno;
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
            participantid_array.push(row.participantid);
            duid_array.push(row.duid);
            lseprice_array.push({
                row.lseprice.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            mcpprice_array.push({
                row.mcpprice.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            tenderedprice_array.push({
                row.tenderedprice.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            lscr_array.push({
                row.lscr.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            ilscalingfactor_array.push({
                row.ilscalingfactor.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lower60secbreakpoint_array.push({
                row.lower60secbreakpoint.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lower60secmax_array.push({
                row.lower60secmax.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lower6secbreakpoint_array.push({
                row.lower6secbreakpoint.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lower6secmax_array.push({
                row.lower6secmax.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            raise60secbreakpoint_array.push({
                row.raise60secbreakpoint.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            raise60seccapacity_array.push({
                row.raise60seccapacity.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            raise60secmax_array.push({
                row.raise60secmax.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            raise6secbreakpoint_array.push({
                row.raise6secbreakpoint.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            raise6seccapacity_array.push({
                row.raise6seccapacity.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            raise6secmax_array.push({
                row.raise6secmax.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            price6secraisemandatory_array.push({
                row.price6secraisemandatory.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            quant6secraisemandatory_array.push({
                row.quant6secraisemandatory.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            price6secraisecontract_array.push({
                row.price6secraisecontract.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            quant6secraisecontract_array.push({
                row.quant6secraisecontract.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            price60secraisemandatory_array.push({
                row.price60secraisemandatory.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            quant60secraisemandatory_array.push({
                row.quant60secraisemandatory.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            price60secraisecontract_array.push({
                row.price60secraisecontract.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            quant60secraisecontract_array.push({
                row.quant60secraisecontract.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            price6seclowermandatory_array.push({
                row.price6seclowermandatory.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            quant6seclowermandatory_array.push({
                row.quant6seclowermandatory.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            price6seclowercontract_array.push({
                row.price6seclowercontract.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            quant6seclowercontract_array.push({
                row.quant6seclowercontract.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            price60seclowermandatory_array.push({
                row.price60seclowermandatory.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            quant60seclowermandatory_array.push({
                row.quant60seclowermandatory.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            price60seclowercontract_array.push({
                row.price60seclowercontract.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            quant60seclowercontract_array.push({
                row.quant60seclowercontract.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            default_testingpayment_amount_array.push({
                row.default_testingpayment_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            service_start_date_array.push(row.service_start_date.map(|val| {
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
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(participantid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lseprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mcpprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(12, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(tenderedprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lscr_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ilscalingfactor_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secbreakpoint_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secmax_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secbreakpoint_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secmax_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secbreakpoint_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60seccapacity_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secmax_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secbreakpoint_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6seccapacity_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secmax_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(price6secraisemandatory_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(quant6secraisemandatory_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(price6secraisecontract_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(quant6secraisecontract_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(price60secraisemandatory_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(quant60secraisemandatory_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(price60secraisecontract_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(quant60secraisecontract_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(price6seclowermandatory_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(quant6seclowermandatory_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(price6seclowercontract_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(quant6seclowercontract_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(price60seclowermandatory_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(quant60seclowermandatory_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(price60seclowercontract_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(quant60seclowercontract_array)
                        .to(arrow2::datatypes::DataType::Decimal(9, 6)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(default_testingpayment_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(service_start_date_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## CONTRACTREACTIVEPOWER
///  _CONTRACTREACTIVEPOWER shows Reactive Power contract details used in the settlement and dispatch of this service._
///
/// * Data Set Name: Ancilliary Services
/// * File Name: Contractreactivepower
/// * Data Version: 4
///
/// # Description
///  CONTRACTREACTIVEPOWER data is confidential to the relevant participant. Source CONTRACTREACTIVEPOWER updates only where there is a contract variation.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AncilliaryServicesContractreactivepower4 {
    /// Contract Identifier
    pub contractid: String,
    /// Contract Version No.
    pub versionno: rust_decimal::Decimal,
    /// Starting Date of Contract
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Termination Date of Contract
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Unique participant identifier
    pub participantid: Option<String>,
    /// Dispatchable Unit ID
    pub duid: Option<String>,
    /// Sync Compensation Flag - Y for SYNCCOMP
    pub synccompensation: Option<String>,
    /// Availability price per MVAr of RP absorption capability
    pub mvaraprice: Option<rust_decimal::Decimal>,
    /// Enabling price
    pub mvareprice: Option<rust_decimal::Decimal>,
    /// Availability price per MVAr of RP generation capability
    pub mvargprice: Option<rust_decimal::Decimal>,
    /// Compensation Cap
    pub ccprice: Option<rust_decimal::Decimal>,
    /// Reactive Power Absorption Capability (MVAr)
    pub mta: Option<rust_decimal::Decimal>,
    /// Reactive Power Generation Capability (MVAr)
    pub mtg: Option<rust_decimal::Decimal>,
    /// Minimum Capability for MVAr Absorption required by Code
    pub mmca: Option<rust_decimal::Decimal>,
    /// Minimum Capability for MVAr Generation required by Code
    pub mmcg: Option<rust_decimal::Decimal>,
    /// Estimated Power consumption of unit when operating on SYNCCOMP
    pub eu: Option<rust_decimal::Decimal>,
    /// Estimated Price for supply
    pub pp: Option<rust_decimal::Decimal>,
    /// Block Size of Unit
    pub bs: Option<rust_decimal::Decimal>,
    /// User Name
    pub authorisedby: Option<String>,
    /// Date Contract was Authorised
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The NMAS default payment amount
    pub default_testingpayment_amount: Option<rust_decimal::Decimal>,
    /// The NMAS Testing Service Start Date
    #[serde(with = "crate::mms_datetime_opt")]
    pub service_start_date: Option<chrono::NaiveDateTime>,
    /// The MWh the unit must produce in a trading interval to be eligible for an excess-to-gen availability payment
    pub availability_mwh_threshold: Option<rust_decimal::Decimal>,
    /// The threshold value for MegaVar (MVAr) to check whether the service is fully available.
    pub mvar_threshold: Option<rust_decimal::Decimal>,
    /// The maximum capped amount for the rebate payment.
    pub rebate_cap: Option<rust_decimal::Decimal>,
    /// The per MVAR rebate amount used to calculate the rebate payment.
    pub rebate_amount_per_mvar: Option<rust_decimal::Decimal>,
    /// Used to check whether the contract is eligible for rebate. For new NSCAS contracts to apply new payment methodology this flag is 1.
    pub isrebateapplicable: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for AncilliaryServicesContractreactivepower4 {
    type PrimaryKey = AncilliaryServicesContractreactivepower4PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ANCILLIARY_SERVICES".into(),
            table_name: Some("CONTRACTREACTIVEPOWER".into()),
            version: 4,
        }
    }

    fn primary_key(&self) -> AncilliaryServicesContractreactivepower4PrimaryKey {
        AncilliaryServicesContractreactivepower4PrimaryKey {
            contractid: self.contractid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "ancilliary_services_contractreactivepower_v4".to_string()
    }
}
impl crate::CompareWithRow for AncilliaryServicesContractreactivepower4 {
    type Row = AncilliaryServicesContractreactivepower4;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for AncilliaryServicesContractreactivepower4 {
    type PrimaryKey = AncilliaryServicesContractreactivepower4PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AncilliaryServicesContractreactivepower4PrimaryKey {
    pub contractid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for AncilliaryServicesContractreactivepower4PrimaryKey {
    type Row = AncilliaryServicesContractreactivepower4;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for AncilliaryServicesContractreactivepower4PrimaryKey {
    type PrimaryKey = AncilliaryServicesContractreactivepower4PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for AncilliaryServicesContractreactivepower4PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for AncilliaryServicesContractreactivepower4 {
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
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "synccompensation",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "mvaraprice",
                arrow2::datatypes::DataType::Decimal(10, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mvareprice",
                arrow2::datatypes::DataType::Decimal(10, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mvargprice",
                arrow2::datatypes::DataType::Decimal(10, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "ccprice",
                arrow2::datatypes::DataType::Decimal(10, 2),
                true,
            ),
            arrow2::datatypes::Field::new("mta", arrow2::datatypes::DataType::Decimal(10, 2), true),
            arrow2::datatypes::Field::new("mtg", arrow2::datatypes::DataType::Decimal(10, 2), true),
            arrow2::datatypes::Field::new(
                "mmca",
                arrow2::datatypes::DataType::Decimal(10, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mmcg",
                arrow2::datatypes::DataType::Decimal(10, 2),
                true,
            ),
            arrow2::datatypes::Field::new("eu", arrow2::datatypes::DataType::Decimal(10, 2), true),
            arrow2::datatypes::Field::new("pp", arrow2::datatypes::DataType::Decimal(10, 2), true),
            arrow2::datatypes::Field::new("bs", arrow2::datatypes::DataType::Decimal(10, 2), true),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "default_testingpayment_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "service_start_date",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "availability_mwh_threshold",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mvar_threshold",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rebate_cap",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rebate_amount_per_mvar",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "isrebateapplicable",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut synccompensation_array = Vec::new();
        let mut mvaraprice_array = Vec::new();
        let mut mvareprice_array = Vec::new();
        let mut mvargprice_array = Vec::new();
        let mut ccprice_array = Vec::new();
        let mut mta_array = Vec::new();
        let mut mtg_array = Vec::new();
        let mut mmca_array = Vec::new();
        let mut mmcg_array = Vec::new();
        let mut eu_array = Vec::new();
        let mut pp_array = Vec::new();
        let mut bs_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut default_testingpayment_amount_array = Vec::new();
        let mut service_start_date_array = Vec::new();
        let mut availability_mwh_threshold_array = Vec::new();
        let mut mvar_threshold_array = Vec::new();
        let mut rebate_cap_array = Vec::new();
        let mut rebate_amount_per_mvar_array = Vec::new();
        let mut isrebateapplicable_array = Vec::new();
        for (_, row) in partition {
            contractid_array.push(row.contractid);
            versionno_array.push({
                let mut val = row.versionno;
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
            participantid_array.push(row.participantid);
            duid_array.push(row.duid);
            synccompensation_array.push(row.synccompensation);
            mvaraprice_array.push({
                row.mvaraprice.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            mvareprice_array.push({
                row.mvareprice.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            mvargprice_array.push({
                row.mvargprice.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            ccprice_array.push({
                row.ccprice.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            mta_array.push({
                row.mta.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            mtg_array.push({
                row.mtg.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            mmca_array.push({
                row.mmca.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            mmcg_array.push({
                row.mmcg.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            eu_array.push({
                row.eu.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            pp_array.push({
                row.pp.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            bs_array.push({
                row.bs.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            default_testingpayment_amount_array.push({
                row.default_testingpayment_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            service_start_date_array.push(row.service_start_date.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            availability_mwh_threshold_array.push({
                row.availability_mwh_threshold.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            mvar_threshold_array.push({
                row.mvar_threshold.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            rebate_cap_array.push({
                row.rebate_cap.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            rebate_amount_per_mvar_array.push({
                row.rebate_amount_per_mvar.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            isrebateapplicable_array.push({
                row.isrebateapplicable.map(|mut val| {
                    val.rescale(0);
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
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(participantid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(duid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    synccompensation_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mvaraprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mvareprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mvargprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ccprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mta_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mtg_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mmca_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mmcg_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(eu_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(pp_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bs_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 2)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(default_testingpayment_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(service_start_date_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(availability_mwh_threshold_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mvar_threshold_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rebate_cap_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rebate_amount_per_mvar_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(isrebateapplicable_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## CONTRACTRESTARTSERVICES
///  _CONTRACTRESTARTSERVICES shows Restart Services contract details used in the settlement and dispatch of this service._
///
/// * Data Set Name: Ancilliary Services
/// * File Name: Contractrestartservices
/// * Data Version: 2
///
/// # Description
///  CONTRACTRESTARTSERVICES data is confidential to the participant holding the contract. Source CONTRACTRESTARTSERVICES updates only where there is a contract variation.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AncilliaryServicesContractrestartservices2 {
    /// Contract Identifier
    pub contractid: String,
    /// Contract Version No.
    pub versionno: rust_decimal::Decimal,
    /// Starting Date of Contract
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// Termination Date of Contract
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Unique participant identifier
    pub participantid: Option<String>,
    /// Restart Type - 0 = BlackStart, 1 = Combination, 2 = Trip To House
    pub restarttype: Option<rust_decimal::Decimal>,
    /// Availability Price
    pub rcprice: Option<rust_decimal::Decimal>,
    /// Trip To House Level
    pub triptohouselevel: Option<rust_decimal::Decimal>,
    /// User Name
    pub authorisedby: Option<String>,
    /// Date Contract was Authorised
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The NMAS default payment amount
    pub default_testingpayment_amount: Option<rust_decimal::Decimal>,
    /// The NMAS Testing Service Start Date
    #[serde(with = "crate::mms_datetime_opt")]
    pub service_start_date: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for AncilliaryServicesContractrestartservices2 {
    type PrimaryKey = AncilliaryServicesContractrestartservices2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ANCILLIARY_SERVICES".into(),
            table_name: Some("CONTRACTRESTARTSERVICES".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> AncilliaryServicesContractrestartservices2PrimaryKey {
        AncilliaryServicesContractrestartservices2PrimaryKey {
            contractid: self.contractid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "ancilliary_services_contractrestartservices_v2".to_string()
    }
}
impl crate::CompareWithRow for AncilliaryServicesContractrestartservices2 {
    type Row = AncilliaryServicesContractrestartservices2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for AncilliaryServicesContractrestartservices2 {
    type PrimaryKey = AncilliaryServicesContractrestartservices2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AncilliaryServicesContractrestartservices2PrimaryKey {
    pub contractid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for AncilliaryServicesContractrestartservices2PrimaryKey {
    type Row = AncilliaryServicesContractrestartservices2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for AncilliaryServicesContractrestartservices2PrimaryKey {
    type PrimaryKey = AncilliaryServicesContractrestartservices2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for AncilliaryServicesContractrestartservices2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for AncilliaryServicesContractrestartservices2 {
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
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "restarttype",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rcprice",
                arrow2::datatypes::DataType::Decimal(6, 2),
                true,
            ),
            arrow2::datatypes::Field::new(
                "triptohouselevel",
                arrow2::datatypes::DataType::Decimal(5, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "default_testingpayment_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "service_start_date",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut restarttype_array = Vec::new();
        let mut rcprice_array = Vec::new();
        let mut triptohouselevel_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut default_testingpayment_amount_array = Vec::new();
        let mut service_start_date_array = Vec::new();
        for (_, row) in partition {
            contractid_array.push(row.contractid);
            versionno_array.push({
                let mut val = row.versionno;
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
            participantid_array.push(row.participantid);
            restarttype_array.push({
                row.restarttype.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            rcprice_array.push({
                row.rcprice.map(|mut val| {
                    val.rescale(2);
                    val.mantissa()
                })
            });
            triptohouselevel_array.push({
                row.triptohouselevel.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            default_testingpayment_amount_array.push({
                row.default_testingpayment_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            service_start_date_array.push(row.service_start_date.map(|val| {
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
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(participantid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(restarttype_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rcprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 2)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(triptohouselevel_array)
                        .to(arrow2::datatypes::DataType::Decimal(5, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(default_testingpayment_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(service_start_date_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## CONTRACTRESTARTUNITS
///  _CONTRACTRESTARTUNITS shows Restart units provided under a system restart contract. A service can have multiple units._
///
/// * Data Set Name: Ancilliary Services
/// * File Name: Contractrestartunits
/// * Data Version: 1
///
/// # Description
///  CONTRACTRESTARTUNITS data is confidential to each participant with a restart contract. Source CONTRACTRESTARTUNITS updates only where there is a contract variation.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * DUID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct AncilliaryServicesContractrestartunits1 {
    /// Contract Identifier
    pub contractid: String,
    /// Version No of contract
    pub versionno: rust_decimal::Decimal,
    /// Dispatchable Unit identifier
    pub duid: String,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub authorisedby: Option<String>,
    /// &nbsp;
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for AncilliaryServicesContractrestartunits1 {
    type PrimaryKey = AncilliaryServicesContractrestartunits1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "ANCILLIARY_SERVICES".into(),
            table_name: Some("CONTRACTRESTARTUNITS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> AncilliaryServicesContractrestartunits1PrimaryKey {
        AncilliaryServicesContractrestartunits1PrimaryKey {
            contractid: self.contractid.clone(),
            duid: self.duid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "ancilliary_services_contractrestartunits_v1".to_string()
    }
}
impl crate::CompareWithRow for AncilliaryServicesContractrestartunits1 {
    type Row = AncilliaryServicesContractrestartunits1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.duid == row.duid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for AncilliaryServicesContractrestartunits1 {
    type PrimaryKey = AncilliaryServicesContractrestartunits1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.duid == key.duid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AncilliaryServicesContractrestartunits1PrimaryKey {
    pub contractid: String,
    pub duid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for AncilliaryServicesContractrestartunits1PrimaryKey {
    type Row = AncilliaryServicesContractrestartunits1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.duid == row.duid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for AncilliaryServicesContractrestartunits1PrimaryKey {
    type PrimaryKey = AncilliaryServicesContractrestartunits1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.duid == key.duid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for AncilliaryServicesContractrestartunits1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for AncilliaryServicesContractrestartunits1 {
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
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut contractid_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        for (_, row) in partition {
            contractid_array.push(row.contractid);
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            duid_array.push(row.duid);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| {
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
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
