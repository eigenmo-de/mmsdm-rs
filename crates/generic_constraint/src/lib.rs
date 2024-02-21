#![no_std]
#![allow(unused_imports)]
extern crate alloc;
use alloc::string::ToString;
use chrono::Datelike as _;
#[cfg(feature = "arrow")]
extern crate std;
pub struct GenericConstraintEmsmaster1;
pub struct GenericConstraintEmsmaster1Mapping([usize; 5]);
/// # Summary
///
/// ## EMSMASTER
///  _EMSMASTER provides a description of the SCADA measurements that are associated with the SPD_ID points utilised in generic equation RHS terms_
///
/// * Data Set Name: Generic Constraint
/// * File Name: Emsmaster
/// * Data Version: 1
///
///
///
///
///
/// # Primary Key Columns
///
/// * SPD_ID
/// * SPD_TYPE
#[derive(Debug, PartialEq, Eq)]
pub struct GenericConstraintEmsmaster1Row<'data> {
    /// ID defining data source
    pub spd_id: core::ops::Range<usize>,
    /// ID describing type of data source
    pub spd_type: core::ops::Range<usize>,
    /// The detailed description of the SCADA point associated with the SPD_ID
    pub description: core::ops::Range<usize>,
    /// The Grouping associated with the SPD ID - most often a RegionID
    pub grouping_id: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> GenericConstraintEmsmaster1Row<'data> {
    pub fn spd_id(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.spd_id.clone())
    }
    pub fn spd_type(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.spd_type.clone())
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
    pub fn grouping_id(&self) -> Option<&str> {
        if self.grouping_id.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.grouping_id.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for GenericConstraintEmsmaster1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "GENERIC_CONSTRAINT";
    const TABLE_NAME: &'static str = "EMSMASTER";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = GenericConstraintEmsmaster1Mapping([
        4,
        5,
        6,
        7,
        8,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "SPD_ID",
        "SPD_TYPE",
        "DESCRIPTION",
        "GROUPING_ID",
        "LASTCHANGED",
    ];
    type Row<'row> = GenericConstraintEmsmaster1Row<'row>;
    type FieldMapping = GenericConstraintEmsmaster1Mapping;
    type PrimaryKey = GenericConstraintEmsmaster1PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(GenericConstraintEmsmaster1Row {
            spd_id: row.get_range("spd_id", field_mapping.0[0])?,
            spd_type: row.get_range("spd_type", field_mapping.0[1])?,
            description: row.get_opt_range("description", field_mapping.0[2])?,
            grouping_id: row.get_opt_range("grouping_id", field_mapping.0[3])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[4],
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
        Ok(GenericConstraintEmsmaster1Mapping(base_mapping))
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
    fn primary_key(row: &Self::Row<'_>) -> GenericConstraintEmsmaster1PrimaryKey {
        GenericConstraintEmsmaster1PrimaryKey {
            spd_id: row.spd_id().to_string(),
            spd_type: row.spd_type().to_string(),
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "generic_constraint_emsmaster_v1".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        GenericConstraintEmsmaster1Row {
            spd_id: row.spd_id.clone(),
            spd_type: row.spd_type.clone(),
            description: row.description.clone(),
            grouping_id: row.grouping_id.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenericConstraintEmsmaster1PrimaryKey {
    pub spd_id: alloc::string::String,
    pub spd_type: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for GenericConstraintEmsmaster1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for GenericConstraintEmsmaster1Row<'data> {
    type Row<'other> = GenericConstraintEmsmaster1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.spd_id() == row.spd_id() && self.spd_type() == row.spd_type()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for GenericConstraintEmsmaster1Row<'data> {
    type PrimaryKey = GenericConstraintEmsmaster1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.spd_id() == key.spd_id && self.spd_type() == key.spd_type
    }
}
impl<'data> mmsdm_core::CompareWithRow for GenericConstraintEmsmaster1PrimaryKey {
    type Row<'other> = GenericConstraintEmsmaster1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.spd_id == row.spd_id() && self.spd_type == row.spd_type()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GenericConstraintEmsmaster1PrimaryKey {
    type PrimaryKey = GenericConstraintEmsmaster1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.spd_id == key.spd_id && self.spd_type == key.spd_type
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for GenericConstraintEmsmaster1 {
    type Builder = GenericConstraintEmsmaster1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "spd_id",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "spd_type",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "description",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "grouping_id",
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
        GenericConstraintEmsmaster1Builder {
            spd_id_array: arrow::array::builder::StringBuilder::new(),
            spd_type_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            grouping_id_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.spd_id_array.append_value(row.spd_id());
        builder.spd_type_array.append_value(row.spd_type());
        builder.description_array.append_option(row.description());
        builder.grouping_id_array.append_option(row.grouping_id());
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
                    alloc::sync::Arc::new(builder.spd_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.grouping_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct GenericConstraintEmsmaster1Builder {
    spd_id_array: arrow::array::builder::StringBuilder,
    spd_type_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    grouping_id_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct GencondataNull6;
pub struct GencondataNull6Mapping([usize; 26]);
/// # Summary
///
/// ## GENCONDATA
///  _GENCONDATA sets out the generic constraints contained within a generic constraint set invoked in PASA, predispatch and dispatch.<br>Fields enable selective application of invoked constraints in the Dispatch, Predispatch, ST PASA or MT PASA processes.<br>_
///
/// * Data Set Name: Gencondata
/// * File Name: Null
/// * Data Version: 6
///
/// # Description
///  GENCONDATA is a public data, and is available to all participants. Source GENCONDATA updates as constraint details are updated by AEMO. Note The following fields enable selective application of invoked constraints in the Dispatch, Predispatch, ST PASA or MT PASA processes: · DISPATCH · PREDISPATCH · STPASA · MTPASA The flag P5MIN_SCOPE_OVERRIDE indicates for each constraint whether 5MPD makes use of the default Dispatch (P5MIN_SCOPE_OVERRIDE = NULL) or Pre-dispatch (P5MIN_SCOPE_OVERRIDE = ‘PD’) style RHS definition. GENERICCONSTRAINTRHS stores generic constraint RHS definitions. Constraints without records in GENERICCONSTRAINTRHS only make use of the static RHS defined in the CONSTRAINTVALUE column in GENCONDATA . The default value for the P5MIN_SCOPE_OVERRIDE column is NULL, so constraints existing before implementing the column use the DISPATCH RHS definition by default, as was the case before the implementation of the change.
///
///
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * GENCONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct GencondataNull6Row<'data> {
    /// Effective date of this constraint
    pub effectivedate: chrono::NaiveDateTime,
    /// Version with respect to the effective date
    pub versionno: rust_decimal::Decimal,
    /// Unique ID for the constraint
    pub genconid: core::ops::Range<usize>,
    /// The logical operator (=, &gt;=, &lt;=)
    pub constrainttype: core::ops::Range<usize>,
    /// the RHS value used if there is no dynamic RHS defined in GenericConstraintRHS
    pub constraintvalue: Option<rust_decimal::Decimal>,
    /// Detail of the plant that is not in service
    pub description: core::ops::Range<usize>,
    /// Not used
    pub status: core::ops::Range<usize>,
    /// The constraint violation penalty factor
    pub genericconstraintweight: Option<rust_decimal::Decimal>,
    /// Date record authorised
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorising record
    pub authorisedby: core::ops::Range<usize>,
    /// Not used
    pub dynamicrhs: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag: constraint RHS used for Dispatch? 1-used, 0-not used
    pub dispatch: core::ops::Range<usize>,
    /// Flag to indicate if the constraint RHS is to be used for PreDispatch, 1-used, 0-not used
    pub predispatch: core::ops::Range<usize>,
    /// Flag to indicate if the constraint RHS is to be used for ST PASA, 1-used, 0-not used
    pub stpasa: core::ops::Range<usize>,
    /// Flag to indicate if the constraint RHS is to be used for MT PASA, 1-used, 0-not used
    pub mtpasa: core::ops::Range<usize>,
    /// The device(s) that is affected by the constraint e.g. Interconnector, Generator(s) or Cutset
    pub impact: core::ops::Range<usize>,
    /// The source of the constraint formulation
    pub source: core::ops::Range<usize>,
    /// The limit type of the constraint e.g. Transient Stability, Voltage Stability
    pub limittype: core::ops::Range<usize>,
    /// The contingency or reason for the constraint
    pub reason: core::ops::Range<usize>,
    /// Details of the changes made to this version of the constraint
    pub modifications: core::ops::Range<usize>,
    /// Extra notes on the constraint
    pub additionalnotes: core::ops::Range<usize>,
    /// Extra notes on the constraint: NULL = Dispatch RHS applied in 5MPD, PD = PreDispatch RHS applied in 5MPD
    pub p5min_scope_override: core::ops::Range<usize>,
    /// Flag to indicate if PASA LRC run uses the constraint; 1-used, 0-not used
    pub lrc: core::ops::Range<usize>,
    /// Flag to indicate if PASA LOR run uses the constraint; 1-used, 0-not used
    pub lor: core::ops::Range<usize>,
    /// Flags Constraints for which NEMDE must use "InitialMW" values instead of "WhatOfInitialMW" for Intervention Pricing runs
    pub force_scada: Option<rust_decimal::Decimal>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> GencondataNull6Row<'data> {
    pub fn genconid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.genconid.clone())
    }
    pub fn constrainttype(&self) -> Option<&str> {
        if self.constrainttype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.constrainttype.clone(),
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
    pub fn status(&self) -> Option<&str> {
        if self.status.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.status.clone(),
                ),
            )
        }
    }
    pub fn authorisedby(&self) -> Option<&str> {
        if self.authorisedby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.authorisedby.clone(),
                ),
            )
        }
    }
    pub fn dispatch(&self) -> Option<&str> {
        if self.dispatch.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.dispatch.clone(),
                ),
            )
        }
    }
    pub fn predispatch(&self) -> Option<&str> {
        if self.predispatch.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.predispatch.clone(),
                ),
            )
        }
    }
    pub fn stpasa(&self) -> Option<&str> {
        if self.stpasa.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.stpasa.clone(),
                ),
            )
        }
    }
    pub fn mtpasa(&self) -> Option<&str> {
        if self.mtpasa.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.mtpasa.clone(),
                ),
            )
        }
    }
    pub fn impact(&self) -> Option<&str> {
        if self.impact.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.impact.clone(),
                ),
            )
        }
    }
    pub fn source(&self) -> Option<&str> {
        if self.source.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.source.clone(),
                ),
            )
        }
    }
    pub fn limittype(&self) -> Option<&str> {
        if self.limittype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.limittype.clone(),
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
    pub fn modifications(&self) -> Option<&str> {
        if self.modifications.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.modifications.clone(),
                ),
            )
        }
    }
    pub fn additionalnotes(&self) -> Option<&str> {
        if self.additionalnotes.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.additionalnotes.clone(),
                ),
            )
        }
    }
    pub fn p5min_scope_override(&self) -> Option<&str> {
        if self.p5min_scope_override.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.p5min_scope_override.clone(),
                ),
            )
        }
    }
    pub fn lrc(&self) -> Option<&str> {
        if self.lrc.is_empty() {
            None
        } else {
            Some(core::ops::Index::index(self.backing_data.as_slice(), self.lrc.clone()))
        }
    }
    pub fn lor(&self) -> Option<&str> {
        if self.lor.is_empty() {
            None
        } else {
            Some(core::ops::Index::index(self.backing_data.as_slice(), self.lor.clone()))
        }
    }
}
impl mmsdm_core::GetTable for GencondataNull6 {
    const VERSION: i32 = 6;
    const DATA_SET_NAME: &'static str = "GENCONDATA";
    const TABLE_NAME: &'static str = "NULL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = GencondataNull6Mapping([
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
        23,
        24,
        25,
        26,
        27,
        28,
        29,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EFFECTIVEDATE",
        "VERSIONNO",
        "GENCONID",
        "CONSTRAINTTYPE",
        "CONSTRAINTVALUE",
        "DESCRIPTION",
        "STATUS",
        "GENERICCONSTRAINTWEIGHT",
        "AUTHORISEDDATE",
        "AUTHORISEDBY",
        "DYNAMICRHS",
        "LASTCHANGED",
        "DISPATCH",
        "PREDISPATCH",
        "STPASA",
        "MTPASA",
        "IMPACT",
        "SOURCE",
        "LIMITTYPE",
        "REASON",
        "MODIFICATIONS",
        "ADDITIONALNOTES",
        "P5MIN_SCOPE_OVERRIDE",
        "LRC",
        "LOR",
        "FORCE_SCADA",
    ];
    type Row<'row> = GencondataNull6Row<'row>;
    type FieldMapping = GencondataNull6Mapping;
    type PrimaryKey = GencondataNull6PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(GencondataNull6Row {
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[0],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[1],
                    mmsdm_core::mms_decimal::parse,
                )?,
            genconid: row.get_range("genconid", field_mapping.0[2])?,
            constrainttype: row.get_opt_range("constrainttype", field_mapping.0[3])?,
            constraintvalue: row
                .get_opt_custom_parsed_at_idx(
                    "constraintvalue",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            description: row.get_opt_range("description", field_mapping.0[5])?,
            status: row.get_opt_range("status", field_mapping.0[6])?,
            genericconstraintweight: row
                .get_opt_custom_parsed_at_idx(
                    "genericconstraintweight",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[8],
                    mmsdm_core::mms_datetime::parse,
                )?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[9])?,
            dynamicrhs: row
                .get_opt_custom_parsed_at_idx(
                    "dynamicrhs",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[11],
                    mmsdm_core::mms_datetime::parse,
                )?,
            dispatch: row.get_opt_range("dispatch", field_mapping.0[12])?,
            predispatch: row.get_opt_range("predispatch", field_mapping.0[13])?,
            stpasa: row.get_opt_range("stpasa", field_mapping.0[14])?,
            mtpasa: row.get_opt_range("mtpasa", field_mapping.0[15])?,
            impact: row.get_opt_range("impact", field_mapping.0[16])?,
            source: row.get_opt_range("source", field_mapping.0[17])?,
            limittype: row.get_opt_range("limittype", field_mapping.0[18])?,
            reason: row.get_opt_range("reason", field_mapping.0[19])?,
            modifications: row.get_opt_range("modifications", field_mapping.0[20])?,
            additionalnotes: row.get_opt_range("additionalnotes", field_mapping.0[21])?,
            p5min_scope_override: row
                .get_opt_range("p5min_scope_override", field_mapping.0[22])?,
            lrc: row.get_opt_range("lrc", field_mapping.0[23])?,
            lor: row.get_opt_range("lor", field_mapping.0[24])?,
            force_scada: row
                .get_opt_custom_parsed_at_idx(
                    "force_scada",
                    field_mapping.0[25],
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
        Ok(GencondataNull6Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let effectivedate = row
            .get_custom_parsed_at_idx(
                "effectivedate",
                4,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(effectivedate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> GencondataNull6PrimaryKey {
        GencondataNull6PrimaryKey {
            effectivedate: row.effectivedate,
            genconid: row.genconid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.effectivedate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "gencondata_null_v6_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        GencondataNull6Row {
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            genconid: row.genconid.clone(),
            constrainttype: row.constrainttype.clone(),
            constraintvalue: row.constraintvalue.clone(),
            description: row.description.clone(),
            status: row.status.clone(),
            genericconstraintweight: row.genericconstraintweight.clone(),
            authoriseddate: row.authoriseddate.clone(),
            authorisedby: row.authorisedby.clone(),
            dynamicrhs: row.dynamicrhs.clone(),
            lastchanged: row.lastchanged.clone(),
            dispatch: row.dispatch.clone(),
            predispatch: row.predispatch.clone(),
            stpasa: row.stpasa.clone(),
            mtpasa: row.mtpasa.clone(),
            impact: row.impact.clone(),
            source: row.source.clone(),
            limittype: row.limittype.clone(),
            reason: row.reason.clone(),
            modifications: row.modifications.clone(),
            additionalnotes: row.additionalnotes.clone(),
            p5min_scope_override: row.p5min_scope_override.clone(),
            lrc: row.lrc.clone(),
            lor: row.lor.clone(),
            force_scada: row.force_scada.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GencondataNull6PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub genconid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for GencondataNull6PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for GencondataNull6Row<'data> {
    type Row<'other> = GencondataNull6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.genconid() == row.genconid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for GencondataNull6Row<'data> {
    type PrimaryKey = GencondataNull6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.genconid() == key.genconid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for GencondataNull6PrimaryKey {
    type Row<'other> = GencondataNull6Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.genconid == row.genconid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GencondataNull6PrimaryKey {
    type PrimaryKey = GencondataNull6PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.genconid == key.genconid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for GencondataNull6 {
    type Builder = GencondataNull6Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "genconid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "constrainttype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "constraintvalue",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "description",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "status",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "genericconstraintweight",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authoriseddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authorisedby",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "dynamicrhs",
                    arrow::datatypes::DataType::Decimal128(15, 5),
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
                    "dispatch",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "predispatch",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "stpasa",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "mtpasa",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "impact",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "source",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "limittype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reason",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "modifications",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "additionalnotes",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "p5min_scope_override",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lrc",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "lor",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "force_scada",
                    arrow::datatypes::DataType::Decimal128(1, 0),
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        GencondataNull6Builder {
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            genconid_array: arrow::array::builder::StringBuilder::new(),
            constrainttype_array: arrow::array::builder::StringBuilder::new(),
            constraintvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            description_array: arrow::array::builder::StringBuilder::new(),
            status_array: arrow::array::builder::StringBuilder::new(),
            genericconstraintweight_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            dynamicrhs_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(15, 5)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            dispatch_array: arrow::array::builder::StringBuilder::new(),
            predispatch_array: arrow::array::builder::StringBuilder::new(),
            stpasa_array: arrow::array::builder::StringBuilder::new(),
            mtpasa_array: arrow::array::builder::StringBuilder::new(),
            impact_array: arrow::array::builder::StringBuilder::new(),
            source_array: arrow::array::builder::StringBuilder::new(),
            limittype_array: arrow::array::builder::StringBuilder::new(),
            reason_array: arrow::array::builder::StringBuilder::new(),
            modifications_array: arrow::array::builder::StringBuilder::new(),
            additionalnotes_array: arrow::array::builder::StringBuilder::new(),
            p5min_scope_override_array: arrow::array::builder::StringBuilder::new(),
            lrc_array: arrow::array::builder::StringBuilder::new(),
            lor_array: arrow::array::builder::StringBuilder::new(),
            force_scada_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(1, 0)),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.effectivedate_array.append_value(row.effectivedate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.genconid_array.append_value(row.genconid());
        builder.constrainttype_array.append_option(row.constrainttype());
        builder
            .constraintvalue_array
            .append_option({
                row.constraintvalue
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder.description_array.append_option(row.description());
        builder.status_array.append_option(row.status());
        builder
            .genericconstraintweight_array
            .append_option({
                row.genericconstraintweight
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .authoriseddate_array
            .append_option(row.authoriseddate.map(|val| val.timestamp_millis()));
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .dynamicrhs_array
            .append_option({
                row.dynamicrhs
                    .map(|mut val| {
                        val.rescale(5);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder.dispatch_array.append_option(row.dispatch());
        builder.predispatch_array.append_option(row.predispatch());
        builder.stpasa_array.append_option(row.stpasa());
        builder.mtpasa_array.append_option(row.mtpasa());
        builder.impact_array.append_option(row.impact());
        builder.source_array.append_option(row.source());
        builder.limittype_array.append_option(row.limittype());
        builder.reason_array.append_option(row.reason());
        builder.modifications_array.append_option(row.modifications());
        builder.additionalnotes_array.append_option(row.additionalnotes());
        builder.p5min_scope_override_array.append_option(row.p5min_scope_override());
        builder.lrc_array.append_option(row.lrc());
        builder.lor_array.append_option(row.lor());
        builder
            .force_scada_array
            .append_option({
                row.force_scada
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genconid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constrainttype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.constraintvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.status_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genericconstraintweight_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dynamicrhs_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.dispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.predispatch_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.stpasa_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.mtpasa_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.impact_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.source_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.limittype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reason_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.modifications_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.additionalnotes_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.p5min_scope_override_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lrc_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.force_scada_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct GencondataNull6Builder {
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    genconid_array: arrow::array::builder::StringBuilder,
    constrainttype_array: arrow::array::builder::StringBuilder,
    constraintvalue_array: arrow::array::builder::Decimal128Builder,
    description_array: arrow::array::builder::StringBuilder,
    status_array: arrow::array::builder::StringBuilder,
    genericconstraintweight_array: arrow::array::builder::Decimal128Builder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    dynamicrhs_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    dispatch_array: arrow::array::builder::StringBuilder,
    predispatch_array: arrow::array::builder::StringBuilder,
    stpasa_array: arrow::array::builder::StringBuilder,
    mtpasa_array: arrow::array::builder::StringBuilder,
    impact_array: arrow::array::builder::StringBuilder,
    source_array: arrow::array::builder::StringBuilder,
    limittype_array: arrow::array::builder::StringBuilder,
    reason_array: arrow::array::builder::StringBuilder,
    modifications_array: arrow::array::builder::StringBuilder,
    additionalnotes_array: arrow::array::builder::StringBuilder,
    p5min_scope_override_array: arrow::array::builder::StringBuilder,
    lrc_array: arrow::array::builder::StringBuilder,
    lor_array: arrow::array::builder::StringBuilder,
    force_scada_array: arrow::array::builder::Decimal128Builder,
}
pub struct GenconsetNull1;
pub struct GenconsetNull1Mapping([usize; 7]);
/// # Summary
///
/// ## GENCONSET
///  _GENCONSET sets out generic constraint sets that are invoked and revoked, and may contain many generic constraints (GENCONDATA)._
///
/// * Data Set Name: Genconset
/// * File Name: Null
/// * Data Version: 1
///
/// # Description
///  GENCONSET is public data, and is available to all participants. Source GENCONSET updates as sets are updated by AEMO.
///
///
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * GENCONID
/// * GENCONSETID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct GenconsetNull1Row<'data> {
    /// Unique ID for the Constraint Set
    pub genconsetid: core::ops::Range<usize>,
    /// Date this record becomes effective
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of the record for the given effective date
    pub versionno: rust_decimal::Decimal,
    /// Generic Contraint ID
    pub genconid: core::ops::Range<usize>,
    /// Since market start in 1998 these fields have not been used and any data that has been populated in the fields should be ignored
    pub genconeffdate: Option<chrono::NaiveDateTime>,
    /// Since market start in 1998 these fields have not been used and any data that has been populated in the fields should be ignored
    pub genconversionno: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> GenconsetNull1Row<'data> {
    pub fn genconsetid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.genconsetid.clone())
    }
    pub fn genconid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.genconid.clone())
    }
}
impl mmsdm_core::GetTable for GenconsetNull1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "GENCONSET";
    const TABLE_NAME: &'static str = "NULL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = GenconsetNull1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "GENCONSETID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "GENCONID",
        "GENCONEFFDATE",
        "GENCONVERSIONNO",
        "LASTCHANGED",
    ];
    type Row<'row> = GenconsetNull1Row<'row>;
    type FieldMapping = GenconsetNull1Mapping;
    type PrimaryKey = GenconsetNull1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(GenconsetNull1Row {
            genconsetid: row.get_range("genconsetid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            genconid: row.get_range("genconid", field_mapping.0[3])?,
            genconeffdate: row
                .get_opt_custom_parsed_at_idx(
                    "genconeffdate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            genconversionno: row
                .get_opt_custom_parsed_at_idx(
                    "genconversionno",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
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
        Ok(GenconsetNull1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let effectivedate = row
            .get_custom_parsed_at_idx(
                "effectivedate",
                5,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(effectivedate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> GenconsetNull1PrimaryKey {
        GenconsetNull1PrimaryKey {
            effectivedate: row.effectivedate,
            genconid: row.genconid().to_string(),
            genconsetid: row.genconsetid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.effectivedate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "genconset_null_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        GenconsetNull1Row {
            genconsetid: row.genconsetid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            genconid: row.genconid.clone(),
            genconeffdate: row.genconeffdate.clone(),
            genconversionno: row.genconversionno.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenconsetNull1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub genconid: alloc::string::String,
    pub genconsetid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for GenconsetNull1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for GenconsetNull1Row<'data> {
    type Row<'other> = GenconsetNull1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.genconid() == row.genconid()
            && self.genconsetid() == row.genconsetid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for GenconsetNull1Row<'data> {
    type PrimaryKey = GenconsetNull1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.genconid() == key.genconid
            && self.genconsetid() == key.genconsetid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for GenconsetNull1PrimaryKey {
    type Row<'other> = GenconsetNull1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.genconid == row.genconid()
            && self.genconsetid == row.genconsetid() && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GenconsetNull1PrimaryKey {
    type PrimaryKey = GenconsetNull1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.genconid == key.genconid
            && self.genconsetid == key.genconsetid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for GenconsetNull1 {
    type Builder = GenconsetNull1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "genconsetid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "genconid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "genconeffdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "genconversionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
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
        GenconsetNull1Builder {
            genconsetid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            genconid_array: arrow::array::builder::StringBuilder::new(),
            genconeffdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            genconversionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.genconsetid_array.append_value(row.genconsetid());
        builder.effectivedate_array.append_value(row.effectivedate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.genconid_array.append_value(row.genconid());
        builder
            .genconeffdate_array
            .append_option(row.genconeffdate.map(|val| val.timestamp_millis()));
        builder
            .genconversionno_array
            .append_option({
                row.genconversionno
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
                    alloc::sync::Arc::new(builder.genconsetid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genconid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genconeffdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genconversionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct GenconsetNull1Builder {
    genconsetid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    genconid_array: arrow::array::builder::StringBuilder,
    genconeffdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    genconversionno_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct GenconsetinvokeNull2;
pub struct GenconsetinvokeNull2Mapping([usize; 14]);
/// # Summary
///
/// ## GENCONSETINVOKE
///  _GENCONSETINVOKE provides details of invoked and revoked generic constraints. GENCONSETINVOKE is the key table for determining what constraints are active in dispatch, predispatch and PASA.<br>GENCONSETINVOKE also indicates whether constraints are for interconnector limits, ancillary services, etc.<br>_
///
/// * Data Set Name: Genconsetinvoke
/// * File Name: Null
/// * Data Version: 2
///
/// # Description
///  GENCONSETINVOKE is public data. All participants have access to this data. Source GENCONSETINVOKE updates each time a generic constraint is invoked or revoke time is altered. Once past the time, these times cannot be altered. Note The Replica software does not handle the deletion of GENCONSETINVOKE records. To workaround this problem, the field STARTAUTHORISEDBY indicates whether a constraint set invocation is applicable. A non-null value for the STARTAUTHORISEDBY field indicates that the constraint invocation is active. Essentially inactive invocations have a null value for the STARTAUTHORISEDBY field. To remove inactive invocations from queries on the GENCONSETINVOKE table, add the following text to the where clause "and STARTAUTHORISEDBY is not null".
///
///
///
/// # Primary Key Columns
///
/// * INVOCATION_ID
/// * INTERVENTION
#[derive(Debug, PartialEq, Eq)]
pub struct GenconsetinvokeNull2Row<'data> {
    /// Abstract unique identifier for the record. Allows Invocations to be modified without affecting PK values
    pub invocation_id: i64,
    /// Market date of start
    pub startdate: chrono::NaiveDateTime,
    /// The first dispatch interval of the invocation being the dispatch interval number starting from1 at 04:05.
    pub startperiod: rust_decimal::Decimal,
    /// Unique generic constraint set identifier
    pub genconsetid: core::ops::Range<usize>,
    /// Market date end
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Dispatch interval number end
    pub endperiod: Option<rust_decimal::Decimal>,
    /// User authorising invoke, indicating a constraint set invocation is applicable (i.e. non-null). A null value indicates inactive invocation.
    pub startauthorisedby: core::ops::Range<usize>,
    /// user authorising revoke.
    pub endauthorisedby: core::ops::Range<usize>,
    /// 0 is not intervention, 1 is intervention and causes dispatch to solve twice.
    pub intervention: core::ops::Range<usize>,
    /// Constraint type (e.g. ancillary services). This also flags where a constraint is an interconnector or intra-region network limit.
    pub asconstrainttype: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The settlement date and time corresponding to the first interval to which the constraint set is to be applied.
    pub startintervaldatetime: Option<chrono::NaiveDateTime>,
    /// The settlement date and time corresponding to the last interval to which the constraint set is to be applied.
    pub endintervaldatetime: Option<chrono::NaiveDateTime>,
    /// Flag to indicate if the constraint set is a system normal (1) or an outage set (0)
    pub systemnormal: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> GenconsetinvokeNull2Row<'data> {
    pub fn genconsetid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.genconsetid.clone())
    }
    pub fn startauthorisedby(&self) -> Option<&str> {
        if self.startauthorisedby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.startauthorisedby.clone(),
                ),
            )
        }
    }
    pub fn endauthorisedby(&self) -> Option<&str> {
        if self.endauthorisedby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.endauthorisedby.clone(),
                ),
            )
        }
    }
    pub fn intervention(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.intervention.clone())
    }
    pub fn asconstrainttype(&self) -> Option<&str> {
        if self.asconstrainttype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.asconstrainttype.clone(),
                ),
            )
        }
    }
    pub fn systemnormal(&self) -> Option<&str> {
        if self.systemnormal.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.systemnormal.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for GenconsetinvokeNull2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "GENCONSETINVOKE";
    const TABLE_NAME: &'static str = "NULL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = GenconsetinvokeNull2Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "INVOCATION_ID",
        "STARTDATE",
        "STARTPERIOD",
        "GENCONSETID",
        "ENDDATE",
        "ENDPERIOD",
        "STARTAUTHORISEDBY",
        "ENDAUTHORISEDBY",
        "INTERVENTION",
        "ASCONSTRAINTTYPE",
        "LASTCHANGED",
        "STARTINTERVALDATETIME",
        "ENDINTERVALDATETIME",
        "SYSTEMNORMAL",
    ];
    type Row<'row> = GenconsetinvokeNull2Row<'row>;
    type FieldMapping = GenconsetinvokeNull2Mapping;
    type PrimaryKey = GenconsetinvokeNull2PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(GenconsetinvokeNull2Row {
            invocation_id: row.get_parsed_at_idx("invocation_id", field_mapping.0[0])?,
            startdate: row
                .get_custom_parsed_at_idx(
                    "startdate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            startperiod: row
                .get_custom_parsed_at_idx(
                    "startperiod",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            genconsetid: row.get_range("genconsetid", field_mapping.0[3])?,
            enddate: row
                .get_opt_custom_parsed_at_idx(
                    "enddate",
                    field_mapping.0[4],
                    mmsdm_core::mms_datetime::parse,
                )?,
            endperiod: row
                .get_opt_custom_parsed_at_idx(
                    "endperiod",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            startauthorisedby: row
                .get_opt_range("startauthorisedby", field_mapping.0[6])?,
            endauthorisedby: row.get_opt_range("endauthorisedby", field_mapping.0[7])?,
            intervention: row.get_range("intervention", field_mapping.0[8])?,
            asconstrainttype: row.get_opt_range("asconstrainttype", field_mapping.0[9])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[10],
                    mmsdm_core::mms_datetime::parse,
                )?,
            startintervaldatetime: row
                .get_opt_custom_parsed_at_idx(
                    "startintervaldatetime",
                    field_mapping.0[11],
                    mmsdm_core::mms_datetime::parse,
                )?,
            endintervaldatetime: row
                .get_opt_custom_parsed_at_idx(
                    "endintervaldatetime",
                    field_mapping.0[12],
                    mmsdm_core::mms_datetime::parse,
                )?,
            systemnormal: row.get_opt_range("systemnormal", field_mapping.0[13])?,
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
        Ok(GenconsetinvokeNull2Mapping(base_mapping))
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
    fn primary_key(row: &Self::Row<'_>) -> GenconsetinvokeNull2PrimaryKey {
        GenconsetinvokeNull2PrimaryKey {
            invocation_id: row.invocation_id,
            intervention: row.intervention().to_string(),
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "genconsetinvoke_null_v2".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        GenconsetinvokeNull2Row {
            invocation_id: row.invocation_id.clone(),
            startdate: row.startdate.clone(),
            startperiod: row.startperiod.clone(),
            genconsetid: row.genconsetid.clone(),
            enddate: row.enddate.clone(),
            endperiod: row.endperiod.clone(),
            startauthorisedby: row.startauthorisedby.clone(),
            endauthorisedby: row.endauthorisedby.clone(),
            intervention: row.intervention.clone(),
            asconstrainttype: row.asconstrainttype.clone(),
            lastchanged: row.lastchanged.clone(),
            startintervaldatetime: row.startintervaldatetime.clone(),
            endintervaldatetime: row.endintervaldatetime.clone(),
            systemnormal: row.systemnormal.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenconsetinvokeNull2PrimaryKey {
    pub invocation_id: i64,
    pub intervention: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for GenconsetinvokeNull2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for GenconsetinvokeNull2Row<'data> {
    type Row<'other> = GenconsetinvokeNull2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.invocation_id == row.invocation_id
            && self.intervention() == row.intervention()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for GenconsetinvokeNull2Row<'data> {
    type PrimaryKey = GenconsetinvokeNull2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.invocation_id == key.invocation_id
            && self.intervention() == key.intervention
    }
}
impl<'data> mmsdm_core::CompareWithRow for GenconsetinvokeNull2PrimaryKey {
    type Row<'other> = GenconsetinvokeNull2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.invocation_id == row.invocation_id
            && self.intervention == row.intervention()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GenconsetinvokeNull2PrimaryKey {
    type PrimaryKey = GenconsetinvokeNull2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.invocation_id == key.invocation_id && self.intervention == key.intervention
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for GenconsetinvokeNull2 {
    type Builder = GenconsetinvokeNull2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "invocation_id",
                    arrow::datatypes::DataType::Int64,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "startdate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "startperiod",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "genconsetid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "enddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "endperiod",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "startauthorisedby",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "endauthorisedby",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "intervention",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "asconstrainttype",
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
                    "startintervaldatetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "endintervaldatetime",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "systemnormal",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        GenconsetinvokeNull2Builder {
            invocation_id_array: arrow::array::builder::Int64Builder::new(),
            startdate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            startperiod_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            genconsetid_array: arrow::array::builder::StringBuilder::new(),
            enddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            endperiod_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            startauthorisedby_array: arrow::array::builder::StringBuilder::new(),
            endauthorisedby_array: arrow::array::builder::StringBuilder::new(),
            intervention_array: arrow::array::builder::StringBuilder::new(),
            asconstrainttype_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            startintervaldatetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            endintervaldatetime_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            systemnormal_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.invocation_id_array.append_value(row.invocation_id);
        builder.startdate_array.append_value(row.startdate.timestamp_millis());
        builder
            .startperiod_array
            .append_value({
                let mut val = row.startperiod;
                val.rescale(0);
                val.mantissa()
            });
        builder.genconsetid_array.append_value(row.genconsetid());
        builder
            .enddate_array
            .append_option(row.enddate.map(|val| val.timestamp_millis()));
        builder
            .endperiod_array
            .append_option({
                row.endperiod
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.startauthorisedby_array.append_option(row.startauthorisedby());
        builder.endauthorisedby_array.append_option(row.endauthorisedby());
        builder.intervention_array.append_value(row.intervention());
        builder.asconstrainttype_array.append_option(row.asconstrainttype());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder
            .startintervaldatetime_array
            .append_option(row.startintervaldatetime.map(|val| val.timestamp_millis()));
        builder
            .endintervaldatetime_array
            .append_option(row.endintervaldatetime.map(|val| val.timestamp_millis()));
        builder.systemnormal_array.append_option(row.systemnormal());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.invocation_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startdate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startperiod_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genconsetid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.enddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.endperiod_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startauthorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.endauthorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.intervention_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.asconstrainttype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.startintervaldatetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.endintervaldatetime_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.systemnormal_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct GenconsetinvokeNull2Builder {
    invocation_id_array: arrow::array::builder::Int64Builder,
    startdate_array: arrow::array::builder::TimestampMillisecondBuilder,
    startperiod_array: arrow::array::builder::Decimal128Builder,
    genconsetid_array: arrow::array::builder::StringBuilder,
    enddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    endperiod_array: arrow::array::builder::Decimal128Builder,
    startauthorisedby_array: arrow::array::builder::StringBuilder,
    endauthorisedby_array: arrow::array::builder::StringBuilder,
    intervention_array: arrow::array::builder::StringBuilder,
    asconstrainttype_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    startintervaldatetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    endintervaldatetime_array: arrow::array::builder::TimestampMillisecondBuilder,
    systemnormal_array: arrow::array::builder::StringBuilder,
}
pub struct GenconsettrkNull2;
pub struct GenconsettrkNull2Mapping([usize; 11]);
/// # Summary
///
/// ## GENCONSETTRK
///  _GENCONSETTRK assists in determining the correct version of a generic constraint set that has been invoked in GENCONSETINVOKE._
///
/// * Data Set Name: Genconsettrk
/// * File Name: Null
/// * Data Version: 2
///
/// # Description
///  GENCONSETTRK data is public to all participants. Source Ad hoc updates occur to GENCONSETTRK.
///
///
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * GENCONSETID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct GenconsettrkNull2Row<'data> {
    /// Unique ID for the Constraint Set
    pub genconsetid: core::ops::Range<usize>,
    /// Date this record becomes effective
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of the record for the given effective date
    pub versionno: rust_decimal::Decimal,
    /// Description of the constraint
    pub description: core::ops::Range<usize>,
    /// The person who authorised the constraint set
    pub authorisedby: core::ops::Range<usize>,
    /// The date and time of authorising the constraint set
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The region the constraint set is located in or a special grouping (e.g. CHIMERA)
    pub coverage: core::ops::Range<usize>,
    /// Details of the changes made to this version of the constraint set
    pub modifications: core::ops::Range<usize>,
    /// Not used as of 2005 End of Year Release [was Flag to indicate if the constraint set is a system normal (1) or and an outage set (0)]
    pub systemnormal: core::ops::Range<usize>,
    /// Detail of the plant that is not in service
    pub outage: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> GenconsettrkNull2Row<'data> {
    pub fn genconsetid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.genconsetid.clone())
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
    pub fn authorisedby(&self) -> Option<&str> {
        if self.authorisedby.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.authorisedby.clone(),
                ),
            )
        }
    }
    pub fn coverage(&self) -> Option<&str> {
        if self.coverage.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.coverage.clone(),
                ),
            )
        }
    }
    pub fn modifications(&self) -> Option<&str> {
        if self.modifications.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.modifications.clone(),
                ),
            )
        }
    }
    pub fn systemnormal(&self) -> Option<&str> {
        if self.systemnormal.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.systemnormal.clone(),
                ),
            )
        }
    }
    pub fn outage(&self) -> Option<&str> {
        if self.outage.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.outage.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for GenconsettrkNull2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "GENCONSETTRK";
    const TABLE_NAME: &'static str = "NULL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = GenconsettrkNull2Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "GENCONSETID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "DESCRIPTION",
        "AUTHORISEDBY",
        "AUTHORISEDDATE",
        "LASTCHANGED",
        "COVERAGE",
        "MODIFICATIONS",
        "SYSTEMNORMAL",
        "OUTAGE",
    ];
    type Row<'row> = GenconsettrkNull2Row<'row>;
    type FieldMapping = GenconsettrkNull2Mapping;
    type PrimaryKey = GenconsettrkNull2PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(GenconsettrkNull2Row {
            genconsetid: row.get_range("genconsetid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            description: row.get_opt_range("description", field_mapping.0[3])?,
            authorisedby: row.get_opt_range("authorisedby", field_mapping.0[4])?,
            authoriseddate: row
                .get_opt_custom_parsed_at_idx(
                    "authoriseddate",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[6],
                    mmsdm_core::mms_datetime::parse,
                )?,
            coverage: row.get_opt_range("coverage", field_mapping.0[7])?,
            modifications: row.get_opt_range("modifications", field_mapping.0[8])?,
            systemnormal: row.get_opt_range("systemnormal", field_mapping.0[9])?,
            outage: row.get_opt_range("outage", field_mapping.0[10])?,
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
        Ok(GenconsettrkNull2Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let effectivedate = row
            .get_custom_parsed_at_idx(
                "effectivedate",
                5,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(effectivedate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> GenconsettrkNull2PrimaryKey {
        GenconsettrkNull2PrimaryKey {
            effectivedate: row.effectivedate,
            genconsetid: row.genconsetid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.effectivedate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "genconsettrk_null_v2_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        GenconsettrkNull2Row {
            genconsetid: row.genconsetid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            description: row.description.clone(),
            authorisedby: row.authorisedby.clone(),
            authoriseddate: row.authoriseddate.clone(),
            lastchanged: row.lastchanged.clone(),
            coverage: row.coverage.clone(),
            modifications: row.modifications.clone(),
            systemnormal: row.systemnormal.clone(),
            outage: row.outage.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenconsettrkNull2PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub genconsetid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for GenconsettrkNull2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for GenconsettrkNull2Row<'data> {
    type Row<'other> = GenconsettrkNull2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate
            && self.genconsetid() == row.genconsetid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for GenconsettrkNull2Row<'data> {
    type PrimaryKey = GenconsettrkNull2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.genconsetid() == key.genconsetid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for GenconsettrkNull2PrimaryKey {
    type Row<'other> = GenconsettrkNull2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.genconsetid == row.genconsetid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GenconsettrkNull2PrimaryKey {
    type PrimaryKey = GenconsettrkNull2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.genconsetid == key.genconsetid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for GenconsettrkNull2 {
    type Builder = GenconsettrkNull2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "genconsetid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "description",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authorisedby",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "authoriseddate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
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
                    "coverage",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "modifications",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "systemnormal",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "outage",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        GenconsettrkNull2Builder {
            genconsetid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            description_array: arrow::array::builder::StringBuilder::new(),
            authorisedby_array: arrow::array::builder::StringBuilder::new(),
            authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            coverage_array: arrow::array::builder::StringBuilder::new(),
            modifications_array: arrow::array::builder::StringBuilder::new(),
            systemnormal_array: arrow::array::builder::StringBuilder::new(),
            outage_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.genconsetid_array.append_value(row.genconsetid());
        builder.effectivedate_array.append_value(row.effectivedate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.description_array.append_option(row.description());
        builder.authorisedby_array.append_option(row.authorisedby());
        builder
            .authoriseddate_array
            .append_option(row.authoriseddate.map(|val| val.timestamp_millis()));
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder.coverage_array.append_option(row.coverage());
        builder.modifications_array.append_option(row.modifications());
        builder.systemnormal_array.append_option(row.systemnormal());
        builder.outage_array.append_option(row.outage());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.genconsetid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authorisedby_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.authoriseddate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.coverage_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.modifications_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.systemnormal_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.outage_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct GenconsettrkNull2Builder {
    genconsetid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    description_array: arrow::array::builder::StringBuilder,
    authorisedby_array: arrow::array::builder::StringBuilder,
    authoriseddate_array: arrow::array::builder::TimestampMillisecondBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    coverage_array: arrow::array::builder::StringBuilder,
    modifications_array: arrow::array::builder::StringBuilder,
    systemnormal_array: arrow::array::builder::StringBuilder,
    outage_array: arrow::array::builder::StringBuilder,
}
pub struct GcrhsNull1;
pub struct GcrhsNull1Mapping([usize; 15]);
/// # Summary
///
/// ## GENERICCONSTRAINTRHS
///  _GENERICCONSTRAINTRHS sets out details of generic constraint Right Hand Side (RHS) formulations for dispatch (DS), predispatch (PD) and Short Term PASA (ST). GENERICCONSTRAINTRHS also includes general expressions (EQ) used in the dispatch, predispatch and PASA time frames.<br>GENERICCONSTRAINTRHS replaces data previously available via the "Constraint Library” Excel spreadsheet.<br>_
///
/// * Data Set Name: Gcrhs
/// * File Name: Null
/// * Data Version: 1
///
/// # Description
///  GENERICCONSTRAINTRHS is public data, and is available to all participants. Source GENERICCONSTRAINTRHS updates whenever a new generic constraint RHS or expression is created or modified Volume Approximately 70,000 records per year Note GENERICEQUATIONRHS and GENERICEQUATIONDESC allow commonly used constraint right hand side formulations to be defined as a generic equation. Once defined, the generic equation can be referenced from any Generic constraint RHS formulation defined in GENERICCONSTRAINTRHS.
///
///
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * GENCONID
/// * SCOPE
/// * TERMID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct GcrhsNull1Row<'data> {
    /// Generic Constraint Identifier
    pub genconid: core::ops::Range<usize>,
    /// Effective date of this record
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// Scope of RHS term (DS, PD, ST or EQ)
    pub scope: core::ops::Range<usize>,
    /// The unique identifier for the a constraint RHS term
    pub termid: rust_decimal::Decimal,
    /// ID of super-term, if this is a sub-term
    pub groupid: Option<rust_decimal::Decimal>,
    /// ID defining data source
    pub spd_id: core::ops::Range<usize>,
    /// ID describing type of data source
    pub spd_type: core::ops::Range<usize>,
    /// Multiplier applied to operator result
    pub factor: Option<rust_decimal::Decimal>,
    /// Unitary operator to apply to data value
    pub operation: core::ops::Range<usize>,
    /// Default value if primary source given by SPD_ID and SPD_TYPE not available.
    pub defaultvalue: Option<rust_decimal::Decimal>,
    /// The unique identifier for the first term (logic expression) to use in a Branch term
    pub parameterterm1: core::ops::Range<usize>,
    /// The unique identifier for the second term (logic&lt;=0 result) to use in a Branch term
    pub parameterterm2: core::ops::Range<usize>,
    /// The unique identifier for the third term (logic&gt;0 result) to use in a Branch term
    pub parameterterm3: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> GcrhsNull1Row<'data> {
    pub fn genconid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.genconid.clone())
    }
    pub fn scope(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.scope.clone())
    }
    pub fn spd_id(&self) -> Option<&str> {
        if self.spd_id.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.spd_id.clone(),
                ),
            )
        }
    }
    pub fn spd_type(&self) -> Option<&str> {
        if self.spd_type.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.spd_type.clone(),
                ),
            )
        }
    }
    pub fn operation(&self) -> Option<&str> {
        if self.operation.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.operation.clone(),
                ),
            )
        }
    }
    pub fn parameterterm1(&self) -> Option<&str> {
        if self.parameterterm1.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.parameterterm1.clone(),
                ),
            )
        }
    }
    pub fn parameterterm2(&self) -> Option<&str> {
        if self.parameterterm2.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.parameterterm2.clone(),
                ),
            )
        }
    }
    pub fn parameterterm3(&self) -> Option<&str> {
        if self.parameterterm3.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.parameterterm3.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for GcrhsNull1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "GCRHS";
    const TABLE_NAME: &'static str = "NULL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = GcrhsNull1Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "GENCONID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "SCOPE",
        "TERMID",
        "GROUPID",
        "SPD_ID",
        "SPD_TYPE",
        "FACTOR",
        "OPERATION",
        "DEFAULTVALUE",
        "PARAMETERTERM1",
        "PARAMETERTERM2",
        "PARAMETERTERM3",
        "LASTCHANGED",
    ];
    type Row<'row> = GcrhsNull1Row<'row>;
    type FieldMapping = GcrhsNull1Mapping;
    type PrimaryKey = GcrhsNull1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(GcrhsNull1Row {
            genconid: row.get_range("genconid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            scope: row.get_range("scope", field_mapping.0[3])?,
            termid: row
                .get_custom_parsed_at_idx(
                    "termid",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            groupid: row
                .get_opt_custom_parsed_at_idx(
                    "groupid",
                    field_mapping.0[5],
                    mmsdm_core::mms_decimal::parse,
                )?,
            spd_id: row.get_opt_range("spd_id", field_mapping.0[6])?,
            spd_type: row.get_opt_range("spd_type", field_mapping.0[7])?,
            factor: row
                .get_opt_custom_parsed_at_idx(
                    "factor",
                    field_mapping.0[8],
                    mmsdm_core::mms_decimal::parse,
                )?,
            operation: row.get_opt_range("operation", field_mapping.0[9])?,
            defaultvalue: row
                .get_opt_custom_parsed_at_idx(
                    "defaultvalue",
                    field_mapping.0[10],
                    mmsdm_core::mms_decimal::parse,
                )?,
            parameterterm1: row.get_opt_range("parameterterm1", field_mapping.0[11])?,
            parameterterm2: row.get_opt_range("parameterterm2", field_mapping.0[12])?,
            parameterterm3: row.get_opt_range("parameterterm3", field_mapping.0[13])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[14],
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
        Ok(GcrhsNull1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let effectivedate = row
            .get_custom_parsed_at_idx(
                "effectivedate",
                5,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(effectivedate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> GcrhsNull1PrimaryKey {
        GcrhsNull1PrimaryKey {
            effectivedate: row.effectivedate,
            genconid: row.genconid().to_string(),
            scope: row.scope().to_string(),
            termid: row.termid,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.effectivedate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "gcrhs_null_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        GcrhsNull1Row {
            genconid: row.genconid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            scope: row.scope.clone(),
            termid: row.termid.clone(),
            groupid: row.groupid.clone(),
            spd_id: row.spd_id.clone(),
            spd_type: row.spd_type.clone(),
            factor: row.factor.clone(),
            operation: row.operation.clone(),
            defaultvalue: row.defaultvalue.clone(),
            parameterterm1: row.parameterterm1.clone(),
            parameterterm2: row.parameterterm2.clone(),
            parameterterm3: row.parameterterm3.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GcrhsNull1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub genconid: alloc::string::String,
    pub scope: alloc::string::String,
    pub termid: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for GcrhsNull1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for GcrhsNull1Row<'data> {
    type Row<'other> = GcrhsNull1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.genconid() == row.genconid()
            && self.scope() == row.scope() && self.termid == row.termid
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for GcrhsNull1Row<'data> {
    type PrimaryKey = GcrhsNull1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.genconid() == key.genconid
            && self.scope() == key.scope && self.termid == key.termid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for GcrhsNull1PrimaryKey {
    type Row<'other> = GcrhsNull1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.genconid == row.genconid()
            && self.scope == row.scope() && self.termid == row.termid
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GcrhsNull1PrimaryKey {
    type PrimaryKey = GcrhsNull1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.genconid == key.genconid
            && self.scope == key.scope && self.termid == key.termid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for GcrhsNull1 {
    type Builder = GcrhsNull1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "genconid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(22, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "scope",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "termid",
                    arrow::datatypes::DataType::Decimal128(4, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "groupid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "spd_id",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "spd_type",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "factor",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "operation",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "defaultvalue",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "parameterterm1",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "parameterterm2",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "parameterterm3",
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
        GcrhsNull1Builder {
            genconid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(22, 0)),
            scope_array: arrow::array::builder::StringBuilder::new(),
            termid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(4, 0)),
            groupid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            spd_id_array: arrow::array::builder::StringBuilder::new(),
            spd_type_array: arrow::array::builder::StringBuilder::new(),
            factor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            operation_array: arrow::array::builder::StringBuilder::new(),
            defaultvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            parameterterm1_array: arrow::array::builder::StringBuilder::new(),
            parameterterm2_array: arrow::array::builder::StringBuilder::new(),
            parameterterm3_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.genconid_array.append_value(row.genconid());
        builder.effectivedate_array.append_value(row.effectivedate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.scope_array.append_value(row.scope());
        builder
            .termid_array
            .append_value({
                let mut val = row.termid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .groupid_array
            .append_option({
                row.groupid
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.spd_id_array.append_option(row.spd_id());
        builder.spd_type_array.append_option(row.spd_type());
        builder
            .factor_array
            .append_option({
                row.factor
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder.operation_array.append_option(row.operation());
        builder
            .defaultvalue_array
            .append_option({
                row.defaultvalue
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder.parameterterm1_array.append_option(row.parameterterm1());
        builder.parameterterm2_array.append_option(row.parameterterm2());
        builder.parameterterm3_array.append_option(row.parameterterm3());
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
                    alloc::sync::Arc::new(builder.genconid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.scope_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.termid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.groupid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.spd_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.spd_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.factor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.operation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.defaultvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.parameterterm1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.parameterterm2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.parameterterm3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct GcrhsNull1Builder {
    genconid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    scope_array: arrow::array::builder::StringBuilder,
    termid_array: arrow::array::builder::Decimal128Builder,
    groupid_array: arrow::array::builder::Decimal128Builder,
    spd_id_array: arrow::array::builder::StringBuilder,
    spd_type_array: arrow::array::builder::StringBuilder,
    factor_array: arrow::array::builder::Decimal128Builder,
    operation_array: arrow::array::builder::StringBuilder,
    defaultvalue_array: arrow::array::builder::Decimal128Builder,
    parameterterm1_array: arrow::array::builder::StringBuilder,
    parameterterm2_array: arrow::array::builder::StringBuilder,
    parameterterm3_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct GeqdescNull2;
pub struct GeqdescNull2Mapping([usize; 9]);
/// # Summary
///
/// ## GENERICEQUATIONDESC
///  _GENERICEQUATIONDESC defines a generic equation identifier with a description. The formulation of the generic equation is detailed in GENERICEQUATIONRHS._
///
/// * Data Set Name: Geqdesc
/// * File Name: Null
/// * Data Version: 2
///
/// # Description
///  GENERICEQUATIONDESC data is public to all participants. Source GENERICEQUATIONDESC updates when new a generic equation is created for the first time. Volume Approximately 100 records per year Note GENERICEQUATIONRHS and GENERICEQUATIONDESC allow commonly used constraint right hand side formulations to be defined as a generic equation. Once defined, the generic equation can be referenced from any Generic constraint RHS formulation defined in GENERICCONSTRAINTRHS.
///
///
///
/// # Primary Key Columns
///
/// * EQUATIONID
#[derive(Debug, PartialEq, Eq)]
pub struct GeqdescNull2Row<'data> {
    /// Generic Equation Identifier
    pub equationid: core::ops::Range<usize>,
    /// Generic Equation Description
    pub description: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The device(s) affected by the constraint (e.g. Interconnector, Generator(s) or Cutset)
    pub impact: core::ops::Range<usize>,
    /// The source of the constraint formulation
    pub source: core::ops::Range<usize>,
    /// The limit type of the constraint e.g. Transient Stability, Voltage Stability
    pub limittype: core::ops::Range<usize>,
    /// The contingency or reason for the constraint
    pub reason: core::ops::Range<usize>,
    /// Details of the changes made to this version of the generic equation RHS
    pub modifications: core::ops::Range<usize>,
    /// Extra notes on the constraint
    pub additionalnotes: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> GeqdescNull2Row<'data> {
    pub fn equationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.equationid.clone())
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
    pub fn impact(&self) -> Option<&str> {
        if self.impact.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.impact.clone(),
                ),
            )
        }
    }
    pub fn source(&self) -> Option<&str> {
        if self.source.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.source.clone(),
                ),
            )
        }
    }
    pub fn limittype(&self) -> Option<&str> {
        if self.limittype.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.limittype.clone(),
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
    pub fn modifications(&self) -> Option<&str> {
        if self.modifications.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.modifications.clone(),
                ),
            )
        }
    }
    pub fn additionalnotes(&self) -> Option<&str> {
        if self.additionalnotes.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.additionalnotes.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for GeqdescNull2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "GEQDESC";
    const TABLE_NAME: &'static str = "NULL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = GeqdescNull2Mapping([
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
        "EQUATIONID",
        "DESCRIPTION",
        "LASTCHANGED",
        "IMPACT",
        "SOURCE",
        "LIMITTYPE",
        "REASON",
        "MODIFICATIONS",
        "ADDITIONALNOTES",
    ];
    type Row<'row> = GeqdescNull2Row<'row>;
    type FieldMapping = GeqdescNull2Mapping;
    type PrimaryKey = GeqdescNull2PrimaryKey;
    type Partition = ();
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(GeqdescNull2Row {
            equationid: row.get_range("equationid", field_mapping.0[0])?,
            description: row.get_opt_range("description", field_mapping.0[1])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[2],
                    mmsdm_core::mms_datetime::parse,
                )?,
            impact: row.get_opt_range("impact", field_mapping.0[3])?,
            source: row.get_opt_range("source", field_mapping.0[4])?,
            limittype: row.get_opt_range("limittype", field_mapping.0[5])?,
            reason: row.get_opt_range("reason", field_mapping.0[6])?,
            modifications: row.get_opt_range("modifications", field_mapping.0[7])?,
            additionalnotes: row.get_opt_range("additionalnotes", field_mapping.0[8])?,
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
        Ok(GeqdescNull2Mapping(base_mapping))
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
    fn primary_key(row: &Self::Row<'_>) -> GeqdescNull2PrimaryKey {
        GeqdescNull2PrimaryKey {
            equationid: row.equationid().to_string(),
        }
    }
    fn partition_suffix(_row: &Self::Row<'_>) -> Self::Partition {}
    fn partition_name(_row: &Self::Row<'_>) -> alloc::string::String {
        "geqdesc_null_v2".to_string()
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        GeqdescNull2Row {
            equationid: row.equationid.clone(),
            description: row.description.clone(),
            lastchanged: row.lastchanged.clone(),
            impact: row.impact.clone(),
            source: row.source.clone(),
            limittype: row.limittype.clone(),
            reason: row.reason.clone(),
            modifications: row.modifications.clone(),
            additionalnotes: row.additionalnotes.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeqdescNull2PrimaryKey {
    pub equationid: alloc::string::String,
}
impl mmsdm_core::PrimaryKey for GeqdescNull2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for GeqdescNull2Row<'data> {
    type Row<'other> = GeqdescNull2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.equationid() == row.equationid()
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for GeqdescNull2Row<'data> {
    type PrimaryKey = GeqdescNull2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.equationid() == key.equationid
    }
}
impl<'data> mmsdm_core::CompareWithRow for GeqdescNull2PrimaryKey {
    type Row<'other> = GeqdescNull2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.equationid == row.equationid()
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GeqdescNull2PrimaryKey {
    type PrimaryKey = GeqdescNull2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.equationid == key.equationid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for GeqdescNull2 {
    type Builder = GeqdescNull2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "equationid",
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
                arrow::datatypes::Field::new(
                    "impact",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "source",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "limittype",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "reason",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "modifications",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "additionalnotes",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        GeqdescNull2Builder {
            equationid_array: arrow::array::builder::StringBuilder::new(),
            description_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            impact_array: arrow::array::builder::StringBuilder::new(),
            source_array: arrow::array::builder::StringBuilder::new(),
            limittype_array: arrow::array::builder::StringBuilder::new(),
            reason_array: arrow::array::builder::StringBuilder::new(),
            modifications_array: arrow::array::builder::StringBuilder::new(),
            additionalnotes_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.equationid_array.append_value(row.equationid());
        builder.description_array.append_option(row.description());
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder.impact_array.append_option(row.impact());
        builder.source_array.append_option(row.source());
        builder.limittype_array.append_option(row.limittype());
        builder.reason_array.append_option(row.reason());
        builder.modifications_array.append_option(row.modifications());
        builder.additionalnotes_array.append_option(row.additionalnotes());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.equationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.description_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.impact_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.source_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.limittype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.reason_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.modifications_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.additionalnotes_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct GeqdescNull2Builder {
    equationid_array: arrow::array::builder::StringBuilder,
    description_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    impact_array: arrow::array::builder::StringBuilder,
    source_array: arrow::array::builder::StringBuilder,
    limittype_array: arrow::array::builder::StringBuilder,
    reason_array: arrow::array::builder::StringBuilder,
    modifications_array: arrow::array::builder::StringBuilder,
    additionalnotes_array: arrow::array::builder::StringBuilder,
}
pub struct GeqrhsNull1;
pub struct GeqrhsNull1Mapping([usize; 14]);
/// # Summary
///
/// ## GENERICEQUATIONRHS
///  _GENERICEQUATIONRHS stores the formulation of commonly used Generic Constraint Right Hand Side Equations referenced from Generic Constraint Right Hand Side definitions stored in GENERICCONSTRAINTRHS. The Generic Equation definitions are versioned and the latest effective version is applied to the dispatch process._
///
/// * Data Set Name: Geqrhs
/// * File Name: Null
/// * Data Version: 1
///
/// # Description
///  GENERICEQUATIONRHS data is public to all participants. Source GENERICEQUATIONRHS updates whenever a generic equation is created or modified. Volume Approximately 1,000 records per year Note GENERICEQUATIONRHS and GENERICEQUATIONDESC allow commonly used constraint right hand side formulations to be defined as a generic equation. Once defined, the generic equation can be referenced from any Generic constraint RHS formulation defined in GENERICCONSTRAINTRHS. To reference a generic equation from a generic constraint RHS definition, specify a SPD_TYPE of ‘X’ and the SPD_ID equivalent to the EQUATIONID field in GENERICEQUATIONRHS.
///
///
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * EQUATIONID
/// * TERMID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct GeqrhsNull1Row<'data> {
    /// Generic Equation Identifier
    pub equationid: core::ops::Range<usize>,
    /// Effective date of this record
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// The unique identifier for the a equation RHS term
    pub termid: rust_decimal::Decimal,
    /// ID of super-term, if this is a sub-term
    pub groupid: Option<rust_decimal::Decimal>,
    /// ID defining data source
    pub spd_id: core::ops::Range<usize>,
    /// ID describing type of data source
    pub spd_type: core::ops::Range<usize>,
    /// Multiplier applied to operator result
    pub factor: Option<rust_decimal::Decimal>,
    /// Unitary operator to apply to data value
    pub operation: core::ops::Range<usize>,
    /// Default value if primary source given by SPD_ID and SPD_TYPE not available.
    pub defaultvalue: Option<rust_decimal::Decimal>,
    /// The unique identifier for the first term (logic expression) to use in a Branch term
    pub parameterterm1: core::ops::Range<usize>,
    /// The unique identifier for the second term (logic&lt;=0 result) to use in a Branch term
    pub parameterterm2: core::ops::Range<usize>,
    /// The unique identifier for the third term (logic&gt;0 result) to use in a Branch term
    pub parameterterm3: core::ops::Range<usize>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> GeqrhsNull1Row<'data> {
    pub fn equationid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.equationid.clone())
    }
    pub fn spd_id(&self) -> Option<&str> {
        if self.spd_id.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.spd_id.clone(),
                ),
            )
        }
    }
    pub fn spd_type(&self) -> Option<&str> {
        if self.spd_type.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.spd_type.clone(),
                ),
            )
        }
    }
    pub fn operation(&self) -> Option<&str> {
        if self.operation.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.operation.clone(),
                ),
            )
        }
    }
    pub fn parameterterm1(&self) -> Option<&str> {
        if self.parameterterm1.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.parameterterm1.clone(),
                ),
            )
        }
    }
    pub fn parameterterm2(&self) -> Option<&str> {
        if self.parameterterm2.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.parameterterm2.clone(),
                ),
            )
        }
    }
    pub fn parameterterm3(&self) -> Option<&str> {
        if self.parameterterm3.is_empty() {
            None
        } else {
            Some(
                core::ops::Index::index(
                    self.backing_data.as_slice(),
                    self.parameterterm3.clone(),
                ),
            )
        }
    }
}
impl mmsdm_core::GetTable for GeqrhsNull1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "GEQRHS";
    const TABLE_NAME: &'static str = "NULL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = GeqrhsNull1Mapping([
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
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "EQUATIONID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "TERMID",
        "GROUPID",
        "SPD_ID",
        "SPD_TYPE",
        "FACTOR",
        "OPERATION",
        "DEFAULTVALUE",
        "PARAMETERTERM1",
        "PARAMETERTERM2",
        "PARAMETERTERM3",
        "LASTCHANGED",
    ];
    type Row<'row> = GeqrhsNull1Row<'row>;
    type FieldMapping = GeqrhsNull1Mapping;
    type PrimaryKey = GeqrhsNull1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(GeqrhsNull1Row {
            equationid: row.get_range("equationid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            termid: row
                .get_custom_parsed_at_idx(
                    "termid",
                    field_mapping.0[3],
                    mmsdm_core::mms_decimal::parse,
                )?,
            groupid: row
                .get_opt_custom_parsed_at_idx(
                    "groupid",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            spd_id: row.get_opt_range("spd_id", field_mapping.0[5])?,
            spd_type: row.get_opt_range("spd_type", field_mapping.0[6])?,
            factor: row
                .get_opt_custom_parsed_at_idx(
                    "factor",
                    field_mapping.0[7],
                    mmsdm_core::mms_decimal::parse,
                )?,
            operation: row.get_opt_range("operation", field_mapping.0[8])?,
            defaultvalue: row
                .get_opt_custom_parsed_at_idx(
                    "defaultvalue",
                    field_mapping.0[9],
                    mmsdm_core::mms_decimal::parse,
                )?,
            parameterterm1: row.get_opt_range("parameterterm1", field_mapping.0[10])?,
            parameterterm2: row.get_opt_range("parameterterm2", field_mapping.0[11])?,
            parameterterm3: row.get_opt_range("parameterterm3", field_mapping.0[12])?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[13],
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
        Ok(GeqrhsNull1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let effectivedate = row
            .get_custom_parsed_at_idx(
                "effectivedate",
                5,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(effectivedate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> GeqrhsNull1PrimaryKey {
        GeqrhsNull1PrimaryKey {
            effectivedate: row.effectivedate,
            equationid: row.equationid().to_string(),
            termid: row.termid,
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.effectivedate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "geqrhs_null_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        GeqrhsNull1Row {
            equationid: row.equationid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            termid: row.termid.clone(),
            groupid: row.groupid.clone(),
            spd_id: row.spd_id.clone(),
            spd_type: row.spd_type.clone(),
            factor: row.factor.clone(),
            operation: row.operation.clone(),
            defaultvalue: row.defaultvalue.clone(),
            parameterterm1: row.parameterterm1.clone(),
            parameterterm2: row.parameterterm2.clone(),
            parameterterm3: row.parameterterm3.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeqrhsNull1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub equationid: alloc::string::String,
    pub termid: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for GeqrhsNull1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for GeqrhsNull1Row<'data> {
    type Row<'other> = GeqrhsNull1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.equationid() == row.equationid()
            && self.termid == row.termid && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for GeqrhsNull1Row<'data> {
    type PrimaryKey = GeqrhsNull1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.equationid() == key.equationid
            && self.termid == key.termid && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for GeqrhsNull1PrimaryKey {
    type Row<'other> = GeqrhsNull1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.equationid == row.equationid()
            && self.termid == row.termid && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GeqrhsNull1PrimaryKey {
    type PrimaryKey = GeqrhsNull1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.equationid == key.equationid
            && self.termid == key.termid && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for GeqrhsNull1 {
    type Builder = GeqrhsNull1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "equationid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "termid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "groupid",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "spd_id",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "spd_type",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "factor",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "operation",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "defaultvalue",
                    arrow::datatypes::DataType::Decimal128(16, 6),
                    true,
                ),
                arrow::datatypes::Field::new(
                    "parameterterm1",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "parameterterm2",
                    arrow::datatypes::DataType::Utf8,
                    true,
                ),
                arrow::datatypes::Field::new(
                    "parameterterm3",
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
        GeqrhsNull1Builder {
            equationid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            termid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            groupid_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            spd_id_array: arrow::array::builder::StringBuilder::new(),
            spd_type_array: arrow::array::builder::StringBuilder::new(),
            factor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            operation_array: arrow::array::builder::StringBuilder::new(),
            defaultvalue_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            parameterterm1_array: arrow::array::builder::StringBuilder::new(),
            parameterterm2_array: arrow::array::builder::StringBuilder::new(),
            parameterterm3_array: arrow::array::builder::StringBuilder::new(),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.equationid_array.append_value(row.equationid());
        builder.effectivedate_array.append_value(row.effectivedate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .termid_array
            .append_value({
                let mut val = row.termid;
                val.rescale(0);
                val.mantissa()
            });
        builder
            .groupid_array
            .append_option({
                row.groupid
                    .map(|mut val| {
                        val.rescale(0);
                        val.mantissa()
                    })
            });
        builder.spd_id_array.append_option(row.spd_id());
        builder.spd_type_array.append_option(row.spd_type());
        builder
            .factor_array
            .append_option({
                row.factor
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder.operation_array.append_option(row.operation());
        builder
            .defaultvalue_array
            .append_option({
                row.defaultvalue
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder.parameterterm1_array.append_option(row.parameterterm1());
        builder.parameterterm2_array.append_option(row.parameterterm2());
        builder.parameterterm3_array.append_option(row.parameterterm3());
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
                    alloc::sync::Arc::new(builder.equationid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.termid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.groupid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.spd_id_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.spd_type_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.factor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.operation_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.defaultvalue_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.parameterterm1_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.parameterterm2_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.parameterterm3_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct GeqrhsNull1Builder {
    equationid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    termid_array: arrow::array::builder::Decimal128Builder,
    groupid_array: arrow::array::builder::Decimal128Builder,
    spd_id_array: arrow::array::builder::StringBuilder,
    spd_type_array: arrow::array::builder::StringBuilder,
    factor_array: arrow::array::builder::Decimal128Builder,
    operation_array: arrow::array::builder::StringBuilder,
    defaultvalue_array: arrow::array::builder::Decimal128Builder,
    parameterterm1_array: arrow::array::builder::StringBuilder,
    parameterterm2_array: arrow::array::builder::StringBuilder,
    parameterterm3_array: arrow::array::builder::StringBuilder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SpdcpcNull2;
pub struct SpdcpcNull2Mapping([usize; 7]);
/// # Summary
///
/// ## SPDCONNECTIONPOINTCONSTRAINT
///  _SPDCONNECTIONPOINTCONSTRAINT sets out details of connections point constraints issued in dispatch, predispatch and STPASA._
///
/// * Data Set Name: Spdcpc
/// * File Name: Null
/// * Data Version: 2
///
/// # Description
///  The addition of the BIDTYPE field to SPDCONNECTIONPOINTCONSTRAINT allows constraints to be applied to a dispatchable unit energy and/or Frequency Controlled Ancillary Services dispatch. SPDCONNECTIONPOINTCONSTRAINTdata is public, so is available to all participants. Source SPDCONNECTIONPOINTCONSTRAINT updates whenever new connection point constraints are created.
///
///
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * CONNECTIONPOINTID
/// * EFFECTIVEDATE
/// * GENCONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SpdcpcNull2Row<'data> {
    /// Connection Point Identifier
    pub connectionpointid: core::ops::Range<usize>,
    /// Effective date of this record
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// Generic Constraint Identifier
    pub genconid: core::ops::Range<usize>,
    /// Constraint factor
    pub factor: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Bid Type Identifier; one of (RAISE6SEC, RAISE60SEC, RAISE5MIN, LOWER6SEC, LOWER60SEC, LOWER5MIN, RAISEREG, LOWERREG)
    pub bidtype: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SpdcpcNull2Row<'data> {
    pub fn connectionpointid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.connectionpointid.clone(),
        )
    }
    pub fn genconid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.genconid.clone())
    }
    pub fn bidtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bidtype.clone())
    }
}
impl mmsdm_core::GetTable for SpdcpcNull2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "SPDCPC";
    const TABLE_NAME: &'static str = "NULL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SpdcpcNull2Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "CONNECTIONPOINTID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "GENCONID",
        "FACTOR",
        "LASTCHANGED",
        "BIDTYPE",
    ];
    type Row<'row> = SpdcpcNull2Row<'row>;
    type FieldMapping = SpdcpcNull2Mapping;
    type PrimaryKey = SpdcpcNull2PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SpdcpcNull2Row {
            connectionpointid: row.get_range("connectionpointid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            genconid: row.get_range("genconid", field_mapping.0[3])?,
            factor: row
                .get_opt_custom_parsed_at_idx(
                    "factor",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            bidtype: row.get_range("bidtype", field_mapping.0[6])?,
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
        Ok(SpdcpcNull2Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let effectivedate = row
            .get_custom_parsed_at_idx(
                "effectivedate",
                5,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(effectivedate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SpdcpcNull2PrimaryKey {
        SpdcpcNull2PrimaryKey {
            bidtype: row.bidtype().to_string(),
            connectionpointid: row.connectionpointid().to_string(),
            effectivedate: row.effectivedate,
            genconid: row.genconid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.effectivedate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "spdcpc_null_v2_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SpdcpcNull2Row {
            connectionpointid: row.connectionpointid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            genconid: row.genconid.clone(),
            factor: row.factor.clone(),
            lastchanged: row.lastchanged.clone(),
            bidtype: row.bidtype.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpdcpcNull2PrimaryKey {
    pub bidtype: alloc::string::String,
    pub connectionpointid: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub genconid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SpdcpcNull2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SpdcpcNull2Row<'data> {
    type Row<'other> = SpdcpcNull2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype()
            && self.connectionpointid() == row.connectionpointid()
            && self.effectivedate == row.effectivedate
            && self.genconid() == row.genconid() && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SpdcpcNull2Row<'data> {
    type PrimaryKey = SpdcpcNull2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype
            && self.connectionpointid() == key.connectionpointid
            && self.effectivedate == key.effectivedate && self.genconid() == key.genconid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SpdcpcNull2PrimaryKey {
    type Row<'other> = SpdcpcNull2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype()
            && self.connectionpointid == row.connectionpointid()
            && self.effectivedate == row.effectivedate && self.genconid == row.genconid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SpdcpcNull2PrimaryKey {
    type PrimaryKey = SpdcpcNull2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.connectionpointid == key.connectionpointid
            && self.effectivedate == key.effectivedate && self.genconid == key.genconid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SpdcpcNull2 {
    type Builder = SpdcpcNull2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "connectionpointid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "genconid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "factor",
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
                arrow::datatypes::Field::new(
                    "bidtype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SpdcpcNull2Builder {
            connectionpointid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            genconid_array: arrow::array::builder::StringBuilder::new(),
            factor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            bidtype_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.connectionpointid_array.append_value(row.connectionpointid());
        builder.effectivedate_array.append_value(row.effectivedate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.genconid_array.append_value(row.genconid());
        builder
            .factor_array
            .append_option({
                row.factor
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder.bidtype_array.append_value(row.bidtype());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.connectionpointid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genconid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.factor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SpdcpcNull2Builder {
    connectionpointid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    genconid_array: arrow::array::builder::StringBuilder,
    factor_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    bidtype_array: arrow::array::builder::StringBuilder,
}
pub struct SpdiccNull1;
pub struct SpdiccNull1Mapping([usize; 6]);
/// # Summary
///
/// ## SPDINTERCONNECTORCONSTRAINT
///  _SPDINTERCONNECTORCONSTRAINT contains details on the interconnector constraint factors used in dispatch, predispatch and STPASA. The details set a LHS value._
///
/// * Data Set Name: Spdicc
/// * File Name: Null
/// * Data Version: 1
///
/// # Description
///  SPDINTERCONNECTORCONSTRAINT is public data, and is available to all participants. Source SPDINTERCONNECTORCONSTRAINT updates whenever new connection point constraints are created.
///
///
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * GENCONID
/// * INTERCONNECTORID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SpdiccNull1Row<'data> {
    /// Interconnector Identifier
    pub interconnectorid: core::ops::Range<usize>,
    /// Effective date of this record
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// Generic Constraint Identifier
    pub genconid: core::ops::Range<usize>,
    /// Constraint factor
    pub factor: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SpdiccNull1Row<'data> {
    pub fn interconnectorid(&self) -> &str {
        core::ops::Index::index(
            self.backing_data.as_slice(),
            self.interconnectorid.clone(),
        )
    }
    pub fn genconid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.genconid.clone())
    }
}
impl mmsdm_core::GetTable for SpdiccNull1 {
    const VERSION: i32 = 1;
    const DATA_SET_NAME: &'static str = "SPDICC";
    const TABLE_NAME: &'static str = "NULL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SpdiccNull1Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "INTERCONNECTORID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "GENCONID",
        "FACTOR",
        "LASTCHANGED",
    ];
    type Row<'row> = SpdiccNull1Row<'row>;
    type FieldMapping = SpdiccNull1Mapping;
    type PrimaryKey = SpdiccNull1PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SpdiccNull1Row {
            interconnectorid: row.get_range("interconnectorid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            genconid: row.get_range("genconid", field_mapping.0[3])?,
            factor: row
                .get_opt_custom_parsed_at_idx(
                    "factor",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
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
        Ok(SpdiccNull1Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let effectivedate = row
            .get_custom_parsed_at_idx(
                "effectivedate",
                5,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(effectivedate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SpdiccNull1PrimaryKey {
        SpdiccNull1PrimaryKey {
            effectivedate: row.effectivedate,
            genconid: row.genconid().to_string(),
            interconnectorid: row.interconnectorid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.effectivedate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "spdicc_null_v1_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SpdiccNull1Row {
            interconnectorid: row.interconnectorid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            genconid: row.genconid.clone(),
            factor: row.factor.clone(),
            lastchanged: row.lastchanged.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpdiccNull1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub genconid: alloc::string::String,
    pub interconnectorid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SpdiccNull1PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SpdiccNull1Row<'data> {
    type Row<'other> = SpdiccNull1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.genconid() == row.genconid()
            && self.interconnectorid() == row.interconnectorid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SpdiccNull1Row<'data> {
    type PrimaryKey = SpdiccNull1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.genconid() == key.genconid
            && self.interconnectorid() == key.interconnectorid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SpdiccNull1PrimaryKey {
    type Row<'other> = SpdiccNull1Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.effectivedate == row.effectivedate && self.genconid == row.genconid()
            && self.interconnectorid == row.interconnectorid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SpdiccNull1PrimaryKey {
    type PrimaryKey = SpdiccNull1PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate && self.genconid == key.genconid
            && self.interconnectorid == key.interconnectorid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SpdiccNull1 {
    type Builder = SpdiccNull1Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "interconnectorid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "genconid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "factor",
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
        SpdiccNull1Builder {
            interconnectorid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            genconid_array: arrow::array::builder::StringBuilder::new(),
            factor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.interconnectorid_array.append_value(row.interconnectorid());
        builder.effectivedate_array.append_value(row.effectivedate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.genconid_array.append_value(row.genconid());
        builder
            .factor_array
            .append_option({
                row.factor
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
                    alloc::sync::Arc::new(builder.interconnectorid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genconid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.factor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SpdiccNull1Builder {
    interconnectorid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    genconid_array: arrow::array::builder::StringBuilder,
    factor_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
}
pub struct SpdrcNull2;
pub struct SpdrcNull2Mapping([usize; 7]);
/// # Summary
///
/// ## SPDREGIONCONSTRAINT
///  _SPDREGIONCONSTRAINT contains details on region demand constraint factors used in dispatch. SPDREGIONCONSTRAINTsets a LHS value._
///
/// * Data Set Name: Spdrc
/// * File Name: Null
/// * Data Version: 2
///
/// # Description
///  SPDREGIONCONSTRAINT is public data, and is available to all participants. Source SPDREGIONCONSTRAINT is updated whenever AEMO creates new regional constraints.
///
///
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * EFFECTIVEDATE
/// * GENCONID
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, PartialEq, Eq)]
pub struct SpdrcNull2Row<'data> {
    /// Region Identifier
    pub regionid: core::ops::Range<usize>,
    /// Effective date of this record
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// Generic Constraint Identifier
    pub genconid: core::ops::Range<usize>,
    /// Constraint factor; one of (-1, 1)
    pub factor: Option<rust_decimal::Decimal>,
    /// Last date and time record changed
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// AS Service type - relates to the BidType table; one of (RAISE6SEC, RAISE60SEC, RAISE5MIN, LOWER6SEC, LOWER60SEC, LOWER5MIN, RAISEREG, LOWERREG)
    pub bidtype: core::ops::Range<usize>,
    backing_data: mmsdm_core::CsvRow<'data>,
}
impl<'data> SpdrcNull2Row<'data> {
    pub fn regionid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.regionid.clone())
    }
    pub fn genconid(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.genconid.clone())
    }
    pub fn bidtype(&self) -> &str {
        core::ops::Index::index(self.backing_data.as_slice(), self.bidtype.clone())
    }
}
impl mmsdm_core::GetTable for SpdrcNull2 {
    const VERSION: i32 = 2;
    const DATA_SET_NAME: &'static str = "SPDRC";
    const TABLE_NAME: &'static str = "NULL";
    const DEFAULT_FIELD_MAPPING: Self::FieldMapping = SpdrcNull2Mapping([
        4,
        5,
        6,
        7,
        8,
        9,
        10,
    ]);
    const COLUMNS: &'static [&'static str] = &[
        "REGIONID",
        "EFFECTIVEDATE",
        "VERSIONNO",
        "GENCONID",
        "FACTOR",
        "LASTCHANGED",
        "BIDTYPE",
    ];
    type Row<'row> = SpdrcNull2Row<'row>;
    type FieldMapping = SpdrcNull2Mapping;
    type PrimaryKey = SpdrcNull2PrimaryKey;
    type Partition = mmsdm_core::YearMonth;
    fn from_row<'data>(
        row: mmsdm_core::CsvRow<'data>,
        field_mapping: &Self::FieldMapping,
    ) -> mmsdm_core::Result<Self::Row<'data>> {
        Ok(SpdrcNull2Row {
            regionid: row.get_range("regionid", field_mapping.0[0])?,
            effectivedate: row
                .get_custom_parsed_at_idx(
                    "effectivedate",
                    field_mapping.0[1],
                    mmsdm_core::mms_datetime::parse,
                )?,
            versionno: row
                .get_custom_parsed_at_idx(
                    "versionno",
                    field_mapping.0[2],
                    mmsdm_core::mms_decimal::parse,
                )?,
            genconid: row.get_range("genconid", field_mapping.0[3])?,
            factor: row
                .get_opt_custom_parsed_at_idx(
                    "factor",
                    field_mapping.0[4],
                    mmsdm_core::mms_decimal::parse,
                )?,
            lastchanged: row
                .get_opt_custom_parsed_at_idx(
                    "lastchanged",
                    field_mapping.0[5],
                    mmsdm_core::mms_datetime::parse,
                )?,
            bidtype: row.get_range("bidtype", field_mapping.0[6])?,
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
        Ok(SpdrcNull2Mapping(base_mapping))
    }
    fn partition_suffix_from_row<'a>(
        row: mmsdm_core::CsvRow<'a>,
    ) -> mmsdm_core::Result<Self::Partition> {
        let effectivedate = row
            .get_custom_parsed_at_idx(
                "effectivedate",
                5,
                mmsdm_core::mms_datetime::parse,
            )?;
        Ok(mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(effectivedate).month(),
                )
                .unwrap(),
        })
    }
    fn matches_file_key(key: &mmsdm_core::FileKey<'_>, version: i32) -> bool {
        version == key.version && Self::DATA_SET_NAME == key.data_set_name()
            && Self::TABLE_NAME == key.table_name()
    }
    fn primary_key(row: &Self::Row<'_>) -> SpdrcNull2PrimaryKey {
        SpdrcNull2PrimaryKey {
            bidtype: row.bidtype().to_string(),
            effectivedate: row.effectivedate,
            genconid: row.genconid().to_string(),
            regionid: row.regionid().to_string(),
            versionno: row.versionno,
        }
    }
    fn partition_suffix(row: &Self::Row<'_>) -> Self::Partition {
        mmsdm_core::YearMonth {
            year: chrono::NaiveDateTime::from(row.effectivedate).year(),
            month: num_traits::FromPrimitive::from_u32(
                    chrono::NaiveDateTime::from(row.effectivedate).month(),
                )
                .unwrap(),
        }
    }
    fn partition_name(row: &Self::Row<'_>) -> alloc::string::String {
        alloc::format!(
            "spdrc_null_v2_{}_{}", Self::partition_suffix(& row).year,
            Self::partition_suffix(& row).month.number_from_month()
        )
    }
    fn to_static<'a>(row: &Self::Row<'a>) -> Self::Row<'static> {
        SpdrcNull2Row {
            regionid: row.regionid.clone(),
            effectivedate: row.effectivedate.clone(),
            versionno: row.versionno.clone(),
            genconid: row.genconid.clone(),
            factor: row.factor.clone(),
            lastchanged: row.lastchanged.clone(),
            bidtype: row.bidtype.clone(),
            backing_data: row.backing_data.to_owned(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpdrcNull2PrimaryKey {
    pub bidtype: alloc::string::String,
    pub effectivedate: chrono::NaiveDateTime,
    pub genconid: alloc::string::String,
    pub regionid: alloc::string::String,
    pub versionno: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for SpdrcNull2PrimaryKey {}
impl<'data> mmsdm_core::CompareWithRow for SpdrcNull2Row<'data> {
    type Row<'other> = SpdrcNull2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype() == row.bidtype() && self.effectivedate == row.effectivedate
            && self.genconid() == row.genconid() && self.regionid() == row.regionid()
            && self.versionno == row.versionno
    }
}
impl<'data> mmsdm_core::CompareWithPrimaryKey for SpdrcNull2Row<'data> {
    type PrimaryKey = SpdrcNull2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype() == key.bidtype && self.effectivedate == key.effectivedate
            && self.genconid() == key.genconid && self.regionid() == key.regionid
            && self.versionno == key.versionno
    }
}
impl<'data> mmsdm_core::CompareWithRow for SpdrcNull2PrimaryKey {
    type Row<'other> = SpdrcNull2Row<'other>;
    fn compare_with_row<'other>(&self, row: &Self::Row<'other>) -> bool {
        self.bidtype == row.bidtype() && self.effectivedate == row.effectivedate
            && self.genconid == row.genconid() && self.regionid == row.regionid()
            && self.versionno == row.versionno
    }
}
impl mmsdm_core::CompareWithPrimaryKey for SpdrcNull2PrimaryKey {
    type PrimaryKey = SpdrcNull2PrimaryKey;
    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype && self.effectivedate == key.effectivedate
            && self.genconid == key.genconid && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for SpdrcNull2 {
    type Builder = SpdrcNull2Builder;
    fn schema() -> arrow::datatypes::Schema {
        arrow::datatypes::Schema::new(
            alloc::vec::Vec::from([
                arrow::datatypes::Field::new(
                    "regionid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "effectivedate",
                    arrow::datatypes::DataType::Timestamp(
                        arrow::datatypes::TimeUnit::Millisecond,
                        None,
                    ),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "versionno",
                    arrow::datatypes::DataType::Decimal128(3, 0),
                    false,
                ),
                arrow::datatypes::Field::new(
                    "genconid",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
                arrow::datatypes::Field::new(
                    "factor",
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
                arrow::datatypes::Field::new(
                    "bidtype",
                    arrow::datatypes::DataType::Utf8,
                    false,
                ),
            ]),
        )
    }
    fn new_builder() -> Self::Builder {
        SpdrcNull2Builder {
            regionid_array: arrow::array::builder::StringBuilder::new(),
            effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            versionno_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(3, 0)),
            genconid_array: arrow::array::builder::StringBuilder::new(),
            factor_array: arrow::array::builder::Decimal128Builder::new()
                .with_data_type(arrow::datatypes::DataType::Decimal128(16, 6)),
            lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder::new(),
            bidtype_array: arrow::array::builder::StringBuilder::new(),
        }
    }
    fn append_builder(builder: &mut Self::Builder, row: Self::Row<'_>) {
        builder.regionid_array.append_value(row.regionid());
        builder.effectivedate_array.append_value(row.effectivedate.timestamp_millis());
        builder
            .versionno_array
            .append_value({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
        builder.genconid_array.append_value(row.genconid());
        builder
            .factor_array
            .append_option({
                row.factor
                    .map(|mut val| {
                        val.rescale(6);
                        val.mantissa()
                    })
            });
        builder
            .lastchanged_array
            .append_option(row.lastchanged.map(|val| val.timestamp_millis()));
        builder.bidtype_array.append_value(row.bidtype());
    }
    fn finalize_builder(
        builder: &mut Self::Builder,
    ) -> mmsdm_core::Result<arrow::array::RecordBatch> {
        arrow::array::RecordBatch::try_new(
                alloc::sync::Arc::new(<Self as mmsdm_core::ArrowSchema>::schema()),
                alloc::vec::Vec::from([
                    alloc::sync::Arc::new(builder.regionid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.effectivedate_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.versionno_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.genconid_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.factor_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.lastchanged_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                    alloc::sync::Arc::new(builder.bidtype_array.finish())
                        as alloc::sync::Arc<dyn arrow::array::Array>,
                ]),
            )
            .map_err(Into::into)
    }
}
#[cfg(feature = "arrow")]
pub struct SpdrcNull2Builder {
    regionid_array: arrow::array::builder::StringBuilder,
    effectivedate_array: arrow::array::builder::TimestampMillisecondBuilder,
    versionno_array: arrow::array::builder::Decimal128Builder,
    genconid_array: arrow::array::builder::StringBuilder,
    factor_array: arrow::array::builder::Decimal128Builder,
    lastchanged_array: arrow::array::builder::TimestampMillisecondBuilder,
    bidtype_array: arrow::array::builder::StringBuilder,
}
