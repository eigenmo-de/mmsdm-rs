/// # Summary
///
/// ## MR_DAYOFFER_STACK
///  _MR_DAYOFFER_STACK defines the Stack order for each version of the Acceptance Schedule, including all units submitting MR offers for that event. MR_DAYOFFER_STACK is the child to MR_EVENT_SCHEDULE, and parent to MR_PEROFFER_STACK._
///
/// * Data Set Name: Mr
/// * File Name: Dayoffer Stack
/// * Data Version: 1
///
/// # Description
///  Once the offer cut off time has passed and as the schedule changes AEMO is obliged to accept MR capacity to meet the schedule in merit order according to the offers submitted. The relationship to a specific schedule, the merit order of submitted offers and accepted quantities for each trading interval are stored in the MR_EVENT_SCHEDULE, MR_DAYOFFER_STACK and MR_PEROFFER_STACK. MR_DAYOFFER_STACK sets includes all generators/MNSPs in the region that submitted an MR offer and a primary key reference to the Offer tables to identify the specific offer used for that unit. MR_DAYOFFER_STACK also includes a Stack Order, irrespective of whether the unit is required to meet the Schedule. MR_DAYOFFER_STACK updates are confidential on day of submission, with public exposure the next day. Source MR_DAYOFFER_STACK updates are ad hoc. Volume 100 rows per year
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * MR_DATE
/// * REGIONID
/// * STACK_POSITION
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrDayofferStack1 {
    #[serde(with = "crate::mms_datetime")]
    pub mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// Loss Adjusted Offer Factor Stack order starting at 1
    pub stack_position: rust_decimal::Decimal,
    /// Dispatchable Unit ID or LinkID
    pub duid: Option<String>,
    /// Confirms the unit is allowed to Contribute MR Capacity
    pub authorised: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub offer_settlementdate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub offer_offerdate: Option<chrono::NaiveDateTime>,
    /// Foreign key reference to XXXX_DayOffer.VersionNo
    pub offer_versionno: Option<rust_decimal::Decimal>,
    /// Source tables - ENERGY or MNSP
    pub offer_type: Option<String>,
    /// Loss Adjusted Offer Factor = TLF times MR_Factor
    pub laof: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MrDayofferStack1 {
    type PrimaryKey = MrDayofferStack1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MR".into(),
            table_name: Some("DAYOFFER_STACK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> MrDayofferStack1PrimaryKey {
        MrDayofferStack1PrimaryKey {
            mr_date: self.mr_date.clone(),
            regionid: self.regionid.clone(),
            stack_position: self.stack_position.clone(),
            version_datetime: self.version_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "mr_dayoffer_stack_v1".to_string()
    }
}
impl crate::CompareWithRow for MrDayofferStack1 {
    type Row = MrDayofferStack1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mr_date == row.mr_date
            && self.regionid == row.regionid
            && self.stack_position == row.stack_position
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for MrDayofferStack1 {
    type PrimaryKey = MrDayofferStack1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date
            && self.regionid == key.regionid
            && self.stack_position == key.stack_position
            && self.version_datetime == key.version_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MrDayofferStack1PrimaryKey {
    pub mr_date: chrono::NaiveDateTime,
    pub regionid: String,
    pub stack_position: rust_decimal::Decimal,
    pub version_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for MrDayofferStack1PrimaryKey {
    type Row = MrDayofferStack1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mr_date == row.mr_date
            && self.regionid == row.regionid
            && self.stack_position == row.stack_position
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for MrDayofferStack1PrimaryKey {
    type PrimaryKey = MrDayofferStack1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date
            && self.regionid == key.regionid
            && self.stack_position == key.stack_position
            && self.version_datetime == key.version_datetime
    }
}
impl crate::PrimaryKey for MrDayofferStack1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for MrDayofferStack1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("mr_date", arrow2::datatypes::DataType::Date32, false),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "version_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "stack_position",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "authorised",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "offer_settlementdate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "offer_offerdate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "offer_versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "offer_type",
                arrow2::datatypes::DataType::LargeUtf8,
                true,
            ),
            arrow2::datatypes::Field::new(
                "laof",
                arrow2::datatypes::DataType::Decimal(16, 6),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut mr_date_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut stack_position_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut authorised_array = Vec::new();
        let mut offer_settlementdate_array = Vec::new();
        let mut offer_offerdate_array = Vec::new();
        let mut offer_versionno_array = Vec::new();
        let mut offer_type_array = Vec::new();
        let mut laof_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            mr_date_array.push(
                i32::try_from(
                    (row.mr_date.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            regionid_array.push(row.regionid);
            version_datetime_array.push(
                i32::try_from(
                    (row.version_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            stack_position_array.push({
                let mut val = row.stack_position;
                val.rescale(0);
                val.mantissa()
            });
            duid_array.push(row.duid);
            authorised_array.push({
                row.authorised.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            offer_settlementdate_array.push(row.offer_settlementdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            offer_offerdate_array.push(row.offer_offerdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            offer_versionno_array.push({
                row.offer_versionno.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            offer_type_array.push(row.offer_type);
            laof_array.push({
                row.laof.map(|mut val| {
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
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(mr_date_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(version_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(stack_position_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authorised_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(offer_settlementdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(offer_offerdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(offer_versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(offer_type_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(laof_array)
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
/// ## MR_EVENT
///  _MR_EVENT defines an MR Event for a given region on a specific trading date.<br>_
///
/// * Data Set Name: Mr
/// * File Name: Event
/// * Data Version: 1
///
/// # Description
///  MR_EVENT defines a mandatory restriction event for a given region and trading date (04:30 to 04:00). Data within MR_EVENT includes the cut-off time for submission of MR offers for this event and a notification that the settlements figures are locked due to results from an independent expert being engaged to allocate settlement of a significant shortfall. If mandatory restrictions are defined in two regions on the same trading day, two MR events are defined. MR_EVENT data is public, so is available to all participants. Source MR_EVENT updates are ad hoc. Volume 1 Row per year
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * MR_DATE
/// * REGIONID
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrEvent1 {
    #[serde(with = "crate::mms_datetime")]
    pub mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: String,
    /// Description of MR
    pub description: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    /// Ignored - Tracking purpose only
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub offer_cut_off_time: Option<chrono::NaiveDateTime>,
    /// Flag:1 = MR settlement figures locked. Do not recalculate, ·&nbsp;&nbsp;&nbsp;0 = MR settlements to be recalculated
    pub settlement_complete: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MrEvent1 {
    type PrimaryKey = MrEvent1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MR".into(),
            table_name: Some("EVENT".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> MrEvent1PrimaryKey {
        MrEvent1PrimaryKey {
            mr_date: self.mr_date.clone(),
            regionid: self.regionid.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "mr_event_v1".to_string()
    }
}
impl crate::CompareWithRow for MrEvent1 {
    type Row = MrEvent1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mr_date == row.mr_date && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for MrEvent1 {
    type PrimaryKey = MrEvent1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date && self.regionid == key.regionid
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MrEvent1PrimaryKey {
    pub mr_date: chrono::NaiveDateTime,
    pub regionid: String,
}
impl crate::CompareWithRow for MrEvent1PrimaryKey {
    type Row = MrEvent1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mr_date == row.mr_date && self.regionid == row.regionid
    }
}
impl crate::CompareWithPrimaryKey for MrEvent1PrimaryKey {
    type PrimaryKey = MrEvent1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date && self.regionid == key.regionid
    }
}
impl crate::PrimaryKey for MrEvent1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for MrEvent1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("mr_date", arrow2::datatypes::DataType::Date32, false),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "description",
                arrow2::datatypes::DataType::LargeUtf8,
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
                "offer_cut_off_time",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "settlement_complete",
                arrow2::datatypes::DataType::Decimal(1, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut mr_date_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut description_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut offer_cut_off_time_array = Vec::new();
        let mut settlement_complete_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            mr_date_array.push(
                i32::try_from(
                    (row.mr_date.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            regionid_array.push(row.regionid);
            description_array.push(row.description);
            authoriseddate_array.push(row.authoriseddate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            authorisedby_array.push(row.authorisedby);
            offer_cut_off_time_array.push(row.offer_cut_off_time.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            settlement_complete_array.push({
                row.settlement_complete.map(|mut val| {
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
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(mr_date_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(description_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(authoriseddate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(authorisedby_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(offer_cut_off_time_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(settlement_complete_array)
                        .to(arrow2::datatypes::DataType::Decimal(1, 0)),
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
/// ## MR_EVENT_SCHEDULE
///  _MR_EVENT_SCHEDULE defines the Stack version of the Acceptance Schedule and is the parent table to MR_DayOffer_Stack and MR_PerOffer_Stack.<br>_
///
/// * Data Set Name: Mr
/// * File Name: Event Schedule
/// * Data Version: 1
///
/// # Description
///  Once the offer cut off time has passed and as the schedule changes AEMO is obliged to accept MR capacity to meet the schedule in merit order according to the offers submitted. The relationship to a specific schedule, the merit order of submitted offers and accepted quantities for each trading interval are stored in the MR_Event_Schedule, MR_DayOffer_Stack and MR_PerOffer_Stack table. The MR_EVENT_SCHEDULE table determines the existence of an MR offer acceptance stack for a specific MR schedule of an MR event. The MR_EVENT_SCHEDULE table also tracks the time each stack is exercised. MR_EVENT_SCHEDULE is public and notifies the market that a new offer stack has been created. Source MR_EVENT_SCHEDULE updates are ad hoc. Volume 2 Rows per year
///
/// # Notes
///  * (Visibility) Data in this table is: Public
///
/// # Primary Key Columns
///
/// * MR_DATE
/// * REGIONID
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrEventSchedule1 {
    #[serde(with = "crate::mms_datetime")]
    pub mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    #[serde(with = "crate::mms_datetime_opt")]
    pub demand_effectivedate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub demand_offerdate: Option<chrono::NaiveDateTime>,
    /// Foreign key reference to ResDemandTrk.VersionNo
    pub demand_versionno: Option<rust_decimal::Decimal>,
    /// Authorised person confirming Offer Stack (AKA Acceptance)
    pub authorisedby: Option<String>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub authoriseddate: Option<chrono::NaiveDateTime>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MrEventSchedule1 {
    type PrimaryKey = MrEventSchedule1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MR".into(),
            table_name: Some("EVENT_SCHEDULE".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> MrEventSchedule1PrimaryKey {
        MrEventSchedule1PrimaryKey {
            mr_date: self.mr_date.clone(),
            regionid: self.regionid.clone(),
            version_datetime: self.version_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "mr_event_schedule_v1".to_string()
    }
}
impl crate::CompareWithRow for MrEventSchedule1 {
    type Row = MrEventSchedule1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mr_date == row.mr_date
            && self.regionid == row.regionid
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for MrEventSchedule1 {
    type PrimaryKey = MrEventSchedule1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date
            && self.regionid == key.regionid
            && self.version_datetime == key.version_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MrEventSchedule1PrimaryKey {
    pub mr_date: chrono::NaiveDateTime,
    pub regionid: String,
    pub version_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for MrEventSchedule1PrimaryKey {
    type Row = MrEventSchedule1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mr_date == row.mr_date
            && self.regionid == row.regionid
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for MrEventSchedule1PrimaryKey {
    type PrimaryKey = MrEventSchedule1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date
            && self.regionid == key.regionid
            && self.version_datetime == key.version_datetime
    }
}
impl crate::PrimaryKey for MrEventSchedule1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for MrEventSchedule1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("mr_date", arrow2::datatypes::DataType::Date32, false),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "version_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "demand_effectivedate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "demand_offerdate",
                arrow2::datatypes::DataType::Date32,
                true,
            ),
            arrow2::datatypes::Field::new(
                "demand_versionno",
                arrow2::datatypes::DataType::Decimal(3, 0),
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
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut mr_date_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut demand_effectivedate_array = Vec::new();
        let mut demand_offerdate_array = Vec::new();
        let mut demand_versionno_array = Vec::new();
        let mut authorisedby_array = Vec::new();
        let mut authoriseddate_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            mr_date_array.push(
                i32::try_from(
                    (row.mr_date.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            regionid_array.push(row.regionid);
            version_datetime_array.push(
                i32::try_from(
                    (row.version_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            demand_effectivedate_array.push(row.demand_effectivedate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            demand_offerdate_array.push(row.demand_offerdate.map(|val| {
                i32::try_from((val.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days())
                    .unwrap()
            }));
            demand_versionno_array.push({
                row.demand_versionno.map(|mut val| {
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
        }

        arrow2::record_batch::RecordBatch::try_new(
            std::sync::Arc::new(Self::arrow_schema()),
            vec![
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(mr_date_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(version_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(demand_effectivedate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(demand_offerdate_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(demand_versionno_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
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
/// ## MR_PEROFFER_STACK
///  _MR_PEROFFER_STACK defines the accepted capacity on a period basis for the Acceptance Schedule, is a child table to MR_DayOffer_Stack and only includes records or units with accepted_capacity &gt; 0 for the specific period.<br>_
///
/// * Data Set Name: Mr
/// * File Name: Peroffer Stack
/// * Data Version: 1
///
/// # Description
///  Once the offer cut off time has passed and as the schedule changes AEMO is obliged to accept MR capacity to meet the schedule in merit order according to the offers submitted. The relationship to a specific schedule, the merit order of submitted offers and accepted quantities for each trading interval are stored in MR_Event_Schedule, MR_DayOffer_Stack and MR_PerOffer_Stack. MR_PEROFFER_STACK reports the accepted MR capacity (Accepted_Capacity) required from each unit for each trading interval. MR_PEROFFER_STACK is sparse so lists only units with accepted capacity &gt; 0 for that trading interval.  The Deducted_Capacity field allows the tracking and implementation of participant requested reductions to accepted MR capacity to be tracked and applied. MR_PEROFFER_STACK is reported confidentially to each participant to notify acceptance of an MR offer. Source MR_PEROFFER_STACK updates are ad hoc. Volume 4800 rows per year
///
/// # Notes
///  * (Visibility) Data in this table is: Private; Public Next-Day
///
/// # Primary Key Columns
///
/// * MR_DATE
/// * PERIODID
/// * REGIONID
/// * STACK_POSITION
/// * VERSION_DATETIME
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct MrPerofferStack1 {
    #[serde(with = "crate::mms_datetime")]
    pub mr_date: chrono::NaiveDateTime,
    /// Unique RegionID
    pub regionid: String,
    #[serde(with = "crate::mms_datetime")]
    pub version_datetime: chrono::NaiveDateTime,
    /// LAOF Stack order
    pub stack_position: rust_decimal::Decimal,
    /// Trade Period for the MR Offer
    pub periodid: rust_decimal::Decimal,
    /// Dispatchable Unit ID or LinkID. Only required here for CSV reports
    pub duid: Option<String>,
    /// MR Capacity to be Dispatched
    pub accepted_capacity: Option<rust_decimal::Decimal>,
    /// Requested capacity reduction amount
    pub deducted_capacity: Option<rust_decimal::Decimal>,
    #[serde(with = "crate::mms_datetime_opt")]
    pub lastchanged: Option<chrono::NaiveDateTime>,
}
impl crate::GetTable for MrPerofferStack1 {
    type PrimaryKey = MrPerofferStack1PrimaryKey;
    type Partition = ();

    fn get_file_key() -> crate::FileKey {
        crate::FileKey {
            data_set_name: "MR".into(),
            table_name: Some("PEROFFER_STACK".into()),
            version: 1,
        }
    }

    fn primary_key(&self) -> MrPerofferStack1PrimaryKey {
        MrPerofferStack1PrimaryKey {
            mr_date: self.mr_date.clone(),
            periodid: self.periodid.clone(),
            regionid: self.regionid.clone(),
            stack_position: self.stack_position.clone(),
            version_datetime: self.version_datetime.clone(),
        }
    }

    fn partition_suffix(&self) -> Self::Partition {
        ()
    }

    fn partition_name(&self) -> String {
        "mr_peroffer_stack_v1".to_string()
    }
}
impl crate::CompareWithRow for MrPerofferStack1 {
    type Row = MrPerofferStack1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mr_date == row.mr_date
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.stack_position == row.stack_position
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for MrPerofferStack1 {
    type PrimaryKey = MrPerofferStack1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.stack_position == key.stack_position
            && self.version_datetime == key.version_datetime
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MrPerofferStack1PrimaryKey {
    pub mr_date: chrono::NaiveDateTime,
    pub periodid: rust_decimal::Decimal,
    pub regionid: String,
    pub stack_position: rust_decimal::Decimal,
    pub version_datetime: chrono::NaiveDateTime,
}
impl crate::CompareWithRow for MrPerofferStack1PrimaryKey {
    type Row = MrPerofferStack1;

    fn compare_with_row(&self, row: &Self::Row) -> bool {
        self.mr_date == row.mr_date
            && self.periodid == row.periodid
            && self.regionid == row.regionid
            && self.stack_position == row.stack_position
            && self.version_datetime == row.version_datetime
    }
}
impl crate::CompareWithPrimaryKey for MrPerofferStack1PrimaryKey {
    type PrimaryKey = MrPerofferStack1PrimaryKey;

    fn compare_with_key(&self, key: &Self::PrimaryKey) -> bool {
        self.mr_date == key.mr_date
            && self.periodid == key.periodid
            && self.regionid == key.regionid
            && self.stack_position == key.stack_position
            && self.version_datetime == key.version_datetime
    }
}
impl crate::PrimaryKey for MrPerofferStack1PrimaryKey {}
#[cfg(feature = "save_as_parquet")]
impl crate::ArrowSchema for MrPerofferStack1 {
    fn arrow_schema() -> arrow2::datatypes::Schema {
        arrow2::datatypes::Schema::new(vec![
            arrow2::datatypes::Field::new("mr_date", arrow2::datatypes::DataType::Date32, false),
            arrow2::datatypes::Field::new(
                "regionid",
                arrow2::datatypes::DataType::LargeUtf8,
                false,
            ),
            arrow2::datatypes::Field::new(
                "version_datetime",
                arrow2::datatypes::DataType::Date32,
                false,
            ),
            arrow2::datatypes::Field::new(
                "stack_position",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new(
                "periodid",
                arrow2::datatypes::DataType::Decimal(3, 0),
                false,
            ),
            arrow2::datatypes::Field::new("duid", arrow2::datatypes::DataType::LargeUtf8, true),
            arrow2::datatypes::Field::new(
                "accepted_capacity",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new(
                "deducted_capacity",
                arrow2::datatypes::DataType::Decimal(6, 0),
                true,
            ),
            arrow2::datatypes::Field::new("lastchanged", arrow2::datatypes::DataType::Date32, true),
        ])
    }

    fn partition_to_record_batch(
        partition: std::collections::BTreeMap<<Self as crate::GetTable>::PrimaryKey, Self>,
    ) -> crate::Result<arrow2::record_batch::RecordBatch> {
        let mut mr_date_array = Vec::new();
        let mut regionid_array = Vec::new();
        let mut version_datetime_array = Vec::new();
        let mut stack_position_array = Vec::new();
        let mut periodid_array = Vec::new();
        let mut duid_array = Vec::new();
        let mut accepted_capacity_array = Vec::new();
        let mut deducted_capacity_array = Vec::new();
        let mut lastchanged_array = Vec::new();
        for (_, row) in partition {
            mr_date_array.push(
                i32::try_from(
                    (row.mr_date.date() - chrono::NaiveDate::from_ymd(1970, 1, 1)).num_days(),
                )
                .unwrap(),
            );
            regionid_array.push(row.regionid);
            version_datetime_array.push(
                i32::try_from(
                    (row.version_datetime.date() - chrono::NaiveDate::from_ymd(1970, 1, 1))
                        .num_days(),
                )
                .unwrap(),
            );
            stack_position_array.push({
                let mut val = row.stack_position;
                val.rescale(0);
                val.mantissa()
            });
            periodid_array.push({
                let mut val = row.periodid;
                val.rescale(0);
                val.mantissa()
            });
            duid_array.push(row.duid);
            accepted_capacity_array.push({
                row.accepted_capacity.map(|mut val| {
                    val.rescale(0);
                    val.mantissa()
                })
            });
            deducted_capacity_array.push({
                row.deducted_capacity.map(|mut val| {
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
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(mr_date_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from_slice(regionid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(version_datetime_array)
                        .to(arrow2::datatypes::DataType::Date32),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(stack_position_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from_slice(periodid_array)
                        .to(arrow2::datatypes::DataType::Decimal(3, 0)),
                ),
                std::sync::Arc::new(arrow2::array::Utf8Array::<i64>::from(duid_array)),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(accepted_capacity_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
                ),
                std::sync::Arc::new(
                    arrow2::array::PrimitiveArray::from(deducted_capacity_array)
                        .to(arrow2::datatypes::DataType::Decimal(6, 0)),
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
