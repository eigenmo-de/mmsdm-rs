/// # Summary
///
/// ## VOLTAGE_INSTRUCTION
///  _Child record for Voltage Instructions (MVAr Dispatch)_
///
/// * Data Set Name: Voltage Instruction
/// * File Name: Instruction
/// * Data Version: 2
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * EMS_ID
/// * RUN_DATETIME
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct VoltageInstructionInstruction2 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// The unique identifier for reference within AEMO –matches equipment names between NOS and EMS
    pub ems_id: String,
    /// The NEM id of the participant who owns the equipment
    pub participantid: Option<String>,
    /// The id of the station where the control equipment resides
    pub station_id: Option<String>,
    /// The company/participant preferred name of an equipment
    pub device_id: Option<String>,
    /// One of REACTOR, CAPACITOR, GEN, SVC, TRANS or GRPGEN but may be extended to other types
    pub device_type: Option<String>,
    /// One of VOLTAGE, TAP, MVAR, SWITCH or COMMIT but may be extended to other types
    pub control_type: Option<String>,
    /// Instruction for the device, for this interval – null denotes no instruction
    pub target: Option<rust_decimal::Decimal>,
    /// [0,1] Denotes if the Device is currently conforming
    pub conforming: Option<rust_decimal::Decimal>,
    /// Verbose summary of instruction
    pub instruction_summary: Option<String>,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// Order for execution of Instruction
    pub instruction_sequence: Option<rust_decimal::Decimal>,
    /// Additional information pertaining to a particular instruction, e.g. Previously issued instruction revoked
    pub additional_notes: Option<String>,
}
impl crate::GetTable for VoltageInstructionInstruction2 {
    type PrimaryKey = VoltageInstructionInstruction2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "VOLTAGE_INSTRUCTION".into(),
            table_name: Some("INSTRUCTION".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> VoltageInstructionInstruction2PrimaryKey {
        VoltageInstructionInstruction2PrimaryKey {
            ems_id: self.ems_id.clone(),
            run_datetime: self.run_datetime.clone(),
            version_datetime: self.version_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "voltage_instruction_instruction_v2".to_string()
    }
}
impl crate::CompareWithRow for VoltageInstructionInstruction2 {
    type Row = VoltageInstructionInstruction2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.ems_id == row.ems_id
            && self.run_datetime == row.run_datetime
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for VoltageInstructionInstruction2 {
    type PrimaryKey = VoltageInstructionInstruction2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.ems_id == key.ems_id
            && self.run_datetime == key.run_datetime
            && self.version_datetime == key.version_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoltageInstructionInstruction2PrimaryKey {
    pub ems_id: String,
    pub run_datetime: chrono::NaiveDateTime,
    pub version_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for VoltageInstructionInstruction2PrimaryKey {
    type Row = VoltageInstructionInstruction2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.ems_id == row.ems_id
            && self.run_datetime == row.run_datetime
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for VoltageInstructionInstruction2PrimaryKey {
    type PrimaryKey = VoltageInstructionInstruction2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.ems_id == key.ems_id
            && self.run_datetime == key.run_datetime
            && self.version_datetime == key.version_datetime
    }
}
impl crate::PrimaryKey for VoltageInstructionInstruction2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for VoltageInstructionInstruction2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "run_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new("ems_id", arrow2::datatypes::DataType::LargeUtf8, false),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "station_id",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "device_id",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "device_type",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "control_type",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "target",
                arrow2::datatypes::DataType::Decimal(15, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "conforming",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "instruction_summary",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "version_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "instruction_sequence",
                arrow2::datatypes::DataType::Decimal(4, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "additional_notes",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut run_datetime_array = Vec::new();
        let mut ems_id_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut station_id_array = Vec::new();
        let mut device_id_array = Vec::new();
        let mut device_type_array = Vec::new();
        let mut control_type_array = Vec::new();
        let mut target_array = Vec::new();
        let mut conforming_array = Vec::new();
        let mut instruction_summary_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut instruction_sequence_array = Vec::new();
        let mut additional_notes_array = Vec::new();
        for (_, row) in partition {
            run_datetime_array.push(
                i32::try_from(
                    (row.run_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            ems_id_array.push(row.ems_id);
            participantid_array.push(row.participantid);
            station_id_array.push(row.station_id);
            device_id_array.push(row.device_id);
            device_type_array.push(row.device_type);
            control_type_array.push(row.control_type);
            target_array.push({
                row.target.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            conforming_array.push({
                row.conforming.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            instruction_summary_array.push(row.instruction_summary);
            version_datetime_array.push(
                i32::try_from(
                    (row.version_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            instruction_sequence_array.push({
                row.instruction_sequence.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            additional_notes_array.push(row.additional_notes);
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(run_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(ems_id_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(participantid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(station_id_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(device_id_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(device_type_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(control_type_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(target_array)
                        .to(arrow2::datatypes::DataType::Decimal(15, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(conforming_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    instruction_summary_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(version_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(instruction_sequence_array)
                        .to(arrow2::datatypes::DataType::Decimal(4, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    additional_notes_array,
                )),
            ],
        )
        .map_err(Into::into)
    }
}
/// # Summary
///
/// ## VOLTAGE_INSTRUCTION_TRK
///  _Parent record for Voltage Instructions (MVAr Dispatch). 'SIGNAL' records will have no children; 'INSTRUCTION' records will have children_
///
/// * Data Set Name: Voltage Instruction
/// * File Name: Track
/// * Data Version: 2
///
///
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * RUN_DATETIME
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct VoltageInstructionTrack2 {
    #[serde(with = "crate::mms_datetime")]
    pub run_datetime: chrono::NaiveDateTime,
    /// Either 'SIGNAL' (childless) or 'INSTRUCTION'
    pub file_type: Option<String>,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    pub se_datetime: Option<chrono::NaiveDateTime>,
    /// VDS solver solution category. Valid values: SUCCESS, WARNING, FAILURE
    pub solution_category: Option<String>,
    /// VDS solver solution status. Valid values: NOACTCNV [Solved with no instructions], NOVIOACT, CONVERGE, UNMANAGE, UNMANCTG, CTGDIV, SENHDIV [Failed with too many violations], BCDIV
    pub solution_status: Option<String>,
    /// The current VDS operating mode. Valid values: AUTO, AUTO-VERFIED, MANUAL
    pub operating_mode: Option<String>,
    /// Unstructured code and message from AEMO
    pub operating_status: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub est_expiry: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub est_next_instruction: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for VoltageInstructionTrack2 {
    type PrimaryKey = VoltageInstructionTrack2PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "VOLTAGE_INSTRUCTION".into(),
            table_name: Some("TRACK".into()),
            version: 2,
        }
    }

    fn primary_key(&self) -> VoltageInstructionTrack2PrimaryKey {
        VoltageInstructionTrack2PrimaryKey {
            run_datetime: self.run_datetime.clone(),
            version_datetime: self.version_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "voltage_instruction_track_v2".to_string()
    }
}
impl crate::CompareWithRow for VoltageInstructionTrack2 {
    type Row = VoltageInstructionTrack2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.run_datetime == row.run_datetime && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for VoltageInstructionTrack2 {
    type PrimaryKey = VoltageInstructionTrack2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime && self.version_datetime == key.version_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VoltageInstructionTrack2PrimaryKey {
    pub run_datetime: chrono::NaiveDateTime,
    pub version_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for VoltageInstructionTrack2PrimaryKey {
    type Row = VoltageInstructionTrack2;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.run_datetime == row.run_datetime && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for VoltageInstructionTrack2PrimaryKey {
    type PrimaryKey = VoltageInstructionTrack2PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.run_datetime == key.run_datetime && self.version_datetime == key.version_datetime
    }
}
impl crate::PrimaryKey for VoltageInstructionTrack2PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for VoltageInstructionTrack2 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new(
                "run_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "file_type",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "version_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new("se_datetime", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "solution_category",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "solution_status",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "operating_mode",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "operating_status",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("est_expiry", arrow2::datatypes::DataType::Date32, true),
            arrow2::datatypes::Field::new(
                "est_next_instruction",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut run_datetime_array = Vec::new();
        let mut file_type_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut se_datetime_array = Vec::new();
        let mut solution_category_array = Vec::new();
        let mut solution_status_array = Vec::new();
        let mut operating_mode_array = Vec::new();
        let mut operating_status_array = Vec::new();
        let mut est_expiry_array = Vec::new();
        let mut est_next_instruction_array = Vec::new();
        for (_, row) in partition {
            run_datetime_array.push(
                i32::try_from(
                    (row.run_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            file_type_array.push(row.file_type);
            version_datetime_array.push(
                i32::try_from(
                    (row.version_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            se_datetime_array.push(row.se_datetime.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            solution_category_array.push(row.solution_category);
            solution_status_array.push(row.solution_status);
            operating_mode_array.push(row.operating_mode);
            operating_status_array.push(row.operating_status);
            est_expiry_array.push(row.est_expiry.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            est_next_instruction_array.push(row.est_next_instruction.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(run_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(file_type_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(version_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(se_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    solution_category_array,
                )),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(solution_status_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(operating_mode_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    operating_status_array,
                )),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(est_expiry_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(est_next_instruction_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
            ],
        )
        .map_err(Into::into)
    }
}
