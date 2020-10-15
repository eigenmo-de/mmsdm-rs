
create table FileLog (
    id bigint identity(1,1) not null primary key,
    file_name varchar(255) not null,
    data_set varchar(255) not null,
    sub_type varchar(255) not null,
    version tinyint not null
)
go
            
create table ForecastIntermittentGen1 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[duid] varchar(20) not null,
[start_interval_datetime] datetime2 not null,
[end_interval_datetime] datetime2 not null,
[versionno] decimal(10,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ForecastIntermittentGen1 on ForecastIntermittentGen1;
go
                        
create table OperationalDemandForecast1 (
file_log_id bigint not null references FileLog(id),
[interval_datetime] datetime2 not null,
[regionid] varchar(20) not null,
[load_date] datetime2 null,
[operational_demand_poe10] decimal(15,2) null,
[operational_demand_poe50] decimal(15,2) null,
[operational_demand_poe90] decimal(15,2) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_OperationalDemandForecast1 on OperationalDemandForecast1;
go
                        
create table DemandIntermittentClusterAvailDay1 (
file_log_id bigint not null references FileLog(id),
[tradingdate] datetime2 not null,
[duid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[clusterid] varchar(20) not null,

)
go
create clustered columnstore index cci_DemandIntermittentClusterAvailDay1 on DemandIntermittentClusterAvailDay1;
go
                        
create table DemandIntermittentClusterAvail1 (
file_log_id bigint not null references FileLog(id),
[tradingdate] datetime2 not null,
[duid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[clusterid] varchar(20) not null,
[periodid] decimal(3,0) not null,
[elements_unavailable] decimal(3,0) null,

)
go
create clustered columnstore index cci_DemandIntermittentClusterAvail1 on DemandIntermittentClusterAvail1;
go
                        
create table DemandPeriod1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 null,
[settlementdate] datetime2 not null,
[regionid] varchar(10) not null,
[offerdate] datetime2 not null,
[periodid] decimal(3,0) not null,
[versionno] decimal(3,0) not null,
[resdemand] decimal(10,0) null,
[demand90probability] decimal(10,0) null,
[demand10probability] decimal(10,0) null,
[lastchanged] datetime2 null,
[mr_schedule] decimal(6,0) null,

)
go
create clustered columnstore index cci_DemandPeriod1 on DemandPeriod1;
go
                        
create table DemandIntermittentDsRun1 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[duid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[origin] varchar(20) not null,
[forecast_priority] decimal(10,0) not null,
[authorisedby] varchar(20) null,
[comments] varchar(200) null,
[lastchanged] datetime2 null,
[model] varchar(30) null,
[participant_timestamp] datetime2 null,
[suppressed_aemo] decimal(1,0) null,
[suppressed_participant] decimal(1,0) null,
[transaction_id] varchar(100) null,

)
go
create clustered columnstore index cci_DemandIntermittentDsRun1 on DemandIntermittentDsRun1;
go
                        
create table ForecastIntermittentGenData1 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[duid] varchar(20) not null,
[interval_datetime] datetime2 not null,
[powermean] decimal(9,3) null,
[powerpoe50] decimal(9,3) null,
[powerpoelow] decimal(9,3) null,
[powerpoehigh] decimal(9,3) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ForecastIntermittentGenData1 on ForecastIntermittentGenData1;
go
                        
create table DemandIntermittentDsPred1 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[duid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[interval_datetime] datetime2 not null,
[origin] varchar(20) not null,
[forecast_priority] decimal(10,0) not null,
[forecast_mean] decimal(18,8) null,
[forecast_poe10] decimal(18,8) null,
[forecast_poe50] decimal(18,8) null,
[forecast_poe90] decimal(18,8) null,

)
go
create clustered columnstore index cci_DemandIntermittentDsPred1 on DemandIntermittentDsPred1;
go
                        
create table DemandMtpasaIntermittentAvail1 (
file_log_id bigint not null references FileLog(id),
[tradingdate] datetime2 not null,
[duid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[clusterid] varchar(20) not null,
[lastchanged] datetime2 null,
[elements_unavailable] decimal(3,0) null,

)
go
create clustered columnstore index cci_DemandMtpasaIntermittentAvail1 on DemandMtpasaIntermittentAvail1;
go
                        
create table RooftopActual2 (
file_log_id bigint not null references FileLog(id),
[interval_datetime] datetime2 not null,
[type] varchar(20) not null,
[regionid] varchar(20) not null,
[power] decimal(12,3) null,
[qi] decimal(2,1) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_RooftopActual2 on RooftopActual2;
go
                        
create table RooftopForecast1 (
file_log_id bigint not null references FileLog(id),
[version_datetime] datetime2 not null,
[regionid] varchar(20) not null,
[interval_datetime] datetime2 not null,
[powermean] decimal(12,3) null,
[powerpoe50] decimal(12,3) null,
[powerpoelow] decimal(12,3) null,
[powerpoehigh] decimal(12,3) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_RooftopForecast1 on RooftopForecast1;
go
                        
create table DemandMtpasaIntermittentLimit1 (
file_log_id bigint not null references FileLog(id),
[tradingdate] datetime2 not null,
[duid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[lastchanged] datetime2 null,
[uppermwlimit] decimal(6,0) null,
[authorisedbyuser] varchar(20) null,
[authorisedbyparticipantid] varchar(20) null,

)
go
create clustered columnstore index cci_DemandMtpasaIntermittentLimit1 on DemandMtpasaIntermittentLimit1;
go
                        
create table OperationalDemandActual1 (
file_log_id bigint not null references FileLog(id),
[interval_datetime] datetime2 not null,
[regionid] varchar(20) not null,
[operational_demand] decimal(10,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_OperationalDemandActual1 on OperationalDemandActual1;
go
                        
create table DemandTrk1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[regionid] varchar(10) not null,
[offerdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[filename] varchar(40) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(10) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_DemandTrk1 on DemandTrk1;
go
                        
create table DemandIntermittentGenLimit1 (
file_log_id bigint not null references FileLog(id),
[tradingdate] datetime2 not null,
[duid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[periodid] decimal(3,0) not null,
[uppermwlimit] decimal(6,0) null,

)
go
create clustered columnstore index cci_DemandIntermittentGenLimit1 on DemandIntermittentGenLimit1;
go
                        
create table DemandIntermittentGenLimitDay1 (
file_log_id bigint not null references FileLog(id),
[tradingdate] datetime2 not null,
[duid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[participantid] varchar(20) null,
[lastchanged] datetime2 null,
[authorisedbyuser] varchar(20) null,
[authorisedbyparticipantid] varchar(20) null,

)
go
create clustered columnstore index cci_DemandIntermittentGenLimitDay1 on DemandIntermittentGenLimitDay1;
go
                        
create table AsofferOfferagcdata1 (
file_log_id bigint not null references FileLog(id),
[contractid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[availability] decimal(4,0) null,
[upperlimit] decimal(4,0) null,
[lowerlimit] decimal(4,0) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[filename] varchar(40) null,
[lastchanged] datetime2 null,
[periodid] decimal(3,0) not null,
[agcup] decimal(3,0) null,
[agcdown] decimal(3,0) null,

)
go
create clustered columnstore index cci_AsofferOfferagcdata1 on AsofferOfferagcdata1;
go
                        
create table AsofferOfferrpowerdata1 (
file_log_id bigint not null references FileLog(id),
[contractid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[periodid] decimal(3,0) not null,
[availability] decimal(3,0) null,
[mta] decimal(6,0) null,
[mtg] decimal(6,0) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[filename] varchar(40) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_AsofferOfferrpowerdata1 on AsofferOfferrpowerdata1;
go
                        
create table AsofferOfferrestartdata1 (
file_log_id bigint not null references FileLog(id),
[contractid] varchar(10) not null,
[offerdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[availability] varchar(3) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[filename] varchar(40) null,
[lastchanged] datetime2 null,
[periodid] decimal(3,0) not null,

)
go
create clustered columnstore index cci_AsofferOfferrestartdata1 on AsofferOfferrestartdata1;
go
                        
create table AsofferOfferlsheddata1 (
file_log_id bigint not null references FileLog(id),
[contractid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[availableload] decimal(4,0) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[filename] varchar(40) null,
[lastchanged] datetime2 null,
[periodid] decimal(3,0) not null,

)
go
create clustered columnstore index cci_AsofferOfferlsheddata1 on AsofferOfferlsheddata1;
go
                        
create table AsofferOfferastrk1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[filename] varchar(40) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_AsofferOfferastrk1 on AsofferOfferastrk1;
go
                        
create table GdInstructGdinstruct1 (
file_log_id bigint not null references FileLog(id),
[duid] varchar(10) null,
[stationid] varchar(10) null,
[regionid] varchar(10) null,
[id] decimal(22,0) not null,
[instructiontypeid] varchar(10) null,
[instructionsubtypeid] varchar(10) null,
[instructionclassid] varchar(10) null,
[reason] varchar(64) null,
[instlevel] decimal(6,0) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[participantid] varchar(10) null,
[issuedtime] datetime2 null,
[targettime] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_GdInstructGdinstruct1 on GdInstructGdinstruct1;
go
                        
create table GdInstructInstructionsubtype1 (
file_log_id bigint not null references FileLog(id),
[instructiontypeid] varchar(10) not null,
[instructionsubtypeid] varchar(10) not null,
[description] varchar(64) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_GdInstructInstructionsubtype1 on GdInstructInstructionsubtype1;
go
                        
create table GdInstructInstructiontype1 (
file_log_id bigint not null references FileLog(id),
[instructiontypeid] varchar(10) not null,
[description] varchar(64) null,
[regionid] varchar(10) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_GdInstructInstructiontype1 on GdInstructInstructiontype1;
go
                        
create table GeqrhsNull1 (
file_log_id bigint not null references FileLog(id),
[equationid] varchar(20) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[termid] decimal(3,0) not null,
[groupid] decimal(3,0) null,
[spd_id] varchar(21) null,
[spd_type] varchar(1) null,
[factor] decimal(16,6) null,
[operation] varchar(10) null,
[defaultvalue] decimal(16,6) null,
[parameterterm1] varchar(12) null,
[parameterterm2] varchar(12) null,
[parameterterm3] varchar(12) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_GeqrhsNull1 on GeqrhsNull1;
go
                        
create table GenconsetNull1 (
file_log_id bigint not null references FileLog(id),
[genconsetid] varchar(20) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[genconid] varchar(20) not null,
[genconeffdate] datetime2 null,
[genconversionno] decimal(3,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_GenconsetNull1 on GenconsetNull1;
go
                        
create table GenericConstraintGenconsetinvoke2 (
file_log_id bigint not null references FileLog(id),
[invocation_id] decimal(9,0) not null,
[startdate] datetime2 not null,
[startperiod] decimal(3,0) not null,
[genconsetid] varchar(20) not null,
[enddate] datetime2 null,
[endperiod] decimal(3,0) null,
[startauthorisedby] varchar(15) null,
[endauthorisedby] varchar(15) null,
[intervention] varchar(1) null,
[asconstrainttype] varchar(10) null,
[lastchanged] datetime2 null,
[startintervaldatetime] datetime2 null,
[endintervaldatetime] datetime2 null,
[systemnormal] varchar(1) null,

)
go
create clustered columnstore index cci_GenericConstraintGenconsetinvoke2 on GenericConstraintGenconsetinvoke2;
go
                        
create table SpdrcNull2 (
file_log_id bigint not null references FileLog(id),
[regionid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[genconid] varchar(20) not null,
[factor] decimal(16,6) null,
[lastchanged] datetime2 null,
[bidtype] varchar(10) not null,

)
go
create clustered columnstore index cci_SpdrcNull2 on SpdrcNull2;
go
                        
create table GencondataNull6 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[genconid] varchar(20) not null,
[constrainttype] varchar(2) null,
[constraintvalue] decimal(16,6) null,
[description] varchar(256) null,
[status] varchar(8) null,
[genericconstraintweight] decimal(16,6) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[dynamicrhs] decimal(15,5) null,
[lastchanged] datetime2 null,
[dispatch] varchar(1) null,
[predispatch] varchar(1) null,
[stpasa] varchar(1) null,
[mtpasa] varchar(1) null,
[impact] varchar(64) null,
[source] varchar(128) null,
[limittype] varchar(64) null,
[reason] varchar(256) null,
[modifications] varchar(256) null,
[additionalnotes] varchar(256) null,
[p5min_scope_override] varchar(2) null,
[lrc] varchar(1) null,
[lor] varchar(1) null,
[force_scada] decimal(1,0) null,

)
go
create clustered columnstore index cci_GencondataNull6 on GencondataNull6;
go
                        
create table SpdcpcNull2 (
file_log_id bigint not null references FileLog(id),
[connectionpointid] varchar(12) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[genconid] varchar(20) not null,
[factor] decimal(16,6) null,
[lastchanged] datetime2 null,
[bidtype] varchar(12) not null,

)
go
create clustered columnstore index cci_SpdcpcNull2 on SpdcpcNull2;
go
                        
create table GcrhsNull1 (
file_log_id bigint not null references FileLog(id),
[genconid] varchar(20) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(22,0) not null,
[scope] varchar(2) not null,
[termid] decimal(4,0) not null,
[groupid] decimal(3,0) null,
[spd_id] varchar(21) null,
[spd_type] varchar(1) null,
[factor] decimal(16,6) null,
[operation] varchar(10) null,
[defaultvalue] decimal(16,6) null,
[parameterterm1] varchar(12) null,
[parameterterm2] varchar(12) null,
[parameterterm3] varchar(12) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_GcrhsNull1 on GcrhsNull1;
go
                        
create table GenericConstraintEmsmaster1 (
file_log_id bigint not null references FileLog(id),
[spd_id] varchar(21) not null,
[spd_type] varchar(1) not null,
[description] varchar(255) null,
[grouping_id] varchar(20) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_GenericConstraintEmsmaster1 on GenericConstraintEmsmaster1;
go
                        
create table GenconsettrkNull2 (
file_log_id bigint not null references FileLog(id),
[genconsetid] varchar(20) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[description] varchar(256) null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
[coverage] varchar(64) null,
[modifications] varchar(256) null,
[systemnormal] varchar(1) null,
[outage] varchar(256) null,

)
go
create clustered columnstore index cci_GenconsettrkNull2 on GenconsettrkNull2;
go
                        
create table SpdiccNull1 (
file_log_id bigint not null references FileLog(id),
[interconnectorid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[genconid] varchar(20) not null,
[factor] decimal(16,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SpdiccNull1 on SpdiccNull1;
go
                        
create table GeqdescNull2 (
file_log_id bigint not null references FileLog(id),
[equationid] varchar(20) not null,
[description] varchar(256) null,
[lastchanged] datetime2 null,
[impact] varchar(64) null,
[source] varchar(128) null,
[limittype] varchar(64) null,
[reason] varchar(256) null,
[modifications] varchar(256) null,
[additionalnotes] varchar(256) null,

)
go
create clustered columnstore index cci_GeqdescNull2 on GeqdescNull2;
go
                        
create table MtpasaReservelimitSet1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[version_datetime] datetime2 not null,
[reservelimit_set_id] varchar(20) null,
[description] varchar(200) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(20) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MtpasaReservelimitSet1 on MtpasaReservelimitSet1;
go
                        
create table ReserveDataReserve1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[regionid] varchar(12) not null,
[periodid] decimal(2,0) not null,
[lower5min] decimal(6,0) null,
[lower60sec] decimal(6,0) null,
[lower6sec] decimal(6,0) null,
[raise5min] decimal(6,0) null,
[raise60sec] decimal(6,0) null,
[raise6sec] decimal(6,0) null,
[lastchanged] datetime2 null,
[pasareserve] decimal(6,0) null,
[loadrejectionreservereq] decimal(10,0) null,
[raisereg] decimal(6,0) null,
[lowerreg] decimal(6,0) null,
[lor1level] decimal(6,0) null,
[lor2level] decimal(6,0) null,

)
go
create clustered columnstore index cci_ReserveDataReserve1 on ReserveDataReserve1;
go
                        
create table MtpasaReservelimit1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[version_datetime] datetime2 not null,
[reservelimitid] varchar(20) not null,
[description] varchar(200) null,
[rhs] decimal(16,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MtpasaReservelimit1 on MtpasaReservelimit1;
go
                        
create table MtpasaReservelimitRegion1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[version_datetime] datetime2 not null,
[reservelimitid] varchar(20) not null,
[regionid] varchar(20) not null,
[coef] decimal(16,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MtpasaReservelimitRegion1 on MtpasaReservelimitRegion1;
go
                        
create table MrEventSchedule1 (
file_log_id bigint not null references FileLog(id),
[mr_date] datetime2 not null,
[regionid] varchar(10) not null,
[version_datetime] datetime2 not null,
[demand_effectivedate] datetime2 null,
[demand_offerdate] datetime2 null,
[demand_versionno] decimal(3,0) null,
[authorisedby] varchar(20) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MrEventSchedule1 on MrEventSchedule1;
go
                        
create table MrDayofferStack1 (
file_log_id bigint not null references FileLog(id),
[mr_date] datetime2 not null,
[regionid] varchar(10) not null,
[version_datetime] datetime2 not null,
[stack_position] decimal(3,0) not null,
[duid] varchar(10) null,
[authorised] decimal(1,0) null,
[offer_settlementdate] datetime2 null,
[offer_offerdate] datetime2 null,
[offer_versionno] decimal(3,0) null,
[offer_type] varchar(20) null,
[laof] decimal(16,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MrDayofferStack1 on MrDayofferStack1;
go
                        
create table MrPerofferStack1 (
file_log_id bigint not null references FileLog(id),
[mr_date] datetime2 not null,
[regionid] varchar(10) not null,
[version_datetime] datetime2 not null,
[stack_position] decimal(3,0) not null,
[periodid] decimal(3,0) not null,
[duid] varchar(10) null,
[accepted_capacity] decimal(6,0) null,
[deducted_capacity] decimal(6,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MrPerofferStack1 on MrPerofferStack1;
go
                        
create table MrEvent1 (
file_log_id bigint not null references FileLog(id),
[mr_date] datetime2 not null,
[regionid] varchar(10) not null,
[description] varchar(200) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(20) null,
[offer_cut_off_time] datetime2 null,
[settlement_complete] decimal(1,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MrEvent1 on MrEvent1;
go
                        
create table MarketNoticeParticipantnoticetrk1 (
file_log_id bigint not null references FileLog(id),
[participantid] varchar(10) not null,
[noticeid] decimal(10,0) not null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MarketNoticeParticipantnoticetrk1 on MarketNoticeParticipantnoticetrk1;
go
                        
create table MarketNoticeMarketnoticetype1 (
file_log_id bigint not null references FileLog(id),
[typeid] varchar(25) not null,
[description] varchar(64) null,
[raisedby] varchar(10) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MarketNoticeMarketnoticetype1 on MarketNoticeMarketnoticetype1;
go
                        
create table MarketNoticeMarketnoticedata1 (
file_log_id bigint not null references FileLog(id),
[noticeid] decimal(10,0) not null,
[effectivedate] datetime2 null,
[typeid] varchar(25) null,
[noticetype] varchar(25) null,
[lastchanged] datetime2 null,
[reason] varchar(2000) null,
[externalreference] varchar(255) null,

)
go
create clustered columnstore index cci_MarketNoticeMarketnoticedata1 on MarketNoticeMarketnoticedata1;
go
                        
create table P5minBlockedConstraints1 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[constraintid] varchar(20) not null,

)
go
create clustered columnstore index cci_P5minBlockedConstraints1 on P5minBlockedConstraints1;
go
                        
create table P5minCasesolution2 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[startinterval_datetime] varchar(20) null,
[totalobjective] decimal(27,10) null,
[nonphysicallosses] decimal(1,0) null,
[totalareagenviolation] decimal(15,5) null,
[totalinterconnectorviolation] decimal(15,5) null,
[totalgenericviolation] decimal(15,5) null,
[totalramprateviolation] decimal(15,5) null,
[totalunitmwcapacityviolation] decimal(15,5) null,
[total5minviolation] decimal(15,5) null,
[totalregviolation] decimal(15,5) null,
[total6secviolation] decimal(15,5) null,
[total60secviolation] decimal(15,5) null,
[totalenergyconstrviolation] decimal(15,5) null,
[totalenergyofferviolation] decimal(15,5) null,
[totalasprofileviolation] decimal(15,5) null,
[totalfaststartviolation] decimal(15,5) null,
[lastchanged] datetime2 null,
[intervention] decimal(2,0) null,

)
go
create clustered columnstore index cci_P5minCasesolution2 on P5minCasesolution2;
go
                        
create table P5minLocalPrice1 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[interval_datetime] datetime2 not null,
[duid] varchar(20) not null,
[local_price_adjustment] decimal(10,2) null,
[locally_constrained] decimal(1,0) null,

)
go
create clustered columnstore index cci_P5minLocalPrice1 on P5minLocalPrice1;
go
                        
create table P5minRegionsolution5 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[interval_datetime] datetime2 not null,
[regionid] varchar(10) not null,
[rrp] decimal(15,5) null,
[rop] decimal(15,5) null,
[excessgeneration] decimal(15,5) null,
[raise6secrrp] decimal(15,5) null,
[raise6secrop] decimal(15,5) null,
[raise60secrrp] decimal(15,5) null,
[raise60secrop] decimal(15,5) null,
[raise5minrrp] decimal(15,5) null,
[raise5minrop] decimal(15,5) null,
[raiseregrrp] decimal(15,5) null,
[raiseregrop] decimal(15,5) null,
[lower6secrrp] decimal(15,5) null,
[lower6secrop] decimal(15,5) null,
[lower60secrrp] decimal(15,5) null,
[lower60secrop] decimal(15,5) null,
[lower5minrrp] decimal(15,5) null,
[lower5minrop] decimal(15,5) null,
[lowerregrrp] decimal(15,5) null,
[lowerregrop] decimal(15,5) null,
[totaldemand] decimal(15,5) null,
[availablegeneration] decimal(15,5) null,
[availableload] decimal(15,5) null,
[demandforecast] decimal(15,5) null,
[dispatchablegeneration] decimal(15,5) null,
[dispatchableload] decimal(15,5) null,
[netinterchange] decimal(15,5) null,
[lower5mindispatch] decimal(15,5) null,
[lower5minimport] decimal(15,5) null,
[lower5minlocaldispatch] decimal(15,5) null,
[lower5minlocalreq] decimal(15,5) null,
[lower5minreq] decimal(15,5) null,
[lower60secdispatch] decimal(15,5) null,
[lower60secimport] decimal(15,5) null,
[lower60seclocaldispatch] decimal(15,5) null,
[lower60seclocalreq] decimal(15,5) null,
[lower60secreq] decimal(15,5) null,
[lower6secdispatch] decimal(15,5) null,
[lower6secimport] decimal(15,5) null,
[lower6seclocaldispatch] decimal(15,5) null,
[lower6seclocalreq] decimal(15,5) null,
[lower6secreq] decimal(15,5) null,
[raise5mindispatch] decimal(15,5) null,
[raise5minimport] decimal(15,5) null,
[raise5minlocaldispatch] decimal(15,5) null,
[raise5minlocalreq] decimal(15,5) null,
[raise5minreq] decimal(15,5) null,
[raise60secdispatch] decimal(15,5) null,
[raise60secimport] decimal(15,5) null,
[raise60seclocaldispatch] decimal(15,5) null,
[raise60seclocalreq] decimal(15,5) null,
[raise60secreq] decimal(15,5) null,
[raise6secdispatch] decimal(15,5) null,
[raise6secimport] decimal(15,5) null,
[raise6seclocaldispatch] decimal(15,5) null,
[raise6seclocalreq] decimal(15,5) null,
[raise6secreq] decimal(15,5) null,
[aggregatedispatcherror] decimal(15,5) null,
[initialsupply] decimal(15,5) null,
[clearedsupply] decimal(15,5) null,
[lowerregimport] decimal(15,5) null,
[lowerregdispatch] decimal(15,5) null,
[lowerreglocaldispatch] decimal(15,5) null,
[lowerreglocalreq] decimal(15,5) null,
[lowerregreq] decimal(15,5) null,
[raiseregimport] decimal(15,5) null,
[raiseregdispatch] decimal(15,5) null,
[raisereglocaldispatch] decimal(15,5) null,
[raisereglocalreq] decimal(15,5) null,
[raiseregreq] decimal(15,5) null,
[raise5minlocalviolation] decimal(15,5) null,
[raisereglocalviolation] decimal(15,5) null,
[raise60seclocalviolation] decimal(15,5) null,
[raise6seclocalviolation] decimal(15,5) null,
[lower5minlocalviolation] decimal(15,5) null,
[lowerreglocalviolation] decimal(15,5) null,
[lower60seclocalviolation] decimal(15,5) null,
[lower6seclocalviolation] decimal(15,5) null,
[raise5minviolation] decimal(15,5) null,
[raiseregviolation] decimal(15,5) null,
[raise60secviolation] decimal(15,5) null,
[raise6secviolation] decimal(15,5) null,
[lower5minviolation] decimal(15,5) null,
[lowerregviolation] decimal(15,5) null,
[lower60secviolation] decimal(15,5) null,
[lower6secviolation] decimal(15,5) null,
[lastchanged] datetime2 null,
[totalintermittentgeneration] decimal(15,5) null,
[demand_and_nonschedgen] decimal(15,5) null,
[uigf] decimal(15,5) null,
[semischedule_clearedmw] decimal(15,5) null,
[semischedule_compliancemw] decimal(15,5) null,
[intervention] decimal(2,0) null,
[ss_solar_uigf] decimal(15,5) null,
[ss_wind_uigf] decimal(15,5) null,
[ss_solar_clearedmw] decimal(15,5) null,
[ss_wind_clearedmw] decimal(15,5) null,
[ss_solar_compliancemw] decimal(15,5) null,
[ss_wind_compliancemw] decimal(15,5) null,

)
go
create clustered columnstore index cci_P5minRegionsolution5 on P5minRegionsolution5;
go
                        
create table P5minUnitsolution3 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[interval_datetime] datetime2 not null,
[duid] varchar(10) not null,
[connectionpointid] varchar(12) null,
[tradetype] decimal(2,0) null,
[agcstatus] decimal(2,0) null,
[initialmw] decimal(15,5) null,
[totalcleared] decimal(15,5) null,
[rampdownrate] decimal(15,5) null,
[rampuprate] decimal(15,5) null,
[lower5min] decimal(15,5) null,
[lower60sec] decimal(15,5) null,
[lower6sec] decimal(15,5) null,
[raise5min] decimal(15,5) null,
[raise60sec] decimal(15,5) null,
[raise6sec] decimal(15,5) null,
[lowerreg] decimal(15,5) null,
[raisereg] decimal(15,5) null,
[availability] decimal(15,5) null,
[raise6secflags] decimal(3,0) null,
[raise60secflags] decimal(3,0) null,
[raise5minflags] decimal(3,0) null,
[raiseregflags] decimal(3,0) null,
[lower6secflags] decimal(3,0) null,
[lower60secflags] decimal(3,0) null,
[lower5minflags] decimal(3,0) null,
[lowerregflags] decimal(3,0) null,
[lastchanged] datetime2 null,
[semidispatchcap] decimal(3,0) null,
[intervention] decimal(2,0) null,

)
go
create clustered columnstore index cci_P5minUnitsolution3 on P5minUnitsolution3;
go
                        
create table P5minConstraintsolution6 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[interval_datetime] datetime2 not null,
[constraintid] varchar(20) not null,
[rhs] decimal(15,5) null,
[marginalvalue] decimal(15,5) null,
[violationdegree] decimal(15,5) null,
[lastchanged] datetime2 null,
[duid] varchar(20) null,
[genconid_effectivedate] datetime2 null,
[genconid_versionno] decimal(22,0) null,
[lhs] decimal(15,5) null,
[intervention] decimal(2,0) null,

)
go
create clustered columnstore index cci_P5minConstraintsolution6 on P5minConstraintsolution6;
go
                        
create table P5minInterconnectorsoln4 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[interconnectorid] varchar(10) not null,
[interval_datetime] datetime2 not null,
[meteredmwflow] decimal(15,5) null,
[mwflow] decimal(15,5) null,
[mwlosses] decimal(15,5) null,
[marginalvalue] decimal(15,5) null,
[violationdegree] decimal(15,5) null,
[mnsp] decimal(1,0) null,
[exportlimit] decimal(15,5) null,
[importlimit] decimal(15,5) null,
[marginalloss] decimal(15,5) null,
[exportgenconid] varchar(20) null,
[importgenconid] varchar(20) null,
[fcasexportlimit] decimal(15,5) null,
[fcasimportlimit] decimal(15,5) null,
[lastchanged] datetime2 null,
[local_price_adjustment_export] decimal(10,2) null,
[locally_constrained_export] decimal(1,0) null,
[local_price_adjustment_import] decimal(10,2) null,
[locally_constrained_import] decimal(1,0) null,
[intervention] decimal(2,0) null,

)
go
create clustered columnstore index cci_P5minInterconnectorsoln4 on P5minInterconnectorsoln4;
go
                        
create table SettlementsLunloadpayment5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[contractid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[duid] varchar(10) null,
[regionid] varchar(10) null,
[tlf] decimal(7,5) null,
[ebp] decimal(15,5) null,
[rrp] decimal(15,5) null,
[enablingprice] decimal(15,5) null,
[usageprice] decimal(15,5) null,
[ccprice] decimal(15,5) null,
[clearedmw] decimal(15,5) null,
[unconstrainedmw] decimal(15,5) null,
[controlrange] decimal(4,0) null,
[enablingpayment] decimal(15,5) null,
[usagepayment] decimal(15,5) null,
[compensationpayment] decimal(15,5) null,
[contractversionno] decimal(3,0) null,
[offerdate] datetime2 null,
[offerversionno] decimal(3,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsLunloadpayment5 on SettlementsLunloadpayment5;
go
                        
create table SettlementsFcasRecovery6 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] varchar(3) not null,
[participantid] varchar(10) not null,
[regionid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[lower6sec_recovery] decimal(18,8) null,
[raise6sec_recovery] decimal(18,8) null,
[lower60sec_recovery] decimal(18,8) null,
[raise60sec_recovery] decimal(18,8) null,
[lower5min_recovery] decimal(18,8) null,
[raise5min_recovery] decimal(18,8) null,
[lowerreg_recovery] decimal(18,8) null,
[raisereg_recovery] decimal(18,8) null,
[lastchanged] datetime2 null,
[lower6sec_recovery_gen] decimal(18,8) null,
[raise6sec_recovery_gen] decimal(18,8) null,
[lower60sec_recovery_gen] decimal(18,8) null,
[raise60sec_recovery_gen] decimal(18,8) null,
[lower5min_recovery_gen] decimal(18,8) null,
[raise5min_recovery_gen] decimal(18,8) null,
[lowerreg_recovery_gen] decimal(18,8) null,
[raisereg_recovery_gen] decimal(18,8) null,

)
go
create clustered columnstore index cci_SettlementsFcasRecovery6 on SettlementsFcasRecovery6;
go
                        
create table SettlementsAncillarySummary5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[service] varchar(20) not null,
[paymenttype] varchar(20) not null,
[regionid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[paymentamount] decimal(18,8) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsAncillarySummary5 on SettlementsAncillarySummary5;
go
                        
create table SettlementsMrRecovery5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[regionid] varchar(10) not null,
[participantid] varchar(10) null,
[duid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[arodef] decimal(16,6) null,
[nta] decimal(16,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsMrRecovery5 on SettlementsMrRecovery5;
go
                        
create table SettlementsApcCompensation1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[apeventid] decimal(6,0) not null,
[claimid] decimal(6,0) not null,
[participantid] varchar(20) not null,
[periodid] decimal(3,0) not null,
[compensation_amount] decimal(18,8) null,

)
go
create clustered columnstore index cci_SettlementsApcCompensation1 on SettlementsApcCompensation1;
go
                        
create table SettlementsInterventionrecovery5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[periodid] decimal(3,0) not null,
[contractid] varchar(10) not null,
[rcf] char(1) null,
[participantid] varchar(10) not null,
[participantdemand] decimal(12,5) null,
[totaldemand] decimal(12,5) null,
[interventionpayment] decimal(12,5) null,
[interventionamount] decimal(12,5) null,
[lastchanged] datetime2 null,
[regionid] varchar(10) null,

)
go
create clustered columnstore index cci_SettlementsInterventionrecovery5 on SettlementsInterventionrecovery5;
go
                        
create table SettlementsIrpartsurplus6 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[settlementrunno] decimal(3,0) not null,
[contractid] varchar(10) not null,
[periodid] decimal(2,0) not null,
[participantid] varchar(10) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[totalsurplus] decimal(15,5) null,
[contractallocation] decimal(8,5) null,
[surplusvalue] decimal(15,5) null,
[lastchanged] datetime2 null,
[csp_derogation_amount] decimal(18,8) null,
[unadjusted_irsr] decimal(18,8) null,

)
go
create clustered columnstore index cci_SettlementsIrpartsurplus6 on SettlementsIrpartsurplus6;
go
                        
create table SettlementsIrsurplus6 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[settlementrunno] decimal(3,0) not null,
[periodid] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[regionid] varchar(10) not null,
[mwflow] decimal(15,6) null,
[lossfactor] decimal(15,5) null,
[surplusvalue] decimal(15,5) null,
[lastchanged] datetime2 null,
[csp_derogation_amount] decimal(18,8) null,
[unadjusted_irsr] decimal(18,8) null,

)
go
create clustered columnstore index cci_SettlementsIrsurplus6 on SettlementsIrsurplus6;
go
                        
create table SettlementsVicenergyflow5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[periodid] decimal(3,0) not null,
[netflow] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsVicenergyflow5 on SettlementsVicenergyflow5;
go
                        
create table SettlementsLshedrecovery5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[contractid] varchar(10) null,
[periodid] decimal(3,0) not null,
[regionid] varchar(10) not null,
[lsepayment] decimal(15,5) null,
[ccpayment] decimal(15,5) null,
[participantdemand] decimal(15,5) null,
[regiondemand] decimal(15,5) null,
[lserecovery] decimal(15,5) null,
[ccrecovery] decimal(15,5) null,
[lastchanged] datetime2 null,
[lserecovery_gen] decimal(15,5) null,
[ccrecovery_gen] decimal(15,5) null,
[participantdemand_gen] decimal(15,5) null,
[regiondemand_gen] decimal(15,5) null,
[availabilityrecovery] decimal(16,6) null,
[availabilityrecovery_gen] decimal(16,6) null,

)
go
create clustered columnstore index cci_SettlementsLshedrecovery5 on SettlementsLshedrecovery5;
go
                        
create table SettlementsRestartrecovery6 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[contractid] varchar(10) null,
[periodid] decimal(3,0) not null,
[regionid] varchar(10) not null,
[availabilitypayment] decimal(15,5) null,
[participantdemand] decimal(15,5) null,
[regiondemand] decimal(15,5) null,
[availabilityrecovery] decimal(15,5) null,
[lastchanged] datetime2 null,
[availabilityrecovery_gen] decimal(15,5) null,
[participantdemand_gen] decimal(15,5) null,
[regiondemand_gen] decimal(15,5) null,
[enablingpayment] decimal(18,8) null,
[enablingrecovery] decimal(18,8) null,
[enablingrecovery_gen] decimal(18,8) null,

)
go
create clustered columnstore index cci_SettlementsRestartrecovery6 on SettlementsRestartrecovery6;
go
                        
create table SettlementsGendata6 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(10,0) not null,
[periodid] decimal(10,0) not null,
[participantid] varchar(10) null,
[stationid] varchar(10) not null,
[duid] varchar(10) not null,
[gensetid] varchar(10) not null,
[regionid] varchar(10) not null,
[genergy] decimal(16,6) null,
[aenergy] decimal(16,6) null,
[gpower] decimal(16,6) null,
[apower] decimal(16,6) null,
[rrp] decimal(20,5) null,
[eep] decimal(16,6) null,
[tlf] decimal(7,5) null,
[cprrp] decimal(16,6) null,
[cpeep] decimal(16,6) null,
[netenergy] decimal(16,6) null,
[energycost] decimal(16,6) null,
[excessenergycost] decimal(16,6) null,
[apc] decimal(16,6) null,
[resc] decimal(16,6) null,
[resp] decimal(16,6) null,
[lastchanged] datetime2 null,
[expenergy] decimal(15,6) null,
[expenergycost] decimal(15,6) null,
[meterrunno] decimal(6,0) null,
[mda] varchar(10) null,
[secondary_tlf] decimal(7,5) null,

)
go
create clustered columnstore index cci_SettlementsGendata6 on SettlementsGendata6;
go
                        
create table SettlementsRunParameter5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[parameterid] varchar(20) not null,
[numvalue] decimal(18,8) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsRunParameter5 on SettlementsRunParameter5;
go
                        
create table SettlementsReallocations5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[periodid] decimal(3,0) not null,
[participantid] varchar(10) not null,
[reallocationid] varchar(20) not null,
[reallocationvalue] decimal(15,5) null,
[energy] decimal(15,5) null,
[rrp] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsReallocations5 on SettlementsReallocations5;
go
                        
create table SettlementsAgcrecovery5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[contractid] varchar(10) null,
[periodid] decimal(3,0) not null,
[regionid] varchar(10) not null,
[enablingpayment] decimal(15,5) null,
[participantdemand] decimal(15,5) null,
[regiondemand] decimal(15,5) null,
[enablingrecovery] decimal(15,5) null,
[lastchanged] datetime2 null,
[enablingrecovery_gen] decimal(15,5) null,
[participantdemand_gen] decimal(15,5) null,
[regiondemand_gen] decimal(15,5) null,

)
go
create clustered columnstore index cci_SettlementsAgcrecovery5 on SettlementsAgcrecovery5;
go
                        
create table SettlementsRpowerpayment6 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[contractid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[duid] varchar(10) null,
[regionid] varchar(10) null,
[tlf] decimal(7,5) null,
[ebp] decimal(15,5) null,
[rrp] decimal(15,5) null,
[mvaraprice] decimal(15,5) null,
[mvareprice] decimal(15,5) null,
[mvargprice] decimal(15,5) null,
[ccprice] decimal(15,5) null,
[synccompensation] decimal(1,0) null,
[mta] decimal(15,5) null,
[mtg] decimal(15,5) null,
[blocksize] decimal(4,0) null,
[avaflag] decimal(1,0) null,
[clearedmw] decimal(15,5) null,
[unconstrainedmw] decimal(15,5) null,
[availabilitypayment] decimal(15,5) null,
[enablingpayment] decimal(15,5) null,
[ccpayment] decimal(15,5) null,
[contractversionno] decimal(3,0) null,
[offerdate] datetime2 null,
[offerversionno] decimal(3,0) null,
[lastchanged] datetime2 null,
[availabilitypayment_rebate] decimal(18,8) null,

)
go
create clustered columnstore index cci_SettlementsRpowerpayment6 on SettlementsRpowerpayment6;
go
                        
create table SettlementsVicenergyfigures5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[periodid] decimal(3,0) not null,
[totalgenoutput] decimal(15,5) null,
[totalpcsd] decimal(15,5) null,
[lastchanged] datetime2 null,
[tlr] decimal(15,6) null,
[mlf] decimal(15,6) null,

)
go
create clustered columnstore index cci_SettlementsVicenergyfigures5 on SettlementsVicenergyfigures5;
go
                        
create table SettlementsGendataregion5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(22,10) not null,
[periodid] decimal(22,10) not null,
[regionid] varchar(10) not null,
[genergy] decimal(22,0) null,
[aenergy] decimal(22,0) null,
[gpower] decimal(22,0) null,
[apower] decimal(22,0) null,
[netenergy] decimal(27,5) null,
[energycost] decimal(27,5) null,
[excessenergycost] decimal(27,5) null,
[expenergy] decimal(27,6) null,
[expenergycost] decimal(27,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsGendataregion5 on SettlementsGendataregion5;
go
                        
create table SettlementsNmasRecoveryRbf1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[periodid] decimal(3,0) not null,
[service] varchar(10) not null,
[contractid] varchar(10) not null,
[paymenttype] varchar(20) not null,
[regionid] varchar(10) not null,
[rbf] decimal(18,8) null,
[payment_amount] decimal(18,8) null,
[recovery_amount] decimal(18,8) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsNmasRecoveryRbf1 on SettlementsNmasRecoveryRbf1;
go
                        
create table SettlementsMarketfees5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[marketfeeid] varchar(10) not null,
[marketfeevalue] decimal(15,5) null,
[energy] decimal(16,6) null,
[lastchanged] datetime2 null,
[participantcategoryid] varchar(10) not null,

)
go
create clustered columnstore index cci_SettlementsMarketfees5 on SettlementsMarketfees5;
go
                        
create table SettlementsApcRecovery1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[apeventid] decimal(6,0) not null,
[claimid] decimal(6,0) not null,
[participantid] varchar(20) not null,
[periodid] decimal(3,0) not null,
[regionid] varchar(20) not null,
[recovery_amount] decimal(18,8) null,
[region_recovery_br_amount] decimal(18,8) null,

)
go
create clustered columnstore index cci_SettlementsApcRecovery1 on SettlementsApcRecovery1;
go
                        
create table SettlementsLunloadrecovery5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[contractid] varchar(10) null,
[periodid] decimal(3,0) not null,
[regionid] varchar(10) not null,
[enablingpayment] decimal(15,5) null,
[usagepayment] decimal(15,5) null,
[compensationpayment] decimal(15,5) null,
[participantdemand] decimal(15,5) null,
[regiondemand] decimal(15,5) null,
[enablingrecovery] decimal(15,5) null,
[usagerecovery] decimal(15,5) null,
[compensationrecovery] decimal(15,5) null,
[lastchanged] datetime2 null,
[enablingrecovery_gen] decimal(15,5) null,
[usagerecovery_gen] decimal(15,5) null,
[compensationrecovery_gen] decimal(15,5) null,
[participantdemand_gen] decimal(15,5) null,
[regiondemand_gen] decimal(15,5) null,

)
go
create clustered columnstore index cci_SettlementsLunloadrecovery5 on SettlementsLunloadrecovery5;
go
                        
create table SettlementsFcasPayment5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) null,
[duid] varchar(10) not null,
[regionid] varchar(10) null,
[periodid] decimal(3,0) not null,
[lower6sec_payment] decimal(18,8) null,
[raise6sec_payment] decimal(18,8) null,
[lower60sec_payment] decimal(18,8) null,
[raise60sec_payment] decimal(18,8) null,
[lower5min_payment] decimal(18,8) null,
[raise5min_payment] decimal(18,8) null,
[lowerreg_payment] decimal(18,8) null,
[raisereg_payment] decimal(18,8) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsFcasPayment5 on SettlementsFcasPayment5;
go
                        
create table SettlementsIntraregionresidues5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[periodid] decimal(3,0) not null,
[regionid] varchar(10) not null,
[ep] decimal(15,5) null,
[ec] decimal(15,5) null,
[rrp] decimal(15,5) null,
[exp] decimal(15,5) null,
[irss] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsIntraregionresidues5 on SettlementsIntraregionresidues5;
go
                        
create table SettlementsAgcpayment5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[contractid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[duid] varchar(10) null,
[regionid] varchar(10) null,
[tlf] decimal(7,5) null,
[ebp] decimal(15,5) null,
[rrp] decimal(15,5) null,
[clearedmw] decimal(15,5) null,
[initialmw] decimal(15,5) null,
[enablingpayment] decimal(15,5) null,
[contractversionno] decimal(3,0) null,
[offerdate] datetime2 null,
[offerversionno] decimal(3,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsAgcpayment5 on SettlementsAgcpayment5;
go
                        
create table SettlementsDaytrack5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[regionid] varchar(10) null,
[exanterunstatus] varchar(15) null,
[exanterunno] decimal(3,0) null,
[expostrunstatus] varchar(15) null,
[expostrunno] decimal(3,0) not null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsDaytrack5 on SettlementsDaytrack5;
go
                        
create table SettlementsIntervention5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[periodid] decimal(3,0) not null,
[contractid] varchar(10) null,
[contractversion] decimal(3,0) null,
[participantid] varchar(10) null,
[regionid] varchar(10) null,
[duid] varchar(10) not null,
[rcf] char(1) null,
[interventionpayment] decimal(12,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsIntervention5 on SettlementsIntervention5;
go
                        
create table SettlementsCpdata5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(10,0) not null,
[periodid] decimal(10,0) not null,
[participantid] varchar(10) not null,
[tcpid] varchar(10) not null,
[regionid] varchar(10) null,
[igenergy] decimal(16,6) null,
[xgenergy] decimal(16,6) null,
[inenergy] decimal(16,6) null,
[xnenergy] decimal(16,6) null,
[ipower] decimal(16,6) null,
[xpower] decimal(16,6) null,
[rrp] decimal(20,5) null,
[eep] decimal(16,6) null,
[tlf] decimal(7,5) null,
[cprrp] decimal(16,6) null,
[cpeep] decimal(16,6) null,
[ta] decimal(16,6) null,
[ep] decimal(16,6) null,
[apc] decimal(16,6) null,
[resc] decimal(16,6) null,
[resp] decimal(16,6) null,
[meterrunno] decimal(10,0) null,
[lastchanged] datetime2 null,
[hostdistributor] varchar(10) null,
[mda] varchar(10) not null,

)
go
create clustered columnstore index cci_SettlementsCpdata5 on SettlementsCpdata5;
go
                        
create table SettlementsVicboundaryenergy5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[boundaryenergy] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsVicboundaryenergy5 on SettlementsVicboundaryenergy5;
go
                        
create table SettlementsLuloadrecovery5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[contractid] varchar(10) null,
[periodid] decimal(3,0) not null,
[regionid] varchar(10) not null,
[enablingpayment] decimal(15,5) null,
[usagepayment] decimal(15,5) null,
[compensationpayment] decimal(15,5) null,
[participantdemand] decimal(15,5) null,
[regiondemand] decimal(15,5) null,
[enablingrecovery] decimal(15,5) null,
[usagerecovery] decimal(15,5) null,
[compensationrecovery] decimal(15,5) null,
[lastchanged] datetime2 null,
[enablingrecovery_gen] decimal(15,5) null,
[usagerecovery_gen] decimal(15,5) null,
[compensationrecovery_gen] decimal(15,5) null,
[participantdemand_gen] decimal(15,5) null,
[regiondemand_gen] decimal(15,5) null,

)
go
create clustered columnstore index cci_SettlementsLuloadrecovery5 on SettlementsLuloadrecovery5;
go
                        
create table SettlementsNmasRecovery2 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[periodid] decimal(3,0) not null,
[participantid] varchar(20) not null,
[service] varchar(10) not null,
[contractid] varchar(10) not null,
[paymenttype] varchar(20) not null,
[regionid] varchar(10) not null,
[rbf] decimal(18,8) null,
[payment_amount] decimal(18,8) null,
[participant_energy] decimal(18,8) null,
[region_energy] decimal(18,8) null,
[recovery_amount] decimal(18,8) null,
[lastchanged] datetime2 null,
[participant_generation] decimal(18,8) null,
[region_generation] decimal(18,8) null,
[recovery_amount_customer] decimal(18,8) null,
[recovery_amount_generator] decimal(18,8) null,

)
go
create clustered columnstore index cci_SettlementsNmasRecovery2 on SettlementsNmasRecovery2;
go
                        
create table SettlementsLshedpayment5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[contractid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[duid] varchar(10) null,
[regionid] varchar(10) null,
[tlf] decimal(7,5) null,
[rrp] decimal(15,5) null,
[lseprice] decimal(15,5) null,
[mcpprice] decimal(15,5) null,
[lscr] decimal(4,0) null,
[lsepayment] decimal(15,5) null,
[ccpayment] decimal(15,5) null,
[constrainedmw] decimal(15,5) null,
[unconstrainedmw] decimal(15,5) null,
[als] decimal(15,5) null,
[initialdemand] decimal(15,5) null,
[finaldemand] decimal(15,5) null,
[contractversionno] decimal(3,0) null,
[offerdate] datetime2 null,
[offerversionno] decimal(3,0) null,
[lastchanged] datetime2 null,
[availabilitypayment] decimal(16,6) null,

)
go
create clustered columnstore index cci_SettlementsLshedpayment5 on SettlementsLshedpayment5;
go
                        
create table SettlementsFcascomp5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[duid] varchar(10) not null,
[regionid] varchar(10) null,
[periodid] decimal(3,0) not null,
[ccprice] decimal(15,5) null,
[clearedmw] decimal(15,5) null,
[unconstrainedmw] decimal(15,5) null,
[ebp] decimal(15,5) null,
[tlf] decimal(7,5) null,
[rrp] decimal(15,5) null,
[excessgen] decimal(15,5) null,
[fcascomp] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsFcascomp5 on SettlementsFcascomp5;
go
                        
create table SettlementsFcasregionrecovery5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[bidtype] varchar(10) not null,
[regionid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[generatorregionenergy] decimal(16,6) null,
[customerregionenergy] decimal(16,6) null,
[regionrecovery] decimal(18,8) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsFcasregionrecovery5 on SettlementsFcasregionrecovery5;
go
                        
create table SettlementsIrfmrecovery5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[periodid] decimal(3,0) not null,
[irfmid] varchar(10) not null,
[irmfversion] decimal(3,0) null,
[participantid] varchar(10) not null,
[participantdemand] decimal(12,5) null,
[totaltcd] decimal(12,5) null,
[totaltfd] decimal(12,5) null,
[irfmamount] decimal(12,5) null,
[irfmpayment] decimal(12,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsIrfmrecovery5 on SettlementsIrfmrecovery5;
go
                        
create table SettlementsRestartpayment6 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[contractid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[regionid] varchar(10) null,
[restarttype] decimal(1,0) null,
[avaflag] decimal(1,0) null,
[availabilityprice] decimal(15,5) null,
[tcf] decimal(1,0) null,
[availabilitypayment] decimal(15,5) null,
[contractversionno] decimal(3,0) null,
[offerdate] datetime2 null,
[offerversionno] decimal(3,0) null,
[lastchanged] datetime2 null,
[enablingpayment] decimal(18,8) null,

)
go
create clustered columnstore index cci_SettlementsRestartpayment6 on SettlementsRestartpayment6;
go
                        
create table SettlementsMrPayment5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[regionid] varchar(10) not null,
[participantid] varchar(10) null,
[duid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[mr_capacity] decimal(16,6) null,
[uncapped_payment] decimal(16,6) null,
[capped_payment] decimal(16,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsMrPayment5 on SettlementsMrPayment5;
go
                        
create table SettlementsIrnspsurplus6 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[settlementrunno] decimal(3,0) not null,
[contractid] varchar(10) not null,
[periodid] decimal(2,0) not null,
[participantid] varchar(10) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[totalsurplus] decimal(15,5) null,
[contractallocation] decimal(8,5) null,
[surplusvalue] decimal(15,5) null,
[lastchanged] datetime2 null,
[csp_derogation_amount] decimal(18,8) null,
[unadjusted_irsr] decimal(18,8) null,

)
go
create clustered columnstore index cci_SettlementsIrnspsurplus6 on SettlementsIrnspsurplus6;
go
                        
create table SettlementsCpdataregion5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(22,10) not null,
[periodid] decimal(22,10) not null,
[regionid] varchar(10) not null,
[sumigenergy] decimal(27,5) null,
[sumxgenergy] decimal(27,5) null,
[suminenergy] decimal(27,5) null,
[sumxnenergy] decimal(27,5) null,
[sumipower] decimal(22,0) null,
[sumxpower] decimal(22,0) null,
[lastchanged] datetime2 null,
[sumep] decimal(15,5) null,

)
go
create clustered columnstore index cci_SettlementsCpdataregion5 on SettlementsCpdataregion5;
go
                        
create table SettlementsSmallgendata1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[connectionpointid] varchar(20) not null,
[periodid] decimal(3,0) not null,
[participantid] varchar(20) not null,
[regionid] varchar(20) null,
[importenergy] decimal(18,8) null,
[exportenergy] decimal(18,8) null,
[rrp] decimal(18,8) null,
[tlf] decimal(18,8) null,
[impenergycost] decimal(18,8) null,
[expenergycost] decimal(18,8) null,
[lastchanged] datetime2 null,
[raise60secreq] decimal(15,5) null,
[raise60secsupplyprice] decimal(15,5) null,
[raise6secdispatch] decimal(15,5) null,
[raise6secimport] decimal(15,5) null,
[raise6seclocaldispatch] decimal(15,5) null,
[raise6seclocalprice] decimal(15,5) null,
[raise6seclocalreq] decimal(15,5) null,
[raise6secprice] decimal(15,5) null,
[raise6secreq] decimal(15,5) null,
[raise6secsupplyprice] decimal(15,5) null,
[lastchanged] datetime2 null,
[initialsupply] decimal(15,5) null,
[clearedsupply] decimal(15,5) null,
[lowerregimport] decimal(15,5) null,
[lowerreglocaldispatch] decimal(15,5) null,
[lowerreglocalreq] decimal(15,5) null,
[lowerregreq] decimal(15,5) null,
[raiseregimport] decimal(15,5) null,
[raisereglocaldispatch] decimal(15,5) null,
[raisereglocalreq] decimal(15,5) null,
[raiseregreq] decimal(15,5) null,
[raise5minlocalviolation] decimal(15,5) null,
[raisereglocalviolation] decimal(15,5) null,
[raise60seclocalviolation] decimal(15,5) null,
[raise6seclocalviolation] decimal(15,5) null,
[lower5minlocalviolation] decimal(15,5) null,
[lowerreglocalviolation] decimal(15,5) null,
[lower60seclocalviolation] decimal(15,5) null,
[lower6seclocalviolation] decimal(15,5) null,
[raise5minviolation] decimal(15,5) null,
[raiseregviolation] decimal(15,5) null,
[raise60secviolation] decimal(15,5) null,
[raise6secviolation] decimal(15,5) null,
[lower5minviolation] decimal(15,5) null,
[lowerregviolation] decimal(15,5) null,
[lower60secviolation] decimal(15,5) null,
[lower6secviolation] decimal(15,5) null,
[totalintermittentgeneration] decimal(15,5) null,
[demand_and_nonschedgen] decimal(15,5) null,
[uigf] decimal(15,5) null,

)
go
create clustered columnstore index cci_SettlementsSmallgendata1 on SettlementsSmallgendata1;
go
                        
create table SettlementsSetFcasRegulationTrk1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[interval_datetime] datetime2 not null,
[constraintid] varchar(20) not null,
[cmpf] decimal(18,8) null,
[crmpf] decimal(18,8) null,
[recovery_factor_cmpf] decimal(18,8) null,
[recovery_factor_crmpf] decimal(18,8) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementsSetFcasRegulationTrk1 on SettlementsSetFcasRegulationTrk1;
go
                        
create table SettlementsIraucsurplus6 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[settlementrunno] decimal(3,0) not null,
[contractid] varchar(10) not null,
[periodid] decimal(2,0) not null,
[participantid] varchar(10) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[totalsurplus] decimal(15,5) null,
[contractallocation] decimal(8,5) null,
[surplusvalue] decimal(15,5) null,
[lastchanged] datetime2 null,
[csp_derogation_amount] decimal(18,8) null,
[unadjusted_irsr] decimal(18,8) null,

)
go
create clustered columnstore index cci_SettlementsIraucsurplus6 on SettlementsIraucsurplus6;
go
                        
create table SettlementsRpowerrecovery5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[contractid] varchar(10) null,
[periodid] decimal(3,0) not null,
[regionid] varchar(10) not null,
[availabilitypayment] decimal(15,5) null,
[enablingpayment] decimal(15,5) null,
[ccpayment] decimal(15,5) null,
[participantdemand] decimal(15,5) null,
[regiondemand] decimal(15,5) null,
[availabilityrecovery] decimal(15,5) null,
[enablingrecovery] decimal(15,5) null,
[ccrecovery] decimal(15,5) null,
[lastchanged] datetime2 null,
[availabilityrecovery_gen] decimal(15,5) null,
[enablingrecovery_gen] decimal(15,5) null,
[ccrecovery_gen] decimal(15,5) null,
[participantdemand_gen] decimal(15,5) null,
[regiondemand_gen] decimal(15,5) null,

)
go
create clustered columnstore index cci_SettlementsRpowerrecovery5 on SettlementsRpowerrecovery5;
go
                        
create table IrauctionConfigAuctionRpEstimate1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[quarter] decimal(1,0) not null,
[valuationid] varchar(15) not null,
[versionno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[rpestimate] decimal(17,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionConfigAuctionRpEstimate1 on IrauctionConfigAuctionRpEstimate1;
go
                        
create table IrauctionSraFinancialAucpaySum1 (
file_log_id bigint not null references FileLog(id),
[sra_year] decimal(4,0) not null,
[sra_quarter] decimal(3,0) not null,
[sra_runno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[gross_proceeds_amount] decimal(18,8) null,
[total_gross_proceeds_amount] decimal(18,8) null,
[shortfall_amount] decimal(18,8) null,
[total_shortfall_amount] decimal(18,8) null,
[net_payment_amount] decimal(18,8) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionSraFinancialAucpaySum1 on IrauctionSraFinancialAucpaySum1;
go
                        
create table IrauctionBidsFundsBid1 (
file_log_id bigint not null references FileLog(id),
[contractid] varchar(30) not null,
[participantid] varchar(10) not null,
[loaddate] datetime2 not null,
[optionid] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[units] decimal(5,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionBidsFundsBid1 on IrauctionBidsFundsBid1;
go
                        
create table IrauctionResiduePublicData1 (
file_log_id bigint not null references FileLog(id),
[contractid] varchar(30) not null,
[versionno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[unitsoffered] decimal(5,0) null,
[unitssold] decimal(16,6) null,
[clearingprice] decimal(17,5) null,
[reserveprice] decimal(17,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionResiduePublicData1 on IrauctionResiduePublicData1;
go
                        
create table IrauctionSraFinancialAucReceipts1 (
file_log_id bigint not null references FileLog(id),
[sra_year] decimal(4,0) not null,
[sra_quarter] decimal(3,0) not null,
[sra_runno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[contractid] varchar(10) not null,
[units_purchased] decimal(18,8) null,
[clearing_price] decimal(18,8) null,
[receipt_amount] decimal(18,8) null,
[lastchanged] datetime2 null,
[proceeds_amount] decimal(18,8) null,
[units_sold] decimal(18,8) null,

)
go
create clustered columnstore index cci_IrauctionSraFinancialAucReceipts1 on IrauctionSraFinancialAucReceipts1;
go
                        
create table IrauctionConfigAuctionTranche1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[quarter] decimal(1,0) not null,
[versionno] decimal(3,0) not null,
[tranche] decimal(2,0) not null,
[auctiondate] datetime2 null,
[notifydate] datetime2 null,
[unitallocation] decimal(18,8) null,
[changedate] datetime2 null,
[changedby] varchar(15) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionConfigAuctionTranche1 on IrauctionConfigAuctionTranche1;
go
                        
create table IrauctionSraFinancialAucMargin1 (
file_log_id bigint not null references FileLog(id),
[sra_year] decimal(4,0) not null,
[sra_quarter] decimal(3,0) not null,
[sra_runno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[total_cash_security] decimal(18,8) null,
[required_margin] decimal(18,8) null,
[returned_margin] decimal(18,8) null,
[returned_margin_interest] decimal(18,8) null,

)
go
create clustered columnstore index cci_IrauctionSraFinancialAucMargin1 on IrauctionSraFinancialAucMargin1;
go
                        
create table SettlementConfigResiduecontractpayments1 (
file_log_id bigint not null references FileLog(id),
[contractid] varchar(30) not null,
[participantid] varchar(10) not null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementConfigResiduecontractpayments1 on SettlementConfigResiduecontractpayments1;
go
                        
create table IrauctionResidueContracts1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[quarter] decimal(1,0) not null,
[tranche] decimal(2,0) not null,
[contractid] varchar(30) null,
[startdate] datetime2 null,
[enddate] datetime2 null,
[notifydate] datetime2 null,
[auctiondate] datetime2 null,
[calcmethod] varchar(20) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[notifypostdate] datetime2 null,
[notifyby] varchar(15) null,
[postdate] datetime2 null,
[postedby] varchar(15) null,
[lastchanged] datetime2 null,
[description] varchar(80) null,
[auctionid] varchar(30) null,

)
go
create clustered columnstore index cci_IrauctionResidueContracts1 on IrauctionResidueContracts1;
go
                        
create table IrauctionValuationid1 (
file_log_id bigint not null references FileLog(id),
[valuationid] varchar(15) not null,
[description] varchar(80) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionValuationid1 on IrauctionValuationid1;
go
                        
create table IrauctionConfigAuctionRevenueTrack1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[quarter] decimal(1,0) not null,
[valuationid] varchar(15) not null,
[versionno] decimal(3,0) not null,
[effectivedate] datetime2 null,
[status] varchar(10) null,
[documentref] varchar(30) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionConfigAuctionRevenueTrack1 on IrauctionConfigAuctionRevenueTrack1;
go
                        
create table IrauctionResidueConData2 (
file_log_id bigint not null references FileLog(id),
[contractid] varchar(30) not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[unitspurchased] decimal(17,5) null,
[linkpayment] decimal(17,5) null,
[lastchanged] datetime2 null,
[secondary_units_sold] decimal(18,8) null,

)
go
create clustered columnstore index cci_IrauctionResidueConData2 on IrauctionResidueConData2;
go
                        
create table IrauctionBidsFileTrk1 (
file_log_id bigint not null references FileLog(id),
[contractid] varchar(30) null,
[participantid] varchar(10) not null,
[loaddate] datetime2 not null,
[filename] varchar(40) null,
[ackfilename] varchar(40) null,
[status] varchar(10) null,
[lastchanged] datetime2 null,
[auctionid] varchar(30) not null,

)
go
create clustered columnstore index cci_IrauctionBidsFileTrk1 on IrauctionBidsFileTrk1;
go
                        
create table IrauctionSraFinancialAucpayDetail1 (
file_log_id bigint not null references FileLog(id),
[sra_year] decimal(4,0) not null,
[sra_quarter] decimal(3,0) not null,
[sra_runno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[contractid] varchar(10) not null,
[maximum_units] decimal(18,8) null,
[units_sold] decimal(18,8) null,
[shortfall_units] decimal(18,8) null,
[reserve_price] decimal(18,8) null,
[clearing_price] decimal(18,8) null,
[payment_amount] decimal(18,8) null,
[shortfall_amount] decimal(18,8) null,
[allocation] decimal(18,8) null,
[net_payment_amount] decimal(18,8) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionSraFinancialAucpayDetail1 on IrauctionSraFinancialAucpayDetail1;
go
                        
create table IrauctionConfigAuctionRevenueEstimate1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[quarter] decimal(1,0) not null,
[valuationid] varchar(15) not null,
[versionno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[monthno] decimal(1,0) not null,
[startdate] datetime2 null,
[enddate] datetime2 null,
[revenue] decimal(17,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionConfigAuctionRevenueEstimate1 on IrauctionConfigAuctionRevenueEstimate1;
go
                        
create table IrauctionSraFinancialAucMardetail1 (
file_log_id bigint not null references FileLog(id),
[sra_year] decimal(4,0) not null,
[sra_quarter] decimal(3,0) not null,
[sra_runno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[cash_security_id] varchar(36) not null,
[returned_amount] decimal(18,8) null,
[returned_interest] decimal(18,8) null,

)
go
create clustered columnstore index cci_IrauctionSraFinancialAucMardetail1 on IrauctionSraFinancialAucMardetail1;
go
                        
create table IrauctionSraPrudentialExposure1 (
file_log_id bigint not null references FileLog(id),
[prudential_date] datetime2 not null,
[prudential_runno] decimal(8,0) not null,
[participantid] varchar(10) not null,
[sra_year] decimal(4,0) not null,
[sra_quarter] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[max_tranche] decimal(2,0) null,
[auctionid] varchar(30) null,
[offer_submissiontime] datetime2 null,
[average_purchase_price] decimal(18,8) null,
[average_cancellation_price] decimal(18,8) null,
[cancellation_volume] decimal(18,8) null,
[trading_position] decimal(18,8) null,

)
go
create clustered columnstore index cci_IrauctionSraPrudentialExposure1 on IrauctionSraPrudentialExposure1;
go
                        
create table IrauctionSraCashSecurity1 (
file_log_id bigint not null references FileLog(id),
[cash_security_id] varchar(36) not null,
[participantid] varchar(10) null,
[provision_date] datetime2 null,
[cash_amount] decimal(18,8) null,
[interest_acct_id] varchar(20) null,
[authoriseddate] datetime2 null,
[finalreturndate] datetime2 null,
[cash_security_returned] decimal(18,8) null,
[deletiondate] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionSraCashSecurity1 on IrauctionSraCashSecurity1;
go
                        
create table IrauctionSraPrudentialRun1 (
file_log_id bigint not null references FileLog(id),
[prudential_date] datetime2 not null,
[prudential_runno] decimal(8,0) not null,

)
go
create clustered columnstore index cci_IrauctionSraPrudentialRun1 on IrauctionSraPrudentialRun1;
go
                        
create table IrauctionConfigAuction1 (
file_log_id bigint not null references FileLog(id),
[auctionid] varchar(30) not null,
[auctiondate] datetime2 null,
[notifydate] datetime2 null,
[startdate] datetime2 null,
[enddate] datetime2 null,
[description] varchar(100) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(30) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionConfigAuction1 on IrauctionConfigAuction1;
go
                        
create table IrauctionSraPrudentialCompPosition1 (
file_log_id bigint not null references FileLog(id),
[prudential_date] datetime2 not null,
[prudential_runno] decimal(8,0) not null,
[participantid] varchar(10) not null,
[trading_limit] decimal(18,8) null,
[prudential_exposure_amount] decimal(18,8) null,
[trading_margin] decimal(18,8) null,

)
go
create clustered columnstore index cci_IrauctionSraPrudentialCompPosition1 on IrauctionSraPrudentialCompPosition1;
go
                        
create table IrauctionConfigAuctionIcAllocations2 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[quarter] decimal(1,0) not null,
[versionno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[maximumunits] decimal(5,0) null,
[proportion] decimal(8,5) null,
[auctionfee] decimal(17,5) null,
[changedate] datetime2 null,
[changedby] varchar(15) null,
[lastchanged] datetime2 null,
[auctionfee_sales] decimal(18,8) null,

)
go
create clustered columnstore index cci_IrauctionConfigAuctionIcAllocations2 on IrauctionConfigAuctionIcAllocations2;
go
                        
create table IrauctionResidueBidTrk1 (
file_log_id bigint not null references FileLog(id),
[contractid] varchar(30) null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[bidloaddate] datetime2 null,
[lastchanged] datetime2 null,
[auctionid] varchar(30) not null,

)
go
create clustered columnstore index cci_IrauctionResidueBidTrk1 on IrauctionResidueBidTrk1;
go
                        
create table IrauctionResidueTrk1 (
file_log_id bigint not null references FileLog(id),
[contractid] varchar(30) null,
[versionno] decimal(3,0) not null,
[rundate] datetime2 null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[postdate] datetime2 null,
[postedby] varchar(15) null,
[lastchanged] datetime2 null,
[status] varchar(15) null,
[auctionid] varchar(30) not null,

)
go
create clustered columnstore index cci_IrauctionResidueTrk1 on IrauctionResidueTrk1;
go
                        
create table IrauctionResidueConFunds1 (
file_log_id bigint not null references FileLog(id),
[contractid] varchar(30) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[defaultunits] decimal(5,0) null,
[rolloverunits] decimal(5,0) null,
[reallocatedunits] decimal(5,0) null,
[unitsoffered] decimal(5,0) null,
[meanreserveprice] decimal(9,2) null,
[scalefactor] decimal(8,5) null,
[actualreserveprice] decimal(9,2) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionResidueConFunds1 on IrauctionResidueConFunds1;
go
                        
create table IrauctionResidueConEstimatesTrk1 (
file_log_id bigint not null references FileLog(id),
[contractid] varchar(30) not null,
[contractyear] decimal(4,0) not null,
[quarter] decimal(1,0) not null,
[valuationid] varchar(15) not null,
[versionno] decimal(3,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionResidueConEstimatesTrk1 on IrauctionResidueConEstimatesTrk1;
go
                        
create table IrauctionResiduePriceFundsBid1 (
file_log_id bigint not null references FileLog(id),
[contractid] varchar(30) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[units] decimal(5,0) null,
[bidprice] decimal(17,5) null,
[linkedbidflag] decimal(6,0) not null,
[auctionid] varchar(30) not null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionResiduePriceFundsBid1 on IrauctionResiduePriceFundsBid1;
go
                        
create table IrauctionSraOfferProduct1 (
file_log_id bigint not null references FileLog(id),
[auctionid] varchar(30) not null,
[participantid] varchar(10) not null,
[loaddate] datetime2 not null,
[optionid] decimal(4,0) not null,
[interconnectorid] varchar(10) null,
[fromregionid] varchar(10) null,
[offer_quantity] decimal(5,0) null,
[offer_price] decimal(18,8) null,
[trancheid] varchar(30) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionSraOfferProduct1 on IrauctionSraOfferProduct1;
go
                        
create table IrauctionConfigAuctionCalendar2 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[quarter] decimal(1,0) not null,
[startdate] datetime2 null,
[enddate] datetime2 null,
[notifydate] datetime2 null,
[paymentdate] datetime2 null,
[reconciliationdate] datetime2 null,
[lastchanged] datetime2 null,
[prelimpurchasestmtdate] datetime2 null,
[prelimproceedsstmtdate] datetime2 null,
[finalpurchasestmtdate] datetime2 null,
[finalproceedsstmtdate] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionConfigAuctionCalendar2 on IrauctionConfigAuctionCalendar2;
go
                        
create table IrauctionSraPrudentialCashSecurity1 (
file_log_id bigint not null references FileLog(id),
[prudential_date] datetime2 not null,
[prudential_runno] decimal(8,0) not null,
[participantid] varchar(10) not null,
[cash_security_id] varchar(36) not null,
[cash_security_amount] decimal(18,8) null,

)
go
create clustered columnstore index cci_IrauctionSraPrudentialCashSecurity1 on IrauctionSraPrudentialCashSecurity1;
go
                        
create table IrauctionSraOfferProfile1 (
file_log_id bigint not null references FileLog(id),
[auctionid] varchar(30) not null,
[participantid] varchar(10) not null,
[loaddate] datetime2 not null,
[filename] varchar(40) null,
[ackfilename] varchar(40) null,
[transactionid] varchar(100) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionSraOfferProfile1 on IrauctionSraOfferProfile1;
go
                        
create table IrauctionBidsPriceBid1 (
file_log_id bigint not null references FileLog(id),
[contractid] varchar(30) null,
[participantid] varchar(10) not null,
[loaddate] datetime2 not null,
[optionid] decimal(3,0) not null,
[bidprice] decimal(17,5) null,
[lastchanged] datetime2 null,
[auctionid] varchar(30) not null,

)
go
create clustered columnstore index cci_IrauctionBidsPriceBid1 on IrauctionBidsPriceBid1;
go
                        
create table IrauctionSraFinancialRuntrk1 (
file_log_id bigint not null references FileLog(id),
[sra_year] decimal(4,0) not null,
[sra_quarter] decimal(3,0) not null,
[sra_runno] decimal(3,0) not null,
[runtype] varchar(20) null,
[rundate] datetime2 null,
[posteddate] datetime2 null,
[interest_versionno] decimal(3,0) null,
[makeup_versionno] decimal(3,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_IrauctionSraFinancialRuntrk1 on IrauctionSraFinancialRuntrk1;
go
                        
create table OfferBidofferfiletrk1 (
file_log_id bigint not null references FileLog(id),
[participantid] varchar(10) not null,
[offerdate] datetime2 not null,
[filename] varchar(80) not null,
[status] varchar(10) null,
[lastchanged] datetime2 null,
[authorisedby] varchar(20) null,
[authoriseddate] datetime2 null,

)
go
create clustered columnstore index cci_OfferBidofferfiletrk1 on OfferBidofferfiletrk1;
go
                        
create table BidMnspFiletrk1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[offerdate] datetime2 not null,
[participantid] varchar(10) not null,
[filename] varchar(40) not null,
[status] varchar(10) null,
[ackfilename] varchar(40) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BidMnspFiletrk1 on BidMnspFiletrk1;
go
                        
create table BidBidperofferD2 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[duid] varchar(10) not null,
[bidtype] varchar(10) not null,
[bidsettlementdate] datetime2 null,
[offerdate] datetime2 null,
[periodid] decimal(22,0) null,
[versionno] decimal(22,0) null,
[maxavail] decimal(12,6) null,
[fixedload] decimal(12,6) null,
[rocup] decimal(6,0) null,
[rocdown] decimal(6,0) null,
[enablementmin] decimal(6,0) null,
[enablementmax] decimal(6,0) null,
[lowbreakpoint] decimal(6,0) null,
[highbreakpoint] decimal(6,0) null,
[bandavail1] decimal(22,0) null,
[bandavail2] decimal(22,0) null,
[bandavail3] decimal(22,0) null,
[bandavail4] decimal(22,0) null,
[bandavail5] decimal(22,0) null,
[bandavail6] decimal(22,0) null,
[bandavail7] decimal(22,0) null,
[bandavail8] decimal(22,0) null,
[bandavail9] decimal(22,0) null,
[bandavail10] decimal(22,0) null,
[lastchanged] datetime2 null,
[pasaavailability] decimal(12,0) null,
[interval_datetime] datetime2 not null,
[mr_capacity] decimal(6,0) null,

)
go
create clustered columnstore index cci_BidBidperofferD2 on BidBidperofferD2;
go
                        
create table OfferMnspOffertrk1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[offerdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[filename] varchar(40) not null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_OfferMnspOffertrk1 on OfferMnspOffertrk1;
go
                        
create table BidBiddayofferD2 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[duid] varchar(10) not null,
[bidtype] varchar(10) not null,
[bidsettlementdate] datetime2 null,
[offerdate] datetime2 null,
[versionno] decimal(22,0) null,
[participantid] varchar(10) null,
[dailyenergyconstraint] decimal(12,6) null,
[rebidexplanation] varchar(500) null,
[priceband1] decimal(9,2) null,
[priceband2] decimal(9,2) null,
[priceband3] decimal(9,2) null,
[priceband4] decimal(9,2) null,
[priceband5] decimal(9,2) null,
[priceband6] decimal(9,2) null,
[priceband7] decimal(9,2) null,
[priceband8] decimal(9,2) null,
[priceband9] decimal(9,2) null,
[priceband10] decimal(9,2) null,
[minimumload] decimal(22,0) null,
[t1] decimal(22,0) null,
[t2] decimal(22,0) null,
[t3] decimal(22,0) null,
[t4] decimal(22,0) null,
[normalstatus] varchar(3) null,
[lastchanged] datetime2 null,
[mr_factor] decimal(16,6) null,
[entrytype] varchar(20) null,

)
go
create clustered columnstore index cci_BidBiddayofferD2 on BidBiddayofferD2;
go
                        
create table OfferMnspDayoffer2 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[offerdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[linkid] varchar(10) not null,
[entrytype] varchar(20) null,
[rebidexplanation] varchar(500) null,
[priceband1] decimal(9,2) null,
[priceband2] decimal(9,2) null,
[priceband3] decimal(9,2) null,
[priceband4] decimal(9,2) null,
[priceband5] decimal(9,2) null,
[priceband6] decimal(9,2) null,
[priceband7] decimal(9,2) null,
[priceband8] decimal(9,2) null,
[priceband9] decimal(9,2) null,
[priceband10] decimal(9,2) null,
[lastchanged] datetime2 null,
[mr_factor] decimal(16,6) null,

)
go
create clustered columnstore index cci_OfferMnspDayoffer2 on OfferMnspDayoffer2;
go
                        
create table OfferMnspPeroffer1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[offerdate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[linkid] varchar(10) not null,
[periodid] decimal(22,0) not null,
[maxavail] decimal(6,0) null,
[bandavail1] decimal(6,0) null,
[bandavail2] decimal(6,0) null,
[bandavail3] decimal(6,0) null,
[bandavail4] decimal(6,0) null,
[bandavail5] decimal(6,0) null,
[bandavail6] decimal(6,0) null,
[bandavail7] decimal(6,0) null,
[bandavail8] decimal(6,0) null,
[bandavail9] decimal(6,0) null,
[bandavail10] decimal(6,0) null,
[lastchanged] datetime2 null,
[fixedload] decimal(12,6) null,
[rampuprate] decimal(6,0) null,
[pasaavailability] decimal(12,0) null,
[mr_capacity] decimal(6,0) null,

)
go
create clustered columnstore index cci_OfferMnspPeroffer1 on OfferMnspPeroffer1;
go
                        
create table OfferBidperoffer1 (
file_log_id bigint not null references FileLog(id),
[duid] varchar(10) not null,
[bidtype] varchar(10) not null,
[settlementdate] datetime2 not null,
[offerdate] datetime2 not null,
[periodid] decimal(22,0) not null,
[versionno] decimal(22,0) null,
[maxavail] decimal(12,6) null,
[fixedload] decimal(12,6) null,
[rocup] decimal(6,0) null,
[rocdown] decimal(6,0) null,
[enablementmin] decimal(6,0) null,
[enablementmax] decimal(6,0) null,
[lowbreakpoint] decimal(6,0) null,
[highbreakpoint] decimal(6,0) null,
[bandavail1] decimal(22,0) null,
[bandavail2] decimal(22,0) null,
[bandavail3] decimal(22,0) null,
[bandavail4] decimal(22,0) null,
[bandavail5] decimal(22,0) null,
[bandavail6] decimal(22,0) null,
[bandavail7] decimal(22,0) null,
[bandavail8] decimal(22,0) null,
[bandavail9] decimal(22,0) null,
[bandavail10] decimal(22,0) null,
[lastchanged] datetime2 null,
[pasaavailability] decimal(12,0) null,
[mr_capacity] decimal(6,0) null,

)
go
create clustered columnstore index cci_OfferBidperoffer1 on OfferBidperoffer1;
go
                        
create table OfferMtpasaOfferfiletrk1 (
file_log_id bigint not null references FileLog(id),
[participantid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[filename] varchar(200) null,

)
go
create clustered columnstore index cci_OfferMtpasaOfferfiletrk1 on OfferMtpasaOfferfiletrk1;
go
                        
create table OfferBiddayoffer2 (
file_log_id bigint not null references FileLog(id),
[duid] varchar(10) not null,
[bidtype] varchar(10) not null,
[settlementdate] datetime2 not null,
[offerdate] datetime2 not null,
[versionno] decimal(22,0) null,
[participantid] varchar(10) null,
[dailyenergyconstraint] decimal(12,6) null,
[rebidexplanation] varchar(500) null,
[priceband1] decimal(9,2) null,
[priceband2] decimal(9,2) null,
[priceband3] decimal(9,2) null,
[priceband4] decimal(9,2) null,
[priceband5] decimal(9,2) null,
[priceband6] decimal(9,2) null,
[priceband7] decimal(9,2) null,
[priceband8] decimal(9,2) null,
[priceband9] decimal(9,2) null,
[priceband10] decimal(9,2) null,
[minimumload] decimal(22,0) null,
[t1] decimal(22,0) null,
[t2] decimal(22,0) null,
[t3] decimal(22,0) null,
[t4] decimal(22,0) null,
[normalstatus] varchar(3) null,
[lastchanged] datetime2 null,
[mr_factor] decimal(16,6) null,
[entrytype] varchar(20) null,

)
go
create clustered columnstore index cci_OfferBiddayoffer2 on OfferBiddayoffer2;
go
                        
create table OfferMtpasaOfferdata1 (
file_log_id bigint not null references FileLog(id),
[participantid] varchar(20) not null,
[offerdatetime] datetime2 not null,
[unitid] varchar(20) not null,
[effectivedate] datetime2 not null,
[energy] decimal(9,0) null,
[capacity1] decimal(9,0) null,
[capacity2] decimal(9,0) null,
[capacity3] decimal(9,0) null,
[capacity4] decimal(9,0) null,
[capacity5] decimal(9,0) null,
[capacity6] decimal(9,0) null,
[capacity7] decimal(9,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_OfferMtpasaOfferdata1 on OfferMtpasaOfferdata1;
go
                        
create table MeterdataIndividualReads1 (
file_log_id bigint not null references FileLog(id),
[case_id] decimal(15,0) not null,
[settlementdate] datetime2 not null,
[meter_id] varchar(20) not null,
[meter_id_suffix] varchar(20) not null,
[frmp] varchar(20) not null,
[lr] varchar(20) not null,
[periodid] decimal(3,0) not null,
[connectionpointid] varchar(20) not null,
[meter_type] varchar(20) not null,
[importvalue] decimal(18,8) not null,
[exportvalue] decimal(18,8) not null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MeterdataIndividualReads1 on MeterdataIndividualReads1;
go
                        
create table MeterdataTrk1 (
file_log_id bigint not null references FileLog(id),
[case_id] decimal(15,0) not null,
[aggregate_reads_load_datetime] datetime2 null,
[individual_reads_load_datetime] datetime2 null,
[startdate] datetime2 null,
[enddate] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MeterdataTrk1 on MeterdataTrk1;
go
                        
create table MeterdataAggregateReads1 (
file_log_id bigint not null references FileLog(id),
[case_id] decimal(15,0) not null,
[settlementdate] datetime2 not null,
[connectionpointid] varchar(20) not null,
[meter_type] varchar(20) not null,
[frmp] varchar(20) not null,
[lr] varchar(20) not null,
[periodid] decimal(3,0) not null,
[importvalue] decimal(18,8) not null,
[exportvalue] decimal(18,8) not null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MeterdataAggregateReads1 on MeterdataAggregateReads1;
go
                        
create table MeterdataInterconnector1 (
file_log_id bigint not null references FileLog(id),
[case_id] decimal(15,0) not null,
[settlementdate] datetime2 not null,
[interconnectorid] varchar(20) not null,
[periodid] decimal(3,0) not null,
[importvalue] decimal(18,8) null,
[exportvalue] decimal(18,8) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MeterdataInterconnector1 on MeterdataInterconnector1;
go
                        
create table PrudentialRuntrk1 (
file_log_id bigint not null references FileLog(id),
[prudential_date] datetime2 not null,
[runno] decimal(3,0) not null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_PrudentialRuntrk1 on PrudentialRuntrk1;
go
                        
create table PrudentialCompanyPosition1 (
file_log_id bigint not null references FileLog(id),
[prudential_date] datetime2 not null,
[runno] decimal(3,0) not null,
[company_id] varchar(20) not null,
[mcl] decimal(16,6) null,
[credit_support] decimal(16,6) null,
[trading_limit] decimal(16,6) null,
[current_amount_balance] decimal(16,6) null,
[security_deposit_provision] decimal(16,6) null,
[security_deposit_offset] decimal(16,6) null,
[security_deposit_balance] decimal(16,6) null,
[expost_realloc_balance] decimal(16,6) null,
[default_balance] decimal(16,6) null,
[outstandings] decimal(16,6) null,
[trading_margin] decimal(16,6) null,
[typical_accrual] decimal(16,6) null,
[prudential_margin] decimal(16,6) null,
[early_payment_amount] decimal(18,8) null,
[percentage_outstandings] decimal(18,8) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_PrudentialCompanyPosition1 on PrudentialCompanyPosition1;
go
                        
create table TradingInterconnectorres2 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[meteredmwflow] decimal(15,5) null,
[mwflow] decimal(15,5) null,
[mwlosses] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_TradingInterconnectorres2 on TradingInterconnectorres2;
go
                        
create table TradingRegionsum4 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[regionid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[totaldemand] decimal(15,5) null,
[availablegeneration] decimal(15,5) null,
[availableload] decimal(15,5) null,
[demandforecast] decimal(15,5) null,
[dispatchablegeneration] decimal(15,5) null,
[dispatchableload] decimal(15,5) null,
[netinterchange] decimal(15,5) null,
[excessgeneration] decimal(15,5) null,
[lower5mindispatch] decimal(15,5) null,
[lower5minimport] decimal(15,5) null,
[lower5minlocaldispatch] decimal(15,5) null,
[lower5minlocalprice] decimal(15,5) null,
[lower5minlocalreq] decimal(15,5) null,
[lower5minprice] decimal(15,5) null,
[lower5minreq] decimal(15,5) null,
[lower5minsupplyprice] decimal(15,5) null,
[lower60secdispatch] decimal(15,5) null,
[lower60secimport] decimal(15,5) null,
[lower60seclocaldispatch] decimal(15,5) null,
[lower60seclocalprice] decimal(15,5) null,
[lower60seclocalreq] decimal(15,5) null,
[lower60secprice] decimal(15,5) null,
[lower60secreq] decimal(15,5) null,
[lower60secsupplyprice] decimal(15,5) null,
[lower6secdispatch] decimal(15,5) null,
[lower6secimport] decimal(15,5) null,
[lower6seclocaldispatch] decimal(15,5) null,
[lower6seclocalprice] decimal(15,5) null,
[lower6seclocalreq] decimal(15,5) null,
[lower6secprice] decimal(15,5) null,
[lower6secreq] decimal(15,5) null,
[lower6secsupplyprice] decimal(15,5) null,
[raise5mindispatch] decimal(15,5) null,
[raise5minimport] decimal(15,5) null,
[raise5minlocaldispatch] decimal(15,5) null,
[raise5minlocalprice] decimal(15,5) null,
[raise5minlocalreq] decimal(15,5) null,
[raise5minprice] decimal(15,5) null,
[raise5minreq] decimal(15,5) null,
[raise5minsupplyprice] decimal(15,5) null,
[raise60secdispatch] decimal(15,5) null,
[raise60secimport] decimal(15,5) null,
[raise60seclocaldispatch] decimal(15,5) null,
[raise60seclocalprice] decimal(15,5) null,
[raise60seclocalreq] decimal(15,5) null,
[raise60secprice] decimal(15,5) null,
[raise60secreq] decimal(15,5) null,
[raise60secsupplyprice] decimal(15,5) null,
[raise6secdispatch] decimal(15,5) null,
[raise6secimport] decimal(15,5) null,
[raise6seclocaldispatch] decimal(15,5) null,
[raise6seclocalprice] decimal(15,5) null,
[raise6seclocalreq] decimal(15,5) null,
[raise6secprice] decimal(15,5) null,
[raise6secreq] decimal(15,5) null,
[raise6secsupplyprice] decimal(15,5) null,
[lastchanged] datetime2 null,
[initialsupply] decimal(15,5) null,
[clearedsupply] decimal(15,5) null,
[lowerregimport] decimal(15,5) null,
[lowerreglocaldispatch] decimal(15,5) null,
[lowerreglocalreq] decimal(15,5) null,
[lowerregreq] decimal(15,5) null,
[raiseregimport] decimal(15,5) null,
[raisereglocaldispatch] decimal(15,5) null,
[raisereglocalreq] decimal(15,5) null,
[raiseregreq] decimal(15,5) null,
[raise5minlocalviolation] decimal(15,5) null,
[raisereglocalviolation] decimal(15,5) null,
[raise60seclocalviolation] decimal(15,5) null,
[raise6seclocalviolation] decimal(15,5) null,
[lower5minlocalviolation] decimal(15,5) null,
[lowerreglocalviolation] decimal(15,5) null,
[lower60seclocalviolation] decimal(15,5) null,
[lower6seclocalviolation] decimal(15,5) null,
[raise5minviolation] decimal(15,5) null,
[raiseregviolation] decimal(15,5) null,
[raise60secviolation] decimal(15,5) null,
[raise6secviolation] decimal(15,5) null,
[lower5minviolation] decimal(15,5) null,
[lowerregviolation] decimal(15,5) null,
[lower60secviolation] decimal(15,5) null,
[lower6secviolation] decimal(15,5) null,
[totalintermittentgeneration] decimal(15,5) null,
[demand_and_nonschedgen] decimal(15,5) null,
[uigf] decimal(15,5) null,

)
go
create clustered columnstore index cci_TradingRegionsum4 on TradingRegionsum4;
go
                        
create table TradingPrice2 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[regionid] varchar(10) not null,
[periodid] decimal(3,0) not null,
[rrp] decimal(15,5) null,
[eep] decimal(15,5) null,
[invalidflag] varchar(1) null,
[lastchanged] datetime2 null,
[rop] decimal(15,5) null,
[raise6secrrp] decimal(15,5) null,
[raise6secrop] decimal(15,5) null,
[raise60secrrp] decimal(15,5) null,
[raise60secrop] decimal(15,5) null,
[raise5minrrp] decimal(15,5) null,
[raise5minrop] decimal(15,5) null,
[raiseregrrp] decimal(15,5) null,
[raiseregrop] decimal(15,5) null,
[lower6secrrp] decimal(15,5) null,
[lower6secrop] decimal(15,5) null,
[lower60secrrp] decimal(15,5) null,
[lower60secrop] decimal(15,5) null,
[lower5minrrp] decimal(15,5) null,
[lower5minrop] decimal(15,5) null,
[lowerregrrp] decimal(15,5) null,
[lowerregrop] decimal(15,5) null,
[price_status] varchar(20) null,

)
go
create clustered columnstore index cci_TradingPrice2 on TradingPrice2;
go
                        
create table TradingUnitSolution2 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[duid] varchar(10) not null,
[tradetype] decimal(2,0) not null,
[periodid] decimal(3,0) not null,
[initialmw] decimal(15,5) null,
[totalcleared] decimal(15,5) null,
[rampdownrate] decimal(15,5) null,
[rampuprate] decimal(15,5) null,
[lower5min] decimal(15,5) null,
[lower60sec] decimal(15,5) null,
[lower6sec] decimal(15,5) null,
[raise5min] decimal(15,5) null,
[raise60sec] decimal(15,5) null,
[raise6sec] decimal(15,5) null,
[lastchanged] datetime2 null,
[lowerreg] decimal(15,5) null,
[raisereg] decimal(15,5) null,
[availability] decimal(15,5) null,
[semidispatchcap] decimal(3,0) null,

)
go
create clustered columnstore index cci_TradingUnitSolution2 on TradingUnitSolution2;
go
                        
create table StpasaRegionsolution5 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[interval_datetime] datetime2 not null,
[regionid] varchar(10) not null,
[demand10] decimal(12,2) null,
[demand50] decimal(12,2) null,
[demand90] decimal(12,2) null,
[reservereq] decimal(12,2) null,
[capacityreq] decimal(12,2) null,
[energyreqdemand50] decimal(12,2) null,
[unconstrainedcapacity] decimal(12,0) null,
[constrainedcapacity] decimal(12,0) null,
[netinterchangeunderscarcity] decimal(12,2) null,
[surpluscapacity] decimal(12,2) null,
[surplusreserve] decimal(12,2) null,
[reservecondition] decimal(1,0) null,
[maxsurplusreserve] decimal(12,2) null,
[maxsparecapacity] decimal(12,2) null,
[lorcondition] decimal(1,0) null,
[aggregatecapacityavailable] decimal(12,2) null,
[aggregatescheduledload] decimal(12,2) null,
[lastchanged] datetime2 null,
[aggregatepasaavailability] decimal(12,0) null,
[runtype] varchar(20) not null,
[energyreqdemand10] decimal(12,2) null,
[calculatedlor1level] decimal(16,6) null,
[calculatedlor2level] decimal(16,6) null,
[msrnetinterchangeunderscarcity] decimal(12,2) null,
[lornetinterchangeunderscarcity] decimal(12,2) null,
[totalintermittentgeneration] decimal(15,5) null,
[demand_and_nonschedgen] decimal(15,5) null,
[uigf] decimal(12,2) null,
[semi_scheduled_capacity] decimal(12,2) null,
[lor_semi_scheduled_capacity] decimal(12,2) null,
[lcr] decimal(16,6) null,
[lcr2] decimal(16,6) null,
[fum] decimal(16,6) null,
[ss_solar_uigf] decimal(12,2) null,
[ss_wind_uigf] decimal(12,2) null,
[ss_solar_capacity] decimal(12,2) null,
[ss_wind_capacity] decimal(12,2) null,
[ss_solar_cleared] decimal(12,2) null,
[ss_wind_cleared] decimal(12,2) null,

)
go
create clustered columnstore index cci_StpasaRegionsolution5 on StpasaRegionsolution5;
go
                        
create table StpasaInterconnectorsoln2 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[interval_datetime] datetime2 not null,
[interconnectorid] varchar(10) not null,
[capacitymwflow] decimal(12,2) null,
[capacitymarginalvalue] decimal(12,2) null,
[capacityviolationdegree] decimal(12,2) null,
[calculatedexportlimit] decimal(12,2) null,
[calculatedimportlimit] decimal(12,2) null,
[lastchanged] datetime2 null,
[runtype] varchar(20) not null,
[exportlimitconstraintid] varchar(20) null,
[importlimitconstraintid] varchar(20) null,

)
go
create clustered columnstore index cci_StpasaInterconnectorsoln2 on StpasaInterconnectorsoln2;
go
                        
create table StpasaCasesolution3 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[pasaversion] varchar(10) null,
[reservecondition] decimal(1,0) null,
[lorcondition] decimal(1,0) null,
[capacityobjfunction] decimal(12,3) null,
[capacityoption] decimal(12,3) null,
[maxsurplusreserveoption] decimal(12,3) null,
[maxsparecapacityoption] decimal(12,3) null,
[interconnectorflowpenalty] decimal(12,3) null,
[lastchanged] datetime2 null,
[reliabilitylrcdemandoption] decimal(12,3) null,
[outagelrcdemandoption] decimal(12,3) null,
[lordemandoption] decimal(12,3) null,
[reliabilitylrccapacityoption] varchar(10) null,
[outagelrccapacityoption] varchar(10) null,
[lorcapacityoption] varchar(10) null,
[loruigf_option] decimal(3,0) null,
[reliability_lrcuigf_option] decimal(3,0) null,
[outage_lrcuigf_option] decimal(3,0) null,

)
go
create clustered columnstore index cci_StpasaCasesolution3 on StpasaCasesolution3;
go
                        
create table StpasaConstraintsolution2 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[interval_datetime] datetime2 not null,
[constraintid] varchar(20) not null,
[capacityrhs] decimal(12,2) null,
[capacitymarginalvalue] decimal(12,2) null,
[capacityviolationdegree] decimal(12,2) null,
[lastchanged] datetime2 null,
[runtype] varchar(20) not null,

)
go
create clustered columnstore index cci_StpasaConstraintsolution2 on StpasaConstraintsolution2;
go
                        
create table BillingSmelterreduction5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(22,0) not null,
[weekno] decimal(22,0) not null,
[billrunno] decimal(22,0) not null,
[participantid] varchar(10) not null,
[rate1] decimal(15,6) null,
[ra1] decimal(15,6) null,
[rate2] decimal(15,6) null,
[ra2] decimal(15,6) null,
[te] decimal(15,6) null,
[pcsd] decimal(15,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingSmelterreduction5 on BillingSmelterreduction5;
go
                        
create table BillingIraucsurplus5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(2,0) not null,
[residueyear] decimal(4,0) null,
[quarter] decimal(2,0) null,
[billrunno] decimal(3,0) not null,
[contractid] varchar(30) not null,
[participantid] varchar(10) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[totalresidues] decimal(15,5) null,
[adjustment] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingIraucsurplus5 on BillingIraucsurplus5;
go
                        
create table BillingApcRecovery2 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[apeventid] decimal(6,0) not null,
[claimid] decimal(6,0) not null,
[participantid] varchar(20) not null,
[regionid] varchar(20) not null,
[recovery_amount] decimal(18,8) null,
[eligibility_start_interval] datetime2 null,
[eligibility_end_interval] datetime2 null,
[participant_demand] decimal(18,8) null,
[region_demand] decimal(18,8) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingApcRecovery2 on BillingApcRecovery2;
go
                        
create table BillingDaytrk5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[settlementdate] datetime2 not null,
[runno] decimal(3,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingDaytrk5 on BillingDaytrk5;
go
                        
create table BillingIntraresidues5 (
file_log_id bigint not null references FileLog(id),
[allocation] decimal(6,3) null,
[totalsurplus] decimal(15,5) null,
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[surplusvalue] decimal(15,6) null,
[lastchanged] datetime2 null,
[regionid] varchar(10) not null,

)
go
create clustered columnstore index cci_BillingIntraresidues5 on BillingIntraresidues5;
go
                        
create table BillingIrpartsurplus5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(2,0) not null,
[residueyear] decimal(4,0) null,
[quarter] decimal(2,0) null,
[billrunno] decimal(3,0) not null,
[contractid] varchar(30) not null,
[participantid] varchar(10) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[totalresidues] decimal(15,5) null,
[adjustment] decimal(15,5) null,
[lastchanged] datetime2 null,
[actualpayment] decimal(15,5) null,

)
go
create clustered columnstore index cci_BillingIrpartsurplus5 on BillingIrpartsurplus5;
go
                        
create table BillingReallocDetail5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[counterparty] varchar(10) not null,
[reallocationid] varchar(20) not null,
[value] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingReallocDetail5 on BillingReallocDetail5;
go
                        
create table BillingEftshortfallAmount1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(20) not null,
[shortfall_amount] decimal(18,8) null,
[shortfall] decimal(18,8) null,
[shortfall_company_id] varchar(20) null,
[company_shortfall_amount] decimal(18,8) null,
[participant_net_energy] decimal(18,8) null,
[company_net_energy] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingEftshortfallAmount1 on BillingEftshortfallAmount1;
go
                        
create table BillingIraucsurplussum7 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[residueyear] decimal(4,0) not null,
[quarter] decimal(2,0) not null,
[billrunno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[participantid] varchar(10) not null,
[totalsurplus] decimal(15,5) null,
[auctionfees] decimal(15,5) null,
[actualpayment] decimal(15,5) null,
[auctionfees_gst] decimal(15,5) null,
[lastchanged] datetime2 null,
[csp_derogation_amount] decimal(18,8) null,
[unadjusted_irsr] decimal(18,8) null,
[negative_residues] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingIraucsurplussum7 on BillingIraucsurplussum7;
go
                        
create table BillingSecdepInterestPay1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[security_deposit_id] varchar(20) not null,
[participantid] varchar(20) not null,
[interest_amount] decimal(18,8) null,
[interest_calc_type] varchar(20) null,
[interest_acct_id] varchar(20) null,
[interest_rate] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingSecdepInterestPay1 on BillingSecdepInterestPay1;
go
                        
create table BillingNmasTstRecvryTrk1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[recovery_contractyear] decimal(4,0) not null,
[recovery_weekno] decimal(3,0) not null,
[recovery_billrunno] decimal(3,0) not null,

)
go
create clustered columnstore index cci_BillingNmasTstRecvryTrk1 on BillingNmasTstRecvryTrk1;
go
                        
create table BillingIrfm5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[irfmpayment] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingIrfm5 on BillingIrfm5;
go
                        
create table BillingWhitehole5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(22,0) not null,
[weekno] decimal(22,0) not null,
[billrunno] decimal(22,0) not null,
[participantid] varchar(10) not null,
[nl] decimal(15,6) null,
[participantdemand] decimal(15,6) null,
[regiondemand] decimal(15,6) null,
[whiteholepayment] decimal(15,6) null,
[lastchanged] datetime2 null,
[interconnectorid] varchar(10) not null,

)
go
create clustered columnstore index cci_BillingWhitehole5 on BillingWhitehole5;
go
                        
create table BillingNmasTstRecovery1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(20) not null,
[service] varchar(10) not null,
[contractid] varchar(10) not null,
[regionid] varchar(10) not null,
[rbf] decimal(18,8) null,
[test_payment] decimal(18,8) null,
[recovery_start_date] datetime2 null,
[recovery_end_date] datetime2 null,
[participant_energy] decimal(18,8) null,
[region_energy] decimal(18,8) null,
[nem_energy] decimal(18,8) null,
[customer_proportion] decimal(18,8) null,
[generator_proportion] decimal(18,8) null,
[participant_generation] decimal(18,8) null,
[nem_generation] decimal(18,8) null,
[recovery_amount] decimal(18,8) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingNmasTstRecovery1 on BillingNmasTstRecovery1;
go
                        
create table BillingIrnspsurplussum6 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[residueyear] decimal(4,0) not null,
[quarter] decimal(2,0) not null,
[billrunno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[participantid] varchar(10) not null,
[totalsurplus] decimal(15,5) null,
[auctionfees] decimal(15,5) null,
[auctionfees_gst] decimal(15,5) null,
[lastchanged] datetime2 null,
[csp_derogation_amount] decimal(18,8) null,
[unadjusted_irsr] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingIrnspsurplussum6 on BillingIrnspsurplussum6;
go
                        
create table BillingRuntrk5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[status] varchar(6) null,
[adj_cleared] varchar(1) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(10) null,
[postdate] datetime2 null,
[postby] varchar(10) null,
[lastchanged] datetime2 null,
[receiptpostdate] datetime2 null,
[receiptpostby] varchar(10) null,
[paymentpostdate] datetime2 null,
[paymentpostby] varchar(10) null,
[shortfall] decimal(16,6) null,
[makeup] decimal(15,5) null,

)
go
create clustered columnstore index cci_BillingRuntrk5 on BillingRuntrk5;
go
                        
create table BillingAspayments6 (
file_log_id bigint not null references FileLog(id),
[regionid] varchar(10) null,
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[connectionpointid] varchar(10) not null,
[raise6sec] decimal(15,5) null,
[lower6sec] decimal(15,5) null,
[raise60sec] decimal(15,5) null,
[lower60sec] decimal(15,5) null,
[agc] decimal(15,5) null,
[fcascomp] decimal(15,5) null,
[loadshed] decimal(15,5) null,
[rgul] decimal(15,5) null,
[rguu] decimal(15,5) null,
[reactivepower] decimal(15,5) null,
[systemrestart] decimal(15,5) null,
[lastchanged] datetime2 null,
[lower5min] decimal(15,5) null,
[raise5min] decimal(15,5) null,
[lowerreg] decimal(15,5) null,
[raisereg] decimal(15,5) null,
[availability_reactive] decimal(18,8) null,
[availability_reactive_rbt] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingAspayments6 on BillingAspayments6;
go
                        
create table BillingRegionfigures5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[regionid] varchar(10) not null,
[energyout] decimal(16,6) null,
[valueout] decimal(16,6) null,
[energypurchased] decimal(16,6) null,
[valuepurchased] decimal(16,6) null,
[excessgen] decimal(16,6) null,
[reservetrading] decimal(16,6) null,
[intcompo] decimal(16,6) null,
[adminpricecompo] decimal(16,6) null,
[settsurplus] decimal(16,6) null,
[aspayment] decimal(16,6) null,
[poolfees] decimal(16,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingRegionfigures5 on BillingRegionfigures5;
go
                        
create table BillingEftshortfallDetail1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(20) not null,
[transaction_type] varchar(40) not null,
[amount] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingEftshortfallDetail1 on BillingEftshortfallDetail1;
go
                        
create table BillingAsrecovery7 (
file_log_id bigint not null references FileLog(id),
[regionid] varchar(10) not null,
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[raise6sec] decimal(15,5) null,
[lower6sec] decimal(15,5) null,
[raise60sec] decimal(15,5) null,
[lower60sec] decimal(15,5) null,
[agc] decimal(15,5) null,
[fcascomp] decimal(15,5) null,
[loadshed] decimal(15,5) null,
[rgul] decimal(15,5) null,
[rguu] decimal(15,5) null,
[reactivepower] decimal(15,5) null,
[systemrestart] decimal(15,5) null,
[lastchanged] datetime2 null,
[raise6sec_gen] decimal(15,5) null,
[lower6sec_gen] decimal(15,5) null,
[raise60sec_gen] decimal(15,5) null,
[lower60sec_gen] decimal(15,5) null,
[agc_gen] decimal(15,5) null,
[fcascomp_gen] decimal(15,5) null,
[loadshed_gen] decimal(15,5) null,
[rgul_gen] decimal(15,5) null,
[rguu_gen] decimal(15,5) null,
[reactivepower_gen] decimal(15,5) null,
[systemrestart_gen] decimal(15,5) null,
[lower5min] decimal(15,5) null,
[raise5min] decimal(15,5) null,
[lowerreg] decimal(15,5) null,
[raisereg] decimal(15,5) null,
[lower5min_gen] decimal(16,6) null,
[raise5min_gen] decimal(16,6) null,
[lowerreg_gen] decimal(16,6) null,
[raisereg_gen] decimal(16,6) null,
[availability_reactive] decimal(18,8) null,
[availability_reactive_rbt] decimal(18,8) null,
[availability_reactive_gen] decimal(18,8) null,
[availability_reactive_rbt_gen] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingAsrecovery7 on BillingAsrecovery7;
go
                        
create table BillingMrSummary5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[mr_date] datetime2 not null,
[regionid] varchar(10) not null,
[total_payments] decimal(16,6) null,
[total_recovery] decimal(16,6) null,
[total_rsa] decimal(16,6) null,
[aage] decimal(16,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingMrSummary5 on BillingMrSummary5;
go
                        
create table BillingRegionexports5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[regionid] varchar(10) not null,
[exportto] varchar(10) not null,
[energy] decimal(16,6) null,
[value] decimal(15,5) null,
[surplusenergy] decimal(16,6) null,
[surplusvalue] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingRegionexports5 on BillingRegionexports5;
go
                        
create table BillingSecdepInterestRate1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[interest_acct_id] varchar(20) not null,
[effectivedate] datetime2 not null,
[interest_rate] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingSecdepInterestRate1 on BillingSecdepInterestRate1;
go
                        
create table BillingIrpartsurplussum7 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[residueyear] decimal(4,0) not null,
[quarter] decimal(2,0) not null,
[billrunno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[participantid] varchar(10) not null,
[totalsurplus] decimal(15,5) null,
[auctionfees] decimal(15,5) null,
[actualpayment] decimal(15,5) null,
[auctionfees_gst] decimal(15,5) null,
[lastchanged] datetime2 null,
[csp_derogation_amount] decimal(18,8) null,
[unadjusted_irsr] decimal(18,8) null,
[auctionfees_totalgross_adj] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingIrpartsurplussum7 on BillingIrpartsurplussum7;
go
                        
create table BillingBillingCo2ePublicationTrk1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingBillingCo2ePublicationTrk1 on BillingBillingCo2ePublicationTrk1;
go
                        
create table BillingGstDetail5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[bas_class] varchar(30) not null,
[transaction_type] varchar(30) not null,
[gst_exclusive_amount] decimal(15,5) null,
[gst_amount] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingGstDetail5 on BillingGstDetail5;
go
                        
create table BillingDirectionReconciliatn1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[direction_id] varchar(20) not null,
[direction_desc] varchar(200) null,
[direction_start_date] datetime2 null,
[direction_end_date] datetime2 null,
[compensation_amount] decimal(16,6) null,
[independent_expert_fee] decimal(16,6) null,
[interest_amount] decimal(16,6) null,
[cra] decimal(16,6) null,
[nem_fee_id] varchar(20) null,
[nem_fixed_fee_amount] decimal(16,6) null,
[mkt_customer_perc] decimal(16,6) null,
[generator_perc] decimal(16,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingDirectionReconciliatn1 on BillingDirectionReconciliatn1;
go
                        
create table BillingBillingCo2ePublication1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[settlementdate] datetime2 not null,
[regionid] varchar(20) not null,
[sentoutenergy] decimal(18,8) null,
[generatoremissions] decimal(18,8) null,
[intensityindex] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingBillingCo2ePublication1 on BillingBillingCo2ePublication1;
go
                        
create table BillingInterresidues5 (
file_log_id bigint not null references FileLog(id),
[allocation] decimal(6,3) null,
[totalsurplus] decimal(15,5) null,
[interconnectorid] varchar(10) not null,
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[surplusvalue] decimal(15,6) null,
[lastchanged] datetime2 null,
[regionid] varchar(10) not null,

)
go
create clustered columnstore index cci_BillingInterresidues5 on BillingInterresidues5;
go
                        
create table BillingNmasTstPayments1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(20) not null,
[service] varchar(10) not null,
[contractid] varchar(10) not null,
[payment_amount] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingNmasTstPayments1 on BillingNmasTstPayments1;
go
                        
create table BillingIrnspsurplus5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(2,0) not null,
[residueyear] decimal(4,0) null,
[quarter] decimal(2,0) null,
[billrunno] decimal(3,0) not null,
[contractid] varchar(30) not null,
[participantid] varchar(10) not null,
[interconnectorid] varchar(10) not null,
[fromregionid] varchar(10) not null,
[totalresidues] decimal(15,5) null,
[adjustment] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingIrnspsurplus5 on BillingIrnspsurplus5;
go
                        
create table BillingMrShortfall5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[mr_date] datetime2 not null,
[regionid] varchar(10) not null,
[participantid] varchar(10) not null,
[age] decimal(16,6) null,
[rsa] decimal(16,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingMrShortfall5 on BillingMrShortfall5;
go
                        
create table BillingApcCompensation2 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[apeventid] decimal(6,0) not null,
[claimid] decimal(6,0) not null,
[participantid] varchar(20) null,
[compensation_amount] decimal(18,8) null,
[event_type] varchar(20) null,
[compensation_type] varchar(20) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingApcCompensation2 on BillingApcCompensation2;
go
                        
create table BillingGendata5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[connectionpointid] varchar(10) not null,
[stationid] varchar(10) null,
[duid] varchar(10) null,
[aggregateenergy] decimal(16,6) null,
[sales] decimal(16,6) null,
[purchases] decimal(16,6) null,
[lastchanged] datetime2 null,
[purchasedenergy] decimal(16,6) null,
[mda] varchar(10) null,

)
go
create clustered columnstore index cci_BillingGendata5 on BillingGendata5;
go
                        
create table BillingRealloc5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[counterparty] varchar(10) not null,
[value] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingRealloc5 on BillingRealloc5;
go
                        
create table BillingResTraderPayment1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[contractid] varchar(20) not null,
[payment_type] varchar(40) not null,
[participantid] varchar(20) not null,
[payment_amount] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingResTraderPayment1 on BillingResTraderPayment1;
go
                        
create table BillingFees5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[marketfeeid] varchar(10) not null,
[rate] decimal(15,5) null,
[energy] decimal(16,6) null,
[value] decimal(15,5) null,
[lastchanged] datetime2 null,
[participantcategoryid] varchar(10) not null,

)
go
create clustered columnstore index cci_BillingFees5 on BillingFees5;
go
                        
create table BillingRegionimports5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[regionid] varchar(10) not null,
[importfrom] varchar(10) not null,
[energy] decimal(16,6) null,
[value] decimal(15,5) null,
[surplusenergy] decimal(16,6) null,
[surplusvalue] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingRegionimports5 on BillingRegionimports5;
go
                        
create table BillingPrioradjustments5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[adjcontractyear] decimal(4,0) not null,
[adjweekno] decimal(3,0) not null,
[adjbillrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[prevamount] decimal(15,5) null,
[adjamount] decimal(15,5) null,
[irn] decimal(15,5) null,
[irp] decimal(15,5) null,
[interestamount] decimal(15,5) null,
[lastchanged] datetime2 null,
[irsr_prevamount] decimal(15,5) null,
[irsr_adjamount] decimal(15,5) null,
[irsr_interestamount] decimal(15,5) null,

)
go
create clustered columnstore index cci_BillingPrioradjustments5 on BillingPrioradjustments5;
go
                        
create table BillingMrRecovery5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[mr_date] datetime2 not null,
[regionid] varchar(10) not null,
[participantid] varchar(10) null,
[duid] varchar(10) not null,
[mr_amount] decimal(16,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingMrRecovery5 on BillingMrRecovery5;
go
                        
create table BillingDailyEnergySummary1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[settlementdate] datetime2 not null,
[participantid] varchar(20) not null,
[regionid] varchar(20) not null,
[customer_energy_purchased] decimal(18,8) null,
[generator_energy_sold] decimal(18,8) null,
[generator_energy_purchased] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingDailyEnergySummary1 on BillingDailyEnergySummary1;
go
                        
create table BillingBillingDirectionReconOther1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[direction_id] varchar(20) not null,
[regionid] varchar(20) not null,
[direction_desc] varchar(200) null,
[direction_type_id] varchar(20) null,
[direction_start_date] datetime2 null,
[direction_end_date] datetime2 null,
[direction_start_interval] datetime2 null,
[direction_end_interval] datetime2 null,
[compensation_amount] decimal(18,8) null,
[interest_amount] decimal(18,8) null,
[independent_expert_fee] decimal(18,8) null,
[cra] decimal(18,8) null,
[regional_customer_energy] decimal(18,8) null,
[regional_generator_energy] decimal(18,8) null,
[regional_benefit_factor] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingBillingDirectionReconOther1 on BillingBillingDirectionReconOther1;
go
                        
create table BillingMrPayment5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[mr_date] datetime2 not null,
[regionid] varchar(10) not null,
[participantid] varchar(10) null,
[duid] varchar(10) not null,
[mr_amount] decimal(16,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingMrPayment5 on BillingMrPayment5;
go
                        
create table BillingResTraderRecovery1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[regionid] varchar(20) not null,
[participantid] varchar(20) not null,
[recovery_amount] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingResTraderRecovery1 on BillingResTraderRecovery1;
go
                        
create table BillingCpdata5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[connectionpointid] varchar(10) not null,
[aggregateenergy] decimal(16,6) null,
[purchases] decimal(16,6) null,
[lastchanged] datetime2 null,
[mda] varchar(10) not null,

)
go
create clustered columnstore index cci_BillingCpdata5 on BillingCpdata5;
go
                        
create table BillingNmasTstRecvryRbf1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[service] varchar(10) not null,
[contractid] varchar(10) not null,
[regionid] varchar(10) not null,
[rbf] decimal(18,8) null,
[payment_amount] decimal(18,8) null,
[recovery_amount] decimal(18,8) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingNmasTstRecvryRbf1 on BillingNmasTstRecvryRbf1;
go
                        
create table BillingGstSummary5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[bas_class] varchar(30) not null,
[gst_exclusive_amount] decimal(15,5) null,
[gst_amount] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingGstSummary5 on BillingGstSummary5;
go
                        
create table BillingSecdepositApplication1 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(20) not null,
[application_amount] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingSecdepositApplication1 on BillingSecdepositApplication1;
go
                        
create table BillingFinancialadjustments5 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[billrunno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[participanttype] varchar(10) null,
[adjustmentitem] varchar(64) not null,
[amount] decimal(15,5) null,
[value] decimal(15,5) null,
[lastchanged] datetime2 null,
[financialcode] decimal(10,0) null,
[bas_class] varchar(30) null,

)
go
create clustered columnstore index cci_BillingFinancialadjustments5 on BillingFinancialadjustments5;
go
                        
create table ParticipantRegistrationStation1 (
file_log_id bigint not null references FileLog(id),
[stationid] varchar(10) not null,
[stationname] varchar(80) null,
[address1] varchar(80) null,
[address2] varchar(80) null,
[address3] varchar(80) null,
[address4] varchar(80) null,
[city] varchar(40) null,
[state] varchar(10) null,
[postcode] varchar(10) null,
[lastchanged] datetime2 null,
[connectionpointid] varchar(10) null,
[semidispatchcap] decimal(3,0) null,

)
go
create clustered columnstore index cci_ParticipantRegistrationStation1 on ParticipantRegistrationStation1;
go
                        
create table ParticipantRegistrationStadualloc1 (
file_log_id bigint not null references FileLog(id),
[duid] varchar(10) not null,
[effectivedate] datetime2 not null,
[stationid] varchar(10) not null,
[versionno] decimal(3,0) not null,
[lastchanged] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ParticipantRegistrationStadualloc1 on ParticipantRegistrationStadualloc1;
go
                        
create table ParticipantRegistrationDudetailsummary4 (
file_log_id bigint not null references FileLog(id),
[duid] varchar(10) not null,
[start_date] datetime2 not null,
[end_date] datetime2 not null,
[dispatchtype] varchar(10) null,
[connectionpointid] varchar(10) null,
[regionid] varchar(10) null,
[stationid] varchar(10) null,
[participantid] varchar(10) null,
[lastchanged] datetime2 null,
[transmissionlossfactor] decimal(15,5) null,
[starttype] varchar(20) null,
[distributionlossfactor] decimal(15,5) null,
[minimum_energy_price] decimal(9,2) null,
[maximum_energy_price] decimal(9,2) null,
[schedule_type] varchar(20) null,
[min_ramp_rate_up] decimal(6,0) null,
[min_ramp_rate_down] decimal(6,0) null,
[max_ramp_rate_up] decimal(6,0) null,
[max_ramp_rate_down] decimal(6,0) null,
[is_aggregated] decimal(1,0) null,

)
go
create clustered columnstore index cci_ParticipantRegistrationDudetailsummary4 on ParticipantRegistrationDudetailsummary4;
go
                        
create table ParticipantRegistrationStationownertrk1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[participantid] varchar(10) not null,
[versionno] decimal(3,0) not null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
[rrpeep43] decimal(15,5) null,

)
go
create clustered columnstore index cci_ParticipantRegistrationStationownertrk1 on ParticipantRegistrationStationownertrk1;
go
                        
create table ParticipantRegistrationGenunitsUnit1 (
file_log_id bigint not null references FileLog(id),
[gensetid] varchar(20) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(6,0) not null,
[unit_grouping_label] varchar(20) not null,
[unit_count] decimal(3,0) null,
[unit_size] decimal(8,3) null,
[unit_max_size] decimal(8,3) null,
[aggregation_flag] decimal(1,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ParticipantRegistrationGenunitsUnit1 on ParticipantRegistrationGenunitsUnit1;
go
                        
create table ParticipantRegistrationParticipantcategoryalloc1 (
file_log_id bigint not null references FileLog(id),
[participantcategoryid] varchar(10) not null,
[participantid] varchar(10) not null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ParticipantRegistrationParticipantcategoryalloc1 on ParticipantRegistrationParticipantcategoryalloc1;
go
                        
create table ParticipantRegistrationGenunits2 (
file_log_id bigint not null references FileLog(id),
[gensetid] varchar(20) not null,
[stationid] varchar(10) null,
[setlossfactor] decimal(16,6) null,
[cdindicator] varchar(10) null,
[agcflag] varchar(2) null,
[spinningflag] varchar(2) null,
[voltlevel] decimal(6,0) null,
[registeredcapacity] decimal(6,0) null,
[dispatchtype] varchar(10) null,
[starttype] varchar(20) null,
[mktgeneratorind] varchar(10) null,
[normalstatus] varchar(10) null,
[maxcapacity] decimal(6,0) null,
[gensettype] varchar(15) null,
[gensetname] varchar(40) null,
[lastchanged] datetime2 null,
[co2e_emissions_factor] decimal(18,8) null,
[co2e_energy_source] varchar(100) null,
[co2e_data_source] varchar(20) null,

)
go
create clustered columnstore index cci_ParticipantRegistrationGenunits2 on ParticipantRegistrationGenunits2;
go
                        
create table ParticipantRegistrationParticipantcreditdetail1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[participantid] varchar(10) not null,
[creditlimit] decimal(10,0) null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ParticipantRegistrationParticipantcreditdetail1 on ParticipantRegistrationParticipantcreditdetail1;
go
                        
create table ParticipantRegistrationMnspInterconnector2 (
file_log_id bigint not null references FileLog(id),
[linkid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[interconnectorid] varchar(10) null,
[fromregion] varchar(10) null,
[toregion] varchar(10) null,
[maxcapacity] decimal(5,0) null,
[tlf] decimal(12,7) null,
[lhsfactor] decimal(12,7) null,
[meterflowconstant] decimal(12,7) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[lastchanged] datetime2 null,
[from_region_tlf] decimal(12,7) null,
[to_region_tlf] decimal(12,7) null,

)
go
create clustered columnstore index cci_ParticipantRegistrationMnspInterconnector2 on ParticipantRegistrationMnspInterconnector2;
go
                        
create table ParticipantRegistrationParticipantaccount1 (
file_log_id bigint not null references FileLog(id),
[accountname] varchar(80) null,
[participantid] varchar(10) not null,
[accountnumber] varchar(16) null,
[bankname] varchar(16) null,
[banknumber] decimal(10,0) null,
[branchname] varchar(16) null,
[branchnumber] decimal(10,0) null,
[bsbnumber] varchar(20) null,
[nemmcocreditaccountnumber] decimal(10,0) null,
[nemmcodebitaccountnumber] decimal(10,0) null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[effectivedate] datetime2 null,
[lastchanged] datetime2 null,
[abn] varchar(20) null,

)
go
create clustered columnstore index cci_ParticipantRegistrationParticipantaccount1 on ParticipantRegistrationParticipantaccount1;
go
                        
create table ParticipantRegistrationDualloc1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[duid] varchar(10) not null,
[gensetid] varchar(20) not null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ParticipantRegistrationDualloc1 on ParticipantRegistrationDualloc1;
go
                        
create table ParticipantRegistrationGenmeter1 (
file_log_id bigint not null references FileLog(id),
[meterid] varchar(12) not null,
[gensetid] varchar(20) null,
[connectionpointid] varchar(10) null,
[stationid] varchar(10) null,
[metertype] varchar(20) null,
[meterclass] varchar(10) null,
[voltagelevel] decimal(6,0) null,
[applydate] datetime2 not null,
[versionno] decimal(3,0) not null,
[authorisedby] varchar(10) null,
[authoriseddate] datetime2 null,
[comdate] datetime2 null,
[decomdate] datetime2 null,
[enddate] datetime2 null,
[startdate] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ParticipantRegistrationGenmeter1 on ParticipantRegistrationGenmeter1;
go
                        
create table ParticipantRegistrationParticipantclass1 (
file_log_id bigint not null references FileLog(id),
[participantclassid] varchar(20) not null,
[description] varchar(64) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ParticipantRegistrationParticipantclass1 on ParticipantRegistrationParticipantclass1;
go
                        
create table ParticipantRegistrationMnspParticipant1 (
file_log_id bigint not null references FileLog(id),
[interconnectorid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ParticipantRegistrationMnspParticipant1 on ParticipantRegistrationMnspParticipant1;
go
                        
create table ParticipantRegistrationParticipant1 (
file_log_id bigint not null references FileLog(id),
[participantid] varchar(10) not null,
[participantclassid] varchar(20) null,
[name] varchar(80) null,
[description] varchar(64) null,
[acn] varchar(9) null,
[primarybusiness] varchar(40) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ParticipantRegistrationParticipant1 on ParticipantRegistrationParticipant1;
go
                        
create table ParticipantRegistrationStationoperatingstatus1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[stationid] varchar(10) not null,
[versionno] decimal(3,0) not null,
[status] varchar(20) null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ParticipantRegistrationStationoperatingstatus1 on ParticipantRegistrationStationoperatingstatus1;
go
                        
create table ParticipantRegistrationStationowner1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[participantid] varchar(10) not null,
[stationid] varchar(10) not null,
[versionno] decimal(3,0) not null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ParticipantRegistrationStationowner1 on ParticipantRegistrationStationowner1;
go
                        
create table ParticipantRegistrationDispatchableunit1 (
file_log_id bigint not null references FileLog(id),
[duid] varchar(10) not null,
[duname] varchar(20) null,
[unittype] varchar(20) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ParticipantRegistrationDispatchableunit1 on ParticipantRegistrationDispatchableunit1;
go
                        
create table ParticipantRegistrationParticipantcategory1 (
file_log_id bigint not null references FileLog(id),
[participantcategoryid] varchar(10) not null,
[description] varchar(64) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ParticipantRegistrationParticipantcategory1 on ParticipantRegistrationParticipantcategory1;
go
                        
create table ParticipantRegistrationBidduiddetails1 (
file_log_id bigint not null references FileLog(id),
[duid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[bidtype] varchar(10) not null,
[maxcapacity] decimal(22,0) null,
[minenablementlevel] decimal(22,0) null,
[maxenablementlevel] decimal(22,0) null,
[maxlowerangle] decimal(3,0) null,
[maxupperangle] decimal(3,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ParticipantRegistrationBidduiddetails1 on ParticipantRegistrationBidduiddetails1;
go
                        
create table ParticipantRegistrationBidduiddetailstrk1 (
file_log_id bigint not null references FileLog(id),
[duid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ParticipantRegistrationBidduiddetailstrk1 on ParticipantRegistrationBidduiddetailstrk1;
go
                        
create table ParticipantRegistrationDudetail3 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[duid] varchar(10) not null,
[versionno] decimal(3,0) not null,
[connectionpointid] varchar(10) null,
[voltlevel] varchar(10) null,
[registeredcapacity] decimal(6,0) null,
[agccapability] varchar(1) null,
[dispatchtype] varchar(10) null,
[maxcapacity] decimal(6,0) null,
[starttype] varchar(20) null,
[normallyonflag] varchar(1) null,
[physicaldetailsflag] varchar(1) null,
[spinningreserveflag] varchar(1) null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,
[intermittentflag] varchar(1) null,
[semi_schedule_flag] varchar(1) null,
[maxrateofchangeup] decimal(6,0) null,
[maxrateofchangedown] decimal(6,0) null,

)
go
create clustered columnstore index cci_ParticipantRegistrationDudetail3 on ParticipantRegistrationDudetail3;
go
                        
create table SettlementConfigMarketFeeCatExclTrk1 (
file_log_id bigint not null references FileLog(id),
[marketfeeid] varchar(20) not null,
[effectivedate] datetime2 not null,
[version_datetime] datetime2 not null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementConfigMarketFeeCatExclTrk1 on SettlementConfigMarketFeeCatExclTrk1;
go
                        
create table SettlementConfigMarketfeetrk1 (
file_log_id bigint not null references FileLog(id),
[marketfeeversionno] decimal(3,0) not null,
[effectivedate] datetime2 not null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementConfigMarketfeetrk1 on SettlementConfigMarketfeetrk1;
go
                        
create table SettlementConfigSetcfgParticipantMpf1 (
file_log_id bigint not null references FileLog(id),
[participantid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantcategoryid] varchar(10) not null,
[connectionpointid] varchar(10) not null,
[mpf] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementConfigSetcfgParticipantMpf1 on SettlementConfigSetcfgParticipantMpf1;
go
                        
create table SettlementConfigMarketfee1 (
file_log_id bigint not null references FileLog(id),
[marketfeeid] varchar(10) not null,
[marketfeeperiod] varchar(20) null,
[marketfeetype] varchar(12) null,
[description] varchar(64) null,
[lastchanged] datetime2 null,
[gl_tcode] varchar(15) null,
[gl_financialcode] varchar(10) null,
[fee_class] varchar(40) null,

)
go
create clustered columnstore index cci_SettlementConfigMarketfee1 on SettlementConfigMarketfee1;
go
                        
create table SettlementConfigMarketFeeExclusionTrk1 (
file_log_id bigint not null references FileLog(id),
[participantid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementConfigMarketFeeExclusionTrk1 on SettlementConfigMarketFeeExclusionTrk1;
go
                        
create table SetcfgReallocationinterval1 (
file_log_id bigint not null references FileLog(id),
[reallocationid] varchar(20) not null,
[periodid] decimal(3,0) not null,
[value] decimal(15,5) null,
[lastchanged] datetime2 null,
[nrp] decimal(15,5) null,

)
go
create clustered columnstore index cci_SetcfgReallocationinterval1 on SetcfgReallocationinterval1;
go
                        
create table SettlementConfigMarketfeedata1 (
file_log_id bigint not null references FileLog(id),
[marketfeeid] varchar(10) not null,
[marketfeeversionno] decimal(3,0) not null,
[effectivedate] datetime2 not null,
[marketfeevalue] decimal(22,8) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementConfigMarketfeedata1 on SettlementConfigMarketfeedata1;
go
                        
create table SettlementConfigParticipantBandfeeAlloc1 (
file_log_id bigint not null references FileLog(id),
[participantid] varchar(10) not null,
[marketfeeid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[participantcategoryid] varchar(10) not null,
[marketfeevalue] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementConfigParticipantBandfeeAlloc1 on SettlementConfigParticipantBandfeeAlloc1;
go
                        
create table SettlementConfigSetcfgParticipantMpftrk1 (
file_log_id bigint not null references FileLog(id),
[participantid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementConfigSetcfgParticipantMpftrk1 on SettlementConfigSetcfgParticipantMpftrk1;
go
                        
create table SettlementConfigMarketFeeCatExcl1 (
file_log_id bigint not null references FileLog(id),
[marketfeeid] varchar(20) not null,
[effectivedate] datetime2 not null,
[version_datetime] datetime2 not null,
[participant_categoryid] varchar(20) not null,

)
go
create clustered columnstore index cci_SettlementConfigMarketFeeCatExcl1 on SettlementConfigMarketFeeCatExcl1;
go
                        
create table SettlementConfigMarketFeeExclusion1 (
file_log_id bigint not null references FileLog(id),
[participantid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[marketfeeid] varchar(10) not null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementConfigMarketFeeExclusion1 on SettlementConfigMarketFeeExclusion1;
go
                        
create table SetcfgReallocation2 (
file_log_id bigint not null references FileLog(id),
[reallocationid] varchar(20) not null,
[creditparticipantid] varchar(10) null,
[debitparticipantid] varchar(10) null,
[regionid] varchar(10) null,
[agreementtype] varchar(10) null,
[creditreference] varchar(400) null,
[debitreference] varchar(400) null,
[lastchanged] datetime2 null,
[startdate] datetime2 null,
[enddate] datetime2 null,
[current_stepid] varchar(20) null,
[daytype] varchar(20) null,
[reallocation_type] varchar(1) null,
[calendarid] varchar(30) null,
[intervallength] decimal(3,0) null,

)
go
create clustered columnstore index cci_SetcfgReallocation2 on SetcfgReallocation2;
go
                        
create table SettlementConfigAncillaryRecoverySplit1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[service] varchar(10) not null,
[paymenttype] varchar(20) not null,
[customer_portion] decimal(8,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_SettlementConfigAncillaryRecoverySplit1 on SettlementConfigAncillaryRecoverySplit1;
go
                        
create table PdpasaRegionsolution5 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[interval_datetime] datetime2 not null,
[regionid] varchar(10) not null,
[demand10] decimal(12,2) null,
[demand50] decimal(12,2) null,
[demand90] decimal(12,2) null,
[reservereq] decimal(12,2) null,
[capacityreq] decimal(12,2) null,
[energyreqdemand50] decimal(12,2) null,
[unconstrainedcapacity] decimal(12,0) null,
[constrainedcapacity] decimal(12,0) null,
[netinterchangeunderscarcity] decimal(12,2) null,
[surpluscapacity] decimal(12,2) null,
[surplusreserve] decimal(12,2) null,
[reservecondition] decimal(1,0) null,
[maxsurplusreserve] decimal(12,2) null,
[maxsparecapacity] decimal(12,2) null,
[lorcondition] decimal(1,0) null,
[aggregatecapacityavailable] decimal(12,2) null,
[aggregatescheduledload] decimal(12,2) null,
[lastchanged] datetime2 null,
[aggregatepasaavailability] decimal(12,0) null,
[runtype] varchar(20) not null,
[energyreqdemand10] decimal(12,2) null,
[calculatedlor1level] decimal(16,6) null,
[calculatedlor2level] decimal(16,6) null,
[msrnetinterchangeunderscarcity] decimal(12,2) null,
[lornetinterchangeunderscarcity] decimal(12,2) null,
[totalintermittentgeneration] decimal(15,5) null,
[demand_and_nonschedgen] decimal(15,5) null,
[uigf] decimal(12,2) null,
[semi_scheduled_capacity] decimal(12,2) null,
[lor_semi_scheduled_capacity] decimal(12,2) null,
[lcr] decimal(16,6) null,
[lcr2] decimal(16,6) null,
[fum] decimal(16,6) null,
[ss_solar_uigf] decimal(12,2) null,
[ss_wind_uigf] decimal(12,2) null,
[ss_solar_capacity] decimal(12,2) null,
[ss_wind_capacity] decimal(12,2) null,
[ss_solar_cleared] decimal(12,2) null,
[ss_wind_cleared] decimal(12,2) null,

)
go
create clustered columnstore index cci_PdpasaRegionsolution5 on PdpasaRegionsolution5;
go
                        
create table PdpasaCasesolution3 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[pasaversion] varchar(10) null,
[reservecondition] decimal(1,0) null,
[lorcondition] decimal(1,0) null,
[capacityobjfunction] decimal(12,3) null,
[capacityoption] decimal(12,3) null,
[maxsurplusreserveoption] decimal(12,3) null,
[maxsparecapacityoption] decimal(12,3) null,
[interconnectorflowpenalty] decimal(12,3) null,
[lastchanged] datetime2 null,
[reliabilitylrcdemandoption] decimal(12,3) null,
[outagelrcdemandoption] decimal(12,3) null,
[lordemandoption] decimal(12,3) null,
[reliabilitylrccapacityoption] varchar(10) null,
[outagelrccapacityoption] varchar(10) null,
[lorcapacityoption] varchar(10) null,
[loruigf_option] decimal(3,0) null,
[reliability_lrcuigf_option] decimal(3,0) null,
[outage_lrcuigf_option] decimal(3,0) null,

)
go
create clustered columnstore index cci_PdpasaCasesolution3 on PdpasaCasesolution3;
go
                        
create table PredispatchScenarioDemand1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[scenario] decimal(2,0) not null,
[regionid] varchar(20) not null,
[deltamw] decimal(4,0) null,

)
go
create clustered columnstore index cci_PredispatchScenarioDemand1 on PredispatchScenarioDemand1;
go
                        
create table PredispatchRegionSolution4 (
file_log_id bigint not null references FileLog(id),
[predispatchseqno] varchar(20) null,
[runno] decimal(3,0) null,
[regionid] varchar(10) not null,
[periodid] varchar(20) null,
[intervention] decimal(2,0) null,
[totaldemand] decimal(15,5) null,
[availablegeneration] decimal(15,5) null,
[availableload] decimal(15,5) null,
[demandforecast] decimal(15,5) null,
[dispatchablegeneration] decimal(15,5) null,
[dispatchableload] decimal(15,5) null,
[netinterchange] decimal(15,5) null,
[excessgeneration] decimal(15,5) null,
[lower5mindispatch] decimal(15,5) null,
[lower5minimport] decimal(15,5) null,
[lower5minlocaldispatch] decimal(15,5) null,
[lower5minlocalprice] decimal(15,5) null,
[lower5minlocalreq] decimal(15,5) null,
[lower5minprice] decimal(15,5) null,
[lower5minreq] decimal(15,5) null,
[lower5minsupplyprice] decimal(15,5) null,
[lower60secdispatch] decimal(15,5) null,
[lower60secimport] decimal(15,5) null,
[lower60seclocaldispatch] decimal(15,5) null,
[lower60seclocalprice] decimal(15,5) null,
[lower60seclocalreq] decimal(15,5) null,
[lower60secprice] decimal(15,5) null,
[lower60secreq] decimal(15,5) null,
[lower60secsupplyprice] decimal(15,5) null,
[lower6secdispatch] decimal(15,5) null,
[lower6secimport] decimal(15,5) null,
[lower6seclocaldispatch] decimal(15,5) null,
[lower6seclocalprice] decimal(15,5) null,
[lower6seclocalreq] decimal(15,5) null,
[lower6secprice] decimal(15,5) null,
[lower6secreq] decimal(15,5) null,
[lower6secsupplyprice] decimal(15,5) null,
[raise5mindispatch] decimal(15,5) null,
[raise5minimport] decimal(15,5) null,
[raise5minlocaldispatch] decimal(15,5) null,
[raise5minlocalprice] decimal(15,5) null,
[raise5minlocalreq] decimal(15,5) null,
[raise5minprice] decimal(15,5) null,
[raise5minreq] decimal(15,5) null,
[raise5minsupplyprice] decimal(15,5) null,
[raise60secdispatch] decimal(15,5) null,
[raise60secimport] decimal(15,5) null,
[raise60seclocaldispatch] decimal(15,5) null,
[raise60seclocalprice] decimal(15,5) null,
[raise60seclocalreq] decimal(15,5) null,
[raise60secprice] decimal(15,5) null,
[raise60secreq] decimal(15,5) null,
[raise60secsupplyprice] decimal(15,5) null,
[raise6secdispatch] decimal(15,5) null,
[raise6secimport] decimal(15,5) null,
[raise6seclocaldispatch] decimal(15,5) null,
[raise6seclocalprice] decimal(15,5) null,
[raise6seclocalreq] decimal(15,5) null,
[raise6secprice] decimal(15,5) null,
[raise6secreq] decimal(15,5) null,
[raise6secsupplyprice] decimal(15,5) null,
[lastchanged] datetime2 null,
[datetime] datetime2 not null,
[initialsupply] decimal(15,5) null,
[clearedsupply] decimal(15,5) null,
[lowerregimport] decimal(15,5) null,
[lowerreglocaldispatch] decimal(15,5) null,
[lowerreglocalreq] decimal(15,5) null,
[lowerregreq] decimal(15,5) null,
[raiseregimport] decimal(15,5) null,
[raisereglocaldispatch] decimal(15,5) null,
[raisereglocalreq] decimal(15,5) null,
[raiseregreq] decimal(15,5) null,
[raise5minlocalviolation] decimal(15,5) null,
[raisereglocalviolation] decimal(15,5) null,
[raise60seclocalviolation] decimal(15,5) null,
[raise6seclocalviolation] decimal(15,5) null,
[lower5minlocalviolation] decimal(15,5) null,
[lowerreglocalviolation] decimal(15,5) null,
[lower60seclocalviolation] decimal(15,5) null,
[lower6seclocalviolation] decimal(15,5) null,
[raise5minviolation] decimal(15,5) null,
[raiseregviolation] decimal(15,5) null,
[raise60secviolation] decimal(15,5) null,
[raise6secviolation] decimal(15,5) null,
[lower5minviolation] decimal(15,5) null,
[lowerregviolation] decimal(15,5) null,
[lower60secviolation] decimal(15,5) null,
[lower6secviolation] decimal(15,5) null,
[raise6secactualavailability] decimal(16,6) null,
[raise60secactualavailability] decimal(16,6) null,
[raise5minactualavailability] decimal(16,6) null,
[raiseregactualavailability] decimal(16,6) null,
[lower6secactualavailability] decimal(16,6) null,
[lower60secactualavailability] decimal(16,6) null,
[lower5minactualavailability] decimal(16,6) null,
[lowerregactualavailability] decimal(16,6) null,
[decavailability] decimal(16,6) null,
[lorsurplus] decimal(16,6) null,
[lrcsurplus] decimal(16,6) null,
[totalintermittentgeneration] decimal(15,5) null,
[demand_and_nonschedgen] decimal(15,5) null,
[uigf] decimal(15,5) null,
[semischedule_clearedmw] decimal(15,5) null,
[semischedule_compliancemw] decimal(15,5) null,
[ss_solar_uigf] decimal(15,5) null,
[ss_wind_uigf] decimal(15,5) null,
[ss_solar_clearedmw] decimal(15,5) null,
[ss_wind_clearedmw] decimal(15,5) null,
[ss_solar_compliancemw] decimal(15,5) null,
[ss_wind_compliancemw] decimal(15,5) null,

)
go
create clustered columnstore index cci_PredispatchRegionSolution4 on PredispatchRegionSolution4;
go
                        
create table PredispatchMnspbidtrk1 (
file_log_id bigint not null references FileLog(id),
[predispatchseqno] varchar(20) not null,
[linkid] varchar(10) not null,
[periodid] varchar(20) not null,
[participantid] varchar(10) null,
[settlementdate] datetime2 null,
[offerdate] datetime2 null,
[versionno] decimal(3,0) null,
[datetime] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_PredispatchMnspbidtrk1 on PredispatchMnspbidtrk1;
go
                        
create table PredispatchConstraintSolution5 (
file_log_id bigint not null references FileLog(id),
[predispatchseqno] varchar(20) null,
[runno] decimal(3,0) null,
[constraintid] varchar(20) not null,
[periodid] varchar(20) null,
[intervention] decimal(2,0) null,
[rhs] decimal(15,5) null,
[marginalvalue] decimal(15,5) null,
[violationdegree] decimal(15,5) null,
[lastchanged] datetime2 null,
[datetime] datetime2 not null,
[duid] varchar(20) null,
[genconid_effectivedate] datetime2 null,
[genconid_versionno] decimal(22,0) null,
[lhs] decimal(15,5) null,

)
go
create clustered columnstore index cci_PredispatchConstraintSolution5 on PredispatchConstraintSolution5;
go
                        
create table PredispatchOffertrk1 (
file_log_id bigint not null references FileLog(id),
[predispatchseqno] varchar(20) not null,
[duid] varchar(10) not null,
[bidtype] varchar(20) not null,
[periodid] varchar(20) not null,
[bidsettlementdate] datetime2 null,
[bidofferdate] datetime2 null,
[datetime] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_PredispatchOffertrk1 on PredispatchOffertrk1;
go
                        
create table PredispatchPricesensitivities1 (
file_log_id bigint not null references FileLog(id),
[predispatchseqno] varchar(20) null,
[runno] decimal(3,0) null,
[regionid] varchar(10) not null,
[periodid] varchar(20) null,
[intervention] decimal(2,0) null,
[rrpeep1] decimal(15,5) null,
[rrpeep2] decimal(15,5) null,
[rrpeep3] decimal(15,5) null,
[rrpeep4] decimal(15,5) null,
[rrpeep5] decimal(15,5) null,
[rrpeep6] decimal(15,5) null,
[rrpeep7] decimal(15,5) null,
[rrpeep8] decimal(15,5) null,
[rrpeep9] decimal(15,5) null,
[rrpeep10] decimal(15,5) null,
[rrpeep11] decimal(15,5) null,
[rrpeep12] decimal(15,5) null,
[rrpeep13] decimal(15,5) null,
[rrpeep14] decimal(15,5) null,
[rrpeep15] decimal(15,5) null,
[rrpeep16] decimal(15,5) null,
[rrpeep17] decimal(15,5) null,
[rrpeep18] decimal(15,5) null,
[rrpeep19] decimal(15,5) null,
[rrpeep20] decimal(15,5) null,
[rrpeep21] decimal(15,5) null,
[rrpeep22] decimal(15,5) null,
[rrpeep23] decimal(15,5) null,
[rrpeep24] decimal(15,5) null,
[rrpeep25] decimal(15,5) null,
[rrpeep26] decimal(15,5) null,
[rrpeep27] decimal(15,5) null,
[rrpeep28] decimal(15,5) null,
[lastchanged] datetime2 null,
[datetime] datetime2 not null,
[rrpeep29] decimal(15,5) null,
[rrpeep30] decimal(15,5) null,
[rrpeep31] decimal(15,5) null,
[rrpeep32] decimal(15,5) null,
[rrpeep33] decimal(15,5) null,
[rrpeep34] decimal(15,5) null,
[rrpeep35] decimal(15,5) null,
[intervention_active] decimal(1,0) null,
[rrpeep36] decimal(15,5) null,
[rrpeep37] decimal(15,5) null,
[rrpeep38] decimal(15,5) null,
[rrpeep39] decimal(15,5) null,
[rrpeep40] decimal(15,5) null,
[rrpeep41] decimal(15,5) null,
[rrpeep42] decimal(15,5) null,
[rrpeep43] decimal(15,5) null,

)
go
create clustered columnstore index cci_PredispatchPricesensitivities1 on PredispatchPricesensitivities1;
go
                        
create table PredispatchInterconnectorSoln3 (
file_log_id bigint not null references FileLog(id),
[predispatchseqno] varchar(20) null,
[runno] decimal(3,0) null,
[interconnectorid] varchar(10) not null,
[periodid] varchar(20) null,
[intervention] decimal(2,0) null,
[meteredmwflow] decimal(15,5) null,
[mwflow] decimal(15,5) null,
[mwlosses] decimal(15,5) null,
[marginalvalue] decimal(15,5) null,
[violationdegree] decimal(15,5) null,
[lastchanged] datetime2 null,
[datetime] datetime2 not null,
[exportlimit] decimal(15,5) null,
[importlimit] decimal(15,5) null,
[marginalloss] decimal(15,5) null,
[exportgenconid] varchar(20) null,
[importgenconid] varchar(20) null,
[fcasexportlimit] decimal(15,5) null,
[fcasimportlimit] decimal(15,5) null,
[local_price_adjustment_export] decimal(10,2) null,
[locally_constrained_export] decimal(1,0) null,
[local_price_adjustment_import] decimal(10,2) null,
[locally_constrained_import] decimal(1,0) null,

)
go
create clustered columnstore index cci_PredispatchInterconnectorSoln3 on PredispatchInterconnectorSoln3;
go
                        
create table PredispatchLocalPrice1 (
file_log_id bigint not null references FileLog(id),
[predispatchseqno] varchar(20) not null,
[datetime] datetime2 not null,
[duid] varchar(20) not null,
[periodid] varchar(20) null,
[local_price_adjustment] decimal(10,2) null,
[locally_constrained] decimal(1,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_PredispatchLocalPrice1 on PredispatchLocalPrice1;
go
                        
create table PredispatchRegionfcasrequirement2 (
file_log_id bigint not null references FileLog(id),
[predispatchseqno] varchar(20) null,
[runno] decimal(3,0) null,
[intervention] decimal(2,0) null,
[periodid] varchar(20) null,
[genconid] varchar(20) not null,
[regionid] varchar(10) not null,
[bidtype] varchar(10) not null,
[genconeffectivedate] datetime2 null,
[genconversionno] decimal(3,0) null,
[marginalvalue] decimal(16,6) null,
[datetime] datetime2 not null,
[lastchanged] datetime2 null,
[base_cost] decimal(18,8) null,
[adjusted_cost] decimal(18,8) null,
[estimated_cmpf] decimal(18,8) null,
[estimated_crmpf] decimal(18,8) null,
[recovery_factor_cmpf] decimal(18,8) null,
[recovery_factor_crmpf] decimal(18,8) null,

)
go
create clustered columnstore index cci_PredispatchRegionfcasrequirement2 on PredispatchRegionfcasrequirement2;
go
                        
create table PredispatchRegionPrices1 (
file_log_id bigint not null references FileLog(id),
[predispatchseqno] varchar(20) null,
[runno] decimal(3,0) null,
[regionid] varchar(10) not null,
[periodid] varchar(20) null,
[intervention] decimal(2,0) null,
[rrp] decimal(15,5) null,
[eep] decimal(15,5) null,
[rrp1] decimal(15,5) null,
[eep1] decimal(15,5) null,
[rrp2] decimal(15,5) null,
[eep2] decimal(15,5) null,
[rrp3] decimal(15,5) null,
[eep3] decimal(15,5) null,
[rrp4] decimal(15,5) null,
[eep4] decimal(15,5) null,
[rrp5] decimal(15,5) null,
[eep5] decimal(15,5) null,
[rrp6] decimal(15,5) null,
[eep6] decimal(15,5) null,
[rrp7] decimal(15,5) null,
[eep7] decimal(15,5) null,
[rrp8] decimal(15,5) null,
[eep8] decimal(15,5) null,
[lastchanged] datetime2 null,
[datetime] datetime2 not null,
[raise6secrrp] decimal(15,5) null,
[raise60secrrp] decimal(15,5) null,
[raise5minrrp] decimal(15,5) null,
[raiseregrrp] decimal(15,5) null,
[lower6secrrp] decimal(15,5) null,
[lower60secrrp] decimal(15,5) null,
[lower5minrrp] decimal(15,5) null,
[lowerregrrp] decimal(15,5) null,

)
go
create clustered columnstore index cci_PredispatchRegionPrices1 on PredispatchRegionPrices1;
go
                        
create table PredispatchUnitSolution2 (
file_log_id bigint not null references FileLog(id),
[predispatchseqno] varchar(20) null,
[runno] decimal(3,0) null,
[duid] varchar(10) not null,
[tradetype] decimal(2,0) null,
[periodid] varchar(20) null,
[intervention] decimal(2,0) null,
[connectionpointid] varchar(12) null,
[agcstatus] decimal(2,0) null,
[dispatchmode] decimal(2,0) null,
[initialmw] decimal(15,5) null,
[totalcleared] decimal(15,5) null,
[lower5min] decimal(15,5) null,
[lower60sec] decimal(15,5) null,
[lower6sec] decimal(15,5) null,
[raise5min] decimal(15,5) null,
[raise60sec] decimal(15,5) null,
[raise6sec] decimal(15,5) null,
[rampdownrate] decimal(15,5) null,
[rampuprate] decimal(15,5) null,
[downepf] decimal(15,5) null,
[upepf] decimal(15,5) null,
[marginal5minvalue] decimal(15,5) null,
[marginal60secvalue] decimal(15,5) null,
[marginal6secvalue] decimal(15,5) null,
[marginalvalue] decimal(15,5) null,
[violation5mindegree] decimal(15,5) null,
[violation60secdegree] decimal(15,5) null,
[violation6secdegree] decimal(15,5) null,
[violationdegree] decimal(15,5) null,
[lastchanged] datetime2 null,
[datetime] datetime2 not null,
[lowerreg] decimal(15,5) null,
[raisereg] decimal(15,5) null,
[availability] decimal(15,5) null,
[raise6secflags] decimal(3,0) null,
[raise60secflags] decimal(3,0) null,
[raise5minflags] decimal(3,0) null,
[raiseregflags] decimal(3,0) null,
[lower6secflags] decimal(3,0) null,
[lower60secflags] decimal(3,0) null,
[lower5minflags] decimal(3,0) null,
[lowerregflags] decimal(3,0) null,
[raise6secactualavailability] decimal(16,6) null,
[raise60secactualavailability] decimal(16,6) null,
[raise5minactualavailability] decimal(16,6) null,
[raiseregactualavailability] decimal(16,6) null,
[lower6secactualavailability] decimal(16,6) null,
[lower60secactualavailability] decimal(16,6) null,
[lower5minactualavailability] decimal(16,6) null,
[lowerregactualavailability] decimal(16,6) null,
[semidispatchcap] decimal(3,0) null,

)
go
create clustered columnstore index cci_PredispatchUnitSolution2 on PredispatchUnitSolution2;
go
                        
create table PredispatchScenarioDemandTrk1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_PredispatchScenarioDemandTrk1 on PredispatchScenarioDemandTrk1;
go
                        
create table PredispatchCaseSolution1 (
file_log_id bigint not null references FileLog(id),
[predispatchseqno] varchar(20) not null,
[runno] decimal(3,0) not null,
[solutionstatus] decimal(2,0) null,
[spdversion] varchar(20) null,
[nonphysicallosses] decimal(1,0) null,
[totalobjective] decimal(27,10) null,
[totalareagenviolation] decimal(15,5) null,
[totalinterconnectorviolation] decimal(15,5) null,
[totalgenericviolation] decimal(15,5) null,
[totalramprateviolation] decimal(15,5) null,
[totalunitmwcapacityviolation] decimal(15,5) null,
[total5minviolation] decimal(15,5) null,
[totalregviolation] decimal(15,5) null,
[total6secviolation] decimal(15,5) null,
[total60secviolation] decimal(15,5) null,
[totalasprofileviolation] decimal(15,5) null,
[totalenergyconstrviolation] decimal(15,5) null,
[totalenergyofferviolation] decimal(15,5) null,
[lastchanged] datetime2 null,
[intervention] decimal(2,0) null,

)
go
create clustered columnstore index cci_PredispatchCaseSolution1 on PredispatchCaseSolution1;
go
                        
create table PredispatchBlockedConstraints1 (
file_log_id bigint not null references FileLog(id),
[predispatchseqno] varchar(20) not null,
[constraintid] varchar(20) not null,

)
go
create clustered columnstore index cci_PredispatchBlockedConstraints1 on PredispatchBlockedConstraints1;
go
                        
create table PredispatchInterconnectrSens1 (
file_log_id bigint not null references FileLog(id),
[predispatchseqno] varchar(20) null,
[runno] decimal(3,0) null,
[interconnectorid] varchar(10) not null,
[periodid] varchar(20) null,
[intervention] decimal(2,0) null,
[datetime] datetime2 not null,
[intervention_active] decimal(1,0) null,
[mwflow1] decimal(15,5) null,
[mwflow2] decimal(15,5) null,
[mwflow3] decimal(15,5) null,
[mwflow4] decimal(15,5) null,
[mwflow5] decimal(15,5) null,
[mwflow6] decimal(15,5) null,
[mwflow7] decimal(15,5) null,
[mwflow8] decimal(15,5) null,
[mwflow9] decimal(15,5) null,
[mwflow10] decimal(15,5) null,
[mwflow11] decimal(15,5) null,
[mwflow12] decimal(15,5) null,
[mwflow13] decimal(15,5) null,
[mwflow14] decimal(15,5) null,
[mwflow15] decimal(15,5) null,
[mwflow16] decimal(15,5) null,
[mwflow17] decimal(15,5) null,
[mwflow18] decimal(15,5) null,
[mwflow19] decimal(15,5) null,
[mwflow20] decimal(15,5) null,
[mwflow21] decimal(15,5) null,
[mwflow22] decimal(15,5) null,
[mwflow23] decimal(15,5) null,
[mwflow24] decimal(15,5) null,
[mwflow25] decimal(15,5) null,
[mwflow26] decimal(15,5) null,
[mwflow27] decimal(15,5) null,
[mwflow28] decimal(15,5) null,
[mwflow29] decimal(15,5) null,
[mwflow30] decimal(15,5) null,
[mwflow31] decimal(15,5) null,
[mwflow32] decimal(15,5) null,
[mwflow33] decimal(15,5) null,
[mwflow34] decimal(15,5) null,
[mwflow35] decimal(15,5) null,
[mwflow36] decimal(15,5) null,
[mwflow37] decimal(15,5) null,
[mwflow38] decimal(15,5) null,
[mwflow39] decimal(15,5) null,
[mwflow40] decimal(15,5) null,
[mwflow41] decimal(15,5) null,
[mwflow42] decimal(15,5) null,
[mwflow43] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_PredispatchInterconnectrSens1 on PredispatchInterconnectrSens1;
go
                        
create table VoltageInstructionTrack2 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[file_type] varchar(20) null,
[version_datetime] datetime2 not null,
[se_datetime] datetime2 null,
[solution_category] varchar(60) null,
[solution_status] varchar(60) null,
[operating_mode] varchar(60) null,
[operating_status] varchar(100) null,
[est_expiry] datetime2 null,
[est_next_instruction] datetime2 null,

)
go
create clustered columnstore index cci_VoltageInstructionTrack2 on VoltageInstructionTrack2;
go
                        
create table VoltageInstructionInstruction2 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[ems_id] varchar(60) not null,
[participantid] varchar(20) null,
[station_id] varchar(60) null,
[device_id] varchar(60) null,
[device_type] varchar(20) null,
[control_type] varchar(20) null,
[target] decimal(15,0) null,
[conforming] decimal(1,0) null,
[instruction_summary] varchar(400) null,
[version_datetime] datetime2 not null,
[instruction_sequence] decimal(4,0) null,
[additional_notes] varchar(60) null,

)
go
create clustered columnstore index cci_VoltageInstructionInstruction2 on VoltageInstructionInstruction2;
go
                        
create table BillingConfigBillingcalendar2 (
file_log_id bigint not null references FileLog(id),
[contractyear] decimal(4,0) not null,
[weekno] decimal(3,0) not null,
[startdate] datetime2 null,
[enddate] datetime2 null,
[preliminarystatementdate] datetime2 null,
[finalstatementdate] datetime2 null,
[paymentdate] datetime2 null,
[lastchanged] datetime2 null,
[revision1_statementdate] datetime2 null,
[revision2_statementdate] datetime2 null,

)
go
create clustered columnstore index cci_BillingConfigBillingcalendar2 on BillingConfigBillingcalendar2;
go
                        
create table BillingConfigGstTransactionClass1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[transaction_type] varchar(30) not null,
[bas_class] varchar(30) not null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingConfigGstTransactionClass1 on BillingConfigGstTransactionClass1;
go
                        
create table BillingConfigGstBasClass1 (
file_log_id bigint not null references FileLog(id),
[bas_class] varchar(30) not null,
[description] varchar(100) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingConfigGstBasClass1 on BillingConfigGstBasClass1;
go
                        
create table BillingConfigSecdepositProvision1 (
file_log_id bigint not null references FileLog(id),
[security_deposit_id] varchar(20) not null,
[participantid] varchar(20) not null,
[transaction_date] datetime2 null,
[maturity_contractyear] decimal(4,0) null,
[maturity_weekno] decimal(3,0) null,
[amount] decimal(18,8) null,
[interest_rate] decimal(18,8) null,
[interest_calc_type] varchar(20) null,
[interest_acct_id] varchar(20) null,

)
go
create clustered columnstore index cci_BillingConfigSecdepositProvision1 on BillingConfigSecdepositProvision1;
go
                        
create table BillingConfigGstTransactionType1 (
file_log_id bigint not null references FileLog(id),
[transaction_type] varchar(30) not null,
[description] varchar(100) null,
[gl_financialcode] varchar(10) null,
[gl_tcode] varchar(15) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingConfigGstTransactionType1 on BillingConfigGstTransactionType1;
go
                        
create table BillingConfigSecdepositInterestRate1 (
file_log_id bigint not null references FileLog(id),
[interest_acct_id] varchar(20) not null,
[effectivedate] datetime2 not null,
[version_datetime] datetime2 not null,
[interest_rate] decimal(18,8) null,

)
go
create clustered columnstore index cci_BillingConfigSecdepositInterestRate1 on BillingConfigSecdepositInterestRate1;
go
                        
create table BillingConfigGstRate1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[bas_class] varchar(30) not null,
[gst_rate] decimal(8,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_BillingConfigGstRate1 on BillingConfigGstRate1;
go
                        
create table ForceMajeureOverriderrp1 (
file_log_id bigint not null references FileLog(id),
[regionid] varchar(10) not null,
[startdate] datetime2 not null,
[startperiod] decimal(3,0) not null,
[enddate] datetime2 null,
[endperiod] decimal(3,0) null,
[rrp] decimal(15,0) null,
[description] varchar(128) null,
[authorisestart] varchar(15) null,
[authoriseend] varchar(15) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ForceMajeureOverriderrp1 on ForceMajeureOverriderrp1;
go
                        
create table ForceMajeureIrfmevents1 (
file_log_id bigint not null references FileLog(id),
[irfmid] varchar(10) not null,
[startdate] datetime2 null,
[startperiod] decimal(3,0) null,
[enddate] datetime2 null,
[endperiod] decimal(3,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ForceMajeureIrfmevents1 on ForceMajeureIrfmevents1;
go
                        
create table ForceMajeureMarketSuspendRegionSum1 (
file_log_id bigint not null references FileLog(id),
[suspension_id] varchar(20) not null,
[regionid] varchar(20) not null,
[initial_interval] datetime2 null,
[end_region_interval] datetime2 null,
[end_suspension_interval] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ForceMajeureMarketSuspendRegionSum1 on ForceMajeureMarketSuspendRegionSum1;
go
                        
create table ForceMajeureMarketSuspendSchedule1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[day_type] varchar(20) not null,
[regionid] varchar(20) not null,
[periodid] decimal(3,0) not null,
[energy_rrp] decimal(15,5) null,
[r6_rrp] decimal(15,5) null,
[r60_rrp] decimal(15,5) null,
[r5_rrp] decimal(15,5) null,
[rreg_rrp] decimal(15,5) null,
[l6_rrp] decimal(15,5) null,
[l60_rrp] decimal(15,5) null,
[l5_rrp] decimal(15,5) null,
[lreg_rrp] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ForceMajeureMarketSuspendSchedule1 on ForceMajeureMarketSuspendSchedule1;
go
                        
create table ApApevent1 (
file_log_id bigint not null references FileLog(id),
[apeventid] decimal(22,0) not null,
[effectivefrominterval] datetime2 null,
[effectivetointerval] datetime2 null,
[reason] varchar(2000) null,
[startauthorisedby] varchar(15) null,
[startauthoriseddate] datetime2 null,
[endauthorisedby] varchar(15) null,
[endauthoriseddate] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ApApevent1 on ApApevent1;
go
                        
create table ForceMajeureIrfmamount1 (
file_log_id bigint not null references FileLog(id),
[irfmid] varchar(10) not null,
[effectivedate] datetime2 null,
[versionno] decimal(3,0) not null,
[periodid] decimal(4,0) not null,
[amount] decimal(15,5) null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ForceMajeureIrfmamount1 on ForceMajeureIrfmamount1;
go
                        
create table ApApeventregion1 (
file_log_id bigint not null references FileLog(id),
[apeventid] decimal(22,0) not null,
[regionid] varchar(10) not null,
[lastchanged] datetime2 null,
[energyapflag] decimal(1,0) null,
[raise6secapflag] decimal(1,0) null,
[raise60secapflag] decimal(1,0) null,
[raise5minapflag] decimal(1,0) null,
[raiseregapflag] decimal(1,0) null,
[lower6secapflag] decimal(1,0) null,
[lower60secapflag] decimal(1,0) null,
[lower5minapflag] decimal(1,0) null,
[lowerregapflag] decimal(1,0) null,

)
go
create clustered columnstore index cci_ApApeventregion1 on ApApeventregion1;
go
                        
create table ForceMajeureMarketSuspendRegimeSum1 (
file_log_id bigint not null references FileLog(id),
[suspension_id] varchar(20) not null,
[regionid] varchar(20) not null,
[start_interval] datetime2 not null,
[end_interval] datetime2 null,
[pricing_regime] varchar(20) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ForceMajeureMarketSuspendRegimeSum1 on ForceMajeureMarketSuspendRegimeSum1;
go
                        
create table ApRegionapcintervals1 (
file_log_id bigint not null references FileLog(id),
[regionid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[periodid] decimal(3,0) not null,
[apcvalue] decimal(16,6) null,
[lastchanged] datetime2 null,
[apctype] decimal(3,0) null,
[fcasapcvalue] decimal(16,6) null,
[apfvalue] decimal(16,6) null,

)
go
create clustered columnstore index cci_ApRegionapcintervals1 on ApRegionapcintervals1;
go
                        
create table ForceMajeureMarketSuspendScheduleTrk1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[source_start_date] datetime2 null,
[source_end_date] datetime2 null,
[comments] varchar(1000) null,
[authoriseddate] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ForceMajeureMarketSuspendScheduleTrk1 on ForceMajeureMarketSuspendScheduleTrk1;
go
                        
create table ApRegionapc1 (
file_log_id bigint not null references FileLog(id),
[regionid] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(10) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_ApRegionapc1 on ApRegionapc1;
go
                        
create table MccCasesolution1 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,

)
go
create clustered columnstore index cci_MccCasesolution1 on MccCasesolution1;
go
                        
create table MccConstraintsolution1 (
file_log_id bigint not null references FileLog(id),
[run_datetime] datetime2 not null,
[constraintid] varchar(20) not null,
[rhs] decimal(15,5) null,
[marginalvalue] decimal(15,5) null,

)
go
create clustered columnstore index cci_MccConstraintsolution1 on MccConstraintsolution1;
go
                        
create table NetworkOutagestatuscode1 (
file_log_id bigint not null references FileLog(id),
[outagestatuscode] varchar(10) not null,
[description] varchar(100) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_NetworkOutagestatuscode1 on NetworkOutagestatuscode1;
go
                        
create table NetworkStaticrating1 (
file_log_id bigint not null references FileLog(id),
[substationid] varchar(30) not null,
[equipmenttype] varchar(10) not null,
[equipmentid] varchar(30) not null,
[ratinglevel] varchar(10) not null,
[applicationid] varchar(20) not null,
[validfrom] datetime2 not null,
[validto] datetime2 null,
[ratingvalue] decimal(16,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_NetworkStaticrating1 on NetworkStaticrating1;
go
                        
create table NetworkEquipmentdetail1 (
file_log_id bigint not null references FileLog(id),
[substationid] varchar(30) not null,
[equipmenttype] varchar(10) not null,
[equipmentid] varchar(30) not null,
[validfrom] datetime2 not null,
[validto] datetime2 null,
[voltage] varchar(20) null,
[description] varchar(100) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_NetworkEquipmentdetail1 on NetworkEquipmentdetail1;
go
                        
create table NetworkRating1 (
file_log_id bigint not null references FileLog(id),
[spd_id] varchar(21) not null,
[validfrom] datetime2 not null,
[validto] datetime2 null,
[regionid] varchar(10) null,
[substationid] varchar(30) null,
[equipmenttype] varchar(10) null,
[equipmentid] varchar(30) null,
[ratinglevel] varchar(10) null,
[isdynamic] decimal(1,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_NetworkRating1 on NetworkRating1;
go
                        
create table NetworkOutageconstraintset1 (
file_log_id bigint not null references FileLog(id),
[outageid] decimal(15,0) not null,
[genconsetid] varchar(50) not null,
[startinterval] datetime2 null,
[endinterval] datetime2 null,

)
go
create clustered columnstore index cci_NetworkOutageconstraintset1 on NetworkOutageconstraintset1;
go
                        
create table NetworkSubstationdetail1 (
file_log_id bigint not null references FileLog(id),
[substationid] varchar(30) not null,
[validfrom] datetime2 not null,
[validto] datetime2 null,
[description] varchar(100) null,
[regionid] varchar(10) null,
[ownerid] varchar(30) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_NetworkSubstationdetail1 on NetworkSubstationdetail1;
go
                        
create table NetworkRealtimerating1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[spd_id] varchar(21) not null,
[ratingvalue] decimal(16,6) not null,

)
go
create clustered columnstore index cci_NetworkRealtimerating1 on NetworkRealtimerating1;
go
                        
create table NetworkOutagedetail3 (
file_log_id bigint not null references FileLog(id),
[outageid] decimal(15,0) not null,
[substationid] varchar(30) not null,
[equipmenttype] varchar(10) not null,
[equipmentid] varchar(30) not null,
[starttime] datetime2 not null,
[endtime] datetime2 null,
[submitteddate] datetime2 null,
[outagestatuscode] varchar(10) null,
[resubmitreason] varchar(50) null,
[resubmitoutageid] decimal(15,0) null,
[recalltimeday] decimal(10,0) null,
[recalltimenight] decimal(10,0) null,
[lastchanged] datetime2 null,
[reason] varchar(100) null,
[issecondary] decimal(1,0) null,
[actual_starttime] datetime2 null,
[actual_endtime] datetime2 null,
[companyrefcode] varchar(20) null,

)
go
create clustered columnstore index cci_NetworkOutagedetail3 on NetworkOutagedetail3;
go
                        
create table MarketConfigTransmissionlossfactor2 (
file_log_id bigint not null references FileLog(id),
[transmissionlossfactor] decimal(15,5) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(22,0) not null,
[connectionpointid] varchar(10) not null,
[regionid] varchar(10) null,
[lastchanged] datetime2 null,
[secondary_tlf] decimal(18,8) null,

)
go
create clustered columnstore index cci_MarketConfigTransmissionlossfactor2 on MarketConfigTransmissionlossfactor2;
go
                        
create table MarketConfigLossfactormodel1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[regionid] varchar(10) not null,
[demandcoefficient] decimal(27,17) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MarketConfigLossfactormodel1 on MarketConfigLossfactormodel1;
go
                        
create table MarketConfigRegionstandingdata1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[regionid] varchar(10) not null,
[rsoid] varchar(10) null,
[regionalreferencepointid] varchar(10) null,
[peaktradingperiod] decimal(3,0) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[scalingfactor] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MarketConfigRegionstandingdata1 on MarketConfigRegionstandingdata1;
go
                        
create table MarketConfigMarketPriceThresholds1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[versionno] decimal(4,0) not null,
[voll] decimal(15,5) null,
[marketpricefloor] decimal(15,5) null,
[administered_price_threshold] decimal(15,5) null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MarketConfigMarketPriceThresholds1 on MarketConfigMarketPriceThresholds1;
go
                        
create table MarketConfigBidtypes1 (
file_log_id bigint not null references FileLog(id),
[bidtype] varchar(10) not null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[description] varchar(64) null,
[numberofbands] decimal(3,0) null,
[numdaysaheadpricelocked] decimal(2,0) null,
[validationrule] varchar(10) null,
[lastchanged] datetime2 null,
[spdalias] varchar(10) null,

)
go
create clustered columnstore index cci_MarketConfigBidtypes1 on MarketConfigBidtypes1;
go
                        
create table MarketConfigInterconnectorconstraint1 (
file_log_id bigint not null references FileLog(id),
[reserveoverallloadfactor] decimal(5,2) null,
[fromregionlossshare] decimal(5,2) null,
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[maxmwin] decimal(15,5) null,
[maxmwout] decimal(15,5) null,
[lossconstant] decimal(15,6) null,
[lossflowcoefficient] decimal(27,17) null,
[emsmeasurand] varchar(40) null,
[authorisedby] varchar(15) null,
[authoriseddate] datetime2 null,
[dynamicrhs] varchar(1) null,
[importlimit] decimal(6,0) null,
[exportlimit] decimal(6,0) null,
[outagederationfactor] decimal(15,5) null,
[nonphysicallossfactor] decimal(15,5) null,
[overloadfactor60sec] decimal(15,5) null,
[overloadfactor6sec] decimal(15,5) null,
[lastchanged] datetime2 null,
[fcassupportunavailable] decimal(1,0) null,
[ictype] varchar(10) null,

)
go
create clustered columnstore index cci_MarketConfigInterconnectorconstraint1 on MarketConfigInterconnectorconstraint1;
go
                        
create table MarketConfigIntraregionalloc1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[versionno] decimal(5,0) not null,
[regionid] varchar(10) not null,
[participantid] varchar(10) not null,
[allocation] decimal(12,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MarketConfigIntraregionalloc1 on MarketConfigIntraregionalloc1;
go
                        
create table MarketConfigRegion1 (
file_log_id bigint not null references FileLog(id),
[regionid] varchar(10) not null,
[description] varchar(64) null,
[regionstatus] varchar(8) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MarketConfigRegion1 on MarketConfigRegion1;
go
                        
create table MarketConfigLossmodel1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[periodid] varchar(20) null,
[losssegment] decimal(6,0) not null,
[mwbreakpoint] decimal(6,0) null,
[lossfactor] decimal(16,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MarketConfigLossmodel1 on MarketConfigLossmodel1;
go
                        
create table MarketConfigBidtypestrk1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[versionno] decimal(3,0) not null,
[authoriseddate] datetime2 null,
[authorisedby] varchar(15) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MarketConfigBidtypestrk1 on MarketConfigBidtypestrk1;
go
                        
create table MarketConfigInterconnector1 (
file_log_id bigint not null references FileLog(id),
[interconnectorid] varchar(10) not null,
[regionfrom] varchar(10) null,
[rsoid] varchar(10) null,
[regionto] varchar(10) null,
[description] varchar(64) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MarketConfigInterconnector1 on MarketConfigInterconnector1;
go
                        
create table MarketConfigInterconnectoralloc1 (
file_log_id bigint not null references FileLog(id),
[effectivedate] datetime2 not null,
[versionno] decimal(5,0) not null,
[interconnectorid] varchar(10) not null,
[regionid] varchar(10) not null,
[participantid] varchar(10) not null,
[allocation] decimal(12,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_MarketConfigInterconnectoralloc1 on MarketConfigInterconnectoralloc1;
go
                        
create table DispatchIntermittentForecastTrk1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[duid] varchar(20) not null,
[origin] varchar(20) null,
[forecast_priority] decimal(10,0) null,
[offerdatetime] datetime2 null,

)
go
create clustered columnstore index cci_DispatchIntermittentForecastTrk1 on DispatchIntermittentForecastTrk1;
go
                        
create table DispatchMrScheduleTrk1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[regionid] varchar(10) not null,
[mr_date] datetime2 null,
[version_datetime] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_DispatchMrScheduleTrk1 on DispatchMrScheduleTrk1;
go
                        
create table DispatchUnitSolution2 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[duid] varchar(10) not null,
[tradetype] decimal(2,0) null,
[dispatchinterval] decimal(22,0) null,
[intervention] decimal(2,0) not null,
[connectionpointid] varchar(12) null,
[dispatchmode] decimal(2,0) null,
[agcstatus] decimal(2,0) null,
[initialmw] decimal(15,5) null,
[totalcleared] decimal(15,5) null,
[rampdownrate] decimal(15,5) null,
[rampuprate] decimal(15,5) null,
[lower5min] decimal(15,5) null,
[lower60sec] decimal(15,5) null,
[lower6sec] decimal(15,5) null,
[raise5min] decimal(15,5) null,
[raise60sec] decimal(15,5) null,
[raise6sec] decimal(15,5) null,
[downepf] decimal(15,5) null,
[upepf] decimal(15,5) null,
[marginal5minvalue] decimal(15,5) null,
[marginal60secvalue] decimal(15,5) null,
[marginal6secvalue] decimal(15,5) null,
[marginalvalue] decimal(15,5) null,
[violation5mindegree] decimal(15,5) null,
[violation60secdegree] decimal(15,5) null,
[violation6secdegree] decimal(15,5) null,
[violationdegree] decimal(15,5) null,
[lastchanged] datetime2 null,
[lowerreg] decimal(15,5) null,
[raisereg] decimal(15,5) null,
[availability] decimal(15,5) null,
[raise6secflags] decimal(3,0) null,
[raise60secflags] decimal(3,0) null,
[raise5minflags] decimal(3,0) null,
[raiseregflags] decimal(3,0) null,
[lower6secflags] decimal(3,0) null,
[lower60secflags] decimal(3,0) null,
[lower5minflags] decimal(3,0) null,
[lowerregflags] decimal(3,0) null,
[raiseregavailability] decimal(15,5) null,
[raiseregenablementmax] decimal(15,5) null,
[raiseregenablementmin] decimal(15,5) null,
[lowerregavailability] decimal(15,5) null,
[lowerregenablementmax] decimal(15,5) null,
[lowerregenablementmin] decimal(15,5) null,
[raise6secactualavailability] decimal(16,6) null,
[raise60secactualavailability] decimal(16,6) null,
[raise5minactualavailability] decimal(16,6) null,
[raiseregactualavailability] decimal(16,6) null,
[lower6secactualavailability] decimal(16,6) null,
[lower60secactualavailability] decimal(16,6) null,
[lower5minactualavailability] decimal(16,6) null,
[lowerregactualavailability] decimal(16,6) null,
[semidispatchcap] decimal(3,0) null,

)
go
create clustered columnstore index cci_DispatchUnitSolution2 on DispatchUnitSolution2;
go
                        
create table PriceloadPriceRevision1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[intervention] decimal(2,0) not null,
[regionid] varchar(10) not null,
[bidtype] varchar(10) not null,
[versionno] decimal(3,0) not null,
[rrp_new] decimal(15,5) null,
[rrp_old] decimal(15,5) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_PriceloadPriceRevision1 on PriceloadPriceRevision1;
go
                        
create table DispatchPrice4 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[regionid] varchar(10) not null,
[dispatchinterval] varchar(22) not null,
[intervention] decimal(2,0) not null,
[rrp] decimal(15,5) null,
[eep] decimal(15,5) null,
[rop] decimal(15,5) null,
[apcflag] decimal(3,0) null,
[marketsuspendedflag] decimal(3,0) null,
[lastchanged] datetime2 null,
[raise6secrrp] decimal(15,5) null,
[raise6secrop] decimal(15,5) null,
[raise6secapcflag] decimal(3,0) null,
[raise60secrrp] decimal(15,5) null,
[raise60secrop] decimal(15,5) null,
[raise60secapcflag] decimal(3,0) null,
[raise5minrrp] decimal(15,5) null,
[raise5minrop] decimal(15,5) null,
[raise5minapcflag] decimal(3,0) null,
[raiseregrrp] decimal(15,5) null,
[raiseregrop] decimal(15,5) null,
[raiseregapcflag] decimal(3,0) null,
[lower6secrrp] decimal(15,5) null,
[lower6secrop] decimal(15,5) null,
[lower6secapcflag] decimal(3,0) null,
[lower60secrrp] decimal(15,5) null,
[lower60secrop] decimal(15,5) null,
[lower60secapcflag] decimal(3,0) null,
[lower5minrrp] decimal(15,5) null,
[lower5minrop] decimal(15,5) null,
[lower5minapcflag] decimal(3,0) null,
[lowerregrrp] decimal(15,5) null,
[lowerregrop] decimal(15,5) null,
[lowerregapcflag] decimal(3,0) null,
[price_status] varchar(20) null,
[pre_ap_energy_price] decimal(15,5) null,
[pre_ap_raise6_price] decimal(15,5) null,
[pre_ap_raise60_price] decimal(15,5) null,
[pre_ap_raise5min_price] decimal(15,5) null,
[pre_ap_raisereg_price] decimal(15,5) null,
[pre_ap_lower6_price] decimal(15,5) null,
[pre_ap_lower60_price] decimal(15,5) null,
[pre_ap_lower5min_price] decimal(15,5) null,
[pre_ap_lowerreg_price] decimal(15,5) null,
[cumul_pre_ap_energy_price] decimal(15,5) null,
[cumul_pre_ap_raise6_price] decimal(15,5) null,
[cumul_pre_ap_raise60_price] decimal(15,5) null,
[cumul_pre_ap_raise5min_price] decimal(15,5) null,
[cumul_pre_ap_raisereg_price] decimal(15,5) null,
[cumul_pre_ap_lower6_price] decimal(15,5) null,
[cumul_pre_ap_lower60_price] decimal(15,5) null,
[cumul_pre_ap_lower5min_price] decimal(15,5) null,
[cumul_pre_ap_lowerreg_price] decimal(15,5) null,
[ocd_status] varchar(14) null,
[mii_status] varchar(21) null,

)
go
create clustered columnstore index cci_DispatchPrice4 on DispatchPrice4;
go
                        
create table DispatchUnitConformance1 (
file_log_id bigint not null references FileLog(id),
[interval_datetime] datetime2 not null,
[duid] varchar(20) not null,
[totalcleared] decimal(16,6) null,
[actualmw] decimal(16,6) null,
[roc] decimal(16,6) null,
[availability] decimal(16,6) null,
[lowerreg] decimal(16,6) null,
[raisereg] decimal(16,6) null,
[striglm] decimal(16,6) null,
[ltriglm] decimal(16,6) null,
[mwerror] decimal(16,6) null,
[max_mwerror] decimal(16,6) null,
[lecount] decimal(6,0) null,
[secount] decimal(6,0) null,
[status] varchar(20) null,
[participant_status_action] varchar(100) null,
[operating_mode] varchar(20) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_DispatchUnitConformance1 on DispatchUnitConformance1;
go
                        
create table DispatchBlockedConstraints1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[constraintid] varchar(20) not null,

)
go
create clustered columnstore index cci_DispatchBlockedConstraints1 on DispatchBlockedConstraints1;
go
                        
create table DispatchFcasReq2 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[intervention] decimal(2,0) not null,
[genconid] varchar(20) not null,
[regionid] varchar(10) not null,
[bidtype] varchar(10) not null,
[genconeffectivedate] datetime2 null,
[genconversionno] decimal(3,0) null,
[marginalvalue] decimal(16,6) null,
[lastchanged] datetime2 null,
[base_cost] decimal(18,8) null,
[adjusted_cost] decimal(18,8) null,
[estimated_cmpf] decimal(18,8) null,
[estimated_crmpf] decimal(18,8) null,
[recovery_factor_cmpf] decimal(18,8) null,
[recovery_factor_crmpf] decimal(18,8) null,

)
go
create clustered columnstore index cci_DispatchFcasReq2 on DispatchFcasReq2;
go
                        
create table DispatchLocalPrice1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[duid] varchar(20) not null,
[local_price_adjustment] decimal(10,2) null,
[locally_constrained] decimal(1,0) null,

)
go
create clustered columnstore index cci_DispatchLocalPrice1 on DispatchLocalPrice1;
go
                        
create table DispatchInterconnection1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[intervention] decimal(2,0) not null,
[from_regionid] varchar(20) not null,
[to_regionid] varchar(20) not null,
[dispatchinterval] decimal(22,0) null,
[irlf] decimal(15,5) null,
[mwflow] decimal(16,6) null,
[meteredmwflow] decimal(16,6) null,
[from_region_mw_losses] decimal(16,6) null,
[to_region_mw_losses] decimal(16,6) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_DispatchInterconnection1 on DispatchInterconnection1;
go
                        
create table DispatchNegativeResidue1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[nrm_datetime] datetime2 not null,
[directional_interconnectorid] varchar(30) not null,
[nrm_activated_flag] decimal(1,0) null,
[cumul_negresidue_amount] decimal(15,5) null,
[cumul_negresidue_prev_ti] decimal(15,5) null,
[negresidue_current_ti] decimal(15,5) null,
[negresidue_pd_next_ti] decimal(15,5) null,
[price_revision] varchar(30) null,
[predispatchseqno] varchar(20) null,
[event_activated_di] datetime2 null,
[event_deactivated_di] datetime2 null,
[di_notbinding_count] decimal(2,0) null,
[di_violated_count] decimal(2,0) null,
[nrmconstraint_blocked_flag] decimal(1,0) null,

)
go
create clustered columnstore index cci_DispatchNegativeResidue1 on DispatchNegativeResidue1;
go
                        
create table DispatchRegionsum4 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[regionid] varchar(10) not null,
[dispatchinterval] decimal(22,0) not null,
[intervention] decimal(2,0) not null,
[totaldemand] decimal(15,5) null,
[availablegeneration] decimal(15,5) null,
[availableload] decimal(15,5) null,
[demandforecast] decimal(15,5) null,
[dispatchablegeneration] decimal(15,5) null,
[dispatchableload] decimal(15,5) null,
[netinterchange] decimal(15,5) null,
[excessgeneration] decimal(15,5) null,
[lower5mindispatch] decimal(15,5) null,
[lower5minimport] decimal(15,5) null,
[lower5minlocaldispatch] decimal(15,5) null,
[lower5minlocalprice] decimal(15,5) null,
[lower5minlocalreq] decimal(15,5) null,
[lower5minprice] decimal(15,5) null,
[lower5minreq] decimal(15,5) null,
[lower5minsupplyprice] decimal(15,5) null,
[lower60secdispatch] decimal(15,5) null,
[lower60secimport] decimal(15,5) null,
[lower60seclocaldispatch] decimal(15,5) null,
[lower60seclocalprice] decimal(15,5) null,
[lower60seclocalreq] decimal(15,5) null,
[lower60secprice] decimal(15,5) null,
[lower60secreq] decimal(15,5) null,
[lower60secsupplyprice] decimal(15,5) null,
[lower6secdispatch] decimal(15,5) null,
[lower6secimport] decimal(15,5) null,
[lower6seclocaldispatch] decimal(15,5) null,
[lower6seclocalprice] decimal(15,5) null,
[lower6seclocalreq] decimal(15,5) null,
[lower6secprice] decimal(15,5) null,
[lower6secreq] decimal(15,5) null,
[lower6secsupplyprice] decimal(15,5) null,
[raise5mindispatch] decimal(15,5) null,
[raise5minimport] decimal(15,5) null,
[raise5minlocaldispatch] decimal(15,5) null,
[raise5minlocalprice] decimal(15,5) null,
[raise5minlocalreq] decimal(15,5) null,
[raise5minprice] decimal(15,5) null,
[raise5minreq] decimal(15,5) null,
[raise5minsupplyprice] decimal(15,5) null,
[raise60secdispatch] decimal(15,5) null,
[raise60secimport] decimal(15,5) null,
[raise60seclocaldispatch] decimal(15,5) null,
[raise60seclocalprice] decimal(15,5) null,
[raise60seclocalreq] decimal(15,5) null,
[raise60secprice] decimal(15,5) null,
[raise60secreq] decimal(15,5) null,
[raise60secsupplyprice] decimal(15,5) null,
[raise6secdispatch] decimal(15,5) null,
[raise6secimport] decimal(15,5) null,
[raise6seclocaldispatch] decimal(15,5) null,
[raise6seclocalprice] decimal(15,5) null,
[raise6seclocalreq] decimal(15,5) null,
[raise6secprice] decimal(15,5) null,
[raise6secreq] decimal(15,5) null,
[raise6secsupplyprice] decimal(15,5) null,
[aggegatedispatcherror] decimal(15,5) null,
[aggregatedispatcherror] decimal(15,5) null,
[lastchanged] datetime2 null,
[initialsupply] decimal(15,5) null,
[clearedsupply] decimal(15,5) null,
[lowerregimport] decimal(15,5) null,
[lowerreglocaldispatch] decimal(15,5) null,
[lowerreglocalreq] decimal(15,5) null,
[lowerregreq] decimal(15,5) null,
[raiseregimport] decimal(15,5) null,
[raisereglocaldispatch] decimal(15,5) null,
[raisereglocalreq] decimal(15,5) null,
[raiseregreq] decimal(15,5) null,
[raise5minlocalviolation] decimal(15,5) null,
[raisereglocalviolation] decimal(15,5) null,
[raise60seclocalviolation] decimal(15,5) null,
[raise6seclocalviolation] decimal(15,5) null,
[lower5minlocalviolation] decimal(15,5) null,
[lowerreglocalviolation] decimal(15,5) null,
[lower60seclocalviolation] decimal(15,5) null,
[lower6seclocalviolation] decimal(15,5) null,
[raise5minviolation] decimal(15,5) null,
[raiseregviolation] decimal(15,5) null,
[raise60secviolation] decimal(15,5) null,
[raise6secviolation] decimal(15,5) null,
[lower5minviolation] decimal(15,5) null,
[lowerregviolation] decimal(15,5) null,
[lower60secviolation] decimal(15,5) null,
[lower6secviolation] decimal(15,5) null,
[raise6secactualavailability] decimal(16,6) null,
[raise60secactualavailability] decimal(16,6) null,
[raise5minactualavailability] decimal(16,6) null,
[raiseregactualavailability] decimal(16,6) null,
[lower6secactualavailability] decimal(16,6) null,
[lower60secactualavailability] decimal(16,6) null,
[lower5minactualavailability] decimal(16,6) null,
[lowerregactualavailability] decimal(16,6) null,
[lorsurplus] decimal(16,6) null,
[lrcsurplus] decimal(16,6) null,
[totalintermittentgeneration] decimal(15,5) null,
[demand_and_nonschedgen] decimal(15,5) null,
[uigf] decimal(15,5) null,
[semischedule_clearedmw] decimal(15,5) null,
[semischedule_compliancemw] decimal(15,5) null,
[ss_solar_uigf] decimal(15,5) null,
[ss_wind_uigf] decimal(15,5) null,
[ss_solar_clearedmw] decimal(15,5) null,
[ss_wind_clearedmw] decimal(15,5) null,
[ss_solar_compliancemw] decimal(15,5) null,
[ss_wind_compliancemw] decimal(15,5) null,

)
go
create clustered columnstore index cci_DispatchRegionsum4 on DispatchRegionsum4;
go
                        
create table DispatchOffertrk1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[duid] varchar(10) not null,
[bidtype] varchar(10) not null,
[bidsettlementdate] datetime2 null,
[bidofferdate] datetime2 null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_DispatchOffertrk1 on DispatchOffertrk1;
go
                        
create table DispatchMnspbidtrk1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[participantid] varchar(10) not null,
[linkid] varchar(10) not null,
[offersettlementdate] datetime2 null,
[offereffectivedate] datetime2 null,
[offerversionno] decimal(3,0) null,
[lastchanged] datetime2 null,

)
go
create clustered columnstore index cci_DispatchMnspbidtrk1 on DispatchMnspbidtrk1;
go
                        
create table DispatchInterconnectorres3 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[interconnectorid] varchar(10) not null,
[dispatchinterval] decimal(22,0) not null,
[intervention] decimal(2,0) not null,
[meteredmwflow] decimal(15,5) null,
[mwflow] decimal(15,5) null,
[mwlosses] decimal(15,5) null,
[marginalvalue] decimal(15,5) null,
[violationdegree] decimal(15,5) null,
[lastchanged] datetime2 null,
[exportlimit] decimal(15,5) null,
[importlimit] decimal(15,5) null,
[marginalloss] decimal(15,5) null,
[exportgenconid] varchar(20) null,
[importgenconid] varchar(20) null,
[fcasexportlimit] decimal(15,5) null,
[fcasimportlimit] decimal(15,5) null,
[local_price_adjustment_export] decimal(10,2) null,
[locally_constrained_export] decimal(1,0) null,
[local_price_adjustment_import] decimal(10,2) null,
[locally_constrained_import] decimal(1,0) null,

)
go
create clustered columnstore index cci_DispatchInterconnectorres3 on DispatchInterconnectorres3;
go
                        
create table DispatchConstraint5 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[constraintid] varchar(20) not null,
[dispatchinterval] decimal(22,0) not null,
[intervention] decimal(2,0) not null,
[rhs] decimal(15,5) null,
[marginalvalue] decimal(15,5) null,
[violationdegree] decimal(15,5) null,
[lastchanged] datetime2 null,
[duid] varchar(20) null,
[genconid_effectivedate] datetime2 null,
[genconid_versionno] decimal(22,0) null,
[lhs] decimal(15,5) null,

)
go
create clustered columnstore index cci_DispatchConstraint5 on DispatchConstraint5;
go
                        
create table PriceloadConstraintFcasOcd1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[intervention] decimal(2,0) not null,
[constraintid] varchar(20) not null,
[versionno] decimal(3,0) not null,
[lastchanged] datetime2 null,
[rhs] decimal(15,5) null,
[marginalvalue] decimal(15,5) null,
[violationdegree] decimal(15,5) null,

)
go
create clustered columnstore index cci_PriceloadConstraintFcasOcd1 on PriceloadConstraintFcasOcd1;
go
                        
create table PriceloadConstraintrelaxation1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[constraintid] varchar(20) not null,
[rhs] decimal(16,6) null,
[lastchanged] datetime2 null,
[versionno] decimal(3,0) not null,

)
go
create clustered columnstore index cci_PriceloadConstraintrelaxation1 on PriceloadConstraintrelaxation1;
go
                        
create table DispatchUnitScada1 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[duid] varchar(20) not null,
[scadavalue] decimal(16,6) null,

)
go
create clustered columnstore index cci_DispatchUnitScada1 on DispatchUnitScada1;
go
                        
create table DispatchCaseSolution2 (
file_log_id bigint not null references FileLog(id),
[settlementdate] datetime2 not null,
[runno] decimal(3,0) not null,
[intervention] decimal(2,0) not null,
[casesubtype] varchar(3) null,
[solutionstatus] decimal(2,0) null,
[spdversion] varchar(20) null,
[nonphysicallosses] decimal(1,0) null,
[totalobjective] decimal(27,10) null,
[totalareagenviolation] decimal(15,5) null,
[totalinterconnectorviolation] decimal(15,5) null,
[totalgenericviolation] decimal(15,5) null,
[totalramprateviolation] decimal(15,5) null,
[totalunitmwcapacityviolation] decimal(15,5) null,
[total5minviolation] decimal(15,5) null,
[totalregviolation] decimal(15,5) null,
[total6secviolation] decimal(15,5) null,
[total60secviolation] decimal(15,5) null,
[totalasprofileviolation] decimal(15,5) null,
[totalfaststartviolation] decimal(15,5) null,
[totalenergyofferviolation] decimal(15,5) null,
[lastchanged] datetime2 null,
[switchruninitialstatus] decimal(1,0) null,
[switchrunbeststatus] decimal(1,0) null,
[switchrunbeststatus_int] decimal(1,0) null,

)
go
create clustered columnstore index cci_DispatchCaseSolution2 on DispatchCaseSolution2;
go
                        