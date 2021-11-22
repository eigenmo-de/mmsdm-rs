/// # Summary
///
/// ## NETWORK_EQUIPMENTDETAIL
///  _NETWORK_EQUIPMENTDETAIL Provides details on equipment that may have outages or ratings. A single piece of equipment may have multiple records if its details change.<br>A line will typically have at least two valid records at a time, once for each end of the line.<br>_
///
/// * Data Set Name: Network
/// * File Name: Equipmentdetail
/// * Data Version: 2
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * ELEMENTID
/// * EQUIPMENTID
/// * EQUIPMENTTYPE
/// * SUBSTATIONID
/// * VALIDFROM
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkEquipmentdetail2 {
    /// ID uniquely identifying the substation this equipment is located at
    pub substationid: String,
    /// The type of equipment. Valid values are:<br>LINE = Line<br>TRANS = Transformer<br>CB = Circuit breaker<br>ISOL = Isolator<br>CAP = Capacitor<br>REAC = Reactor<br>UNIT = Unit<br>
    pub equipmenttype: String,
    /// A unique identifier for this type of equipment at this substation
    pub equipmentid: String,
    /// The date that this record is applies from (inclusive)
    #[serde(with = "crate::mms_datetime")]
    pub validfrom: chrono::NaiveDateTime,
    /// The date that this record applies until (exclusive)
    #[serde(with = "crate::mms_datetime_opt")]
    pub validto: Option<chrono::NaiveDateTime>,
    /// The voltage in KV for this equipment.<br>Transformers may have multiple voltages defined.<br>E.g. 132_110_33<br>
    pub voltage: Option<String>,
    /// A short description for this equipment.
    pub description: Option<String>,
    /// The time that this record was last changed.
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Equipment element id
    pub elementid: rust_decimal::Decimal,
}
impl crate::GetTable for NetworkEquipmentdetail2 {
    type PrimaryKey = NetworkEquipmentdetail2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "NETWORK".into(),
            table_name: Some("EQUIPMENTDETAIL".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> NetworkEquipmentdetail2PrimaryKey {
        NetworkEquipmentdetail2PrimaryKey {
            elementid: self.elementid,
            equipmentid: self.equipmentid.clone(),
            equipmenttype: self.equipmenttype.clone(),
            substationid: self.substationid.clone(),
            validfrom: self.validfrom,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "network_equipmentdetail_v2".to_string()
    }
}
impl crate::CompareWithRow for NetworkEquipmentdetail2 {
    type Row = NetworkEquipmentdetail2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.elementid == row.elementid
            && self.equipmentid == row.equipmentid
            && self.equipmenttype == row.equipmenttype
            && self.substationid == row.substationid
            && self.validfrom == row.validfrom
    }
}
impl crate::CompareWithPrimaryKey for NetworkEquipmentdetail2 {
    type PrimaryKey = NetworkEquipmentdetail2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.elementid == key.elementid
            && self.equipmentid == key.equipmentid
            && self.equipmenttype == key.equipmenttype
            && self.substationid == key.substationid
            && self.validfrom == key.validfrom
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkEquipmentdetail2PrimaryKey {
    pub elementid: rust_decimal::Decimal,
    pub equipmentid: String,
    pub equipmenttype: String,
    pub substationid: String,
    pub validfrom: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for NetworkEquipmentdetail2PrimaryKey {
    type Row = NetworkEquipmentdetail2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.elementid == row.elementid
            && self.equipmentid == row.equipmentid
            && self.equipmenttype == row.equipmenttype
            && self.substationid == row.substationid
            && self.validfrom == row.validfrom
    }
}
impl crate::CompareWithPrimaryKey for NetworkEquipmentdetail2PrimaryKey {
    type PrimaryKey = NetworkEquipmentdetail2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.elementid == key.elementid
            && self.equipmentid == key.equipmentid
            && self.equipmenttype == key.equipmenttype
            && self.substationid == key.substationid
            && self.validfrom == key.validfrom
    }
}
impl crate::PrimaryKey for NetworkEquipmentdetail2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for NetworkEquipmentdetail2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "substationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "equipmenttype",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "equipmentid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("validfrom", arrow2::datatypes::DataType::Date64, false),
            arrow2::datatypes::Field::new("validto", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("voltage", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "elementid",
                arrow2::datatypes::DataType::Decimal(15, 0),
                false,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut substationid_array = Vec::new();
        let mut equipmenttype_array = Vec::new();
        let mut equipmentid_array = Vec::new();
        let mut validfrom_array = Vec::new();
        let mut validto_array = Vec::new();
        let mut voltage_array = Vec::new();
        let mut description_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut elementid_array = Vec::new();
        for (_, row) in partition {
            substationid_array.push(row.substationid);
            equipmenttype_array.push(row.equipmenttype);
            equipmentid_array.push(row.equipmentid);
            validfrom_array.push(row.validfrom.timestamp_millis());
            validto_array.push(row.validto.map(|val| val.timestamp_millis()));
            voltage_array.push(row.voltage);
            description_array.push(row.description);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            elementid_array.push({
                let mut val = row.elementid;
                val.rescale(0);
                val.mantissa()
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    substationid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    equipmenttype_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    equipmentid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(validfrom_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(validto_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(voltage_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(elementid_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## NETWORK_OUTAGECONSTRAINTSET
///  _NETWORK_OUTAGECONSTRAINTSET lists the Constraint Set or Sets that are expected to be invoked for the outage once it is confirmed to proceed._
///
/// * Data Set Name: Network
/// * File Name: Outageconstraintset
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * GENCONSETID
/// * OUTAGEID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkOutageconstraintset1 {
    /// ID uniquely identifying the outage
    pub outageid: rust_decimal::Decimal,
    /// ID for the constraint set
    pub genconsetid: String,
    /// The dispatch interval that this constraint applies from
    #[serde(with = "crate::mms_datetime_opt")]
    pub startinterval: Option<chrono::NaiveDateTime>,
    /// The dispatch interval that this constraint applies until.
    #[serde(with = "crate::mms_datetime_opt")]
    pub endinterval: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for NetworkOutageconstraintset1 {
    type PrimaryKey = NetworkOutageconstraintset1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "NETWORK".into(),
            table_name: Some("OUTAGECONSTRAINTSET".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> NetworkOutageconstraintset1PrimaryKey {
        NetworkOutageconstraintset1PrimaryKey {
            genconsetid: self.genconsetid.clone(),
            outageid: self.outageid,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "network_outageconstraintset_v1".to_string()
    }
}
impl crate::CompareWithRow for NetworkOutageconstraintset1 {
    type Row = NetworkOutageconstraintset1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.genconsetid == row.genconsetid && self.outageid == row.outageid
    }
}
impl crate::CompareWithPrimaryKey for NetworkOutageconstraintset1 {
    type PrimaryKey = NetworkOutageconstraintset1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.genconsetid == key.genconsetid && self.outageid == key.outageid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkOutageconstraintset1PrimaryKey {
    pub genconsetid: String,
    pub outageid: rust_decimal::Decimal,
}
impl crate::CompareWithRow for NetworkOutageconstraintset1PrimaryKey {
    type Row = NetworkOutageconstraintset1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.genconsetid == row.genconsetid && self.outageid == row.outageid
    }
}
impl crate::CompareWithPrimaryKey for NetworkOutageconstraintset1PrimaryKey {
    type PrimaryKey = NetworkOutageconstraintset1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.genconsetid == key.genconsetid && self.outageid == key.outageid
    }
}
impl crate::PrimaryKey for NetworkOutageconstraintset1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for NetworkOutageconstraintset1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "outageid",
                arrow2::datatypes::DataType::Decimal(15, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "genconsetid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "startinterval",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new("endinterval", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut outageid_array = Vec::new();
        let mut genconsetid_array = Vec::new();
        let mut startinterval_array = Vec::new();
        let mut endinterval_array = Vec::new();
        for (_, row) in partition {
            outageid_array.push({
                let mut val = row.outageid;
                val.rescale(0);
                val.mantissa()
            });
            genconsetid_array.push(row.genconsetid);
            startinterval_array.push(row.startinterval.map(|val| val.timestamp_millis()));
            endinterval_array.push(row.endinterval.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(outageid_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    genconsetid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startinterval_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(endinterval_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## NETWORK_OUTAGEDETAIL
///  _Lists asset owners planned outages for transmission equipment. This also includes details for transmission equipment that will not have an outage, but associated secondary equipment has an outage and a related constraint set may be invoked. This scenario is indicated by the ISSECONDARY field in the table_
///
/// * Data Set Name: Network
/// * File Name: Outagedetail
/// * Data Version: 4
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * ELEMENTID
/// * EQUIPMENTID
/// * EQUIPMENTTYPE
/// * OUTAGEID
/// * STARTTIME
/// * SUBSTATIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkOutagedetail4 {
    /// ID uniquely identifying the outage
    pub outageid: rust_decimal::Decimal,
    /// The substation this equipment is located at
    pub substationid: String,
    /// The type of equipment. Valid values are:<br>LINE = Line<br>TRANS = Transformer<br>CB = Circuit breaker<br>ISOL = Isolator<br>CAP = Capacitor<br>REAC = Reactor<br>UNIT = Unit<br>
    pub equipmenttype: String,
    /// A unique identifier for this equipment at this substation, and based on its type
    pub equipmentid: String,
    /// The planned starting date and time of the outage
    #[serde(with = "crate::mms_datetime")]
    pub starttime: chrono::NaiveDateTime,
    /// The planned ending date and time of the outage
    #[serde(with = "crate::mms_datetime_opt")]
    pub endtime: Option<chrono::NaiveDateTime>,
    /// The date and time this outage was first submitted
    #[serde(with = "crate::mms_datetime_opt")]
    pub submitteddate: Option<chrono::NaiveDateTime>,
    /// A code representing the status of the outage.<br>The OUTAGESTATUSCODE table will store a detailed description of each code.<br>
    pub outagestatuscode: Option<String>,
    /// Changes to an outage key details may require the outage to be resubmitted.<br>A new outage id will then be allocated and the outage will be reassessed.<br>This field will detail the reason for the change.<br>
    pub resubmitreason: Option<String>,
    /// The new outage id created from a resubmit.
    pub resubmitoutageid: Option<rust_decimal::Decimal>,
    /// The recall time in minutes during the day
    pub recalltimeday: Option<rust_decimal::Decimal>,
    /// The recall time in minutes during the night
    pub recalltimenight: Option<rust_decimal::Decimal>,
    /// The time that this record was last changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The reason provided by the asset owner for this outage
    pub reason: Option<String>,
    /// 1 = The outage is for a secondary piece of equipment that has an associated constraint set. The transmission equipment is still in service. 0 = The outage is for the transmission equipment
    pub issecondary: Option<rust_decimal::Decimal>,
    /// The actual starting date/time of the outage
    #[serde(with = "crate::mms_datetime_opt")]
    pub actual_starttime: Option<chrono::NaiveDateTime>,
    /// The actual ending date/time of the outage
    #[serde(with = "crate::mms_datetime_opt")]
    pub actual_endtime: Option<chrono::NaiveDateTime>,
    /// The asset owners reference code for this outage
    pub companyrefcode: Option<String>,
    /// Equipment element id
    pub elementid: rust_decimal::Decimal,
}
impl crate::GetTable for NetworkOutagedetail4 {
    type PrimaryKey = NetworkOutagedetail4PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "NETWORK".into(),
            table_name: Some("OUTAGEDETAIL".into()),
            version: 4,
        }
    }

    fn primary_key(&self) -> NetworkOutagedetail4PrimaryKey {
        NetworkOutagedetail4PrimaryKey {
            elementid: self.elementid,
            equipmentid: self.equipmentid.clone(),
            equipmenttype: self.equipmenttype.clone(),
            outageid: self.outageid,
            starttime: self.starttime,
            substationid: self.substationid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "network_outagedetail_v4".to_string()
    }
}
impl crate::CompareWithRow for NetworkOutagedetail4 {
    type Row = NetworkOutagedetail4;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.elementid == row.elementid
            && self.equipmentid == row.equipmentid
            && self.equipmenttype == row.equipmenttype
            && self.outageid == row.outageid
            && self.starttime == row.starttime
            && self.substationid == row.substationid
    }
}
impl crate::CompareWithPrimaryKey for NetworkOutagedetail4 {
    type PrimaryKey = NetworkOutagedetail4PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.elementid == key.elementid
            && self.equipmentid == key.equipmentid
            && self.equipmenttype == key.equipmenttype
            && self.outageid == key.outageid
            && self.starttime == key.starttime
            && self.substationid == key.substationid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkOutagedetail4PrimaryKey {
    pub elementid: rust_decimal::Decimal,
    pub equipmentid: String,
    pub equipmenttype: String,
    pub outageid: rust_decimal::Decimal,
    pub starttime: chrono::NaiveDateTime,
    pub substationid: String,
}
impl crate::CompareWithRow for NetworkOutagedetail4PrimaryKey {
    type Row = NetworkOutagedetail4;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.elementid == row.elementid
            && self.equipmentid == row.equipmentid
            && self.equipmenttype == row.equipmenttype
            && self.outageid == row.outageid
            && self.starttime == row.starttime
            && self.substationid == row.substationid
    }
}
impl crate::CompareWithPrimaryKey for NetworkOutagedetail4PrimaryKey {
    type PrimaryKey = NetworkOutagedetail4PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.elementid == key.elementid
            && self.equipmentid == key.equipmentid
            && self.equipmenttype == key.equipmenttype
            && self.outageid == key.outageid
            && self.starttime == key.starttime
            && self.substationid == key.substationid
    }
}
impl crate::PrimaryKey for NetworkOutagedetail4PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for NetworkOutagedetail4 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "outageid",
                arrow2::datatypes::DataType::Decimal(15, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "substationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "equipmenttype",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "equipmentid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("starttime", arrow2::datatypes::DataType::Date64, false),
            arrow2::datatypes::Field::new("endtime", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "submitteddate",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "outagestatuscode",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "resubmitreason",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "resubmitoutageid",
                arrow2::datatypes::DataType::Decimal(15, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "recalltimeday",
                arrow2::datatypes::DataType::Decimal(10, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "recalltimenight",
                arrow2::datatypes::DataType::Decimal(10, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("reason", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "issecondary",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "actual_starttime",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "actual_endtime",
                arrow2::datatypes::DataType::Date64,
                true,
            ),
            arrow2::datatypes::Field::new(
                "companyrefcode",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "elementid",
                arrow2::datatypes::DataType::Decimal(15, 0),
                false,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut outageid_array = Vec::new();
        let mut substationid_array = Vec::new();
        let mut equipmenttype_array = Vec::new();
        let mut equipmentid_array = Vec::new();
        let mut starttime_array = Vec::new();
        let mut endtime_array = Vec::new();
        let mut submitteddate_array = Vec::new();
        let mut outagestatuscode_array = Vec::new();
        let mut resubmitreason_array = Vec::new();
        let mut resubmitoutageid_array = Vec::new();
        let mut recalltimeday_array = Vec::new();
        let mut recalltimenight_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut reason_array = Vec::new();
        let mut issecondary_array = Vec::new();
        let mut actual_starttime_array = Vec::new();
        let mut actual_endtime_array = Vec::new();
        let mut companyrefcode_array = Vec::new();
        let mut elementid_array = Vec::new();
        for (_, row) in partition {
            outageid_array.push({
                let mut val = row.outageid;
                val.rescale(0);
                val.mantissa()
            });
            substationid_array.push(row.substationid);
            equipmenttype_array.push(row.equipmenttype);
            equipmentid_array.push(row.equipmentid);
            starttime_array.push(row.starttime.timestamp_millis());
            endtime_array.push(row.endtime.map(|val| val.timestamp_millis()));
            submitteddate_array.push(row.submitteddate.map(|val| val.timestamp_millis()));
            outagestatuscode_array.push(row.outagestatuscode);
            resubmitreason_array.push(row.resubmitreason);
            resubmitoutageid_array.push({
                row.resubmitoutageid.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            recalltimeday_array.push({
                row.recalltimeday.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            recalltimenight_array.push({
                row.recalltimenight.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
            reason_array.push(row.reason);
            issecondary_array.push({
                row.issecondary.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            actual_starttime_array.push(row.actual_starttime.map(|val| val.timestamp_millis()));
            actual_endtime_array.push(row.actual_endtime.map(|val| val.timestamp_millis()));
            companyrefcode_array.push(row.companyrefcode);
            elementid_array.push({
                let mut val = row.elementid;
                val.rescale(0);
                val.mantissa()
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(outageid_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    substationid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    equipmenttype_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    equipmentid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(starttime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(endtime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(submitteddate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    outagestatuscode_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(resubmitreason_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(resubmitoutageid_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(recalltimeday_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(recalltimenight_array)
                        .to(arrow2::datatypes::DataType::Decimal(10, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(reason_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(issecondary_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(actual_starttime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(actual_endtime_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(companyrefcode_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(elementid_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## NETWORK_OUTAGESTATUSCODE
///  _NETWORK_OUTAGESTATUSCODE describes the different outage status codes_
///
/// * Data Set Name: Network
/// * File Name: Outagestatuscode
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * OUTAGESTATUSCODE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkOutagestatuscode1 {
    /// A code representing the status of an outage
    pub outagestatuscode: String,
    /// A description of the status code
    pub description: Option<String>,
    /// The time that this record was last changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for NetworkOutagestatuscode1 {
    type PrimaryKey = NetworkOutagestatuscode1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "NETWORK".into(),
            table_name: Some("OUTAGESTATUSCODE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> NetworkOutagestatuscode1PrimaryKey {
        NetworkOutagestatuscode1PrimaryKey {
            outagestatuscode: self.outagestatuscode.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "network_outagestatuscode_v1".to_string()
    }
}
impl crate::CompareWithRow for NetworkOutagestatuscode1 {
    type Row = NetworkOutagestatuscode1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.outagestatuscode == row.outagestatuscode
    }
}
impl crate::CompareWithPrimaryKey for NetworkOutagestatuscode1 {
    type PrimaryKey = NetworkOutagestatuscode1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.outagestatuscode == key.outagestatuscode
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkOutagestatuscode1PrimaryKey {
    pub outagestatuscode: String,
}
impl crate::CompareWithRow for NetworkOutagestatuscode1PrimaryKey {
    type Row = NetworkOutagestatuscode1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.outagestatuscode == row.outagestatuscode
    }
}
impl crate::CompareWithPrimaryKey for NetworkOutagestatuscode1PrimaryKey {
    type PrimaryKey = NetworkOutagestatuscode1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.outagestatuscode == key.outagestatuscode
    }
}
impl crate::PrimaryKey for NetworkOutagestatuscode1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for NetworkOutagestatuscode1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "outagestatuscode",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut outagestatuscode_array = Vec::new();
        let mut description_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            outagestatuscode_array.push(row.outagestatuscode);
            description_array.push(row.description);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    outagestatuscode_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)),
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
/// ## NETWORK_RATING
///  _NETWORK_RATING defines a list of the equipment ratings that may be used as inputs to market constraints.<br>If the rating is flagged as dynamic then in real-time the rating will be dynamically determined and the static value will be used as a fallback value should the dynamic value fail.<br>Note:<br>In some rare cases equipment has ratings provided from more than one TNSP. This is identified by a different SPD Id. The value used in the NEM is normally the more restrictive of the two values.<br>_
///
/// * Data Set Name: Network
/// * File Name: Rating
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * SPD_ID
/// * VALIDFROM
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkRating1 {
    /// ID defining this data source for use in constraints
    pub spd_id: String,
    /// The date that this record is applies from (inclusive)
    #[serde(with = "crate::mms_datetime")]
    pub validfrom: chrono::NaiveDateTime,
    /// The date that this record applies until (exclusive)
    #[serde(with = "crate::mms_datetime_opt")]
    pub validto: Option<chrono::NaiveDateTime>,
    /// The region that this rating is for
    pub regionid: Option<String>,
    /// The substation the equipment is located at
    pub substationid: Option<String>,
    /// The type of equipment. Valid values are:<br>LINE = Line<br>TRANS = Transformer<br>CB = Circuit breaker<br>ISOL = Isolator<br>CAP = Capacitor<br>REAC = Reactor<br>UNIT = Unit<br>
    pub equipmenttype: Option<String>,
    /// A unique identifier for this equipment at this substation, and based on its type
    pub equipmentid: Option<String>,
    /// The rating level of the value used, one of:<br>NORM = Continuous rating value. Applied under pre-contingent conditions.<br>EMER = Continuous rating value. Applied under pre-contingent conditions<br>LDSH = Load Shedding<br>
    pub ratinglevel: Option<String>,
    /// One of:<br>1 = Normally uses dynamic ratings<br>0 = No dynamic ratings, static ratings are used<br>
    pub isdynamic: Option<rust_decimal::Decimal>,
    /// The time that this record was last changed
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for NetworkRating1 {
    type PrimaryKey = NetworkRating1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "NETWORK".into(),
            table_name: Some("RATING".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> NetworkRating1PrimaryKey {
        NetworkRating1PrimaryKey {
            spd_id: self.spd_id.clone(),
            validfrom: self.validfrom,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "network_rating_v1".to_string()
    }
}
impl crate::CompareWithRow for NetworkRating1 {
    type Row = NetworkRating1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.spd_id == row.spd_id && self.validfrom == row.validfrom
    }
}
impl crate::CompareWithPrimaryKey for NetworkRating1 {
    type PrimaryKey = NetworkRating1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.spd_id == key.spd_id && self.validfrom == key.validfrom
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkRating1PrimaryKey {
    pub spd_id: String,
    pub validfrom: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for NetworkRating1PrimaryKey {
    type Row = NetworkRating1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.spd_id == row.spd_id && self.validfrom == row.validfrom
    }
}
impl crate::CompareWithPrimaryKey for NetworkRating1PrimaryKey {
    type PrimaryKey = NetworkRating1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.spd_id == key.spd_id && self.validfrom == key.validfrom
    }
}
impl crate::PrimaryKey for NetworkRating1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for NetworkRating1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("spd_id", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new("validfrom", arrow2::datatypes::DataType::Date64, false),
            arrow2::datatypes::Field::new("validto", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "substationid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "equipmenttype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "equipmentid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "ratinglevel",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "isdynamic",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut spd_id_array = Vec::new();
        let mut validfrom_array = Vec::new();
        let mut validto_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut substationid_array = Vec::new();
        let mut equipmenttype_array = Vec::new();
        let mut equipmentid_array = Vec::new();
        let mut ratinglevel_array = Vec::new();
        let mut isdynamic_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            spd_id_array.push(row.spd_id);
            validfrom_array.push(row.validfrom.timestamp_millis());
            validto_array.push(row.validto.map(|val| val.timestamp_millis()));
            regionid_array.push(row.regionid);
            substationid_array.push(row.substationid);
            equipmenttype_array.push(row.equipmenttype);
            equipmentid_array.push(row.equipmentid);
            ratinglevel_array.push(row.ratinglevel);
            isdynamic_array.push({
                row.isdynamic.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(spd_id_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(validfrom_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(validto_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(regionid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(substationid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(equipmenttype_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(equipmentid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(ratinglevel_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(isdynamic_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
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
/// ## NETWORK_REALTIMERATING
///  _The NETWORK_REALTIMERATING table shows the equipment rating values in MVA used as inputs to constraints in the dispatch solution. This includes values for both static and dynamic ratings. The NETWORK_RATING table can be used to determine the physical equipment the rating is for based on the SPD_ID value._
///
/// * Data Set Name: Network
/// * File Name: Realtimerating
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * SETTLEMENTDATE
/// * SPD_ID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkRealtimerating1 {
    /// The dispatch interval the rating applies to
    #[serde(with = "crate::mms_datetime")]
    pub settlementdate: chrono::NaiveDateTime,
    /// ID defining this data source for use in constraints
    pub spd_id: String,
    /// The defined equipment rating value in MVA
    pub ratingvalue: rust_decimal::Decimal,
}
impl crate::GetTable for NetworkRealtimerating1 {
    type PrimaryKey = NetworkRealtimerating1PrimaryKey;
    type Partition = (i32, chrono::Month);

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "NETWORK".into(),
            table_name: Some("REALTIMERATING".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> NetworkRealtimerating1PrimaryKey {
        NetworkRealtimerating1PrimaryKey {
            settlementdate: self.settlementdate,
            spd_id: self.spd_id.clone(),
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
            "network_realtimerating_v1_{}_{}",
            chrono::Datelike::year(&self.settlementdate),
            chrono::Datelike::month(&self.settlementdate)
        )
    }
}
impl crate::CompareWithRow for NetworkRealtimerating1 {
    type Row = NetworkRealtimerating1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.settlementdate == row.settlementdate && self.spd_id == row.spd_id
    }
}
impl crate::CompareWithPrimaryKey for NetworkRealtimerating1 {
    type PrimaryKey = NetworkRealtimerating1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.settlementdate == key.settlementdate && self.spd_id == key.spd_id
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkRealtimerating1PrimaryKey {
    pub settlementdate: chrono::NaiveDateTime,
    pub spd_id: String,
}
impl crate::CompareWithRow for NetworkRealtimerating1PrimaryKey {
    type Row = NetworkRealtimerating1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.settlementdate == row.settlementdate && self.spd_id == row.spd_id
    }
}
impl crate::CompareWithPrimaryKey for NetworkRealtimerating1PrimaryKey {
    type PrimaryKey = NetworkRealtimerating1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.settlementdate == key.settlementdate && self.spd_id == key.spd_id
    }
}
impl crate::PrimaryKey for NetworkRealtimerating1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for NetworkRealtimerating1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "settlementdate",
                arrow2::datatypes::DataType::Date64,
                false,
            ),
            arrow2::datatypes::Field::new("spd_id", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "ratingvalue",
                arrow2::datatypes::DataType::Decimal(16, 6),
                false,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut settlementdate_array = Vec::new();
        let mut spd_id_array = Vec::new();
        let mut ratingvalue_array = Vec::new();
        for (_, row) in partition {
            settlementdate_array.push(row.settlementdate.timestamp_millis());
            spd_id_array.push(row.spd_id);
            ratingvalue_array.push({
                let mut val = row.ratingvalue;
                val.rescale(6);
                val.mantissa()
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(spd_id_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(ratingvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## NETWORK_STATICRATING
///  _NETWORK_STATICRATING lists the static rating values that will apply for a Rating Application ID.<br>This data does not provide information for when the rating actually applies in the NEM. This is dependent on the Rating Application definition.<br>For information on the Rating Applications please refer to the information published on the AEMO website under the topic "Transmission Equipment Ratings". The Rating Applications are referred to as Alternate Value Application Ratings.<br>Ratings that normally use dynamic values will also have static rating values defined. These are used as a fallback if the dynamic rating fails.<br>_
///
/// * Data Set Name: Network
/// * File Name: Staticrating
/// * Data Version: 1
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * APPLICATIONID
/// * EQUIPMENTID
/// * EQUIPMENTTYPE
/// * RATINGLEVEL
/// * SUBSTATIONID
/// * VALIDFROM
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkStaticrating1 {
    /// The substation the equipment is located at
    pub substationid: String,
    /// The type of equipment. Valid values are:<br>LINE = Line<br>TRANS = Transformer<br>CB = Circuit breaker<br>ISOL = Isolator<br>CAP = Capacitor<br>REAC = Reactor<br>UNIT = Unit<br>
    pub equipmenttype: String,
    /// A unique identifier for this type of equipment at this substation
    pub equipmentid: String,
    /// The rating level of the value used, one of:<br>NORM = Continuous rating value. Applied under pre-contingent conditions.<br>EMER = Continuous rating value. Applied under pre-contingent conditions<br>LDSH = Load Shedding
    pub ratinglevel: String,
    /// The applicationid which defines the application timeframes of this rating.
    pub applicationid: String,
    /// The date that this record is applies from (inclusive)
    #[serde(with = "crate::mms_datetime")]
    pub validfrom: chrono::NaiveDateTime,
    /// The date that this record applies until (exclusive)
    #[serde(with = "crate::mms_datetime_opt")]
    pub validto: Option<chrono::NaiveDateTime>,
    /// The rating value in MVA that applies. This may be positive or negative depending on which side of the nominal MW flow direction the rating value applies.<br>Flow into a transmission device is positive, flow out of the device is negative.<br>
    pub ratingvalue: Option<rust_decimal::Decimal>,
    /// The time that this record was last changed.
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for NetworkStaticrating1 {
    type PrimaryKey = NetworkStaticrating1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "NETWORK".into(),
            table_name: Some("STATICRATING".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> NetworkStaticrating1PrimaryKey {
        NetworkStaticrating1PrimaryKey {
            applicationid: self.applicationid.clone(),
            equipmentid: self.equipmentid.clone(),
            equipmenttype: self.equipmenttype.clone(),
            ratinglevel: self.ratinglevel.clone(),
            substationid: self.substationid.clone(),
            validfrom: self.validfrom,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "network_staticrating_v1".to_string()
    }
}
impl crate::CompareWithRow for NetworkStaticrating1 {
    type Row = NetworkStaticrating1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.applicationid == row.applicationid
            && self.equipmentid == row.equipmentid
            && self.equipmenttype == row.equipmenttype
            && self.ratinglevel == row.ratinglevel
            && self.substationid == row.substationid
            && self.validfrom == row.validfrom
    }
}
impl crate::CompareWithPrimaryKey for NetworkStaticrating1 {
    type PrimaryKey = NetworkStaticrating1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.applicationid == key.applicationid
            && self.equipmentid == key.equipmentid
            && self.equipmenttype == key.equipmenttype
            && self.ratinglevel == key.ratinglevel
            && self.substationid == key.substationid
            && self.validfrom == key.validfrom
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkStaticrating1PrimaryKey {
    pub applicationid: String,
    pub equipmentid: String,
    pub equipmenttype: String,
    pub ratinglevel: String,
    pub substationid: String,
    pub validfrom: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for NetworkStaticrating1PrimaryKey {
    type Row = NetworkStaticrating1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.applicationid == row.applicationid
            && self.equipmentid == row.equipmentid
            && self.equipmenttype == row.equipmenttype
            && self.ratinglevel == row.ratinglevel
            && self.substationid == row.substationid
            && self.validfrom == row.validfrom
    }
}
impl crate::CompareWithPrimaryKey for NetworkStaticrating1PrimaryKey {
    type PrimaryKey = NetworkStaticrating1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.applicationid == key.applicationid
            && self.equipmentid == key.equipmentid
            && self.equipmenttype == key.equipmenttype
            && self.ratinglevel == key.ratinglevel
            && self.substationid == key.substationid
            && self.validfrom == key.validfrom
    }
}
impl crate::PrimaryKey for NetworkStaticrating1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for NetworkStaticrating1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "substationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "equipmenttype",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "equipmentid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "ratinglevel",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "applicationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("validfrom", arrow2::datatypes::DataType::Date64, false),
            arrow2::datatypes::Field::new("validto", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "ratingvalue",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut substationid_array = Vec::new();
        let mut equipmenttype_array = Vec::new();
        let mut equipmentid_array = Vec::new();
        let mut ratinglevel_array = Vec::new();
        let mut applicationid_array = Vec::new();
        let mut validfrom_array = Vec::new();
        let mut validto_array = Vec::new();
        let mut ratingvalue_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            substationid_array.push(row.substationid);
            equipmenttype_array.push(row.equipmenttype);
            equipmentid_array.push(row.equipmentid);
            ratinglevel_array.push(row.ratinglevel);
            applicationid_array.push(row.applicationid);
            validfrom_array.push(row.validfrom.timestamp_millis());
            validto_array.push(row.validto.map(|val| val.timestamp_millis()));
            ratingvalue_array.push({
                row.ratingvalue.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    substationid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    equipmenttype_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    equipmentid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    ratinglevel_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    applicationid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(validfrom_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(validto_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(ratingvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
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
/// ## NETWORK_SUBSTATIONDETAIL
///  _NETWORK_SUBSTATIONDETAIL sets out the attributes of sub-stations across time_
///
/// * Data Set Name: Network
/// * File Name: Substationdetail
/// * Data Version: 2
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * SUBSTATIONID
/// * VALIDFROM
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct NetworkSubstationdetail2 {
    /// ID uniquely identifying this substation
    pub substationid: String,
    /// The record is valid from this date (inclusive)
    #[serde(with = "crate::mms_datetime")]
    pub validfrom: chrono::NaiveDateTime,
    /// The record is valid up until this date (exclusive)
    #[serde(with = "crate::mms_datetime_opt")]
    pub validto: Option<chrono::NaiveDateTime>,
    /// Description of the substation
    pub description: Option<String>,
    /// The NEM region the substation is in
    pub regionid: Option<String>,
    /// The TNSP who is responsible for this substation
    pub ownerid: Option<String>,
    /// The time that this record was last changed.
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for NetworkSubstationdetail2 {
    type PrimaryKey = NetworkSubstationdetail2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "NETWORK".into(),
            table_name: Some("SUBSTATIONDETAIL".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> NetworkSubstationdetail2PrimaryKey {
        NetworkSubstationdetail2PrimaryKey {
            substationid: self.substationid.clone(),
            validfrom: self.validfrom,
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "network_substationdetail_v2".to_string()
    }
}
impl crate::CompareWithRow for NetworkSubstationdetail2 {
    type Row = NetworkSubstationdetail2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.substationid == row.substationid && self.validfrom == row.validfrom
    }
}
impl crate::CompareWithPrimaryKey for NetworkSubstationdetail2 {
    type PrimaryKey = NetworkSubstationdetail2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.substationid == key.substationid && self.validfrom == key.validfrom
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkSubstationdetail2PrimaryKey {
    pub substationid: String,
    pub validfrom: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for NetworkSubstationdetail2PrimaryKey {
    type Row = NetworkSubstationdetail2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.substationid == row.substationid && self.validfrom == row.validfrom
    }
}
impl crate::CompareWithPrimaryKey for NetworkSubstationdetail2PrimaryKey {
    type PrimaryKey = NetworkSubstationdetail2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.substationid == key.substationid && self.validfrom == key.validfrom
    }
}
impl crate::PrimaryKey for NetworkSubstationdetail2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for NetworkSubstationdetail2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "substationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("validfrom", arrow2::datatypes::DataType::Date64, false),
            arrow2::datatypes::Field::new("validto", arrow2::datatypes::DataType::Date64, true),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("ownerid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date64, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut substationid_array = Vec::new();
        let mut validfrom_array = Vec::new();
        let mut validto_array = Vec::new();
        let mut description_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut ownerid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            substationid_array.push(row.substationid);
            validfrom_array.push(row.validfrom.timestamp_millis());
            validto_array.push(row.validto.map(|val| val.timestamp_millis()));
            description_array.push(row.description);
            regionid_array.push(row.regionid);
            ownerid_array.push(row.ownerid);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp_millis()));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    substationid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(validfrom_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(validto_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(regionid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(ownerid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date64),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
