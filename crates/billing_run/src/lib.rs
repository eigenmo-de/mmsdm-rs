#[allow(unused_imports)]
use chrono::Datelike as _;
/// # Summary
///
/// ## BILLINGASPAYMENTS
///  _BILLINGASPAYMENTS shows Ancillary Service payments for each billing period by each of the Ancillary Service types for each participant’s connection points._
///
/// * Data Set Name: Billing
/// * File Name: Aspayments
/// * Data Version: 7
///
/// # Description
///  BILLINGASPAYMENTS data is confidential to relevant participant. Source Updated  with each billing run. Volume The volume is according to the number of Transmission ConnectionPointIDs a Participant may have subject to ancillary payment per billrunno. An indicative maximum is approximately 20 records are inserted per billrunno, or about 220 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONNECTIONPOINTID
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingAspayments7 {
    /// Region Identifier
    pub regionid: Option<String>,
    /// Contract Year
    pub contractyear: rust_decimal::Decimal,
    /// Week No
    pub weekno: rust_decimal::Decimal,
    /// Billing Run No.
    pub billrunno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// Connection point identifier
    pub connectionpointid: String,
    /// Raise 6 Sec Payments
    pub raise6sec: Option<rust_decimal::Decimal>,
    /// Lower 6 Sec Payments
    pub lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 60 Sec Payments
    pub raise60sec: Option<rust_decimal::Decimal>,
    /// Lower 60 Sec Payments
    pub lower60sec: Option<rust_decimal::Decimal>,
    /// AGC Payments
    pub agc: Option<rust_decimal::Decimal>,
    /// Frequency Control Compensation Payments
    pub fcascomp: Option<rust_decimal::Decimal>,
    /// Load Shed Payments
    pub loadshed: Option<rust_decimal::Decimal>,
    /// Rapid Generator unit Loading Payments
    pub rgul: Option<rust_decimal::Decimal>,
    /// Rapid Generator Unit Unloading Payments
    pub rguu: Option<rust_decimal::Decimal>,
    /// Reactive Power Payments
    pub reactivepower: Option<rust_decimal::Decimal>,
    /// System Restart Payments
    pub systemrestart: Option<rust_decimal::Decimal>,
    /// The latest date and time that a file was updated or inserted
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Lower 5 Minute Payment
    pub lower5min: Option<rust_decimal::Decimal>,
    /// Raise 5 Minute Payment
    pub raise5min: Option<rust_decimal::Decimal>,
    /// Lower 5 Minute Regulation Payment
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Raise 5 Minute Regulation Payment
    pub raisereg: Option<rust_decimal::Decimal>,
    /// The total availability payment
    pub availability_reactive: Option<rust_decimal::Decimal>,
    /// The total availability payment rebate
    pub availability_reactive_rbt: Option<rust_decimal::Decimal>,
    /// Payment amount for the very fast raise service
    pub raise1sec: Option<rust_decimal::Decimal>,
    /// Payment amount for the very fast lower service
    pub lower1sec: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingAspayments7 {
    type PrimaryKey = BillingAspayments7PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("ASPAYMENTS".into()),
            version: 7,
        }
    }
    fn primary_key(&self) -> BillingAspayments7PrimaryKey {
        BillingAspayments7PrimaryKey {
            billrunno: self.billrunno,
            connectionpointid: self.connectionpointid.clone(),
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_aspayments_v7".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingAspayments7PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub connectionpointid: String,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingAspayments7PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingAspayments7 {
    type Row = BillingAspayments7;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno
            && self.connectionpointid == row.connectionpointid
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingAspayments7 {
    type PrimaryKey = BillingAspayments7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno
            && self.connectionpointid == key.connectionpointid
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingAspayments7PrimaryKey {
    type Row = BillingAspayments7;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno
            && self.connectionpointid == row.connectionpointid
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingAspayments7PrimaryKey {
    type PrimaryKey = BillingAspayments7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno
            && self.connectionpointid == key.connectionpointid
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingAspayments7 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("raise6sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("agc",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("fcascomp",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("loadshed",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("rgul",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("rguu",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("reactivepower",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("systemrestart",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("lower5min",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise5min",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lowerreg",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raisereg",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("availability_reactive",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("availability_reactive_rbt",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("raise1sec",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("lower1sec",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut regionid_array = Vec::new();
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut raise6sec_array = Vec::new();
        let mut lower6sec_array = Vec::new();
        let mut raise60sec_array = Vec::new();
        let mut lower60sec_array = Vec::new();
        let mut agc_array = Vec::new();
        let mut fcascomp_array = Vec::new();
        let mut loadshed_array = Vec::new();
        let mut rgul_array = Vec::new();
        let mut rguu_array = Vec::new();
        let mut reactivepower_array = Vec::new();
        let mut systemrestart_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut lower5min_array = Vec::new();
        let mut raise5min_array = Vec::new();
        let mut lowerreg_array = Vec::new();
        let mut raisereg_array = Vec::new();
        let mut availability_reactive_array = Vec::new();
        let mut availability_reactive_rbt_array = Vec::new();
        let mut raise1sec_array = Vec::new();
        let mut lower1sec_array = Vec::new();
        for row in partition {
            regionid_array.push(row.regionid);
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            connectionpointid_array.push(row.connectionpointid);
            raise6sec_array
                .push({
                    row.raise6sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6sec_array
                .push({
                    row.lower6sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60sec_array
                .push({
                    row.raise60sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60sec_array
                .push({
                    row.lower60sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            agc_array
                .push({
                    row.agc
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            fcascomp_array
                .push({
                    row.fcascomp
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            loadshed_array
                .push({
                    row.loadshed
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            rgul_array
                .push({
                    row.rgul
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            rguu_array
                .push({
                    row.rguu
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            reactivepower_array
                .push({
                    row.reactivepower
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            systemrestart_array
                .push({
                    row.systemrestart
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            lower5min_array
                .push({
                    row.lower5min
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise5min_array
                .push({
                    row.raise5min
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lowerreg_array
                .push({
                    row.lowerreg
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raisereg_array
                .push({
                    row.raisereg
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            availability_reactive_array
                .push({
                    row.availability_reactive
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            availability_reactive_rbt_array
                .push({
                    row.availability_reactive_rbt
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            raise1sec_array
                .push({
                    row.raise1sec
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            lower1sec_array
                .push({
                    row.lower1sec
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(regionid_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(connectionpointid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(agc_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(fcascomp_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(loadshed_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rgul_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rguu_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(reactivepower_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(systemrestart_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5min_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5min_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowerreg_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raisereg_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(availability_reactive_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(availability_reactive_rbt_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise1sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower1sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLINGASRECOVERY
///  _BILLINGASRECOVERY shows participant charges for Ancillary Services for the billing period. This view shows the billing amounts for Ancillary Service Recovery._
///
/// * Data Set Name: Billing
/// * File Name: Asrecovery
/// * Data Version: 8
///
/// # Description
///  BILLINGASRECOVERY data is confidential to relevant participant. Source Updated  with each billing run. Volume Approximately 5 records are inserted per billrunno, or about 55 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingAsrecovery8 {
    /// Region Identifier
    pub regionid: String,
    /// Contract Year
    pub contractyear: rust_decimal::Decimal,
    /// Week No
    pub weekno: rust_decimal::Decimal,
    /// Billing Run No.
    pub billrunno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// Raise 6 Sec Recovery
    pub raise6sec: Option<rust_decimal::Decimal>,
    /// Lower 6 Sec Recovery
    pub lower6sec: Option<rust_decimal::Decimal>,
    /// Raise 60 Sec Recovery
    pub raise60sec: Option<rust_decimal::Decimal>,
    /// Lower 60 Sec Recovery
    pub lower60sec: Option<rust_decimal::Decimal>,
    /// AGC Recovery - Not used since circa 2000
    pub agc: Option<rust_decimal::Decimal>,
    /// Frequency Control Compensation Recovery - Not used since circa 2000
    pub fcascomp: Option<rust_decimal::Decimal>,
    /// Load Shed Recovery
    pub loadshed: Option<rust_decimal::Decimal>,
    /// Rapid Generator Unit Loading Recovery - Not used since December 2001
    pub rgul: Option<rust_decimal::Decimal>,
    /// Rapid Generator Unit Unloading Recovery - Not used since December 2001
    pub rguu: Option<rust_decimal::Decimal>,
    /// Reactive Power Recovery
    pub reactivepower: Option<rust_decimal::Decimal>,
    /// System Restart Recovery
    pub systemrestart: Option<rust_decimal::Decimal>,
    /// The latest date and time a file was updated/inserted
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Raise 6 Sec Recovery for Generator
    pub raise6sec_gen: Option<rust_decimal::Decimal>,
    /// Lower 6 Sec Recovery for Generator
    pub lower6sec_gen: Option<rust_decimal::Decimal>,
    /// Raise 60 Sec Recovery for Generator
    pub raise60sec_gen: Option<rust_decimal::Decimal>,
    /// Lower 60 Sec Recovery for Generator
    pub lower60sec_gen: Option<rust_decimal::Decimal>,
    /// AGC Recovery for Generator
    pub agc_gen: Option<rust_decimal::Decimal>,
    /// Frequency Control Compensation Recovery for Generator
    pub fcascomp_gen: Option<rust_decimal::Decimal>,
    /// Load Shed Recovery for Generator
    pub loadshed_gen: Option<rust_decimal::Decimal>,
    /// Rapid Generator unit Loading Recovery for. Generator - Not used since December 2001
    pub rgul_gen: Option<rust_decimal::Decimal>,
    /// Rapid Generator Unit Unloading Recovery for Generator - Not used since December 2001
    pub rguu_gen: Option<rust_decimal::Decimal>,
    /// Reactive Power Recovery for Generator
    pub reactivepower_gen: Option<rust_decimal::Decimal>,
    /// System Restart Recovery for Generator
    pub systemrestart_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 5 Minute service attributable to customer connection points
    pub lower5min: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 5 Minute service attributable to customer connection points
    pub raise5min: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower Regulation service attributable to customer connection points
    pub lowerreg: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise Regulation Second service attributable to customer connection points
    pub raisereg: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 5 Minute service attributable to generator connection points
    pub lower5min_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 5 Minute service attributable to generator connection points
    pub raise5min_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower Regulation service attributable to generator connection points
    pub lowerreg_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise Regulation Second service attributable to generator connection points
    pub raisereg_gen: Option<rust_decimal::Decimal>,
    /// The total availability payment recovery amount (customer).
    pub availability_reactive: Option<rust_decimal::Decimal>,
    /// The total availability payment rebate recovery amount (customer).
    pub availability_reactive_rbt: Option<rust_decimal::Decimal>,
    /// The total availability payment recovery amount (Generator).
    pub availability_reactive_gen: Option<rust_decimal::Decimal>,
    /// The total availability payment rebate recovery amount (Generator).
    pub availability_reactive_rbt_gen: Option<rust_decimal::Decimal>,
    /// Customer recovery amount for the very fast raise service
    pub raise1sec: Option<rust_decimal::Decimal>,
    /// Customer recovery amount for the very fast lower service
    pub lower1sec: Option<rust_decimal::Decimal>,
    /// Generator recovery amount for the very fast raise service
    pub raise1sec_gen: Option<rust_decimal::Decimal>,
    /// Generator recovery amount for the very fast lower service
    pub lower1sec_gen: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingAsrecovery8 {
    type PrimaryKey = BillingAsrecovery8PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("ASRECOVERY".into()),
            version: 8,
        }
    }
    fn primary_key(&self) -> BillingAsrecovery8PrimaryKey {
        BillingAsrecovery8PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            regionid: self.regionid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_asrecovery_v8".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingAsrecovery8PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub regionid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingAsrecovery8PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingAsrecovery8 {
    type Row = BillingAsrecovery8;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingAsrecovery8 {
    type PrimaryKey = BillingAsrecovery8PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingAsrecovery8PrimaryKey {
    type Row = BillingAsrecovery8;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingAsrecovery8PrimaryKey {
    type PrimaryKey = BillingAsrecovery8PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingAsrecovery8 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("raise6sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60sec",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("agc",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("fcascomp",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("loadshed",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("rgul",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("rguu",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("reactivepower",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("systemrestart",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("raise6sec_gen",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower6sec_gen",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise60sec_gen",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower60sec_gen",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("agc_gen",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("fcascomp_gen",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("loadshed_gen",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("rgul_gen",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("rguu_gen",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("reactivepower_gen",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("systemrestart_gen",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower5min",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raise5min",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lowerreg",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("raisereg",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lower5min_gen",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("raise5min_gen",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("lowerreg_gen",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("raisereg_gen",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("availability_reactive",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("availability_reactive_rbt",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("availability_reactive_gen",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("availability_reactive_rbt_gen",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("raise1sec",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("lower1sec",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("raise1sec_gen",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("lower1sec_gen",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut regionid_array = Vec::new();
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut raise6sec_array = Vec::new();
        let mut lower6sec_array = Vec::new();
        let mut raise60sec_array = Vec::new();
        let mut lower60sec_array = Vec::new();
        let mut agc_array = Vec::new();
        let mut fcascomp_array = Vec::new();
        let mut loadshed_array = Vec::new();
        let mut rgul_array = Vec::new();
        let mut rguu_array = Vec::new();
        let mut reactivepower_array = Vec::new();
        let mut systemrestart_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut raise6sec_gen_array = Vec::new();
        let mut lower6sec_gen_array = Vec::new();
        let mut raise60sec_gen_array = Vec::new();
        let mut lower60sec_gen_array = Vec::new();
        let mut agc_gen_array = Vec::new();
        let mut fcascomp_gen_array = Vec::new();
        let mut loadshed_gen_array = Vec::new();
        let mut rgul_gen_array = Vec::new();
        let mut rguu_gen_array = Vec::new();
        let mut reactivepower_gen_array = Vec::new();
        let mut systemrestart_gen_array = Vec::new();
        let mut lower5min_array = Vec::new();
        let mut raise5min_array = Vec::new();
        let mut lowerreg_array = Vec::new();
        let mut raisereg_array = Vec::new();
        let mut lower5min_gen_array = Vec::new();
        let mut raise5min_gen_array = Vec::new();
        let mut lowerreg_gen_array = Vec::new();
        let mut raisereg_gen_array = Vec::new();
        let mut availability_reactive_array = Vec::new();
        let mut availability_reactive_rbt_array = Vec::new();
        let mut availability_reactive_gen_array = Vec::new();
        let mut availability_reactive_rbt_gen_array = Vec::new();
        let mut raise1sec_array = Vec::new();
        let mut lower1sec_array = Vec::new();
        let mut raise1sec_gen_array = Vec::new();
        let mut lower1sec_gen_array = Vec::new();
        for row in partition {
            regionid_array.push(row.regionid);
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            raise6sec_array
                .push({
                    row.raise6sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6sec_array
                .push({
                    row.lower6sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60sec_array
                .push({
                    row.raise60sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60sec_array
                .push({
                    row.lower60sec
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            agc_array
                .push({
                    row.agc
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            fcascomp_array
                .push({
                    row.fcascomp
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            loadshed_array
                .push({
                    row.loadshed
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            rgul_array
                .push({
                    row.rgul
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            rguu_array
                .push({
                    row.rguu
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            reactivepower_array
                .push({
                    row.reactivepower
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            systemrestart_array
                .push({
                    row.systemrestart
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            raise6sec_gen_array
                .push({
                    row.raise6sec_gen
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower6sec_gen_array
                .push({
                    row.lower6sec_gen
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise60sec_gen_array
                .push({
                    row.raise60sec_gen
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower60sec_gen_array
                .push({
                    row.lower60sec_gen
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            agc_gen_array
                .push({
                    row.agc_gen
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            fcascomp_gen_array
                .push({
                    row.fcascomp_gen
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            loadshed_gen_array
                .push({
                    row.loadshed_gen
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            rgul_gen_array
                .push({
                    row.rgul_gen
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            rguu_gen_array
                .push({
                    row.rguu_gen
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            reactivepower_gen_array
                .push({
                    row.reactivepower_gen
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            systemrestart_gen_array
                .push({
                    row.systemrestart_gen
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower5min_array
                .push({
                    row.lower5min
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raise5min_array
                .push({
                    row.raise5min
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lowerreg_array
                .push({
                    row.lowerreg
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            raisereg_array
                .push({
                    row.raisereg
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lower5min_gen_array
                .push({
                    row.lower5min_gen
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            raise5min_gen_array
                .push({
                    row.raise5min_gen
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            lowerreg_gen_array
                .push({
                    row.lowerreg_gen
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            raisereg_gen_array
                .push({
                    row.raisereg_gen
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            availability_reactive_array
                .push({
                    row.availability_reactive
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            availability_reactive_rbt_array
                .push({
                    row.availability_reactive_rbt
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            availability_reactive_gen_array
                .push({
                    row.availability_reactive_gen
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            availability_reactive_rbt_gen_array
                .push({
                    row.availability_reactive_rbt_gen
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            raise1sec_array
                .push({
                    row.raise1sec
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            lower1sec_array
                .push({
                    row.lower1sec
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            raise1sec_gen_array
                .push({
                    row.raise1sec_gen
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            lower1sec_gen_array
                .push({
                    row.lower1sec_gen
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(agc_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(fcascomp_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(loadshed_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rgul_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rguu_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(reactivepower_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(systemrestart_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise6sec_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower6sec_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise60sec_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower60sec_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(agc_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(fcascomp_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(loadshed_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rgul_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rguu_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(reactivepower_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(systemrestart_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5min_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5min_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowerreg_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raisereg_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower5min_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise5min_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lowerreg_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raisereg_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(availability_reactive_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(availability_reactive_rbt_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(availability_reactive_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(availability_reactive_rbt_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise1sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower1sec_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(raise1sec_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lower1sec_gen_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLINGCPDATA
///  _BILLINGCPDATA shows energy quantity and $ value purchased per participant connection point._
///
/// * Data Set Name: Billing
/// * File Name: Cpdata
/// * Data Version: 7
///
/// # Description
///  BILLINGCPDATA data is confidential to relevant participant. Source Populated by the posting of a billing run, being several times each week. Volume The number of records depends on  the number of Transmission ConnectionPointIDs a participant may use to purchase energy. An indicative maximum is approximately 150 records per billrunno, or about 1,500 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONNECTIONPOINTID
/// * CONTRACTYEAR
/// * MDA
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingCpdata7 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Unique connection point identifier
    pub connectionpointid: String,
    /// Aggregate energy purchased/sold by customer, in MWh, plus UFEA. When GS commences, this includes the UFEA amount in the settlement runs.
    pub aggregateenergy: Option<rust_decimal::Decimal>,
    /// The Purchase column has the dollar value of the Energy Purchased rather than Aggregate Energy Dollar
    pub purchases: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// relevant MDA for this connection point.
    pub mda: String,
    /// Adjusted Gross Energy for this Market Customer FRMP and TNI in the Billing run, excluding any UFEA component.
    pub afe: Option<rust_decimal::Decimal>,
    /// Sum of ME- for all NMIs at this Market Customer FRMP and TNI in the Billing run.
    pub dme: Option<rust_decimal::Decimal>,
    /// Share of UFE allocated to this FRMP and TNI in the Billing run.
    pub ufea: Option<rust_decimal::Decimal>,
    /// Adjusted Gross Energy for this Market Customer FRMP and TNI in the trading interval. This will include the UFEA value once financial settlement of UFE commences with GS.
    pub age: Option<rust_decimal::Decimal>,
    /// Energy sold at the connection point by the participant in this billing run
    pub soldenergy: Option<rust_decimal::Decimal>,
    /// The total cost of energy sold at the connection point by the participant in this billing run
    pub sales: Option<rust_decimal::Decimal>,
    /// The energy consumed at the connection point by the participant in this billing run
    pub purchasedenergy: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingCpdata7 {
    type PrimaryKey = BillingCpdata7PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("CPDATA".into()),
            version: 7,
        }
    }
    fn primary_key(&self) -> BillingCpdata7PrimaryKey {
        BillingCpdata7PrimaryKey {
            billrunno: self.billrunno,
            connectionpointid: self.connectionpointid.clone(),
            contractyear: self.contractyear,
            mda: self.mda.clone(),
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_cpdata_v7".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingCpdata7PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub connectionpointid: String,
    pub contractyear: rust_decimal::Decimal,
    pub mda: String,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingCpdata7PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingCpdata7 {
    type Row = BillingCpdata7;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno
            && self.connectionpointid == row.connectionpointid
            && self.contractyear == row.contractyear && self.mda == row.mda
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingCpdata7 {
    type PrimaryKey = BillingCpdata7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno
            && self.connectionpointid == key.connectionpointid
            && self.contractyear == key.contractyear && self.mda == key.mda
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingCpdata7PrimaryKey {
    type Row = BillingCpdata7;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno
            && self.connectionpointid == row.connectionpointid
            && self.contractyear == row.contractyear && self.mda == row.mda
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingCpdata7PrimaryKey {
    type PrimaryKey = BillingCpdata7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno
            && self.connectionpointid == key.connectionpointid
            && self.contractyear == key.contractyear && self.mda == key.mda
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingCpdata7 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("aggregateenergy",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("purchases",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("mda",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("afe",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("dme",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("ufea",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("age",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("soldenergy",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("sales",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("purchasedenergy",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut aggregateenergy_array = Vec::new();
        let mut purchases_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut mda_array = Vec::new();
        let mut afe_array = Vec::new();
        let mut dme_array = Vec::new();
        let mut ufea_array = Vec::new();
        let mut age_array = Vec::new();
        let mut soldenergy_array = Vec::new();
        let mut sales_array = Vec::new();
        let mut purchasedenergy_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            connectionpointid_array.push(row.connectionpointid);
            aggregateenergy_array
                .push({
                    row.aggregateenergy
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            purchases_array
                .push({
                    row.purchases
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            mda_array.push(row.mda);
            afe_array
                .push({
                    row.afe
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            dme_array
                .push({
                    row.dme
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            ufea_array
                .push({
                    row.ufea
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            age_array
                .push({
                    row.age
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            soldenergy_array
                .push({
                    row.soldenergy
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            sales_array
                .push({
                    row.sales
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            purchasedenergy_array
                .push({
                    row.purchasedenergy
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(connectionpointid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(aggregateenergy_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(purchases_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(mda_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(afe_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(dme_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(ufea_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(age_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(soldenergy_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(sales_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(purchasedenergy_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLINGDAYTRK
///  _BILLINGDAYTRK is key for matching settlement versions with billing runs. BILLINGDAYTRK displays the billrunnos per billing week, and the settlement version numbers per settlement day comprising the billrunno._
///
/// * Data Set Name: Billing
/// * File Name: Daytrk
/// * Data Version: 5
///
/// # Description
///  BILLINGDAYTRK  is public data, and is available to all participants. Source BILLINGDAYTRK is populated by the posting of a billing run, being several times each week. Volume Each billing run inserts approximately 7 records, being about 77 records per week.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * SETTLEMENTDATE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingDaytrk5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Calendar Settlement Date contained in the billing run.
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number used for each settlement date in that billing run.
    pub runno: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingDaytrk5 {
    type PrimaryKey = BillingDaytrk5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("DAYTRK".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingDaytrk5PrimaryKey {
        BillingDaytrk5PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            settlementdate: self.settlementdate,
            weekno: self.weekno,
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
            "billing_daytrk_v5_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingDaytrk5PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingDaytrk5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingDaytrk5 {
    type Row = BillingDaytrk5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.settlementdate == row.settlementdate && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingDaytrk5 {
    type PrimaryKey = BillingDaytrk5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.settlementdate == key.settlementdate && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingDaytrk5PrimaryKey {
    type Row = BillingDaytrk5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.settlementdate == row.settlementdate && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingDaytrk5PrimaryKey {
    type PrimaryKey = BillingDaytrk5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.settlementdate == key.settlementdate && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingDaytrk5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("runno",
                arrow2::datatypes::DataType::Decimal(3, 0), true),
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            settlementdate_array.push(row.settlementdate.timestamp());
            runno_array
                .push({
                    row.runno
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(runno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
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
/// ## BILLINGFEES
///  _BILLINGFEES presents pool fees applied to the statement, per billing run._
///
/// * Data Set Name: Billing
/// * File Name: Fees
/// * Data Version: 5
///
/// # Description
///  BILLINGFEES data is confidential to the relevant participant. Source BILLINGFEES is populated by the posting of a billing run, being several times each week. Volume The number of records varies according to the number of pool fee types the participant may be subject to. An indicative maximum is about 13 records inserted per billrunno or 143 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * MARKETFEEID
/// * PARTICIPANTCATEGORYID
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingFees5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Market fee identifier
    pub marketfeeid: String,
    /// Market fee rate
    pub rate: Option<rust_decimal::Decimal>,
    /// Energy, in MWh
    pub energy: Option<rust_decimal::Decimal>,
    /// Fee in $
    pub value: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The participant category pertaining to the market fee recovery. Corresponds to the PARTICIPANTCATEGORYID column of the SETMARKETFEES table.
    pub participantcategoryid: String,
}
impl mmsdm_core::GetTable for BillingFees5 {
    type PrimaryKey = BillingFees5PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("FEES".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingFees5PrimaryKey {
        BillingFees5PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            marketfeeid: self.marketfeeid.clone(),
            participantcategoryid: self.participantcategoryid.clone(),
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_fees_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingFees5PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub marketfeeid: String,
    pub participantcategoryid: String,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingFees5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingFees5 {
    type Row = BillingFees5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.marketfeeid == row.marketfeeid
            && self.participantcategoryid == row.participantcategoryid
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingFees5 {
    type PrimaryKey = BillingFees5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.marketfeeid == key.marketfeeid
            && self.participantcategoryid == key.participantcategoryid
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingFees5PrimaryKey {
    type Row = BillingFees5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.marketfeeid == row.marketfeeid
            && self.participantcategoryid == row.participantcategoryid
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingFees5PrimaryKey {
    type PrimaryKey = BillingFees5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.marketfeeid == key.marketfeeid
            && self.participantcategoryid == key.participantcategoryid
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingFees5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("marketfeeid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("rate",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("energy",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("value",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("participantcategoryid",
                arrow2::datatypes::DataType::LargeUtf8, false)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut marketfeeid_array = Vec::new();
        let mut rate_array = Vec::new();
        let mut energy_array = Vec::new();
        let mut value_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut participantcategoryid_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            marketfeeid_array.push(row.marketfeeid);
            rate_array
                .push({
                    row.rate
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            energy_array
                .push({
                    row.energy
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            value_array
                .push({
                    row.value
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            participantcategoryid_array.push(row.participantcategoryid);
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(marketfeeid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rate_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(energy_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(value_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantcategoryid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLINGFINANCIALADJUSTMENTS
///  _BILLINGFINANCIALADJUSTMENTS contains any manual adjustments included in the billing run._
///
/// * Data Set Name: Billing
/// * File Name: Financialadjustments
/// * Data Version: 5
///
/// # Description
///  BILLINGFINANCIALADJUSTMENTS data is confidential to the relevant participant. Source BILLINGFINANCIALADJUSTMENTS is populated by the posting of a billing run, being several times each week. The insertion of a manual adjustment in a billing run is infrequent. Volume Infrequent and, if included in a billing run, low volume. An indicative maximum is 15 records inserted.
///
///
///
/// # Primary Key Columns
///
/// * ADJUSTMENTITEM
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingFinancialadjustments5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Not Used
    pub participanttype: Option<String>,
    /// Description of the adjustment being made
    pub adjustmentitem: String,
    /// The amount of the manual adjustment line item
    pub amount: Option<rust_decimal::Decimal>,
    /// Not Used
    pub value: Option<rust_decimal::Decimal>,
    /// Last date and time the record changed.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The GL financial code of the manual adjustment line item. Used internally by AEMO systems.
    pub financialcode: Option<rust_decimal::Decimal>,
    /// The BAS classification of the manual adjustment line item.
    pub bas_class: Option<String>,
}
impl mmsdm_core::GetTable for BillingFinancialadjustments5 {
    type PrimaryKey = BillingFinancialadjustments5PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("FINANCIALADJUSTMENTS".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingFinancialadjustments5PrimaryKey {
        BillingFinancialadjustments5PrimaryKey {
            adjustmentitem: self.adjustmentitem.clone(),
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_financialadjustments_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingFinancialadjustments5PrimaryKey {
    pub adjustmentitem: String,
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingFinancialadjustments5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingFinancialadjustments5 {
    type Row = BillingFinancialadjustments5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.adjustmentitem == row.adjustmentitem && self.billrunno == row.billrunno
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingFinancialadjustments5 {
    type PrimaryKey = BillingFinancialadjustments5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.adjustmentitem == key.adjustmentitem && self.billrunno == key.billrunno
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingFinancialadjustments5PrimaryKey {
    type Row = BillingFinancialadjustments5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.adjustmentitem == row.adjustmentitem && self.billrunno == row.billrunno
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingFinancialadjustments5PrimaryKey {
    type PrimaryKey = BillingFinancialadjustments5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.adjustmentitem == key.adjustmentitem && self.billrunno == key.billrunno
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingFinancialadjustments5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participanttype",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("adjustmentitem",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("amount",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("value",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("financialcode",
                arrow2::datatypes::DataType::Decimal(10, 0), true),
                arrow2::datatypes::Field::new("bas_class",
                arrow2::datatypes::DataType::LargeUtf8, true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut participanttype_array = Vec::new();
        let mut adjustmentitem_array = Vec::new();
        let mut amount_array = Vec::new();
        let mut value_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut financialcode_array = Vec::new();
        let mut bas_class_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            participanttype_array.push(row.participanttype);
            adjustmentitem_array.push(row.adjustmentitem);
            amount_array
                .push({
                    row.amount
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            value_array
                .push({
                    row.value
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            financialcode_array
                .push({
                    row.financialcode
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            bas_class_array.push(row.bas_class);
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(participanttype_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(adjustmentitem_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(value_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(financialcode_array)
                    .to(arrow2::datatypes::DataType::Decimal(10, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(bas_class_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLINGGENDATA
///  _BILLINGGENDATA shows the total energy sold and purchased per participant transmission connection point for a billing period._
///
/// * Data Set Name: Billing
/// * File Name: Gendata
/// * Data Version: 5
///
/// # Description
///  BILLINGGENDATA data is confidential to the the relevant participant. Source BILLINGGENDATA is populated by the posting of a billing run, being several times each week. Volume The number of records depends on the number of transmission ConnectionPointIDs a Participant may have sold energy from per billrunno.  An indicative maximum is approximately 15 records inserted per billrunno, or about 165 records inserted per week. BILLINGGENDATA is confidential to the the relevant participant.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONNECTIONPOINTID
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingGendata5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Connection point identifier
    pub connectionpointid: String,
    /// not populated
    pub stationid: Option<String>,
    /// not populated
    pub duid: Option<String>,
    /// Aggregate energy sold, in MWh
    pub aggregateenergy: Option<rust_decimal::Decimal>,
    /// $ income
    pub sales: Option<rust_decimal::Decimal>,
    /// $ outgoing
    pub purchases: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Amount of energy purchased in MWh
    pub purchasedenergy: Option<rust_decimal::Decimal>,
    /// Metering Data Agent supplying data
    pub mda: Option<String>,
}
impl mmsdm_core::GetTable for BillingGendata5 {
    type PrimaryKey = BillingGendata5PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("GENDATA".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingGendata5PrimaryKey {
        BillingGendata5PrimaryKey {
            billrunno: self.billrunno,
            connectionpointid: self.connectionpointid.clone(),
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_gendata_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingGendata5PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub connectionpointid: String,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingGendata5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingGendata5 {
    type Row = BillingGendata5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno
            && self.connectionpointid == row.connectionpointid
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingGendata5 {
    type PrimaryKey = BillingGendata5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno
            && self.connectionpointid == key.connectionpointid
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingGendata5PrimaryKey {
    type Row = BillingGendata5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno
            && self.connectionpointid == row.connectionpointid
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingGendata5PrimaryKey {
    type PrimaryKey = BillingGendata5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno
            && self.connectionpointid == key.connectionpointid
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingGendata5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("stationid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("duid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("aggregateenergy",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("sales",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("purchases",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("purchasedenergy",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("mda",
                arrow2::datatypes::DataType::LargeUtf8, true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut stationid_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut aggregateenergy_array = Vec::new();
        let mut sales_array = Vec::new();
        let mut purchases_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut purchasedenergy_array = Vec::new();
        let mut mda_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            connectionpointid_array.push(row.connectionpointid);
            stationid_array.push(row.stationid);
            duid_array.push(row.duid);
            aggregateenergy_array
                .push({
                    row.aggregateenergy
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            sales_array
                .push({
                    row.sales
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            purchases_array
                .push({
                    row.purchases
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            purchasedenergy_array
                .push({
                    row.purchasedenergy
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            mda_array.push(row.mda);
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(connectionpointid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(stationid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(duid_array)) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(aggregateenergy_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(sales_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(purchases_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(purchasedenergy_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(mda_array)) as std::sync::Arc < dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLINGINTERRESIDUES
///  _BILLINGINTERRESIDUES shows interregion residues payable to NSP._
///
/// * Data Set Name: Billing
/// * File Name: Interresidues
/// * Data Version: 5
///
/// # Description
///  Source Obsolete, was weekly with billing run.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingInterresidues5 {
    /// May not be necessary
    pub allocation: Option<rust_decimal::Decimal>,
    /// May not be necessary
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// Unique identifier for an interconnector which joins two regions.
    pub interconnectorid: String,
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Amount NSP is paid for Inter-Regional Residues
    pub surplusvalue: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Region ID
    pub regionid: String,
}
impl mmsdm_core::GetTable for BillingInterresidues5 {
    type PrimaryKey = BillingInterresidues5PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("INTERRESIDUES".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingInterresidues5PrimaryKey {
        BillingInterresidues5PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            interconnectorid: self.interconnectorid.clone(),
            participantid: self.participantid.clone(),
            regionid: self.regionid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_interresidues_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingInterresidues5PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub interconnectorid: String,
    pub participantid: String,
    pub regionid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingInterresidues5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingInterresidues5 {
    type Row = BillingInterresidues5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingInterresidues5 {
    type PrimaryKey = BillingInterresidues5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingInterresidues5PrimaryKey {
    type Row = BillingInterresidues5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingInterresidues5PrimaryKey {
    type PrimaryKey = BillingInterresidues5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingInterresidues5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("allocation",
                arrow2::datatypes::DataType::Decimal(6, 3), true),
                arrow2::datatypes::Field::new("totalsurplus",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("surplusvalue",
                arrow2::datatypes::DataType::Decimal(15, 6), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut allocation_array = Vec::new();
        let mut totalsurplus_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut surplusvalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut regionid_array = Vec::new();
        for row in partition {
            allocation_array
                .push({
                    row.allocation
                        .map(|mut val| {
                            val.rescale(3);
                            val.mantissa()
                        })
                });
            totalsurplus_array
                .push({
                    row.totalsurplus
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            interconnectorid_array.push(row.interconnectorid);
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            surplusvalue_array
                .push({
                    row.surplusvalue
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            regionid_array.push(row.regionid);
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(allocation_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 3))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalsurplus_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(interconnectorid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(surplusvalue_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLINGINTRARESIDUES
///  _BILLINGINTRARESIDUES shows intra-region settlement residue details for each Transmission Network Service Provider participant by region._
///
/// * Data Set Name: Billing
/// * File Name: Intraresidues
/// * Data Version: 5
///
/// # Description
///  BILLINGINTRARESIDUES is confidential to the relevant participant. Source BILLINGINTRARESIDUES is populated by the posting of a billing run, being several times each week. Volume An indicative maximum is two records inserted per billing run, or 22 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIntraresidues5 {
    /// TNSP allocation
    pub allocation: Option<rust_decimal::Decimal>,
    /// Total $ residue amount for the region
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Amount TNSP is paid for Intra-Regional Residues
    pub surplusvalue: Option<rust_decimal::Decimal>,
    /// Last changed date
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Region ID
    pub regionid: String,
}
impl mmsdm_core::GetTable for BillingIntraresidues5 {
    type PrimaryKey = BillingIntraresidues5PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("INTRARESIDUES".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingIntraresidues5PrimaryKey {
        BillingIntraresidues5PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            regionid: self.regionid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_intraresidues_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingIntraresidues5PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub regionid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingIntraresidues5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingIntraresidues5 {
    type Row = BillingIntraresidues5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingIntraresidues5 {
    type PrimaryKey = BillingIntraresidues5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingIntraresidues5PrimaryKey {
    type Row = BillingIntraresidues5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingIntraresidues5PrimaryKey {
    type PrimaryKey = BillingIntraresidues5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingIntraresidues5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("allocation",
                arrow2::datatypes::DataType::Decimal(6, 3), true),
                arrow2::datatypes::Field::new("totalsurplus",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("surplusvalue",
                arrow2::datatypes::DataType::Decimal(15, 6), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut allocation_array = Vec::new();
        let mut totalsurplus_array = Vec::new();
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut surplusvalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut regionid_array = Vec::new();
        for row in partition {
            allocation_array
                .push({
                    row.allocation
                        .map(|mut val| {
                            val.rescale(3);
                            val.mantissa()
                        })
                });
            totalsurplus_array
                .push({
                    row.totalsurplus
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            surplusvalue_array
                .push({
                    row.surplusvalue
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            regionid_array.push(row.regionid);
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(allocation_array)
                    .to(arrow2::datatypes::DataType::Decimal(6, 3))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalsurplus_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(surplusvalue_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLINGIRAUCSURPLUS
///  _BILLINGIRAUCSURPLUS supports the Settlements Residue Auction, by showing the weekly billing Interconnector Residue (IR) payments as calculated for each bill run for Network Service Providers (NSPs) from the amount not auctioned._
///
/// * Data Set Name: Billing
/// * File Name: Iraucsurplus
/// * Data Version: 5
///
/// # Description
///  Source Obsolete Volume This view contains a maximum of 30, 000 records per year.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTID
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIraucsurplus5 {
    /// SRA Contracted Year (calendar year)
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    pub residueyear: Option<rust_decimal::Decimal>,
    /// Residue Contract Quarter
    pub quarter: Option<rust_decimal::Decimal>,
    /// The sequential number of a billing run
    pub billrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    pub contractid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Total residues allocated to participant
    pub totalresidues: Option<rust_decimal::Decimal>,
    /// Adjustment allocated to participant
    pub adjustment: Option<rust_decimal::Decimal>,
    /// Date and time this record was last modified
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingIraucsurplus5 {
    type PrimaryKey = BillingIraucsurplus5PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("IRAUCSURPLUS".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingIraucsurplus5PrimaryKey {
        BillingIraucsurplus5PrimaryKey {
            billrunno: self.billrunno,
            contractid: self.contractid.clone(),
            contractyear: self.contractyear,
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_iraucsurplus_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingIraucsurplus5PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractid: String,
    pub contractyear: rust_decimal::Decimal,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingIraucsurplus5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingIraucsurplus5 {
    type Row = BillingIraucsurplus5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractid == row.contractid
            && self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingIraucsurplus5 {
    type PrimaryKey = BillingIraucsurplus5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractid == key.contractid
            && self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingIraucsurplus5PrimaryKey {
    type Row = BillingIraucsurplus5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractid == row.contractid
            && self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingIraucsurplus5PrimaryKey {
    type PrimaryKey = BillingIraucsurplus5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractid == key.contractid
            && self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingIraucsurplus5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(2, 0), false),
                arrow2::datatypes::Field::new("residueyear",
                arrow2::datatypes::DataType::Decimal(4, 0), true),
                arrow2::datatypes::Field::new("quarter",
                arrow2::datatypes::DataType::Decimal(2, 0), true),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("contractid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("fromregionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("totalresidues",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("adjustment",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut residueyear_array = Vec::new();
        let mut quarter_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut totalresidues_array = Vec::new();
        let mut adjustment_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            residueyear_array
                .push({
                    row.residueyear
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            quarter_array
                .push({
                    row.quarter
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            contractid_array.push(row.contractid);
            participantid_array.push(row.participantid);
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            totalresidues_array
                .push({
                    row.totalresidues
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            adjustment_array
                .push({
                    row.adjustment
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(2, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(residueyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(quarter_array)
                    .to(arrow2::datatypes::DataType::Decimal(2, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(contractid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(interconnectorid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(fromregionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalresidues_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(adjustment_array)
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
/// ## BILLINGIRAUCSURPLUSSUM
///  _BILLINGIRAUCSURPLUSSUM contains Auction fees and Settlements Residue Auction distribution that may arise from unpurchased auction units that accrue to Transmission Network Service Providers._
///
/// * Data Set Name: Billing
/// * File Name: Iraucsurplussum
/// * Data Version: 7
///
/// # Description
///  BILLINGIRAUCSURPLUSSUM is confidential to the relevant participant. Source BILLINGIRAUCSURPLUSSUM is populated by the posting of a billing run where there are unpurchased auction units. Volume An indicative maximum is eight records inserted per billing run, or 88 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * QUARTER
/// * RESIDUEYEAR
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIraucsurplussum7 {
    /// Contracted Year (calendar year)
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    pub residueyear: rust_decimal::Decimal,
    /// Residue Contract Quarter
    pub quarter: rust_decimal::Decimal,
    /// The sequential number of a billing run
    pub billrunno: rust_decimal::Decimal,
    /// Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Total residue amount allocated to participant
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// Total auction fees payable in this week (negative amount). If AUCTIONFEES + AUCTIONFEES_GST &gt;= TOTALSURPLUS then ACTUALPAYMENT is zero
    pub auctionfees: Option<rust_decimal::Decimal>,
    /// Net payment to participant, including auction fees
    pub actualpayment: Option<rust_decimal::Decimal>,
    /// The GST amount on the auction fees, always being zero.
    pub auctionfees_gst: Option<rust_decimal::Decimal>,
    /// Date and time this record was last modified
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA.
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP.
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
    /// Negative residues in the billing week for this participant in the SRA Year/Quarter
    pub negative_residues: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingIraucsurplussum7 {
    type PrimaryKey = BillingIraucsurplussum7PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("IRAUCSURPLUSSUM".into()),
            version: 7,
        }
    }
    fn primary_key(&self) -> BillingIraucsurplussum7PrimaryKey {
        BillingIraucsurplussum7PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            participantid: self.participantid.clone(),
            quarter: self.quarter,
            residueyear: self.residueyear,
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_iraucsurplussum_v7".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingIraucsurplussum7PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub participantid: String,
    pub quarter: rust_decimal::Decimal,
    pub residueyear: rust_decimal::Decimal,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingIraucsurplussum7PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingIraucsurplussum7 {
    type Row = BillingIraucsurplussum7;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid && self.quarter == row.quarter
            && self.residueyear == row.residueyear && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingIraucsurplussum7 {
    type PrimaryKey = BillingIraucsurplussum7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.quarter == key.quarter
            && self.residueyear == key.residueyear && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingIraucsurplussum7PrimaryKey {
    type Row = BillingIraucsurplussum7;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid && self.quarter == row.quarter
            && self.residueyear == row.residueyear && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingIraucsurplussum7PrimaryKey {
    type PrimaryKey = BillingIraucsurplussum7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.quarter == key.quarter
            && self.residueyear == key.residueyear && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingIraucsurplussum7 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("residueyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("quarter",
                arrow2::datatypes::DataType::Decimal(2, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("fromregionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("totalsurplus",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("auctionfees",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("actualpayment",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("auctionfees_gst",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("csp_derogation_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("unadjusted_irsr",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("negative_residues",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut residueyear_array = Vec::new();
        let mut quarter_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut totalsurplus_array = Vec::new();
        let mut auctionfees_array = Vec::new();
        let mut actualpayment_array = Vec::new();
        let mut auctionfees_gst_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut csp_derogation_amount_array = Vec::new();
        let mut unadjusted_irsr_array = Vec::new();
        let mut negative_residues_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            residueyear_array
                .push({
                    let mut val = row.residueyear;
                    val.rescale(0);
                    val.mantissa()
                });
            quarter_array
                .push({
                    let mut val = row.quarter;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            participantid_array.push(row.participantid);
            totalsurplus_array
                .push({
                    row.totalsurplus
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            auctionfees_array
                .push({
                    row.auctionfees
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            actualpayment_array
                .push({
                    row.actualpayment
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            auctionfees_gst_array
                .push({
                    row.auctionfees_gst
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            csp_derogation_amount_array
                .push({
                    row.csp_derogation_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            unadjusted_irsr_array
                .push({
                    row.unadjusted_irsr
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            negative_residues_array
                .push({
                    row.negative_residues
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(residueyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(quarter_array)
                    .to(arrow2::datatypes::DataType::Decimal(2, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(interconnectorid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(fromregionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalsurplus_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(auctionfees_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(actualpayment_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(auctionfees_gst_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(csp_derogation_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(unadjusted_irsr_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(negative_residues_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLINGIRNSPSURPLUS
///  _BILLINGIRNSPSURPLUS supports the Settlements Residue Auction (SRA), by showing the weekly billing Interconnector Residue (IR) payments as calculated for each bill run for Transmission Network Service Providers (TNSP) from the amount paid by participants (i.e. derogated amounts)._
///
/// * Data Set Name: Billing
/// * File Name: Irnspsurplus
/// * Data Version: 5
///
/// # Description
///  BILLINGIRNSPSURPLUS data is confidential to the relevant participant. Source BILLINGIRNSPSURPLUS updates in a billing run where any derogated Settlement Residue Auction purchase flows to a TNSP. Volume BILLINGIRNSPSURPLUS contains a maximum of 30, 000 records per year.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTID
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrnspsurplus5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    pub residueyear: Option<rust_decimal::Decimal>,
    /// Residue Contract Quarter
    pub quarter: Option<rust_decimal::Decimal>,
    /// The sequential number of a billing run
    pub billrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    pub contractid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Total residues allocated to participant
    pub totalresidues: Option<rust_decimal::Decimal>,
    /// Adjustment allocated to participant
    pub adjustment: Option<rust_decimal::Decimal>,
    /// Date and time this record was last modified
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingIrnspsurplus5 {
    type PrimaryKey = BillingIrnspsurplus5PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("IRNSPSURPLUS".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingIrnspsurplus5PrimaryKey {
        BillingIrnspsurplus5PrimaryKey {
            billrunno: self.billrunno,
            contractid: self.contractid.clone(),
            contractyear: self.contractyear,
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_irnspsurplus_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingIrnspsurplus5PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractid: String,
    pub contractyear: rust_decimal::Decimal,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingIrnspsurplus5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingIrnspsurplus5 {
    type Row = BillingIrnspsurplus5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractid == row.contractid
            && self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingIrnspsurplus5 {
    type PrimaryKey = BillingIrnspsurplus5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractid == key.contractid
            && self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingIrnspsurplus5PrimaryKey {
    type Row = BillingIrnspsurplus5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractid == row.contractid
            && self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingIrnspsurplus5PrimaryKey {
    type PrimaryKey = BillingIrnspsurplus5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractid == key.contractid
            && self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingIrnspsurplus5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(2, 0), false),
                arrow2::datatypes::Field::new("residueyear",
                arrow2::datatypes::DataType::Decimal(4, 0), true),
                arrow2::datatypes::Field::new("quarter",
                arrow2::datatypes::DataType::Decimal(2, 0), true),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("contractid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("fromregionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("totalresidues",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("adjustment",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut residueyear_array = Vec::new();
        let mut quarter_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut totalresidues_array = Vec::new();
        let mut adjustment_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            residueyear_array
                .push({
                    row.residueyear
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            quarter_array
                .push({
                    row.quarter
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            contractid_array.push(row.contractid);
            participantid_array.push(row.participantid);
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            totalresidues_array
                .push({
                    row.totalresidues
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            adjustment_array
                .push({
                    row.adjustment
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(2, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(residueyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(quarter_array)
                    .to(arrow2::datatypes::DataType::Decimal(2, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(contractid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(interconnectorid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(fromregionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalresidues_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(adjustment_array)
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
/// ## BILLINGIRNSPSURPLUSSUM
///  _BILLINGIRNSPSURPLUSSUM contains derogated payments made to TNSPs arising from the Settlements Residue Auction process._
///
/// * Data Set Name: Billing
/// * File Name: Irnspsurplussum
/// * Data Version: 6
///
/// # Description
///  BILLINGIRNSPSURPLUSSUM data is confidential to the relevant participant. Source BILLINGIRNSPSURPLUSSUM is populated by the posting of a billing run where derogated payments apply. Volume An indicative maximum is two records inserted per billing run, or 22 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * QUARTER
/// * RESIDUEYEAR
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrnspsurplussum6 {
    /// SRA Contracted Year (calendar year)
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    pub residueyear: rust_decimal::Decimal,
    /// SRA Contracted Quarter
    pub quarter: rust_decimal::Decimal,
    /// The sequential number of a billing run
    pub billrunno: rust_decimal::Decimal,
    /// Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Total residue amount allocated to participant
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// This field is 0.
    pub auctionfees: Option<rust_decimal::Decimal>,
    /// The GST amount on the auction fees, always being zero.
    pub auctionfees_gst: Option<rust_decimal::Decimal>,
    /// Date and time this record was last modified
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA.
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP.
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingIrnspsurplussum6 {
    type PrimaryKey = BillingIrnspsurplussum6PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("IRNSPSURPLUSSUM".into()),
            version: 6,
        }
    }
    fn primary_key(&self) -> BillingIrnspsurplussum6PrimaryKey {
        BillingIrnspsurplussum6PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            participantid: self.participantid.clone(),
            quarter: self.quarter,
            residueyear: self.residueyear,
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_irnspsurplussum_v6".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingIrnspsurplussum6PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub participantid: String,
    pub quarter: rust_decimal::Decimal,
    pub residueyear: rust_decimal::Decimal,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingIrnspsurplussum6PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingIrnspsurplussum6 {
    type Row = BillingIrnspsurplussum6;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid && self.quarter == row.quarter
            && self.residueyear == row.residueyear && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingIrnspsurplussum6 {
    type PrimaryKey = BillingIrnspsurplussum6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.quarter == key.quarter
            && self.residueyear == key.residueyear && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingIrnspsurplussum6PrimaryKey {
    type Row = BillingIrnspsurplussum6;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid && self.quarter == row.quarter
            && self.residueyear == row.residueyear && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingIrnspsurplussum6PrimaryKey {
    type PrimaryKey = BillingIrnspsurplussum6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.quarter == key.quarter
            && self.residueyear == key.residueyear && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingIrnspsurplussum6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("residueyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("quarter",
                arrow2::datatypes::DataType::Decimal(2, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("fromregionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("totalsurplus",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("auctionfees",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("auctionfees_gst",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("csp_derogation_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("unadjusted_irsr",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut residueyear_array = Vec::new();
        let mut quarter_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut totalsurplus_array = Vec::new();
        let mut auctionfees_array = Vec::new();
        let mut auctionfees_gst_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut csp_derogation_amount_array = Vec::new();
        let mut unadjusted_irsr_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            residueyear_array
                .push({
                    let mut val = row.residueyear;
                    val.rescale(0);
                    val.mantissa()
                });
            quarter_array
                .push({
                    let mut val = row.quarter;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            participantid_array.push(row.participantid);
            totalsurplus_array
                .push({
                    row.totalsurplus
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            auctionfees_array
                .push({
                    row.auctionfees
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            auctionfees_gst_array
                .push({
                    row.auctionfees_gst
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            csp_derogation_amount_array
                .push({
                    row.csp_derogation_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            unadjusted_irsr_array
                .push({
                    row.unadjusted_irsr
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(residueyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(quarter_array)
                    .to(arrow2::datatypes::DataType::Decimal(2, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(interconnectorid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(fromregionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalsurplus_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(auctionfees_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(auctionfees_gst_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(csp_derogation_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(unadjusted_irsr_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLINGIRPARTSURPLUS
///  _BILLINGIRPARTSURPLUS supports the Settlements Residue Auction, by showing the weekly billing SRA distribution to Auction participants by Contract Identifier._
///
/// * Data Set Name: Billing
/// * File Name: Irpartsurplus
/// * Data Version: 5
///
/// # Description
///  BILLINGIRPARTSURPLUS data is confidential to the relevant participant. Source BILLINGIRPARTSURPLUS is populated by the posting of a billing run where the participant has purchased auction units relating to that billing run. Volume An indicative maximum is 64 records inserted per billing run, or 700 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTID
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrpartsurplus5 {
    /// SRA Contracted Year (calendar year)
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    pub residueyear: Option<rust_decimal::Decimal>,
    /// Residue Contract Quarter
    pub quarter: Option<rust_decimal::Decimal>,
    /// The sequential number of a billing run
    pub billrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    pub contractid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Total residues allocated to participant
    pub totalresidues: Option<rust_decimal::Decimal>,
    /// Adjustment allocated to participant
    pub adjustment: Option<rust_decimal::Decimal>,
    /// Date and time this record was last modified
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Net actual payment to participant, including auction fees
    pub actualpayment: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingIrpartsurplus5 {
    type PrimaryKey = BillingIrpartsurplus5PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("IRPARTSURPLUS".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingIrpartsurplus5PrimaryKey {
        BillingIrpartsurplus5PrimaryKey {
            billrunno: self.billrunno,
            contractid: self.contractid.clone(),
            contractyear: self.contractyear,
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_irpartsurplus_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingIrpartsurplus5PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractid: String,
    pub contractyear: rust_decimal::Decimal,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingIrpartsurplus5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingIrpartsurplus5 {
    type Row = BillingIrpartsurplus5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractid == row.contractid
            && self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingIrpartsurplus5 {
    type PrimaryKey = BillingIrpartsurplus5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractid == key.contractid
            && self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingIrpartsurplus5PrimaryKey {
    type Row = BillingIrpartsurplus5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractid == row.contractid
            && self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingIrpartsurplus5PrimaryKey {
    type PrimaryKey = BillingIrpartsurplus5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractid == key.contractid
            && self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingIrpartsurplus5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(2, 0), false),
                arrow2::datatypes::Field::new("residueyear",
                arrow2::datatypes::DataType::Decimal(4, 0), true),
                arrow2::datatypes::Field::new("quarter",
                arrow2::datatypes::DataType::Decimal(2, 0), true),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("contractid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("fromregionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("totalresidues",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("adjustment",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("actualpayment",
                arrow2::datatypes::DataType::Decimal(15, 5), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut residueyear_array = Vec::new();
        let mut quarter_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut totalresidues_array = Vec::new();
        let mut adjustment_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut actualpayment_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            residueyear_array
                .push({
                    row.residueyear
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            quarter_array
                .push({
                    row.quarter
                        .map(|mut val| {
                            val.rescale(0);
                            val.mantissa()
                        })
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            contractid_array.push(row.contractid);
            participantid_array.push(row.participantid);
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            totalresidues_array
                .push({
                    row.totalresidues
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            adjustment_array
                .push({
                    row.adjustment
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            actualpayment_array
                .push({
                    row.actualpayment
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(2, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(residueyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(quarter_array)
                    .to(arrow2::datatypes::DataType::Decimal(2, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(contractid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(interconnectorid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(fromregionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalresidues_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(adjustment_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(actualpayment_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLINGIRPARTSURPLUSSUM
///  _BILLINGIRPARTSURPLUSSUM supports the Settlements Residue Auction, by showing the weekly billing SRA distribution and associated fees to Auction participants._
///
/// * Data Set Name: Billing
/// * File Name: Irpartsurplussum
/// * Data Version: 7
///
/// # Description
///  BILLINGIRPARTSURPLUSSUM data is confidential to the relevant participant. Source BILLINGIRPARTSURPLUSSUM is populated by the posting of a billing run where the participant has purchased auction units relating to that billing run. Volume An indicative maximum is 16 records inserted per billing run, or 166 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * QUARTER
/// * RESIDUEYEAR
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingIrpartsurplussum7 {
    /// SRA Contracted Year (calendar year)
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Year of the Residue Contract; may differ from the calendar year at week 1.
    pub residueyear: rust_decimal::Decimal,
    /// Residue Contract Quarter
    pub quarter: rust_decimal::Decimal,
    /// The sequential number of a billing run
    pub billrunno: rust_decimal::Decimal,
    /// Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Unique participant identifier
    pub participantid: String,
    /// Total residue amount allocated to participant
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// Total auction fees payable in this week (negative amount). If AUCTIONFEES + AUCTIONFEES_GST &gt;= TOTALSURPLUS then ACTUALPAYMENT is zero.
    pub auctionfees: Option<rust_decimal::Decimal>,
    /// Net payment to participant, including auction fees
    pub actualpayment: Option<rust_decimal::Decimal>,
    /// The GST amount on the auction fees, always being zero.
    pub auctionfees_gst: Option<rust_decimal::Decimal>,
    /// Date and time this record was last modified
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA.
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP.
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
    /// The adjusted total Auction fees for the Directional Interconnector. Calculated as the amount of the total fees due from the SRA Auction Participant, pro-rated based on the total surplus for each Directional Interconnector the SRA Auction Participant contracted.
    pub auctionfees_totalgross_adj: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingIrpartsurplussum7 {
    type PrimaryKey = BillingIrpartsurplussum7PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("IRPARTSURPLUSSUM".into()),
            version: 7,
        }
    }
    fn primary_key(&self) -> BillingIrpartsurplussum7PrimaryKey {
        BillingIrpartsurplussum7PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            participantid: self.participantid.clone(),
            quarter: self.quarter,
            residueyear: self.residueyear,
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_irpartsurplussum_v7".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingIrpartsurplussum7PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub participantid: String,
    pub quarter: rust_decimal::Decimal,
    pub residueyear: rust_decimal::Decimal,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingIrpartsurplussum7PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingIrpartsurplussum7 {
    type Row = BillingIrpartsurplussum7;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid && self.quarter == row.quarter
            && self.residueyear == row.residueyear && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingIrpartsurplussum7 {
    type PrimaryKey = BillingIrpartsurplussum7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.quarter == key.quarter
            && self.residueyear == key.residueyear && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingIrpartsurplussum7PrimaryKey {
    type Row = BillingIrpartsurplussum7;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid && self.quarter == row.quarter
            && self.residueyear == row.residueyear && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingIrpartsurplussum7PrimaryKey {
    type PrimaryKey = BillingIrpartsurplussum7PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid && self.quarter == key.quarter
            && self.residueyear == key.residueyear && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingIrpartsurplussum7 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("residueyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("quarter",
                arrow2::datatypes::DataType::Decimal(2, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("fromregionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("totalsurplus",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("auctionfees",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("actualpayment",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("auctionfees_gst",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("csp_derogation_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("unadjusted_irsr",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("auctionfees_totalgross_adj",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut residueyear_array = Vec::new();
        let mut quarter_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut totalsurplus_array = Vec::new();
        let mut auctionfees_array = Vec::new();
        let mut actualpayment_array = Vec::new();
        let mut auctionfees_gst_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut csp_derogation_amount_array = Vec::new();
        let mut unadjusted_irsr_array = Vec::new();
        let mut auctionfees_totalgross_adj_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            residueyear_array
                .push({
                    let mut val = row.residueyear;
                    val.rescale(0);
                    val.mantissa()
                });
            quarter_array
                .push({
                    let mut val = row.quarter;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            participantid_array.push(row.participantid);
            totalsurplus_array
                .push({
                    row.totalsurplus
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            auctionfees_array
                .push({
                    row.auctionfees
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            actualpayment_array
                .push({
                    row.actualpayment
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            auctionfees_gst_array
                .push({
                    row.auctionfees_gst
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            csp_derogation_amount_array
                .push({
                    row.csp_derogation_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            unadjusted_irsr_array
                .push({
                    row.unadjusted_irsr
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            auctionfees_totalgross_adj_array
                .push({
                    row.auctionfees_totalgross_adj
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(residueyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(quarter_array)
                    .to(arrow2::datatypes::DataType::Decimal(2, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(interconnectorid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(fromregionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(totalsurplus_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(auctionfees_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(actualpayment_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(auctionfees_gst_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(csp_derogation_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(unadjusted_irsr_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(auctionfees_totalgross_adj_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLINGPRIORADJUSTMENTS
///  _BILLINGPRIORADJUSTMENTS sets out prior period adjustments and associated interest inserted in subsequent Final Statements arising from Revision Statement postings._
///
/// * Data Set Name: Billing
/// * File Name: Prioradjustments
/// * Data Version: 5
///
/// # Description
///  BILLINGPRIORADJUSTMENTS data is confidential to the relevant participant. Source BILLINGPRIORADJUSTMENTS is populated on the posting of a Final billing run only. Volume Approximately two records inserted per week. Note Actual adjustment payable is ADJAMOUNT - PERAMOUNT + INTEREST AMOUNT.
///
///
///
/// # Primary Key Columns
///
/// * ADJBILLRUNNO
/// * ADJCONTRACTYEAR
/// * ADJWEEKNO
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingPrioradjustments5 {
    /// Settlement year.
    pub contractyear: rust_decimal::Decimal,
    /// Settlement week number.
    pub weekno: rust_decimal::Decimal,
    /// Billing run number.
    pub billrunno: rust_decimal::Decimal,
    /// ContractYear of the posted revision statement inserted to the Final Statement
    pub adjcontractyear: rust_decimal::Decimal,
    /// WeekNo of the posted revision statement inserted to the Final Statement
    pub adjweekno: rust_decimal::Decimal,
    /// Bill run number of the posted revision statement inserted to the Final Statement
    pub adjbillrunno: rust_decimal::Decimal,
    /// Participant ID
    pub participantid: String,
    /// Statement total of the previous posted revision statement inserted to the Final Statement.
    pub prevamount: Option<rust_decimal::Decimal>,
    /// Adjusted amount.
    pub adjamount: Option<rust_decimal::Decimal>,
    /// Interest rate applied to the revision adjustment
    pub irn: Option<rust_decimal::Decimal>,
    /// unused; always null
    pub irp: Option<rust_decimal::Decimal>,
    /// Interest amount.
    pub interestamount: Option<rust_decimal::Decimal>,
    /// Last changed.
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// unused; always null
    pub irsr_prevamount: Option<rust_decimal::Decimal>,
    /// unused; always null
    pub irsr_adjamount: Option<rust_decimal::Decimal>,
    /// unused; always null
    pub irsr_interestamount: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingPrioradjustments5 {
    type PrimaryKey = BillingPrioradjustments5PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("PRIORADJUSTMENTS".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingPrioradjustments5PrimaryKey {
        BillingPrioradjustments5PrimaryKey {
            adjbillrunno: self.adjbillrunno,
            adjcontractyear: self.adjcontractyear,
            adjweekno: self.adjweekno,
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_prioradjustments_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingPrioradjustments5PrimaryKey {
    pub adjbillrunno: rust_decimal::Decimal,
    pub adjcontractyear: rust_decimal::Decimal,
    pub adjweekno: rust_decimal::Decimal,
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingPrioradjustments5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingPrioradjustments5 {
    type Row = BillingPrioradjustments5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.adjbillrunno == row.adjbillrunno
            && self.adjcontractyear == row.adjcontractyear
            && self.adjweekno == row.adjweekno && self.billrunno == row.billrunno
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingPrioradjustments5 {
    type PrimaryKey = BillingPrioradjustments5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.adjbillrunno == key.adjbillrunno
            && self.adjcontractyear == key.adjcontractyear
            && self.adjweekno == key.adjweekno && self.billrunno == key.billrunno
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingPrioradjustments5PrimaryKey {
    type Row = BillingPrioradjustments5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.adjbillrunno == row.adjbillrunno
            && self.adjcontractyear == row.adjcontractyear
            && self.adjweekno == row.adjweekno && self.billrunno == row.billrunno
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingPrioradjustments5PrimaryKey {
    type PrimaryKey = BillingPrioradjustments5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.adjbillrunno == key.adjbillrunno
            && self.adjcontractyear == key.adjcontractyear
            && self.adjweekno == key.adjweekno && self.billrunno == key.billrunno
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingPrioradjustments5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("adjcontractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("adjweekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("adjbillrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("prevamount",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("adjamount",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("irn",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("irp",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("interestamount",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("irsr_prevamount",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("irsr_adjamount",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("irsr_interestamount",
                arrow2::datatypes::DataType::Decimal(15, 5), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut adjcontractyear_array = Vec::new();
        let mut adjweekno_array = Vec::new();
        let mut adjbillrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut prevamount_array = Vec::new();
        let mut adjamount_array = Vec::new();
        let mut irn_array = Vec::new();
        let mut irp_array = Vec::new();
        let mut interestamount_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut irsr_prevamount_array = Vec::new();
        let mut irsr_adjamount_array = Vec::new();
        let mut irsr_interestamount_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            adjcontractyear_array
                .push({
                    let mut val = row.adjcontractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            adjweekno_array
                .push({
                    let mut val = row.adjweekno;
                    val.rescale(0);
                    val.mantissa()
                });
            adjbillrunno_array
                .push({
                    let mut val = row.adjbillrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            prevamount_array
                .push({
                    row.prevamount
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            adjamount_array
                .push({
                    row.adjamount
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            irn_array
                .push({
                    row.irn
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            irp_array
                .push({
                    row.irp
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            interestamount_array
                .push({
                    row.interestamount
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            irsr_prevamount_array
                .push({
                    row.irsr_prevamount
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            irsr_adjamount_array
                .push({
                    row.irsr_adjamount
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            irsr_interestamount_array
                .push({
                    row.irsr_interestamount
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(adjcontractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(adjweekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(adjbillrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(prevamount_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(adjamount_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(irn_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(irp_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(interestamount_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(irsr_prevamount_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(irsr_adjamount_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(irsr_interestamount_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLINGREALLOC
///  _BILLINGREALLOC shows reallocation contract values in each billing run, where participants have used reallocations._
///
/// * Data Set Name: Billing
/// * File Name: Realloc
/// * Data Version: 5
///
/// # Description
///  BILLINGREALLOC data is confidential to the relevant participant. Source BILLINGREALLOC is populated by the posting of a billing run. Volume An indicative maximum is two records inserted per billing run, or 22 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * COUNTERPARTY
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRealloc5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Participant who is the counter party to this contract
    pub counterparty: String,
    /// Value billed on this contract
    pub value: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingRealloc5 {
    type PrimaryKey = BillingRealloc5PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("REALLOC".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingRealloc5PrimaryKey {
        BillingRealloc5PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            counterparty: self.counterparty.clone(),
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_realloc_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingRealloc5PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub counterparty: String,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingRealloc5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingRealloc5 {
    type Row = BillingRealloc5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.counterparty == row.counterparty
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingRealloc5 {
    type PrimaryKey = BillingRealloc5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.counterparty == key.counterparty
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingRealloc5PrimaryKey {
    type Row = BillingRealloc5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.counterparty == row.counterparty
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingRealloc5PrimaryKey {
    type PrimaryKey = BillingRealloc5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.counterparty == key.counterparty
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingRealloc5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("counterparty",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("value",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut counterparty_array = Vec::new();
        let mut value_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            counterparty_array.push(row.counterparty);
            value_array
                .push({
                    row.value
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(counterparty_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(value_array)
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
/// ## BILLINGREALLOC_DETAIL
///  _Billing Reallocation Data aggregated by REALLOCATIONID for each billing run over the billing week._
///
/// * Data Set Name: Billing
/// * File Name: Realloc Detail
/// * Data Version: 5
///
/// # Description
///  The BILLINGREALLOC_DETAIL table that will give a breakdown of the reallocations that form part of that billing run. This assists participants in their settlement reconciliation process. &nbsp; Private data Volume max 100 rows per day
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * COUNTERPARTY
/// * PARTICIPANTID
/// * REALLOCATIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingReallocDetail5 {
    /// BILLING CONTRACTYEAR
    pub contractyear: rust_decimal::Decimal,
    /// BILLING WEEKNO
    pub weekno: rust_decimal::Decimal,
    /// BILLING RUN NO
    pub billrunno: rust_decimal::Decimal,
    /// REALLOCATION PARTICIPANTID
    pub participantid: String,
    /// REALLOCATION COUNTERPARTY PARTICIPANTID
    pub counterparty: String,
    /// REALLOCATIONID
    pub reallocationid: String,
    /// REALLOCATION VALUE
    pub value: Option<rust_decimal::Decimal>,
    /// DATETIME WHEN RECORD SAVED
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingReallocDetail5 {
    type PrimaryKey = BillingReallocDetail5PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("REALLOC_DETAIL".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingReallocDetail5PrimaryKey {
        BillingReallocDetail5PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            counterparty: self.counterparty.clone(),
            participantid: self.participantid.clone(),
            reallocationid: self.reallocationid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_realloc_detail_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingReallocDetail5PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub counterparty: String,
    pub participantid: String,
    pub reallocationid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingReallocDetail5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingReallocDetail5 {
    type Row = BillingReallocDetail5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.counterparty == row.counterparty
            && self.participantid == row.participantid
            && self.reallocationid == row.reallocationid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingReallocDetail5 {
    type PrimaryKey = BillingReallocDetail5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.counterparty == key.counterparty
            && self.participantid == key.participantid
            && self.reallocationid == key.reallocationid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingReallocDetail5PrimaryKey {
    type Row = BillingReallocDetail5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.counterparty == row.counterparty
            && self.participantid == row.participantid
            && self.reallocationid == row.reallocationid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingReallocDetail5PrimaryKey {
    type PrimaryKey = BillingReallocDetail5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.counterparty == key.counterparty
            && self.participantid == key.participantid
            && self.reallocationid == key.reallocationid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingReallocDetail5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("counterparty",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("reallocationid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("value",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut counterparty_array = Vec::new();
        let mut reallocationid_array = Vec::new();
        let mut value_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            counterparty_array.push(row.counterparty);
            reallocationid_array.push(row.reallocationid);
            value_array
                .push({
                    row.value
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(counterparty_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(reallocationid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(value_array)
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
/// ## BILLINGREGIONEXPORTS
///  _BILLINGREGIONEXPORTS sets out the region summary table of overall energy exported to and from each region for each billing run._
///
/// * Data Set Name: Billing
/// * File Name: Regionexports
/// * Data Version: 5
///
/// # Description
///  BILLINGREGIONEXPORTS  data is public, and is available to all participants. Source BILLINGREGIONEXPORTS is populated by the posting of a billing run. Volume Eight records inserted per billing run, or 88 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * EXPORTTO
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRegionexports5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique region identifier
    pub regionid: String,
    /// Region exported to
    pub exportto: String,
    /// MWh Energy value exported
    pub energy: Option<rust_decimal::Decimal>,
    /// $ Value of energy exported
    pub value: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    pub surplusenergy: Option<rust_decimal::Decimal>,
    /// $ Interregional residue
    pub surplusvalue: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingRegionexports5 {
    type PrimaryKey = BillingRegionexports5PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("REGIONEXPORTS".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingRegionexports5PrimaryKey {
        BillingRegionexports5PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            exportto: self.exportto.clone(),
            regionid: self.regionid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_regionexports_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingRegionexports5PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub exportto: String,
    pub regionid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingRegionexports5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingRegionexports5 {
    type Row = BillingRegionexports5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.exportto == row.exportto && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingRegionexports5 {
    type PrimaryKey = BillingRegionexports5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.exportto == key.exportto && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingRegionexports5PrimaryKey {
    type Row = BillingRegionexports5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.exportto == row.exportto && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingRegionexports5PrimaryKey {
    type PrimaryKey = BillingRegionexports5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.exportto == key.exportto && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingRegionexports5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("exportto",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("energy",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("value",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("surplusenergy",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("surplusvalue",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut exportto_array = Vec::new();
        let mut energy_array = Vec::new();
        let mut value_array = Vec::new();
        let mut surplusenergy_array = Vec::new();
        let mut surplusvalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            regionid_array.push(row.regionid);
            exportto_array.push(row.exportto);
            energy_array
                .push({
                    row.energy
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            value_array
                .push({
                    row.value
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            surplusenergy_array
                .push({
                    row.surplusenergy
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            surplusvalue_array
                .push({
                    row.surplusvalue
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(exportto_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(energy_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(value_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(surplusenergy_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(surplusvalue_array)
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
/// ## BILLINGREGIONFIGURES
///  _BILLINGREGIONFIGURES sets out additional summary region details including ancillary service amounts for each billing run._
///
/// * Data Set Name: Billing
/// * File Name: Regionfigures
/// * Data Version: 6
///
/// # Description
///  BILLINGREGIONFIGURES is public data, and is available to all participants. Source BILLINGREGIONFIGURES is populated by the posting of a billing run. Volume Five records inserted per billing run, or 55 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRegionfigures6 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique region identifier
    pub regionid: String,
    /// MWh Energy output in the region during the billing period
    pub energyout: Option<rust_decimal::Decimal>,
    /// $ Value of energy output in region during billing period
    pub valueout: Option<rust_decimal::Decimal>,
    /// MWh Amount of energy purchased in region during billing period
    pub energypurchased: Option<rust_decimal::Decimal>,
    /// $ Value of energy purchased during billing period
    pub valuepurchased: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    pub excessgen: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    pub reservetrading: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    pub intcompo: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    pub adminpricecompo: Option<rust_decimal::Decimal>,
    /// Intraregional residues in $
    pub settsurplus: Option<rust_decimal::Decimal>,
    /// Ancillary service payments in $
    pub aspayment: Option<rust_decimal::Decimal>,
    /// This field is populated with 0
    pub poolfees: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// WDR Settlement Quantity Capped in MWh
    pub wdrsq: Option<rust_decimal::Decimal>,
    /// WDR transaction amount in $
    pub wdrta: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingRegionfigures6 {
    type PrimaryKey = BillingRegionfigures6PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("REGIONFIGURES".into()),
            version: 6,
        }
    }
    fn primary_key(&self) -> BillingRegionfigures6PrimaryKey {
        BillingRegionfigures6PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            regionid: self.regionid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_regionfigures_v6".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingRegionfigures6PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub regionid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingRegionfigures6PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingRegionfigures6 {
    type Row = BillingRegionfigures6;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.regionid == row.regionid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingRegionfigures6 {
    type PrimaryKey = BillingRegionfigures6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.regionid == key.regionid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingRegionfigures6PrimaryKey {
    type Row = BillingRegionfigures6;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.regionid == row.regionid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingRegionfigures6PrimaryKey {
    type PrimaryKey = BillingRegionfigures6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.regionid == key.regionid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingRegionfigures6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("energyout",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("valueout",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("energypurchased",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("valuepurchased",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("excessgen",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("reservetrading",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("intcompo",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("adminpricecompo",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("settsurplus",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("aspayment",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("poolfees",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("wdrsq",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("wdrta",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut energyout_array = Vec::new();
        let mut valueout_array = Vec::new();
        let mut energypurchased_array = Vec::new();
        let mut valuepurchased_array = Vec::new();
        let mut excessgen_array = Vec::new();
        let mut reservetrading_array = Vec::new();
        let mut intcompo_array = Vec::new();
        let mut adminpricecompo_array = Vec::new();
        let mut settsurplus_array = Vec::new();
        let mut aspayment_array = Vec::new();
        let mut poolfees_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut wdrsq_array = Vec::new();
        let mut wdrta_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            regionid_array.push(row.regionid);
            energyout_array
                .push({
                    row.energyout
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            valueout_array
                .push({
                    row.valueout
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            energypurchased_array
                .push({
                    row.energypurchased
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            valuepurchased_array
                .push({
                    row.valuepurchased
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            excessgen_array
                .push({
                    row.excessgen
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            reservetrading_array
                .push({
                    row.reservetrading
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            intcompo_array
                .push({
                    row.intcompo
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            adminpricecompo_array
                .push({
                    row.adminpricecompo
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            settsurplus_array
                .push({
                    row.settsurplus
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            aspayment_array
                .push({
                    row.aspayment
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            poolfees_array
                .push({
                    row.poolfees
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            wdrsq_array
                .push({
                    row.wdrsq
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            wdrta_array
                .push({
                    row.wdrta
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(energyout_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(valueout_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(energypurchased_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(valuepurchased_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(excessgen_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(reservetrading_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(intcompo_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(adminpricecompo_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(settsurplus_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(aspayment_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(poolfees_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(wdrsq_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(wdrta_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLINGREGIONIMPORTS
///  _BILLINGREGIONIMPORTS sets out the region summary table of overall energy imported to and from each region for each billing run._
///
/// * Data Set Name: Billing
/// * File Name: Regionimports
/// * Data Version: 5
///
/// # Description
///  BILLINGREGIONIMPORTS is public data, and is available to all participants. Source BILLINGREGIONIMPORTS is populated by the posting of a billing run. Volume Eight records inserted per billing run, or 88 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * IMPORTFROM
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRegionimports5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique region identifier
    pub regionid: String,
    /// Region energy imported from
    pub importfrom: String,
    /// Amount of energy imported
    pub energy: Option<rust_decimal::Decimal>,
    /// Value of energy imported
    pub value: Option<rust_decimal::Decimal>,
    /// Populated with 0
    pub surplusenergy: Option<rust_decimal::Decimal>,
    /// Interregional residue
    pub surplusvalue: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingRegionimports5 {
    type PrimaryKey = BillingRegionimports5PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("REGIONIMPORTS".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingRegionimports5PrimaryKey {
        BillingRegionimports5PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            importfrom: self.importfrom.clone(),
            regionid: self.regionid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_regionimports_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingRegionimports5PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub importfrom: String,
    pub regionid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingRegionimports5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingRegionimports5 {
    type Row = BillingRegionimports5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.importfrom == row.importfrom && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingRegionimports5 {
    type PrimaryKey = BillingRegionimports5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.importfrom == key.importfrom && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingRegionimports5PrimaryKey {
    type Row = BillingRegionimports5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.importfrom == row.importfrom && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingRegionimports5PrimaryKey {
    type PrimaryKey = BillingRegionimports5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.importfrom == key.importfrom && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingRegionimports5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("importfrom",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("energy",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("value",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("surplusenergy",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("surplusvalue",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut importfrom_array = Vec::new();
        let mut energy_array = Vec::new();
        let mut value_array = Vec::new();
        let mut surplusenergy_array = Vec::new();
        let mut surplusvalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            regionid_array.push(row.regionid);
            importfrom_array.push(row.importfrom);
            energy_array
                .push({
                    row.energy
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            value_array
                .push({
                    row.value
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            surplusenergy_array
                .push({
                    row.surplusenergy
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            surplusvalue_array
                .push({
                    row.surplusvalue
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(importfrom_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(energy_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(value_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(surplusenergy_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(surplusvalue_array)
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
/// ## BILLINGRUNTRK
///  _BILLINGRUNTRK identifies the Statement type (i.e. Status of PRELIM, FINAL, REVISE) and date of the BillRunNo posted, per WeekNo. This provides a further extension of tracking data from the BILLINGDAYTRK table._
///
/// * Data Set Name: Billing
/// * File Name: Runtrk
/// * Data Version: 5
///
/// # Description
///  BILLINGRUNTRK is public data, and is available to all participants. Source BILLINGRUNTRK is populated by the posting of a billing run. Volume An indicative maximum is one record inserted per billing run, or 11 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingRuntrk5 {
    /// Year of the run
    pub contractyear: rust_decimal::Decimal,
    /// Week number of the run
    pub weekno: rust_decimal::Decimal,
    /// Sequential run number
    pub billrunno: rust_decimal::Decimal,
    /// The billing run type, PRELIM, FINAL, REVISE or INTERIM
    pub status: Option<String>,
    /// Flag
    pub adj_cleared: Option<String>,
    /// null, since not used
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// null, since not used
    pub authorisedby: Option<String>,
    /// When the results were posted
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub postdate: Option<chrono::NaiveDateTime>,
    /// Who posted the results
    pub postby: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// null, since not used
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub receiptpostdate: Option<chrono::NaiveDateTime>,
    /// null, since not used
    pub receiptpostby: Option<String>,
    /// When the payment was posted
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub paymentpostdate: Option<chrono::NaiveDateTime>,
    /// Who posted the payment
    pub paymentpostby: Option<String>,
    /// Payment shortfall amount
    pub shortfall: Option<rust_decimal::Decimal>,
    /// Not Used
    pub makeup: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingRuntrk5 {
    type PrimaryKey = BillingRuntrk5PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("RUNTRK".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingRuntrk5PrimaryKey {
        BillingRuntrk5PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_runtrk_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingRuntrk5PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingRuntrk5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingRuntrk5 {
    type Row = BillingRuntrk5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingRuntrk5 {
    type PrimaryKey = BillingRuntrk5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingRuntrk5PrimaryKey {
    type Row = BillingRuntrk5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingRuntrk5PrimaryKey {
    type PrimaryKey = BillingRuntrk5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingRuntrk5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("status",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("adj_cleared",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("authoriseddate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("authorisedby",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("postdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("postby",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("receiptpostdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("receiptpostby",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("paymentpostdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("paymentpostby",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("shortfall",
                arrow2::datatypes::DataType::Decimal(16, 6), true),
                arrow2::datatypes::Field::new("makeup",
                arrow2::datatypes::DataType::Decimal(15, 5), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut status_array = Vec::new();
        let mut adj_cleared_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut postdate_array = Vec::new();
        let mut postby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut receiptpostdate_array = Vec::new();
        let mut receiptpostby_array = Vec::new();
        let mut paymentpostdate_array = Vec::new();
        let mut paymentpostby_array = Vec::new();
        let mut shortfall_array = Vec::new();
        let mut makeup_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            status_array.push(row.status);
            adj_cleared_array.push(row.adj_cleared);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            authorisedby_array.push(row.authorisedby);
            postdate_array.push(row.postdate.map(|val| val.timestamp()));
            postby_array.push(row.postby);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            receiptpostdate_array.push(row.receiptpostdate.map(|val| val.timestamp()));
            receiptpostby_array.push(row.receiptpostby);
            paymentpostdate_array.push(row.paymentpostdate.map(|val| val.timestamp()));
            paymentpostby_array.push(row.paymentpostby);
            shortfall_array
                .push({
                    row.shortfall
                        .map(|mut val| {
                            val.rescale(6);
                            val.mantissa()
                        })
                });
            makeup_array
                .push({
                    row.makeup
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(status_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >, std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(adj_cleared_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(authoriseddate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(authorisedby_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(postdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(postby_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(receiptpostdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(receiptpostby_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(paymentpostdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(paymentpostby_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(shortfall_array)
                    .to(arrow2::datatypes::DataType::Decimal(16, 6))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(makeup_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLING_APC_COMPENSATION
///  _Billing result table for APC compensation payments._
///
/// * Data Set Name: Billing
/// * File Name: Apc Compensation
/// * Data Version: 2
///
/// # Description
///  Updated with each billing run
///
///
///
/// # Primary Key Columns
///
/// * APEVENTID
/// * BILLRUNNO
/// * CLAIMID
/// * CONTRACTYEAR
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingApcCompensation2 {
    /// Billing contract year
    pub contractyear: i64,
    /// Billing week number
    pub weekno: i64,
    /// Billing run number
    pub billrunno: i64,
    /// AP Event Id
    pub apeventid: i64,
    /// AP Event Claim Id
    pub claimid: i64,
    /// Participant identifier
    pub participantid: Option<String>,
    /// Payment amount to the participant
    pub compensation_amount: Option<rust_decimal::Decimal>,
    /// The Administered Price Event Type. Valid values: ENERGY, FCAS, BOTH
    pub event_type: Option<String>,
    /// The Type of Administered Price Compensation Claim. Valid values: DIRECT_COST, OTHER_COST
    pub compensation_type: Option<String>,
    /// The date and time of last changed record
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingApcCompensation2 {
    type PrimaryKey = BillingApcCompensation2PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("APC_COMPENSATION".into()),
            version: 2,
        }
    }
    fn primary_key(&self) -> BillingApcCompensation2PrimaryKey {
        BillingApcCompensation2PrimaryKey {
            apeventid: self.apeventid,
            billrunno: self.billrunno,
            claimid: self.claimid,
            contractyear: self.contractyear,
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_apc_compensation_v2".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingApcCompensation2PrimaryKey {
    pub apeventid: i64,
    pub billrunno: i64,
    pub claimid: i64,
    pub contractyear: i64,
    pub weekno: i64,
}
impl mmsdm_core::PrimaryKey for BillingApcCompensation2PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingApcCompensation2 {
    type Row = BillingApcCompensation2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.apeventid == row.apeventid && self.billrunno == row.billrunno
            && self.claimid == row.claimid && self.contractyear == row.contractyear
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingApcCompensation2 {
    type PrimaryKey = BillingApcCompensation2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid && self.billrunno == key.billrunno
            && self.claimid == key.claimid && self.contractyear == key.contractyear
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingApcCompensation2PrimaryKey {
    type Row = BillingApcCompensation2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.apeventid == row.apeventid && self.billrunno == row.billrunno
            && self.claimid == row.claimid && self.contractyear == row.contractyear
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingApcCompensation2PrimaryKey {
    type PrimaryKey = BillingApcCompensation2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid && self.billrunno == key.billrunno
            && self.claimid == key.claimid && self.contractyear == key.contractyear
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingApcCompensation2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("apeventid",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("claimid",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("compensation_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("event_type",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("compensation_type",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut apeventid_array = Vec::new();
        let mut claimid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut compensation_amount_array = Vec::new();
        let mut event_type_array = Vec::new();
        let mut compensation_type_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array.push(row.contractyear);
            weekno_array.push(row.weekno);
            billrunno_array.push(row.billrunno);
            apeventid_array.push(row.apeventid);
            claimid_array.push(row.claimid);
            participantid_array.push(row.participantid);
            compensation_amount_array
                .push({
                    row.compensation_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            event_type_array.push(row.event_type);
            compensation_type_array.push(row.compensation_type);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(apeventid_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(claimid_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(compensation_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(event_type_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(compensation_type_array)) as std::sync::Arc < dyn
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
/// ## BILLING_APC_RECOVERY
///  _Billing result table for recovery of APC compensation payments_
///
/// * Data Set Name: Billing
/// * File Name: Apc Recovery
/// * Data Version: 2
///
/// # Description
///  Updated with each billing run
///
///
///
/// # Primary Key Columns
///
/// * APEVENTID
/// * BILLRUNNO
/// * CLAIMID
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingApcRecovery2 {
    /// Billing contract year
    pub contractyear: i64,
    /// Billing week number
    pub weekno: i64,
    /// Billing run number
    pub billrunno: i64,
    /// AP Event Id
    pub apeventid: i64,
    /// AP Event Claim Id
    pub claimid: i64,
    /// Participant identifier
    pub participantid: String,
    /// Region Identifier
    pub regionid: String,
    /// Recovery amount attributable to the participant in that region
    pub recovery_amount: Option<rust_decimal::Decimal>,
    /// The starting half hourly interval for the eligibility period for recovery of APC Payment
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub eligibility_start_interval: Option<chrono::NaiveDateTime>,
    /// The ending half hourly interval for the eligibility period for recovery of APC Payment
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub eligibility_end_interval: Option<chrono::NaiveDateTime>,
    /// The participant demand in the cost recovery region
    pub participant_demand: Option<rust_decimal::Decimal>,
    /// The sum of demand of all participants in the cost recovery region (Region Sum)
    pub region_demand: Option<rust_decimal::Decimal>,
    /// The date and time of last changed record
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingApcRecovery2 {
    type PrimaryKey = BillingApcRecovery2PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("APC_RECOVERY".into()),
            version: 2,
        }
    }
    fn primary_key(&self) -> BillingApcRecovery2PrimaryKey {
        BillingApcRecovery2PrimaryKey {
            apeventid: self.apeventid,
            billrunno: self.billrunno,
            claimid: self.claimid,
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            regionid: self.regionid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_apc_recovery_v2".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingApcRecovery2PrimaryKey {
    pub apeventid: i64,
    pub billrunno: i64,
    pub claimid: i64,
    pub contractyear: i64,
    pub participantid: String,
    pub regionid: String,
    pub weekno: i64,
}
impl mmsdm_core::PrimaryKey for BillingApcRecovery2PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingApcRecovery2 {
    type Row = BillingApcRecovery2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.apeventid == row.apeventid && self.billrunno == row.billrunno
            && self.claimid == row.claimid && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingApcRecovery2 {
    type PrimaryKey = BillingApcRecovery2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid && self.billrunno == key.billrunno
            && self.claimid == key.claimid && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingApcRecovery2PrimaryKey {
    type Row = BillingApcRecovery2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.apeventid == row.apeventid && self.billrunno == row.billrunno
            && self.claimid == row.claimid && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingApcRecovery2PrimaryKey {
    type PrimaryKey = BillingApcRecovery2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid && self.billrunno == key.billrunno
            && self.claimid == key.claimid && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingApcRecovery2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("apeventid",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("claimid",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("recovery_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("eligibility_start_interval",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("eligibility_end_interval",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("participant_demand",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("region_demand",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut apeventid_array = Vec::new();
        let mut claimid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut recovery_amount_array = Vec::new();
        let mut eligibility_start_interval_array = Vec::new();
        let mut eligibility_end_interval_array = Vec::new();
        let mut participant_demand_array = Vec::new();
        let mut region_demand_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array.push(row.contractyear);
            weekno_array.push(row.weekno);
            billrunno_array.push(row.billrunno);
            apeventid_array.push(row.apeventid);
            claimid_array.push(row.claimid);
            participantid_array.push(row.participantid);
            regionid_array.push(row.regionid);
            recovery_amount_array
                .push({
                    row.recovery_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            eligibility_start_interval_array
                .push(row.eligibility_start_interval.map(|val| val.timestamp()));
            eligibility_end_interval_array
                .push(row.eligibility_end_interval.map(|val| val.timestamp()));
            participant_demand_array
                .push({
                    row.participant_demand
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            region_demand_array
                .push({
                    row.region_demand
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(apeventid_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(claimid_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(recovery_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(eligibility_start_interval_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(eligibility_end_interval_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(participant_demand_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(region_demand_array)
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
/// ## BILLING_CO2E_PUBLICATION
///  _Carbon Dioxide Intensity Index publication table_
///
/// * Data Set Name: Billing
/// * File Name: Billing Co2e Publication
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * REGIONID
/// * SETTLEMENTDATE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingBillingCo2ePublication1 {
    /// Billing contract year
    pub contractyear: i64,
    /// Billing week no
    pub weekno: i64,
    /// Billing run no
    pub billrunno: i64,
    /// Settlement Date (Calendar)
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Region identifier
    pub regionid: String,
    /// Total sent out energy for region (MWh)
    pub sentoutenergy: Option<rust_decimal::Decimal>,
    /// Total generator emissions for region (Co2-e)
    pub generatoremissions: Option<rust_decimal::Decimal>,
    /// Carbon Dioxide Intensity index for region (CO2-e/MWh)
    pub intensityindex: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingBillingCo2ePublication1 {
    type PrimaryKey = BillingBillingCo2ePublication1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("BILLING_CO2E_PUBLICATION".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingBillingCo2ePublication1PrimaryKey {
        BillingBillingCo2ePublication1PrimaryKey {
            contractyear: self.contractyear,
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
            weekno: self.weekno,
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
            "billing_billing_co2e_publication_v1_{}_{}", self.partition_suffix().year,
            self.partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingBillingCo2ePublication1PrimaryKey {
    pub contractyear: i64,
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub weekno: i64,
}
impl mmsdm_core::PrimaryKey for BillingBillingCo2ePublication1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingBillingCo2ePublication1 {
    type Row = BillingBillingCo2ePublication1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingBillingCo2ePublication1 {
    type PrimaryKey = BillingBillingCo2ePublication1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingBillingCo2ePublication1PrimaryKey {
    type Row = BillingBillingCo2ePublication1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingBillingCo2ePublication1PrimaryKey {
    type PrimaryKey = BillingBillingCo2ePublication1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingBillingCo2ePublication1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("sentoutenergy",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("generatoremissions",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("intensityindex",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut settlementdate_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut sentoutenergy_array = Vec::new();
        let mut generatoremissions_array = Vec::new();
        let mut intensityindex_array = Vec::new();
        for row in partition {
            contractyear_array.push(row.contractyear);
            weekno_array.push(row.weekno);
            billrunno_array.push(row.billrunno);
            settlementdate_array.push(row.settlementdate.timestamp());
            regionid_array.push(row.regionid);
            sentoutenergy_array
                .push({
                    row.sentoutenergy
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            generatoremissions_array
                .push({
                    row.generatoremissions
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            intensityindex_array
                .push({
                    row.intensityindex
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(sentoutenergy_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(generatoremissions_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(intensityindex_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLING_CO2E_PUBLICATION_TRK
///  _Carbon Dioxide Intensity Index publication tracking table_
///
/// * Data Set Name: Billing
/// * File Name: Billing Co2e Publication Trk
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * CONTRACTYEAR
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingBillingCo2ePublicationTrk1 {
    /// Billing contract year
    pub contractyear: i64,
    /// Billing week no
    pub weekno: i64,
    /// Billing run no
    pub billrunno: Option<i64>,
    /// Last changed date time
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingBillingCo2ePublicationTrk1 {
    type PrimaryKey = BillingBillingCo2ePublicationTrk1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("BILLING_CO2E_PUBLICATION_TRK".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingBillingCo2ePublicationTrk1PrimaryKey {
        BillingBillingCo2ePublicationTrk1PrimaryKey {
            contractyear: self.contractyear,
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_billing_co2e_publication_trk_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingBillingCo2ePublicationTrk1PrimaryKey {
    pub contractyear: i64,
    pub weekno: i64,
}
impl mmsdm_core::PrimaryKey for BillingBillingCo2ePublicationTrk1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingBillingCo2ePublicationTrk1 {
    type Row = BillingBillingCo2ePublicationTrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingBillingCo2ePublicationTrk1 {
    type PrimaryKey = BillingBillingCo2ePublicationTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingBillingCo2ePublicationTrk1PrimaryKey {
    type Row = BillingBillingCo2ePublicationTrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractyear == row.contractyear && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingBillingCo2ePublicationTrk1PrimaryKey {
    type PrimaryKey = BillingBillingCo2ePublicationTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractyear == key.contractyear && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingBillingCo2ePublicationTrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Int64, true),
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array.push(row.contractyear);
            weekno_array.push(row.weekno);
            billrunno_array.push(row.billrunno);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(billrunno_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
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
/// ## BILLING_DAILY_ENERGY_SUMMARY
///  _Billing result table containing daily summary data for customer and generator energy amounts_
///
/// * Data Set Name: Billing
/// * File Name: Daily Energy Summary
/// * Data Version: 1
///
/// # Description
///  BILLING_DAILY_ENERGY_SUMMARY data is confidential  to the relevant participant. Source Populated by the posting of a billing run. Volume Approximately 20 records per billrunno.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * REGIONID
/// * SETTLEMENTDATE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingDailyEnergySummary1 {
    /// Billing Contract Year
    pub contractyear: rust_decimal::Decimal,
    /// Billing Week number
    pub weekno: rust_decimal::Decimal,
    /// Billing Run number
    pub billrunno: rust_decimal::Decimal,
    /// settlement date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// participant identifier
    pub participantid: String,
    /// Unique Region Identifier
    pub regionid: String,
    /// customer energy amount purchased on this settlement day by the participant in the region
    pub customer_energy_purchased: Option<rust_decimal::Decimal>,
    /// generator energy amount sold on this settlement day by the participant in the region
    pub generator_energy_sold: Option<rust_decimal::Decimal>,
    /// generator energy amount purchased on this settlement day by the participant in the region
    pub generator_energy_purchased: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingDailyEnergySummary1 {
    type PrimaryKey = BillingDailyEnergySummary1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("DAILY_ENERGY_SUMMARY".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingDailyEnergySummary1PrimaryKey {
        BillingDailyEnergySummary1PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
            weekno: self.weekno,
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
            "billing_daily_energy_summary_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingDailyEnergySummary1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingDailyEnergySummary1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingDailyEnergySummary1 {
    type Row = BillingDailyEnergySummary1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingDailyEnergySummary1 {
    type PrimaryKey = BillingDailyEnergySummary1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingDailyEnergySummary1PrimaryKey {
    type Row = BillingDailyEnergySummary1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingDailyEnergySummary1PrimaryKey {
    type PrimaryKey = BillingDailyEnergySummary1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingDailyEnergySummary1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("customer_energy_purchased",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("generator_energy_sold",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("generator_energy_purchased",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut settlementdate_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut customer_energy_purchased_array = Vec::new();
        let mut generator_energy_sold_array = Vec::new();
        let mut generator_energy_purchased_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            settlementdate_array.push(row.settlementdate.timestamp());
            participantid_array.push(row.participantid);
            regionid_array.push(row.regionid);
            customer_energy_purchased_array
                .push({
                    row.customer_energy_purchased
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            generator_energy_sold_array
                .push({
                    row.generator_energy_sold
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            generator_energy_purchased_array
                .push({
                    row.generator_energy_purchased
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(customer_energy_purchased_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(generator_energy_sold_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(generator_energy_purchased_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLING_DIRECTION_RECON_OTHER
///  _Billing reconciliation result table for both provisional and final directions_
///
/// * Data Set Name: Billing
/// * File Name: Billing Direction Recon Other
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * DIRECTION_ID
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingBillingDirectionReconOther1 {
    /// Billing contract year
    pub contractyear: i64,
    /// Billing week no
    pub weekno: i64,
    /// Billing run no
    pub billrunno: i64,
    /// Direction identifier
    pub direction_id: String,
    /// Region Identifier
    pub regionid: String,
    /// Direction description
    pub direction_desc: Option<String>,
    /// The service for which the direction occurred (ENERGY, ANCILLARY, NON_ENERGY_NON_AS, etc)
    pub direction_type_id: Option<String>,
    /// Settlement day on which the direction starts
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub direction_start_date: Option<chrono::NaiveDateTime>,
    /// Settlement day on which the direction ends.  The same value for all regions
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub direction_end_date: Option<chrono::NaiveDateTime>,
    /// Dispatch interval in which the direction starts.  The same value for all regions
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub direction_start_interval: Option<chrono::NaiveDateTime>,
    /// Dispatch interval in which the direction ends.  The same value for all regions
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub direction_end_interval: Option<chrono::NaiveDateTime>,
    /// The final compensation amount for the direction.  The same value for all regions
    pub compensation_amount: Option<rust_decimal::Decimal>,
    /// The interest amount calculated on the final compensation amount for the direction.  The same value for all regions
    pub interest_amount: Option<rust_decimal::Decimal>,
    /// The independent expert fee amount for the direction.  The same value for all regions
    pub independent_expert_fee: Option<rust_decimal::Decimal>,
    /// The total recovery amount for the direction.  The same value for all regions
    pub cra: Option<rust_decimal::Decimal>,
    /// The total customer energy for this region, over the duration of the direction
    pub regional_customer_energy: Option<rust_decimal::Decimal>,
    /// The total generator energy for this region, over the duration of the direction
    pub regional_generator_energy: Option<rust_decimal::Decimal>,
    /// The regional benefit factor allocated to this region for the direction
    pub regional_benefit_factor: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingBillingDirectionReconOther1 {
    type PrimaryKey = BillingBillingDirectionReconOther1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("BILLING_DIRECTION_RECON_OTHER".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingBillingDirectionReconOther1PrimaryKey {
        BillingBillingDirectionReconOther1PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            direction_id: self.direction_id.clone(),
            regionid: self.regionid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_billing_direction_recon_other_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingBillingDirectionReconOther1PrimaryKey {
    pub billrunno: i64,
    pub contractyear: i64,
    pub direction_id: String,
    pub regionid: String,
    pub weekno: i64,
}
impl mmsdm_core::PrimaryKey for BillingBillingDirectionReconOther1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingBillingDirectionReconOther1 {
    type Row = BillingBillingDirectionReconOther1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.direction_id == row.direction_id && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingBillingDirectionReconOther1 {
    type PrimaryKey = BillingBillingDirectionReconOther1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.direction_id == key.direction_id && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingBillingDirectionReconOther1PrimaryKey {
    type Row = BillingBillingDirectionReconOther1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.direction_id == row.direction_id && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingBillingDirectionReconOther1PrimaryKey {
    type PrimaryKey = BillingBillingDirectionReconOther1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.direction_id == key.direction_id && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingBillingDirectionReconOther1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Int64, false),
                arrow2::datatypes::Field::new("direction_id",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("direction_desc",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("direction_type_id",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("direction_start_date",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("direction_end_date",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("direction_start_interval",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("direction_end_interval",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("compensation_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("interest_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("independent_expert_fee",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("cra",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("regional_customer_energy",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("regional_generator_energy",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("regional_benefit_factor",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut direction_id_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut direction_desc_array = Vec::new();
        let mut direction_type_id_array = Vec::new();
        let mut direction_start_date_array = Vec::new();
        let mut direction_end_date_array = Vec::new();
        let mut direction_start_interval_array = Vec::new();
        let mut direction_end_interval_array = Vec::new();
        let mut compensation_amount_array = Vec::new();
        let mut interest_amount_array = Vec::new();
        let mut independent_expert_fee_array = Vec::new();
        let mut cra_array = Vec::new();
        let mut regional_customer_energy_array = Vec::new();
        let mut regional_generator_energy_array = Vec::new();
        let mut regional_benefit_factor_array = Vec::new();
        for row in partition {
            contractyear_array.push(row.contractyear);
            weekno_array.push(row.weekno);
            billrunno_array.push(row.billrunno);
            direction_id_array.push(row.direction_id);
            regionid_array.push(row.regionid);
            direction_desc_array.push(row.direction_desc);
            direction_type_id_array.push(row.direction_type_id);
            direction_start_date_array
                .push(row.direction_start_date.map(|val| val.timestamp()));
            direction_end_date_array
                .push(row.direction_end_date.map(|val| val.timestamp()));
            direction_start_interval_array
                .push(row.direction_start_interval.map(|val| val.timestamp()));
            direction_end_interval_array
                .push(row.direction_end_interval.map(|val| val.timestamp()));
            compensation_amount_array
                .push({
                    row.compensation_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            interest_amount_array
                .push({
                    row.interest_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            independent_expert_fee_array
                .push({
                    row.independent_expert_fee
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            cra_array
                .push({
                    row.cra
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            regional_customer_energy_array
                .push({
                    row.regional_customer_energy
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            regional_generator_energy_array
                .push({
                    row.regional_generator_energy
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            regional_benefit_factor_array
                .push({
                    row.regional_benefit_factor
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array))
                    as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(direction_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(direction_desc_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(direction_type_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(direction_start_date_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(direction_end_date_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(direction_start_interval_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(direction_end_interval_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(compensation_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(interest_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(independent_expert_fee_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(cra_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(regional_customer_energy_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(regional_generator_energy_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(regional_benefit_factor_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLING_DIR_FINAL_AMOUNT
///  _The Billing Final Directions Payment Amount for Directed/Affected/Eligible participants_
///
/// * Data Set Name: Billing
/// * File Name: Dir Final Amount
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * COMPENSATION_TYPE
/// * CONTRACTYEAR
/// * DIRECTION_ID
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingDirFinalAmount1 {
    /// The Billing Contract Year
    pub contractyear: rust_decimal::Decimal,
    /// The Billing WeekNo
    pub weekno: rust_decimal::Decimal,
    /// The Billing RunNo
    pub billrunno: rust_decimal::Decimal,
    /// The Direction Unique Identifier
    pub direction_id: String,
    /// The Direction Payment Participant ID
    pub participantid: String,
    /// The Direction Payment Type, Directed_Comp, Affected_Comp, Eligible_Comp.
    pub compensation_type: String,
    /// The Direction Provisional Payment Amount
    pub provisional_amount: Option<rust_decimal::Decimal>,
    /// The Direction Final Payment Amount
    pub final_amount: Option<rust_decimal::Decimal>,
    /// The Last datetime record is updated
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingDirFinalAmount1 {
    type PrimaryKey = BillingDirFinalAmount1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("DIR_FINAL_AMOUNT".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingDirFinalAmount1PrimaryKey {
        BillingDirFinalAmount1PrimaryKey {
            billrunno: self.billrunno,
            compensation_type: self.compensation_type.clone(),
            contractyear: self.contractyear,
            direction_id: self.direction_id.clone(),
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_dir_final_amount_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingDirFinalAmount1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub compensation_type: String,
    pub contractyear: rust_decimal::Decimal,
    pub direction_id: String,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingDirFinalAmount1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingDirFinalAmount1 {
    type Row = BillingDirFinalAmount1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno
            && self.compensation_type == row.compensation_type
            && self.contractyear == row.contractyear
            && self.direction_id == row.direction_id
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingDirFinalAmount1 {
    type PrimaryKey = BillingDirFinalAmount1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno
            && self.compensation_type == key.compensation_type
            && self.contractyear == key.contractyear
            && self.direction_id == key.direction_id
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingDirFinalAmount1PrimaryKey {
    type Row = BillingDirFinalAmount1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno
            && self.compensation_type == row.compensation_type
            && self.contractyear == row.contractyear
            && self.direction_id == row.direction_id
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingDirFinalAmount1PrimaryKey {
    type PrimaryKey = BillingDirFinalAmount1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno
            && self.compensation_type == key.compensation_type
            && self.contractyear == key.contractyear
            && self.direction_id == key.direction_id
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingDirFinalAmount1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("direction_id",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("compensation_type",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("provisional_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("final_amount",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut direction_id_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut compensation_type_array = Vec::new();
        let mut provisional_amount_array = Vec::new();
        let mut final_amount_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            direction_id_array.push(row.direction_id);
            participantid_array.push(row.participantid);
            compensation_type_array.push(row.compensation_type);
            provisional_amount_array
                .push({
                    row.provisional_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            final_amount_array
                .push({
                    row.final_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(direction_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(compensation_type_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(provisional_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(final_amount_array)
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
/// ## BILLING_DIR_FINAL_RECOVERY
///  _The Billing Final Directions Recovery Amount for the participants_
///
/// * Data Set Name: Billing
/// * File Name: Dir Final Recovery
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * DIRECTION_ID
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingDirFinalRecovery1 {
    /// The Billing Contract Year
    pub contractyear: rust_decimal::Decimal,
    /// The Billing WeekNo
    pub weekno: rust_decimal::Decimal,
    /// The Billing RunNo
    pub billrunno: rust_decimal::Decimal,
    /// The Direction Unique Identifier
    pub direction_id: String,
    /// The Direction Payment Participant ID
    pub participantid: String,
    /// The Direction Compensation Recovery Amount
    pub cra_amount: Option<rust_decimal::Decimal>,
    /// The Provisional Recovery Amount
    pub provisional_amount: Option<rust_decimal::Decimal>,
    /// The Final Recovery Amount
    pub final_amount: Option<rust_decimal::Decimal>,
    /// The Last datetime record is updated
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingDirFinalRecovery1 {
    type PrimaryKey = BillingDirFinalRecovery1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("DIR_FINAL_RECOVERY".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingDirFinalRecovery1PrimaryKey {
        BillingDirFinalRecovery1PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            direction_id: self.direction_id.clone(),
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_dir_final_recovery_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingDirFinalRecovery1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub direction_id: String,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingDirFinalRecovery1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingDirFinalRecovery1 {
    type Row = BillingDirFinalRecovery1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.direction_id == row.direction_id
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingDirFinalRecovery1 {
    type PrimaryKey = BillingDirFinalRecovery1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.direction_id == key.direction_id
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingDirFinalRecovery1PrimaryKey {
    type Row = BillingDirFinalRecovery1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.direction_id == row.direction_id
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingDirFinalRecovery1PrimaryKey {
    type PrimaryKey = BillingDirFinalRecovery1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.direction_id == key.direction_id
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingDirFinalRecovery1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("direction_id",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("cra_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("provisional_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("final_amount",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut direction_id_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut cra_amount_array = Vec::new();
        let mut provisional_amount_array = Vec::new();
        let mut final_amount_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            direction_id_array.push(row.direction_id);
            participantid_array.push(row.participantid);
            cra_amount_array
                .push({
                    row.cra_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            provisional_amount_array
                .push({
                    row.provisional_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            final_amount_array
                .push({
                    row.final_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(direction_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(cra_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(provisional_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(final_amount_array)
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
/// ## BILLING_DIR_PROV_AMOUNT
///  _The Billing Provisional Directions Payment Amount for Directed/Affected/Eligible participants_
///
/// * Data Set Name: Billing
/// * File Name: Dir Prov Amount
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * COMPENSATION_TYPE
/// * CONTRACTYEAR
/// * DIRECTION_ID
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingDirProvAmount1 {
    /// The Billing Contract Year
    pub contractyear: rust_decimal::Decimal,
    /// The Billing WeekNo
    pub weekno: rust_decimal::Decimal,
    /// The Billing RunNo
    pub billrunno: rust_decimal::Decimal,
    /// The Direction Unique Identifier
    pub direction_id: String,
    /// The Direction Payment Participant ID
    pub participantid: String,
    /// The Direction Payment Type, Directed_Comp, Affected_Comp, Eligible_Comp
    pub compensation_type: String,
    /// The Direction Payment Amount
    pub compensation_amount: Option<rust_decimal::Decimal>,
    /// The Last datetime record is updated
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingDirProvAmount1 {
    type PrimaryKey = BillingDirProvAmount1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("DIR_PROV_AMOUNT".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingDirProvAmount1PrimaryKey {
        BillingDirProvAmount1PrimaryKey {
            billrunno: self.billrunno,
            compensation_type: self.compensation_type.clone(),
            contractyear: self.contractyear,
            direction_id: self.direction_id.clone(),
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_dir_prov_amount_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingDirProvAmount1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub compensation_type: String,
    pub contractyear: rust_decimal::Decimal,
    pub direction_id: String,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingDirProvAmount1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingDirProvAmount1 {
    type Row = BillingDirProvAmount1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno
            && self.compensation_type == row.compensation_type
            && self.contractyear == row.contractyear
            && self.direction_id == row.direction_id
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingDirProvAmount1 {
    type PrimaryKey = BillingDirProvAmount1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno
            && self.compensation_type == key.compensation_type
            && self.contractyear == key.contractyear
            && self.direction_id == key.direction_id
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingDirProvAmount1PrimaryKey {
    type Row = BillingDirProvAmount1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno
            && self.compensation_type == row.compensation_type
            && self.contractyear == row.contractyear
            && self.direction_id == row.direction_id
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingDirProvAmount1PrimaryKey {
    type PrimaryKey = BillingDirProvAmount1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno
            && self.compensation_type == key.compensation_type
            && self.contractyear == key.contractyear
            && self.direction_id == key.direction_id
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingDirProvAmount1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("direction_id",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("compensation_type",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("compensation_amount",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut direction_id_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut compensation_type_array = Vec::new();
        let mut compensation_amount_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            direction_id_array.push(row.direction_id);
            participantid_array.push(row.participantid);
            compensation_type_array.push(row.compensation_type);
            compensation_amount_array
                .push({
                    row.compensation_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(direction_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(compensation_type_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(compensation_amount_array)
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
/// ## BILLING_DIR_PROV_RECOVERY
///  _The Billing Provisional Directions Recovery Amount for the participants_
///
/// * Data Set Name: Billing
/// * File Name: Dir Prov Recovery
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * DIRECTION_ID
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingDirProvRecovery1 {
    /// The Billing Contract Year
    pub contractyear: rust_decimal::Decimal,
    /// The Billing WeekNo
    pub weekno: rust_decimal::Decimal,
    /// The Billing RunNo
    pub billrunno: rust_decimal::Decimal,
    /// The Direction Unique Identifier
    pub direction_id: String,
    /// The Direction Payment Participant ID
    pub participantid: String,
    /// The Direction Compensation Recovery Amount
    pub cra_amount: Option<rust_decimal::Decimal>,
    /// The Direction Recovery Amount
    pub recovery_amount: Option<rust_decimal::Decimal>,
    /// The Last datetime record is updated
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingDirProvRecovery1 {
    type PrimaryKey = BillingDirProvRecovery1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("DIR_PROV_RECOVERY".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingDirProvRecovery1PrimaryKey {
        BillingDirProvRecovery1PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            direction_id: self.direction_id.clone(),
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_dir_prov_recovery_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingDirProvRecovery1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub direction_id: String,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingDirProvRecovery1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingDirProvRecovery1 {
    type Row = BillingDirProvRecovery1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.direction_id == row.direction_id
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingDirProvRecovery1 {
    type PrimaryKey = BillingDirProvRecovery1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.direction_id == key.direction_id
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingDirProvRecovery1PrimaryKey {
    type Row = BillingDirProvRecovery1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.direction_id == row.direction_id
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingDirProvRecovery1PrimaryKey {
    type PrimaryKey = BillingDirProvRecovery1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.direction_id == key.direction_id
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingDirProvRecovery1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("direction_id",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("cra_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("recovery_amount",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut direction_id_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut cra_amount_array = Vec::new();
        let mut recovery_amount_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            direction_id_array.push(row.direction_id);
            participantid_array.push(row.participantid);
            cra_amount_array
                .push({
                    row.cra_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            recovery_amount_array
                .push({
                    row.recovery_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(direction_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(cra_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(recovery_amount_array)
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
/// ## BILLING_DIR_RECOVERY_DETAIL
///  _The Billing Directions Recovery Details for the participants_
///
/// * Data Set Name: Billing
/// * File Name: Dir Recovery Detail
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * DIRECTION_ID
/// * PARTICIPANTCATEGORYID
/// * PARTICIPANTID
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingDirRecoveryDetail1 {
    /// The Billing Contract Year
    pub contractyear: rust_decimal::Decimal,
    /// The Billing WeekNo
    pub weekno: rust_decimal::Decimal,
    /// The Billing RunNo
    pub billrunno: rust_decimal::Decimal,
    /// The Direction Unique Identifier
    pub direction_id: String,
    /// The Direction Payment Participant ID
    pub participantid: String,
    /// The Participant Category for recovery Customer/Generator /SmallGen
    pub participantcategoryid: String,
    /// The Region ID for the recovery
    pub regionid: String,
    /// The Direction Recovery Amount
    pub recovery_amount: Option<rust_decimal::Decimal>,
    /// The Energy Value used for the Recovery
    pub recovery_energy: Option<rust_decimal::Decimal>,
    /// The total Energy at the Region ID
    pub region_energy: Option<rust_decimal::Decimal>,
    /// The Energy Value (Scheduled Loads) that is excluded
    pub excluded_energy: Option<rust_decimal::Decimal>,
    /// The Last datetime record is updated
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingDirRecoveryDetail1 {
    type PrimaryKey = BillingDirRecoveryDetail1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("DIR_RECOVERY_DETAIL".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingDirRecoveryDetail1PrimaryKey {
        BillingDirRecoveryDetail1PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            direction_id: self.direction_id.clone(),
            participantcategoryid: self.participantcategoryid.clone(),
            participantid: self.participantid.clone(),
            regionid: self.regionid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_dir_recovery_detail_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingDirRecoveryDetail1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub direction_id: String,
    pub participantcategoryid: String,
    pub participantid: String,
    pub regionid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingDirRecoveryDetail1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingDirRecoveryDetail1 {
    type Row = BillingDirRecoveryDetail1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.direction_id == row.direction_id
            && self.participantcategoryid == row.participantcategoryid
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingDirRecoveryDetail1 {
    type PrimaryKey = BillingDirRecoveryDetail1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.direction_id == key.direction_id
            && self.participantcategoryid == key.participantcategoryid
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingDirRecoveryDetail1PrimaryKey {
    type Row = BillingDirRecoveryDetail1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.direction_id == row.direction_id
            && self.participantcategoryid == row.participantcategoryid
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingDirRecoveryDetail1PrimaryKey {
    type PrimaryKey = BillingDirRecoveryDetail1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.direction_id == key.direction_id
            && self.participantcategoryid == key.participantcategoryid
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingDirRecoveryDetail1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("direction_id",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantcategoryid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("recovery_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("recovery_energy",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("region_energy",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("excluded_energy",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut direction_id_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut participantcategoryid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut recovery_amount_array = Vec::new();
        let mut recovery_energy_array = Vec::new();
        let mut region_energy_array = Vec::new();
        let mut excluded_energy_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            direction_id_array.push(row.direction_id);
            participantid_array.push(row.participantid);
            participantcategoryid_array.push(row.participantcategoryid);
            regionid_array.push(row.regionid);
            recovery_amount_array
                .push({
                    row.recovery_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            recovery_energy_array
                .push({
                    row.recovery_energy
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            region_energy_array
                .push({
                    row.region_energy
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            excluded_energy_array
                .push({
                    row.excluded_energy
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(direction_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantcategoryid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(recovery_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(recovery_energy_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(region_energy_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(excluded_energy_array)
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
/// ## BILLING_EFTSHORTFALL_AMOUNT
///  _The billing shortfall run amounts_
///
/// * Data Set Name: Billing
/// * File Name: Eftshortfall Amount
/// * Data Version: 1
///
/// # Description
///  BILLING_EFTSHORTFALL_AMOUNT data is confidential, and is available only to the relevant participant.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingEftshortfallAmount1 {
    /// The shortfall affected billing contract year
    pub contractyear: rust_decimal::Decimal,
    /// The shortfall affected billing week no
    pub weekno: rust_decimal::Decimal,
    /// The shortfall affected billing week run no
    pub billrunno: rust_decimal::Decimal,
    /// The participant affected by the shortfall calculation
    pub participantid: String,
    /// The Participant shortfall amount
    pub shortfall_amount: Option<rust_decimal::Decimal>,
    /// The market shortfall amount
    pub shortfall: Option<rust_decimal::Decimal>,
    /// The Company ID associated with the Participant ID used in the shortfall calculation
    pub shortfall_company_id: Option<String>,
    /// The shortfall amount for the Company ID associated with the Participant ID used in the shortfall calculation
    pub company_shortfall_amount: Option<rust_decimal::Decimal>,
    /// The participant NET energy used in shortfall calculation
    pub participant_net_energy: Option<rust_decimal::Decimal>,
    /// The NET energy for the Company ID associated with the Participant ID used in the shortfall calculation
    pub company_net_energy: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingEftshortfallAmount1 {
    type PrimaryKey = BillingEftshortfallAmount1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("EFTSHORTFALL_AMOUNT".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingEftshortfallAmount1PrimaryKey {
        BillingEftshortfallAmount1PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_eftshortfall_amount_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingEftshortfallAmount1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingEftshortfallAmount1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingEftshortfallAmount1 {
    type Row = BillingEftshortfallAmount1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingEftshortfallAmount1 {
    type PrimaryKey = BillingEftshortfallAmount1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingEftshortfallAmount1PrimaryKey {
    type Row = BillingEftshortfallAmount1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingEftshortfallAmount1PrimaryKey {
    type PrimaryKey = BillingEftshortfallAmount1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingEftshortfallAmount1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("shortfall_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("shortfall",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("shortfall_company_id",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("company_shortfall_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("participant_net_energy",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("company_net_energy",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut shortfall_amount_array = Vec::new();
        let mut shortfall_array = Vec::new();
        let mut shortfall_company_id_array = Vec::new();
        let mut company_shortfall_amount_array = Vec::new();
        let mut participant_net_energy_array = Vec::new();
        let mut company_net_energy_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            shortfall_amount_array
                .push({
                    row.shortfall_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            shortfall_array
                .push({
                    row.shortfall
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            shortfall_company_id_array.push(row.shortfall_company_id);
            company_shortfall_amount_array
                .push({
                    row.company_shortfall_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            participant_net_energy_array
                .push({
                    row.participant_net_energy
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            company_net_energy_array
                .push({
                    row.company_net_energy
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(shortfall_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(shortfall_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(shortfall_company_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(company_shortfall_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(participant_net_energy_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(company_net_energy_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLING_EFTSHORTFALL_DETAIL
///  _The Billing Shortfall Run Amount details_
///
/// * Data Set Name: Billing
/// * File Name: Eftshortfall Detail
/// * Data Version: 1
///
/// # Description
///  BILLING_EFTSHORTFALL_DETAIL data is confidential, and is available only to the relevant participant.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * TRANSACTION_TYPE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingEftshortfallDetail1 {
    /// The shortfall affected billing contract year
    pub contractyear: rust_decimal::Decimal,
    /// The shortfall affected billing week no
    pub weekno: rust_decimal::Decimal,
    /// The shortfall affected billing week run no
    pub billrunno: rust_decimal::Decimal,
    /// The participant affected by the shortfall calculation
    pub participantid: String,
    /// The transaction type details associated with the shortfall calculation
    pub transaction_type: String,
    /// The amount for each transaction type
    pub amount: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingEftshortfallDetail1 {
    type PrimaryKey = BillingEftshortfallDetail1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("EFTSHORTFALL_DETAIL".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingEftshortfallDetail1PrimaryKey {
        BillingEftshortfallDetail1PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            transaction_type: self.transaction_type.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_eftshortfall_detail_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingEftshortfallDetail1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub transaction_type: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingEftshortfallDetail1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingEftshortfallDetail1 {
    type Row = BillingEftshortfallDetail1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid
            && self.transaction_type == row.transaction_type && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingEftshortfallDetail1 {
    type PrimaryKey = BillingEftshortfallDetail1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid
            && self.transaction_type == key.transaction_type && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingEftshortfallDetail1PrimaryKey {
    type Row = BillingEftshortfallDetail1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid
            && self.transaction_type == row.transaction_type && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingEftshortfallDetail1PrimaryKey {
    type PrimaryKey = BillingEftshortfallDetail1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid
            && self.transaction_type == key.transaction_type && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingEftshortfallDetail1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("transaction_type",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut transaction_type_array = Vec::new();
        let mut amount_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            transaction_type_array.push(row.transaction_type);
            amount_array
                .push({
                    row.amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(transaction_type_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLING_ENERGY_TRAN_SAPS
///  _The SAP Billing Transaction Details for the Participants_
///
/// * Data Set Name: Billing
/// * File Name: Energy Tran Saps
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * TNI
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingEnergyTranSaps1 {
    /// The Billing Contract Year
    pub contractyear: rust_decimal::Decimal,
    /// The Billing WeekNo
    pub weekno: rust_decimal::Decimal,
    /// The Billing RunNo
    pub billrunno: rust_decimal::Decimal,
    /// The SAP Participant ID
    pub participantid: String,
    /// The SAPS Connection Point ID
    pub tni: String,
    /// The Region ID associated with the TNI
    pub regionid: Option<String>,
    /// The Energy MWh Consumed for that TNI for the Participant Id in that Billing Week
    pub consumed_energy_mwh: Option<rust_decimal::Decimal>,
    /// The Energy MWh Sent Out for the TNI for the Participant Id in that Billing Week
    pub sentout_energy_mwh: Option<rust_decimal::Decimal>,
    /// The Cost of the Consumed Energy
    pub consumed_energy_cost: Option<rust_decimal::Decimal>,
    /// The Cost of the Sent Out Energy
    pub sentout_energy_cost: Option<rust_decimal::Decimal>,
    /// The Last datetime record is updated
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingEnergyTranSaps1 {
    type PrimaryKey = BillingEnergyTranSaps1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("ENERGY_TRAN_SAPS".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingEnergyTranSaps1PrimaryKey {
        BillingEnergyTranSaps1PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            tni: self.tni.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_energy_tran_saps_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingEnergyTranSaps1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub tni: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingEnergyTranSaps1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingEnergyTranSaps1 {
    type Row = BillingEnergyTranSaps1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.tni == row.tni
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingEnergyTranSaps1 {
    type PrimaryKey = BillingEnergyTranSaps1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.tni == key.tni
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingEnergyTranSaps1PrimaryKey {
    type Row = BillingEnergyTranSaps1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.tni == row.tni
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingEnergyTranSaps1PrimaryKey {
    type PrimaryKey = BillingEnergyTranSaps1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.tni == key.tni
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingEnergyTranSaps1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("tni",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("consumed_energy_mwh",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("sentout_energy_mwh",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("consumed_energy_cost",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("sentout_energy_cost",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut tni_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut consumed_energy_mwh_array = Vec::new();
        let mut sentout_energy_mwh_array = Vec::new();
        let mut consumed_energy_cost_array = Vec::new();
        let mut sentout_energy_cost_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            tni_array.push(row.tni);
            regionid_array.push(row.regionid);
            consumed_energy_mwh_array
                .push({
                    row.consumed_energy_mwh
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            sentout_energy_mwh_array
                .push({
                    row.sentout_energy_mwh
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            consumed_energy_cost_array
                .push({
                    row.consumed_energy_cost
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            sentout_energy_cost_array
                .push({
                    row.sentout_energy_cost
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(tni_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(regionid_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(consumed_energy_mwh_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(sentout_energy_mwh_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(consumed_energy_cost_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(sentout_energy_cost_array)
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
/// ## BILLING_GST_DETAIL
///  _BILLING_GST_DETAIL shows the BAS class, GST_Exclusive and GST amount (if any) attributable to a participant for each transaction type._
///
/// * Data Set Name: Billing
/// * File Name: Gst Detail
/// * Data Version: 5
///
/// # Description
///  BILLING_GST_DETAIL data is confidential to NSP participants. Source Populated by the posting of a billing run. Volume Approximately 20 records are inserted per billrunno, or about 220 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * BAS_CLASS
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * TRANSACTION_TYPE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingGstDetail5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// The BAS classification that the transaction type belongs to.
    pub bas_class: String,
    /// The transaction type (e.g. CUSTOMER_ENERGY_PURCHASES)
    pub transaction_type: String,
    /// The GST exclusive amount paid by/to the participant to/by AEMO for this transaction type.
    pub gst_exclusive_amount: Option<rust_decimal::Decimal>,
    /// The GST amount for this transaction type.
    pub gst_amount: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingGstDetail5 {
    type PrimaryKey = BillingGstDetail5PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("GST_DETAIL".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingGstDetail5PrimaryKey {
        BillingGstDetail5PrimaryKey {
            bas_class: self.bas_class.clone(),
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            transaction_type: self.transaction_type.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_gst_detail_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingGstDetail5PrimaryKey {
    pub bas_class: String,
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub transaction_type: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingGstDetail5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingGstDetail5 {
    type Row = BillingGstDetail5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bas_class == row.bas_class && self.billrunno == row.billrunno
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid
            && self.transaction_type == row.transaction_type && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingGstDetail5 {
    type PrimaryKey = BillingGstDetail5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class && self.billrunno == key.billrunno
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid
            && self.transaction_type == key.transaction_type && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingGstDetail5PrimaryKey {
    type Row = BillingGstDetail5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bas_class == row.bas_class && self.billrunno == row.billrunno
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid
            && self.transaction_type == row.transaction_type && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingGstDetail5PrimaryKey {
    type PrimaryKey = BillingGstDetail5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class && self.billrunno == key.billrunno
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid
            && self.transaction_type == key.transaction_type && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingGstDetail5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("bas_class",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("transaction_type",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("gst_exclusive_amount",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("gst_amount",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut bas_class_array = Vec::new();
        let mut transaction_type_array = Vec::new();
        let mut gst_exclusive_amount_array = Vec::new();
        let mut gst_amount_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            bas_class_array.push(row.bas_class);
            transaction_type_array.push(row.transaction_type);
            gst_exclusive_amount_array
                .push({
                    row.gst_exclusive_amount
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            gst_amount_array
                .push({
                    row.gst_amount
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(bas_class_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(transaction_type_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(gst_exclusive_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(gst_amount_array)
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
/// ## BILLING_GST_SUMMARY
///  _BILLING_GST_SUMMARY shows the GST_Exclusive and GST amount (if any)  attributable to a participant for each BAS class._
///
/// * Data Set Name: Billing
/// * File Name: Gst Summary
/// * Data Version: 5
///
/// # Description
///  BILLING_GST_SUMMARY data is confidential to NSP participants. Source Populated by the posting of a billing run. Volume Approximately 5 records are inserted per billrunno, or about 55 records inserted per week.
///
///
///
/// # Primary Key Columns
///
/// * BAS_CLASS
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingGstSummary5 {
    /// AEMO Contract Year number starting in week containing 1st January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1st January
    pub weekno: rust_decimal::Decimal,
    /// Unique run no within a given contract year and week no
    pub billrunno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// The BAS classification
    pub bas_class: String,
    /// The GST exclusive amount paid by/to the participant to/by AEMO for this BAS classification.
    pub gst_exclusive_amount: Option<rust_decimal::Decimal>,
    /// The GST amount for this BAS classification.
    pub gst_amount: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingGstSummary5 {
    type PrimaryKey = BillingGstSummary5PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("GST_SUMMARY".into()),
            version: 5,
        }
    }
    fn primary_key(&self) -> BillingGstSummary5PrimaryKey {
        BillingGstSummary5PrimaryKey {
            bas_class: self.bas_class.clone(),
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_gst_summary_v5".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingGstSummary5PrimaryKey {
    pub bas_class: String,
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingGstSummary5PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingGstSummary5 {
    type Row = BillingGstSummary5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bas_class == row.bas_class && self.billrunno == row.billrunno
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingGstSummary5 {
    type PrimaryKey = BillingGstSummary5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class && self.billrunno == key.billrunno
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingGstSummary5PrimaryKey {
    type Row = BillingGstSummary5;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bas_class == row.bas_class && self.billrunno == row.billrunno
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingGstSummary5PrimaryKey {
    type PrimaryKey = BillingGstSummary5PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bas_class == key.bas_class && self.billrunno == key.billrunno
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingGstSummary5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("bas_class",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("gst_exclusive_amount",
                arrow2::datatypes::DataType::Decimal(15, 5), true),
                arrow2::datatypes::Field::new("gst_amount",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut bas_class_array = Vec::new();
        let mut gst_exclusive_amount_array = Vec::new();
        let mut gst_amount_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            bas_class_array.push(row.bas_class);
            gst_exclusive_amount_array
                .push({
                    row.gst_exclusive_amount
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            gst_amount_array
                .push({
                    row.gst_amount
                        .map(|mut val| {
                            val.rescale(5);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(bas_class_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(gst_exclusive_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(15, 5))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(gst_amount_array)
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
/// ## BILLING_NMAS_TST_PAYMENTS
///  _BILLING_NMAS_TEST_PAYMENTS publish the NSCAS/SRAS Testing Payments data for a posted billing week._
///
/// * Data Set Name: Billing
/// * File Name: Nmas Tst Payments
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTID
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * SERVICE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingNmasTstPayments1 {
    /// AEMO Contract Year number starting in week containing 1 January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1 January
    pub weekno: rust_decimal::Decimal,
    /// The current Billing RunNo for the week
    pub billrunno: rust_decimal::Decimal,
    /// The Participant from whom the amount is recovered
    pub participantid: String,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>
    pub service: String,
    /// The NMAS Contract Id
    pub contractid: String,
    /// The Testing Payment Amount to recover
    pub payment_amount: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingNmasTstPayments1 {
    type PrimaryKey = BillingNmasTstPayments1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("NMAS_TST_PAYMENTS".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingNmasTstPayments1PrimaryKey {
        BillingNmasTstPayments1PrimaryKey {
            billrunno: self.billrunno,
            contractid: self.contractid.clone(),
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            service: self.service.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_nmas_tst_payments_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingNmasTstPayments1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractid: String,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub service: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingNmasTstPayments1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingNmasTstPayments1 {
    type Row = BillingNmasTstPayments1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractid == row.contractid
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.service == row.service
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingNmasTstPayments1 {
    type PrimaryKey = BillingNmasTstPayments1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractid == key.contractid
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.service == key.service
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingNmasTstPayments1PrimaryKey {
    type Row = BillingNmasTstPayments1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractid == row.contractid
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.service == row.service
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingNmasTstPayments1PrimaryKey {
    type PrimaryKey = BillingNmasTstPayments1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractid == key.contractid
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.service == key.service
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingNmasTstPayments1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("service",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("contractid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("payment_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut service_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut payment_amount_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            service_array.push(row.service);
            contractid_array.push(row.contractid);
            payment_amount_array
                .push({
                    row.payment_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(service_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(contractid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(payment_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLING_NMAS_TST_RECOVERY
///  _BILLING_NMAS_TEST_RECOVERY sets out the recovery of NMAS testing payments_
///
/// * Data Set Name: Billing
/// * File Name: Nmas Tst Recovery
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTID
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * REGIONID
/// * SERVICE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingNmasTstRecovery1 {
    /// AEMO Contract Year number starting in week containing 1 January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1 January
    pub weekno: rust_decimal::Decimal,
    /// The current Billing RunNo for the week
    pub billrunno: rust_decimal::Decimal,
    /// The Participant from whom the amount is recovered
    pub participantid: String,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>- RESTART<br>
    pub service: String,
    /// The NMAS Contract Id
    pub contractid: String,
    /// The region from where the amount is recovered
    pub regionid: String,
    /// The Benefitting Factor for the RegionId
    pub rbf: Option<rust_decimal::Decimal>,
    /// The total Testing Payment Amount to recover from all benefitting regions
    pub test_payment: Option<rust_decimal::Decimal>,
    /// The Recovery Start Date for the Testing Payment Calculation
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub recovery_start_date: Option<chrono::NaiveDateTime>,
    /// The Recovery End Date for the Testing Payment Calculation
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub recovery_end_date: Option<chrono::NaiveDateTime>,
    /// The Participant energy in MWh for the recovery period
    pub participant_energy: Option<rust_decimal::Decimal>,
    /// The RegionId energy in MWh for the recovery period
    pub region_energy: Option<rust_decimal::Decimal>,
    /// The NEM energy in MWh for the recovery period
    pub nem_energy: Option<rust_decimal::Decimal>,
    /// The Customer Proportion for recovery amount in Percent
    pub customer_proportion: Option<rust_decimal::Decimal>,
    /// The Generator Proportion for recovery amount in Percent (100-Customer Portion)
    pub generator_proportion: Option<rust_decimal::Decimal>,
    /// The Participant Generation for the recovery period
    pub participant_generation: Option<rust_decimal::Decimal>,
    /// The NEM Generation for the recovery period
    pub nem_generation: Option<rust_decimal::Decimal>,
    /// The Total recovery amount for the billing week, being the sum of the customer and generator proportions for the PARTICIPANTID in REGIONID
    pub recovery_amount: Option<rust_decimal::Decimal>,
    /// The Last Updated date and time
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingNmasTstRecovery1 {
    type PrimaryKey = BillingNmasTstRecovery1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("NMAS_TST_RECOVERY".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingNmasTstRecovery1PrimaryKey {
        BillingNmasTstRecovery1PrimaryKey {
            billrunno: self.billrunno,
            contractid: self.contractid.clone(),
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            regionid: self.regionid.clone(),
            service: self.service.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_nmas_tst_recovery_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingNmasTstRecovery1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractid: String,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub regionid: String,
    pub service: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingNmasTstRecovery1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingNmasTstRecovery1 {
    type Row = BillingNmasTstRecovery1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractid == row.contractid
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.service == row.service && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingNmasTstRecovery1 {
    type PrimaryKey = BillingNmasTstRecovery1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractid == key.contractid
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.service == key.service && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingNmasTstRecovery1PrimaryKey {
    type Row = BillingNmasTstRecovery1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractid == row.contractid
            && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.regionid == row.regionid
            && self.service == row.service && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingNmasTstRecovery1PrimaryKey {
    type PrimaryKey = BillingNmasTstRecovery1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractid == key.contractid
            && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.regionid == key.regionid
            && self.service == key.service && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingNmasTstRecovery1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("service",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("contractid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("rbf",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("test_payment",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("recovery_start_date",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("recovery_end_date",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("participant_energy",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("region_energy",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("nem_energy",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("customer_proportion",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("generator_proportion",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("participant_generation",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("nem_generation",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("recovery_amount",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut service_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut rbf_array = Vec::new();
        let mut test_payment_array = Vec::new();
        let mut recovery_start_date_array = Vec::new();
        let mut recovery_end_date_array = Vec::new();
        let mut participant_energy_array = Vec::new();
        let mut region_energy_array = Vec::new();
        let mut nem_energy_array = Vec::new();
        let mut customer_proportion_array = Vec::new();
        let mut generator_proportion_array = Vec::new();
        let mut participant_generation_array = Vec::new();
        let mut nem_generation_array = Vec::new();
        let mut recovery_amount_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            service_array.push(row.service);
            contractid_array.push(row.contractid);
            regionid_array.push(row.regionid);
            rbf_array
                .push({
                    row.rbf
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            test_payment_array
                .push({
                    row.test_payment
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            recovery_start_date_array
                .push(row.recovery_start_date.map(|val| val.timestamp()));
            recovery_end_date_array
                .push(row.recovery_end_date.map(|val| val.timestamp()));
            participant_energy_array
                .push({
                    row.participant_energy
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            region_energy_array
                .push({
                    row.region_energy
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            nem_energy_array
                .push({
                    row.nem_energy
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            customer_proportion_array
                .push({
                    row.customer_proportion
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            generator_proportion_array
                .push({
                    row.generator_proportion
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            participant_generation_array
                .push({
                    row.participant_generation
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            nem_generation_array
                .push({
                    row.nem_generation
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            recovery_amount_array
                .push({
                    row.recovery_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(service_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(contractid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rbf_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(test_payment_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(recovery_start_date_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(recovery_end_date_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(participant_energy_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(region_energy_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(nem_energy_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(customer_proportion_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(generator_proportion_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(participant_generation_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(nem_generation_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(recovery_amount_array)
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
/// ## BILLING_NMAS_TST_RECVRY_RBF
///  _BILLING_NMAS_TEST_RECVRY_RBF sets out the NSCAS/SRAS Testing Payment recovery data for the posted billing week._
///
/// * Data Set Name: Billing
/// * File Name: Nmas Tst Recvry Rbf
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTID
/// * CONTRACTYEAR
/// * REGIONID
/// * SERVICE
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingNmasTstRecvryRbf1 {
    /// AEMO Contract Year number starting in week containing 1 January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1 January
    pub weekno: rust_decimal::Decimal,
    /// The current Billing RunNo for the week
    pub billrunno: rust_decimal::Decimal,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>
    pub service: String,
    /// The NMAS Contract Id
    pub contractid: String,
    /// The region from where the amount is recovered
    pub regionid: String,
    /// The Benefitting Factor for the RegionId
    pub rbf: Option<rust_decimal::Decimal>,
    /// The total Testing Payment Amount to recover from all benefitting regions
    pub payment_amount: Option<rust_decimal::Decimal>,
    /// The Testing Payment amount to recover from RegionId
    pub recovery_amount: Option<rust_decimal::Decimal>,
    /// The Last Updated date and time
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for BillingNmasTstRecvryRbf1 {
    type PrimaryKey = BillingNmasTstRecvryRbf1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("NMAS_TST_RECVRY_RBF".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingNmasTstRecvryRbf1PrimaryKey {
        BillingNmasTstRecvryRbf1PrimaryKey {
            billrunno: self.billrunno,
            contractid: self.contractid.clone(),
            contractyear: self.contractyear,
            regionid: self.regionid.clone(),
            service: self.service.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_nmas_tst_recvry_rbf_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingNmasTstRecvryRbf1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractid: String,
    pub contractyear: rust_decimal::Decimal,
    pub regionid: String,
    pub service: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingNmasTstRecvryRbf1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingNmasTstRecvryRbf1 {
    type Row = BillingNmasTstRecvryRbf1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractid == row.contractid
            && self.contractyear == row.contractyear && self.regionid == row.regionid
            && self.service == row.service && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingNmasTstRecvryRbf1 {
    type PrimaryKey = BillingNmasTstRecvryRbf1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractid == key.contractid
            && self.contractyear == key.contractyear && self.regionid == key.regionid
            && self.service == key.service && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingNmasTstRecvryRbf1PrimaryKey {
    type Row = BillingNmasTstRecvryRbf1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractid == row.contractid
            && self.contractyear == row.contractyear && self.regionid == row.regionid
            && self.service == row.service && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingNmasTstRecvryRbf1PrimaryKey {
    type PrimaryKey = BillingNmasTstRecvryRbf1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractid == key.contractid
            && self.contractyear == key.contractyear && self.regionid == key.regionid
            && self.service == key.service && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingNmasTstRecvryRbf1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("service",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("contractid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("rbf",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("payment_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("recovery_amount",
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
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut service_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut rbf_array = Vec::new();
        let mut payment_amount_array = Vec::new();
        let mut recovery_amount_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            service_array.push(row.service);
            contractid_array.push(row.contractid);
            regionid_array.push(row.regionid);
            rbf_array
                .push({
                    row.rbf
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            payment_amount_array
                .push({
                    row.payment_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            recovery_amount_array
                .push({
                    row.recovery_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(service_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(contractid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(rbf_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(payment_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(recovery_amount_array)
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
/// ## BILLING_NMAS_TST_RECVRY_TRK
///  _BILLING_NMAS_TEST_RECVRY_TRK tracks the energy data used to allocate the test payment recovery over the recovery period._
///
/// * Data Set Name: Billing
/// * File Name: Nmas Tst Recvry Trk
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * RECOVERY_BILLRUNNO
/// * RECOVERY_CONTRACTYEAR
/// * RECOVERY_WEEKNO
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingNmasTstRecvryTrk1 {
    /// AEMO Contract Year number starting in week containing 1 January
    pub contractyear: rust_decimal::Decimal,
    /// Week no within the contract year. Week no 1 is the week containing 1 January
    pub weekno: rust_decimal::Decimal,
    /// The current Billing RunNo for the week
    pub billrunno: rust_decimal::Decimal,
    /// AEMO Contract Year for energy data used in recovery calculation
    pub recovery_contractyear: rust_decimal::Decimal,
    /// Week no for energy data used in recovery calculation
    pub recovery_weekno: rust_decimal::Decimal,
    /// Billing RunNo for energy data used in recovery calculation
    pub recovery_billrunno: rust_decimal::Decimal,
}
impl mmsdm_core::GetTable for BillingNmasTstRecvryTrk1 {
    type PrimaryKey = BillingNmasTstRecvryTrk1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("NMAS_TST_RECVRY_TRK".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingNmasTstRecvryTrk1PrimaryKey {
        BillingNmasTstRecvryTrk1PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            recovery_billrunno: self.recovery_billrunno,
            recovery_contractyear: self.recovery_contractyear,
            recovery_weekno: self.recovery_weekno,
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_nmas_tst_recvry_trk_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingNmasTstRecvryTrk1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub recovery_billrunno: rust_decimal::Decimal,
    pub recovery_contractyear: rust_decimal::Decimal,
    pub recovery_weekno: rust_decimal::Decimal,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingNmasTstRecvryTrk1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingNmasTstRecvryTrk1 {
    type Row = BillingNmasTstRecvryTrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.recovery_billrunno == row.recovery_billrunno
            && self.recovery_contractyear == row.recovery_contractyear
            && self.recovery_weekno == row.recovery_weekno && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingNmasTstRecvryTrk1 {
    type PrimaryKey = BillingNmasTstRecvryTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.recovery_billrunno == key.recovery_billrunno
            && self.recovery_contractyear == key.recovery_contractyear
            && self.recovery_weekno == key.recovery_weekno && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingNmasTstRecvryTrk1PrimaryKey {
    type Row = BillingNmasTstRecvryTrk1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.recovery_billrunno == row.recovery_billrunno
            && self.recovery_contractyear == row.recovery_contractyear
            && self.recovery_weekno == row.recovery_weekno && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingNmasTstRecvryTrk1PrimaryKey {
    type PrimaryKey = BillingNmasTstRecvryTrk1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.recovery_billrunno == key.recovery_billrunno
            && self.recovery_contractyear == key.recovery_contractyear
            && self.recovery_weekno == key.recovery_weekno && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingNmasTstRecvryTrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("recovery_contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("recovery_weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("recovery_billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut recovery_contractyear_array = Vec::new();
        let mut recovery_weekno_array = Vec::new();
        let mut recovery_billrunno_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            recovery_contractyear_array
                .push({
                    let mut val = row.recovery_contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            recovery_weekno_array
                .push({
                    let mut val = row.recovery_weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            recovery_billrunno_array
                .push({
                    let mut val = row.recovery_billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(recovery_contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(recovery_weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(recovery_billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLING_SECDEPOSIT_APPLICATION
///  _The security deposit application details_
///
/// * Data Set Name: Billing
/// * File Name: Secdeposit Application
/// * Data Version: 1
///
/// # Description
///  BILLING_SECDEPOSIT_APPLICATION data is confidential, and is available only to the relevant participant.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSecdepositApplication1 {
    /// The billing contract year where (security deposit application) SDA is applied
    pub contractyear: rust_decimal::Decimal,
    /// The billing week no. where the SDA is applied
    pub weekno: rust_decimal::Decimal,
    /// The billing run no. where the SDA is applied
    pub billrunno: rust_decimal::Decimal,
    /// The Participant ID lodging the SDA
    pub participantid: String,
    /// The SDA application amount
    pub application_amount: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingSecdepositApplication1 {
    type PrimaryKey = BillingSecdepositApplication1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("SECDEPOSIT_APPLICATION".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingSecdepositApplication1PrimaryKey {
        BillingSecdepositApplication1PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_secdeposit_application_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingSecdepositApplication1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingSecdepositApplication1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingSecdepositApplication1 {
    type Row = BillingSecdepositApplication1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingSecdepositApplication1 {
    type PrimaryKey = BillingSecdepositApplication1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingSecdepositApplication1PrimaryKey {
    type Row = BillingSecdepositApplication1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingSecdepositApplication1PrimaryKey {
    type PrimaryKey = BillingSecdepositApplication1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingSecdepositApplication1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("application_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut application_amount_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            application_amount_array
                .push({
                    row.application_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(application_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLING_SECDEP_INTEREST_PAY
///  _The interest amount for security deposit calculated by billing, based on whether it is a fixed/floating rate_
///
/// * Data Set Name: Billing
/// * File Name: Secdep Interest Pay
/// * Data Version: 1
///
/// # Description
///  BILLING_SECDEP_INTEREST_PAY data is confidential, and is available only to the relevant participant.
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * SECURITY_DEPOSIT_ID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSecdepInterestPay1 {
    /// The billing contract year the SDA application is processed and interest calculated
    pub contractyear: rust_decimal::Decimal,
    /// The billing week no. the SDA application is processed and interest calculated
    pub weekno: rust_decimal::Decimal,
    /// The billing run no. the SDA application is processed and interest calculated
    pub billrunno: rust_decimal::Decimal,
    /// The security deposit ID for which billing has calculated the Interest amount
    pub security_deposit_id: String,
    /// The participant ID of the security deposit for whom the interest is paid
    pub participantid: String,
    /// The security deposit interest amount calculated by billing
    pub interest_amount: Option<rust_decimal::Decimal>,
    /// FIXED or DAILY
    pub interest_calc_type: Option<String>,
    /// The interest account ID used by billing for calculating the interest. <br>NULL if INTEREST_CALC_TYPE = FIXED<br>
    pub interest_acct_id: Option<String>,
    /// The STATIC Interest Rate used by Billing for calculating the interest. This is NULL if INTEREST_CALC_TYPE &lt;&gt; FIXED
    pub interest_rate: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingSecdepInterestPay1 {
    type PrimaryKey = BillingSecdepInterestPay1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("SECDEP_INTEREST_PAY".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingSecdepInterestPay1PrimaryKey {
        BillingSecdepInterestPay1PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            security_deposit_id: self.security_deposit_id.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_secdep_interest_pay_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingSecdepInterestPay1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub security_deposit_id: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingSecdepInterestPay1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingSecdepInterestPay1 {
    type Row = BillingSecdepInterestPay1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid
            && self.security_deposit_id == row.security_deposit_id
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingSecdepInterestPay1 {
    type PrimaryKey = BillingSecdepInterestPay1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid
            && self.security_deposit_id == key.security_deposit_id
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingSecdepInterestPay1PrimaryKey {
    type Row = BillingSecdepInterestPay1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid
            && self.security_deposit_id == row.security_deposit_id
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingSecdepInterestPay1PrimaryKey {
    type PrimaryKey = BillingSecdepInterestPay1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid
            && self.security_deposit_id == key.security_deposit_id
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingSecdepInterestPay1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("security_deposit_id",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("interest_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("interest_calc_type",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("interest_acct_id",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("interest_rate",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut security_deposit_id_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut interest_amount_array = Vec::new();
        let mut interest_calc_type_array = Vec::new();
        let mut interest_acct_id_array = Vec::new();
        let mut interest_rate_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            security_deposit_id_array.push(row.security_deposit_id);
            participantid_array.push(row.participantid);
            interest_amount_array
                .push({
                    row.interest_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            interest_calc_type_array.push(row.interest_calc_type);
            interest_acct_id_array.push(row.interest_acct_id);
            interest_rate_array
                .push({
                    row.interest_rate
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(security_deposit_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(interest_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(interest_calc_type_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(interest_acct_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(interest_rate_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLING_SECDEP_INTEREST_RATE
///  _The DAILY interest rates used by billing when calculating the interest amount_
///
/// * Data Set Name: Billing
/// * File Name: Secdep Interest Rate
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * EFFECTIVEDATE
/// * INTEREST_ACCT_ID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSecdepInterestRate1 {
    /// The billing contract year the SDA application is processed and interest calculated
    pub contractyear: rust_decimal::Decimal,
    /// The billing week no. the SDA application is processed and interest calculated
    pub weekno: rust_decimal::Decimal,
    /// The billing run no. the SDA application is processed and interest calculated
    pub billrunno: rust_decimal::Decimal,
    /// The interest account ID used by security deposit interest calculation
    pub interest_acct_id: String,
    /// The effective date of the new interest change
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// The interest rate to apply from the effective date
    pub interest_rate: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingSecdepInterestRate1 {
    type PrimaryKey = BillingSecdepInterestRate1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("SECDEP_INTEREST_RATE".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingSecdepInterestRate1PrimaryKey {
        BillingSecdepInterestRate1PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            effectivedate: self.effectivedate,
            interest_acct_id: self.interest_acct_id.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: self.effectivedate.year(),
            month: num_traits::FromPrimitive::from_u32(self.effectivedate.month())
                .unwrap(),
        }
    }
    fn partition_name(&self) -> String {
        format!(
            "billing_secdep_interest_rate_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingSecdepInterestRate1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub effectivedate: chrono::NaiveDateTime,
    pub interest_acct_id: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingSecdepInterestRate1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingSecdepInterestRate1 {
    type Row = BillingSecdepInterestRate1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.effectivedate == row.effectivedate
            && self.interest_acct_id == row.interest_acct_id && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingSecdepInterestRate1 {
    type PrimaryKey = BillingSecdepInterestRate1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.effectivedate == key.effectivedate
            && self.interest_acct_id == key.interest_acct_id && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingSecdepInterestRate1PrimaryKey {
    type Row = BillingSecdepInterestRate1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.effectivedate == row.effectivedate
            && self.interest_acct_id == row.interest_acct_id && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingSecdepInterestRate1PrimaryKey {
    type PrimaryKey = BillingSecdepInterestRate1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.effectivedate == key.effectivedate
            && self.interest_acct_id == key.interest_acct_id && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingSecdepInterestRate1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("interest_acct_id",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("effectivedate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("interest_rate",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut interest_acct_id_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut interest_rate_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            interest_acct_id_array.push(row.interest_acct_id);
            effectivedate_array.push(row.effectivedate.timestamp());
            interest_rate_array
                .push({
                    row.interest_rate
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(interest_acct_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(interest_rate_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLING_SUBST_DEMAND
///  _Demand Values Substituted in Billing Calculation_
///
/// * Data Set Name: Billing
/// * File Name: Subst Demand
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * SETTLEMENTDATE
/// * TNI
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSubstDemand1 {
    /// Billing contract year
    pub contractyear: rust_decimal::Decimal,
    /// Billing week number
    pub weekno: rust_decimal::Decimal,
    /// Billing run number
    pub billrunno: rust_decimal::Decimal,
    /// Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Unique identifier for the connection point
    pub tni: String,
    /// Unique identifier for the participant
    pub participantid: String,
    /// Unique identifier for the region to which the TNI belongs to on this settlement date
    pub regionid: Option<String>,
    /// Substitute metered quantity for non-energy recovery in MWh for the TNI and participant in the trading interval. A negative value indicates net consumption and a positive value indicates net generation
    pub substitutedemand: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingSubstDemand1 {
    type PrimaryKey = BillingSubstDemand1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("SUBST_DEMAND".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingSubstDemand1PrimaryKey {
        BillingSubstDemand1PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            settlementdate: self.settlementdate,
            tni: self.tni.clone(),
            weekno: self.weekno,
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
            "billing_subst_demand_v1_{}_{}", self.partition_suffix().year, self
            .partition_suffix().month.number_from_month()
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingSubstDemand1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub tni: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingSubstDemand1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingSubstDemand1 {
    type Row = BillingSubstDemand1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid
            && self.settlementdate == row.settlementdate && self.tni == row.tni
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingSubstDemand1 {
    type PrimaryKey = BillingSubstDemand1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid
            && self.settlementdate == key.settlementdate && self.tni == key.tni
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingSubstDemand1PrimaryKey {
    type Row = BillingSubstDemand1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid
            && self.settlementdate == row.settlementdate && self.tni == row.tni
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingSubstDemand1PrimaryKey {
    type PrimaryKey = BillingSubstDemand1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid
            && self.settlementdate == key.settlementdate && self.tni == key.tni
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingSubstDemand1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("tni",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("substitutedemand",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut settlementdate_array = Vec::new();
        let mut tni_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut substitutedemand_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            settlementdate_array.push(row.settlementdate.timestamp());
            tni_array.push(row.tni);
            participantid_array.push(row.participantid);
            regionid_array.push(row.regionid);
            substitutedemand_array
                .push({
                    row.substitutedemand
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(settlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(tni_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(regionid_array)) as std::sync::Arc < dyn arrow2::array::Array
                    >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(substitutedemand_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLING_SUBST_RUN_VERSION
///  _Details of settlement runs used as input in the substitute demand calculation_
///
/// * Data Set Name: Billing
/// * File Name: Subst Run Version
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * REFERENCESETTLEMENTDATE
/// * REFERENCESETTLEMENTRUNNO
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingSubstRunVersion1 {
    /// Billing contract year
    pub contractyear: rust_decimal::Decimal,
    /// Billing week number
    pub weekno: rust_decimal::Decimal,
    /// Billing run number
    pub billrunno: rust_decimal::Decimal,
    /// Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub referencesettlementdate: chrono::NaiveDateTime,
    /// The settlement run number matching the settlement date for a settlement run included in the reference period
    pub referencesettlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::GetTable for BillingSubstRunVersion1 {
    type PrimaryKey = BillingSubstRunVersion1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("SUBST_RUN_VERSION".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingSubstRunVersion1PrimaryKey {
        BillingSubstRunVersion1PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            referencesettlementdate: self.referencesettlementdate,
            referencesettlementrunno: self.referencesettlementrunno,
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_subst_run_version_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingSubstRunVersion1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub referencesettlementdate: chrono::NaiveDateTime,
    pub referencesettlementrunno: rust_decimal::Decimal,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingSubstRunVersion1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingSubstRunVersion1 {
    type Row = BillingSubstRunVersion1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.referencesettlementdate == row.referencesettlementdate
            && self.referencesettlementrunno == row.referencesettlementrunno
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingSubstRunVersion1 {
    type PrimaryKey = BillingSubstRunVersion1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.referencesettlementdate == key.referencesettlementdate
            && self.referencesettlementrunno == key.referencesettlementrunno
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingSubstRunVersion1PrimaryKey {
    type Row = BillingSubstRunVersion1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.referencesettlementdate == row.referencesettlementdate
            && self.referencesettlementrunno == row.referencesettlementrunno
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingSubstRunVersion1PrimaryKey {
    type PrimaryKey = BillingSubstRunVersion1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.referencesettlementdate == key.referencesettlementdate
            && self.referencesettlementrunno == key.referencesettlementrunno
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingSubstRunVersion1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("referencesettlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), false), arrow2::datatypes::Field::new("referencesettlementrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut referencesettlementdate_array = Vec::new();
        let mut referencesettlementrunno_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            referencesettlementdate_array.push(row.referencesettlementdate.timestamp());
            referencesettlementrunno_array
                .push({
                    let mut val = row.referencesettlementrunno;
                    val.rescale(0);
                    val.mantissa()
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(referencesettlementdate_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(referencesettlementrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLING_WDR
///  _Billing WDR Transaction Weekly Summary_
///
/// * Data Set Name: Billing
/// * File Name: Wdr
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingWdr1 {
    /// Contract year of the Billing run
    pub contractyear: rust_decimal::Decimal,
    /// Week number of the Billing run
    pub weekno: rust_decimal::Decimal,
    /// Billing run number identifier
    pub billrunno: rust_decimal::Decimal,
    /// DRSP or FRMP Participant Identifier
    pub participantid: String,
    /// WDR credit transaction amount
    pub wdr_credit_amount: Option<rust_decimal::Decimal>,
    /// WDR debit transaction amount
    pub wdr_debit_amount: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingWdr1 {
    type PrimaryKey = BillingWdr1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("WDR".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingWdr1PrimaryKey {
        BillingWdr1PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_wdr_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingWdr1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingWdr1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingWdr1 {
    type Row = BillingWdr1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingWdr1 {
    type PrimaryKey = BillingWdr1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingWdr1PrimaryKey {
    type Row = BillingWdr1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingWdr1PrimaryKey {
    type PrimaryKey = BillingWdr1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingWdr1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("wdr_credit_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("wdr_debit_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut wdr_credit_amount_array = Vec::new();
        let mut wdr_debit_amount_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            wdr_credit_amount_array
                .push({
                    row.wdr_credit_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            wdr_debit_amount_array
                .push({
                    row.wdr_debit_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(wdr_credit_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(wdr_debit_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLING_WDR_DETAIL
///  _Billing WDR transaction detail summary_
///
/// * Data Set Name: Billing
/// * File Name: Wdr Detail
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * DRSP
/// * FRMP
/// * REGIONID
/// * WDRRRPERIOD
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingWdrDetail1 {
    /// Contract year of the Billing run
    pub contractyear: rust_decimal::Decimal,
    /// Week number of the Billing run
    pub weekno: rust_decimal::Decimal,
    /// Billing run number identifier
    pub billrunno: rust_decimal::Decimal,
    /// Unique identifier for the period to which the WDRRR applies. For quarter-based periods, this will be equal to YYYY[Q]NN, for example, 2020Q3 for 2020 Quarter 3.
    pub wdrrrperiod: String,
    /// Region identifier
    pub regionid: String,
    /// Financial Responsible Market Participant Identifier
    pub frmp: String,
    /// Demand Response Service Provider Identifier
    pub drsp: String,
    /// WDR Settlement Quantity capped in MWh
    pub wdrsq: Option<rust_decimal::Decimal>,
    /// WDR reimbursement rate in $/MWh
    pub wdrrr: Option<rust_decimal::Decimal>,
    /// WDR transaction amount in $ for demand response
    pub wdrta: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingWdrDetail1 {
    type PrimaryKey = BillingWdrDetail1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("WDR_DETAIL".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingWdrDetail1PrimaryKey {
        BillingWdrDetail1PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            drsp: self.drsp.clone(),
            frmp: self.frmp.clone(),
            regionid: self.regionid.clone(),
            wdrrrperiod: self.wdrrrperiod.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_wdr_detail_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingWdrDetail1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub drsp: String,
    pub frmp: String,
    pub regionid: String,
    pub wdrrrperiod: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingWdrDetail1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingWdrDetail1 {
    type Row = BillingWdrDetail1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.drsp == row.drsp && self.frmp == row.frmp
            && self.regionid == row.regionid && self.wdrrrperiod == row.wdrrrperiod
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingWdrDetail1 {
    type PrimaryKey = BillingWdrDetail1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.drsp == key.drsp && self.frmp == key.frmp
            && self.regionid == key.regionid && self.wdrrrperiod == key.wdrrrperiod
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingWdrDetail1PrimaryKey {
    type Row = BillingWdrDetail1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.drsp == row.drsp && self.frmp == row.frmp
            && self.regionid == row.regionid && self.wdrrrperiod == row.wdrrrperiod
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingWdrDetail1PrimaryKey {
    type PrimaryKey = BillingWdrDetail1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.drsp == key.drsp && self.frmp == key.frmp
            && self.regionid == key.regionid && self.wdrrrperiod == key.wdrrrperiod
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingWdrDetail1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("wdrrrperiod",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("frmp",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("drsp",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("wdrsq",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("wdrrr",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("wdrta",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut wdrrrperiod_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut frmp_array = Vec::new();
        let mut drsp_array = Vec::new();
        let mut wdrsq_array = Vec::new();
        let mut wdrrr_array = Vec::new();
        let mut wdrta_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            wdrrrperiod_array.push(row.wdrrrperiod);
            regionid_array.push(row.regionid);
            frmp_array.push(row.frmp);
            drsp_array.push(row.drsp);
            wdrsq_array
                .push({
                    row.wdrsq
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            wdrrr_array
                .push({
                    row.wdrrr
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            wdrta_array
                .push({
                    row.wdrta
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(wdrrrperiod_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(frmp_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(drsp_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(wdrsq_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(wdrrr_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(wdrta_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLRESERVETRADERPAYMENT
///  _Details of the RERT Usage and Availability Payments made to the participant. _
///
/// * Data Set Name: Billing
/// * File Name: Reservetraderpayment
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTID
/// * CONTRACTYEAR
/// * PAYMENT_ID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingReservetraderpayment1 {
    /// Billing contract year
    pub contractyear: rust_decimal::Decimal,
    /// Billing week number
    pub weekno: rust_decimal::Decimal,
    /// Billing posted run number
    pub billrunno: rust_decimal::Decimal,
    /// Participant identifier.
    pub participantid: Option<String>,
    /// RERT payment contract ID
    pub contractid: String,
    /// RERT payment number
    pub payment_id: rust_decimal::Decimal,
    /// Description for the reserve trader contract payment amount.
    pub payment_type: Option<String>,
    /// RERT payment amount for the payment type
    pub payment_amount: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingReservetraderpayment1 {
    type PrimaryKey = BillingReservetraderpayment1PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("RESERVETRADERPAYMENT".into()),
            version: 1,
        }
    }
    fn primary_key(&self) -> BillingReservetraderpayment1PrimaryKey {
        BillingReservetraderpayment1PrimaryKey {
            billrunno: self.billrunno,
            contractid: self.contractid.clone(),
            contractyear: self.contractyear,
            payment_id: self.payment_id,
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_reservetraderpayment_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingReservetraderpayment1PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractid: String,
    pub contractyear: rust_decimal::Decimal,
    pub payment_id: rust_decimal::Decimal,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingReservetraderpayment1PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingReservetraderpayment1 {
    type Row = BillingReservetraderpayment1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractid == row.contractid
            && self.contractyear == row.contractyear && self.payment_id == row.payment_id
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingReservetraderpayment1 {
    type PrimaryKey = BillingReservetraderpayment1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractid == key.contractid
            && self.contractyear == key.contractyear && self.payment_id == key.payment_id
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingReservetraderpayment1PrimaryKey {
    type Row = BillingReservetraderpayment1;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractid == row.contractid
            && self.contractyear == row.contractyear && self.payment_id == row.payment_id
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingReservetraderpayment1PrimaryKey {
    type PrimaryKey = BillingReservetraderpayment1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractid == key.contractid
            && self.contractyear == key.contractyear && self.payment_id == key.payment_id
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingReservetraderpayment1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("contractid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("payment_id",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("payment_type",
                arrow2::datatypes::DataType::LargeUtf8, true),
                arrow2::datatypes::Field::new("payment_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut payment_id_array = Vec::new();
        let mut payment_type_array = Vec::new();
        let mut payment_amount_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            participantid_array.push(row.participantid);
            contractid_array.push(row.contractid);
            payment_id_array
                .push({
                    let mut val = row.payment_id;
                    val.rescale(0);
                    val.mantissa()
                });
            payment_type_array.push(row.payment_type);
            payment_amount_array
                .push({
                    row.payment_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(contractid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(payment_id_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from(payment_type_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(payment_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                ],
            )
            .map_err(Into::into)
    }
}
/// # Summary
///
/// ## BILLRESERVETRADERRECOVERY
///  _Provides details of the RERT Recovery Amount for the Market Customers._
///
/// * Data Set Name: Billing
/// * File Name: Reservetraderrecovery
/// * Data Version: 2
///
///
///
///
///
/// # Primary Key Columns
///
/// * BILLRUNNO
/// * CONTRACTYEAR
/// * PARTICIPANTID
/// * PAYMENT_ID
/// * PUBLICATION_ID
/// * REGIONID
/// * WEEKNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BillingReservetraderrecovery2 {
    /// Billing contract year
    pub contractyear: rust_decimal::Decimal,
    /// Billing week number
    pub weekno: rust_decimal::Decimal,
    /// Billing posted run number
    pub billrunno: rust_decimal::Decimal,
    /// Unique Publication Identifier for RERT Payment
    pub publication_id: String,
    /// RERT payment number
    pub payment_id: rust_decimal::Decimal,
    /// RERT payment amount
    pub payment_amount: Option<rust_decimal::Decimal>,
    /// Participant identifier.
    pub participantid: String,
    /// Region from which the amount is recovered
    pub regionid: String,
    /// Participant Demand Value used for RERT Recovery
    pub participant_demand: Option<rust_decimal::Decimal>,
    /// Region Demand Value used for RERT Recovery
    pub region_demand: Option<rust_decimal::Decimal>,
    /// Starting Period of RERT Recovery for Usage Payments
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub eligibility_start_interval: Option<chrono::NaiveDateTime>,
    /// Ending Period of RERT Recovery for Usage Payments
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub eligibility_end_interval: Option<chrono::NaiveDateTime>,
    /// Recovery Amount applicable for each Market Customer
    pub recovery_amount: Option<rust_decimal::Decimal>,
    /// The Energy Value (Scheduled Loads) that is excluded
    pub excluded_energy: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for BillingReservetraderrecovery2 {
    type PrimaryKey = BillingReservetraderrecovery2PrimaryKey;
    type Partition = ();
    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "BILLING".into(),
            table_name: Some("RESERVETRADERRECOVERY".into()),
            version: 2,
        }
    }
    fn primary_key(&self) -> BillingReservetraderrecovery2PrimaryKey {
        BillingReservetraderrecovery2PrimaryKey {
            billrunno: self.billrunno,
            contractyear: self.contractyear,
            participantid: self.participantid.clone(),
            payment_id: self.payment_id,
            publication_id: self.publication_id.clone(),
            regionid: self.regionid.clone(),
            weekno: self.weekno,
        }
    }
    fn partition_suffix(&self) -> Self::Partition {}
    fn partition_name(&self) -> String {
        "billing_reservetraderrecovery_v2".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct BillingReservetraderrecovery2PrimaryKey {
    pub billrunno: rust_decimal::Decimal,
    pub contractyear: rust_decimal::Decimal,
    pub participantid: String,
    pub payment_id: rust_decimal::Decimal,
    pub publication_id: String,
    pub regionid: String,
    pub weekno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for BillingReservetraderrecovery2PrimaryKey {}
impl mmsdm_core::CompareWithRow for BillingReservetraderrecovery2 {
    type Row = BillingReservetraderrecovery2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid
            && self.payment_id == row.payment_id
            && self.publication_id == row.publication_id && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingReservetraderrecovery2 {
    type PrimaryKey = BillingReservetraderrecovery2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid
            && self.payment_id == key.payment_id
            && self.publication_id == key.publication_id && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
impl mmsdm_core::CompareWithRow for BillingReservetraderrecovery2PrimaryKey {
    type Row = BillingReservetraderrecovery2;
    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.billrunno == row.billrunno && self.contractyear == row.contractyear
            && self.participantid == row.participantid
            && self.payment_id == row.payment_id
            && self.publication_id == row.publication_id && self.regionid == row.regionid
            && self.weekno == row.weekno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for BillingReservetraderrecovery2PrimaryKey {
    type PrimaryKey = BillingReservetraderrecovery2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.billrunno == key.billrunno && self.contractyear == key.contractyear
            && self.participantid == key.participantid
            && self.payment_id == key.payment_id
            && self.publication_id == key.publication_id && self.regionid == key.regionid
            && self.weekno == key.weekno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for BillingReservetraderrecovery2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(
            vec![
                arrow2::datatypes::Field::new("contractyear",
                arrow2::datatypes::DataType::Decimal(4, 0), false),
                arrow2::datatypes::Field::new("weekno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("billrunno",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("publication_id",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("payment_id",
                arrow2::datatypes::DataType::Decimal(3, 0), false),
                arrow2::datatypes::Field::new("payment_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("participantid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("regionid",
                arrow2::datatypes::DataType::LargeUtf8, false),
                arrow2::datatypes::Field::new("participant_demand",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("region_demand",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("eligibility_start_interval",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("eligibility_end_interval",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                None), true), arrow2::datatypes::Field::new("recovery_amount",
                arrow2::datatypes::DataType::Decimal(18, 8), true),
                arrow2::datatypes::Field::new("excluded_energy",
                arrow2::datatypes::DataType::Decimal(18, 8), true)
            ],
        )
    }
    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<
        arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>,
    > {
        let mut contractyear_array = Vec::new();
        let mut weekno_array = Vec::new();
        let mut billrunno_array = Vec::new();
        let mut publication_id_array = Vec::new();
        let mut payment_id_array = Vec::new();
        let mut payment_amount_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut participant_demand_array = Vec::new();
        let mut region_demand_array = Vec::new();
        let mut eligibility_start_interval_array = Vec::new();
        let mut eligibility_end_interval_array = Vec::new();
        let mut recovery_amount_array = Vec::new();
        let mut excluded_energy_array = Vec::new();
        for row in partition {
            contractyear_array
                .push({
                    let mut val = row.contractyear;
                    val.rescale(0);
                    val.mantissa()
                });
            weekno_array
                .push({
                    let mut val = row.weekno;
                    val.rescale(0);
                    val.mantissa()
                });
            billrunno_array
                .push({
                    let mut val = row.billrunno;
                    val.rescale(0);
                    val.mantissa()
                });
            publication_id_array.push(row.publication_id);
            payment_id_array
                .push({
                    let mut val = row.payment_id;
                    val.rescale(0);
                    val.mantissa()
                });
            payment_amount_array
                .push({
                    row.payment_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            participantid_array.push(row.participantid);
            regionid_array.push(row.regionid);
            participant_demand_array
                .push({
                    row.participant_demand
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            region_demand_array
                .push({
                    row.region_demand
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            eligibility_start_interval_array
                .push(row.eligibility_start_interval.map(|val| val.timestamp()));
            eligibility_end_interval_array
                .push(row.eligibility_end_interval.map(|val| val.timestamp()));
            recovery_amount_array
                .push({
                    row.recovery_amount
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
            excluded_energy_array
                .push({
                    row.excluded_energy
                        .map(|mut val| {
                            val.rescale(8);
                            val.mantissa()
                        })
                });
        }
        arrow2::chunk::Chunk::try_new(
                vec![
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(contractyear_array)
                    .to(arrow2::datatypes::DataType::Decimal(4, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(weekno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(billrunno_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(publication_id_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(payment_id_array)
                    .to(arrow2::datatypes::DataType::Decimal(3, 0))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(payment_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(participantid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::Utf8Array::< i64
                    >::from_slice(regionid_array)) as std::sync::Arc < dyn
                    arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(participant_demand_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(region_demand_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(eligibility_start_interval_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(eligibility_end_interval_array)
                    .to(arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second,
                    None))) as std::sync::Arc < dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(recovery_amount_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
                    std::sync::Arc::new(arrow2::array::PrimitiveArray::from(excluded_energy_array)
                    .to(arrow2::datatypes::DataType::Decimal(18, 8))) as std::sync::Arc <
                    dyn arrow2::array::Array >,
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
        (Some("ASPAYMENTS"), version) if version <= 7_i32 => {
            let d: Vec<BillingAspayments7> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingAspayments7 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("ASRECOVERY"), version) if version <= 8_i32 => {
            let d: Vec<BillingAsrecovery8> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingAsrecovery8 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("CPDATA"), version) if version <= 7_i32 => {
            let d: Vec<BillingCpdata7> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingCpdata7 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("DAYTRK"), version) if version <= 5_i32 => {
            let d: Vec<BillingDaytrk5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingDaytrk5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("FEES"), version) if version <= 5_i32 => {
            let d: Vec<BillingFees5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingFees5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("FINANCIALADJUSTMENTS"), version) if version <= 5_i32 => {
            let d: Vec<BillingFinancialadjustments5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingFinancialadjustments5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("GENDATA"), version) if version <= 5_i32 => {
            let d: Vec<BillingGendata5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingGendata5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("INTERRESIDUES"), version) if version <= 5_i32 => {
            let d: Vec<BillingInterresidues5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingInterresidues5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("INTRARESIDUES"), version) if version <= 5_i32 => {
            let d: Vec<BillingIntraresidues5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingIntraresidues5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("IRAUCSURPLUS"), version) if version <= 5_i32 => {
            let d: Vec<BillingIraucsurplus5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingIraucsurplus5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("IRAUCSURPLUSSUM"), version) if version <= 7_i32 => {
            let d: Vec<BillingIraucsurplussum7> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingIraucsurplussum7 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("IRNSPSURPLUS"), version) if version <= 5_i32 => {
            let d: Vec<BillingIrnspsurplus5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingIrnspsurplus5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("IRNSPSURPLUSSUM"), version) if version <= 6_i32 => {
            let d: Vec<BillingIrnspsurplussum6> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingIrnspsurplussum6 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("IRPARTSURPLUS"), version) if version <= 5_i32 => {
            let d: Vec<BillingIrpartsurplus5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingIrpartsurplus5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("IRPARTSURPLUSSUM"), version) if version <= 7_i32 => {
            let d: Vec<BillingIrpartsurplussum7> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingIrpartsurplussum7 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("PRIORADJUSTMENTS"), version) if version <= 5_i32 => {
            let d: Vec<BillingPrioradjustments5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingPrioradjustments5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("REALLOC"), version) if version <= 5_i32 => {
            let d: Vec<BillingRealloc5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingRealloc5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("REALLOC_DETAIL"), version) if version <= 5_i32 => {
            let d: Vec<BillingReallocDetail5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingReallocDetail5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("REGIONEXPORTS"), version) if version <= 5_i32 => {
            let d: Vec<BillingRegionexports5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingRegionexports5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("REGIONFIGURES"), version) if version <= 6_i32 => {
            let d: Vec<BillingRegionfigures6> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingRegionfigures6 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("REGIONIMPORTS"), version) if version <= 5_i32 => {
            let d: Vec<BillingRegionimports5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingRegionimports5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("RUNTRK"), version) if version <= 5_i32 => {
            let d: Vec<BillingRuntrk5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingRuntrk5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("APC_COMPENSATION"), version) if version <= 2_i32 => {
            let d: Vec<BillingApcCompensation2> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingApcCompensation2 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("APC_RECOVERY"), version) if version <= 2_i32 => {
            let d: Vec<BillingApcRecovery2> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingApcRecovery2 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("BILLING_CO2E_PUBLICATION"), version) if version <= 1_i32 => {
            let d: Vec<BillingBillingCo2ePublication1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingBillingCo2ePublication1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("BILLING_CO2E_PUBLICATION_TRK"), version) if version <= 1_i32 => {
            let d: Vec<BillingBillingCo2ePublicationTrk1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingBillingCo2ePublicationTrk1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("DAILY_ENERGY_SUMMARY"), version) if version <= 1_i32 => {
            let d: Vec<BillingDailyEnergySummary1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingDailyEnergySummary1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("BILLING_DIRECTION_RECON_OTHER"), version) if version <= 1_i32 => {
            let d: Vec<BillingBillingDirectionReconOther1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingBillingDirectionReconOther1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("DIR_FINAL_AMOUNT"), version) if version <= 1_i32 => {
            let d: Vec<BillingDirFinalAmount1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingDirFinalAmount1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("DIR_FINAL_RECOVERY"), version) if version <= 1_i32 => {
            let d: Vec<BillingDirFinalRecovery1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingDirFinalRecovery1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("DIR_PROV_AMOUNT"), version) if version <= 1_i32 => {
            let d: Vec<BillingDirProvAmount1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingDirProvAmount1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("DIR_PROV_RECOVERY"), version) if version <= 1_i32 => {
            let d: Vec<BillingDirProvRecovery1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingDirProvRecovery1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("DIR_RECOVERY_DETAIL"), version) if version <= 1_i32 => {
            let d: Vec<BillingDirRecoveryDetail1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingDirRecoveryDetail1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("EFTSHORTFALL_AMOUNT"), version) if version <= 1_i32 => {
            let d: Vec<BillingEftshortfallAmount1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingEftshortfallAmount1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("EFTSHORTFALL_DETAIL"), version) if version <= 1_i32 => {
            let d: Vec<BillingEftshortfallDetail1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingEftshortfallDetail1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("ENERGY_TRAN_SAPS"), version) if version <= 1_i32 => {
            let d: Vec<BillingEnergyTranSaps1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingEnergyTranSaps1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("GST_DETAIL"), version) if version <= 5_i32 => {
            let d: Vec<BillingGstDetail5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingGstDetail5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("GST_SUMMARY"), version) if version <= 5_i32 => {
            let d: Vec<BillingGstSummary5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingGstSummary5 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("NMAS_TST_PAYMENTS"), version) if version <= 1_i32 => {
            let d: Vec<BillingNmasTstPayments1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingNmasTstPayments1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("NMAS_TST_RECOVERY"), version) if version <= 1_i32 => {
            let d: Vec<BillingNmasTstRecovery1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingNmasTstRecovery1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("NMAS_TST_RECVRY_RBF"), version) if version <= 1_i32 => {
            let d: Vec<BillingNmasTstRecvryRbf1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingNmasTstRecvryRbf1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("NMAS_TST_RECVRY_TRK"), version) if version <= 1_i32 => {
            let d: Vec<BillingNmasTstRecvryTrk1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingNmasTstRecvryTrk1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("SECDEPOSIT_APPLICATION"), version) if version <= 1_i32 => {
            let d: Vec<BillingSecdepositApplication1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingSecdepositApplication1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("SECDEP_INTEREST_PAY"), version) if version <= 1_i32 => {
            let d: Vec<BillingSecdepInterestPay1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingSecdepInterestPay1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("SECDEP_INTEREST_RATE"), version) if version <= 1_i32 => {
            let d: Vec<BillingSecdepInterestRate1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingSecdepInterestRate1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("SUBST_DEMAND"), version) if version <= 1_i32 => {
            let d: Vec<BillingSubstDemand1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingSubstDemand1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("SUBST_RUN_VERSION"), version) if version <= 1_i32 => {
            let d: Vec<BillingSubstRunVersion1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingSubstRunVersion1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("WDR"), version) if version <= 1_i32 => {
            let d: Vec<BillingWdr1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingWdr1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("WDR_DETAIL"), version) if version <= 1_i32 => {
            let d: Vec<BillingWdrDetail1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingWdrDetail1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("RESERVETRADERPAYMENT"), version) if version <= 1_i32 => {
            let d: Vec<BillingReservetraderpayment1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingReservetraderpayment1 @P1, @P2",
                    chunk_size,
                )
                .await?;
        }
        (Some("RESERVETRADERRECOVERY"), version) if version <= 2_i32 => {
            let d: Vec<BillingReservetraderrecovery2> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                    client,
                    file_key,
                    mms_file.header(),
                    &d,
                    "exec mmsdm_proc.InsertBillingReservetraderrecovery2 @P1, @P2",
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
