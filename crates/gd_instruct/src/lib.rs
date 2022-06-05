/// # Summary
///
/// ## GDINSTRUCT
///  _GDINSTRUCT shows all manually issued dispatch instructions for a dispatchable unit. Ancillary Service instructions are to enable and to disable (i.e. 2 separate instructions) a service. Non-conforming units are also instructed via this facility. However, this facility is not the same as the market notice._
///
/// * Data Set Name: Gd Instruct
/// * File Name: Gdinstruct
/// * Data Version: 1
///
/// # Description
///  Source GDINSTRUCT updates on issue of an instruction by AEMO, with visibility restricted on the day of issue to the relevant participant. All participants have previous days' data available.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * ID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GdInstructGdinstruct1 {
    /// Dispatchable unit identifier
    pub duid: Option<String>,
    /// Station Identifier
    pub stationid: Option<String>,
    /// Region Identifier
    pub regionid: Option<String>,
    /// Instruction ID (sequential number)
    pub id: rust_decimal::Decimal,
    /// Instruction type
    pub instructiontypeid: Option<String>,
    /// Instruction sub type
    pub instructionsubtypeid: Option<String>,
    /// Instruction class
    pub instructionclassid: Option<String>,
    /// Reason
    pub reason: Option<String>,
    /// Instruction target level
    pub instlevel: Option<rust_decimal::Decimal>,
    /// Authorised date
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// User authorised by
    pub authorisedby: Option<String>,
    /// Unique participant identifier
    pub participantid: Option<String>,
    /// Date / time issued
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub issuedtime: Option<chrono::NaiveDateTime>,
    /// Date / time instruction to apply
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub targettime: Option<chrono::NaiveDateTime>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for GdInstructGdinstruct1 {
    type PrimaryKey = GdInstructGdinstruct1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "GD_INSTRUCT".into(),
            table_name: Some("GDINSTRUCT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> GdInstructGdinstruct1PrimaryKey {
        GdInstructGdinstruct1PrimaryKey { id: self.id }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "gd_instruct_gdinstruct_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct GdInstructGdinstruct1PrimaryKey {
    pub id: rust_decimal::Decimal,
}
impl mmsdm_core::PrimaryKey for GdInstructGdinstruct1PrimaryKey {}
impl mmsdm_core::CompareWithRow for GdInstructGdinstruct1 {
    type Row = GdInstructGdinstruct1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.id == row.id
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GdInstructGdinstruct1 {
    type PrimaryKey = GdInstructGdinstruct1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.id == key.id
    }
}
impl mmsdm_core::CompareWithRow for GdInstructGdinstruct1PrimaryKey {
    type Row = GdInstructGdinstruct1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.id == row.id
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GdInstructGdinstruct1PrimaryKey {
    type PrimaryKey = GdInstructGdinstruct1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.id == key.id
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for GdInstructGdinstruct1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "stationid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new("id", arrow2::datatypes::DataType::Decimal(22, 0), false),
            arrow2::datatypes::Field::new(
                "instructiontypeid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "instructionsubtypeid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "instructionclassid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("reason", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "instlevel",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "authoriseddate",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "authorisedby",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "participantid",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "issuedtime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
                true,
            ),
            arrow2::datatypes::Field::new(
                "targettime",
                arrow2::datatypes::DataType::Timestamp(arrow2::datatypes::TimeUnit::Second, None),
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
        let mut duid_array = Vec::new();
        let mut stationid_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut id_array = Vec::new();
        let mut instructiontypeid_array = Vec::new();
        let mut instructionsubtypeid_array = Vec::new();
        let mut instructionclassid_array = Vec::new();
        let mut reason_array = Vec::new();
        let mut instlevel_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut participantid_array = Vec::new();
        let mut issuedtime_array = Vec::new();
        let mut targettime_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            duid_array.push(row.duid);
            stationid_array.push(row.stationid);
            regionid_array.push(row.regionid);
            id_array.push({
                let mut val = row.id;
                val.rescale(0);
                val.mantissa()
            });
            instructiontypeid_array.push(row.instructiontypeid);
            instructionsubtypeid_array.push(row.instructionsubtypeid);
            instructionclassid_array.push(row.instructionclassid);
            reason_array.push(row.reason);
            instlevel_array.push({
                row.instlevel.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            authoriseddate_array.push(row.authoriseddate.map(|val| val.timestamp()));
            authorisedby_array.push(row.authorisedby);
            participantid_array.push(row.participantid);
            issuedtime_array.push(row.issuedtime.map(|val| val.timestamp()));
            targettime_array.push(row.targettime.map(|val| val.timestamp()));
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(duid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(stationid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(regionid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_vec(id_array)
                        .to(arrow2::datatypes::DataType::Decimal(22, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    instructiontypeid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    instructionsubtypeid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(
                    instructionclassid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(reason_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(instlevel_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array).to(
                        arrow2::datatypes::DataType::Timestamp(
                            arrow2::datatypes::TimeUnit::Second,
                            None,
                        ),
                    ),
                ) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(participantid_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(issuedtime_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::PrimitiveArray::from(targettime_array).to(
                    arrow2::datatypes::DataType::Timestamp(
                        arrow2::datatypes::TimeUnit::Second,
                        None,
                    ),
                )) as std::sync::Arc<dyn arrow2::array::Array>,
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
/// ## INSTRUCTIONSUBTYPE
///  _Each Dispatch instruction (GD instruct) has a type and subtype. INSTRUCTIONSUBTYPE, together with INSTRUCTIONTYPE, sets out valid instruction types._
///
/// * Data Set Name: Gd Instruct
/// * File Name: Instructionsubtype
/// * Data Version: 1
///
/// # Description
///  INSTRUCTIONSUBTYPE is public data, and is available to all participants. Source INSTRUCTIONSUBTYPE shows ad hoc updates to market configuration.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INSTRUCTIONSUBTYPEID
/// * INSTRUCTIONTYPEID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GdInstructInstructionsubtype1 {
    /// Instruction type
    pub instructiontypeid: String,
    /// Subtype for each dispatch instruction type, for example governor off.
    pub instructionsubtypeid: String,
    /// Description of instruction subtype
    pub description: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for GdInstructInstructionsubtype1 {
    type PrimaryKey = GdInstructInstructionsubtype1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "GD_INSTRUCT".into(),
            table_name: Some("INSTRUCTIONSUBTYPE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> GdInstructInstructionsubtype1PrimaryKey {
        GdInstructInstructionsubtype1PrimaryKey {
            instructionsubtypeid: self.instructionsubtypeid.clone(),
            instructiontypeid: self.instructiontypeid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "gd_instruct_instructionsubtype_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct GdInstructInstructionsubtype1PrimaryKey {
    pub instructionsubtypeid: String,
    pub instructiontypeid: String,
}
impl mmsdm_core::PrimaryKey for GdInstructInstructionsubtype1PrimaryKey {}
impl mmsdm_core::CompareWithRow for GdInstructInstructionsubtype1 {
    type Row = GdInstructInstructionsubtype1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.instructionsubtypeid == row.instructionsubtypeid
            && self.instructiontypeid == row.instructiontypeid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GdInstructInstructionsubtype1 {
    type PrimaryKey = GdInstructInstructionsubtype1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.instructionsubtypeid == key.instructionsubtypeid
            && self.instructiontypeid == key.instructiontypeid
    }
}
impl mmsdm_core::CompareWithRow for GdInstructInstructionsubtype1PrimaryKey {
    type Row = GdInstructInstructionsubtype1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.instructionsubtypeid == row.instructionsubtypeid
            && self.instructiontypeid == row.instructiontypeid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GdInstructInstructionsubtype1PrimaryKey {
    type PrimaryKey = GdInstructInstructionsubtype1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.instructionsubtypeid == key.instructionsubtypeid
            && self.instructiontypeid == key.instructiontypeid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for GdInstructInstructionsubtype1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "instructiontypeid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "instructionsubtypeid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
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
        let mut instructiontypeid_array = Vec::new();
        let mut instructionsubtypeid_array = Vec::new();
        let mut description_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            instructiontypeid_array.push(row.instructiontypeid);
            instructionsubtypeid_array.push(row.instructionsubtypeid);
            description_array.push(row.description);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    instructiontypeid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    instructionsubtypeid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array))
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
/// ## INSTRUCTIONTYPE
///  _Dispatch instruction (GD instruct) has types and subtypes. INSTRUCTIONTYPE, together with INSTRUCTIONSUBTYPE, sets out valid instruction types._
///
/// * Data Set Name: Gd Instruct
/// * File Name: Instructiontype
/// * Data Version: 1
///
/// # Description
///  INSTRUCTIONTYPE data is public to all participants. Source INSTRUCTIONTYPE shows ad hoc updates to market configuration.
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * INSTRUCTIONTYPEID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct GdInstructInstructiontype1 {
    /// Dispatch instruction type for example FCAS service.
    pub instructiontypeid: String,
    /// Description of instruction type
    pub description: Option<String>,
    /// Region id if regional instruction only.
    pub regionid: Option<String>,
    /// Last date and time record changed
    #[serde(with = "mmsdm_core::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl mmsdm_core::GetTable for GdInstructInstructiontype1 {
    type PrimaryKey = GdInstructInstructiontype1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> mmsdm_core::FileKey {
        mmsdm_core::FileKey {
            data_set_name: "GD_INSTRUCT".into(),
            table_name: Some("INSTRUCTIONTYPE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> GdInstructInstructiontype1PrimaryKey {
        GdInstructInstructiontype1PrimaryKey {
            instructiontypeid: self.instructiontypeid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {}

    fn partition_name(&self) -> String {
        "gd_instruct_instructiontype_v1".to_string()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, serde::Serialize, Ord)]
pub struct GdInstructInstructiontype1PrimaryKey {
    pub instructiontypeid: String,
}
impl mmsdm_core::PrimaryKey for GdInstructInstructiontype1PrimaryKey {}
impl mmsdm_core::CompareWithRow for GdInstructInstructiontype1 {
    type Row = GdInstructInstructiontype1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.instructiontypeid == row.instructiontypeid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GdInstructInstructiontype1 {
    type PrimaryKey = GdInstructInstructiontype1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.instructiontypeid == key.instructiontypeid
    }
}
impl mmsdm_core::CompareWithRow for GdInstructInstructiontype1PrimaryKey {
    type Row = GdInstructInstructiontype1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.instructiontypeid == row.instructiontypeid
    }
}
impl mmsdm_core::CompareWithPrimaryKey for GdInstructInstructiontype1PrimaryKey {
    type PrimaryKey = GdInstructInstructiontype1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.instructiontypeid == key.instructiontypeid
    }
}
#[cfg(feature = "arrow")]
impl mmsdm_core::ArrowSchema for GdInstructInstructiontype1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::from(vec![
            arrow2::datatypes::Field::new(
                "instructiontypeid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new("regionid", arrow2::datatypes::DataType::LargeUtf8, true),
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
        let mut instructiontypeid_array = Vec::new();
        let mut description_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for row in partition {
            instructiontypeid_array.push(row.instructiontypeid);
            description_array.push(row.description);
            regionid_array.push(row.regionid);
            lastchanged_array.push(row.lastchanged.map(|val| val.timestamp()));
        }

        arrow2::chunk::Chunk::try_new(
            //std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(
                    instructiontypeid_array,
                )) as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array))
                    as std::sync::Arc<dyn arrow2::array::Array>,
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(regionid_array))
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
        (Some("GDINSTRUCT"), version) if version <= 1_i32 => {
            let d: Vec<GdInstructGdinstruct1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertGdInstructGdinstruct1 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("INSTRUCTIONSUBTYPE"), version) if version <= 1_i32 => {
            let d: Vec<GdInstructInstructionsubtype1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertGdInstructInstructionsubtype1 @P1, @P2",
                chunk_size,
            )
            .await?;
        }
        (Some("INSTRUCTIONTYPE"), version) if version <= 1_i32 => {
            let d: Vec<GdInstructInstructiontype1> = mms_file.get_table()?;
            mmsdm_core::sql_server::batched_insert(
                client,
                file_key,
                mms_file.header(),
                &d,
                "exec mmsdm_proc.InsertGdInstructInstructiontype1 @P1, @P2",
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
