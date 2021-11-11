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
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * SPD_ID
/// * SPD_TYPE
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GenericConstraintEmsmaster1 {
    /// ID defining data source
    pub spd_id: String,
    /// ID describing type of data source
    pub spd_type: String,
    /// The detailed description of the SCADA point associated with the SPD_ID
    pub description: Option<String>,
    /// The Grouping associated with the SPD ID - most often a RegionID
    pub grouping_id: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for GenericConstraintEmsmaster1 {
    type PrimaryKey = GenericConstraintEmsmaster1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GENERIC_CONSTRAINT".into(),
            table_name: Some("EMSMASTER".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> GenericConstraintEmsmaster1PrimaryKey {
        GenericConstraintEmsmaster1PrimaryKey {
            spd_id: self.spd_id.clone(),
            spd_type: self.spd_type.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "generic_constraint_emsmaster_v1".to_string()
    }
}
impl crate::CompareWithRow for GenericConstraintEmsmaster1 {
    type Row = GenericConstraintEmsmaster1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.spd_id == row.spd_id && self.spd_type == row.spd_type
    }
}
impl crate::CompareWithPrimaryKey for GenericConstraintEmsmaster1 {
    type PrimaryKey = GenericConstraintEmsmaster1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.spd_id == key.spd_id && self.spd_type == key.spd_type
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenericConstraintEmsmaster1PrimaryKey {
    pub spd_id: String,
    pub spd_type: String,
}
impl crate::CompareWithRow for GenericConstraintEmsmaster1PrimaryKey {
    type Row = GenericConstraintEmsmaster1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.spd_id == row.spd_id && self.spd_type == row.spd_type
    }
}
impl crate::CompareWithPrimaryKey for GenericConstraintEmsmaster1PrimaryKey {
    type PrimaryKey = GenericConstraintEmsmaster1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.spd_id == key.spd_id && self.spd_type == key.spd_type
    }
}
impl crate::PrimaryKey for GenericConstraintEmsmaster1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for GenericConstraintEmsmaster1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("spd_id", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "spd_type",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "grouping_id",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut spd_id_array = Vec::new();
        let mut spd_type_array = Vec::new();
        let mut description_array = Vec::new();
        let mut grouping_id_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            spd_id_array.push(row.spd_id);
            spd_type_array.push(row.spd_type);
            description_array.push(row.description);
            grouping_id_array.push(row.grouping_id);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(spd_id_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(spd_type_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(grouping_id_array)),
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
/// ## GENCONDATA
///  _GENCONDATA sets out the generic constraints contained within a generic constraint set invoked in PASA, predispatch and dispatch.<br>Fields enable selective application of invoked constraints in the Dispatch, Predispatch, ST PASA or MT PASA processes.<br>_
///
/// * Data Set Name: Gencondata
/// * File Name: Null
/// * Data Version: 6
///
/// # Description
///  GENCONDATA is a public data, and is available to all participants. Source GENCONDATA updates as constraint details are updated by AEMO. Note The following fields enable selective application of invoked constraints in the Dispatch, Predispatch, ST PASA or MT PASA processes: ·	 DISPATCH ·	 PREDISPATCH ·	 STPASA ·	 MTPASA The flag P5MIN_SCOPE_OVERRIDE indicates for each constraint whether 5MPD makes use of the default Dispatch (P5MIN_SCOPE_OVERRIDE = NULL) or Pre-dispatch (P5MIN_SCOPE_OVERRIDE = ‘PD’) style RHS definition. GENERICCONSTRAINTRHS stores generic constraint RHS definitions. Constraints without records in GENERICCONSTRAINTRHS only make use of the static RHS defined in the CONSTRAINTVALUE column in GENCONDATA . The default value for the P5MIN_SCOPE_OVERRIDE column is NULL, so constraints existing before implementing the column use the DISPATCH RHS definition by default, as was the case before the implementation of the change.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * GENCONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GencondataNull6 {
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version with respect to the effective date
    pub versionno: rust_decimal::Decimal,
    /// Unique ID for the constraint
    pub genconid: String,
    /// The logical operator (=, &gt;=, &lt;=)
    pub constrainttype: Option<String>,
    /// the RHS value used if there is no dynamic RHS defined in GenericConstraintRHS
    pub constraintvalue: Option<rust_decimal::Decimal>,
    /// Detail of the plant that is not in service
    pub description: Option<String>,
    /// Not used
    pub status: Option<String>,
    /// The constraint violation penalty factor
    pub genericconstraintweight: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorising record
    pub authorisedby: Option<String>,
    /// Not used
    pub dynamicrhs: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Flag: constraint RHS used for Dispatch? 1-used, 0-not used
    pub dispatch: Option<String>,
    /// Flag to indicate if the constraint RHS is to be used for PreDispatch, 1-used, 0-not used
    pub predispatch: Option<String>,
    /// Flag to indicate if the constraint RHS is to be used for ST PASA, 1-used, 0-not used
    pub stpasa: Option<String>,
    /// Flag to indicate if the constraint RHS is to be used for MT PASA, 1-used, 0-not used
    pub mtpasa: Option<String>,
    /// The device(s) that is affected by the constraint e.g. Interconnector, Generator(s) or Cutset
    pub impact: Option<String>,
    /// The source of the constraint formulation
    pub source: Option<String>,
    /// The limit type of the constraint e.g. Transient Stability, Voltage Stability
    pub limittype: Option<String>,
    /// The contingency or reason for the constraint
    pub reason: Option<String>,
    /// Details of the changes made to this version of the constraint
    pub modifications: Option<String>,
    /// Extra notes on the constraint
    pub additionalnotes: Option<String>,
    /// Extra notes on the constraint: NULL = Dispatch RHS applied in 5MPD, PD = PreDispatch RHS applied in 5MPD
    pub p5min_scope_override: Option<String>,
    /// Flag to indicate if PASA LRC run uses the constraint; 1-used, 0-not used
    pub lrc: Option<String>,
    /// Flag to indicate if PASA LOR run uses the constraint; 1-used, 0-not used
    pub lor: Option<String>,
    /// Flags Constraints for which NEMDE must use "InitialMW" values instead of "WhatOfInitialMW" for Intervention Pricing runs
    pub force_scada: Option<rust_decimal::Decimal>,
}
impl crate::GetTable for GencondataNull6 {
    type PrimaryKey = GencondataNull6PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GENCONDATA".into(),
            table_name: Some("NULL".into()),
            version: 6,
        }
    }

    fn primary_key(&self) -> GencondataNull6PrimaryKey {
        GencondataNull6PrimaryKey {
            effectivedate: self.effectivedate.clone(),
            genconid: self.genconid.clone(),
            versionno: self.versionno.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "gencondata_null_v6".to_string()
    }
}
impl crate::CompareWithRow for GencondataNull6 {
    type Row = GencondataNull6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.genconid == row.genconid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for GencondataNull6 {
    type PrimaryKey = GencondataNull6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.genconid == key.genconid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GencondataNull6PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub genconid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for GencondataNull6PrimaryKey {
    type Row = GencondataNull6;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.genconid == row.genconid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for GencondataNull6PrimaryKey {
    type PrimaryKey = GencondataNull6PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.genconid == key.genconid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for GencondataNull6PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for GencondataNull6 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "genconid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "constrainttype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "constraintvalue",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("status", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "genericconstraintweight",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "dynamicrhs",
                arrow2::datatypes::DataType::Decimal(15, 5),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("dispatch", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "predispatch",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("stpasa", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("mtpasa", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("impact", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("source", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "limittype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("reason", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "modifications",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "additionalnotes",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "p5min_scope_override",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lrc", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("lor", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "force_scada",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut genconid_array = Vec::new();
        let mut constrainttype_array = Vec::new();
        let mut constraintvalue_array = Vec::new();
        let mut description_array = Vec::new();
        let mut status_array = Vec::new();
        let mut genericconstraintweight_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut dynamicrhs_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut dispatch_array = Vec::new();
        let mut predispatch_array = Vec::new();
        let mut stpasa_array = Vec::new();
        let mut mtpasa_array = Vec::new();
        let mut impact_array = Vec::new();
        let mut source_array = Vec::new();
        let mut limittype_array = Vec::new();
        let mut reason_array = Vec::new();
        let mut modifications_array = Vec::new();
        let mut additionalnotes_array = Vec::new();
        let mut p5min_scope_override_array = Vec::new();
        let mut lrc_array = Vec::new();
        let mut lor_array = Vec::new();
        let mut force_scada_array = Vec::new();
        for (_, row) in partition {
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            genconid_array.push(row.genconid);
            constrainttype_array.push(row.constrainttype);
            constraintvalue_array.push({
                row.constraintvalue.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            description_array.push(row.description);
            status_array.push(row.status);
            genericconstraintweight_array.push({
                row.genericconstraintweight.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            authoriseddate_array.push(row.authoriseddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            authorisedby_array.push(row.authorisedby);
            dynamicrhs_array.push({
                row.dynamicrhs.map(|mut val| {
                    val.rescale(5);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            dispatch_array.push(row.dispatch);
            predispatch_array.push(row.predispatch);
            stpasa_array.push(row.stpasa);
            mtpasa_array.push(row.mtpasa);
            impact_array.push(row.impact);
            source_array.push(row.source);
            limittype_array.push(row.limittype);
            reason_array.push(row.reason);
            modifications_array.push(row.modifications);
            additionalnotes_array.push(row.additionalnotes);
            p5min_scope_override_array.push(row.p5min_scope_override);
            lrc_array.push(row.lrc);
            lor_array.push(row.lor);
            force_scada_array.push({
                row.force_scada.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(genconid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(constrainttype_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(constraintvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(status_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(genericconstraintweight_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(dynamicrhs_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 5)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(dispatch_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(predispatch_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(stpasa_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(mtpasa_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(impact_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(source_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(limittype_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(reason_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(modifications_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(additionalnotes_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    p5min_scope_override_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(lrc_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(lor_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(force_scada_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
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
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * GENCONID
/// * GENCONSETID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GenconsetNull1 {
    /// Unique ID for the Constraint Set
    pub genconsetid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of the record for the given effective date
    pub versionno: rust_decimal::Decimal,
    /// Generic Contraint ID
    pub genconid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub genconeffdate: Option<chrono::NaiveDateTime>,
    /// Since market start in 1998 these fields have not been used and any data that has been populated in the fields should be ignored
    pub genconversionno: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for GenconsetNull1 {
    type PrimaryKey = GenconsetNull1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GENCONSET".into(),
            table_name: Some("NULL".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> GenconsetNull1PrimaryKey {
        GenconsetNull1PrimaryKey {
            effectivedate: self.effectivedate.clone(),
            genconid: self.genconid.clone(),
            genconsetid: self.genconsetid.clone(),
            versionno: self.versionno.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "genconset_null_v1".to_string()
    }
}
impl crate::CompareWithRow for GenconsetNull1 {
    type Row = GenconsetNull1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.genconid == row.genconid
            && self.genconsetid == row.genconsetid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for GenconsetNull1 {
    type PrimaryKey = GenconsetNull1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.genconid == key.genconid
            && self.genconsetid == key.genconsetid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenconsetNull1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub genconid: String,
    pub genconsetid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for GenconsetNull1PrimaryKey {
    type Row = GenconsetNull1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.genconid == row.genconid
            && self.genconsetid == row.genconsetid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for GenconsetNull1PrimaryKey {
    type PrimaryKey = GenconsetNull1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.genconid == key.genconid
            && self.genconsetid == key.genconsetid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for GenconsetNull1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for GenconsetNull1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "genconsetid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "genconid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "genconeffdate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "genconversionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut genconsetid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut genconid_array = Vec::new();
        let mut genconeffdate_array = Vec::new();
        let mut genconversionno_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            genconsetid_array.push(row.genconsetid);
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            genconid_array.push(row.genconid);
            genconeffdate_array.push(row.genconeffdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            genconversionno_array.push({
                row.genconversionno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    genconsetid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(genconid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(genconeffdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(genconversionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
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
/// ## GENCONSETINVOKE
///  _GENCONSETINVOKE provides details of invoked and revoked generic constraints. GENCONSETINVOKE is the key table for determining what constraints are active in dispatch, predispatch and PASA.<br>GENCONSETINVOKE also indicates whether constraints are for interconnector limits, ancillary services, etc.<br>_
///
/// * Data Set Name: Generic Constraint
/// * File Name: Genconsetinvoke
/// * Data Version: 2
///
/// # Description
///  GENCONSETINVOKE is public data. All participants have access to this data. Source GENCONSETINVOKE updates each time a generic constraint is invoked or revoke time is altered. Once past the time, these times cannot be altered. Note The Replica software does not handle the deletion of GENCONSETINVOKE records. To workaround this problem, the field STARTAUTHORISEDBY indicates whether a constraint set invocation is applicable. A non-null value for the STARTAUTHORISEDBY field indicates that the constraint invocation is active. Essentially inactive invocations have a null value for the STARTAUTHORISEDBY field. To remove inactive invocations from queries on the GENCONSETINVOKE table, add the following text to the where clause "and STARTAUTHORISEDBY is not null".
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INVOCATION_ID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GenericConstraintGenconsetinvoke2 {
    /// Abstract unique identifier for the record. Allows Invocations to be modified without affecting PK values
    pub invocation_id: i64,
    #[serde(with = "crate::mms_datetime")]
    pub startdate: chrono::NaiveDateTime,
    /// The first dispatch interval of the invocation being the dispatch interval number starting from1 at 04:05.
    pub startperiod: rust_decimal::Decimal,
    /// Unique generic constraint set identifier
    pub genconsetid: String,
    #[serde(with = "crate::mms_datetime_opt")]
    pub enddate: Option<chrono::NaiveDateTime>,
    /// Dispatch interval number end
    pub endperiod: Option<rust_decimal::Decimal>,
    /// User authorising invoke, indicating a constraint set invocation is applicable (i.e. non-null). A null value indicates inactive invocation.
    pub startauthorisedby: Option<String>,
    /// user authorising revoke.
    pub endauthorisedby: Option<String>,
    /// 0 is not intervention, 1 is intervention and causes dispatch to solve twice.
    pub intervention: Option<String>,
    /// Constraint type (e.g. ancillary services). This also flags where a constraint is an interconnector or intra-region network limit.
    pub asconstrainttype: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub startintervaldatetime: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub endintervaldatetime: Option<chrono::NaiveDateTime>,
    /// Flag to indicate if the constraint set is a system normal (1) or an outage set (0)
    pub systemnormal: Option<String>,
}
impl crate::GetTable for GenericConstraintGenconsetinvoke2 {
    type PrimaryKey = GenericConstraintGenconsetinvoke2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GENERIC_CONSTRAINT".into(),
            table_name: Some("GENCONSETINVOKE".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> GenericConstraintGenconsetinvoke2PrimaryKey {
        GenericConstraintGenconsetinvoke2PrimaryKey {
            invocation_id: self.invocation_id.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "generic_constraint_genconsetinvoke_v2".to_string()
    }
}
impl crate::CompareWithRow for GenericConstraintGenconsetinvoke2 {
    type Row = GenericConstraintGenconsetinvoke2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.invocation_id == row.invocation_id
    }
}
impl crate::CompareWithPrimaryKey for GenericConstraintGenconsetinvoke2 {
    type PrimaryKey = GenericConstraintGenconsetinvoke2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.invocation_id == key.invocation_id
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenericConstraintGenconsetinvoke2PrimaryKey {
    pub invocation_id: i64,
}
impl crate::CompareWithRow for GenericConstraintGenconsetinvoke2PrimaryKey {
    type Row = GenericConstraintGenconsetinvoke2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.invocation_id == row.invocation_id
    }
}
impl crate::CompareWithPrimaryKey for GenericConstraintGenconsetinvoke2PrimaryKey {
    type PrimaryKey = GenericConstraintGenconsetinvoke2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.invocation_id == key.invocation_id
    }
}
impl crate::PrimaryKey for GenericConstraintGenconsetinvoke2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for GenericConstraintGenconsetinvoke2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "invocation_id",
                arrow2::datatypes::DataType::Int64,
                false,
            ),
            arrow2::datatypes::Field::new("startdate", arrow2::datatypes::DataType::Date32, false),
            arrow2::datatypes::Field::new(
                "startperiod",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "genconsetid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new("enddate", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "endperiod",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "startauthorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "endauthorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "intervention",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "asconstrainttype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "startintervaldatetime",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "endintervaldatetime",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "systemnormal",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut invocation_id_array = Vec::new();
        let mut startdate_array = Vec::new();
        let mut startperiod_array = Vec::new();
        let mut genconsetid_array = Vec::new();
        let mut enddate_array = Vec::new();
        let mut endperiod_array = Vec::new();
        let mut startauthorisedby_array = Vec::new();
        let mut endauthorisedby_array = Vec::new();
        let mut intervention_array = Vec::new();
        let mut asconstrainttype_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut startintervaldatetime_array = Vec::new();
        let mut endintervaldatetime_array = Vec::new();
        let mut systemnormal_array = Vec::new();
        for (_, row) in partition {
            invocation_id_array.push(row.invocation_id);
            startdate_array.push(
                i32::try_from(
                    (row.startdate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            startperiod_array.push({
                let mut val = row.startperiod;
                val.rescale(0);
                val.mantissa()
            });
            genconsetid_array.push(row.genconsetid);
            enddate_array.push(row.enddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            endperiod_array.push({
                row.endperiod.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            startauthorisedby_array.push(row.startauthorisedby);
            endauthorisedby_array.push(row.endauthorisedby);
            intervention_array.push(row.intervention);
            asconstrainttype_array.push(row.asconstrainttype);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            startintervaldatetime_array.push(row.startintervaldatetime.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            endintervaldatetime_array.push(row.endintervaldatetime.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            systemnormal_array.push(row.systemnormal);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from_slice(
                    invocation_id_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(startdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(startperiod_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    genconsetid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(enddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(endperiod_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    startauthorisedby_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(endauthorisedby_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(intervention_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    asconstrainttype_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(startintervaldatetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(endintervaldatetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(systemnormal_array)),
            ],
        )
        .map_err(Into::into)
    }
}
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
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * GENCONSETID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GenconsettrkNull2 {
    /// Unique ID for the Constraint Set
    pub genconsetid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of the record for the given effective date
    pub versionno: rust_decimal::Decimal,
    /// Description of the constraint
    pub description: Option<String>,
    /// The person who authorised the constraint set
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The region the constraint set is located in or a special grouping (e.g. CHIMERA)
    pub coverage: Option<String>,
    /// Details of the changes made to this version of the constraint set
    pub modifications: Option<String>,
    /// Not used as of 2005 End of Year Release [was Flag to indicate if the constraint set is a system normal (1) or and an outage set (0)]
    pub systemnormal: Option<String>,
    /// Detail of the plant that is not in service
    pub outage: Option<String>,
}
impl crate::GetTable for GenconsettrkNull2 {
    type PrimaryKey = GenconsettrkNull2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GENCONSETTRK".into(),
            table_name: Some("NULL".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> GenconsettrkNull2PrimaryKey {
        GenconsettrkNull2PrimaryKey {
            effectivedate: self.effectivedate.clone(),
            genconsetid: self.genconsetid.clone(),
            versionno: self.versionno.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "genconsettrk_null_v2".to_string()
    }
}
impl crate::CompareWithRow for GenconsettrkNull2 {
    type Row = GenconsettrkNull2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.genconsetid == row.genconsetid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for GenconsettrkNull2 {
    type PrimaryKey = GenconsettrkNull2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.genconsetid == key.genconsetid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GenconsettrkNull2PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub genconsetid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for GenconsettrkNull2PrimaryKey {
    type Row = GenconsettrkNull2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.genconsetid == row.genconsetid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for GenconsettrkNull2PrimaryKey {
    type PrimaryKey = GenconsettrkNull2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.genconsetid == key.genconsetid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for GenconsettrkNull2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for GenconsettrkNull2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "genconsetid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
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
            arrow2::datatypes::Field::new("coverage", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "modifications",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "systemnormal",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("outage", arrow2::datatypes::DataType::LargeUtf8, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut genconsetid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut description_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut coverage_array = Vec::new();
        let mut modifications_array = Vec::new();
        let mut systemnormal_array = Vec::new();
        let mut outage_array = Vec::new();
        for (_, row) in partition {
            genconsetid_array.push(row.genconsetid);
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            description_array.push(row.description);
            authorisedby_array.push(row.authorisedby);
            authoriseddate_array.push(row.authoriseddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            coverage_array.push(row.coverage);
            modifications_array.push(row.modifications);
            systemnormal_array.push(row.systemnormal);
            outage_array.push(row.outage);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    genconsetid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(coverage_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(modifications_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(systemnormal_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(outage_array)),
            ],
        )
        .map_err(Into::into)
    }
}
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
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * GENCONID
/// * SCOPE
/// * TERMID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GcrhsNull1 {
    /// Generic Constraint Identifier
    pub genconid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// Scope of RHS term (DS, PD, ST or EQ)
    pub scope: String,
    /// The unique identifier for the a constraint RHS term
    pub termid: rust_decimal::Decimal,
    /// ID of super-term, if this is a sub-term
    pub groupid: Option<rust_decimal::Decimal>,
    /// ID defining data source
    pub spd_id: Option<String>,
    /// ID describing type of data source
    pub spd_type: Option<String>,
    /// Multiplier applied to operator result
    pub factor: Option<rust_decimal::Decimal>,
    /// Unitary operator to apply to data value
    pub operation: Option<String>,
    /// Default value if primary source given by SPD_ID and SPD_TYPE not available.
    pub defaultvalue: Option<rust_decimal::Decimal>,
    /// The unique identifier for the first term (logic expression) to use in a Branch term
    pub parameterterm1: Option<String>,
    /// The unique identifier for the second term (logic&lt;=0 result) to use in a Branch term
    pub parameterterm2: Option<String>,
    /// The unique identifier for the third term (logic&gt;0 result) to use in a Branch term
    pub parameterterm3: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for GcrhsNull1 {
    type PrimaryKey = GcrhsNull1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GCRHS".into(),
            table_name: Some("NULL".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> GcrhsNull1PrimaryKey {
        GcrhsNull1PrimaryKey {
            effectivedate: self.effectivedate.clone(),
            genconid: self.genconid.clone(),
            scope: self.scope.clone(),
            termid: self.termid.clone(),
            versionno: self.versionno.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "gcrhs_null_v1".to_string()
    }
}
impl crate::CompareWithRow for GcrhsNull1 {
    type Row = GcrhsNull1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.genconid == row.genconid
            && self.scope == row.scope
            && self.termid == row.termid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for GcrhsNull1 {
    type PrimaryKey = GcrhsNull1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.genconid == key.genconid
            && self.scope == key.scope
            && self.termid == key.termid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GcrhsNull1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub genconid: String,
    pub scope: String,
    pub termid: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for GcrhsNull1PrimaryKey {
    type Row = GcrhsNull1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.genconid == row.genconid
            && self.scope == row.scope
            && self.termid == row.termid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for GcrhsNull1PrimaryKey {
    type PrimaryKey = GcrhsNull1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.genconid == key.genconid
            && self.scope == key.scope
            && self.termid == key.termid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for GcrhsNull1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for GcrhsNull1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "genconid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(22, 0),
                false,
            ),
            arrow2::datatypes::Field::new("scope", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "termid",
                arrow2::datatypes::DataType::Decimal(4, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "groupid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new("spd_id", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("spd_type", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "factor",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "operation",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "defaultvalue",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "parameterterm1",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "parameterterm2",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "parameterterm3",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut genconid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut scope_array = Vec::new();
        let mut termid_array = Vec::new();
        let mut groupid_array = Vec::new();
        let mut spd_id_array = Vec::new();
        let mut spd_type_array = Vec::new();
        let mut factor_array = Vec::new();
        let mut operation_array = Vec::new();
        let mut defaultvalue_array = Vec::new();
        let mut parameterterm1_array = Vec::new();
        let mut parameterterm2_array = Vec::new();
        let mut parameterterm3_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            genconid_array.push(row.genconid);
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            scope_array.push(row.scope);
            termid_array.push({
                let mut val = row.termid;
                val.rescale(0);
                val.mantissa()
            });
            groupid_array.push({
                row.groupid.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            spd_id_array.push(row.spd_id);
            spd_type_array.push(row.spd_type);
            factor_array.push({
                row.factor.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            operation_array.push(row.operation);
            defaultvalue_array.push({
                row.defaultvalue.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            parameterterm1_array.push(row.parameterterm1);
            parameterterm2_array.push(row.parameterterm2);
            parameterterm3_array.push(row.parameterterm3);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(genconid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(scope_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(termid_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(groupid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(spd_id_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(spd_type_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(factor_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(operation_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(defaultvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(parameterterm1_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(parameterterm2_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(parameterterm3_array)),
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
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EQUATIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GeqdescNull2 {
    /// Generic Equation Identifier
    pub equationid: String,
    /// Generic Equation Description
    pub description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// The device(s) affected by the constraint (e.g. Interconnector, Generator(s) or Cutset)
    pub impact: Option<String>,
    /// The source of the constraint formulation
    pub source: Option<String>,
    /// The limit type of the constraint e.g. Transient Stability, Voltage Stability
    pub limittype: Option<String>,
    /// The contingency or reason for the constraint
    pub reason: Option<String>,
    /// Details of the changes made to this version of the generic equation RHS
    pub modifications: Option<String>,
    /// Extra notes on the constraint
    pub additionalnotes: Option<String>,
}
impl crate::GetTable for GeqdescNull2 {
    type PrimaryKey = GeqdescNull2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GEQDESC".into(),
            table_name: Some("NULL".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> GeqdescNull2PrimaryKey {
        GeqdescNull2PrimaryKey {
            equationid: self.equationid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "geqdesc_null_v2".to_string()
    }
}
impl crate::CompareWithRow for GeqdescNull2 {
    type Row = GeqdescNull2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.equationid == row.equationid
    }
}
impl crate::CompareWithPrimaryKey for GeqdescNull2 {
    type PrimaryKey = GeqdescNull2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.equationid == key.equationid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeqdescNull2PrimaryKey {
    pub equationid: String,
}
impl crate::CompareWithRow for GeqdescNull2PrimaryKey {
    type Row = GeqdescNull2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.equationid == row.equationid
    }
}
impl crate::CompareWithPrimaryKey for GeqdescNull2PrimaryKey {
    type PrimaryKey = GeqdescNull2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.equationid == key.equationid
    }
}
impl crate::PrimaryKey for GeqdescNull2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for GeqdescNull2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "equationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("impact", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("source", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "limittype",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("reason", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "modifications",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "additionalnotes",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut equationid_array = Vec::new();
        let mut description_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut impact_array = Vec::new();
        let mut source_array = Vec::new();
        let mut limittype_array = Vec::new();
        let mut reason_array = Vec::new();
        let mut modifications_array = Vec::new();
        let mut additionalnotes_array = Vec::new();
        for (_, row) in partition {
            equationid_array.push(row.equationid);
            description_array.push(row.description);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            impact_array.push(row.impact);
            source_array.push(row.source);
            limittype_array.push(row.limittype);
            reason_array.push(row.reason);
            modifications_array.push(row.modifications);
            additionalnotes_array.push(row.additionalnotes);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    equationid_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(impact_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(source_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(limittype_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(reason_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(modifications_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(additionalnotes_array)),
            ],
        )
        .map_err(Into::into)
    }
}
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
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * EQUATIONID
/// * TERMID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GeqrhsNull1 {
    /// Generic Equation Identifier
    pub equationid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// The unique identifier for the a equation RHS term
    pub termid: rust_decimal::Decimal,
    /// ID of super-term, if this is a sub-term
    pub groupid: Option<rust_decimal::Decimal>,
    /// ID defining data source
    pub spd_id: Option<String>,
    /// ID describing type of data source
    pub spd_type: Option<String>,
    /// Multiplier applied to operator result
    pub factor: Option<rust_decimal::Decimal>,
    /// Unitary operator to apply to data value
    pub operation: Option<String>,
    /// Default value if primary source given by SPD_ID and SPD_TYPE not available.
    pub defaultvalue: Option<rust_decimal::Decimal>,
    /// The unique identifier for the first term (logic expression) to use in a Branch term
    pub parameterterm1: Option<String>,
    /// The unique identifier for the second term (logic&lt;=0 result) to use in a Branch term
    pub parameterterm2: Option<String>,
    /// The unique identifier for the third term (logic&gt;0 result) to use in a Branch term
    pub parameterterm3: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for GeqrhsNull1 {
    type PrimaryKey = GeqrhsNull1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "GEQRHS".into(),
            table_name: Some("NULL".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> GeqrhsNull1PrimaryKey {
        GeqrhsNull1PrimaryKey {
            effectivedate: self.effectivedate.clone(),
            equationid: self.equationid.clone(),
            termid: self.termid.clone(),
            versionno: self.versionno.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "geqrhs_null_v1".to_string()
    }
}
impl crate::CompareWithRow for GeqrhsNull1 {
    type Row = GeqrhsNull1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.equationid == row.equationid
            && self.termid == row.termid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for GeqrhsNull1 {
    type PrimaryKey = GeqrhsNull1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.equationid == key.equationid
            && self.termid == key.termid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GeqrhsNull1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub equationid: String,
    pub termid: rust_decimal::Decimal,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for GeqrhsNull1PrimaryKey {
    type Row = GeqrhsNull1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.equationid == row.equationid
            && self.termid == row.termid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for GeqrhsNull1PrimaryKey {
    type PrimaryKey = GeqrhsNull1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.equationid == key.equationid
            && self.termid == key.termid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for GeqrhsNull1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for GeqrhsNull1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "equationid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "termid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "groupid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new("spd_id", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("spd_type", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "factor",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "operation",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "defaultvalue",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new(
                "parameterterm1",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "parameterterm2",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "parameterterm3",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut equationid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut termid_array = Vec::new();
        let mut groupid_array = Vec::new();
        let mut spd_id_array = Vec::new();
        let mut spd_type_array = Vec::new();
        let mut factor_array = Vec::new();
        let mut operation_array = Vec::new();
        let mut defaultvalue_array = Vec::new();
        let mut parameterterm1_array = Vec::new();
        let mut parameterterm2_array = Vec::new();
        let mut parameterterm3_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            equationid_array.push(row.equationid);
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            termid_array.push({
                let mut val = row.termid;
                val.rescale(0);
                val.mantissa()
            });
            groupid_array.push({
                row.groupid.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            spd_id_array.push(row.spd_id);
            spd_type_array.push(row.spd_type);
            factor_array.push({
                row.factor.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            operation_array.push(row.operation);
            defaultvalue_array.push({
                row.defaultvalue.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            parameterterm1_array.push(row.parameterterm1);
            parameterterm2_array.push(row.parameterterm2);
            parameterterm3_array.push(row.parameterterm3);
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    equationid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(termid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(groupid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(spd_id_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(spd_type_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(factor_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(operation_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(defaultvalue_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(parameterterm1_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(parameterterm2_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(parameterterm3_array)),
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
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * CONNECTIONPOINTID
/// * EFFECTIVEDATE
/// * GENCONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SpdcpcNull2 {
    /// Connection Point Identifier
    pub connectionpointid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// Generic Constraint Identifier
    pub genconid: String,
    /// Constraint factor
    pub factor: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// Bid Type Identifier; one of (RAISE6SEC, RAISE60SEC, RAISE5MIN, LOWER6SEC, LOWER60SEC, LOWER5MIN, RAISEREG, LOWERREG)
    pub bidtype: String,
}
impl crate::GetTable for SpdcpcNull2 {
    type PrimaryKey = SpdcpcNull2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SPDCPC".into(),
            table_name: Some("NULL".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> SpdcpcNull2PrimaryKey {
        SpdcpcNull2PrimaryKey {
            bidtype: self.bidtype.clone(),
            connectionpointid: self.connectionpointid.clone(),
            effectivedate: self.effectivedate.clone(),
            genconid: self.genconid.clone(),
            versionno: self.versionno.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "spdcpc_null_v2".to_string()
    }
}
impl crate::CompareWithRow for SpdcpcNull2 {
    type Row = SpdcpcNull2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.connectionpointid == row.connectionpointid
            && self.effectivedate == row.effectivedate
            && self.genconid == row.genconid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SpdcpcNull2 {
    type PrimaryKey = SpdcpcNull2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.connectionpointid == key.connectionpointid
            && self.effectivedate == key.effectivedate
            && self.genconid == key.genconid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpdcpcNull2PrimaryKey {
    pub bidtype: String,
    pub connectionpointid: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub genconid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for SpdcpcNull2PrimaryKey {
    type Row = SpdcpcNull2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.connectionpointid == row.connectionpointid
            && self.effectivedate == row.effectivedate
            && self.genconid == row.genconid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SpdcpcNull2PrimaryKey {
    type PrimaryKey = SpdcpcNull2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.connectionpointid == key.connectionpointid
            && self.effectivedate == key.effectivedate
            && self.genconid == key.genconid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for SpdcpcNull2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SpdcpcNull2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "connectionpointid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "genconid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "factor",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("bidtype", arrow2::datatypes::DataType::LargeUtf8, false),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut connectionpointid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut genconid_array = Vec::new();
        let mut factor_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut bidtype_array = Vec::new();
        for (_, row) in partition {
            connectionpointid_array.push(row.connectionpointid);
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            genconid_array.push(row.genconid);
            factor_array.push({
                row.factor.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            bidtype_array.push(row.bidtype);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    connectionpointid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(genconid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(factor_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bidtype_array)),
            ],
        )
        .map_err(Into::into)
    }
}
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
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EFFECTIVEDATE
/// * GENCONID
/// * INTERCONNECTORID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SpdiccNull1 {
    /// Interconnector Identifier
    pub interconnectorid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// Generic Constraint Identifier
    pub genconid: String,
    /// Constraint factor
    pub factor: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for SpdiccNull1 {
    type PrimaryKey = SpdiccNull1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SPDICC".into(),
            table_name: Some("NULL".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> SpdiccNull1PrimaryKey {
        SpdiccNull1PrimaryKey {
            effectivedate: self.effectivedate.clone(),
            genconid: self.genconid.clone(),
            interconnectorid: self.interconnectorid.clone(),
            versionno: self.versionno.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "spdicc_null_v1".to_string()
    }
}
impl crate::CompareWithRow for SpdiccNull1 {
    type Row = SpdiccNull1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.genconid == row.genconid
            && self.interconnectorid == row.interconnectorid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SpdiccNull1 {
    type PrimaryKey = SpdiccNull1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.genconid == key.genconid
            && self.interconnectorid == key.interconnectorid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpdiccNull1PrimaryKey {
    pub effectivedate: chrono::NaiveDateTime,
    pub genconid: String,
    pub interconnectorid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for SpdiccNull1PrimaryKey {
    type Row = SpdiccNull1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.effectivedate == row.effectivedate
            && self.genconid == row.genconid
            && self.interconnectorid == row.interconnectorid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SpdiccNull1PrimaryKey {
    type PrimaryKey = SpdiccNull1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.effectivedate == key.effectivedate
            && self.genconid == key.genconid
            && self.interconnectorid == key.interconnectorid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for SpdiccNull1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SpdiccNull1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "interconnectorid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "genconid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "factor",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut interconnectorid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut genconid_array = Vec::new();
        let mut factor_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            interconnectorid_array.push(row.interconnectorid);
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            genconid_array.push(row.genconid);
            factor_array.push({
                row.factor.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    interconnectorid_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(genconid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(factor_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
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
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * BIDTYPE
/// * EFFECTIVEDATE
/// * GENCONID
/// * REGIONID
/// * VERSIONNO
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SpdrcNull2 {
    /// Region Identifier
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub effectivedate: chrono::NaiveDateTime,
    /// Version no of this record for the effective date
    pub versionno: rust_decimal::Decimal,
    /// Generic Constraint Identifier
    pub genconid: String,
    /// Constraint factor; one of (-1, 1)
    pub factor: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
    /// AS Service type - relates to the BidType table; one of (RAISE6SEC, RAISE60SEC, RAISE5MIN, LOWER6SEC, LOWER60SEC, LOWER5MIN, RAISEREG, LOWERREG)
    pub bidtype: String,
}
impl crate::GetTable for SpdrcNull2 {
    type PrimaryKey = SpdrcNull2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "SPDRC".into(),
            table_name: Some("NULL".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> SpdrcNull2PrimaryKey {
        SpdrcNull2PrimaryKey {
            bidtype: self.bidtype.clone(),
            effectivedate: self.effectivedate.clone(),
            genconid: self.genconid.clone(),
            regionid: self.regionid.clone(),
            versionno: self.versionno.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "spdrc_null_v2".to_string()
    }
}
impl crate::CompareWithRow for SpdrcNull2 {
    type Row = SpdrcNull2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.effectivedate == row.effectivedate
            && self.genconid == row.genconid
            && self.regionid == row.regionid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SpdrcNull2 {
    type PrimaryKey = SpdrcNull2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.effectivedate == key.effectivedate
            && self.genconid == key.genconid
            && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SpdrcNull2PrimaryKey {
    pub bidtype: String,
    pub effectivedate: chrono::NaiveDateTime,
    pub genconid: String,
    pub regionid: String,
    pub versionno: rust_decimal::Decimal,
}
impl crate::CompareWithRow for SpdrcNull2PrimaryKey {
    type Row = SpdrcNull2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.bidtype == row.bidtype
            && self.effectivedate == row.effectivedate
            && self.genconid == row.genconid
            && self.regionid == row.regionid
            && self.versionno == row.versionno
    }
}
impl crate::CompareWithPrimaryKey for SpdrcNull2PrimaryKey {
    type PrimaryKey = SpdrcNull2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.bidtype == key.bidtype
            && self.effectivedate == key.effectivedate
            && self.genconid == key.genconid
            && self.regionid == key.regionid
            && self.versionno == key.versionno
    }
}
impl crate::PrimaryKey for SpdrcNull2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for SpdrcNull2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "effectivedate",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "genconid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "factor",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new("bidtype", arrow2::datatypes::DataType::LargeUtf8, false),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut regionid_array = Vec::new();
        let mut effectivedate_array = Vec::new();
        let mut versionno_array = Vec::new();
        let mut genconid_array = Vec::new();
        let mut factor_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        let mut bidtype_array = Vec::new();
        for (_, row) in partition {
            regionid_array.push(row.regionid);
            effectivedate_array.push(
                i32::try_from(
                    (row.effectivedate.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            versionno_array.push({
                let mut val = row.versionno;
                val.rescale(0);
                val.mantissa()
            });
            genconid_array.push(row.genconid);
            factor_array.push({
                row.factor.map(|mut val| {
                    val.rescale(6);
                    val.mantissa()
                })
            });
            lastchanged_array.push(row.lastchanged.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            bidtype_array.push(row.bidtype);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(genconid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(factor_array)
                        .to(arrow2::datatypes::DataType::Decimal(16, 6)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(lastchanged_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(bidtype_array)),
            ],
        )
        .map_err(Into::into)
    }
}
