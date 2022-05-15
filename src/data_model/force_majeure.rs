/// # Summary
///
/// ## APEVENT
///  _APEVENT is the driving data defining the existence and timeframes of an administered pricing event._
///
/// * Data Set Name: Ap
/// * File Name: Apevent
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * APEVENTID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApApevent1 {
    /// Unique identifier for this administered pricing event
    pub apeventid: rust_decimal::Decimal,
    /// Date Time of the first Dispatch Interval to which the administered event applies
    #[serde(with = "crate::mms_datetime_opt")]
    pub effectivefrominterval: Option<chrono::NaiveDateTime>,
    /// Date Time of the final Dispatch Interval to which the administered event applies
    #[serde(with = "crate::mms_datetime_opt")]
    pub effectivetointerval: Option<chrono::NaiveDateTime>,
    /// Description of the driver for the Event
    pub reason: Option<String>,
    /// Authorising staff for start of AP event
    pub startauthorisedby: Option<String>,
    /// Date-Time start authorised
    #[serde(with = "crate::mms_datetime_opt")]
    pub startauthoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorising staff for end of AP event
    pub endauthorisedby: Option<String>,
    /// Date Time end authorised
    #[serde(with = "crate::mms_datetime_opt")]
    pub endauthoriseddate: Option<chrono::NaiveDateTime>,
    /// Date-Time the record was last modified
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ApApevent1 {
    type PrimaryKey = ApApevent1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "AP".into(),
            table_name: Some("APEVENT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ApApevent1PrimaryKey {
        ApApevent1PrimaryKey {
            apeventid: self.apeventid,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "ap_apevent_v1".to_string()
    }
}
impl crate::CompareWithRow for ApApevent1 {
    type Row = ApApevent1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.apeventid == row.apeventid
    }
}
impl crate::CompareWithPrimaryKey for ApApevent1 {
    type PrimaryKey = ApApevent1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ApApevent1PrimaryKey {
    pub apeventid: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ApApevent1PrimaryKey {
    type Row = ApApevent1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.apeventid == row.apeventid
    }
}
impl crate::CompareWithPrimaryKey for ApApevent1PrimaryKey {
    type PrimaryKey = ApApevent1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid
    }
}
impl crate::PrimaryKey for ApApevent1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ApApevent1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "apeventid",
                arrow2::datatypes::DataType::Decimal(22, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivefrominterval",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "effectivetointerval",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new("reason", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "startauthorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "startauthoriseddate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "endauthorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "endauthoriseddate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut apeventid_array = Vec::new();
        let mut effectivefrominterval_array = Vec::new();
        let mut effectivetointerval_array = Vec::new();
        let mut reason_array = Vec::new();
        let mut startauthorisedby_array = Vec::new();
        let mut startauthoriseddate_array = Vec::new();
        let mut endauthorisedby_array = Vec::new();
        let mut endauthoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            apeventid_array.push({
                let mut val = row.apeventid;
                val.rescale(0);
                val.mantissa()
            });
            effectivefrominterval_array
                .push(row.effectivefrominterval.map(|val| val.timestamp_millis()));
            effectivetointerval_array
                .push(row.effectivetointerval.map(|val| val.timestamp_millis()));
            reason_array.push(row.reason);
            startauthorisedby_array.push(row.startauthorisedby);
            startauthoriseddate_array
                .push(row.startauthoriseddate.map(|val| val.timestamp_millis()));
            endauthorisedby_array.push(row.endauthorisedby);
            endauthoriseddate_array.push(row.endauthoriseddate.map(|val| val.timestamp_millis()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(apeventid_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(effectivefrominterval_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(effectivetointerval_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(reason_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    startauthorisedby_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startauthoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(endauthorisedby_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(endauthoriseddate_array)
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
/// ## APEVENTREGION
///  _APEVENTREGION is the Region detail for an administered pricing event defined through APEVENT._
///
/// * Data Set Name: Ap
/// * File Name: Apeventregion
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * APEVENTID
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApApeventregion1 {
    /// Unique identifier for this administered pricing event
    pub apeventid: rust_decimal::Decimal,
    /// Date-Time of the first Dispatch Interval to which the administered event applies
    pub regionid: String,
    /// Date Time of the final Dispatch Interval to which the administered event applies
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// flag indicating if the apevent covers an energy AP
    pub energyapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a raise6sec AP
    pub raise6secapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a raise60sec AP
    pub raise60secapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a raise5min AP
    pub raise5minapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a raisereg AP
    pub raiseregapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a lower6sec AP
    pub lower6secapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a lower60sec AP<br>flag indicating if the apevent covers a lower5min AP<br>flag indicating if the apevent covers a lowerreg AP<br>flag indicating if the apevent covers a lower60sec AP
    pub lower60secapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a lower5min AP
    pub lower5minapflag: Option<rust_decimal::Decimal>,
    /// flag indicating if the apevent covers a lowerreg AP
    pub lowerregapflag: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for ApApeventregion1 {
    type PrimaryKey = ApApeventregion1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "AP".into(),
            table_name: Some("APEVENTREGION".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ApApeventregion1PrimaryKey {
        ApApeventregion1PrimaryKey {
            apeventid: self.apeventid,
            regionid: self.regionid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "ap_apeventregion_v1".to_string()
    }
}
impl crate::CompareWithRow for ApApeventregion1 {
    type Row = ApApeventregion1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.apeventid == row.apeventid && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for ApApeventregion1 {
    type PrimaryKey = ApApeventregion1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid && self.regionid == key.regionid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ApApeventregion1PrimaryKey {
    pub apeventid: rust_decimal::Decimal,
    pub regionid: String,
}
impl crate::CompareWithRow for ApApeventregion1PrimaryKey {
    type Row = ApApeventregion1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.apeventid == row.apeventid && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for ApApeventregion1PrimaryKey {
    type PrimaryKey = ApApeventregion1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.apeventid == key.apeventid && self.regionid == key.regionid
    }
}
impl crate::PrimaryKey for ApApeventregion1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ApApeventregion1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "apeventid",
                arrow2::datatypes::DataType::Decimal(22, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "energyapflag",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise6secapflag",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise60secapflag",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raise5minapflag",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "raiseregapflag",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower6secapflag",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower60secapflag",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lower5minapflag",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lowerregapflag",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut apeventid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut energyapflag_array = Vec::new();
        let mut raise6secapflag_array = Vec::new();
        let mut raise60secapflag_array = Vec::new();
        let mut raise5minapflag_array = Vec::new();
        let mut raiseregapflag_array = Vec::new();
        let mut lower6secapflag_array = Vec::new();
        let mut lower60secapflag_array = Vec::new();
        let mut lower5minapflag_array = Vec::new();
        let mut lowerregapflag_array = Vec::new();
        for (_, row) in partition {
            apeventid_array.push({
                let mut val = row.apeventid;
                val.rescale(0);
                val.mantissa()
            });
            regionid_array.push(row.regionid);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            energyapflag_array.push({
                row.energyapflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            raise6secapflag_array.push({
                row.raise6secapflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            raise60secapflag_array.push({
                row.raise60secapflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            raise5minapflag_array.push({
                row.raise5minapflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            raiseregapflag_array.push({
                row.raiseregapflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lower6secapflag_array.push({
                row.lower6secapflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lower60secapflag_array.push({
                row.lower60secapflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lower5minapflag_array.push({
                row.lower5minapflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lowerregapflag_array.push({
                row.lowerregapflag.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(apeventid_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(energyapflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise6secapflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise60secapflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raise5minapflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(raiseregapflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower6secapflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower60secapflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lower5minapflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lowerregapflag_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## IRFMAMOUNT
///  _IRFMAMOUNT sets out settlement amounts associated with Industrial Relations Forced Majeure events._
///
/// * Data Set Name: Force Majeure
/// * File Name: Irfmamount
/// * Data Version: 1
///
/// # Description
///  IRFMAMOUNTis public data. Source IRFMAMOUNT is obsolete; was updated with each settlement run as required.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * IRFMID
/// * PERIODID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureIrfmamount1 {
    /// Unique Industrial Relations Force Majeure event
    pub irfmid: String,
    /// Date of event
    #[serde(with = "crate::mms_datetime_opt")]
    pub effectivedate: Option<chrono::NaiveDateTime>,
    /// Version number of record of event
    pub versionno: rust_decimal::Decimal,
    /// Settlement period
    pub periodid: rust_decimal::Decimal,
    /// Total settlement amount in $
    pub amount: Option<rust_decimal::Decimal>,
    /// Person authorising amount
    pub authorisedby: Option<String>,
    /// Authorised date
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// last changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ForceMajeureIrfmamount1 {
    type PrimaryKey = ForceMajeureIrfmamount1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: Some("IRFMAMOUNT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ForceMajeureIrfmamount1PrimaryKey {
        ForceMajeureIrfmamount1PrimaryKey {
            irfmid: self.irfmid.clone(),
            periodid: self.periodid,
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "force_majeure_irfmamount_v1".to_string()
    }
}
impl crate::CompareWithRow for ForceMajeureIrfmamount1 {
    type Row = ForceMajeureIrfmamount1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.irfmid == row.irfmid
            && self.periodid == row.periodid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ForceMajeureIrfmamount1 {
    type PrimaryKey = ForceMajeureIrfmamount1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.irfmid == key.irfmid
            && self.periodid == key.periodid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ForceMajeureIrfmamount1PrimaryKey {
    pub irfmid: String,
    pub periodid: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ForceMajeureIrfmamount1PrimaryKey {
    type Row = ForceMajeureIrfmamount1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.irfmid == row.irfmid
            && self.periodid == row.periodid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ForceMajeureIrfmamount1PrimaryKey {
    type PrimaryKey = ForceMajeureIrfmamount1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.irfmid == key.irfmid
            && self.periodid == key.periodid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for ForceMajeureIrfmamount1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ForceMajeureIrfmamount1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("irfmid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(4, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "amount",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
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
        let mut irfmid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut amount_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            irfmid_array.push(row.irfmid);
            effectivedate_array.push(row.effectivedate.map(|val| val.timestamp_millis()));
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
            amount_array.push({
                row.amount.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp_millis()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(irfmid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(amount_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
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
/// ## IRFMEVENTS
///  _IRFMEVENTS sets out specific Industrial Relations Forced Majeure events._
///
/// * Data Set Name: Force Majeure
/// * File Name: Irfmevents
/// * Data Version: 1
///
/// # Description
///  IRFMEVENTS is public data. Source IRFMEVENTS updates with the occurrence of any such events.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * IRFMID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureIrfmevents1 {
    /// &nbsp;
    pub irfmid: String,
    /// &nbsp;
    #[serde(with = "crate::mms_datetime_opt")]
    pub startdate: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub startperiod: Option<rust_decimal::Decimal>,
    /// &nbsp;
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// &nbsp;
    pub endperiod: Option<rust_decimal::Decimal>,
    /// &nbsp;
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ForceMajeureIrfmevents1 {
    type PrimaryKey = ForceMajeureIrfmevents1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: Some("IRFMEVENTS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ForceMajeureIrfmevents1PrimaryKey {
        ForceMajeureIrfmevents1PrimaryKey {
            irfmid: self.irfmid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "force_majeure_irfmevents_v1".to_string()
    }
}
impl crate::CompareWithRow for ForceMajeureIrfmevents1 {
    type Row = ForceMajeureIrfmevents1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.irfmid == row.irfmid
    }
}
impl crate::CompareWithPrimaryKey for ForceMajeureIrfmevents1 {
    type PrimaryKey = ForceMajeureIrfmevents1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.irfmid == key.irfmid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ForceMajeureIrfmevents1PrimaryKey {
    pub irfmid: String,
}
impl crate::CompareWithRow for ForceMajeureIrfmevents1PrimaryKey {
    type Row = ForceMajeureIrfmevents1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.irfmid == row.irfmid
    }
}
impl crate::CompareWithPrimaryKey for ForceMajeureIrfmevents1PrimaryKey {
    type PrimaryKey = ForceMajeureIrfmevents1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.irfmid == key.irfmid
    }
}
impl crate::PrimaryKey for ForceMajeureIrfmevents1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ForceMajeureIrfmevents1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("irfmid", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "startperiod",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "endperiod",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut irfmid_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut startperiod_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut endperiod_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            irfmid_array.push(row.irfmid);
            startdate_array.push(row.startdate.map(|val| val.timestamp_millis()));
            startperiod_array.push({
                row.startperiod.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            enddate_array.push(row.enddate.map(|val| val.timestamp_millis()));
            endperiod_array.push({
                row.endperiod.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(irfmid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startperiod_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enddate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(endperiod_array)
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
/// # Summary
///
/// ## MARKET_SUSPEND_REGIME_SUM
///  _Tracks the evolution of pricing regimes applied to the suspended region and from which Dispatch Interval_
///
/// * Data Set Name: Force Majeure
/// * File Name: Market Suspend Regime Sum
/// * Data Version: 1
///
/// # Description
///  MARKET_SUSPEND_REGIME_SUM is public data, so is available to all participants.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * REGIONID
/// * START_INTERVAL
/// * SUSPENSION_ID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureMarketSuspendRegimeSum1 {
    /// Unique identifier for this suspension event
    pub suspension_id: String,
    /// Region(s) covered by this evolution of the event
    pub regionid: String,
    /// First Dispatch interval from which this regime applies
    #[serde(with = "crate::mms_datetime")]
    pub start_interval: chrono::NaiveDateTime,
    /// Last Dispatch interval for which this regime applies
    #[serde(with = "crate::mms_datetime_opt")]
    pub end_interval: Option<chrono::NaiveDateTime>,
    /// Pricing Regime applied
    pub pricing_regime: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ForceMajeureMarketSuspendRegimeSum1 {
    type PrimaryKey = ForceMajeureMarketSuspendRegimeSum1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: Some("MARKET_SUSPEND_REGIME_SUM".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ForceMajeureMarketSuspendRegimeSum1PrimaryKey {
        ForceMajeureMarketSuspendRegimeSum1PrimaryKey {
            regionid: self.regionid.clone(),
            start_interval: self.start_interval,
            suspension_id: self.suspension_id.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "force_majeure_market_suspend_regime_sum_v1".to_string()
    }
}
impl crate::CompareWithRow for ForceMajeureMarketSuspendRegimeSum1 {
    type Row = ForceMajeureMarketSuspendRegimeSum1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.regionid == row.regionid
            && self.start_interval == row.start_interval
            && self.suspension_id == row.suspension_id
    }
}
impl crate::CompareWithPrimaryKey for ForceMajeureMarketSuspendRegimeSum1 {
    type PrimaryKey = ForceMajeureMarketSuspendRegimeSum1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid
            && self.start_interval == key.start_interval
            && self.suspension_id == key.suspension_id
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ForceMajeureMarketSuspendRegimeSum1PrimaryKey {
    pub regionid: String,
    pub start_interval: chrono::NaiveDateTime,
    pub suspension_id: String,
}
impl crate::CompareWithRow for ForceMajeureMarketSuspendRegimeSum1PrimaryKey {
    type Row = ForceMajeureMarketSuspendRegimeSum1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.regionid == row.regionid
            && self.start_interval == row.start_interval
            && self.suspension_id == row.suspension_id
    }
}
impl crate::CompareWithPrimaryKey for ForceMajeureMarketSuspendRegimeSum1PrimaryKey {
    type PrimaryKey = ForceMajeureMarketSuspendRegimeSum1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid
            && self.start_interval == key.start_interval
            && self.suspension_id == key.suspension_id
    }
}
impl crate::PrimaryKey for ForceMajeureMarketSuspendRegimeSum1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ForceMajeureMarketSuspendRegimeSum1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "suspension_id",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "start_interval",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "end_interval",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "pricing_regime",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut suspension_id_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut start_interval_array = Vec::new();
        let mut end_interval_array = Vec::new();
        let mut pricing_regime_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            suspension_id_array.push(row.suspension_id);
            regionid_array.push(row.regionid);
            start_interval_array.push(row.start_interval.timestamp_millis());
            end_interval_array.push(row.end_interval.map(|val| val.timestamp_millis()));
            pricing_regime_array.push(row.pricing_regime);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    suspension_id_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(start_interval_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(end_interval_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(pricing_regime_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
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
/// ## MARKET_SUSPEND_REGION_SUM
///  _Summary of Market Suspension timings_
///
/// * Data Set Name: Force Majeure
/// * File Name: Market Suspend Region Sum
/// * Data Version: 1
///
/// # Description
///  MARKET_SUSPEND is public data, so is available to all participants.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * REGIONID
/// * SUSPENSION_ID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureMarketSuspendRegionSum1 {
    /// Unique identifier for this suspension event
    pub suspension_id: String,
    /// Region(s) covered by the Suspension event
    pub regionid: String,
    /// Initial interval of the Suspension event
    #[serde(with = "crate::mms_datetime_opt")]
    pub initial_interval: Option<chrono::NaiveDateTime>,
    /// Last Dispatch interval for the Suspension event for this Region
    #[serde(with = "crate::mms_datetime_opt")]
    pub end_region_interval: Option<chrono::NaiveDateTime>,
    /// Last Dispatch interval for the Suspension event
    #[serde(with = "crate::mms_datetime_opt")]
    pub end_suspension_interval: Option<chrono::NaiveDateTime>,
    /// Last DateTime the Suspension was administered
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ForceMajeureMarketSuspendRegionSum1 {
    type PrimaryKey = ForceMajeureMarketSuspendRegionSum1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: Some("MARKET_SUSPEND_REGION_SUM".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ForceMajeureMarketSuspendRegionSum1PrimaryKey {
        ForceMajeureMarketSuspendRegionSum1PrimaryKey {
            regionid: self.regionid.clone(),
            suspension_id: self.suspension_id.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "force_majeure_market_suspend_region_sum_v1".to_string()
    }
}
impl crate::CompareWithRow for ForceMajeureMarketSuspendRegionSum1 {
    type Row = ForceMajeureMarketSuspendRegionSum1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.regionid == row.regionid && self.suspension_id == row.suspension_id
    }
}
impl crate::CompareWithPrimaryKey for ForceMajeureMarketSuspendRegionSum1 {
    type PrimaryKey = ForceMajeureMarketSuspendRegionSum1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid && self.suspension_id == key.suspension_id
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ForceMajeureMarketSuspendRegionSum1PrimaryKey {
    pub regionid: String,
    pub suspension_id: String,
}
impl crate::CompareWithRow for ForceMajeureMarketSuspendRegionSum1PrimaryKey {
    type Row = ForceMajeureMarketSuspendRegionSum1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.regionid == row.regionid && self.suspension_id == row.suspension_id
    }
}
impl crate::CompareWithPrimaryKey for ForceMajeureMarketSuspendRegionSum1PrimaryKey {
    type PrimaryKey = ForceMajeureMarketSuspendRegionSum1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid && self.suspension_id == key.suspension_id
    }
}
impl crate::PrimaryKey for ForceMajeureMarketSuspendRegionSum1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ForceMajeureMarketSuspendRegionSum1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "suspension_id",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "initial_interval",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "end_region_interval",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "end_suspension_interval",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut suspension_id_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut initial_interval_array = Vec::new();
        let mut end_region_interval_array = Vec::new();
        let mut end_suspension_interval_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            suspension_id_array.push(row.suspension_id);
            regionid_array.push(row.regionid);
            initial_interval_array.push(row.initial_interval.map(|val| val.timestamp_millis()));
            end_region_interval_array
                .push(row.end_region_interval.map(|val| val.timestamp_millis()));
            end_suspension_interval_array.push(
                row.end_suspension_interval
                    .map(|val| val.timestamp_millis()),
            );
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    suspension_id_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(initial_interval_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(end_region_interval_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(end_suspension_interval_array)
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
/// ## MARKET_SUSPEND_SCHEDULE
///  _Trading prices that will apply in the event of a market suspension event updated weekly._
///
/// * Data Set Name: Force Majeure
/// * File Name: Market Suspend Schedule
/// * Data Version: 1
///
/// # Description
///  MARKET_SUSPEND_SCHEDULE is public data, so is available to all participants.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * DAY_TYPE
/// * EFFECTIVEDATE
/// * PERIODID
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureMarketSuspendSchedule1 {
    /// Calendar date from when this record set is effective
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Distinguishes which record set to apply - at time of writing this was Business or Non-business day but may change in the future depending on outcome of consultation
    pub day_type: String,
    /// Region affected.
    pub regionid: String,
    /// 48 intervals for a day, midnight base (equates to 00:30 - 00:00)
    pub periodid: rust_decimal::Decimal,
    /// Energy Price applied for this period for this Day Type
    pub energy_rrp: Option<rust_decimal::Decimal>,
    /// Raise 6Sec contingency Price applied for this period for this Day Type
    pub r6_rrp: Option<rust_decimal::Decimal>,
    /// Raise 60Sec contingency Price applied for this period for this Day Type
    pub r60_rrp: Option<rust_decimal::Decimal>,
    /// Raise 5Min contingency Price applied for this period for this Day Type
    pub r5_rrp: Option<rust_decimal::Decimal>,
    /// Raise Regulation contingency Price applied for this period for this Day Type
    pub rreg_rrp: Option<rust_decimal::Decimal>,
    /// Lower 6Sec contingency Price applied for this period for this Day Type
    pub l6_rrp: Option<rust_decimal::Decimal>,
    /// Lower 60Sec contingency Price applied for this period for this Day Type
    pub l60_rrp: Option<rust_decimal::Decimal>,
    /// Lower 5Min contingency Price applied for this period for this Day Type
    pub l5_rrp: Option<rust_decimal::Decimal>,
    /// Lower Regulation Price applied for this period for this Day Type
    pub lreg_rrp: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ForceMajeureMarketSuspendSchedule1 {
    type PrimaryKey = ForceMajeureMarketSuspendSchedule1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: Some("MARKET_SUSPEND_SCHEDULE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ForceMajeureMarketSuspendSchedule1PrimaryKey {
        ForceMajeureMarketSuspendSchedule1PrimaryKey {
            day_type: self.day_type.clone(),
            effectivedate: self.effectivedate,
            periodid: self.periodid,
            regionid: self.regionid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "force_majeure_market_suspend_schedule_v1".to_string()
    }
}
impl crate::CompareWithRow for ForceMajeureMarketSuspendSchedule1 {
    type Row = ForceMajeureMarketSuspendSchedule1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.day_type == row.day_type
            && self.effectivedate == row.effectivedate
            && self.periodid == row.periodid
            && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for ForceMajeureMarketSuspendSchedule1 {
    type PrimaryKey = ForceMajeureMarketSuspendSchedule1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day_type == key.day_type
            && self.effectivedate == key.effectivedate
            && self.periodid == key.periodid
            && self.regionid == key.regionid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ForceMajeureMarketSuspendSchedule1PrimaryKey {
    pub day_type: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
}
impl crate::CompareWithRow for ForceMajeureMarketSuspendSchedule1PrimaryKey {
    type Row = ForceMajeureMarketSuspendSchedule1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.day_type == row.day_type
            && self.effectivedate == row.effectivedate
            && self.periodid == row.periodid
            && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for ForceMajeureMarketSuspendSchedule1PrimaryKey {
    type PrimaryKey = ForceMajeureMarketSuspendSchedule1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.day_type == key.day_type
            && self.effectivedate == key.effectivedate
            && self.periodid == key.periodid
            && self.regionid == key.regionid
    }
}
impl crate::PrimaryKey for ForceMajeureMarketSuspendSchedule1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ForceMajeureMarketSuspendSchedule1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "day_type",
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
                "energy_rrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "r6_rrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "r60_rrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "r5_rrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "rreg_rrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "l6_rrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "l60_rrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "l5_rrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new(
                "lreg_rrp",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut effectivedate_array = Vec::new();
        let mut day_type_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut energy_rrp_array = Vec::new();
        let mut r6_rrp_array = Vec::new();
        let mut r60_rrp_array = Vec::new();
        let mut r5_rrp_array = Vec::new();
        let mut rreg_rrp_array = Vec::new();
        let mut l6_rrp_array = Vec::new();
        let mut l60_rrp_array = Vec::new();
        let mut l5_rrp_array = Vec::new();
        let mut lreg_rrp_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            day_type_array.push(row.day_type);
            regionid_array.push(row.regionid);
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            energy_rrp_array.push({
                row.energy_rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            r6_rrp_array.push({
                row.r6_rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            r60_rrp_array.push({
                row.r60_rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            r5_rrp_array.push({
                row.r5_rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            rreg_rrp_array.push({
                row.rreg_rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            l6_rrp_array.push({
                row.l6_rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            l60_rrp_array.push({
                row.l60_rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            l5_rrp_array.push({
                row.l5_rrp.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lreg_rrp_array.push({
                row.lreg_rrp.map(|mut val| {
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
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(day_type_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(energy_rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(r6_rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(r60_rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(r5_rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rreg_rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(l6_rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(l60_rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(l5_rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lreg_rrp_array)
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
/// ## MARKET_SUSPEND_SCHEDULE_TRK
///  _Parent table for pricing regimes used in suspensions_
///
/// * Data Set Name: Force Majeure
/// * File Name: Market Suspend Schedule Trk
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureMarketSuspendScheduleTrk1 {
    /// Calendar date from when this record set is effective
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Start Date of the date range for the source data
    #[serde(with = "crate::mms_datetime_opt")]
    pub source_start_date: Option<chrono::NaiveDateTime>,
    /// End Date of the date range for the source data
    #[serde(with = "crate::mms_datetime_opt")]
    pub source_end_date: Option<chrono::NaiveDateTime>,
    /// Reason why this regime was applied
    pub comments: Option<String>,
    /// DateTime this record set was loaded
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ForceMajeureMarketSuspendScheduleTrk1 {
    type PrimaryKey = ForceMajeureMarketSuspendScheduleTrk1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: Some("MARKET_SUSPEND_SCHEDULE_TRK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ForceMajeureMarketSuspendScheduleTrk1PrimaryKey {
        ForceMajeureMarketSuspendScheduleTrk1PrimaryKey {
            effectivedate: self.effectivedate,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "force_majeure_market_suspend_schedule_trk_v1".to_string()
    }
}
impl crate::CompareWithRow for ForceMajeureMarketSuspendScheduleTrk1 {
    type Row = ForceMajeureMarketSuspendScheduleTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
    }
}
impl crate::CompareWithPrimaryKey for ForceMajeureMarketSuspendScheduleTrk1 {
    type PrimaryKey = ForceMajeureMarketSuspendScheduleTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ForceMajeureMarketSuspendScheduleTrk1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for ForceMajeureMarketSuspendScheduleTrk1PrimaryKey {
    type Row = ForceMajeureMarketSuspendScheduleTrk1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
    }
}
impl crate::CompareWithPrimaryKey for ForceMajeureMarketSuspendScheduleTrk1PrimaryKey {
    type PrimaryKey = ForceMajeureMarketSuspendScheduleTrk1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
    }
}
impl crate::PrimaryKey for ForceMajeureMarketSuspendScheduleTrk1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ForceMajeureMarketSuspendScheduleTrk1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new(
                "source_start_date",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "source_end_date",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new("comments", arrow2::datatypes::DataType::LargeUtf8, true),
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
        let mut effectivedate_array = Vec::new();
        let mut source_start_date_array = Vec::new();
        let mut source_end_date_array = Vec::new();
        let mut comments_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            source_start_date_array.push(row.source_start_date.map(|val| val.timestamp_millis()));
            source_end_date_array.push(row.source_end_date.map(|val| val.timestamp_millis()));
            comments_array.push(row.comments);
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp_millis()));
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
                    arrow2::array::PrimitiveArray::from(source_start_date_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(source_end_date_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(comments_array))
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
/// ## OVERRIDERRP
///  _OVERRIDERRP shows details of override price periods._
///
/// * Data Set Name: Force Majeure
/// * File Name: Overriderrp
/// * Data Version: 1
///
/// # Description
///  OVERRIDERRP data is public, so is available to all participants. Source OVERRIDERRP updates every five minutes when override prices apply for the period.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * REGIONID
/// * STARTDATE
/// * STARTPERIOD
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ForceMajeureOverriderrp1 {
    /// Region Identifier
    pub regionid: String,
    /// Starting date of override
    #[serde(with = "crate::mms_datetime")]
    pub startdate: chrono::NaiveDateTime,
    /// Starting period of override
    pub startperiod: rust_decimal::Decimal,
    /// Termination date of override
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Terminate period of override
    pub endperiod: Option<rust_decimal::Decimal>,
    /// Dispatch Price
    pub rrp: Option<rust_decimal::Decimal>,
    /// Description of reason for override
    pub description: Option<String>,
    /// Authorise Start of Override
    pub authorisestart: Option<String>,
    /// Authorise End of Override
    pub authoriseend: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ForceMajeureOverriderrp1 {
    type PrimaryKey = ForceMajeureOverriderrp1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "FORCE_MAJEURE".into(),
            table_name: Some("OVERRIDERRP".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ForceMajeureOverriderrp1PrimaryKey {
        ForceMajeureOverriderrp1PrimaryKey {
            regionid: self.regionid.clone(),
            startdate: self.startdate,
            startperiod: self.startperiod,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "force_majeure_overriderrp_v1".to_string()
    }
}
impl crate::CompareWithRow for ForceMajeureOverriderrp1 {
    type Row = ForceMajeureOverriderrp1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.regionid == row.regionid
            && self.startdate == row.startdate
            && self.startperiod == row.startperiod
    }
}
impl crate::CompareWithPrimaryKey for ForceMajeureOverriderrp1 {
    type PrimaryKey = ForceMajeureOverriderrp1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid
            && self.startdate == key.startdate
            && self.startperiod == key.startperiod
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ForceMajeureOverriderrp1PrimaryKey {
    pub regionid: String,
    pub startdate: chrono::NaiveDateTime,
    pub startperiod: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ForceMajeureOverriderrp1PrimaryKey {
    type Row = ForceMajeureOverriderrp1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.regionid == row.regionid
            && self.startdate == row.startdate
            && self.startperiod == row.startperiod
    }
}
impl crate::CompareWithPrimaryKey for ForceMajeureOverriderrp1PrimaryKey {
    type PrimaryKey = ForceMajeureOverriderrp1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.regionid == key.regionid
            && self.startdate == key.startdate
            && self.startperiod == key.startperiod
    }
}
impl crate::PrimaryKey for ForceMajeureOverriderrp1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ForceMajeureOverriderrp1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Date64, false),
            arrow2::datatypes::Field::new(
                "startperiod",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "endperiod",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new("rrp", arrow2::datatypes::DataType::Decimal(15, 0), true),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authorisestart",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseend",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut regionid_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut startperiod_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut endperiod_array = Vec::new();
        let mut rrp_array = Vec::new();
        let mut description_array = Vec::new();
        let mut authorisestart_array = Vec::new();
        let mut authoriseend_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            regionid_array.push(row.regionid);
            startdate_array.push(row.startdate.timestamp_millis());
            startperiod_array.push({
                let mut val = row.startperiod;
                val.rescale(0);
                val.mantissa()
            });
            enddate_array.push(row.enddate.map(|val| val.timestamp_millis()));
            endperiod_array.push({
                row.endperiod.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            rrp_array.push({
                row.rrp.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            description_array.push(row.description);
            authorisestart_array.push(row.authorisestart);
            authoriseend_array.push(row.authoriseend);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(startdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(startperiod_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enddate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(endperiod_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(rrp_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisestart_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authoriseend_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
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
/// ## REGIONAPC
///  _REGIONAPC defines Administered Price profiles (Energy and FCAS) for a region._
///
/// * Data Set Name: Ap
/// * File Name: Regionapc
/// * Data Version: 1
///
/// # Description
///  REGIONAPC data is public, so is available to all participants. Source REGIONAPC updates when a change is ever made to the Administered Price Cap details. Changes to this table are infrequent.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApRegionapc1 {
    /// Region Identifier
    pub regionid: String,
    /// Date the APC profile applies from
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number for the same date
    pub versionno: rust_decimal::Decimal,
    /// Authorised date
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Authorised by
    pub authorisedby: Option<String>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for ApRegionapc1 {
    type PrimaryKey = ApRegionapc1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "AP".into(),
            table_name: Some("REGIONAPC".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ApRegionapc1PrimaryKey {
        ApRegionapc1PrimaryKey {
            effectivedate: self.effectivedate,
            regionid: self.regionid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "ap_regionapc_v1".to_string()
    }
}
impl crate::CompareWithRow for ApRegionapc1 {
    type Row = ApRegionapc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.regionid == row.regionid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ApRegionapc1 {
    type PrimaryKey = ApRegionapc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ApRegionapc1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub regionid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ApRegionapc1PrimaryKey {
    type Row = ApRegionapc1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.regionid == row.regionid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ApRegionapc1PrimaryKey {
    type PrimaryKey = ApRegionapc1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for ApRegionapc1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ApRegionapc1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "regionid",
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

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut regionid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            regionid_array.push(row.regionid);
            effectivedate_array.push(row.effectivedate.timestamp_millis());
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp_millis()));
            authorisedby_array.push(row.authorisedby);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
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
/// ## REGIONAPCINTERVALS
///  _REGIONAPCINTERVALS contains Administered Price profiles (Energy and FCAS) applicable to each interval for a region._
///
/// * Data Set Name: Ap
/// * File Name: Regionapcintervals
/// * Data Version: 1
///
/// # Description
///  REGIONAPCINTERVALS data is public, so is available to all participants. Source REGIONAPCINTERVALS is updated whenever an Administered Price Cap occurs.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * PERIODID
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct ApRegionapcintervals1 {
    /// Region Identifier
    pub regionid: String,
    /// Date the APC profile applies from
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version number for the same date
    pub versionno: rust_decimal::Decimal,
    /// 30-minute period
    pub periodid: rust_decimal::Decimal,
    /// Administered price cap in $
    pub apcvalue: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// not used
    pub apctype: Option<rust_decimal::Decimal>,
    /// FCAS Administered price cap in $
    pub fcasapcvalue: Option<rust_decimal::Decimal>,
    /// Administered price floor in $
    pub apfvalue: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for ApRegionapcintervals1 {
    type PrimaryKey = ApRegionapcintervals1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "AP".into(),
            table_name: Some("REGIONAPCINTERVALS".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> ApRegionapcintervals1PrimaryKey {
        ApRegionapcintervals1PrimaryKey {
            effectivedate: self.effectivedate,
            periodid: self.periodid,
            regionid: self.regionid.clone(),
            versionno: self.versionno,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "ap_regionapcintervals_v1".to_string()
    }
}
impl crate::CompareWithRow for ApRegionapcintervals1 {
    type Row = ApRegionapcintervals1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ApRegionapcintervals1 {
    type PrimaryKey = ApRegionapcintervals1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct ApRegionapcintervals1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for ApRegionapcintervals1PrimaryKey {
    type Row = ApRegionapcintervals1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for ApRegionapcintervals1PrimaryKey {
    type PrimaryKey = ApRegionapcintervals1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for ApRegionapcintervals1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for ApRegionapcintervals1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "regionid",
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
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "apcvalue",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "apctype",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "fcasapcvalue",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "apfvalue",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
        ])
    }

    fn partition_to_chunk(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::chunk::Chunk<std::sync::Arc<dyn arrow2::array::Array>>> {
        let mut regionid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut apcvalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut apctype_array = Vec::new();
        let mut fcasapcvalue_array = Vec::new();
        let mut apfvalue_array = Vec::new();
        for (_, row) in partition {
            regionid_array.push(row.regionid);
            effectivedate_array.push(row.effectivedate.timestamp_millis());
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
            apcvalue_array.push({
                row.apcvalue.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            apctype_array.push({
                row.apctype.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            fcasapcvalue_array.push({
                row.fcasapcvalue.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            apfvalue_array.push({
                row.apfvalue.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(apcvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(apctype_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(fcasapcvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(apfvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
            ],
        )
        .map_err(Into::into)
    }
}
