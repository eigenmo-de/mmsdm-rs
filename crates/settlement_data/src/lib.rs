/// # Summary
///
/// ## DAYTRACK
///  _DAYTRACK identifies the actual settlement run processed for each settlement day. Settlement run is in the column EXPOSTRUNNO. Generally the number of the settlement run used in the latest statement is the maximum number._
///
/// * Data Set Name: Settlements
/// * File Name: Daytrack
/// * Data Version: 6
///
/// # Description
///  DAYTRACK is a public data, and is available to all participants. Source DAYTRACK is populated by the posting of a billing run. Volume Daily billing runs insert one row per day. A non-interim statement has seven records inserted per week. An indicative maximum is 35 records inserted per week.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EXPOSTRUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsDaytrack6 {
    /// Calendar Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Not Used
    pub regionid: Option<String>,
    /// Not Used
    pub exanterunstatus: Option<String>,
    /// Not Used
    pub exanterunno: Option<rust_decimal::Decimal>,
    /// Not Used
    pub expostrunstatus: Option<String>,
    /// Settlement Run No
    pub expostrunno: rust_decimal::Decimal,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Length of a settlement interval, in minutes (was 30 minutes, will be 5 minutes starting the commencement of 5MS rule change date).
    pub settlementintervallength: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsDaytrack6 {
    type PrimaryKey = SettlementsDaytrack6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("DAYTRACK".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> SettlementsDaytrack6PrimaryKey {
        SettlementsDaytrack6PrimaryKey {
            expostrunno: self.expostrunno,
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_daytrack_v6_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsDaytrack6PrimaryKey {
    pub expostrunno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for SettlementsDaytrack6PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsDaytrack6 {
    type Row = SettlementsDaytrack6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.expostrunno == row.expostrunno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsDaytrack6 {
    type PrimaryKey = SettlementsDaytrack6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.expostrunno == key.expostrunno && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for SettlementsDaytrack6PrimaryKey {
    type Row = SettlementsDaytrack6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.expostrunno == row.expostrunno && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsDaytrack6PrimaryKey {
    type PrimaryKey = SettlementsDaytrack6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.expostrunno == key.expostrunno && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsDaytrack6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "exanterunstatus",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "exanterunno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "expostrunstatus",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "expostrunno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "settlementintervallength",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut exanterunstatus_array = Vec::new();
        let mut exanterunno_array = Vec::new();
        let mut expostrunstatus_array = Vec::new();
        let mut expostrunno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut settlementintervallength_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            regionid_array.push(row.regionid);
            exanterunstatus_array.push(row.exanterunstatus);
            exanterunno_array.push({
                row.exanterunno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            expostrunstatus_array.push(row.expostrunstatus);
            expostrunno_array.push({
                let mut val = row.expostrunno;
                val.rescale(0);
                val.mantissa()
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            settlementintervallength_array.push({
                row.settlementintervallength.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(exanterunstatus_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(exanterunno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(expostrunstatus_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(expostrunno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(settlementintervallength_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETCPDATA
///  _SETCPDATA shows meter settlement data for each connection point. This is the key view for retailers to verify energy charges. A regional summary view is also provided. As the view has values for each connection point by period, for each meter data file, it is a very large view._
///
/// * Data Set Name: Settlements
/// * File Name: Cpdata
/// * Data Version: 6
///
/// # Description
///  The Connection point details (in SETCPDATA) are confidential to the participant and host retailer that the connection points relate to. By comparison, the regional data (SETCPDATAREGION) is publically available. Source SETCPDATA updates with each Settlement run.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * MDA
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * TCPID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsCpdata6 {
    /// Calendar Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Connection point identifier
    pub tcpid: String,
    /// Region Identifier
    pub regionid: Option<String>,
    /// Import Gross energy into the pool - MWh
    pub igenergy: Option<rust_decimal::Decimal>,
    /// Export Gross energy from the pool - MWh
    pub xgenergy: Option<rust_decimal::Decimal>,
    /// Import Nett energy into the pool - MWh, plus UFEA if the UFEA amount is positive. When GS commences, this includes the UFEA amount in the settlement runs.
    pub inenergy: Option<rust_decimal::Decimal>,
    /// Export Nett energy from the pool - MWh, plus (UFEA * -1) if the UFEA amount is negative. When GS commences, this includes the UFEA amount in the settlement runs.
    pub xnenergy: Option<rust_decimal::Decimal>,
    /// Import reactive power
    pub ipower: Option<rust_decimal::Decimal>,
    /// Export reactive power
    pub xpower: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess Energy Price
    pub eep: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Connection Point Price = RRP * TLF
    pub cprrp: Option<rust_decimal::Decimal>,
    /// Connection Point Excess Energy Price = EEP * TLF
    pub cpeep: Option<rust_decimal::Decimal>,
    /// Export - Import of Net energy (MWh)
    pub ta: Option<rust_decimal::Decimal>,
    /// settlement amount in $ for trading period
    pub ep: Option<rust_decimal::Decimal>,
    /// Not used
    pub apc: Option<rust_decimal::Decimal>,
    /// Not used
    pub resc: Option<rust_decimal::Decimal>,
    /// Not used
    pub resp: Option<rust_decimal::Decimal>,
    /// Meter Run Number = version number of the meter file
    pub meterrunno: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Not used
    pub hostdistributor: Option<String>,
    /// Metering Data Agent
    pub mda: String,
    /// Adjusted Gross Energy for this Market Customer FRMP and TNI in the Settlements Trading Interval, excluding any UFEA component.
    pub afe: Option<rust_decimal::Decimal>,
    /// Sum of ME- for all NMIs at this Market Customer FRMP and TNI in the Settlements Trading Interval.
    pub dme: Option<rust_decimal::Decimal>,
    /// Share of UFE allocated to this FRMP and TNI in the Settlements Trading Interval.
    pub ufea: Option<rust_decimal::Decimal>,
    /// Adjusted Gross Energy for this Market Customer FRMP and TNI in the Settlements Trading Interval. When GS commences, this includes the UFEA amount in the settlement runs.
    pub age: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsCpdata6 {
    type PrimaryKey = SettlementsCpdata6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("CPDATA".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> SettlementsCpdata6PrimaryKey {
        SettlementsCpdata6PrimaryKey {
            mda: self.mda.clone(),
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
            tcpid: self.tcpid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_cpdata_v6_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsCpdata6PrimaryKey {
    pub mda: String,
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub tcpid: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsCpdata6PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsCpdata6 {
    type Row = SettlementsCpdata6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mda == row.mda
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.tcpid == row.tcpid
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsCpdata6 {
    type PrimaryKey = SettlementsCpdata6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mda == key.mda
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.tcpid == key.tcpid
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsCpdata6PrimaryKey {
    type Row = SettlementsCpdata6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mda == row.mda
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.tcpid == row.tcpid
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsCpdata6PrimaryKey {
    type PrimaryKey = SettlementsCpdata6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mda == key.mda
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.tcpid == key.tcpid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsCpdata6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(10, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(10, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("tcpid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "igenergy",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "xgenergy",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "inenergy",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "xnenergy",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "ipower",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "xpower",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new("rrp", arrow2::datatypes::DataType::Decimal(20, 5), true),
            arrow2::datatypes::Field::new("eep", arrow2::datatypes::DataType::Decimal(16, 6), true),
            arrow2::datatypes::Field::new("tlf", arrow2::datatypes::DataType::Decimal(7, 5), true),
            arrow2::datatypes::Field::new(
                "cprrp",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "cpeep",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new("ta", arrow2::datatypes::DataType::Decimal(16, 6), true),
            arrow2::datatypes::Field::new("ep", arrow2::datatypes::DataType::Decimal(16, 6), true),
            arrow2::datatypes::Field::new("apc", arrow2::datatypes::DataType::Decimal(16, 6), true),
            arrow2::datatypes::Field::new(
                "resc",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "resp",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "meterrunno",
                arrow2::datatypes::DataType::Decimal(10, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "hostdistributor",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("mda", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("afe", arrow2::datatypes::DataType::Decimal(18, 8), true),
            arrow2::datatypes::Field::new("dme", arrow2::datatypes::DataType::Decimal(18, 8), true),
            arrow2::datatypes::Field::new(
                "ufea",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new("age", arrow2::datatypes::DataType::Decimal(18, 8), true),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut tcpid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut igenergy_array = Vec::new();
        let mut xgenergy_array = Vec::new();
        let mut inenergy_array = Vec::new();
        let mut xnenergy_array = Vec::new();
        let mut ipower_array = Vec::new();
        let mut xpower_array = Vec::new();
        let mut rrp_array = Vec::new();
        let mut eep_array = Vec::new();
        let mut tlf_array = Vec::new();
        let mut cprrp_array = Vec::new();
        let mut cpeep_array = Vec::new();
        let mut ta_array = Vec::new();
        let mut ep_array = Vec::new();
        let mut apc_array = Vec::new();
        let mut resc_array = Vec::new();
        let mut resp_array = Vec::new();
        let mut meterrunno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut hostdistributor_array = Vec::new();
        let mut mda_array = Vec::new();
        let mut afe_array = Vec::new();
        let mut dme_array = Vec::new();
        let mut ufea_array = Vec::new();
        let mut age_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            tcpid_array.push(row.tcpid);
            regionid_array.push(row.regionid);
            igenergy_array.push({
                row.igenergy.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            xgenergy_array.push({
                row.xgenergy.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            inenergy_array.push({
                row.inenergy.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            xnenergy_array.push({
                row.xnenergy.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            ipower_array.push({
                row.ipower.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            xpower_array.push({
                row.xpower.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            rrp_array.push({
                row.rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            eep_array.push({
                row.eep.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            tlf_array.push({
                row.tlf.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            cprrp_array.push({
                row.cprrp.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            cpeep_array.push({
                row.cpeep.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            ta_array.push({
                row.ta.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            ep_array.push({
                row.ep.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            apc_array.push({
                row.apc.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            resc_array.push({
                row.resc.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            resp_array.push({
                row.resp.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            meterrunno_array.push({
                row.meterrunno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            hostdistributor_array.push(row.hostdistributor);
            mda_array.push(row.mda);
            afe_array.push({
                row.afe.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            dme_array.push({
                row.dme.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            ufea_array.push({
                row.ufea.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            age_array.push({
                row.age.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(tcpid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(igenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(xgenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(inenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(xnenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ipower_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(xpower_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(20, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(eep_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(tlf_array)
                        .to(arrow2::datatypes::DataType::Decimal(7, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cprrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cpeep_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ta_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ep_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(apc_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(resc_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(resp_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(meterrunno_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(hostdistributor_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(mda_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(afe_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(dme_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ufea_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(age_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETCPDATAREGION
///  _SETCPDATAREGION sets out summary meter settlement data for each region._
///
/// * Data Set Name: Settlements
/// * File Name: Cpdataregion
/// * Data Version: 5
///
/// # Description
///  SETCPDATAREGION data is public, so is available to all participants. Source SETCPDATAREGION is a summary based on grouping on SETCPDATA and is updated with each settlement run.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsCpdataregion5 {
    /// Calendar Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Import Gross energy into the pool - MWh
    pub sumigenergy: Option<rust_decimal::Decimal>,
    /// Export Gross energy from the pool - MWh
    pub sumxgenergy: Option<rust_decimal::Decimal>,
    /// Import Nett energy into the pool - MWh
    pub suminenergy: Option<rust_decimal::Decimal>,
    /// Export Nett energy from the pool - MWh
    pub sumxnenergy: Option<rust_decimal::Decimal>,
    /// Not used
    pub sumipower: Option<rust_decimal::Decimal>,
    /// Not used
    pub sumxpower: Option<rust_decimal::Decimal>,
    /// current system date, to enable automatic replication
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Sum of energy price across the region
    pub sumep: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsCpdataregion5 {
    type PrimaryKey = SettlementsCpdataregion5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("CPDATAREGION".into()),
            version: 5,
        }
    }

    fn primary_key(&self) -> SettlementsCpdataregion5PrimaryKey {
        SettlementsCpdataregion5PrimaryKey {
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_cpdataregion_v5_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsCpdataregion5PrimaryKey {
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsCpdataregion5PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsCpdataregion5 {
    type Row = SettlementsCpdataregion5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsCpdataregion5 {
    type PrimaryKey = SettlementsCpdataregion5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsCpdataregion5PrimaryKey {
    type Row = SettlementsCpdataregion5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsCpdataregion5PrimaryKey {
    type PrimaryKey = SettlementsCpdataregion5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsCpdataregion5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(22, 10),
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(22, 10),
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "sumigenergy",
                arrow2::datatypes::DataType::Decimal(27, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "sumxgenergy",
                arrow2::datatypes::DataType::Decimal(27, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "suminenergy",
                arrow2::datatypes::DataType::Decimal(27, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "sumxnenergy",
                arrow2::datatypes::DataType::Decimal(27, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "sumipower",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "sumxpower",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "sumep",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut sumigenergy_array = Vec::new();
        let mut sumxgenergy_array = Vec::new();
        let mut suminenergy_array = Vec::new();
        let mut sumxnenergy_array = Vec::new();
        let mut sumipower_array = Vec::new();
        let mut sumxpower_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut sumep_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(10);
                val.mantissa()
            });
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(10);
                val.mantissa()
            });
            regionid_array.push(row.regionid);
            sumigenergy_array.push({
                row.sumigenergy.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            sumxgenergy_array.push({
                row.sumxgenergy.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            suminenergy_array.push({
                row.suminenergy.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            sumxnenergy_array.push({
                row.sumxnenergy.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            sumipower_array.push({
                row.sumipower.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            sumxpower_array.push({
                row.sumxpower.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            sumep_array.push({
                row.sumep.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 10)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 10)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(sumigenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(27, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(sumxgenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(27, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(suminenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(27, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(sumxnenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(27, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(sumipower_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(sumxpower_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(sumep_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETFCASREGIONRECOVERY
///  _SETFCASREGIONRECOVERY shows FCAS Regional Recovery Data against each Trading Interval._
///
/// * Data Set Name: Settlements
/// * File Name: Fcasregionrecovery
/// * Data Version: 5
///
/// # Description
///  SETFCASREGIONRECOVERY contains public data and is available to all participants. Source SETFCASREGIONRECOVERY updates with each settlements run. Volume Approximately 10,000 rows per day
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsFcasregionrecovery5 {
    /// Settlement Date of trading interval
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// FCAS Service Type
    pub bidtype: String,
    /// RegionID
    pub regionid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Generator Regional Energy Amount
    pub generatorregionenergy: Option<rust_decimal::Decimal>,
    /// Customer Region Energy Amount
    pub customerregionenergy: Option<rust_decimal::Decimal>,
    /// The NEM Regional Recovery Amount for FCAS
    pub regionrecovery: Option<rust_decimal::Decimal>,
    /// Last Date record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for SettlementsFcasregionrecovery5 {
    type PrimaryKey = SettlementsFcasregionrecovery5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("FCASREGIONRECOVERY".into()),
            version: 5,
        }
    }

    fn primary_key(&self) -> SettlementsFcasregionrecovery5PrimaryKey {
        SettlementsFcasregionrecovery5PrimaryKey {
            bidtype: self.bidtype.clone(),
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_fcasregionrecovery_v5_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsFcasregionrecovery5PrimaryKey {
    pub bidtype: String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsFcasregionrecovery5PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsFcasregionrecovery5 {
    type Row = SettlementsFcasregionrecovery5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsFcasregionrecovery5 {
    type PrimaryKey = SettlementsFcasregionrecovery5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsFcasregionrecovery5PrimaryKey {
    type Row = SettlementsFcasregionrecovery5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsFcasregionrecovery5PrimaryKey {
    type PrimaryKey = SettlementsFcasregionrecovery5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsFcasregionrecovery5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("bidtype", arrow2::datatypes::DataType::LargeUtf8, false),
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
            arrow2::datatypes::Field::new(
                "generatorregionenergy",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "customerregionenergy",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "regionrecovery",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut bidtype_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut generatorregionenergy_array = Vec::new();
        let mut customerregionenergy_array = Vec::new();
        let mut regionrecovery_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            bidtype_array.push(row.bidtype);
            regionid_array.push(row.regionid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            generatorregionenergy_array.push({
                row.generatorregionenergy.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            customerregionenergy_array.push({
                row.customerregionenergy.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            regionrecovery_array.push({
                row.regionrecovery.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bidtype_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(generatorregionenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(customerregionenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(regionrecovery_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETGENDATA
///  _SETGENDATA shows meter settlement data for each generation meter point. A regional summary is also provided._
///
/// * Data Set Name: Settlements
/// * File Name: Gendata
/// * Data Version: 6
///
/// # Description
///  SETGENDATA shows generator meter details, and SETGENDATA data is confidential to the participant. By comparison, the regional summary (SETGENDATAREGION) is public data. Source SETGENDATA updates with each Settlement run.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * DUID
/// * GENSETID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * STATIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsGendata6 {
    /// Calendar Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: Option<String>,
    /// Station Identifier
    pub stationid: String,
    /// Dispatchable Unit identifier
    pub duid: String,
    /// Physical unit identifier
    pub gensetid: String,
    /// Region Identifier
    pub regionid: String,
    /// Generated energy
    pub genergy: Option<rust_decimal::Decimal>,
    /// Purchased Energy
    pub aenergy: Option<rust_decimal::Decimal>,
    /// Not used
    pub gpower: Option<rust_decimal::Decimal>,
    /// Not used
    pub apower: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Excess Energy Price
    pub eep: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Connection Point Price = RRP * TLF
    pub cprrp: Option<rust_decimal::Decimal>,
    /// Connection Point Excess Energy Price = EEP * TLF
    pub cpeep: Option<rust_decimal::Decimal>,
    /// Generated energy
    pub netenergy: Option<rust_decimal::Decimal>,
    /// Cost of net energy $
    pub energycost: Option<rust_decimal::Decimal>,
    /// Cost of excess energy $
    pub excessenergycost: Option<rust_decimal::Decimal>,
    /// Administered Price Compensation
    pub apc: Option<rust_decimal::Decimal>,
    /// Not used
    pub resc: Option<rust_decimal::Decimal>,
    /// Not used
    pub resp: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Export Energy (Generator Purchases) (MWh)
    pub expenergy: Option<rust_decimal::Decimal>,
    /// Export Energy Cost ($)
    pub expenergycost: Option<rust_decimal::Decimal>,
    /// Identifier of the meter run used in this settlement calculation
    pub meterrunno: Option<rust_decimal::Decimal>,
    /// Metering Data Agent
    pub mda: Option<String>,
    /// Secondary Transmission Loss Factor
    pub secondary_tlf: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsGendata6 {
    type PrimaryKey = SettlementsGendata6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("GENDATA".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> SettlementsGendata6PrimaryKey {
        SettlementsGendata6PrimaryKey {
            duid: self.duid.clone(),
            gensetid: self.gensetid.clone(),
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
            stationid: self.stationid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_gendata_v6_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsGendata6PrimaryKey {
    pub duid: String,
    pub gensetid: String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub stationid: String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsGendata6PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsGendata6 {
    type Row = SettlementsGendata6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.gensetid == row.gensetid
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.stationid == row.stationid
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsGendata6 {
    type PrimaryKey = SettlementsGendata6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.gensetid == key.gensetid
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.stationid == key.stationid
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsGendata6PrimaryKey {
    type Row = SettlementsGendata6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.gensetid == row.gensetid
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.stationid == row.stationid
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsGendata6PrimaryKey {
    type PrimaryKey = SettlementsGendata6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.gensetid == key.gensetid
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.stationid == key.stationid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsGendata6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(10, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(10, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "stationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "gensetid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "genergy",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "aenergy",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "gpower",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "apower",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new("rrp", arrow2::datatypes::DataType::Decimal(20, 5), true),
            arrow2::datatypes::Field::new("eep", arrow2::datatypes::DataType::Decimal(16, 6), true),
            arrow2::datatypes::Field::new("tlf", arrow2::datatypes::DataType::Decimal(7, 5), true),
            arrow2::datatypes::Field::new(
                "cprrp",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "cpeep",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "netenergy",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "energycost",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "excessenergycost",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new("apc", arrow2::datatypes::DataType::Decimal(16, 6), true),
            arrow2::datatypes::Field::new(
                "resc",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "resp",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "expenergy",
                arrow2::datatypes::DataType::Decimal(15, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "expenergycost",
                arrow2::datatypes::DataType::Decimal(15, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "meterrunno",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new("mda", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "secondary_tlf",
                arrow2::datatypes::DataType::Decimal(7, 5),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut stationid_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut gensetid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut genergy_array = Vec::new();
        let mut aenergy_array = Vec::new();
        let mut gpower_array = Vec::new();
        let mut apower_array = Vec::new();
        let mut rrp_array = Vec::new();
        let mut eep_array = Vec::new();
        let mut tlf_array = Vec::new();
        let mut cprrp_array = Vec::new();
        let mut cpeep_array = Vec::new();
        let mut netenergy_array = Vec::new();
        let mut energycost_array = Vec::new();
        let mut excessenergycost_array = Vec::new();
        let mut apc_array = Vec::new();
        let mut resc_array = Vec::new();
        let mut resp_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut expenergy_array = Vec::new();
        let mut expenergycost_array = Vec::new();
        let mut meterrunno_array = Vec::new();
        let mut mda_array = Vec::new();
        let mut secondary_tlf_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            stationid_array.push(row.stationid);
            duid_array.push(row.duid);
            gensetid_array.push(row.gensetid);
            regionid_array.push(row.regionid);
            genergy_array.push({
                row.genergy.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            aenergy_array.push({
                row.aenergy.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            gpower_array.push({
                row.gpower.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            apower_array.push({
                row.apower.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            rrp_array.push({
                row.rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            eep_array.push({
                row.eep.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            tlf_array.push({
                row.tlf.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            cprrp_array.push({
                row.cprrp.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            cpeep_array.push({
                row.cpeep.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            netenergy_array.push({
                row.netenergy.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            energycost_array.push({
                row.energycost.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            excessenergycost_array.push({
                row.excessenergycost.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            apc_array.push({
                row.apc.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            resc_array.push({
                row.resc.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            resp_array.push({
                row.resp.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            expenergy_array.push({
                row.expenergy.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            expenergycost_array.push({
                row.expenergycost.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            meterrunno_array.push({
                row.meterrunno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            mda_array.push(row.mda);
            secondary_tlf_array.push({
                row.secondary_tlf.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(participantid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(stationid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(gensetid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(genergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(aenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(gpower_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(apower_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(20, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(eep_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(tlf_array)
                        .to(arrow2::datatypes::DataType::Decimal(7, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cprrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cpeep_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(netenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(energycost_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(excessenergycost_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(apc_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(resc_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(resp_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(expenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(expenergycost_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(meterrunno_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(mda_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(secondary_tlf_array)
                        .to(arrow2::datatypes::DataType::Decimal(7, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETGENDATAREGION
///  _SETGENDATAREGION sets out summary settlement data for generation within the specified region._
///
/// * Data Set Name: Settlements
/// * File Name: Gendataregion
/// * Data Version: 5
///
/// # Description
///  SETGENDATAREGION shows the regional summary. SETGENDATAREGION is public data. Source SETGENDATAREGION updates with each Settlement run.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsGendataregion5 {
    /// Calendar Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Generated energy - Not used in MMS Data Model
    pub genergy: Option<rust_decimal::Decimal>,
    /// Purchased Energy - Not used in MMS Data Model
    pub aenergy: Option<rust_decimal::Decimal>,
    /// Not used in MMS Data Model
    pub gpower: Option<rust_decimal::Decimal>,
    /// Not used in MMS Data Model
    pub apower: Option<rust_decimal::Decimal>,
    /// Net energy MW/hours
    pub netenergy: Option<rust_decimal::Decimal>,
    /// Cost of net energy $
    pub energycost: Option<rust_decimal::Decimal>,
    /// Cost of excess energy $
    pub excessenergycost: Option<rust_decimal::Decimal>,
    /// Export Energy (Generator Purchases)
    pub expenergy: Option<rust_decimal::Decimal>,
    /// Export Energy Cost
    pub expenergycost: Option<rust_decimal::Decimal>,
    /// current system date, to enable automatic replication
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for SettlementsGendataregion5 {
    type PrimaryKey = SettlementsGendataregion5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("GENDATAREGION".into()),
            version: 5,
        }
    }

    fn primary_key(&self) -> SettlementsGendataregion5PrimaryKey {
        SettlementsGendataregion5PrimaryKey {
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_gendataregion_v5_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsGendataregion5PrimaryKey {
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsGendataregion5PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsGendataregion5 {
    type Row = SettlementsGendataregion5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsGendataregion5 {
    type PrimaryKey = SettlementsGendataregion5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsGendataregion5PrimaryKey {
    type Row = SettlementsGendataregion5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsGendataregion5PrimaryKey {
    type PrimaryKey = SettlementsGendataregion5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsGendataregion5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(22, 10),
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(22, 10),
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "genergy",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "aenergy",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "gpower",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "apower",
                arrow2::datatypes::DataType::Decimal(22, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "netenergy",
                arrow2::datatypes::DataType::Decimal(27, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "energycost",
                arrow2::datatypes::DataType::Decimal(27, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "excessenergycost",
                arrow2::datatypes::DataType::Decimal(27, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "expenergy",
                arrow2::datatypes::DataType::Decimal(27, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "expenergycost",
                arrow2::datatypes::DataType::Decimal(27, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut genergy_array = Vec::new();
        let mut aenergy_array = Vec::new();
        let mut gpower_array = Vec::new();
        let mut apower_array = Vec::new();
        let mut netenergy_array = Vec::new();
        let mut energycost_array = Vec::new();
        let mut excessenergycost_array = Vec::new();
        let mut expenergy_array = Vec::new();
        let mut expenergycost_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(10);
                val.mantissa()
            });
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(10);
                val.mantissa()
            });
            regionid_array.push(row.regionid);
            genergy_array.push({
                row.genergy.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            aenergy_array.push({
                row.aenergy.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            gpower_array.push({
                row.gpower.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            apower_array.push({
                row.apower.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            netenergy_array.push({
                row.netenergy.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            energycost_array.push({
                row.energycost.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            excessenergycost_array.push({
                row.excessenergycost.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            expenergy_array.push({
                row.expenergy.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            expenergycost_array.push({
                row.expenergycost.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 10)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 10)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(genergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(aenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(gpower_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(apower_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(netenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(27, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(energycost_array)
                        .to(arrow2::datatypes::DataType::Decimal(27, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(excessenergycost_array)
                        .to(arrow2::datatypes::DataType::Decimal(27, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(expenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(27, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(expenergycost_array)
                        .to(arrow2::datatypes::DataType::Decimal(27, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETINTRAREGIONRESIDUES
///  _&nbsp; _
///
/// * Data Set Name: Settlements
/// * File Name: Intraregionresidues
/// * Data Version: 5
///
/// # Description
///  SETINTRAREGIONRESIDUES data is public to all participants. Source SETINTRAREGIONRESIDUES updates with each settlement run. Note The relationship between the data columns for each key is expressed in the following formula: EP + EC + (EXP * RRP) = IRSS
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
pub struct SettlementsIntraregionresidues5 {
    /// Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub runno: i64,
    /// Settlements Trading Interval.
    pub periodid: i64,
    /// Region Identifier
    pub regionid: String,
    /// Energy payments to generators
    pub ep: Option<rust_decimal::Decimal>,
    /// Energy purchased by customers
    pub ec: Option<rust_decimal::Decimal>,
    /// Regional price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Net import in MWh into the region calculated at the regional reference node (export is negative)
    pub exp: Option<rust_decimal::Decimal>,
    /// Intra-regional surplus (a negative sign indicates surplus, and a positive sign indicates a deficiency)
    pub irss: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for SettlementsIntraregionresidues5 {
    type PrimaryKey = SettlementsIntraregionresidues5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("INTRAREGIONRESIDUES".into()),
            version: 5,
        }
    }

    fn primary_key(&self) -> SettlementsIntraregionresidues5PrimaryKey {
        SettlementsIntraregionresidues5PrimaryKey {
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_intraregionresidues_v5_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsIntraregionresidues5PrimaryKey {
    pub periodid: i64,
    pub regionid: String,
    pub runno: i64,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for SettlementsIntraregionresidues5PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsIntraregionresidues5 {
    type Row = SettlementsIntraregionresidues5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIntraregionresidues5 {
    type PrimaryKey = SettlementsIntraregionresidues5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for SettlementsIntraregionresidues5PrimaryKey {
    type Row = SettlementsIntraregionresidues5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIntraregionresidues5PrimaryKey {
    type PrimaryKey = SettlementsIntraregionresidues5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsIntraregionresidues5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new("runno", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("ep", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new("ec", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new("rrp", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new("exp", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new(
                "irss",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut ep_array = Vec::new();
        let mut ec_array = Vec::new();
        let mut rrp_array = Vec::new();
        let mut exp_array = Vec::new();
        let mut irss_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            runno_array.push(row.runno);
            periodid_array.push(row.periodid);
            regionid_array.push(row.regionid);
            ep_array.push({
                row.ep.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            ec_array.push({
                row.ec.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp_array.push({
                row.rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            exp_array.push({
                row.exp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            irss_array.push({
                row.irss.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(runno_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ep_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ec_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(exp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(irss_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETIRAUCSURPLUS
///  _This view supports the Settlements Residue Auction, by holding the NSP participant allocations of IRSurplus arising as a result of the unsold units for a quarter._
///
/// * Data Set Name: Settlements
/// * File Name: Iraucsurplus
/// * Data Version: 6
///
/// # Description
///  SETIRAUCSURPLUS data is confidential to the relevant participant. Source SETIRAUCSURPLUS updates with each settlement run. Volume SETIRAUCSURPLUS contains a maximum of 10 million records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIraucsurplus6 {
    /// Calendar Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    pub contractid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Contracted Interconnector identifier
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Total value of surplus before allocation
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// Percentage allocated to participant
    pub contractallocation: Option<rust_decimal::Decimal>,
    /// Amount NSP is paid for Inter/intra-Regional surplus energy produced
    pub surplusvalue: Option<rust_decimal::Decimal>,
    /// Date and time this record was last modified
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsIraucsurplus6 {
    type PrimaryKey = SettlementsIraucsurplus6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("IRAUCSURPLUS".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> SettlementsIraucsurplus6PrimaryKey {
        SettlementsIraucsurplus6PrimaryKey {
            contractid: self.contractid.clone(),
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
            settlementrunno: self.settlementrunno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_iraucsurplus_v6_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsIraucsurplus6PrimaryKey {
    pub contractid: String,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsIraucsurplus6PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsIraucsurplus6 {
    type Row = SettlementsIraucsurplus6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIraucsurplus6 {
    type PrimaryKey = SettlementsIraucsurplus6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsIraucsurplus6PrimaryKey {
    type Row = SettlementsIraucsurplus6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIraucsurplus6PrimaryKey {
    type PrimaryKey = SettlementsIraucsurplus6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsIraucsurplus6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "settlementrunno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "fromregionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "totalsurplus",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "contractallocation",
                arrow2::datatypes::DataType::Decimal(8, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "surplusvalue",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "csp_derogation_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "unadjusted_irsr",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut settlementrunno_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut totalsurplus_array = Vec::new();
        let mut contractallocation_array = Vec::new();
        let mut surplusvalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut csp_derogation_amount_array = Vec::new();
        let mut unadjusted_irsr_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            settlementrunno_array.push({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
            contractid_array.push(row.contractid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            totalsurplus_array.push({
                row.totalsurplus.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            contractallocation_array.push({
                row.contractallocation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            surplusvalue_array.push({
                row.surplusvalue.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            csp_derogation_amount_array.push({
                row.csp_derogation_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            unadjusted_irsr_array.push({
                row.unadjusted_irsr.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementrunno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    contractid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    fromregionid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalsurplus_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(contractallocation_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(surplusvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(csp_derogation_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(unadjusted_irsr_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETIRNSPSURPLUS
///  _This view supports the Settlements Residue Auction, by showing the TNSP participant allocations of  Interconnector Residue (IR) Surplus (i.e. derogated amounts) arising as a result of the sold units for a quarter._
///
/// * Data Set Name: Settlements
/// * File Name: Irnspsurplus
/// * Data Version: 6
///
/// # Description
///  SETIRNSPSURPLUS data is confidential to the relevant participant. Source SETIRNSPSURPLUS updates with each settlement run. Volume SETIRNSPSURPLUS contains a maximum of 10 million records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIrnspsurplus6 {
    /// Settlement date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// SRA Contract unique identifier
    pub contractid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Participant unique identifier
    pub participantid: String,
    /// Identifier of Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Total value of surplus
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// Percentage of total surplus allocated to participant
    pub contractallocation: Option<rust_decimal::Decimal>,
    /// Amount NSP is paid for Inter/intra-Regional surplus energy produced by the participant
    pub surplusvalue: Option<rust_decimal::Decimal>,
    /// Date and time this record was last modified
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsIrnspsurplus6 {
    type PrimaryKey = SettlementsIrnspsurplus6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("IRNSPSURPLUS".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> SettlementsIrnspsurplus6PrimaryKey {
        SettlementsIrnspsurplus6PrimaryKey {
            contractid: self.contractid.clone(),
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
            settlementrunno: self.settlementrunno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_irnspsurplus_v6_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsIrnspsurplus6PrimaryKey {
    pub contractid: String,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsIrnspsurplus6PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsIrnspsurplus6 {
    type Row = SettlementsIrnspsurplus6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIrnspsurplus6 {
    type PrimaryKey = SettlementsIrnspsurplus6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsIrnspsurplus6PrimaryKey {
    type Row = SettlementsIrnspsurplus6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIrnspsurplus6PrimaryKey {
    type PrimaryKey = SettlementsIrnspsurplus6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsIrnspsurplus6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "settlementrunno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "fromregionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "totalsurplus",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "contractallocation",
                arrow2::datatypes::DataType::Decimal(8, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "surplusvalue",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "csp_derogation_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "unadjusted_irsr",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut settlementrunno_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut totalsurplus_array = Vec::new();
        let mut contractallocation_array = Vec::new();
        let mut surplusvalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut csp_derogation_amount_array = Vec::new();
        let mut unadjusted_irsr_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            settlementrunno_array.push({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
            contractid_array.push(row.contractid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            totalsurplus_array.push({
                row.totalsurplus.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            contractallocation_array.push({
                row.contractallocation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            surplusvalue_array.push({
                row.surplusvalue.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            csp_derogation_amount_array.push({
                row.csp_derogation_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            unadjusted_irsr_array.push({
                row.unadjusted_irsr.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementrunno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    contractid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    fromregionid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalsurplus_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(contractallocation_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(surplusvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(csp_derogation_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(unadjusted_irsr_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETIRPARTSURPLUS
///  _This view supports the Settlements Residue Auction, holding the participant allocations of IRSurplus._
///
/// * Data Set Name: Settlements
/// * File Name: Irpartsurplus
/// * Data Version: 6
///
/// # Description
///  SETIRPARTSURPLUS data is confidential to each participant. Source SETIRPARTSURPLUS updates with each settlement run. Volume SETIRPARTSURPLUS contains a maximum of 20 million records per year.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * FROMREGIONID
/// * INTERCONNECTORID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIrpartsurplus6 {
    /// Settlement date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// Ancillary Service Contract
    pub contractid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Participant unique identifier
    pub participantid: String,
    /// Identifier of the Contracted Interconnector
    pub interconnectorid: String,
    /// Nominated source region for Interconnector
    pub fromregionid: String,
    /// Total value of surplus before allocation
    pub totalsurplus: Option<rust_decimal::Decimal>,
    /// Allocated percentage to participant
    pub contractallocation: Option<rust_decimal::Decimal>,
    /// Amount NSP is paid for Inter/intra-Regional surplus energy produced
    pub surplusvalue: Option<rust_decimal::Decimal>,
    /// Date and time this record was last updated
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsIrpartsurplus6 {
    type PrimaryKey = SettlementsIrpartsurplus6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("IRPARTSURPLUS".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> SettlementsIrpartsurplus6PrimaryKey {
        SettlementsIrpartsurplus6PrimaryKey {
            contractid: self.contractid.clone(),
            fromregionid: self.fromregionid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
            settlementrunno: self.settlementrunno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_irpartsurplus_v6_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsIrpartsurplus6PrimaryKey {
    pub contractid: String,
    pub fromregionid: String,
    pub interconnectorid: String,
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsIrpartsurplus6PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsIrpartsurplus6 {
    type Row = SettlementsIrpartsurplus6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIrpartsurplus6 {
    type PrimaryKey = SettlementsIrpartsurplus6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsIrpartsurplus6PrimaryKey {
    type Row = SettlementsIrpartsurplus6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.fromregionid == row.fromregionid
            && self.interconnectorid == row.interconnectorid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIrpartsurplus6PrimaryKey {
    type PrimaryKey = SettlementsIrpartsurplus6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.fromregionid == key.fromregionid
            && self.interconnectorid == key.interconnectorid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsIrpartsurplus6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "settlementrunno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "fromregionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "totalsurplus",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "contractallocation",
                arrow2::datatypes::DataType::Decimal(8, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "surplusvalue",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "csp_derogation_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "unadjusted_irsr",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut settlementrunno_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut fromregionid_array = Vec::new();
        let mut totalsurplus_array = Vec::new();
        let mut contractallocation_array = Vec::new();
        let mut surplusvalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut csp_derogation_amount_array = Vec::new();
        let mut unadjusted_irsr_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            settlementrunno_array.push({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
            contractid_array.push(row.contractid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            interconnectorid_array.push(row.interconnectorid);
            fromregionid_array.push(row.fromregionid);
            totalsurplus_array.push({
                row.totalsurplus.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            contractallocation_array.push({
                row.contractallocation.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            surplusvalue_array.push({
                row.surplusvalue.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            csp_derogation_amount_array.push({
                row.csp_derogation_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            unadjusted_irsr_array.push({
                row.unadjusted_irsr.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementrunno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    contractid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    fromregionid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(totalsurplus_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(contractallocation_array)
                        .to(arrow2::datatypes::DataType::Decimal(8, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(surplusvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(csp_derogation_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(unadjusted_irsr_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETIRSURPLUS
///  _SETIRSURPLUS records the interregional residue calculation for each interconnector and each side of the interconnector._
///
/// * Data Set Name: Settlements
/// * File Name: Irsurplus
/// * Data Version: 6
///
/// # Description
///  SETIRSURPLUS data is public, so is available to all participants. Source SETIRSURPLUS updates once a day at 8am. Note MWFLOW and LOSSFACTOR are now both calculated as MWh (energy) values for the half hour, and not MW (average demand) values. By way of clarification, the MWFLOW value is derived from half-hour revenue class metering, adjusted by a fixed fraction of the LOSSFACTOR value. The LOSSFACTOR value is taken to be exactly half of the MWLOSSES value in the TRADINGINTERCONNECT table. The METEREDMWFLOW field in the TRADINGINTERCONNECT table contains averaged SCADA metering demand values available in “real time”, whereas the MWFLOW field in the SETIRSURPLUS table contains settlement energy metering values available only after a settlement run is posted.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INTERCONNECTORID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsIrsurplus6 {
    /// Settlement date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Interconnector
    pub interconnectorid: String,
    /// Side of interconnector
    pub regionid: String,
    /// Net flow at the regional node (MWh), including losses
    pub mwflow: Option<rust_decimal::Decimal>,
    /// MW losses along interconnector NOTE: This is not a loss factor, but a loss figure expressed in MWH
    pub lossfactor: Option<rust_decimal::Decimal>,
    /// Amount of surplus in $
    pub surplusvalue: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The CSP derogation amount applied as an adjustment to SRA
    pub csp_derogation_amount: Option<rust_decimal::Decimal>,
    /// The SRA amount unadjusted by CSP
    pub unadjusted_irsr: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsIrsurplus6 {
    type PrimaryKey = SettlementsIrsurplus6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("IRSURPLUS".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> SettlementsIrsurplus6PrimaryKey {
        SettlementsIrsurplus6PrimaryKey {
            interconnectorid: self.interconnectorid.clone(),
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
            settlementrunno: self.settlementrunno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_irsurplus_v6_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsIrsurplus6PrimaryKey {
    pub interconnectorid: String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsIrsurplus6PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsIrsurplus6 {
    type Row = SettlementsIrsurplus6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interconnectorid == row.interconnectorid
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIrsurplus6 {
    type PrimaryKey = SettlementsIrsurplus6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsIrsurplus6PrimaryKey {
    type Row = SettlementsIrsurplus6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.interconnectorid == row.interconnectorid
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsIrsurplus6PrimaryKey {
    type PrimaryKey = SettlementsIrsurplus6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.interconnectorid == key.interconnectorid
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsIrsurplus6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "settlementrunno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "mwflow",
                arrow2::datatypes::DataType::Decimal(15, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lossfactor",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "surplusvalue",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "csp_derogation_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "unadjusted_irsr",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut settlementrunno_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut interconnectorid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut mwflow_array = Vec::new();
        let mut lossfactor_array = Vec::new();
        let mut surplusvalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut csp_derogation_amount_array = Vec::new();
        let mut unadjusted_irsr_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            settlementrunno_array.push({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            interconnectorid_array.push(row.interconnectorid);
            regionid_array.push(row.regionid);
            mwflow_array.push({
                row.mwflow.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lossfactor_array.push({
                row.lossfactor.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            surplusvalue_array.push({
                row.surplusvalue.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            csp_derogation_amount_array.push({
                row.csp_derogation_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            unadjusted_irsr_array.push({
                row.unadjusted_irsr.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementrunno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mwflow_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lossfactor_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(surplusvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(csp_derogation_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(unadjusted_irsr_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETLOCALAREAENERGY
///  _SETLOCALAREAENERGY shows the UFE, AGE and associated values for each local area and trading interval in a settlement run._
///
/// * Data Set Name: Settlements
/// * File Name: Localareaenergy
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * LOCALAREAID
/// * PERIODID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLocalareaenergy1 {
    /// Settlement date of the settlement run
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number of the settlement run
    pub settlementrunno: rust_decimal::Decimal,
    /// Unique identifier for the local area
    pub localareaid: String,
    /// Settlement Trading Interval
    pub periodid: rust_decimal::Decimal,
    /// Total unaccounted-for energy for the local area in this trading interval, in MWh
    pub ufe: Option<rust_decimal::Decimal>,
    /// DDME component of UFE for the local area in this trading interval, in MWh.
    pub ddme: Option<rust_decimal::Decimal>,
    /// TME component of UFE for the local area in this trading interval, in MWh.
    pub tme: Option<rust_decimal::Decimal>,
    /// ADME component of UFE for the local area in this trading interval, in MWh.
    pub adme: Option<rust_decimal::Decimal>,
    /// The sum of all DME amounts for each Market Customer FRMP and TNI in the local area, in this trading interval.
    pub admela: Option<rust_decimal::Decimal>,
    /// Last changed date time for the record
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for SettlementsLocalareaenergy1 {
    type PrimaryKey = SettlementsLocalareaenergy1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("LOCALAREAENERGY".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementsLocalareaenergy1PrimaryKey {
        SettlementsLocalareaenergy1PrimaryKey {
            localareaid: self.localareaid.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
            settlementrunno: self.settlementrunno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_localareaenergy_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsLocalareaenergy1PrimaryKey {
    pub localareaid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsLocalareaenergy1PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsLocalareaenergy1 {
    type Row = SettlementsLocalareaenergy1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.localareaid == row.localareaid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsLocalareaenergy1 {
    type PrimaryKey = SettlementsLocalareaenergy1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.localareaid == key.localareaid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsLocalareaenergy1PrimaryKey {
    type Row = SettlementsLocalareaenergy1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.localareaid == row.localareaid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsLocalareaenergy1PrimaryKey {
    type PrimaryKey = SettlementsLocalareaenergy1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.localareaid == key.localareaid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsLocalareaenergy1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "settlementrunno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "localareaid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("ufe", arrow2::datatypes::DataType::Decimal(18, 8), true),
            arrow2::datatypes::Field::new(
                "ddme",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new("tme", arrow2::datatypes::DataType::Decimal(18, 8), true),
            arrow2::datatypes::Field::new(
                "adme",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "admela",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut settlementrunno_array = Vec::new();
        let mut localareaid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut ufe_array = Vec::new();
        let mut ddme_array = Vec::new();
        let mut tme_array = Vec::new();
        let mut adme_array = Vec::new();
        let mut admela_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            settlementrunno_array.push({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
            localareaid_array.push(row.localareaid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            ufe_array.push({
                row.ufe.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            ddme_array.push({
                row.ddme.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            tme_array.push({
                row.tme.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            adme_array.push({
                row.adme.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            admela_array.push({
                row.admela.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementrunno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    localareaid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ufe_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ddme_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(tme_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(adme_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(admela_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETLOCALAREATNI
///  _SETLOCALAREATNI shows the list of TNIs constituent to a local area in a settlement run. _
///
/// * Data Set Name: Settlements
/// * File Name: Localareatni
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * LOCALAREAID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
/// * TNI
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLocalareatni1 {
    /// Settlement date of the settlement run
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number of the settlement run
    pub settlementrunno: rust_decimal::Decimal,
    /// Unique identifier for the local area
    pub localareaid: String,
    /// Unique identifier for a TNI constituent to the local area as at the settlement run
    pub tni: String,
    /// Last changed date time for the record
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for SettlementsLocalareatni1 {
    type PrimaryKey = SettlementsLocalareatni1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("LOCALAREATNI".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementsLocalareatni1PrimaryKey {
        SettlementsLocalareatni1PrimaryKey {
            localareaid: self.localareaid.clone(),
            settlementdate: self.settlementdate,
            settlementrunno: self.settlementrunno,
            tni: self.tni.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_localareatni_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsLocalareatni1PrimaryKey {
    pub localareaid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
    pub tni: String,
}
impl mmsdm_core::PrimaryKey for SettlementsLocalareatni1PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsLocalareatni1 {
    type Row = SettlementsLocalareatni1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.localareaid == row.localareaid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
            && self.tni == row.tni
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsLocalareatni1 {
    type PrimaryKey = SettlementsLocalareatni1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.localareaid == key.localareaid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
            && self.tni == key.tni
    }
}
impl mmsdm_core::CompareWithRow for SettlementsLocalareatni1PrimaryKey {
    type Row = SettlementsLocalareatni1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.localareaid == row.localareaid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
            && self.tni == row.tni
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsLocalareatni1PrimaryKey {
    type PrimaryKey = SettlementsLocalareatni1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.localareaid == key.localareaid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
            && self.tni == key.tni
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsLocalareatni1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "settlementrunno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "localareaid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("tni", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut settlementrunno_array = Vec::new();
        let mut localareaid_array = Vec::new();
        let mut tni_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            settlementrunno_array.push({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
            localareaid_array.push(row.localareaid);
            tni_array.push(row.tni);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementrunno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    localareaid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(tni_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETLSHEDPAYMENT
///  _SETLSHEDPAYMENT shows specific payment details for load shed services by period._
///
/// * Data Set Name: Settlements
/// * File Name: Lshedpayment
/// * Data Version: 5
///
/// # Description
///  SETLSHEDPAYMENT data is confidential to the relevant participant. Source SETLSHEDPAYMENT updates with each settlement run.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsLshedpayment5 {
    /// Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// AS Contract Identifier
    pub contractid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Dispatchable Unit Identifier
    pub duid: Option<String>,
    /// Region Identifier
    pub regionid: Option<String>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Load Shed Enabling Price
    pub lseprice: Option<rust_decimal::Decimal>,
    /// Minimum Compensation Price
    pub mcpprice: Option<rust_decimal::Decimal>,
    /// Load Shed Control Range
    pub lscr: Option<rust_decimal::Decimal>,
    /// Load Shed Enabling Payment
    pub lsepayment: Option<rust_decimal::Decimal>,
    /// Compensation Payment
    pub ccpayment: Option<rust_decimal::Decimal>,
    /// Cleared MW of unit at time of load shed usage
    pub constrainedmw: Option<rust_decimal::Decimal>,
    /// Unconstrained MW of unit at time of load shed usage
    pub unconstrainedmw: Option<rust_decimal::Decimal>,
    /// Amount of load shed
    pub als: Option<rust_decimal::Decimal>,
    /// Initial demand of unit at time of load shed usage
    pub initialdemand: Option<rust_decimal::Decimal>,
    /// Final demand of unit at time of load shed usage
    pub finaldemand: Option<rust_decimal::Decimal>,
    /// AS Contract Version No.
    pub contractversionno: Option<rust_decimal::Decimal>,
    /// Re-offer offer date
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// Re-Offer Version No.
    pub offerversionno: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Payment amount for the Load Shed Availability service
    pub availabilitypayment: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsLshedpayment5 {
    type PrimaryKey = SettlementsLshedpayment5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("LSHEDPAYMENT".into()),
            version: 5,
        }
    }

    fn primary_key(&self) -> SettlementsLshedpayment5PrimaryKey {
        SettlementsLshedpayment5PrimaryKey {
            contractid: self.contractid.clone(),
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_lshedpayment_v5_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsLshedpayment5PrimaryKey {
    pub contractid: String,
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsLshedpayment5PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsLshedpayment5 {
    type Row = SettlementsLshedpayment5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsLshedpayment5 {
    type PrimaryKey = SettlementsLshedpayment5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsLshedpayment5PrimaryKey {
    type Row = SettlementsLshedpayment5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsLshedpayment5PrimaryKey {
    type PrimaryKey = SettlementsLshedpayment5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsLshedpayment5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("tlf", arrow2::datatypes::DataType::Decimal(7, 5), true),
            arrow2::datatypes::Field::new("rrp", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new(
                "lseprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mcpprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lscr", arrow2::datatypes::DataType::Decimal(4, 0), true),
            arrow2::datatypes::Field::new(
                "lsepayment",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "ccpayment",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "constrainedmw",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "unconstrainedmw",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("als", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new(
                "initialdemand",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "finaldemand",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "contractversionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "offerdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "offerversionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "availabilitypayment",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut tlf_array = Vec::new();
        let mut rrp_array = Vec::new();
        let mut lseprice_array = Vec::new();
        let mut mcpprice_array = Vec::new();
        let mut lscr_array = Vec::new();
        let mut lsepayment_array = Vec::new();
        let mut ccpayment_array = Vec::new();
        let mut constrainedmw_array = Vec::new();
        let mut unconstrainedmw_array = Vec::new();
        let mut als_array = Vec::new();
        let mut initialdemand_array = Vec::new();
        let mut finaldemand_array = Vec::new();
        let mut contractversionno_array = Vec::new();
        let mut offerdate_array = Vec::new();
        let mut offerversionno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut availabilitypayment_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            contractid_array.push(row.contractid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            duid_array.push(row.duid);
            regionid_array.push(row.regionid);
            tlf_array.push({
                row.tlf.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp_array.push({
                row.rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lseprice_array.push({
                row.lseprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mcpprice_array.push({
                row.mcpprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lscr_array.push({
                row.lscr.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lsepayment_array.push({
                row.lsepayment.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            ccpayment_array.push({
                row.ccpayment.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            constrainedmw_array.push({
                row.constrainedmw.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            unconstrainedmw_array.push({
                row.unconstrainedmw.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            als_array.push({
                row.als.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            initialdemand_array.push({
                row.initialdemand.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            finaldemand_array.push({
                row.finaldemand.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            contractversionno_array.push({
                row.contractversionno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            offerdate_array.push(row.offerdate.map(|val| val.timestamp()));
            offerversionno_array.push({
                row.offerversionno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            availabilitypayment_array.push({
                row.availabilitypayment.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    contractid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(duid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(tlf_array)
                        .to(arrow2::datatypes::DataType::Decimal(7, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lseprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mcpprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lscr_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lsepayment_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ccpayment_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(constrainedmw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(unconstrainedmw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(als_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(initialdemand_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(finaldemand_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(contractversionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(offerdate_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(offerversionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(availabilitypayment_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETMARKETFEES
///  _SETMARKETFEES shows payments for market fees for each settlement date._
///
/// * Data Set Name: Settlements
/// * File Name: Marketfees
/// * Data Version: 6
///
/// # Description
///  SETMARKETFEES is confidential data. Source SETMARKETFEES updates with each settlement run.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * MARKETFEEID
/// * PARTICIPANTCATEGORYID
/// * PARTICIPANTID
/// * PERIODID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsMarketfees6 {
    /// Settlement date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub runno: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Market fee identifier (e.g. V_EST)
    pub marketfeeid: String,
    /// Fee charge
    pub marketfeevalue: Option<rust_decimal::Decimal>,
    /// Energy amount for variable fees
    pub energy: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The participant category that the market fee recovery pertains to. Corresponds to the PARTICIPANTCATEGORYID column of the PARTICIPANT_BANDFEE_CATEGORYALLOC_C_V view for BAND$ type fees, or to the MARKETFEETYPE column of the MARKETFEE_P_V view for all other fee types.
    pub participantcategoryid: String,
    /// The rate applied to this fee for the participant at the settlement date
    pub feerate: Option<rust_decimal::Decimal>,
    /// The number of units applicable to this fee for the participant, in the trading interval.
    pub feeunits: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsMarketfees6 {
    type PrimaryKey = SettlementsMarketfees6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("MARKETFEES".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> SettlementsMarketfees6PrimaryKey {
        SettlementsMarketfees6PrimaryKey {
            marketfeeid: self.marketfeeid.clone(),
            participantcategoryid: self.participantcategoryid.clone(),
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_marketfees_v6_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsMarketfees6PrimaryKey {
    pub marketfeeid: String,
    pub participantcategoryid: String,
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for SettlementsMarketfees6PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsMarketfees6 {
    type Row = SettlementsMarketfees6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.marketfeeid == row.marketfeeid
            && self.participantcategoryid == row.participantcategoryid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsMarketfees6 {
    type PrimaryKey = SettlementsMarketfees6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.marketfeeid == key.marketfeeid
            && self.participantcategoryid == key.participantcategoryid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for SettlementsMarketfees6PrimaryKey {
    type Row = SettlementsMarketfees6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.marketfeeid == row.marketfeeid
            && self.participantcategoryid == row.participantcategoryid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsMarketfees6PrimaryKey {
    type PrimaryKey = SettlementsMarketfees6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.marketfeeid == key.marketfeeid
            && self.participantcategoryid == key.participantcategoryid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsMarketfees6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "marketfeeid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "marketfeevalue",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "energy",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "participantcategoryid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "feerate",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "feeunits",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut marketfeeid_array = Vec::new();
        let mut marketfeevalue_array = Vec::new();
        let mut energy_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut participantcategoryid_array = Vec::new();
        let mut feerate_array = Vec::new();
        let mut feeunits_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            marketfeeid_array.push(row.marketfeeid);
            marketfeevalue_array.push({
                row.marketfeevalue.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            energy_array.push({
                row.energy.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            participantcategoryid_array.push(row.participantcategoryid);
            feerate_array.push({
                row.feerate.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            feeunits_array.push({
                row.feeunits.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    marketfeeid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(marketfeevalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(energy_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantcategoryid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(feerate_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(feeunits_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETREALLOCATIONS
///  _SETREALLOCATIONS shows the trading interval value of reallocations processed, for those participants whose reallocation submissions have been accepted by AEMO._
///
/// * Data Set Name: Settlements
/// * File Name: Reallocations
/// * Data Version: 5
///
/// # Description
///  SETREALLOCATIONS data is confidential to participants party to the reallocation. Source SETREALLOCATIONS updates by the posting of a billing run. Volume Generally, there are approximately 550 records inserted per week.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * PERIODID
/// * REALLOCATIONID
/// * RUNNO
/// * SETTLEMENTDATE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsReallocations5 {
    /// Calendar Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run no
    pub runno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Reallocation contract identifier
    pub reallocationid: String,
    /// Reallocation value in $
    pub reallocationvalue: Option<rust_decimal::Decimal>,
    /// Energy in MWh if reallocation agreement type is MWh
    pub energy: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for SettlementsReallocations5 {
    type PrimaryKey = SettlementsReallocations5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("REALLOCATIONS".into()),
            version: 5,
        }
    }

    fn primary_key(&self) -> SettlementsReallocations5PrimaryKey {
        SettlementsReallocations5PrimaryKey {
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            reallocationid: self.reallocationid.clone(),
            runno: self.runno,
            settlementdate: self.settlementdate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_reallocations_v5_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsReallocations5PrimaryKey {
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub reallocationid: String,
    pub runno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for SettlementsReallocations5PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsReallocations5 {
    type Row = SettlementsReallocations5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.reallocationid == row.reallocationid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsReallocations5 {
    type PrimaryKey = SettlementsReallocations5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.reallocationid == key.reallocationid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
impl mmsdm_core::CompareWithRow for SettlementsReallocations5PrimaryKey {
    type Row = SettlementsReallocations5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.reallocationid == row.reallocationid
            && self.runno == row.runno
            && self.settlementdate == row.settlementdate
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsReallocations5PrimaryKey {
    type PrimaryKey = SettlementsReallocations5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.reallocationid == key.reallocationid
            && self.runno == key.runno
            && self.settlementdate == key.settlementdate
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsReallocations5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "runno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "reallocationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "reallocationvalue",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "energy",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("rrp", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut runno_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut reallocationid_array = Vec::new();
        let mut reallocationvalue_array = Vec::new();
        let mut energy_array = Vec::new();
        let mut rrp_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            runno_array.push({
                let mut val = row.runno;
                val.rescale(0);
                val.mantissa()
            });
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            reallocationid_array.push(row.reallocationid);
            reallocationvalue_array.push({
                row.reallocationvalue.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            energy_array.push({
                row.energy.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp_array.push({
                row.rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(runno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    reallocationid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(reallocationvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(energy_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETRESTARTPAYMENT
///  _SETRESTARTPAYMENT shows specific payment details for System Restart services by period._
///
/// * Data Set Name: Settlements
/// * File Name: Restartpayment
/// * Data Version: 6
///
/// # Description
///  SETRESTARTPAYMENT data is confidential to the relevant participant. Source SETRESTARTPAYMENT updates with each settlement run.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRestartpayment6 {
    /// Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// Contract Identifier
    pub contractid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: Option<String>,
    /// System Restart Type (0 = FRC, 1 = GRC, 2 = TTH)
    pub restarttype: Option<rust_decimal::Decimal>,
    /// Availability Flag
    pub avaflag: Option<rust_decimal::Decimal>,
    /// Availability Price
    pub availabilityprice: Option<rust_decimal::Decimal>,
    /// Service Test Flag
    pub tcf: Option<rust_decimal::Decimal>,
    /// Availability Payment
    pub availabilitypayment: Option<rust_decimal::Decimal>,
    /// Contract Version No.
    pub contractversionno: Option<rust_decimal::Decimal>,
    /// Re-offer offer date
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// Re-Offer Version No.
    pub offerversionno: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The enabling payment made for system restart in this half-hour interval
    pub enablingpayment: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsRestartpayment6 {
    type PrimaryKey = SettlementsRestartpayment6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("RESTARTPAYMENT".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> SettlementsRestartpayment6PrimaryKey {
        SettlementsRestartpayment6PrimaryKey {
            contractid: self.contractid.clone(),
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_restartpayment_v6_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsRestartpayment6PrimaryKey {
    pub contractid: String,
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsRestartpayment6PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsRestartpayment6 {
    type Row = SettlementsRestartpayment6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsRestartpayment6 {
    type PrimaryKey = SettlementsRestartpayment6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsRestartpayment6PrimaryKey {
    type Row = SettlementsRestartpayment6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsRestartpayment6PrimaryKey {
    type PrimaryKey = SettlementsRestartpayment6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsRestartpayment6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "restarttype",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "avaflag",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "availabilityprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("tcf", arrow2::datatypes::DataType::Decimal(1, 0), true),
            arrow2::datatypes::Field::new(
                "availabilitypayment",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "contractversionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "offerdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "offerversionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "enablingpayment",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut restarttype_array = Vec::new();
        let mut avaflag_array = Vec::new();
        let mut availabilityprice_array = Vec::new();
        let mut tcf_array = Vec::new();
        let mut availabilitypayment_array = Vec::new();
        let mut contractversionno_array = Vec::new();
        let mut offerdate_array = Vec::new();
        let mut offerversionno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut enablingpayment_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            contractid_array.push(row.contractid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            regionid_array.push(row.regionid);
            restarttype_array.push({
                row.restarttype.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            avaflag_array.push({
                row.avaflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            availabilityprice_array.push({
                row.availabilityprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            tcf_array.push({
                row.tcf.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            availabilitypayment_array.push({
                row.availabilitypayment.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            contractversionno_array.push({
                row.contractversionno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            offerdate_array.push(row.offerdate.map(|val| val.timestamp()));
            offerversionno_array.push({
                row.offerversionno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            enablingpayment_array.push({
                row.enablingpayment.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    contractid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(restarttype_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(avaflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(availabilityprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(tcf_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(availabilitypayment_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(contractversionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(offerdate_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(offerversionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enablingpayment_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETRESTARTRECOVERY
///  _SETRESTARTRECOVERY shows reimbursements for system restart Ancillary Services to be recovered from participants. (Data no longer created for Settlement Days from 01/07/2012)_
///
/// * Data Set Name: Settlements
/// * File Name: Restartrecovery
/// * Data Version: 6
///
/// # Description
///  SETRESTARTRECOVERY data is confidential to the relevant participant. Source SETRESTARTRECOVERY updates with each settlement run.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRestartrecovery6 {
    /// Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant to pay recovery
    pub participantid: String,
    /// Contract Identifier
    pub contractid: Option<String>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Region Identifier
    pub regionid: String,
    /// Availability Payment
    pub availabilitypayment: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region
    pub participantdemand: Option<rust_decimal::Decimal>,
    /// NEM Demand (NB sum of ALL Regions)
    pub regiondemand: Option<rust_decimal::Decimal>,
    /// Availability Recovery
    pub availabilityrecovery: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Availability Recovery for Generator
    pub availabilityrecovery_gen: Option<rust_decimal::Decimal>,
    /// Participant Demand in Region for Generator
    pub participantdemand_gen: Option<rust_decimal::Decimal>,
    /// Sum of all generation including SGA generation across all regions of the NEM and floored to zero
    pub regiondemand_gen: Option<rust_decimal::Decimal>,
    /// The enabling payment made for system restart in this half-hour interval
    pub enablingpayment: Option<rust_decimal::Decimal>,
    /// The enabling recovery amount for system restart in this half-hour interval attributable to customer activity
    pub enablingrecovery: Option<rust_decimal::Decimal>,
    /// The enabling recovery amount for system restart in this half-hour interval attributable to generator activity
    pub enablingrecovery_gen: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsRestartrecovery6 {
    type PrimaryKey = SettlementsRestartrecovery6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("RESTARTRECOVERY".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> SettlementsRestartrecovery6PrimaryKey {
        SettlementsRestartrecovery6PrimaryKey {
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_restartrecovery_v6_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsRestartrecovery6PrimaryKey {
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsRestartrecovery6PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsRestartrecovery6 {
    type Row = SettlementsRestartrecovery6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsRestartrecovery6 {
    type PrimaryKey = SettlementsRestartrecovery6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsRestartrecovery6PrimaryKey {
    type Row = SettlementsRestartrecovery6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsRestartrecovery6PrimaryKey {
    type PrimaryKey = SettlementsRestartrecovery6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsRestartrecovery6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "availabilitypayment",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "participantdemand",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "regiondemand",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "availabilityrecovery",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "availabilityrecovery_gen",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "participantdemand_gen",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "regiondemand_gen",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "enablingpayment",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "enablingrecovery",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "enablingrecovery_gen",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut availabilitypayment_array = Vec::new();
        let mut participantdemand_array = Vec::new();
        let mut regiondemand_array = Vec::new();
        let mut availabilityrecovery_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut availabilityrecovery_gen_array = Vec::new();
        let mut participantdemand_gen_array = Vec::new();
        let mut regiondemand_gen_array = Vec::new();
        let mut enablingpayment_array = Vec::new();
        let mut enablingrecovery_array = Vec::new();
        let mut enablingrecovery_gen_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            contractid_array.push(row.contractid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            regionid_array.push(row.regionid);
            availabilitypayment_array.push({
                row.availabilitypayment.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            participantdemand_array.push({
                row.participantdemand.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            regiondemand_array.push({
                row.regiondemand.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            availabilityrecovery_array.push({
                row.availabilityrecovery.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            availabilityrecovery_gen_array.push({
                row.availabilityrecovery_gen.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            participantdemand_gen_array.push({
                row.participantdemand_gen.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            regiondemand_gen_array.push({
                row.regiondemand_gen.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            enablingpayment_array.push({
                row.enablingpayment.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            enablingrecovery_array.push({
                row.enablingrecovery.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            enablingrecovery_gen_array.push({
                row.enablingrecovery_gen.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(contractid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(availabilitypayment_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(participantdemand_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(regiondemand_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(availabilityrecovery_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(availabilityrecovery_gen_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(participantdemand_gen_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(regiondemand_gen_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enablingpayment_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enablingrecovery_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enablingrecovery_gen_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETRPOWERPAYMENT
///  _SETRPOWERPAYMENT shows specific payment details for Reactive power services by period._
///
/// * Data Set Name: Settlements
/// * File Name: Rpowerpayment
/// * Data Version: 6
///
/// # Description
///  SETRPOWERPAYMENT data is confidential to the relevant participant. Source SETRPOWERPAYMENT updates with each settlement run.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRpowerpayment6 {
    /// Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No.
    pub versionno: rust_decimal::Decimal,
    /// Participant Identifier
    pub participantid: String,
    /// AS Contract Identifier
    pub contractid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Dispatchable Unit Identifier
    pub duid: Option<String>,
    /// Region Identifier
    pub regionid: Option<String>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Eligible Bid Price
    pub ebp: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Availability price per MVAr of RP absorption capability
    pub mvaraprice: Option<rust_decimal::Decimal>,
    /// Enabling Price
    pub mvareprice: Option<rust_decimal::Decimal>,
    /// Availability price per MVAr of RP generation capability
    pub mvargprice: Option<rust_decimal::Decimal>,
    /// Compensation Cap
    pub ccprice: Option<rust_decimal::Decimal>,
    /// Sync Compensation Flag
    pub synccompensation: Option<rust_decimal::Decimal>,
    /// Reactive Power Absorption Capability (MVAr)
    pub mta: Option<rust_decimal::Decimal>,
    /// Reactive Power Generation Capability (MVAr)
    pub mtg: Option<rust_decimal::Decimal>,
    /// Block size of unit
    pub blocksize: Option<rust_decimal::Decimal>,
    /// Availability Flag
    pub avaflag: Option<rust_decimal::Decimal>,
    /// Cleared MW of unit
    pub clearedmw: Option<rust_decimal::Decimal>,
    /// Unconstrained MW of unit
    pub unconstrainedmw: Option<rust_decimal::Decimal>,
    /// Availability Payment
    pub availabilitypayment: Option<rust_decimal::Decimal>,
    /// Enabling Payment
    pub enablingpayment: Option<rust_decimal::Decimal>,
    /// Compensation Payment
    pub ccpayment: Option<rust_decimal::Decimal>,
    /// AS Contract Version No.
    pub contractversionno: Option<rust_decimal::Decimal>,
    /// Re-offer offer date
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub offerdate: Option<chrono::NaiveDateTime>,
    /// Re-Offer Version No.
    pub offerversionno: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The rebate amount if MegaVar (MVAr) is below the threshold.
    pub availabilitypayment_rebate: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsRpowerpayment6 {
    type PrimaryKey = SettlementsRpowerpayment6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("RPOWERPAYMENT".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> SettlementsRpowerpayment6PrimaryKey {
        SettlementsRpowerpayment6PrimaryKey {
            contractid: self.contractid.clone(),
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_rpowerpayment_v6_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsRpowerpayment6PrimaryKey {
    pub contractid: String,
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsRpowerpayment6PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsRpowerpayment6 {
    type Row = SettlementsRpowerpayment6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsRpowerpayment6 {
    type PrimaryKey = SettlementsRpowerpayment6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsRpowerpayment6PrimaryKey {
    type Row = SettlementsRpowerpayment6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsRpowerpayment6PrimaryKey {
    type PrimaryKey = SettlementsRpowerpayment6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsRpowerpayment6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("tlf", arrow2::datatypes::DataType::Decimal(7, 5), true),
            arrow2::datatypes::Field::new("ebp", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new("rrp", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new(
                "mvaraprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mvareprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "mvargprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "ccprice",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "synccompensation",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new("mta", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new("mtg", arrow2::datatypes::DataType::Decimal(15, 5), true),
            arrow2::datatypes::Field::new(
                "blocksize",
                arrow2::datatypes::DataType::Decimal(4, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "avaflag",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "clearedmw",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "unconstrainedmw",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "availabilitypayment",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "enablingpayment",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "ccpayment",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "contractversionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "offerdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "offerversionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "availabilitypayment_rebate",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut tlf_array = Vec::new();
        let mut ebp_array = Vec::new();
        let mut rrp_array = Vec::new();
        let mut mvaraprice_array = Vec::new();
        let mut mvareprice_array = Vec::new();
        let mut mvargprice_array = Vec::new();
        let mut ccprice_array = Vec::new();
        let mut synccompensation_array = Vec::new();
        let mut mta_array = Vec::new();
        let mut mtg_array = Vec::new();
        let mut blocksize_array = Vec::new();
        let mut avaflag_array = Vec::new();
        let mut clearedmw_array = Vec::new();
        let mut unconstrainedmw_array = Vec::new();
        let mut availabilitypayment_array = Vec::new();
        let mut enablingpayment_array = Vec::new();
        let mut ccpayment_array = Vec::new();
        let mut contractversionno_array = Vec::new();
        let mut offerdate_array = Vec::new();
        let mut offerversionno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut availabilitypayment_rebate_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            contractid_array.push(row.contractid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            duid_array.push(row.duid);
            regionid_array.push(row.regionid);
            tlf_array.push({
                row.tlf.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            ebp_array.push({
                row.ebp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rrp_array.push({
                row.rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mvaraprice_array.push({
                row.mvaraprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mvareprice_array.push({
                row.mvareprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mvargprice_array.push({
                row.mvargprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            ccprice_array.push({
                row.ccprice.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            synccompensation_array.push({
                row.synccompensation.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            mta_array.push({
                row.mta.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            mtg_array.push({
                row.mtg.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            blocksize_array.push({
                row.blocksize.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            avaflag_array.push({
                row.avaflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            clearedmw_array.push({
                row.clearedmw.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            unconstrainedmw_array.push({
                row.unconstrainedmw.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            availabilitypayment_array.push({
                row.availabilitypayment.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            enablingpayment_array.push({
                row.enablingpayment.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            ccpayment_array.push({
                row.ccpayment.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            contractversionno_array.push({
                row.contractversionno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            offerdate_array.push(row.offerdate.map(|val| val.timestamp()));
            offerversionno_array.push({
                row.offerversionno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            availabilitypayment_rebate_array.push({
                row.availabilitypayment_rebate.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    contractid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(duid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(tlf_array)
                        .to(arrow2::datatypes::DataType::Decimal(7, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ebp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mvaraprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mvareprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mvargprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ccprice_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(synccompensation_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mta_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mtg_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(blocksize_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(avaflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(clearedmw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(unconstrainedmw_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(availabilitypayment_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enablingpayment_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ccpayment_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(contractversionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(offerdate_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(offerversionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(availabilitypayment_rebate_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SETSMALLGENDATA
///  _Publishes metering data and associated settlement values for with a registered Small Generator Aggregator participants connection points._
///
/// * Data Set Name: Settlements
/// * File Name: Smallgendata
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONNECTIONPOINTID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsSmallgendata1 {
    /// Settlement date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Version number of the record for the settlement date
    pub versionno: rust_decimal::Decimal,
    /// Transmission Node Identifier (TNI)
    pub connectionpointid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Unique participant identifier
    pub participantid: String,
    /// Region Identifier
    pub regionid: Option<String>,
    /// The import direction value for the meter read (MWh)
    pub importenergy: Option<rust_decimal::Decimal>,
    /// The export direction value for the meter read (MWh)
    pub exportenergy: Option<rust_decimal::Decimal>,
    /// Regional Reference Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Transmission Loss Factor
    pub tlf: Option<rust_decimal::Decimal>,
    /// Import Energy Cost ($)
    pub impenergycost: Option<rust_decimal::Decimal>,
    /// Export Energy Cost ($)
    pub expenergycost: Option<rust_decimal::Decimal>,
    /// Last date and time the record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for SettlementsSmallgendata1 {
    type PrimaryKey = SettlementsSmallgendata1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("SMALLGENDATA".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementsSmallgendata1PrimaryKey {
        SettlementsSmallgendata1PrimaryKey {
            connectionpointid: self.connectionpointid.clone(),
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_smallgendata_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsSmallgendata1PrimaryKey {
    pub connectionpointid: String,
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsSmallgendata1PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsSmallgendata1 {
    type Row = SettlementsSmallgendata1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.connectionpointid == row.connectionpointid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsSmallgendata1 {
    type PrimaryKey = SettlementsSmallgendata1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid == key.connectionpointid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsSmallgendata1PrimaryKey {
    type Row = SettlementsSmallgendata1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.connectionpointid == row.connectionpointid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsSmallgendata1PrimaryKey {
    type PrimaryKey = SettlementsSmallgendata1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.connectionpointid == key.connectionpointid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsSmallgendata1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "importenergy",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "exportenergy",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new("rrp", arrow2::datatypes::DataType::Decimal(18, 8), true),
            arrow2::datatypes::Field::new("tlf", arrow2::datatypes::DataType::Decimal(18, 8), true),
            arrow2::datatypes::Field::new(
                "impenergycost",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "expenergycost",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut connectionpointid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut importenergy_array = Vec::new();
        let mut exportenergy_array = Vec::new();
        let mut rrp_array = Vec::new();
        let mut tlf_array = Vec::new();
        let mut impenergycost_array = Vec::new();
        let mut expenergycost_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            connectionpointid_array.push(row.connectionpointid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            regionid_array.push(row.regionid);
            importenergy_array.push({
                row.importenergy.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            exportenergy_array.push({
                row.exportenergy.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            rrp_array.push({
                row.rrp.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            tlf_array.push({
                row.tlf.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            impenergycost_array.push({
                row.impenergycost.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            expenergycost_array.push({
                row.expenergycost.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    connectionpointid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(importenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(exportenergy_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(tlf_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(impenergycost_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(expenergycost_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SET_ANCILLARY_SUMMARY
///  _SET_ANCILLARY_SUMMARY summarises payments for all Ancillary Services to participants on the basis of regions and trading intervals._
///
/// * Data Set Name: Settlements
/// * File Name: Ancillary Summary
/// * Data Version: 5
///
/// # Description
///  SET_ANCILLARY_SUMMARY data is available to all participants. Volume Approximately 30, 000 per week.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * PAYMENTTYPE
/// * PERIODID
/// * REGIONID
/// * SERVICE
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsAncillarySummary5 {
    /// Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No
    pub versionno: rust_decimal::Decimal,
    /// Ancillary service identifier (e.g. REACTIVE_POWER)
    pub service: String,
    /// Payment type identifier (e.g. COMPENSATION)
    pub paymenttype: String,
    /// Region Identifier
    pub regionid: String,
    /// Trading interval
    pub periodid: rust_decimal::Decimal,
    /// The NEM ancillary summary regional payment amount ($)
    pub paymentamount: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for SettlementsAncillarySummary5 {
    type PrimaryKey = SettlementsAncillarySummary5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("ANCILLARY_SUMMARY".into()),
            version: 5,
        }
    }

    fn primary_key(&self) -> SettlementsAncillarySummary5PrimaryKey {
        SettlementsAncillarySummary5PrimaryKey {
            paymenttype: self.paymenttype.clone(),
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            service: self.service.clone(),
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_ancillary_summary_v5_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsAncillarySummary5PrimaryKey {
    pub paymenttype: String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub service: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsAncillarySummary5PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsAncillarySummary5 {
    type Row = SettlementsAncillarySummary5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.paymenttype == row.paymenttype
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.service == row.service
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsAncillarySummary5 {
    type PrimaryKey = SettlementsAncillarySummary5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.paymenttype == key.paymenttype
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.service == key.service
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsAncillarySummary5PrimaryKey {
    type Row = SettlementsAncillarySummary5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.paymenttype == row.paymenttype
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.service == row.service
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsAncillarySummary5PrimaryKey {
    type PrimaryKey = SettlementsAncillarySummary5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.paymenttype == key.paymenttype
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.service == key.service
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsAncillarySummary5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
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
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "paymentamount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut service_array = Vec::new();
        let mut paymenttype_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut paymentamount_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            service_array.push(row.service);
            paymenttype_array.push(row.paymenttype);
            regionid_array.push(row.regionid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            paymentamount_array.push({
                row.paymentamount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
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
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(paymentamount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SET_APC_COMPENSATION
///  _APC Compensation payment amounts in the Settlements timeframe_
///
/// * Data Set Name: Settlements
/// * File Name: Apc Compensation
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * APEVENTID
/// * CLAIMID
/// * PARTICIPANTID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsApcCompensation1 {
    /// Settlement run date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub versionno: i64,
    /// AP Event Id
    pub apeventid: i64,
    /// AP Event Claim Id
    pub claimid: i64,
    /// Participant identifier
    pub participantid: String,
    /// Trading interval identifier
    pub periodid: i64,
    /// Compensation amount for the event claim in this interval
    pub compensation_amount: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsApcCompensation1 {
    type PrimaryKey = SettlementsApcCompensation1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("APC_COMPENSATION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementsApcCompensation1PrimaryKey {
        SettlementsApcCompensation1PrimaryKey {
            apeventid: self.apeventid,
            claimid: self.claimid,
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_apc_compensation_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsApcCompensation1PrimaryKey {
    pub apeventid: i64,
    pub claimid: i64,
    pub participantid: String,
    pub periodid: i64,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: i64,
}
impl mmsdm_core::PrimaryKey for SettlementsApcCompensation1PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsApcCompensation1 {
    type Row = SettlementsApcCompensation1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.apeventid == row.apeventid
            && self.claimid == row.claimid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsApcCompensation1 {
    type PrimaryKey = SettlementsApcCompensation1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid
            && self.claimid == key.claimid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsApcCompensation1PrimaryKey {
    type Row = SettlementsApcCompensation1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.apeventid == row.apeventid
            && self.claimid == row.claimid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsApcCompensation1PrimaryKey {
    type PrimaryKey = SettlementsApcCompensation1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid
            && self.claimid == key.claimid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsApcCompensation1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("apeventid", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("claimid", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "compensation_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut apeventid_array = Vec::new();
        let mut claimid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut compensation_amount_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push(row.versionno);
            apeventid_array.push(row.apeventid);
            claimid_array.push(row.claimid);
            participantid_array.push(row.participantid);
            periodid_array.push(row.periodid);
            compensation_amount_array.push({
                row.compensation_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(apeventid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(claimid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(compensation_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SET_APC_RECOVERY
///  _APC Compensation recovery amounts in the Settlements timeframe_
///
/// * Data Set Name: Settlements
/// * File Name: Apc Recovery
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * APEVENTID
/// * CLAIMID
/// * PARTICIPANTID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsApcRecovery1 {
    /// Settlement run date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub versionno: i64,
    /// AP Event Id
    pub apeventid: i64,
    /// AP Event Claim Id
    pub claimid: i64,
    /// Participant identifier
    pub participantid: String,
    /// Settlements Trading Interval.
    pub periodid: i64,
    /// Region id for the recovery amount
    pub regionid: String,
    /// Recovery amount in the region attributable to the participant for the event claim in this interval
    pub recovery_amount: Option<rust_decimal::Decimal>,
    /// Total Recovery amount in the region for the event claim in this interval
    pub region_recovery_br_amount: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsApcRecovery1 {
    type PrimaryKey = SettlementsApcRecovery1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("APC_RECOVERY".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementsApcRecovery1PrimaryKey {
        SettlementsApcRecovery1PrimaryKey {
            apeventid: self.apeventid,
            claimid: self.claimid,
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_apc_recovery_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsApcRecovery1PrimaryKey {
    pub apeventid: i64,
    pub claimid: i64,
    pub participantid: String,
    pub periodid: i64,
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: i64,
}
impl mmsdm_core::PrimaryKey for SettlementsApcRecovery1PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsApcRecovery1 {
    type Row = SettlementsApcRecovery1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.apeventid == row.apeventid
            && self.claimid == row.claimid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsApcRecovery1 {
    type PrimaryKey = SettlementsApcRecovery1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid
            && self.claimid == key.claimid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsApcRecovery1PrimaryKey {
    type Row = SettlementsApcRecovery1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.apeventid == row.apeventid
            && self.claimid == row.claimid
            && self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsApcRecovery1PrimaryKey {
    type PrimaryKey = SettlementsApcRecovery1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid
            && self.claimid == key.claimid
            && self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsApcRecovery1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("apeventid", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new("claimid", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("periodid", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "recovery_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "region_recovery_br_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut apeventid_array = Vec::new();
        let mut claimid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut recovery_amount_array = Vec::new();
        let mut region_recovery_br_amount_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push(row.versionno);
            apeventid_array.push(row.apeventid);
            claimid_array.push(row.claimid);
            participantid_array.push(row.participantid);
            periodid_array.push(row.periodid);
            regionid_array.push(row.regionid);
            recovery_amount_array.push({
                row.recovery_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            region_recovery_br_amount_array.push({
                row.region_recovery_br_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(apeventid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(claimid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(periodid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(recovery_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(region_recovery_br_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SET_FCAS_PAYMENT
///  _SET_FCAS_PAYMENT sets out the enabling payment details for frequency controlled Ancillary Services._
///
/// * Data Set Name: Settlements
/// * File Name: Fcas Payment
/// * Data Version: 5
///
/// # Description
///  SET_FCAS_PAYMENT data is confidential to the relevant participant. Volume Approximately 150,000 per week.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * DUID
/// * PERIODID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsFcasPayment5 {
    /// Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No
    pub versionno: rust_decimal::Decimal,
    /// Participant identifier
    pub participantid: Option<String>,
    /// Dispatchable unit identifier
    pub duid: String,
    /// Region Identifier
    pub regionid: Option<String>,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Lower 6 Second Payment
    pub lower6sec_payment: Option<rust_decimal::Decimal>,
    /// Raise 6 Second Payment
    pub raise6sec_payment: Option<rust_decimal::Decimal>,
    /// Lower 60 Second Payment
    pub lower60sec_payment: Option<rust_decimal::Decimal>,
    /// Raise 60 Second Payment
    pub raise60sec_payment: Option<rust_decimal::Decimal>,
    /// Lower 5 Minute Payment
    pub lower5min_payment: Option<rust_decimal::Decimal>,
    /// Raise 5 Minute Payment
    pub raise5min_payment: Option<rust_decimal::Decimal>,
    /// Lower 5 Minute Regulation Payment
    pub lowerreg_payment: Option<rust_decimal::Decimal>,
    /// Raise 5 Minute Regulation Payment
    pub raisereg_payment: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for SettlementsFcasPayment5 {
    type PrimaryKey = SettlementsFcasPayment5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("FCAS_PAYMENT".into()),
            version: 5,
        }
    }

    fn primary_key(&self) -> SettlementsFcasPayment5PrimaryKey {
        SettlementsFcasPayment5PrimaryKey {
            duid: self.duid.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_fcas_payment_v5_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsFcasPayment5PrimaryKey {
    pub duid: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsFcasPayment5PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsFcasPayment5 {
    type Row = SettlementsFcasPayment5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsFcasPayment5 {
    type PrimaryKey = SettlementsFcasPayment5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsFcasPayment5PrimaryKey {
    type Row = SettlementsFcasPayment5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.duid == row.duid
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsFcasPayment5PrimaryKey {
    type PrimaryKey = SettlementsFcasPayment5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.duid == key.duid
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsFcasPayment5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "lower6sec_payment",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6sec_payment",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60sec_payment",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60sec_payment",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5min_payment",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5min_payment",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerreg_payment",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raisereg_payment",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut lower6sec_payment_array = Vec::new();
        let mut raise6sec_payment_array = Vec::new();
        let mut lower60sec_payment_array = Vec::new();
        let mut raise60sec_payment_array = Vec::new();
        let mut lower5min_payment_array = Vec::new();
        let mut raise5min_payment_array = Vec::new();
        let mut lowerreg_payment_array = Vec::new();
        let mut raisereg_payment_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            duid_array.push(row.duid);
            regionid_array.push(row.regionid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            lower6sec_payment_array.push({
                row.lower6sec_payment.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            raise6sec_payment_array.push({
                row.raise6sec_payment.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lower60sec_payment_array.push({
                row.lower60sec_payment.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            raise60sec_payment_array.push({
                row.raise60sec_payment.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lower5min_payment_array.push({
                row.lower5min_payment.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            raise5min_payment_array.push({
                row.raise5min_payment.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lowerreg_payment_array.push({
                row.lowerreg_payment.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            raisereg_payment_array.push({
                row.raisereg_payment.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(participantid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(duid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6sec_payment_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6sec_payment_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60sec_payment_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60sec_payment_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5min_payment_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5min_payment_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerreg_payment_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raisereg_payment_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SET_FCAS_RECOVERY
///  _SET_FCAS_RECOVERY shows reimbursements for the Frequency Control Ancillary Services (FCAS) to be recovered from participants. Beware of potential confusion with the table SETFCASRECOVERY, which reports reimbursements for Frequency Control Ancillary Services Compensation (now unused)._
///
/// * Data Set Name: Settlements
/// * File Name: Fcas Recovery
/// * Data Version: 6
///
/// # Description
///  SET_FCAS_RECOVERY data is confidential to the relevant participant. Volume Approximately 1, 500, 000 per week.
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsFcasRecovery6 {
    /// Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No
    pub versionno: String,
    /// Participant identifier
    pub participantid: String,
    /// Region Identifier
    pub regionid: String,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// Recovery amount for the Lower 6 Second service attributable to customer connection points
    pub lower6sec_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 6 Second service attributable to customer connection points
    pub raise6sec_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 60 Second service attributable to customer connection points
    pub lower60sec_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 60 Second service attributable to customer connection points
    pub raise60sec_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 5 Minute service attributable to customer connection points
    pub lower5min_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 5 Minute service attributable to customer connection points
    pub raise5min_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower Regulation service attributable to customer connection points
    pub lowerreg_recovery: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise Regulation Second service attributable to customer connection points
    pub raisereg_recovery: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Recovery amount for the Lower 6 Second service attributable to generator connection points
    pub lower6sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 6 Second service attributable to generator connection points
    pub raise6sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 60 Second service attributable to generator connection points
    pub lower60sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 60 Second service attributable to generator connection points
    pub raise60sec_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower 5 Minute service attributable to generator connection points
    pub lower5min_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise 5 Minute service attributable to generator connection points
    pub raise5min_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Lower Regulation service attributable to generator connection points
    pub lowerreg_recovery_gen: Option<rust_decimal::Decimal>,
    /// Recovery amount for the Raise Regulation Second service attributable to generator connection points
    pub raisereg_recovery_gen: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsFcasRecovery6 {
    type PrimaryKey = SettlementsFcasRecovery6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("FCAS_RECOVERY".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> SettlementsFcasRecovery6PrimaryKey {
        SettlementsFcasRecovery6PrimaryKey {
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
            versionno: self.versionno.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_fcas_recovery_v6_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsFcasRecovery6PrimaryKey {
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: String,
}
impl mmsdm_core::PrimaryKey for SettlementsFcasRecovery6PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsFcasRecovery6 {
    type Row = SettlementsFcasRecovery6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsFcasRecovery6 {
    type PrimaryKey = SettlementsFcasRecovery6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsFcasRecovery6PrimaryKey {
    type Row = SettlementsFcasRecovery6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsFcasRecovery6PrimaryKey {
    type PrimaryKey = SettlementsFcasRecovery6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsFcasRecovery6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
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
            arrow2::datatypes::Field::new(
                "lower6sec_recovery",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6sec_recovery",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60sec_recovery",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60sec_recovery",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5min_recovery",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5min_recovery",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerreg_recovery",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raisereg_recovery",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6sec_recovery_gen",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6sec_recovery_gen",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60sec_recovery_gen",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60sec_recovery_gen",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5min_recovery_gen",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5min_recovery_gen",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerreg_recovery_gen",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raisereg_recovery_gen",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut lower6sec_recovery_array = Vec::new();
        let mut raise6sec_recovery_array = Vec::new();
        let mut lower60sec_recovery_array = Vec::new();
        let mut raise60sec_recovery_array = Vec::new();
        let mut lower5min_recovery_array = Vec::new();
        let mut raise5min_recovery_array = Vec::new();
        let mut lowerreg_recovery_array = Vec::new();
        let mut raisereg_recovery_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut lower6sec_recovery_gen_array = Vec::new();
        let mut raise6sec_recovery_gen_array = Vec::new();
        let mut lower60sec_recovery_gen_array = Vec::new();
        let mut raise60sec_recovery_gen_array = Vec::new();
        let mut lower5min_recovery_gen_array = Vec::new();
        let mut raise5min_recovery_gen_array = Vec::new();
        let mut lowerreg_recovery_gen_array = Vec::new();
        let mut raisereg_recovery_gen_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push(row.versionno);
            participantid_array.push(row.participantid);
            regionid_array.push(row.regionid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            lower6sec_recovery_array.push({
                row.lower6sec_recovery.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            raise6sec_recovery_array.push({
                row.raise6sec_recovery.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lower60sec_recovery_array.push({
                row.lower60sec_recovery.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            raise60sec_recovery_array.push({
                row.raise60sec_recovery.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lower5min_recovery_array.push({
                row.lower5min_recovery.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            raise5min_recovery_array.push({
                row.raise5min_recovery.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lowerreg_recovery_array.push({
                row.lowerreg_recovery.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            raisereg_recovery_array.push({
                row.raisereg_recovery.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            lower6sec_recovery_gen_array.push({
                row.lower6sec_recovery_gen.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            raise6sec_recovery_gen_array.push({
                row.raise6sec_recovery_gen.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lower60sec_recovery_gen_array.push({
                row.lower60sec_recovery_gen.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            raise60sec_recovery_gen_array.push({
                row.raise60sec_recovery_gen.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lower5min_recovery_gen_array.push({
                row.lower5min_recovery_gen.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            raise5min_recovery_gen_array.push({
                row.raise5min_recovery_gen.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lowerreg_recovery_gen_array.push({
                row.lowerreg_recovery_gen.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            raisereg_recovery_gen_array.push({
                row.raisereg_recovery_gen.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(versionno_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6sec_recovery_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6sec_recovery_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60sec_recovery_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60sec_recovery_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5min_recovery_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5min_recovery_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerreg_recovery_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raisereg_recovery_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6sec_recovery_gen_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6sec_recovery_gen_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60sec_recovery_gen_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60sec_recovery_gen_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5min_recovery_gen_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5min_recovery_gen_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerreg_recovery_gen_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raisereg_recovery_gen_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SET_FCAS_REGULATION_TRK
///  _SET_FCAS_REGULATION_TRK shows FCAS Regulation Service Constraint tracking for Regional FCAS Regulation recovery_
///
/// * Data Set Name: Settlements
/// * File Name: Set Fcas Regulation Trk
/// * Data Version: 2
///
/// # Description
///  SET_FCAS_REGULATION_TRK contains public data and is available to all participants. Volume Approximately 350,000 per week.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONSTRAINTID
/// * INTERVAL_DATETIME
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsSetFcasRegulationTrk2 {
    /// Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run No
    pub versionno: rust_decimal::Decimal,
    /// Dispatch Interval Date Time
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub interval_datetime: chrono::NaiveDateTime,
    /// Generic Constraint ID
    pub constraintid: String,
    /// Constraint Market Participant Factor
    pub cmpf: Option<rust_decimal::Decimal>,
    /// Constraint Residual Market Participant Factor
    pub crmpf: Option<rust_decimal::Decimal>,
    /// Recovery factor for CMPF based recovery
    pub recovery_factor_cmpf: Option<rust_decimal::Decimal>,
    /// Recovery factor for CRMPF based recovery
    pub recovery_factor_crmpf: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag to indication that substitute demand was used to recover this requirement
    pub usesubstitutedemand: Option<rust_decimal::Decimal>,
    /// the aggregate customer demand value used to recover the cost of this requirement
    pub requirementdemand: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsSetFcasRegulationTrk2 {
    type PrimaryKey = SettlementsSetFcasRegulationTrk2PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("SET_FCAS_REGULATION_TRK".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> SettlementsSetFcasRegulationTrk2PrimaryKey {
        SettlementsSetFcasRegulationTrk2PrimaryKey {
            constraintid: self.constraintid.clone(),
            interval_datetime: self.interval_datetime,
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_set_fcas_regulation_trk_v2_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsSetFcasRegulationTrk2PrimaryKey {
    pub constraintid: String,
    pub interval_datetime: chrono::NaiveDateTime,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsSetFcasRegulationTrk2PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsSetFcasRegulationTrk2 {
    type Row = SettlementsSetFcasRegulationTrk2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid
            && self.interval_datetime == row.interval_datetime
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsSetFcasRegulationTrk2 {
    type PrimaryKey = SettlementsSetFcasRegulationTrk2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsSetFcasRegulationTrk2PrimaryKey {
    type Row = SettlementsSetFcasRegulationTrk2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.constraintid == row.constraintid
            && self.interval_datetime == row.interval_datetime
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsSetFcasRegulationTrk2PrimaryKey {
    type PrimaryKey = SettlementsSetFcasRegulationTrk2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.constraintid == key.constraintid
            && self.interval_datetime == key.interval_datetime
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsSetFcasRegulationTrk2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "interval_datetime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "constraintid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "cmpf",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "crmpf",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "recovery_factor_cmpf",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "recovery_factor_crmpf",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "usesubstitutedemand",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "requirementdemand",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut interval_datetime_array = Vec::new();
        let mut constraintid_array = Vec::new();
        let mut cmpf_array = Vec::new();
        let mut crmpf_array = Vec::new();
        let mut recovery_factor_cmpf_array = Vec::new();
        let mut recovery_factor_crmpf_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut usesubstitutedemand_array = Vec::new();
        let mut requirementdemand_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            interval_datetime_array.push(row.interval_datetime.timestamp());
            constraintid_array.push(row.constraintid);
            cmpf_array.push({
                row.cmpf.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            crmpf_array.push({
                row.crmpf.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            recovery_factor_cmpf_array.push({
                row.recovery_factor_cmpf.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            recovery_factor_crmpf_array.push({
                row.recovery_factor_crmpf.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            usesubstitutedemand_array.push({
                row.usesubstitutedemand.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            requirementdemand_array.push({
                row.requirementdemand.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(interval_datetime_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    constraintid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(cmpf_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(crmpf_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(recovery_factor_cmpf_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(recovery_factor_crmpf_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(usesubstitutedemand_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(requirementdemand_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SET_NMAS_RECOVERY
///  _SET_NMAS_RECOVERY sets out the NSCAS recovery data for payments other than testing._
///
/// * Data Set Name: Settlements
/// * File Name: Nmas Recovery
/// * Data Version: 2
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * PARTICIPANTID
/// * PAYMENTTYPE
/// * PERIODID
/// * REGIONID
/// * SERVICE
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsNmasRecovery2 {
    /// Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// The Participant from whom the amount is recovered
    pub participantid: String,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>- RESTART
    pub service: String,
    /// The NMAS Contract Id
    pub contractid: String,
    /// The type of payment being recovered. Valid values are:<br>- AVAILABILITY<br>- ENABLEMENT<br>- COMPENSATION<br>
    pub paymenttype: String,
    /// The region from where the amount is recovered
    pub regionid: String,
    /// The Benefitting Factor for the RegionId
    pub rbf: Option<rust_decimal::Decimal>,
    /// The total Payment Amount to recover from all  benefitting regions
    pub payment_amount: Option<rust_decimal::Decimal>,
    /// The Participant energy in MWh for the period
    pub participant_energy: Option<rust_decimal::Decimal>,
    /// The RegionId energy in MWh for the period
    pub region_energy: Option<rust_decimal::Decimal>,
    /// The Total recovery amount for the period for the PARTICIPANTID and REGIONID
    pub recovery_amount: Option<rust_decimal::Decimal>,
    /// The Last Updated date and time
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Participant Generator Energy in the benefitting region
    pub participant_generation: Option<rust_decimal::Decimal>,
    /// The generator energy in the benefitting region
    pub region_generation: Option<rust_decimal::Decimal>,
    /// The recovery amount allocated to customers
    pub recovery_amount_customer: Option<rust_decimal::Decimal>,
    /// The recovery amount allocated to generators
    pub recovery_amount_generator: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsNmasRecovery2 {
    type PrimaryKey = SettlementsNmasRecovery2PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("NMAS_RECOVERY".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> SettlementsNmasRecovery2PrimaryKey {
        SettlementsNmasRecovery2PrimaryKey {
            contractid: self.contractid.clone(),
            participantid: self.participantid.clone(),
            paymenttype: self.paymenttype.clone(),
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            service: self.service.clone(),
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_nmas_recovery_v2_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsNmasRecovery2PrimaryKey {
    pub contractid: String,
    pub participantid: String,
    pub paymenttype: String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub service: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsNmasRecovery2PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsNmasRecovery2 {
    type Row = SettlementsNmasRecovery2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.participantid == row.participantid
            && self.paymenttype == row.paymenttype
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.service == row.service
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsNmasRecovery2 {
    type PrimaryKey = SettlementsNmasRecovery2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.participantid == key.participantid
            && self.paymenttype == key.paymenttype
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.service == key.service
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsNmasRecovery2PrimaryKey {
    type Row = SettlementsNmasRecovery2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.participantid == row.participantid
            && self.paymenttype == row.paymenttype
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.service == row.service
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsNmasRecovery2PrimaryKey {
    type PrimaryKey = SettlementsNmasRecovery2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.participantid == key.participantid
            && self.paymenttype == key.paymenttype
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.service == key.service
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsNmasRecovery2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("service", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "paymenttype",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("rbf", arrow2::datatypes::DataType::Decimal(18, 8), true),
            arrow2::datatypes::Field::new(
                "payment_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "participant_energy",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "region_energy",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "recovery_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "participant_generation",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "region_generation",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "recovery_amount_customer",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "recovery_amount_generator",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut service_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut paymenttype_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut rbf_array = Vec::new();
        let mut payment_amount_array = Vec::new();
        let mut participant_energy_array = Vec::new();
        let mut region_energy_array = Vec::new();
        let mut recovery_amount_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut participant_generation_array = Vec::new();
        let mut region_generation_array = Vec::new();
        let mut recovery_amount_customer_array = Vec::new();
        let mut recovery_amount_generator_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            service_array.push(row.service);
            contractid_array.push(row.contractid);
            paymenttype_array.push(row.paymenttype);
            regionid_array.push(row.regionid);
            rbf_array.push({
                row.rbf.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            payment_amount_array.push({
                row.payment_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            participant_energy_array.push({
                row.participant_energy.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            region_energy_array.push({
                row.region_energy.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            recovery_amount_array.push({
                row.recovery_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
            participant_generation_array.push({
                row.participant_generation.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            region_generation_array.push({
                row.region_generation.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            recovery_amount_customer_array.push({
                row.recovery_amount_customer.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            recovery_amount_generator_array.push({
                row.recovery_amount_generator.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(service_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    contractid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    paymenttype_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rbf_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(payment_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(participant_energy_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(region_energy_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(recovery_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(participant_generation_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(region_generation_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(recovery_amount_customer_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(recovery_amount_generator_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SET_NMAS_RECOVERY_RBF
///  _SET_NMAS_RECOVERY_RBF publishes the RBF for NSCAS non testing payments on a half hourly basis._
///
/// * Data Set Name: Settlements
/// * File Name: Nmas Recovery Rbf
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * CONTRACTID
/// * PAYMENTTYPE
/// * PERIODID
/// * REGIONID
/// * SERVICE
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsNmasRecoveryRbf1 {
    /// Settlement Date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub versionno: rust_decimal::Decimal,
    /// Settlements Trading Interval.
    pub periodid: rust_decimal::Decimal,
    /// The type of NSCAS service. Current value values are:<br>- REACTIVE<br>- LOADSHED<br>
    pub service: String,
    /// The NMAS Contract Id
    pub contractid: String,
    /// The type of payment being recovered. Valid values are:<br>- AVAILABILITY<br>- ENABLEMENT<br>- COMPENSATION<br>
    pub paymenttype: String,
    /// The region from where the amount is recovered
    pub regionid: String,
    /// The Benefitting Factor for the RegionId
    pub rbf: Option<rust_decimal::Decimal>,
    /// The total Payment Amount to recover from all benefitting regions
    pub payment_amount: Option<rust_decimal::Decimal>,
    /// The Total recovery amount for the period for the REGIONID
    pub recovery_amount: Option<rust_decimal::Decimal>,
    /// The Last Updated date and time
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for SettlementsNmasRecoveryRbf1 {
    type PrimaryKey = SettlementsNmasRecoveryRbf1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("NMAS_RECOVERY_RBF".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementsNmasRecoveryRbf1PrimaryKey {
        SettlementsNmasRecoveryRbf1PrimaryKey {
            contractid: self.contractid.clone(),
            paymenttype: self.paymenttype.clone(),
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            service: self.service.clone(),
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_nmas_recovery_rbf_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsNmasRecoveryRbf1PrimaryKey {
    pub contractid: String,
    pub paymenttype: String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub service: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsNmasRecoveryRbf1PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsNmasRecoveryRbf1 {
    type Row = SettlementsNmasRecoveryRbf1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.paymenttype == row.paymenttype
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.service == row.service
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsNmasRecoveryRbf1 {
    type PrimaryKey = SettlementsNmasRecoveryRbf1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.paymenttype == key.paymenttype
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.service == key.service
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsNmasRecoveryRbf1PrimaryKey {
    type Row = SettlementsNmasRecoveryRbf1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.contractid == row.contractid
            && self.paymenttype == row.paymenttype
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.service == row.service
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsNmasRecoveryRbf1PrimaryKey {
    type PrimaryKey = SettlementsNmasRecoveryRbf1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.contractid == key.contractid
            && self.paymenttype == key.paymenttype
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.service == key.service
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsNmasRecoveryRbf1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("service", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "contractid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "paymenttype",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("rbf", arrow2::datatypes::DataType::Decimal(18, 8), true),
            arrow2::datatypes::Field::new(
                "payment_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "recovery_amount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut service_array = Vec::new();
        let mut contractid_array = Vec::new();
        let mut paymenttype_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut rbf_array = Vec::new();
        let mut payment_amount_array = Vec::new();
        let mut recovery_amount_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            service_array.push(row.service);
            contractid_array.push(row.contractid);
            paymenttype_array.push(row.paymenttype);
            regionid_array.push(row.regionid);
            rbf_array.push({
                row.rbf.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            payment_amount_array.push({
                row.payment_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            recovery_amount_array.push({
                row.recovery_amount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(service_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    contractid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    paymenttype_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rbf_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(payment_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(recovery_amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SET_RECOVERY_ENERGY
///  _Settlements substitution recovery energy used_
///
/// * Data Set Name: Settlements
/// * File Name: Recovery Energy
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRecoveryEnergy1 {
    /// Settlement date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// Unique identifier for the participant
    pub participantid: String,
    /// Unique Identifier for the Region to which the TNI belongs on this settlement date
    pub regionid: String,
    /// Trading interval identifier, with Period 1 being the first TI for the calendar day, i.e interval ending 00:05 for 5MS or 00:30 for 30MS.
    pub periodid: rust_decimal::Decimal,
    /// Actual Customer Demand
    pub customerenergyactual: Option<rust_decimal::Decimal>,
    /// Actual Customer Demand excluding TNIs that have a causer pays MPF
    pub customerenergympfexactual: Option<rust_decimal::Decimal>,
    /// Substitute Customer Demand
    pub customerenergysubstitute: Option<rust_decimal::Decimal>,
    /// Substitute Customer Demand excluding TNIs that have a causer pays MPF
    pub customerenergympfexsubstitute: Option<rust_decimal::Decimal>,
    /// Actual Generator Output
    pub generatorenergyactual: Option<rust_decimal::Decimal>,
    /// Region Total of Actual Customer Demand
    pub regioncustenergyactual: Option<rust_decimal::Decimal>,
    /// Region Total of Actual Customer Demand excluding TNIs that have a causer pays MPF.
    pub regioncustenergympfexactual: Option<rust_decimal::Decimal>,
    /// Region Total of Substitute Customer Demand
    pub regioncustenergysubst: Option<rust_decimal::Decimal>,
    /// Region total of Substitute Customer Demand excluding TNIs that have a causer pays MPF.
    pub regioncustenergympfexsubst: Option<rust_decimal::Decimal>,
    /// Region Total of Actual Generator Output
    pub regiongenenergyactual: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsRecoveryEnergy1 {
    type PrimaryKey = SettlementsRecoveryEnergy1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("RECOVERY_ENERGY".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementsRecoveryEnergy1PrimaryKey {
        SettlementsRecoveryEnergy1PrimaryKey {
            participantid: self.participantid.clone(),
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
            settlementrunno: self.settlementrunno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_recovery_energy_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsRecoveryEnergy1PrimaryKey {
    pub participantid: String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsRecoveryEnergy1PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsRecoveryEnergy1 {
    type Row = SettlementsRecoveryEnergy1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsRecoveryEnergy1 {
    type PrimaryKey = SettlementsRecoveryEnergy1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsRecoveryEnergy1PrimaryKey {
    type Row = SettlementsRecoveryEnergy1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsRecoveryEnergy1PrimaryKey {
    type PrimaryKey = SettlementsRecoveryEnergy1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsRecoveryEnergy1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "settlementrunno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
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
            arrow2::datatypes::Field::new(
                "customerenergyactual",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "customerenergympfexactual",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "customerenergysubstitute",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "customerenergympfexsubstitute",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "generatorenergyactual",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "regioncustenergyactual",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "regioncustenergympfexactual",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "regioncustenergysubst",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "regioncustenergympfexsubst",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "regiongenenergyactual",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut settlementrunno_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut customerenergyactual_array = Vec::new();
        let mut customerenergympfexactual_array = Vec::new();
        let mut customerenergysubstitute_array = Vec::new();
        let mut customerenergympfexsubstitute_array = Vec::new();
        let mut generatorenergyactual_array = Vec::new();
        let mut regioncustenergyactual_array = Vec::new();
        let mut regioncustenergympfexactual_array = Vec::new();
        let mut regioncustenergysubst_array = Vec::new();
        let mut regioncustenergympfexsubst_array = Vec::new();
        let mut regiongenenergyactual_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            settlementrunno_array.push({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
            participantid_array.push(row.participantid);
            regionid_array.push(row.regionid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            customerenergyactual_array.push({
                row.customerenergyactual.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            customerenergympfexactual_array.push({
                row.customerenergympfexactual.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            customerenergysubstitute_array.push({
                row.customerenergysubstitute.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            customerenergympfexsubstitute_array.push({
                row.customerenergympfexsubstitute.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            generatorenergyactual_array.push({
                row.generatorenergyactual.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            regioncustenergyactual_array.push({
                row.regioncustenergyactual.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            regioncustenergympfexactual_array.push({
                row.regioncustenergympfexactual.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            regioncustenergysubst_array.push({
                row.regioncustenergysubst.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            regioncustenergympfexsubst_array.push({
                row.regioncustenergympfexsubst.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            regiongenenergyactual_array.push({
                row.regiongenenergyactual.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementrunno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(customerenergyactual_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(customerenergympfexactual_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(customerenergysubstitute_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(customerenergympfexsubstitute_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(generatorenergyactual_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(regioncustenergyactual_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(regioncustenergympfexactual_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(regioncustenergysubst_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(regioncustenergympfexsubst_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(regiongenenergyactual_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SET_RUN_PARAMETER
///  _SET_RUN_PARAMETER shows the input parameters and value associated with each settlement run (e.g. Residual System Load Causer Pays Factor)._
///
/// * Data Set Name: Settlements
/// * File Name: Run Parameter
/// * Data Version: 5
///
/// # Description
///  Change History 19 August 2005 for 4.5.0: Changed index name again to have suffix of _LCX Note: primary key shows PK_ as prefix in Oracle SQL script, even though name of key has _PK as suffix - but cannot change since would not improve participant systems . &nbsp; 17 August 2005 for v4.5.0 Added tablespace (02) for recently added index, and gave index a better name
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * PARAMETERID
/// * SETTLEMENTDATE
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsRunParameter5 {
    /// Settlement Date (Calendar)
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement Run Number for this date
    pub versionno: i64,
    /// Parameter Identifier
    pub parameterid: String,
    /// Settlement Run Amount for the Constant Identifier
    pub numvalue: Option<rust_decimal::Decimal>,
    /// Last date the record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for SettlementsRunParameter5 {
    type PrimaryKey = SettlementsRunParameter5PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("RUN_PARAMETER".into()),
            version: 5,
        }
    }

    fn primary_key(&self) -> SettlementsRunParameter5PrimaryKey {
        SettlementsRunParameter5PrimaryKey {
            parameterid: self.parameterid.clone(),
            settlementdate: self.settlementdate,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_run_parameter_v5_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsRunParameter5PrimaryKey {
    pub parameterid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub versionno: i64,
}
impl mmsdm_core::PrimaryKey for SettlementsRunParameter5PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsRunParameter5 {
    type Row = SettlementsRunParameter5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.parameterid == row.parameterid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsRunParameter5 {
    type PrimaryKey = SettlementsRunParameter5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.parameterid == key.parameterid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsRunParameter5PrimaryKey {
    type Row = SettlementsRunParameter5;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.parameterid == row.parameterid
            && self.settlementdate == row.settlementdate
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsRunParameter5PrimaryKey {
    type PrimaryKey = SettlementsRunParameter5PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.parameterid == key.parameterid
            && self.settlementdate == key.settlementdate
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsRunParameter5 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new("versionno", arrow2::datatypes::DataType::Int64, false),
            arrow2::datatypes::Field::new(
                "parameterid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "numvalue",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lastchanged",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut parameterid_array = Vec::new();
        let mut numvalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            versionno_array.push(row.versionno);
            parameterid_array.push(row.parameterid);
            numvalue_array.push({
                row.numvalue.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_vec(versionno_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    parameterid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(numvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(lastchanged_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SET_SUBSTITUTE_DEMAND
///  _Settlements substitution demand for Zero Demand figures_
///
/// * Data Set Name: Settlements
/// * File Name: Subst Demand
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * PARTICIPANTID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
/// * TNI
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsSubstDemand1 {
    /// Settlement date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// Unique identifier for the connection point
    pub tni: String,
    /// Unique identifier for the participant
    pub participantid: String,
    /// Unique identifier for the region to which the TNI belongs to on this settlement date
    pub regionid: Option<String>,
    /// Substitute metered quantity for non-energy recovery in MWh for the TNI and participant in the trading interval. A negative value indicates net consumption and a positive value indicates net generation
    pub substitutedemand: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsSubstDemand1 {
    type PrimaryKey = SettlementsSubstDemand1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("SUBST_DEMAND".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementsSubstDemand1PrimaryKey {
        SettlementsSubstDemand1PrimaryKey {
            participantid: self.participantid.clone(),
            settlementdate: self.settlementdate,
            settlementrunno: self.settlementrunno,
            tni: self.tni.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_subst_demand_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsSubstDemand1PrimaryKey {
    pub participantid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
    pub tni: String,
}
impl mmsdm_core::PrimaryKey for SettlementsSubstDemand1PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsSubstDemand1 {
    type Row = SettlementsSubstDemand1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
            && self.tni == row.tni
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsSubstDemand1 {
    type PrimaryKey = SettlementsSubstDemand1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
            && self.tni == key.tni
    }
}
impl mmsdm_core::CompareWithRow for SettlementsSubstDemand1PrimaryKey {
    type Row = SettlementsSubstDemand1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.participantid == row.participantid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
            && self.tni == row.tni
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsSubstDemand1PrimaryKey {
    type PrimaryKey = SettlementsSubstDemand1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.participantid == key.participantid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
            && self.tni == key.tni
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsSubstDemand1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "settlementrunno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("tni", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "substitutedemand",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut settlementrunno_array = Vec::new();
        let mut tni_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut substitutedemand_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            settlementrunno_array.push({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
            tni_array.push(row.tni);
            participantid_array.push(row.participantid);
            regionid_array.push(row.regionid);
            substitutedemand_array.push({
                row.substitutedemand.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementrunno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(tni_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(substitutedemand_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SET_SUBST_RUN_VERSION
///  _Settlements substitution demand run version numbers_
///
/// * Data Set Name: Settlements
/// * File Name: Subst Run Version
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * REFERENCESETTLEMENTDATE
/// * REFERENCESETTLEMENTRUNNO
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsSubstRunVersion1 {
    /// Settlement date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// The settlement date of a settlement run included in the reference period
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub referencesettlementdate: chrono::NaiveDateTime,
    /// The settlement run number matching the settlement date for a settlement run included in the reference period
    pub referencesettlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::GetTable for SettlementsSubstRunVersion1 {
    type PrimaryKey = SettlementsSubstRunVersion1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("SUBST_RUN_VERSION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementsSubstRunVersion1PrimaryKey {
        SettlementsSubstRunVersion1PrimaryKey {
            referencesettlementdate: self.referencesettlementdate,
            referencesettlementrunno: self.referencesettlementrunno,
            settlementdate: self.settlementdate,
            settlementrunno: self.settlementrunno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_subst_run_version_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsSubstRunVersion1PrimaryKey {
    pub referencesettlementdate: chrono::NaiveDateTime,
    pub referencesettlementrunno: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsSubstRunVersion1PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsSubstRunVersion1 {
    type Row = SettlementsSubstRunVersion1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.referencesettlementdate == row.referencesettlementdate
            && self.referencesettlementrunno == row.referencesettlementrunno
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsSubstRunVersion1 {
    type PrimaryKey = SettlementsSubstRunVersion1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.referencesettlementdate == key.referencesettlementdate
            && self.referencesettlementrunno == key.referencesettlementrunno
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsSubstRunVersion1PrimaryKey {
    type Row = SettlementsSubstRunVersion1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.referencesettlementdate == row.referencesettlementdate
            && self.referencesettlementrunno == row.referencesettlementrunno
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsSubstRunVersion1PrimaryKey {
    type PrimaryKey = SettlementsSubstRunVersion1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.referencesettlementdate == key.referencesettlementdate
            && self.referencesettlementrunno == key.referencesettlementrunno
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsSubstRunVersion1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "settlementrunno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "referencesettlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "referencesettlementrunno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut settlementrunno_array = Vec::new();
        let mut referencesettlementdate_array = Vec::new();
        let mut referencesettlementrunno_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            settlementrunno_array.push({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
            referencesettlementdate_array.push(row.referencesettlementdate.timestamp());
            referencesettlementrunno_array.push({
                let mut val = row.referencesettlementrunno;
                val.rescale(0);
                val.mantissa()
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementrunno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(referencesettlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(referencesettlementrunno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SET_WDR_RECON_DETAIL
///  _Settlements WDR reconciliation details_
///
/// * Data Set Name: Settlements
/// * File Name: Wdr Recon Detail
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * NMI
/// * PERIODID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsWdrReconDetail1 {
    /// Settlement date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// Unique identifier for the meter to which the metering records applies
    pub nmi: String,
    /// Unique identifier for the transmission node to which this meter belongs on the settlement date
    pub tni: Option<String>,
    /// Unique identifier for the region to which the TNI belongs on the settlement date
    pub regionid: Option<String>,
    /// Unique identifier for the participant acting as the FRMP for this NMI on the settlement date
    pub frmp: Option<String>,
    /// Unique identifier for the participant acting as the DRSP for this NMI on the settlement date
    pub drsp: Option<String>,
    /// Trading interval identifier with Period 1 being the first TI for the calendar day, that is the interval ending 00:05
    pub periodid: rust_decimal::Decimal,
    /// WDR settlement quantity before any capping or flooring (MWh)
    pub wdrsq_uncapped: Option<rust_decimal::Decimal>,
    /// WDR settlement quantity after capping or flooring (MWh)
    pub wdrsq_capped: Option<rust_decimal::Decimal>,
    /// Maximum responsive component for the NMI (MW)
    pub mrc: Option<rust_decimal::Decimal>,
    /// Maximum responsive component settlement quantity for the NMI (MWh)
    pub mrcsq: Option<rust_decimal::Decimal>,
    /// WDR reimbursement rate for the region ($/MWh)
    pub wdrrr: Option<rust_decimal::Decimal>,
    /// Regional reference price for the region in the settlement interval ($/MWh)
    pub rrp: Option<rust_decimal::Decimal>,
    /// Transmission loss factor for the wholesale connection point associated with the NMI
    pub tlf: Option<rust_decimal::Decimal>,
    /// Metered quantity in MWh for the NMI trading interval. A negative value indicates net consumption and a positive value indicates net generation
    pub me_dlfadjusted: Option<rust_decimal::Decimal>,
    /// Baseline quantity in MWh for the NMI in the trading interval. A negative quantity indicates net consumption, while a positive value indicates net generation
    pub bq_dlfadjusted: Option<rust_decimal::Decimal>,
    /// A value of TRUE (indicated by 1) for this column indicates that financial settlement of WDR transactions for this NMI should not proceed for the settlement date and trading interval. Possible values are 1 and 0.
    pub isnoncompliant: Option<rust_decimal::Decimal>,
    /// Quality flag for the meter read. Where multiple datastreams exist against the NMI with different quality flags for each read, the lowest quality flag will be published against the NMI for the interval
    pub qualityflag: Option<String>,
    /// WDR transaction amount for this NMI in the settlement interval ($)
    pub transactionamount: Option<rust_decimal::Decimal>,
    /// A reference to the baseline run that produced the baseline quantity for this NMI and interval
    pub baselinecalculationid: Option<String>,
}
impl mmsdm_core::GetTable for SettlementsWdrReconDetail1 {
    type PrimaryKey = SettlementsWdrReconDetail1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("WDR_RECON_DETAIL".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementsWdrReconDetail1PrimaryKey {
        SettlementsWdrReconDetail1PrimaryKey {
            nmi: self.nmi.clone(),
            periodid: self.periodid,
            settlementdate: self.settlementdate,
            settlementrunno: self.settlementrunno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_wdr_recon_detail_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsWdrReconDetail1PrimaryKey {
    pub nmi: String,
    pub periodid: rust_decimal::Decimal,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsWdrReconDetail1PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsWdrReconDetail1 {
    type Row = SettlementsWdrReconDetail1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.nmi == row.nmi
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsWdrReconDetail1 {
    type PrimaryKey = SettlementsWdrReconDetail1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.nmi == key.nmi
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsWdrReconDetail1PrimaryKey {
    type Row = SettlementsWdrReconDetail1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.nmi == row.nmi
            && self.periodid == row.periodid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsWdrReconDetail1PrimaryKey {
    type PrimaryKey = SettlementsWdrReconDetail1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.nmi == key.nmi
            && self.periodid == key.periodid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsWdrReconDetail1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "settlementrunno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("nmi", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("tni", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("frmp", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("drsp", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "wdrsq_uncapped",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "wdrsq_capped",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new("mrc", arrow2::datatypes::DataType::Decimal(18, 8), true),
            arrow2::datatypes::Field::new(
                "mrcsq",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "wdrrr",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new("rrp", arrow2::datatypes::DataType::Decimal(18, 8), true),
            arrow2::datatypes::Field::new("tlf", arrow2::datatypes::DataType::Decimal(18, 8), true),
            arrow2::datatypes::Field::new(
                "me_dlfadjusted",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "bq_dlfadjusted",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "isnoncompliant",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "qualityflag",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "transactionamount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
            arrow2::datatypes::Field::new(
                "baselinecalculationid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut settlementrunno_array = Vec::new();
        let mut nmi_array = Vec::new();
        let mut tni_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut frmp_array = Vec::new();
        let mut drsp_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut wdrsq_uncapped_array = Vec::new();
        let mut wdrsq_capped_array = Vec::new();
        let mut mrc_array = Vec::new();
        let mut mrcsq_array = Vec::new();
        let mut wdrrr_array = Vec::new();
        let mut rrp_array = Vec::new();
        let mut tlf_array = Vec::new();
        let mut me_dlfadjusted_array = Vec::new();
        let mut bq_dlfadjusted_array = Vec::new();
        let mut isnoncompliant_array = Vec::new();
        let mut qualityflag_array = Vec::new();
        let mut transactionamount_array = Vec::new();
        let mut baselinecalculationid_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            settlementrunno_array.push({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
            nmi_array.push(row.nmi);
            tni_array.push(row.tni);
            regionid_array.push(row.regionid);
            frmp_array.push(row.frmp);
            drsp_array.push(row.drsp);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            wdrsq_uncapped_array.push({
                row.wdrsq_uncapped.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            wdrsq_capped_array.push({
                row.wdrsq_capped.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            mrc_array.push({
                row.mrc.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            mrcsq_array.push({
                row.mrcsq.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            wdrrr_array.push({
                row.wdrrr.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            rrp_array.push({
                row.rrp.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            tlf_array.push({
                row.tlf.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            me_dlfadjusted_array.push({
                row.me_dlfadjusted.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            bq_dlfadjusted_array.push({
                row.bq_dlfadjusted.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            isnoncompliant_array.push({
                row.isnoncompliant.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            qualityflag_array.push(row.qualityflag);
            transactionamount_array.push({
                row.transactionamount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
            baselinecalculationid_array.push(row.baselinecalculationid);
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementrunno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(nmi_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(tni_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(frmp_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(drsp_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(wdrsq_uncapped_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(wdrsq_capped_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mrc_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(mrcsq_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(wdrrr_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(tlf_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(me_dlfadjusted_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(bq_dlfadjusted_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(isnoncompliant_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(qualityflag_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(transactionamount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    baselinecalculationid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## SET_WDR_TRANSACT
///  _Settlements WDR transactions summary_
///
/// * Data Set Name: Settlements
/// * File Name: Wdr Transact
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Private
///
/// # Primary Key Columns
///
/// * COUNTERPARTYPARTICIPANTID
/// * PARTICIPANTID
/// * PARTICIPANTROLEID
/// * PERIODID
/// * REGIONID
/// * SETTLEMENTDATE
/// * SETTLEMENTRUNNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SettlementsWdrTransact1 {
    /// Settlement date
    #[serde(with = "mmsdm_core::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// Settlement run number
    pub settlementrunno: rust_decimal::Decimal,
    /// Trading interval identifier with Period 1 being the first TI for the calendar day, that is the interval ending 00:05
    pub periodid: rust_decimal::Decimal,
    /// Unique identifier for the region to which the TNI belongs on the settlement date
    pub regionid: String,
    /// Unique identifier for a participant
    pub participantid: String,
    /// Participant role identifier - FRMP or DRSP
    pub participantroleid: String,
    /// Unique identifier for the counter participant id.
    pub counterpartyparticipantid: String,
    /// Aggregate WDR transaction amount for the participant and counterparty in the settlement interval
    pub transactionamount: Option<rust_decimal::Decimal>,
}
impl mmsdm_core::GetTable for SettlementsWdrTransact1 {
    type PrimaryKey = SettlementsWdrTransact1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "SETTLEMENTS".into(),
            table_name: Some("WDR_TRANSACT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SettlementsWdrTransact1PrimaryKey {
        SettlementsWdrTransact1PrimaryKey {
            counterpartyparticipantid: self.counterpartyparticipantid.clone(),
            participantid: self.participantid.clone(),
            participantroleid: self.participantroleid.clone(),
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            settlementdate: self.settlementdate,
            settlementrunno: self.settlementrunno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::Datelike::year(&self.settlementdate),
            month: num_traits::FromPrimitive::from_u32(chrono::Datelike::month(
                &self.settlementdate,
            ))
            .unwrap(),
        }
    }

    fn partition_name(&self) -> String {
        format!(
            "settlements_wdr_transact_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct SettlementsWdrTransact1PrimaryKey {
    pub counterpartyparticipantid: String,
    pub participantid: String,
    pub participantroleid: String,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub settlementdate: chrono::NaiveDateTime,
    pub settlementrunno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SettlementsWdrTransact1PrimaryKey {}
impl mmsdm_core::CompareWithRow for SettlementsWdrTransact1 {
    type Row = SettlementsWdrTransact1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.counterpartyparticipantid == row.counterpartyparticipantid
            && self.participantid == row.participantid
            && self.participantroleid == row.participantroleid
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsWdrTransact1 {
    type PrimaryKey = SettlementsWdrTransact1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.counterpartyparticipantid == key.counterpartyparticipantid
            && self.participantid == key.participantid
            && self.participantroleid == key.participantroleid
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
impl mmsdm_core::CompareWithRow for SettlementsWdrTransact1PrimaryKey {
    type Row = SettlementsWdrTransact1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.counterpartyparticipantid == row.counterpartyparticipantid
            && self.participantid == row.participantid
            && self.participantroleid == row.participantroleid
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.settlementdate == row.settlementdate
            && self.settlementrunno == row.settlementrunno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SettlementsWdrTransact1PrimaryKey {
    type PrimaryKey = SettlementsWdrTransact1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.counterpartyparticipantid == key.counterpartyparticipantid
            && self.participantid == key.participantid
            && self.participantroleid == key.participantroleid
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.settlementdate == key.settlementdate
            && self.settlementrunno == key.settlementrunno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SettlementsWdrTransact1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                false,
            ),
            arrow2::datatypes::Field::new(
                "settlementrunno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "participantroleid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "counterpartyparticipantid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "transactionamount",
                arrow2::datatypes::DataType::Decimal(18, 8),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: impl Iterator<Item = Self>,
    ) -> mmsdm_core::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut settlementdate_array = Vec::new();
        let mut settlementrunno_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut participantroleid_array = Vec::new();
        let mut counterpartyparticipantid_array = Vec::new();
        let mut transactionamount_array = Vec::new();
        for row in partition {
            settlementdate_array.push(row.settlementdate.timestamp());
            settlementrunno_array.push({
                let mut val = row.settlementrunno;
                val.rescale(0);
                val.mantissa()
            });
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            regionid_array.push(row.regionid);
            participantid_array.push(row.participantid);
            participantroleid_array.push(row.participantroleid);
            counterpartyparticipantid_array.push(row.counterpartyparticipantid);
            transactionamount_array.push({
                row.transactionamount.map(|mut val| {
                    val.rescale(8);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementdate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(settlementrunno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    participantroleid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    counterpartyparticipantid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(transactionamount_array)
                        .to(arrow2::datatypes::DataType::Decimal(18, 8)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
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
        (Some("DAYTRACK"), version) if version <= 6_i32 => {
            let d: Vec<SettlementsDaytrack6> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsDaytrack6 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("CPDATA"), version) if version <= 6_i32 => {
            let d: Vec<SettlementsCpdata6> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsCpdata6 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("CPDATAREGION"), version) if version <= 5_i32 => {
            let d: Vec<SettlementsCpdataregion5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsCpdataregion5 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("FCASREGIONRECOVERY"), version) if version <= 5_i32 => {
            let d: Vec<SettlementsFcasregionrecovery5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsFcasregionrecovery5 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("GENDATA"), version) if version <= 6_i32 => {
            let d: Vec<SettlementsGendata6> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsGendata6 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("GENDATAREGION"), version) if version <= 5_i32 => {
            let d: Vec<SettlementsGendataregion5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsGendataregion5 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("INTRAREGIONRESIDUES"), version) if version <= 5_i32 => {
            let d: Vec<SettlementsIntraregionresidues5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsIntraregionresidues5 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("IRAUCSURPLUS"), version) if version <= 6_i32 => {
            let d: Vec<SettlementsIraucsurplus6> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsIraucsurplus6 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("IRNSPSURPLUS"), version) if version <= 6_i32 => {
            let d: Vec<SettlementsIrnspsurplus6> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsIrnspsurplus6 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("IRPARTSURPLUS"), version) if version <= 6_i32 => {
            let d: Vec<SettlementsIrpartsurplus6> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsIrpartsurplus6 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("IRSURPLUS"), version) if version <= 6_i32 => {
            let d: Vec<SettlementsIrsurplus6> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsIrsurplus6 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("LOCALAREAENERGY"), version) if version <= 1_i32 => {
            let d: Vec<SettlementsLocalareaenergy1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsLocalareaenergy1 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("LOCALAREATNI"), version) if version <= 1_i32 => {
            let d: Vec<SettlementsLocalareatni1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsLocalareatni1 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("LSHEDPAYMENT"), version) if version <= 5_i32 => {
            let d: Vec<SettlementsLshedpayment5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsLshedpayment5 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("MARKETFEES"), version) if version <= 6_i32 => {
            let d: Vec<SettlementsMarketfees6> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsMarketfees6 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("REALLOCATIONS"), version) if version <= 5_i32 => {
            let d: Vec<SettlementsReallocations5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsReallocations5 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("RESTARTPAYMENT"), version) if version <= 6_i32 => {
            let d: Vec<SettlementsRestartpayment6> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsRestartpayment6 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("RESTARTRECOVERY"), version) if version <= 6_i32 => {
            let d: Vec<SettlementsRestartrecovery6> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsRestartrecovery6 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("RPOWERPAYMENT"), version) if version <= 6_i32 => {
            let d: Vec<SettlementsRpowerpayment6> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsRpowerpayment6 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("SMALLGENDATA"), version) if version <= 1_i32 => {
            let d: Vec<SettlementsSmallgendata1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsSmallgendata1 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("ANCILLARY_SUMMARY"), version) if version <= 5_i32 => {
            let d: Vec<SettlementsAncillarySummary5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsAncillarySummary5 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("APC_COMPENSATION"), version) if version <= 1_i32 => {
            let d: Vec<SettlementsApcCompensation1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsApcCompensation1 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("APC_RECOVERY"), version) if version <= 1_i32 => {
            let d: Vec<SettlementsApcRecovery1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsApcRecovery1 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("FCAS_PAYMENT"), version) if version <= 5_i32 => {
            let d: Vec<SettlementsFcasPayment5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsFcasPayment5 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("FCAS_RECOVERY"), version) if version <= 6_i32 => {
            let d: Vec<SettlementsFcasRecovery6> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsFcasRecovery6 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("SET_FCAS_REGULATION_TRK"), version) if version <= 2_i32 => {
            let d: Vec<SettlementsSetFcasRegulationTrk2> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsSetFcasRegulationTrk2 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("NMAS_RECOVERY"), version) if version <= 2_i32 => {
            let d: Vec<SettlementsNmasRecovery2> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsNmasRecovery2 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("NMAS_RECOVERY_RBF"), version) if version <= 1_i32 => {
            let d: Vec<SettlementsNmasRecoveryRbf1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsNmasRecoveryRbf1 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("RECOVERY_ENERGY"), version) if version <= 1_i32 => {
            let d: Vec<SettlementsRecoveryEnergy1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsRecoveryEnergy1 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("RUN_PARAMETER"), version) if version <= 5_i32 => {
            let d: Vec<SettlementsRunParameter5> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsRunParameter5 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("SUBST_DEMAND"), version) if version <= 1_i32 => {
            let d: Vec<SettlementsSubstDemand1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsSubstDemand1 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("SUBST_RUN_VERSION"), version) if version <= 1_i32 => {
            let d: Vec<SettlementsSubstRunVersion1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsSubstRunVersion1 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("WDR_RECON_DETAIL"), version) if version <= 1_i32 => {
            let d: Vec<SettlementsWdrReconDetail1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsWdrReconDetail1 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("WDR_TRANSACT"), version) if version <= 1_i32 => {
            let d: Vec<SettlementsWdrTransact1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertSettlementsWdrTransact1 @P1, @P2",
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
