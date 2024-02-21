#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct NetworkEquipmentdetail2;
pub struct NetworkEquipmentdetail2Mapping([usize; 9]);
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
///
///
/// # Primary Key Columns
///
/// * ELEMENTID
/// * EQUIPMENTID
/// * EQUIPMENTTYPE
/// * SUBSTATIONID
/// * VALIDFROM
#[derive(Debug, PartialEq, Eq)]
pub struct NetworkEquipmentdetail2Row<'data> {
    /// ID uniquely identifying the substation this equipment is located at
    pub substationid: core::ops::Range<usize>,
    /// The type of equipment. Valid values are:<br>LINE = Line<br>TRANS = Transformer<br>CB = Circuit breaker<br>ISOL = Isolator<br>CAP = Capacitor<br>REAC = Reactor<br>UNIT = Unit<br>
    pub equipmenttype: core::ops::Range<usize>,
    /// A unique identifier for this type of equipment at this substation
    pub equipmentid: core::ops::Range<usize>,
    /// The date that this record is applies from (inclusive)
    pub validfrom: chrono::NaiveDateTime,
    /// The date that this record applies until (exclusive)
    pub validto: Option<chrono::NaiveDateTime>,
    /// The voltage in KV for this equipment.<br>Transformers may have multiple voltages defined.<br>E.g. 132_110_33<br>
    pub voltage: core::ops::Range<usize>,
    /// A short description for this equipment.
    pub description: core::ops::Range<usize>,
    /// The time that this record was last changed.
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Equipment element id
    pub elementid: rust_decimal::Decimal,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> NetworkEquipmentdetail2Row<'data> {
    pub fn substationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.substationid.clone())
    }
    pub fn equipmenttype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.equipmenttype.clone())
    }
    pub fn equipmentid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.equipmentid.clone())
    }
    pub fn voltage(&self) -> Option<&str> {
        if self.voltage.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.voltage.clone(),
                ),
            )
        }
    }
    pub fn description(&self) -> Option<&str> {
        if self.description.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.description.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for NetworkEquipmentdetail2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "NETWORK";
    const TABLE_NAME: &'static str = "EQUIPMENTDETAIL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = NetworkEquipmentdetail2Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SUBSTATIONID",
        "EQUIPMENTTYPE",
        "EQUIPMENTID",
        "VALIDFROM",
        "VALIDTO",
        "VOLTAGE",
        "DESCRIPTION",
        "LASTCHANGED",
        "ELEMENTID",
    ];
    type Row<'row> = NetworkEquipmentdetail2Row<'row>;
    type FieldMapping = NetworkEquipmentdetail2Mapping;
    type PrimaryKey = NetworkEquipmentdetail2PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(NetworkEquipmentdetail2Row {
            substationid: row.get_range("substationid", field_mapping.0[0])?,
            equipmenttype: row.get_range("equipmenttype", field_mapping.0[1])?,
            equipmentid: row.get_range("equipmentid", field_mapping.0[2])?,
            validfrom: row
                .get_custom_parsed_at_idx(
                    "validfrom",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            validto: row
                .get_opt_custom_parsed_at_idx(
                    "validto",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            voltage: row.get_opt_range("voltage", field_mapping.0[5])?,
            description: row.get_opt_range("description", field_mapping.0[6])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[7],
                    mmsdm_core::mms_datetime::parse,
                )?,
            elementid: row
                .get_custom_parsed_at_idx(
                    "elementid",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(NetworkEquipmentdetail2Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> NetworkEquipmentdetail2PrimaryKey {
        NetworkEquipmentdetail2PrimaryKey {
            elementid: row.elementid,
            equipmentid: row.equipmentid().to_string(),
            equipmenttype: row.equipmenttype().to_string(),
            substationid: row.substationid().to_string(),
            validfrom: row.validfrom,
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "network_equipmentdetail_v2".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        NetworkEquipmentdetail2Row {
            substationid: row.substationid.clone(),
            equipmenttype: row.equipmenttype.clone(),
            equipmentid: row.equipmentid.clone(),
            validfrom: row.validfrom.clone(),
            validto: row.validto.clone(),
            voltage: row.voltage.clone(),
            description: row.description.clone(),
            lastchanged: row.lastchanged.clone(),
            elementid: row.elementid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkEquipmentdetail2PrimaryKey {
    pub elementid: rust_decimal::Decimal,
    pub equipmentid: alloc::string::String,
    pub equipmenttype: alloc::string::String,
    pub substationid: alloc::string::String,
    pub validfrom: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for NetworkEquipmentdetail2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for NetworkEquipmentdetail2Row<'data> {
    type Row<'other> = NetworkEquipmentdetail2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.elementid == row.elementid && self.equipmentid() == row.equipmentid()
            && self.equipmenttype() == row.equipmenttype()
            && self.substationid() == row.substationid()
            && self.validfrom == row.validfrom
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for NetworkEquipmentdetail2Row<'data> {
    type PrimaryKey = NetworkEquipmentdetail2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.elementid == key.elementid && self.equipmentid() == key.equipmentid
            && self.equipmenttype() == key.equipmenttype
            && self.substationid() == key.substationid && self.validfrom == key.validfrom
    }
}
impl<'data> mmsdm_core::CompareWithRow for NetworkEquipmentdetail2PrimaryKey {
    type Row<'other> = NetworkEquipmentdetail2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.elementid == row.elementid && self.equipmentid == row.equipmentid()
            && self.equipmenttype == row.equipmenttype()
            && self.substationid == row.substationid() && self.validfrom == row.validfrom
    }
}
impl mmsdm_core::CompareWithPrimaryKey for NetworkEquipmentdetail2PrimaryKey {
    type PrimaryKey = NetworkEquipmentdetail2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.elementid == key.elementid && self.equipmentid == key.equipmentid
            && self.equipmenttype == key.equipmenttype
            && self.substationid == key.substationid && self.validfrom == key.validfrom
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for NetworkEquipmentdetail2 {
    type Builder = NetworkEquipmentdetail2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "substationid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "equipmenttype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "equipmentid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "validfrom",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "validto",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "voltage",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "description",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "elementid",
                    arrow::datatypes::DataType::Decimal128(15, 0),
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        NetworkEquipmentdetail2Builder {
            substationid_array: arrow::array::builder::StringBuilder::new(),
            equipmenttype_array: arrow::array::builder::StringBuilder::new(),
            equipmentid_array: arrow::array::builder::StringBuilder::new(),
            validfrom_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            validto_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            voltage_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            elementid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.substationid_array.append_value(row.substationid());
        builder.equipmenttype_array.append_value(row.equipmenttype());
        builder.equipmentid_array.append_value(row.equipmentid());
        builder.validfrom_array.append_value(row.validfrom.timestamp_millis());
        builder
            .validto_array
            .append_option(row.validto.map(|val| val.timestamp_millis()));
        builder.voltage_array.append_option(row.voltage());
        builder.description_array.append_option(row.description());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .elementid_array
            .append_value({
                let mut val = row.elementid;
                val.rescale(0);
                val.mantissa()
            });
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.substationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.equipmenttype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.equipmentid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.validfrom_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.validto_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.voltage_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.elementid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct NetworkEquipmentdetail2Builder {
    substationid_array: arrow::array::builder::StringBuilder,
    equipmenttype_array: arrow::array::builder::StringBuilder,
    equipmentid_array: arrow::array::builder::StringBuilder,
    validfrom_array: arrow::array::builder::TimestampMillisecondBuilder,
    validto_array: arrow::array::builder::TimestampMillisecondBuilder,
    voltage_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    elementid_array: arrow::array::builder::Decimal128Builder,
}
pub struct NetworkOutageconstraintset1;
pub struct NetworkOutageconstraintset1Mapping([usize; 4]);
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
///
///
/// # Primary Key Columns
///
/// * GENCONSETID
/// * OUTAGEID
#[derive(Debug, PartialEq, Eq)]
pub struct NetworkOutageconstraintset1Row<'data> {
    /// ID uniquely identifying the outage
    pub outageid: rust_decimal::Decimal,
    /// ID for the constraint set
    pub genconsetid: core::ops::Range<usize>,
    /// The dispatch interval that this constraint applies from
    pub startinterval: Option<chrono::NaiveDateTime>,
    /// The dispatch interval that this constraint applies until.
    pub endinterval: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> NetworkOutageconstraintset1Row<'data> {
    pub fn genconsetid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.genconsetid.clone())
    }
}
impl mmsdm_core::GetTable for NetworkOutageconstraintset1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "NETWORK";
    const TABLE_NAME: &'static str = "OUTAGECONSTRAINTSET";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = NetworkOutageconstraintset1Mapping([
        4,
        5,
        6,
        7,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "OUTAGEID",
        "GENCONSETID",
        "STARTINTERVAL",
        "ENDINTERVAL",
    ];
    type Row<'row> = NetworkOutageconstraintset1Row<'row>;
    type FieldMapping = NetworkOutageconstraintset1Mapping;
    type PrimaryKey = NetworkOutageconstraintset1PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(NetworkOutageconstraintset1Row {
            outageid: row
                .get_custom_parsed_at_idx(
                    "outageid",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            genconsetid: row.get_range("genconsetid", field_mapping.0[1])?,
            startinterval: row
                .get_opt_custom_parsed_at_idx(
                    "startinterval",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            endinterval: row
                .get_opt_custom_parsed_at_idx(
                    "endinterval",
                    field_mapping.0[3],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(NetworkOutageconstraintset1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> NetworkOutageconstraintset1PrimaryKey {
        NetworkOutageconstraintset1PrimaryKey {
            genconsetid: row.genconsetid().to_string(),
            outageid: row.outageid,
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "network_outageconstraintset_v1".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        NetworkOutageconstraintset1Row {
            outageid: row.outageid.clone(),
            genconsetid: row.genconsetid.clone(),
            startinterval: row.startinterval.clone(),
            endinterval: row.endinterval.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkOutageconstraintset1PrimaryKey {
    pub genconsetid: alloc::string::String,
    pub outageid: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for NetworkOutageconstraintset1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for NetworkOutageconstraintset1Row<'data> {
    type Row<'other> = NetworkOutageconstraintset1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.genconsetid() == row.genconsetid() && self.outageid == row.outageid
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for NetworkOutageconstraintset1Row<'data> {
    type PrimaryKey = NetworkOutageconstraintset1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.genconsetid() == key.genconsetid && self.outageid == key.outageid
    }
}
impl<'data> mmsdm_core::CompareWithRow for NetworkOutageconstraintset1PrimaryKey {
    type Row<'other> = NetworkOutageconstraintset1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.genconsetid == row.genconsetid() && self.outageid == row.outageid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for NetworkOutageconstraintset1PrimaryKey {
    type PrimaryKey = NetworkOutageconstraintset1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.genconsetid == key.genconsetid && self.outageid == key.outageid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for NetworkOutageconstraintset1 {
    type Builder = NetworkOutageconstraintset1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "outageid",
                    arrow::datatypes::DataType::Decimal128(15, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "genconsetid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "startinterval",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "endinterval",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        NetworkOutageconstraintset1Builder {
            outageid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 0)),
            genconsetid_array: arrow::array::builder::StringBuilder::new(),
            startinterval_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            endinterval_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .outageid_array
            .append_value({
                let mut val = row.outageid;
                val.rescale(0);
                val.mantissa()
            });
        builder.genconsetid_array.append_value(row.genconsetid());
        builder
            .startinterval_array
            .append_option(row.startinterval.map(|val| val.timestamp_millis()));
        builder
            .endinterval_array
            .append_option(row.endinterval.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.outageid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genconsetid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startinterval_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.endinterval_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct NetworkOutageconstraintset1Builder {
    outageid_array: arrow::array::builder::Decimal128Builder,
    genconsetid_array: arrow::array::builder::StringBuilder,
    startinterval_array: arrow::array::builder::TimestampMillisecondBuilder,
    endinterval_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct NetworkOutagedetail4;
pub struct NetworkOutagedetail4Mapping([usize; 19]);
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
///
///
/// # Primary Key Columns
///
/// * ELEMENTID
/// * EQUIPMENTID
/// * EQUIPMENTTYPE
/// * OUTAGEID
/// * STARTTIME
/// * SUBSTATIONID
#[derive(Debug, PartialEq, Eq)]
pub struct NetworkOutagedetail4Row<'data> {
    /// ID uniquely identifying the outage
    pub outageid: rust_decimal::Decimal,
    /// The substation this equipment is located at
    pub substationid: core::ops::Range<usize>,
    /// The type of equipment. Valid values are:<br>LINE = Line<br>TRANS = Transformer<br>CB = Circuit breaker<br>ISOL = Isolator<br>CAP = Capacitor<br>REAC = Reactor<br>UNIT = Unit<br>
    pub equipmenttype: core::ops::Range<usize>,
    /// A unique identifier for this equipment at this substation, and based on its type
    pub equipmentid: core::ops::Range<usize>,
    /// The planned starting date and time of the outage
    pub starttime: chrono::NaiveDateTime,
    /// The planned ending date and time of the outage
    pub endtime: Option<chrono::NaiveDateTime>,
    /// The date and time this outage was first submitted
    pub submitteddate: Option<chrono::NaiveDateTime>,
    /// A code representing the status of the outage.<br>The OUTAGESTATUSCODE table will store a detailed description of each code.<br>
    pub outagestatuscode: core::ops::Range<usize>,
    /// Changes to an outage key details may require the outage to be resubmitted.<br>A new outage id will then be allocated and the outage will be reassessed.<br>This field will detail the reason for the change.<br>
    pub resubmitreason: core::ops::Range<usize>,
    /// The new outage id created from a resubmit.
    pub resubmitoutageid: Option<rust_decimal::Decimal>,
    /// The recall time in minutes during the day
    pub recalltimeday: Option<rust_decimal::Decimal>,
    /// The recall time in minutes during the night
    pub recalltimenight: Option<rust_decimal::Decimal>,
    /// The time that this record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The reason provided by the asset owner for this outage
    pub reason: core::ops::Range<usize>,
    /// 1 = The outage is for a secondary piece of equipment that has an associated constraint set. The transmission equipment is still in service. 0 = The outage is for the transmission equipment
    pub issecondary: Option<rust_decimal::Decimal>,
    /// The actual starting date/time of the outage
    pub actual_starttime: Option<chrono::NaiveDateTime>,
    /// The actual ending date/time of the outage
    pub actual_endtime: Option<chrono::NaiveDateTime>,
    /// The asset owners reference code for this outage
    pub companyrefcode: core::ops::Range<usize>,
    /// Equipment element id
    pub elementid: rust_decimal::Decimal,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> NetworkOutagedetail4Row<'data> {
    pub fn substationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.substationid.clone())
    }
    pub fn equipmenttype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.equipmenttype.clone())
    }
    pub fn equipmentid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.equipmentid.clone())
    }
    pub fn outagestatuscode(&self) -> Option<&str> {
        if self.outagestatuscode.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.outagestatuscode.clone(),
                ),
            )
        }
    }
    pub fn resubmitreason(&self) -> Option<&str> {
        if self.resubmitreason.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.resubmitreason.clone(),
                ),
            )
        }
    }
    pub fn reason(&self) -> Option<&str> {
        if self.reason.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.reason.clone(),
                ),
            )
        }
    }
    pub fn companyrefcode(&self) -> Option<&str> {
        if self.companyrefcode.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.companyrefcode.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for NetworkOutagedetail4 {
    const VERSION: i32 = 4;
    const DATA_SET_NAME: &'static str = "NETWORK";
    const TABLE_NAME: &'static str = "OUTAGEDETAIL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = NetworkOutagedetail4Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20,
        21,
        22,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "OUTAGEID",
        "SUBSTATIONID",
        "EQUIPMENTTYPE",
        "EQUIPMENTID",
        "STARTTIME",
        "ENDTIME",
        "SUBMITTEDDATE",
        "OUTAGESTATUSCODE",
        "RESUBMITREASON",
        "RESUBMITOUTAGEID",
        "RECALLTIMEDAY",
        "RECALLTIMENIGHT",
        "LASTCHANGED",
        "REASON",
        "ISSECONDARY",
        "ACTUAL_STARTTIME",
        "ACTUAL_ENDTIME",
        "COMPANYREFCODE",
        "ELEMENTID",
    ];
    type Row<'row> = NetworkOutagedetail4Row<'row>;
    type FieldMapping = NetworkOutagedetail4Mapping;
    type PrimaryKey = NetworkOutagedetail4PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(NetworkOutagedetail4Row {
            outageid: row
                .get_custom_parsed_at_idx(
                    "outageid",
                    field_mapping.0[0],
                    mmsdm_core::mms_decimal::parse,
                )?,
            substationid: row.get_range("substationid", field_mapping.0[1])?,
            equipmenttype: row.get_range("equipmenttype", field_mapping.0[2])?,
            equipmentid: row.get_range("equipmentid", field_mapping.0[3])?,
            starttime: row
                .get_custom_parsed_at_idx(
                    "starttime",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            endtime: row
                .get_opt_custom_parsed_at_idx(
                    "endtime",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            submitteddate: row
                .get_opt_custom_parsed_at_idx(
                    "submitteddate",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            outagestatuscode: row.get_opt_range("outagestatuscode", field_mapping.0[7])?,
            resubmitreason: row.get_opt_range("resubmitreason", field_mapping.0[8])?,
            resubmitoutageid: row
                .get_opt_custom_parsed_at_idx(
                    "resubmitoutageid",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            recalltimeday: row
                .get_opt_custom_parsed_at_idx(
                    "recalltimeday",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            recalltimenight: row
                .get_opt_custom_parsed_at_idx(
                    "recalltimenight",
                    field_mapping.0[11],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[12],
                    mmsdm_core::mms_datetime::parse,
                )?,
            reason: row.get_opt_range("reason", field_mapping.0[13])?,
            issecondary: row
                .get_opt_custom_parsed_at_idx(
                    "issecondary",
                    field_mapping.0[14],
                    mmsdm_core::mms_decimal::parse,
                )?,
            actual_starttime: row
                .get_opt_custom_parsed_at_idx(
                    "actual_starttime",
                    field_mapping.0[15],
                    mmsdm_core::mms_datetime::parse,
                )?,
            actual_endtime: row
                .get_opt_custom_parsed_at_idx(
                    "actual_endtime",
                    field_mapping.0[16],
                    mmsdm_core::mms_datetime::parse,
                )?,
            companyrefcode: row.get_opt_range("companyrefcode", field_mapping.0[17])?,
            elementid: row
                .get_custom_parsed_at_idx(
                    "elementid",
                    field_mapping.0[18],
                    mmsdm_core::mms_decimal::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(NetworkOutagedetail4Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> NetworkOutagedetail4PrimaryKey {
        NetworkOutagedetail4PrimaryKey {
            elementid: row.elementid,
            equipmentid: row.equipmentid().to_string(),
            equipmenttype: row.equipmenttype().to_string(),
            outageid: row.outageid,
            starttime: row.starttime,
            substationid: row.substationid().to_string(),
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "network_outagedetail_v4".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        NetworkOutagedetail4Row {
            outageid: row.outageid.clone(),
            substationid: row.substationid.clone(),
            equipmenttype: row.equipmenttype.clone(),
            equipmentid: row.equipmentid.clone(),
            starttime: row.starttime.clone(),
            endtime: row.endtime.clone(),
            submitteddate: row.submitteddate.clone(),
            outagestatuscode: row.outagestatuscode.clone(),
            resubmitreason: row.resubmitreason.clone(),
            resubmitoutageid: row.resubmitoutageid.clone(),
            recalltimeday: row.recalltimeday.clone(),
            recalltimenight: row.recalltimenight.clone(),
            lastchanged: row.lastchanged.clone(),
            reason: row.reason.clone(),
            issecondary: row.issecondary.clone(),
            actual_starttime: row.actual_starttime.clone(),
            actual_endtime: row.actual_endtime.clone(),
            companyrefcode: row.companyrefcode.clone(),
            elementid: row.elementid.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkOutagedetail4PrimaryKey {
    pub elementid: rust_decimal::Decimal,
    pub equipmentid: alloc::string::String,
    pub equipmenttype: alloc::string::String,
    pub outageid: rust_decimal::Decimal,
    pub starttime: chrono::NaiveDateTime,
    pub substationid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for NetworkOutagedetail4PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for NetworkOutagedetail4Row<'data> {
    type Row<'other> = NetworkOutagedetail4Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.elementid == row.elementid && self.equipmentid() == row.equipmentid()
            && self.equipmenttype() == row.equipmenttype()
            && self.outageid == row.outageid && self.starttime == row.starttime
            && self.substationid() == row.substationid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for NetworkOutagedetail4Row<'data> {
    type PrimaryKey = NetworkOutagedetail4PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.elementid == key.elementid && self.equipmentid() == key.equipmentid
            && self.equipmenttype() == key.equipmenttype && self.outageid == key.outageid
            && self.starttime == key.starttime && self.substationid() == key.substationid
    }
}
impl<'data> mmsdm_core::CompareWithRow for NetworkOutagedetail4PrimaryKey {
    type Row<'other> = NetworkOutagedetail4Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.elementid == row.elementid && self.equipmentid == row.equipmentid()
            && self.equipmenttype == row.equipmenttype() && self.outageid == row.outageid
            && self.starttime == row.starttime && self.substationid == row.substationid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for NetworkOutagedetail4PrimaryKey {
    type PrimaryKey = NetworkOutagedetail4PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.elementid == key.elementid && self.equipmentid == key.equipmentid
            && self.equipmenttype == key.equipmenttype && self.outageid == key.outageid
            && self.starttime == key.starttime && self.substationid == key.substationid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for NetworkOutagedetail4 {
    type Builder = NetworkOutagedetail4Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "outageid",
                    arrow::datatypes::DataType::Decimal128(15, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "substationid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "equipmenttype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "equipmentid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "starttime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "endtime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "submitteddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "outagestatuscode",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "resubmitreason",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "resubmitoutageid",
                    arrow::datatypes::DataType::Decimal128(15, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recalltimeday",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "recalltimenight",
                    arrow::datatypes::DataType::Decimal128(10, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reason",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "issecondary",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "actual_starttime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "actual_endtime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "companyrefcode",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "elementid",
                    arrow::datatypes::DataType::Decimal128(15, 0),
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        NetworkOutagedetail4Builder {
            outageid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 0)),
            substationid_array: arrow::array::builder::StringBuilder::new(),
            equipmenttype_array: arrow::array::builder::StringBuilder::new(),
            equipmentid_array: arrow::array::builder::StringBuilder::new(),
            starttime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            endtime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            submitteddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            outagestatuscode_array: arrow::array::builder::StringBuilder::new(),
            resubmitreason_array: arrow::array::builder::StringBuilder::new(),
            resubmitoutageid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 0)),
            recalltimeday_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            recalltimenight_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(10, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            reason_array: arrow::array::builder::StringBuilder::new(),
            issecondary_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            actual_starttime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            actual_endtime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            companyrefcode_array: arrow::array::builder::StringBuilder::new(),
            elementid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder
            .outageid_array
            .append_value({
                let mut val = row.outageid;
                val.rescale(0);
                val.mantissa()
            });
        builder.substationid_array.append_value(row.substationid());
        builder.equipmenttype_array.append_value(row.equipmenttype());
        builder.equipmentid_array.append_value(row.equipmentid());
        builder.starttime_array.append_value(row.starttime.timestamp_millis());
        builder
            .endtime_array
            .append_option(row.endtime.map(|val| val.timestamp_millis()));
        builder
            .submitteddate_array
            .append_option(row.submitteddate.map(|val| val.timestamp_millis()));
        builder.outagestatuscode_array.append_option(row.outagestatuscode());
        builder.resubmitreason_array.append_option(row.resubmitreason());
        builder
            .resubmitoutageid_array
            .append_option({
                row.resubmitoutageid
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .recalltimeday_array
            .append_option({
                row.recalltimeday
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .recalltimenight_array
            .append_option({
                row.recalltimenight
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder.reason_array.append_option(row.reason());
        builder
            .issecondary_array
            .append_option({
                row.issecondary
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .actual_starttime_array
            .append_option(row.actual_starttime.map(|val| val.timestamp_millis()));
        builder
            .actual_endtime_array
            .append_option(row.actual_endtime.map(|val| val.timestamp_millis()));
        builder.companyrefcode_array.append_option(row.companyrefcode());
        builder
            .elementid_array
            .append_value({
                let mut val = row.elementid;
                val.rescale(0);
                val.mantissa()
            });
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.outageid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.substationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.equipmenttype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.equipmentid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.starttime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.endtime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.submitteddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.outagestatuscode_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.resubmitreason_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.resubmitoutageid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.recalltimeday_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.recalltimenight_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reason_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.issecondary_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.actual_starttime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.actual_endtime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.companyrefcode_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.elementid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct NetworkOutagedetail4Builder {
    outageid_array: arrow::array::builder::Decimal128Builder,
    substationid_array: arrow::array::builder::StringBuilder,
    equipmenttype_array: arrow::array::builder::StringBuilder,
    equipmentid_array: arrow::array::builder::StringBuilder,
    starttime_array: arrow::array::builder::TimestampMillisecondBuilder,
    endtime_array: arrow::array::builder::TimestampMillisecondBuilder,
    submitteddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    outagestatuscode_array: arrow::array::builder::StringBuilder,
    resubmitreason_array: arrow::array::builder::StringBuilder,
    resubmitoutageid_array: arrow::array::builder::Decimal128Builder,
    recalltimeday_array: arrow::array::builder::Decimal128Builder,
    recalltimenight_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    reason_array: arrow::array::builder::StringBuilder,
    issecondary_array: arrow::array::builder::Decimal128Builder,
    actual_starttime_array: arrow::array::builder::TimestampMillisecondBuilder,
    actual_endtime_array: arrow::array::builder::TimestampMillisecondBuilder,
    companyrefcode_array: arrow::array::builder::StringBuilder,
    elementid_array: arrow::array::builder::Decimal128Builder,
}
pub struct NetworkOutagestatuscode1;
pub struct NetworkOutagestatuscode1Mapping([usize; 3]);
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
///
///
/// # Primary Key Columns
///
/// * OUTAGESTATUSCODE
#[derive(Debug, PartialEq, Eq)]
pub struct NetworkOutagestatuscode1Row<'data> {
    /// A code representing the status of an outage
    pub outagestatuscode: core::ops::Range<usize>,
    /// A description of the status code
    pub description: core::ops::Range<usize>,
    /// The time that this record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> NetworkOutagestatuscode1Row<'data> {
    pub fn outagestatuscode(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.outagestatuscode.clone(),
        )
    }
    pub fn description(&self) -> Option<&str> {
        if self.description.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.description.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for NetworkOutagestatuscode1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "NETWORK";
    const TABLE_NAME: &'static str = "OUTAGESTATUSCODE";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = NetworkOutagestatuscode1Mapping([
        4,
        5,
        6,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "OUTAGESTATUSCODE",
        "DESCRIPTION",
        "LASTCHANGED",
    ];
    type Row<'row> = NetworkOutagestatuscode1Row<'row>;
    type FieldMapping = NetworkOutagestatuscode1Mapping;
    type PrimaryKey = NetworkOutagestatuscode1PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(NetworkOutagestatuscode1Row {
            outagestatuscode: row.get_range("outagestatuscode", field_mapping.0[0])?,
            description: row.get_opt_range("description", field_mapping.0[1])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(NetworkOutagestatuscode1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> NetworkOutagestatuscode1PrimaryKey {
        NetworkOutagestatuscode1PrimaryKey {
            outagestatuscode: row.outagestatuscode().to_string(),
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "network_outagestatuscode_v1".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        NetworkOutagestatuscode1Row {
            outagestatuscode: row.outagestatuscode.clone(),
            description: row.description.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkOutagestatuscode1PrimaryKey {
    pub outagestatuscode: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for NetworkOutagestatuscode1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for NetworkOutagestatuscode1Row<'data> {
    type Row<'other> = NetworkOutagestatuscode1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.outagestatuscode() == row.outagestatuscode()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for NetworkOutagestatuscode1Row<'data> {
    type PrimaryKey = NetworkOutagestatuscode1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.outagestatuscode() == key.outagestatuscode
    }
}
impl<'data> mmsdm_core::CompareWithRow for NetworkOutagestatuscode1PrimaryKey {
    type Row<'other> = NetworkOutagestatuscode1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.outagestatuscode == row.outagestatuscode()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for NetworkOutagestatuscode1PrimaryKey {
    type PrimaryKey = NetworkOutagestatuscode1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.outagestatuscode == key.outagestatuscode
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for NetworkOutagestatuscode1 {
    type Builder = NetworkOutagestatuscode1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "outagestatuscode",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "description",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        NetworkOutagestatuscode1Builder {
            outagestatuscode_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.outagestatuscode_array.append_value(row.outagestatuscode());
        builder.description_array.append_option(row.description());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.outagestatuscode_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct NetworkOutagestatuscode1Builder {
    outagestatuscode_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct NetworkRating1;
pub struct NetworkRating1Mapping([usize; 10]);
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
///
///
/// # Primary Key Columns
///
/// * SPD_ID
/// * VALIDFROM
#[derive(Debug, PartialEq, Eq)]
pub struct NetworkRating1Row<'data> {
    /// ID defining this data source for use in constraints
    pub spd_id: core::ops::Range<usize>,
    /// The date that this record is applies from (inclusive)
    pub validfrom: chrono::NaiveDateTime,
    /// The date that this record applies until (exclusive)
    pub validto: Option<chrono::NaiveDateTime>,
    /// The region that this rating is for
    pub regionid: core::ops::Range<usize>,
    /// The substation the equipment is located at
    pub substationid: core::ops::Range<usize>,
    /// The type of equipment. Valid values are:<br>LINE = Line<br>TRANS = Transformer<br>CB = Circuit breaker<br>ISOL = Isolator<br>CAP = Capacitor<br>REAC = Reactor<br>UNIT = Unit<br>
    pub equipmenttype: core::ops::Range<usize>,
    /// A unique identifier for this equipment at this substation, and based on its type
    pub equipmentid: core::ops::Range<usize>,
    /// The rating level of the value used, one of:<br>NORM = Continuous rating value. Applied under pre-contingent conditions.<br>EMER = Continuous rating value. Applied under pre-contingent conditions<br>LDSH = Load Shedding<br>
    pub ratinglevel: core::ops::Range<usize>,
    /// One of:<br>1 = Normally uses dynamic ratings<br>0 = No dynamic ratings, static ratings are used<br>
    pub isdynamic: Option<rust_decimal::Decimal>,
    /// The time that this record was last changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> NetworkRating1Row<'data> {
    pub fn spd_id(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.spd_id.clone())
    }
    pub fn regionid(&self) -> Option<&str> {
        if self.regionid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.regionid.clone(),
                ),
            )
        }
    }
    pub fn substationid(&self) -> Option<&str> {
        if self.substationid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.substationid.clone(),
                ),
            )
        }
    }
    pub fn equipmenttype(&self) -> Option<&str> {
        if self.equipmenttype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.equipmenttype.clone(),
                ),
            )
        }
    }
    pub fn equipmentid(&self) -> Option<&str> {
        if self.equipmentid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.equipmentid.clone(),
                ),
            )
        }
    }
    pub fn ratinglevel(&self) -> Option<&str> {
        if self.ratinglevel.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.ratinglevel.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for NetworkRating1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "NETWORK";
    const TABLE_NAME: &'static str = "RATING";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = NetworkRating1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
        13,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SPD_ID",
        "VALIDFROM",
        "VALIDTO",
        "REGIONID",
        "SUBSTATIONID",
        "EQUIPMENTTYPE",
        "EQUIPMENTID",
        "RATINGLEVEL",
        "ISDYNAMIC",
        "LASTCHANGED",
    ];
    type Row<'row> = NetworkRating1Row<'row>;
    type FieldMapping = NetworkRating1Mapping;
    type PrimaryKey = NetworkRating1PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(NetworkRating1Row {
            spd_id: row.get_range("spd_id", field_mapping.0[0])?,
            validfrom: row
                .get_custom_parsed_at_idx(
                    "validfrom",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            validto: row
                .get_opt_custom_parsed_at_idx(
                    "validto",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            regionid: row.get_opt_range("regionid", field_mapping.0[3])?,
            substationid: row.get_opt_range("substationid", field_mapping.0[4])?,
            equipmenttype: row.get_opt_range("equipmenttype", field_mapping.0[5])?,
            equipmentid: row.get_opt_range("equipmentid", field_mapping.0[6])?,
            ratinglevel: row.get_opt_range("ratinglevel", field_mapping.0[7])?,
            isdynamic: row
                .get_opt_custom_parsed_at_idx(
                    "isdynamic",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[9],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(NetworkRating1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> NetworkRating1PrimaryKey {
        NetworkRating1PrimaryKey {
            spd_id: row.spd_id().to_string(),
            validfrom: row.validfrom,
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "network_rating_v1".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        NetworkRating1Row {
            spd_id: row.spd_id.clone(),
            validfrom: row.validfrom.clone(),
            validto: row.validto.clone(),
            regionid: row.regionid.clone(),
            substationid: row.substationid.clone(),
            equipmenttype: row.equipmenttype.clone(),
            equipmentid: row.equipmentid.clone(),
            ratinglevel: row.ratinglevel.clone(),
            isdynamic: row.isdynamic.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkRating1PrimaryKey {
    pub spd_id: alloc::string::String,
    pub validfrom: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for NetworkRating1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for NetworkRating1Row<'data> {
    type Row<'other> = NetworkRating1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.spd_id() == row.spd_id() && self.validfrom == row.validfrom
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for NetworkRating1Row<'data> {
    type PrimaryKey = NetworkRating1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.spd_id() == key.spd_id && self.validfrom == key.validfrom
    }
}
impl<'data> mmsdm_core::CompareWithRow for NetworkRating1PrimaryKey {
    type Row<'other> = NetworkRating1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.spd_id == row.spd_id() && self.validfrom == row.validfrom
    }
}
impl mmsdm_core::CompareWithPrimaryKey for NetworkRating1PrimaryKey {
    type PrimaryKey = NetworkRating1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.spd_id == key.spd_id && self.validfrom == key.validfrom
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for NetworkRating1 {
    type Builder = NetworkRating1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "spd_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "validfrom",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "validto",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "substationid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "equipmenttype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "equipmentid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ratinglevel",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "isdynamic",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        NetworkRating1Builder {
            spd_id_array: arrow::array::builder::StringBuilder::new(),
            validfrom_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            validto_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            substationid_array: arrow::array::builder::StringBuilder::new(),
            equipmenttype_array: arrow::array::builder::StringBuilder::new(),
            equipmentid_array: arrow::array::builder::StringBuilder::new(),
            ratinglevel_array: arrow::array::builder::StringBuilder::new(),
            isdynamic_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.spd_id_array.append_value(row.spd_id());
        builder.validfrom_array.append_value(row.validfrom.timestamp_millis());
        builder
            .validto_array
            .append_option(row.validto.map(|val| val.timestamp_millis()));
        builder.regionid_array.append_option(row.regionid());
        builder.substationid_array.append_option(row.substationid());
        builder.equipmenttype_array.append_option(row.equipmenttype());
        builder.equipmentid_array.append_option(row.equipmentid());
        builder.ratinglevel_array.append_option(row.ratinglevel());
        builder
            .isdynamic_array
            .append_option({
                row.isdynamic
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.spd_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.validfrom_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.validto_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.substationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.equipmenttype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.equipmentid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ratinglevel_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.isdynamic_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct NetworkRating1Builder {
    spd_id_array: arrow::array::builder::StringBuilder,
    validfrom_array: arrow::array::builder::TimestampMillisecondBuilder,
    validto_array: arrow::array::builder::TimestampMillisecondBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    substationid_array: arrow::array::builder::StringBuilder,
    equipmenttype_array: arrow::array::builder::StringBuilder,
    equipmentid_array: arrow::array::builder::StringBuilder,
    ratinglevel_array: arrow::array::builder::StringBuilder,
    isdynamic_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct NetworkRealtimerating1;
pub struct NetworkRealtimerating1Mapping([usize; 3]);
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
///
///
/// # Primary Key Columns
///
/// * SETTLEMENTDATE
/// * SPD_ID
#[derive(Debug, PartialEq, Eq)]
pub struct NetworkRealtimerating1Row<'data> {
    /// The dispatch interval the rating applies to
    pub settlementdate: chrono::NaiveDateTime,
    /// ID defining this data source for use in constraints
    pub spd_id: core::ops::Range<usize>,
    /// The defined equipment rating value in MVA
    pub ratingvalue: rust_decimal::Decimal,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> NetworkRealtimerating1Row<'data> {
    pub fn spd_id(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.spd_id.clone())
    }
}
impl mmsdm_core::GetTable for NetworkRealtimerating1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "NETWORK";
    const TABLE_NAME: &'static str = "REALTIMERATING";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = NetworkRealtimerating1Mapping([
        4,
        5,
        6,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SETTLEMENTDATE",
        "SPD_ID",
        "RATINGVALUE",
    ];
    type Row<'row> = NetworkRealtimerating1Row<'row>;
    type FieldMapping = NetworkRealtimerating1Mapping;
    type PrimaryKey = NetworkRealtimerating1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(NetworkRealtimerating1Row {
            settlementdate: row
                .get_custom_parsed_at_idx(
                    "settlementdate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            spd_id: row.get_range("spd_id", field_mapping.0[1])?,
            ratingvalue: row
                .get_custom_parsed_at_idx(
                    "ratingvalue",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(NetworkRealtimerating1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let settlementdate = row
            .get_custom_parsed_at_idx(
                "settlementdate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(settlementdate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> NetworkRealtimerating1PrimaryKey {
        NetworkRealtimerating1PrimaryKey {
            settlementdate: row.settlementdate,
            spd_id: row.spd_id().to_string(),
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.settlementdate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.settlementdate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "network_realtimerating_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        NetworkRealtimerating1Row {
            settlementdate: row.settlementdate.clone(),
            spd_id: row.spd_id.clone(),
            ratingvalue: row.ratingvalue.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkRealtimerating1PrimaryKey {
    pub settlementdate: chrono::NaiveDateTime,
    pub spd_id: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for NetworkRealtimerating1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for NetworkRealtimerating1Row<'data> {
    type Row<'other> = NetworkRealtimerating1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.settlementdate == row.settlementdate && self.spd_id() == row.spd_id()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for NetworkRealtimerating1Row<'data> {
    type PrimaryKey = NetworkRealtimerating1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.settlementdate == key.settlementdate && self.spd_id() == key.spd_id
    }
}
impl<'data> mmsdm_core::CompareWithRow for NetworkRealtimerating1PrimaryKey {
    type Row<'other> = NetworkRealtimerating1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.settlementdate == row.settlementdate && self.spd_id == row.spd_id()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for NetworkRealtimerating1PrimaryKey {
    type PrimaryKey = NetworkRealtimerating1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.settlementdate == key.settlementdate && self.spd_id == key.spd_id
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for NetworkRealtimerating1 {
    type Builder = NetworkRealtimerating1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "settlementdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "spd_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "ratingvalue",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        NetworkRealtimerating1Builder {
            settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            spd_id_array: arrow::array::builder::StringBuilder::new(),
            ratingvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.settlementdate_array.append_value(row.settlementdate.timestamp_millis());
        builder.spd_id_array.append_value(row.spd_id());
        builder
            .ratingvalue_array
            .append_value({
                let mut val = row.ratingvalue;
                val.rescale(6);
                val.mantissa()
            });
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.settlementdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.spd_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ratingvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct NetworkRealtimerating1Builder {
    settlementdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    spd_id_array: arrow::array::builder::StringBuilder,
    ratingvalue_array: arrow::array::builder::Decimal128Builder,
}
pub struct NetworkStaticrating1;
pub struct NetworkStaticrating1Mapping([usize; 9]);
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
///
///
/// # Primary Key Columns
///
/// * APPLICATIONID
/// * EQUIPMENTID
/// * EQUIPMENTTYPE
/// * RATINGLEVEL
/// * SUBSTATIONID
/// * VALIDFROM
#[derive(Debug, PartialEq, Eq)]
pub struct NetworkStaticrating1Row<'data> {
    /// The substation the equipment is located at
    pub substationid: core::ops::Range<usize>,
    /// The type of equipment. Valid values are:<br>LINE = Line<br>TRANS = Transformer<br>CB = Circuit breaker<br>ISOL = Isolator<br>CAP = Capacitor<br>REAC = Reactor<br>UNIT = Unit<br>
    pub equipmenttype: core::ops::Range<usize>,
    /// A unique identifier for this type of equipment at this substation
    pub equipmentid: core::ops::Range<usize>,
    /// The rating level of the value used, one of:<br>NORM = Continuous rating value. Applied under pre-contingent conditions.<br>EMER = Continuous rating value. Applied under pre-contingent conditions<br>LDSH = Load Shedding
    pub ratinglevel: core::ops::Range<usize>,
    /// The applicationid which defines the application timeframes of this rating.
    pub applicationid: core::ops::Range<usize>,
    /// The date that this record is applies from (inclusive)
    pub validfrom: chrono::NaiveDateTime,
    /// The date that this record applies until (exclusive)
    pub validto: Option<chrono::NaiveDateTime>,
    /// The rating value in MVA that applies. This may be positive or negative depending on which side of the nominal MW flow direction the rating value applies.<br>Flow into a transmission device is positive, flow out of the device is negative.<br>
    pub ratingvalue: Option<rust_decimal::Decimal>,
    /// The time that this record was last changed.
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> NetworkStaticrating1Row<'data> {
    pub fn substationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.substationid.clone())
    }
    pub fn equipmenttype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.equipmenttype.clone())
    }
    pub fn equipmentid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.equipmentid.clone())
    }
    pub fn ratinglevel(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.ratinglevel.clone())
    }
    pub fn applicationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.applicationid.clone())
    }
}
impl mmsdm_core::GetTable for NetworkStaticrating1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "NETWORK";
    const TABLE_NAME: &'static str = "STATICRATING";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = NetworkStaticrating1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11,
        12,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SUBSTATIONID",
        "EQUIPMENTTYPE",
        "EQUIPMENTID",
        "RATINGLEVEL",
        "APPLICATIONID",
        "VALIDFROM",
        "VALIDTO",
        "RATINGVALUE",
        "LASTCHANGED",
    ];
    type Row<'row> = NetworkStaticrating1Row<'row>;
    type FieldMapping = NetworkStaticrating1Mapping;
    type PrimaryKey = NetworkStaticrating1PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(NetworkStaticrating1Row {
            substationid: row.get_range("substationid", field_mapping.0[0])?,
            equipmenttype: row.get_range("equipmenttype", field_mapping.0[1])?,
            equipmentid: row.get_range("equipmentid", field_mapping.0[2])?,
            ratinglevel: row.get_range("ratinglevel", field_mapping.0[3])?,
            applicationid: row.get_range("applicationid", field_mapping.0[4])?,
            validfrom: row
                .get_custom_parsed_at_idx(
                    "validfrom",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            validto: row
                .get_opt_custom_parsed_at_idx(
                    "validto",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            ratingvalue: row
                .get_opt_custom_parsed_at_idx(
                    "ratingvalue",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(NetworkStaticrating1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> NetworkStaticrating1PrimaryKey {
        NetworkStaticrating1PrimaryKey {
            applicationid: row.applicationid().to_string(),
            equipmentid: row.equipmentid().to_string(),
            equipmenttype: row.equipmenttype().to_string(),
            ratinglevel: row.ratinglevel().to_string(),
            substationid: row.substationid().to_string(),
            validfrom: row.validfrom,
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "network_staticrating_v1".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        NetworkStaticrating1Row {
            substationid: row.substationid.clone(),
            equipmenttype: row.equipmenttype.clone(),
            equipmentid: row.equipmentid.clone(),
            ratinglevel: row.ratinglevel.clone(),
            applicationid: row.applicationid.clone(),
            validfrom: row.validfrom.clone(),
            validto: row.validto.clone(),
            ratingvalue: row.ratingvalue.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkStaticrating1PrimaryKey {
    pub applicationid: alloc::string::String,
    pub equipmentid: alloc::string::String,
    pub equipmenttype: alloc::string::String,
    pub ratinglevel: alloc::string::String,
    pub substationid: alloc::string::String,
    pub validfrom: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for NetworkStaticrating1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for NetworkStaticrating1Row<'data> {
    type Row<'other> = NetworkStaticrating1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.applicationid() == row.applicationid()
            && self.equipmentid() == row.equipmentid()
            && self.equipmenttype() == row.equipmenttype()
            && self.ratinglevel() == row.ratinglevel()
            && self.substationid() == row.substationid()
            && self.validfrom == row.validfrom
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for NetworkStaticrating1Row<'data> {
    type PrimaryKey = NetworkStaticrating1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.applicationid() == key.applicationid
            && self.equipmentid() == key.equipmentid
            && self.equipmenttype() == key.equipmenttype
            && self.ratinglevel() == key.ratinglevel
            && self.substationid() == key.substationid && self.validfrom == key.validfrom
    }
}
impl<'data> mmsdm_core::CompareWithRow for NetworkStaticrating1PrimaryKey {
    type Row<'other> = NetworkStaticrating1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.applicationid == row.applicationid()
            && self.equipmentid == row.equipmentid()
            && self.equipmenttype == row.equipmenttype()
            && self.ratinglevel == row.ratinglevel()
            && self.substationid == row.substationid() && self.validfrom == row.validfrom
    }
}
impl mmsdm_core::CompareWithPrimaryKey for NetworkStaticrating1PrimaryKey {
    type PrimaryKey = NetworkStaticrating1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.applicationid == key.applicationid && self.equipmentid == key.equipmentid
            && self.equipmenttype == key.equipmenttype
            && self.ratinglevel == key.ratinglevel
            && self.substationid == key.substationid && self.validfrom == key.validfrom
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for NetworkStaticrating1 {
    type Builder = NetworkStaticrating1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "substationid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "equipmenttype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "equipmentid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "ratinglevel",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "applicationid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "validfrom",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "validto",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ratingvalue",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        NetworkStaticrating1Builder {
            substationid_array: arrow::array::builder::StringBuilder::new(),
            equipmenttype_array: arrow::array::builder::StringBuilder::new(),
            equipmentid_array: arrow::array::builder::StringBuilder::new(),
            ratinglevel_array: arrow::array::builder::StringBuilder::new(),
            applicationid_array: arrow::array::builder::StringBuilder::new(),
            validfrom_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            validto_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            ratingvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.substationid_array.append_value(row.substationid());
        builder.equipmenttype_array.append_value(row.equipmenttype());
        builder.equipmentid_array.append_value(row.equipmentid());
        builder.ratinglevel_array.append_value(row.ratinglevel());
        builder.applicationid_array.append_value(row.applicationid());
        builder.validfrom_array.append_value(row.validfrom.timestamp_millis());
        builder
            .validto_array
            .append_option(row.validto.map(|val| val.timestamp_millis()));
        builder
            .ratingvalue_array
            .append_option({
                row.ratingvalue
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.substationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.equipmenttype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.equipmentid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ratinglevel_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.applicationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.validfrom_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.validto_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ratingvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct NetworkStaticrating1Builder {
    substationid_array: arrow::array::builder::StringBuilder,
    equipmenttype_array: arrow::array::builder::StringBuilder,
    equipmentid_array: arrow::array::builder::StringBuilder,
    ratinglevel_array: arrow::array::builder::StringBuilder,
    applicationid_array: arrow::array::builder::StringBuilder,
    validfrom_array: arrow::array::builder::TimestampMillisecondBuilder,
    validto_array: arrow::array::builder::TimestampMillisecondBuilder,
    ratingvalue_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct NetworkSubstationdetail2;
pub struct NetworkSubstationdetail2Mapping([usize; 7]);
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
///
///
/// # Primary Key Columns
///
/// * SUBSTATIONID
/// * VALIDFROM
#[derive(Debug, PartialEq, Eq)]
pub struct NetworkSubstationdetail2Row<'data> {
    /// ID uniquely identifying this substation
    pub substationid: core::ops::Range<usize>,
    /// The record is valid from this date (inclusive)
    pub validfrom: chrono::NaiveDateTime,
    /// The record is valid up until this date (exclusive)
    pub validto: Option<chrono::NaiveDateTime>,
    /// Description of the substation
    pub description: core::ops::Range<usize>,
    /// The NEM region the substation is in
    pub regionid: core::ops::Range<usize>,
    /// The TNSP who is responsible for this substation
    pub ownerid: core::ops::Range<usize>,
    /// The time that this record was last changed.
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> NetworkSubstationdetail2Row<'data> {
    pub fn substationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.substationid.clone())
    }
    pub fn description(&self) -> Option<&str> {
        if self.description.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.description.clone(),
                ),
            )
        }
    }
    pub fn regionid(&self) -> Option<&str> {
        if self.regionid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.regionid.clone(),
                ),
            )
        }
    }
    pub fn ownerid(&self) -> Option<&str> {
        if self.ownerid.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.ownerid.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for NetworkSubstationdetail2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "NETWORK";
    const TABLE_NAME: &'static str = "SUBSTATIONDETAIL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = NetworkSubstationdetail2Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SUBSTATIONID",
        "VALIDFROM",
        "VALIDTO",
        "DESCRIPTION",
        "REGIONID",
        "OWNERID",
        "LASTCHANGED",
    ];
    type Row<'row> = NetworkSubstationdetail2Row<'row>;
    type FieldMapping = NetworkSubstationdetail2Mapping;
    type PrimaryKey = NetworkSubstationdetail2PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(NetworkSubstationdetail2Row {
            substationid: row.get_range("substationid", field_mapping.0[0])?,
            validfrom: row
                .get_custom_parsed_at_idx(
                    "validfrom",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            validto: row
                .get_opt_custom_parsed_at_idx(
                    "validto",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            description: row.get_opt_range("description", field_mapping.0[3])?,
            regionid: row.get_opt_range("regionid", field_mapping.0[4])?,
            ownerid: row.get_opt_range("ownerid", field_mapping.0[5])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            backing_data: row,
        })
    }
    fn field_mapping_from_row<'a>(
        mut row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::FieldMapping> {
        if !matches!(row.record_type(), mmsdm_core::RecordType::I) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!("Expected an I row but got {row:?}"),
                ),
            );
        }
        let row_key = mmsdm_core::FileKey::from_row(row.borrow())?;
        if !Self::matches_file_key(&row_key, row_key.version) {
            return Err(
                mmsdm_core::Error::UnexpectedRowType(
                    alloc::format!(
                        "Expected a row matching {}.{}.v{} but got {row_key}",
                        Self::DATA_SET_NAME, Self::TABLE_NAME, Self::VERSION
                    ),
                ),
            );
        }
        let mut base_mapping = Self::DEFAULT_FIELD_MAPPING.0;
        for (field_index, field) in Self::COLUMNS.iter().enumerate() {
            base_mapping[field_index] = row
                .iter_fields()
                .position(|f| f == *field)
                .unwrap_or(usize::MAX);
        }
        Ok(NetworkSubstationdetail2Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        _row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        Ok(())
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> NetworkSubstationdetail2PrimaryKey {
        NetworkSubstationdetail2PrimaryKey {
            substationid: row.substationid().to_string(),
            validfrom: row.validfrom,
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "network_substationdetail_v2".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        NetworkSubstationdetail2Row {
            substationid: row.substationid.clone(),
            validfrom: row.validfrom.clone(),
            validto: row.validto.clone(),
            description: row.description.clone(),
            regionid: row.regionid.clone(),
            ownerid: row.ownerid.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NetworkSubstationdetail2PrimaryKey {
    pub substationid: alloc::string::String,
    pub validfrom: chrono::NaiveDateTime,
}
impl mmsdm_core::PrimaryKey for NetworkSubstationdetail2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for NetworkSubstationdetail2Row<'data> {
    type Row<'other> = NetworkSubstationdetail2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.substationid() == row.substationid() && self.validfrom == row.validfrom
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for NetworkSubstationdetail2Row<'data> {
    type PrimaryKey = NetworkSubstationdetail2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.substationid() == key.substationid && self.validfrom == key.validfrom
    }
}
impl<'data> mmsdm_core::CompareWithRow for NetworkSubstationdetail2PrimaryKey {
    type Row<'other> = NetworkSubstationdetail2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.substationid == row.substationid() && self.validfrom == row.validfrom
    }
}
impl mmsdm_core::CompareWithPrimaryKey for NetworkSubstationdetail2PrimaryKey {
    type PrimaryKey = NetworkSubstationdetail2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.substationid == key.substationid && self.validfrom == key.validfrom
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for NetworkSubstationdetail2 {
    type Builder = NetworkSubstationdetail2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "substationid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "validfrom",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "validto",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "description",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "ownerid",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lastchanged",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        NetworkSubstationdetail2Builder {
            substationid_array: arrow::array::builder::StringBuilder::new(),
            validfrom_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            validto_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            regionid_array: arrow::array::builder::StringBuilder::new(),
            ownerid_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.substationid_array.append_value(row.substationid());
        builder.validfrom_array.append_value(row.validfrom.timestamp_millis());
        builder
            .validto_array
            .append_option(row.validto.map(|val| val.timestamp_millis()));
        builder.description_array.append_option(row.description());
        builder.regionid_array.append_option(row.regionid());
        builder.ownerid_array.append_option(row.ownerid());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.substationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.validfrom_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.validto_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.ownerid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct NetworkSubstationdetail2Builder {
    substationid_array: arrow::array::builder::StringBuilder,
    validfrom_array: arrow::array::builder::TimestampMillisecondBuilder,
    validto_array: arrow::array::builder::TimestampMillisecondBuilder,
    description_array: arrow::array::builder::StringBuilder,
    regionid_array: arrow::array::builder::StringBuilder,
    ownerid_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
