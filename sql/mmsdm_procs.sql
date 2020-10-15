
create or alter procedure InsertForecastIntermittentGen1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Forecast'
    and sub_type = 'IntermittentGen'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Forecast'
    and sub_type = 'IntermittentGen'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Forecast', 'IntermittentGen', 1);

insert into ForecastIntermittentGen1(
file_log_id,
[run_datetime],
[duid],
[start_interval_datetime],
[end_interval_datetime],
[versionno],
[lastchanged]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[duid],
d.[start_interval_datetime],
d.[end_interval_datetime],
d.[versionno],
d.[lastchanged]
from openjson(@data) with (
[run_datetime] datetime2,
[duid] varchar(20),
[start_interval_datetime] datetime2,
[end_interval_datetime] datetime2,
[versionno] decimal(10,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertOperationalDemandForecast1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'OperationalDemand'
    and sub_type = 'Forecast'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'OperationalDemand'
    and sub_type = 'Forecast'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'OperationalDemand', 'Forecast', 1);

insert into OperationalDemandForecast1(
file_log_id,
[interval_datetime],
[regionid],
[load_date],
[operational_demand_poe10],
[operational_demand_poe50],
[operational_demand_poe90],
[lastchanged]
)
select 
(select h.id from @header h),
d.[interval_datetime],
d.[regionid],
d.[load_date],
d.[operational_demand_poe10],
d.[operational_demand_poe50],
d.[operational_demand_poe90],
d.[lastchanged]
from openjson(@data) with (
[interval_datetime] datetime2,
[regionid] varchar(20),
[load_date] datetime2,
[operational_demand_poe10] decimal(15,2),
[operational_demand_poe50] decimal(15,2),
[operational_demand_poe90] decimal(15,2),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertDemandIntermittentClusterAvailDay1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'IntermittentClusterAvailDay'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'IntermittentClusterAvailDay'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Demand', 'IntermittentClusterAvailDay', 1);

insert into DemandIntermittentClusterAvailDay1(
file_log_id,
[tradingdate],
[duid],
[offerdatetime],
[clusterid]
)
select 
(select h.id from @header h),
d.[tradingdate],
d.[duid],
d.[offerdatetime],
d.[clusterid]
from openjson(@data) with (
[tradingdate] datetime2,
[duid] varchar(20),
[offerdatetime] datetime2,
[clusterid] varchar(20)
) d
end
go
create or alter procedure InsertDemandIntermittentClusterAvail1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'IntermittentClusterAvail'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'IntermittentClusterAvail'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Demand', 'IntermittentClusterAvail', 1);

insert into DemandIntermittentClusterAvail1(
file_log_id,
[tradingdate],
[duid],
[offerdatetime],
[clusterid],
[periodid],
[elements_unavailable]
)
select 
(select h.id from @header h),
d.[tradingdate],
d.[duid],
d.[offerdatetime],
d.[clusterid],
d.[periodid],
d.[elements_unavailable]
from openjson(@data) with (
[tradingdate] datetime2,
[duid] varchar(20),
[offerdatetime] datetime2,
[clusterid] varchar(20),
[periodid] decimal(3,0),
[elements_unavailable] decimal(3,0)
) d
end
go
create or alter procedure InsertDemandPeriod1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'Period'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'Period'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Demand', 'Period', 1);

insert into DemandPeriod1(
file_log_id,
[effectivedate],
[settlementdate],
[regionid],
[offerdate],
[periodid],
[versionno],
[resdemand],
[demand90probability],
[demand10probability],
[lastchanged],
[mr_schedule]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[settlementdate],
d.[regionid],
d.[offerdate],
d.[periodid],
d.[versionno],
d.[resdemand],
d.[demand90probability],
d.[demand10probability],
d.[lastchanged],
d.[mr_schedule]
from openjson(@data) with (
[effectivedate] datetime2,
[settlementdate] datetime2,
[regionid] varchar(10),
[offerdate] datetime2,
[periodid] decimal(3,0),
[versionno] decimal(3,0),
[resdemand] decimal(10,0),
[demand90probability] decimal(10,0),
[demand10probability] decimal(10,0),
[lastchanged] datetime2,
[mr_schedule] decimal(6,0)
) d
end
go
create or alter procedure InsertDemandIntermittentDsRun1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'IntermittentDsRun'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'IntermittentDsRun'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Demand', 'IntermittentDsRun', 1);

insert into DemandIntermittentDsRun1(
file_log_id,
[run_datetime],
[duid],
[offerdatetime],
[origin],
[forecast_priority],
[authorisedby],
[comments],
[lastchanged],
[model],
[participant_timestamp],
[suppressed_aemo],
[suppressed_participant],
[transaction_id]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[duid],
d.[offerdatetime],
d.[origin],
d.[forecast_priority],
d.[authorisedby],
d.[comments],
d.[lastchanged],
d.[model],
d.[participant_timestamp],
d.[suppressed_aemo],
d.[suppressed_participant],
d.[transaction_id]
from openjson(@data) with (
[run_datetime] datetime2,
[duid] varchar(20),
[offerdatetime] datetime2,
[origin] varchar(20),
[forecast_priority] decimal(10,0),
[authorisedby] varchar(20),
[comments] varchar(200),
[lastchanged] datetime2,
[model] varchar(30),
[participant_timestamp] datetime2,
[suppressed_aemo] decimal(1,0),
[suppressed_participant] decimal(1,0),
[transaction_id] varchar(100)
) d
end
go
create or alter procedure InsertForecastIntermittentGenData1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Forecast'
    and sub_type = 'IntermittentGenData'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Forecast'
    and sub_type = 'IntermittentGenData'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Forecast', 'IntermittentGenData', 1);

insert into ForecastIntermittentGenData1(
file_log_id,
[run_datetime],
[duid],
[interval_datetime],
[powermean],
[powerpoe50],
[powerpoelow],
[powerpoehigh],
[lastchanged]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[duid],
d.[interval_datetime],
d.[powermean],
d.[powerpoe50],
d.[powerpoelow],
d.[powerpoehigh],
d.[lastchanged]
from openjson(@data) with (
[run_datetime] datetime2,
[duid] varchar(20),
[interval_datetime] datetime2,
[powermean] decimal(9,3),
[powerpoe50] decimal(9,3),
[powerpoelow] decimal(9,3),
[powerpoehigh] decimal(9,3),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertDemandIntermittentDsPred1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'IntermittentDsPred'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'IntermittentDsPred'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Demand', 'IntermittentDsPred', 1);

insert into DemandIntermittentDsPred1(
file_log_id,
[run_datetime],
[duid],
[offerdatetime],
[interval_datetime],
[origin],
[forecast_priority],
[forecast_mean],
[forecast_poe10],
[forecast_poe50],
[forecast_poe90]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[duid],
d.[offerdatetime],
d.[interval_datetime],
d.[origin],
d.[forecast_priority],
d.[forecast_mean],
d.[forecast_poe10],
d.[forecast_poe50],
d.[forecast_poe90]
from openjson(@data) with (
[run_datetime] datetime2,
[duid] varchar(20),
[offerdatetime] datetime2,
[interval_datetime] datetime2,
[origin] varchar(20),
[forecast_priority] decimal(10,0),
[forecast_mean] decimal(18,8),
[forecast_poe10] decimal(18,8),
[forecast_poe50] decimal(18,8),
[forecast_poe90] decimal(18,8)
) d
end
go
create or alter procedure InsertDemandMtpasaIntermittentAvail1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'MtpasaIntermittentAvail'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'MtpasaIntermittentAvail'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Demand', 'MtpasaIntermittentAvail', 1);

insert into DemandMtpasaIntermittentAvail1(
file_log_id,
[tradingdate],
[duid],
[offerdatetime],
[clusterid],
[lastchanged],
[elements_unavailable]
)
select 
(select h.id from @header h),
d.[tradingdate],
d.[duid],
d.[offerdatetime],
d.[clusterid],
d.[lastchanged],
d.[elements_unavailable]
from openjson(@data) with (
[tradingdate] datetime2,
[duid] varchar(20),
[offerdatetime] datetime2,
[clusterid] varchar(20),
[lastchanged] datetime2,
[elements_unavailable] decimal(3,0)
) d
end
go
create or alter procedure InsertRooftopActual2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Rooftop'
    and sub_type = 'Actual'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Rooftop'
    and sub_type = 'Actual'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Rooftop', 'Actual', 2);

insert into RooftopActual2(
file_log_id,
[interval_datetime],
[type],
[regionid],
[power],
[qi],
[lastchanged]
)
select 
(select h.id from @header h),
d.[interval_datetime],
d.[type],
d.[regionid],
d.[power],
d.[qi],
d.[lastchanged]
from openjson(@data) with (
[interval_datetime] datetime2,
[type] varchar(20),
[regionid] varchar(20),
[power] decimal(12,3),
[qi] decimal(2,1),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertRooftopForecast1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Rooftop'
    and sub_type = 'Forecast'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Rooftop'
    and sub_type = 'Forecast'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Rooftop', 'Forecast', 1);

insert into RooftopForecast1(
file_log_id,
[version_datetime],
[regionid],
[interval_datetime],
[powermean],
[powerpoe50],
[powerpoelow],
[powerpoehigh],
[lastchanged]
)
select 
(select h.id from @header h),
d.[version_datetime],
d.[regionid],
d.[interval_datetime],
d.[powermean],
d.[powerpoe50],
d.[powerpoelow],
d.[powerpoehigh],
d.[lastchanged]
from openjson(@data) with (
[version_datetime] datetime2,
[regionid] varchar(20),
[interval_datetime] datetime2,
[powermean] decimal(12,3),
[powerpoe50] decimal(12,3),
[powerpoelow] decimal(12,3),
[powerpoehigh] decimal(12,3),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertDemandMtpasaIntermittentLimit1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'MtpasaIntermittentLimit'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'MtpasaIntermittentLimit'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Demand', 'MtpasaIntermittentLimit', 1);

insert into DemandMtpasaIntermittentLimit1(
file_log_id,
[tradingdate],
[duid],
[offerdatetime],
[lastchanged],
[uppermwlimit],
[authorisedbyuser],
[authorisedbyparticipantid]
)
select 
(select h.id from @header h),
d.[tradingdate],
d.[duid],
d.[offerdatetime],
d.[lastchanged],
d.[uppermwlimit],
d.[authorisedbyuser],
d.[authorisedbyparticipantid]
from openjson(@data) with (
[tradingdate] datetime2,
[duid] varchar(20),
[offerdatetime] datetime2,
[lastchanged] datetime2,
[uppermwlimit] decimal(6,0),
[authorisedbyuser] varchar(20),
[authorisedbyparticipantid] varchar(20)
) d
end
go
create or alter procedure InsertOperationalDemandActual1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'OperationalDemand'
    and sub_type = 'Actual'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'OperationalDemand'
    and sub_type = 'Actual'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'OperationalDemand', 'Actual', 1);

insert into OperationalDemandActual1(
file_log_id,
[interval_datetime],
[regionid],
[operational_demand],
[lastchanged]
)
select 
(select h.id from @header h),
d.[interval_datetime],
d.[regionid],
d.[operational_demand],
d.[lastchanged]
from openjson(@data) with (
[interval_datetime] datetime2,
[regionid] varchar(20),
[operational_demand] decimal(10,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertDemandTrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'Trk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'Trk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Demand', 'Trk', 1);

insert into DemandTrk1(
file_log_id,
[effectivedate],
[regionid],
[offerdate],
[versionno],
[filename],
[authoriseddate],
[authorisedby],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[regionid],
d.[offerdate],
d.[versionno],
d.[filename],
d.[authoriseddate],
d.[authorisedby],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[regionid] varchar(10),
[offerdate] datetime2,
[versionno] decimal(3,0),
[filename] varchar(40),
[authoriseddate] datetime2,
[authorisedby] varchar(10),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertDemandIntermittentGenLimit1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'IntermittentGenLimit'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'IntermittentGenLimit'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Demand', 'IntermittentGenLimit', 1);

insert into DemandIntermittentGenLimit1(
file_log_id,
[tradingdate],
[duid],
[offerdatetime],
[periodid],
[uppermwlimit]
)
select 
(select h.id from @header h),
d.[tradingdate],
d.[duid],
d.[offerdatetime],
d.[periodid],
d.[uppermwlimit]
from openjson(@data) with (
[tradingdate] datetime2,
[duid] varchar(20),
[offerdatetime] datetime2,
[periodid] decimal(3,0),
[uppermwlimit] decimal(6,0)
) d
end
go
create or alter procedure InsertDemandIntermittentGenLimitDay1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'IntermittentGenLimitDay'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Demand'
    and sub_type = 'IntermittentGenLimitDay'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Demand', 'IntermittentGenLimitDay', 1);

insert into DemandIntermittentGenLimitDay1(
file_log_id,
[tradingdate],
[duid],
[offerdatetime],
[participantid],
[lastchanged],
[authorisedbyuser],
[authorisedbyparticipantid]
)
select 
(select h.id from @header h),
d.[tradingdate],
d.[duid],
d.[offerdatetime],
d.[participantid],
d.[lastchanged],
d.[authorisedbyuser],
d.[authorisedbyparticipantid]
from openjson(@data) with (
[tradingdate] datetime2,
[duid] varchar(20),
[offerdatetime] datetime2,
[participantid] varchar(20),
[lastchanged] datetime2,
[authorisedbyuser] varchar(20),
[authorisedbyparticipantid] varchar(20)
) d
end
go
create or alter procedure InsertAsofferOfferagcdata1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Asoffer'
    and sub_type = 'Offeragcdata'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Asoffer'
    and sub_type = 'Offeragcdata'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Asoffer', 'Offeragcdata', 1);

insert into AsofferOfferagcdata1(
file_log_id,
[contractid],
[effectivedate],
[versionno],
[availability],
[upperlimit],
[lowerlimit],
[authoriseddate],
[authorisedby],
[filename],
[lastchanged],
[periodid],
[agcup],
[agcdown]
)
select 
(select h.id from @header h),
d.[contractid],
d.[effectivedate],
d.[versionno],
d.[availability],
d.[upperlimit],
d.[lowerlimit],
d.[authoriseddate],
d.[authorisedby],
d.[filename],
d.[lastchanged],
d.[periodid],
d.[agcup],
d.[agcdown]
from openjson(@data) with (
[contractid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[availability] decimal(4,0),
[upperlimit] decimal(4,0),
[lowerlimit] decimal(4,0),
[authoriseddate] datetime2,
[authorisedby] varchar(15),
[filename] varchar(40),
[lastchanged] datetime2,
[periodid] decimal(3,0),
[agcup] decimal(3,0),
[agcdown] decimal(3,0)
) d
end
go
create or alter procedure InsertAsofferOfferrpowerdata1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Asoffer'
    and sub_type = 'Offerrpowerdata'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Asoffer'
    and sub_type = 'Offerrpowerdata'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Asoffer', 'Offerrpowerdata', 1);

insert into AsofferOfferrpowerdata1(
file_log_id,
[contractid],
[effectivedate],
[versionno],
[periodid],
[availability],
[mta],
[mtg],
[authoriseddate],
[authorisedby],
[filename],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractid],
d.[effectivedate],
d.[versionno],
d.[periodid],
d.[availability],
d.[mta],
d.[mtg],
d.[authoriseddate],
d.[authorisedby],
d.[filename],
d.[lastchanged]
from openjson(@data) with (
[contractid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[periodid] decimal(3,0),
[availability] decimal(3,0),
[mta] decimal(6,0),
[mtg] decimal(6,0),
[authoriseddate] datetime2,
[authorisedby] varchar(15),
[filename] varchar(40),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertAsofferOfferrestartdata1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Asoffer'
    and sub_type = 'Offerrestartdata'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Asoffer'
    and sub_type = 'Offerrestartdata'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Asoffer', 'Offerrestartdata', 1);

insert into AsofferOfferrestartdata1(
file_log_id,
[contractid],
[offerdate],
[versionno],
[availability],
[authoriseddate],
[authorisedby],
[filename],
[lastchanged],
[periodid]
)
select 
(select h.id from @header h),
d.[contractid],
d.[offerdate],
d.[versionno],
d.[availability],
d.[authoriseddate],
d.[authorisedby],
d.[filename],
d.[lastchanged],
d.[periodid]
from openjson(@data) with (
[contractid] varchar(10),
[offerdate] datetime2,
[versionno] decimal(3,0),
[availability] varchar(3),
[authoriseddate] datetime2,
[authorisedby] varchar(15),
[filename] varchar(40),
[lastchanged] datetime2,
[periodid] decimal(3,0)
) d
end
go
create or alter procedure InsertAsofferOfferlsheddata1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Asoffer'
    and sub_type = 'Offerlsheddata'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Asoffer'
    and sub_type = 'Offerlsheddata'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Asoffer', 'Offerlsheddata', 1);

insert into AsofferOfferlsheddata1(
file_log_id,
[contractid],
[effectivedate],
[versionno],
[availableload],
[authoriseddate],
[authorisedby],
[filename],
[lastchanged],
[periodid]
)
select 
(select h.id from @header h),
d.[contractid],
d.[effectivedate],
d.[versionno],
d.[availableload],
d.[authoriseddate],
d.[authorisedby],
d.[filename],
d.[lastchanged],
d.[periodid]
from openjson(@data) with (
[contractid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[availableload] decimal(4,0),
[authoriseddate] datetime2,
[authorisedby] varchar(15),
[filename] varchar(40),
[lastchanged] datetime2,
[periodid] decimal(3,0)
) d
end
go
create or alter procedure InsertAsofferOfferastrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Asoffer'
    and sub_type = 'Offerastrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Asoffer'
    and sub_type = 'Offerastrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Asoffer', 'Offerastrk', 1);

insert into AsofferOfferastrk1(
file_log_id,
[effectivedate],
[versionno],
[participantid],
[filename],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[versionno],
d.[participantid],
d.[filename],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[filename] varchar(40),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertGdInstructGdinstruct1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'GdInstruct'
    and sub_type = 'Gdinstruct'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'GdInstruct'
    and sub_type = 'Gdinstruct'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'GdInstruct', 'Gdinstruct', 1);

insert into GdInstructGdinstruct1(
file_log_id,
[duid],
[stationid],
[regionid],
[id],
[instructiontypeid],
[instructionsubtypeid],
[instructionclassid],
[reason],
[instlevel],
[authoriseddate],
[authorisedby],
[participantid],
[issuedtime],
[targettime],
[lastchanged]
)
select 
(select h.id from @header h),
d.[duid],
d.[stationid],
d.[regionid],
d.[id],
d.[instructiontypeid],
d.[instructionsubtypeid],
d.[instructionclassid],
d.[reason],
d.[instlevel],
d.[authoriseddate],
d.[authorisedby],
d.[participantid],
d.[issuedtime],
d.[targettime],
d.[lastchanged]
from openjson(@data) with (
[duid] varchar(10),
[stationid] varchar(10),
[regionid] varchar(10),
[id] decimal(22,0),
[instructiontypeid] varchar(10),
[instructionsubtypeid] varchar(10),
[instructionclassid] varchar(10),
[reason] varchar(64),
[instlevel] decimal(6,0),
[authoriseddate] datetime2,
[authorisedby] varchar(15),
[participantid] varchar(10),
[issuedtime] datetime2,
[targettime] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertGdInstructInstructionsubtype1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'GdInstruct'
    and sub_type = 'Instructionsubtype'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'GdInstruct'
    and sub_type = 'Instructionsubtype'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'GdInstruct', 'Instructionsubtype', 1);

insert into GdInstructInstructionsubtype1(
file_log_id,
[instructiontypeid],
[instructionsubtypeid],
[description],
[lastchanged]
)
select 
(select h.id from @header h),
d.[instructiontypeid],
d.[instructionsubtypeid],
d.[description],
d.[lastchanged]
from openjson(@data) with (
[instructiontypeid] varchar(10),
[instructionsubtypeid] varchar(10),
[description] varchar(64),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertGdInstructInstructiontype1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'GdInstruct'
    and sub_type = 'Instructiontype'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'GdInstruct'
    and sub_type = 'Instructiontype'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'GdInstruct', 'Instructiontype', 1);

insert into GdInstructInstructiontype1(
file_log_id,
[instructiontypeid],
[description],
[regionid],
[lastchanged]
)
select 
(select h.id from @header h),
d.[instructiontypeid],
d.[description],
d.[regionid],
d.[lastchanged]
from openjson(@data) with (
[instructiontypeid] varchar(10),
[description] varchar(64),
[regionid] varchar(10),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertGeqrhsNull1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Geqrhs'
    and sub_type = 'Null'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Geqrhs'
    and sub_type = 'Null'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Geqrhs', 'Null', 1);

insert into GeqrhsNull1(
file_log_id,
[equationid],
[effectivedate],
[versionno],
[termid],
[groupid],
[spd_id],
[spd_type],
[factor],
[operation],
[defaultvalue],
[parameterterm1],
[parameterterm2],
[parameterterm3],
[lastchanged]
)
select 
(select h.id from @header h),
d.[equationid],
d.[effectivedate],
d.[versionno],
d.[termid],
d.[groupid],
d.[spd_id],
d.[spd_type],
d.[factor],
d.[operation],
d.[defaultvalue],
d.[parameterterm1],
d.[parameterterm2],
d.[parameterterm3],
d.[lastchanged]
from openjson(@data) with (
[equationid] varchar(20),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[termid] decimal(3,0),
[groupid] decimal(3,0),
[spd_id] varchar(21),
[spd_type] varchar(1),
[factor] decimal(16,6),
[operation] varchar(10),
[defaultvalue] decimal(16,6),
[parameterterm1] varchar(12),
[parameterterm2] varchar(12),
[parameterterm3] varchar(12),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertGenconsetNull1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Genconset'
    and sub_type = 'Null'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Genconset'
    and sub_type = 'Null'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Genconset', 'Null', 1);

insert into GenconsetNull1(
file_log_id,
[genconsetid],
[effectivedate],
[versionno],
[genconid],
[genconeffdate],
[genconversionno],
[lastchanged]
)
select 
(select h.id from @header h),
d.[genconsetid],
d.[effectivedate],
d.[versionno],
d.[genconid],
d.[genconeffdate],
d.[genconversionno],
d.[lastchanged]
from openjson(@data) with (
[genconsetid] varchar(20),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[genconid] varchar(20),
[genconeffdate] datetime2,
[genconversionno] decimal(3,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertGenericConstraintGenconsetinvoke2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'GenericConstraint'
    and sub_type = 'Genconsetinvoke'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'GenericConstraint'
    and sub_type = 'Genconsetinvoke'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'GenericConstraint', 'Genconsetinvoke', 2);

insert into GenericConstraintGenconsetinvoke2(
file_log_id,
[invocation_id],
[startdate],
[startperiod],
[genconsetid],
[enddate],
[endperiod],
[startauthorisedby],
[endauthorisedby],
[intervention],
[asconstrainttype],
[lastchanged],
[startintervaldatetime],
[endintervaldatetime],
[systemnormal]
)
select 
(select h.id from @header h),
d.[invocation_id],
d.[startdate],
d.[startperiod],
d.[genconsetid],
d.[enddate],
d.[endperiod],
d.[startauthorisedby],
d.[endauthorisedby],
d.[intervention],
d.[asconstrainttype],
d.[lastchanged],
d.[startintervaldatetime],
d.[endintervaldatetime],
d.[systemnormal]
from openjson(@data) with (
[invocation_id] decimal(9,0),
[startdate] datetime2,
[startperiod] decimal(3,0),
[genconsetid] varchar(20),
[enddate] datetime2,
[endperiod] decimal(3,0),
[startauthorisedby] varchar(15),
[endauthorisedby] varchar(15),
[intervention] varchar(1),
[asconstrainttype] varchar(10),
[lastchanged] datetime2,
[startintervaldatetime] datetime2,
[endintervaldatetime] datetime2,
[systemnormal] varchar(1)
) d
end
go
create or alter procedure InsertSpdrcNull2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Spdrc'
    and sub_type = 'Null'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Spdrc'
    and sub_type = 'Null'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Spdrc', 'Null', 2);

insert into SpdrcNull2(
file_log_id,
[regionid],
[effectivedate],
[versionno],
[genconid],
[factor],
[lastchanged],
[bidtype]
)
select 
(select h.id from @header h),
d.[regionid],
d.[effectivedate],
d.[versionno],
d.[genconid],
d.[factor],
d.[lastchanged],
d.[bidtype]
from openjson(@data) with (
[regionid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[genconid] varchar(20),
[factor] decimal(16,6),
[lastchanged] datetime2,
[bidtype] varchar(10)
) d
end
go
create or alter procedure InsertGencondataNull6
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Gencondata'
    and sub_type = 'Null'
    and version = '6'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Gencondata'
    and sub_type = 'Null'
    and version = '6'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Gencondata', 'Null', 6);

insert into GencondataNull6(
file_log_id,
[effectivedate],
[versionno],
[genconid],
[constrainttype],
[constraintvalue],
[description],
[status],
[genericconstraintweight],
[authoriseddate],
[authorisedby],
[dynamicrhs],
[lastchanged],
[dispatch],
[predispatch],
[stpasa],
[mtpasa],
[impact],
[source],
[limittype],
[reason],
[modifications],
[additionalnotes],
[p5min_scope_override],
[lrc],
[lor],
[force_scada]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[versionno],
d.[genconid],
d.[constrainttype],
d.[constraintvalue],
d.[description],
d.[status],
d.[genericconstraintweight],
d.[authoriseddate],
d.[authorisedby],
d.[dynamicrhs],
d.[lastchanged],
d.[dispatch],
d.[predispatch],
d.[stpasa],
d.[mtpasa],
d.[impact],
d.[source],
d.[limittype],
d.[reason],
d.[modifications],
d.[additionalnotes],
d.[p5min_scope_override],
d.[lrc],
d.[lor],
d.[force_scada]
from openjson(@data) with (
[effectivedate] datetime2,
[versionno] decimal(3,0),
[genconid] varchar(20),
[constrainttype] varchar(2),
[constraintvalue] decimal(16,6),
[description] varchar(256),
[status] varchar(8),
[genericconstraintweight] decimal(16,6),
[authoriseddate] datetime2,
[authorisedby] varchar(15),
[dynamicrhs] decimal(15,5),
[lastchanged] datetime2,
[dispatch] varchar(1),
[predispatch] varchar(1),
[stpasa] varchar(1),
[mtpasa] varchar(1),
[impact] varchar(64),
[source] varchar(128),
[limittype] varchar(64),
[reason] varchar(256),
[modifications] varchar(256),
[additionalnotes] varchar(256),
[p5min_scope_override] varchar(2),
[lrc] varchar(1),
[lor] varchar(1),
[force_scada] decimal(1,0)
) d
end
go
create or alter procedure InsertSpdcpcNull2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Spdcpc'
    and sub_type = 'Null'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Spdcpc'
    and sub_type = 'Null'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Spdcpc', 'Null', 2);

insert into SpdcpcNull2(
file_log_id,
[connectionpointid],
[effectivedate],
[versionno],
[genconid],
[factor],
[lastchanged],
[bidtype]
)
select 
(select h.id from @header h),
d.[connectionpointid],
d.[effectivedate],
d.[versionno],
d.[genconid],
d.[factor],
d.[lastchanged],
d.[bidtype]
from openjson(@data) with (
[connectionpointid] varchar(12),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[genconid] varchar(20),
[factor] decimal(16,6),
[lastchanged] datetime2,
[bidtype] varchar(12)
) d
end
go
create or alter procedure InsertGcrhsNull1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Gcrhs'
    and sub_type = 'Null'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Gcrhs'
    and sub_type = 'Null'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Gcrhs', 'Null', 1);

insert into GcrhsNull1(
file_log_id,
[genconid],
[effectivedate],
[versionno],
[scope],
[termid],
[groupid],
[spd_id],
[spd_type],
[factor],
[operation],
[defaultvalue],
[parameterterm1],
[parameterterm2],
[parameterterm3],
[lastchanged]
)
select 
(select h.id from @header h),
d.[genconid],
d.[effectivedate],
d.[versionno],
d.[scope],
d.[termid],
d.[groupid],
d.[spd_id],
d.[spd_type],
d.[factor],
d.[operation],
d.[defaultvalue],
d.[parameterterm1],
d.[parameterterm2],
d.[parameterterm3],
d.[lastchanged]
from openjson(@data) with (
[genconid] varchar(20),
[effectivedate] datetime2,
[versionno] decimal(22,0),
[scope] varchar(2),
[termid] decimal(4,0),
[groupid] decimal(3,0),
[spd_id] varchar(21),
[spd_type] varchar(1),
[factor] decimal(16,6),
[operation] varchar(10),
[defaultvalue] decimal(16,6),
[parameterterm1] varchar(12),
[parameterterm2] varchar(12),
[parameterterm3] varchar(12),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertGenericConstraintEmsmaster1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'GenericConstraint'
    and sub_type = 'Emsmaster'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'GenericConstraint'
    and sub_type = 'Emsmaster'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'GenericConstraint', 'Emsmaster', 1);

insert into GenericConstraintEmsmaster1(
file_log_id,
[spd_id],
[spd_type],
[description],
[grouping_id],
[lastchanged]
)
select 
(select h.id from @header h),
d.[spd_id],
d.[spd_type],
d.[description],
d.[grouping_id],
d.[lastchanged]
from openjson(@data) with (
[spd_id] varchar(21),
[spd_type] varchar(1),
[description] varchar(255),
[grouping_id] varchar(20),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertGenconsettrkNull2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Genconsettrk'
    and sub_type = 'Null'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Genconsettrk'
    and sub_type = 'Null'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Genconsettrk', 'Null', 2);

insert into GenconsettrkNull2(
file_log_id,
[genconsetid],
[effectivedate],
[versionno],
[description],
[authorisedby],
[authoriseddate],
[lastchanged],
[coverage],
[modifications],
[systemnormal],
[outage]
)
select 
(select h.id from @header h),
d.[genconsetid],
d.[effectivedate],
d.[versionno],
d.[description],
d.[authorisedby],
d.[authoriseddate],
d.[lastchanged],
d.[coverage],
d.[modifications],
d.[systemnormal],
d.[outage]
from openjson(@data) with (
[genconsetid] varchar(20),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[description] varchar(256),
[authorisedby] varchar(15),
[authoriseddate] datetime2,
[lastchanged] datetime2,
[coverage] varchar(64),
[modifications] varchar(256),
[systemnormal] varchar(1),
[outage] varchar(256)
) d
end
go
create or alter procedure InsertSpdiccNull1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Spdicc'
    and sub_type = 'Null'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Spdicc'
    and sub_type = 'Null'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Spdicc', 'Null', 1);

insert into SpdiccNull1(
file_log_id,
[interconnectorid],
[effectivedate],
[versionno],
[genconid],
[factor],
[lastchanged]
)
select 
(select h.id from @header h),
d.[interconnectorid],
d.[effectivedate],
d.[versionno],
d.[genconid],
d.[factor],
d.[lastchanged]
from openjson(@data) with (
[interconnectorid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[genconid] varchar(20),
[factor] decimal(16,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertGeqdescNull2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Geqdesc'
    and sub_type = 'Null'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Geqdesc'
    and sub_type = 'Null'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Geqdesc', 'Null', 2);

insert into GeqdescNull2(
file_log_id,
[equationid],
[description],
[lastchanged],
[impact],
[source],
[limittype],
[reason],
[modifications],
[additionalnotes]
)
select 
(select h.id from @header h),
d.[equationid],
d.[description],
d.[lastchanged],
d.[impact],
d.[source],
d.[limittype],
d.[reason],
d.[modifications],
d.[additionalnotes]
from openjson(@data) with (
[equationid] varchar(20),
[description] varchar(256),
[lastchanged] datetime2,
[impact] varchar(64),
[source] varchar(128),
[limittype] varchar(64),
[reason] varchar(256),
[modifications] varchar(256),
[additionalnotes] varchar(256)
) d
end
go
create or alter procedure InsertMtpasaReservelimitSet1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mtpasa'
    and sub_type = 'ReservelimitSet'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mtpasa'
    and sub_type = 'ReservelimitSet'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Mtpasa', 'ReservelimitSet', 1);

insert into MtpasaReservelimitSet1(
file_log_id,
[effectivedate],
[version_datetime],
[reservelimit_set_id],
[description],
[authoriseddate],
[authorisedby],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[version_datetime],
d.[reservelimit_set_id],
d.[description],
d.[authoriseddate],
d.[authorisedby],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[version_datetime] datetime2,
[reservelimit_set_id] varchar(20),
[description] varchar(200),
[authoriseddate] datetime2,
[authorisedby] varchar(20),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertReserveDataReserve1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ReserveData'
    and sub_type = 'Reserve'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ReserveData'
    and sub_type = 'Reserve'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ReserveData', 'Reserve', 1);

insert into ReserveDataReserve1(
file_log_id,
[settlementdate],
[versionno],
[regionid],
[periodid],
[lower5min],
[lower60sec],
[lower6sec],
[raise5min],
[raise60sec],
[raise6sec],
[lastchanged],
[pasareserve],
[loadrejectionreservereq],
[raisereg],
[lowerreg],
[lor1level],
[lor2level]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[regionid],
d.[periodid],
d.[lower5min],
d.[lower60sec],
d.[lower6sec],
d.[raise5min],
d.[raise60sec],
d.[raise6sec],
d.[lastchanged],
d.[pasareserve],
d.[loadrejectionreservereq],
d.[raisereg],
d.[lowerreg],
d.[lor1level],
d.[lor2level]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[regionid] varchar(12),
[periodid] decimal(2,0),
[lower5min] decimal(6,0),
[lower60sec] decimal(6,0),
[lower6sec] decimal(6,0),
[raise5min] decimal(6,0),
[raise60sec] decimal(6,0),
[raise6sec] decimal(6,0),
[lastchanged] datetime2,
[pasareserve] decimal(6,0),
[loadrejectionreservereq] decimal(10,0),
[raisereg] decimal(6,0),
[lowerreg] decimal(6,0),
[lor1level] decimal(6,0),
[lor2level] decimal(6,0)
) d
end
go
create or alter procedure InsertMtpasaReservelimit1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mtpasa'
    and sub_type = 'Reservelimit'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mtpasa'
    and sub_type = 'Reservelimit'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Mtpasa', 'Reservelimit', 1);

insert into MtpasaReservelimit1(
file_log_id,
[effectivedate],
[version_datetime],
[reservelimitid],
[description],
[rhs],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[version_datetime],
d.[reservelimitid],
d.[description],
d.[rhs],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[version_datetime] datetime2,
[reservelimitid] varchar(20),
[description] varchar(200),
[rhs] decimal(16,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMtpasaReservelimitRegion1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mtpasa'
    and sub_type = 'ReservelimitRegion'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mtpasa'
    and sub_type = 'ReservelimitRegion'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Mtpasa', 'ReservelimitRegion', 1);

insert into MtpasaReservelimitRegion1(
file_log_id,
[effectivedate],
[version_datetime],
[reservelimitid],
[regionid],
[coef],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[version_datetime],
d.[reservelimitid],
d.[regionid],
d.[coef],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[version_datetime] datetime2,
[reservelimitid] varchar(20),
[regionid] varchar(20),
[coef] decimal(16,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMrEventSchedule1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mr'
    and sub_type = 'EventSchedule'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mr'
    and sub_type = 'EventSchedule'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Mr', 'EventSchedule', 1);

insert into MrEventSchedule1(
file_log_id,
[mr_date],
[regionid],
[version_datetime],
[demand_effectivedate],
[demand_offerdate],
[demand_versionno],
[authorisedby],
[authoriseddate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[mr_date],
d.[regionid],
d.[version_datetime],
d.[demand_effectivedate],
d.[demand_offerdate],
d.[demand_versionno],
d.[authorisedby],
d.[authoriseddate],
d.[lastchanged]
from openjson(@data) with (
[mr_date] datetime2,
[regionid] varchar(10),
[version_datetime] datetime2,
[demand_effectivedate] datetime2,
[demand_offerdate] datetime2,
[demand_versionno] decimal(3,0),
[authorisedby] varchar(20),
[authoriseddate] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMrDayofferStack1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mr'
    and sub_type = 'DayofferStack'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mr'
    and sub_type = 'DayofferStack'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Mr', 'DayofferStack', 1);

insert into MrDayofferStack1(
file_log_id,
[mr_date],
[regionid],
[version_datetime],
[stack_position],
[duid],
[authorised],
[offer_settlementdate],
[offer_offerdate],
[offer_versionno],
[offer_type],
[laof],
[lastchanged]
)
select 
(select h.id from @header h),
d.[mr_date],
d.[regionid],
d.[version_datetime],
d.[stack_position],
d.[duid],
d.[authorised],
d.[offer_settlementdate],
d.[offer_offerdate],
d.[offer_versionno],
d.[offer_type],
d.[laof],
d.[lastchanged]
from openjson(@data) with (
[mr_date] datetime2,
[regionid] varchar(10),
[version_datetime] datetime2,
[stack_position] decimal(3,0),
[duid] varchar(10),
[authorised] decimal(1,0),
[offer_settlementdate] datetime2,
[offer_offerdate] datetime2,
[offer_versionno] decimal(3,0),
[offer_type] varchar(20),
[laof] decimal(16,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMrPerofferStack1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mr'
    and sub_type = 'PerofferStack'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mr'
    and sub_type = 'PerofferStack'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Mr', 'PerofferStack', 1);

insert into MrPerofferStack1(
file_log_id,
[mr_date],
[regionid],
[version_datetime],
[stack_position],
[periodid],
[duid],
[accepted_capacity],
[deducted_capacity],
[lastchanged]
)
select 
(select h.id from @header h),
d.[mr_date],
d.[regionid],
d.[version_datetime],
d.[stack_position],
d.[periodid],
d.[duid],
d.[accepted_capacity],
d.[deducted_capacity],
d.[lastchanged]
from openjson(@data) with (
[mr_date] datetime2,
[regionid] varchar(10),
[version_datetime] datetime2,
[stack_position] decimal(3,0),
[periodid] decimal(3,0),
[duid] varchar(10),
[accepted_capacity] decimal(6,0),
[deducted_capacity] decimal(6,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMrEvent1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mr'
    and sub_type = 'Event'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mr'
    and sub_type = 'Event'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Mr', 'Event', 1);

insert into MrEvent1(
file_log_id,
[mr_date],
[regionid],
[description],
[authoriseddate],
[authorisedby],
[offer_cut_off_time],
[settlement_complete],
[lastchanged]
)
select 
(select h.id from @header h),
d.[mr_date],
d.[regionid],
d.[description],
d.[authoriseddate],
d.[authorisedby],
d.[offer_cut_off_time],
d.[settlement_complete],
d.[lastchanged]
from openjson(@data) with (
[mr_date] datetime2,
[regionid] varchar(10),
[description] varchar(200),
[authoriseddate] datetime2,
[authorisedby] varchar(20),
[offer_cut_off_time] datetime2,
[settlement_complete] decimal(1,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMarketNoticeParticipantnoticetrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketNotice'
    and sub_type = 'Participantnoticetrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketNotice'
    and sub_type = 'Participantnoticetrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'MarketNotice', 'Participantnoticetrk', 1);

insert into MarketNoticeParticipantnoticetrk1(
file_log_id,
[participantid],
[noticeid],
[lastchanged]
)
select 
(select h.id from @header h),
d.[participantid],
d.[noticeid],
d.[lastchanged]
from openjson(@data) with (
[participantid] varchar(10),
[noticeid] decimal(10,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMarketNoticeMarketnoticetype1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketNotice'
    and sub_type = 'Marketnoticetype'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketNotice'
    and sub_type = 'Marketnoticetype'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'MarketNotice', 'Marketnoticetype', 1);

insert into MarketNoticeMarketnoticetype1(
file_log_id,
[typeid],
[description],
[raisedby],
[lastchanged]
)
select 
(select h.id from @header h),
d.[typeid],
d.[description],
d.[raisedby],
d.[lastchanged]
from openjson(@data) with (
[typeid] varchar(25),
[description] varchar(64),
[raisedby] varchar(10),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMarketNoticeMarketnoticedata1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketNotice'
    and sub_type = 'Marketnoticedata'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketNotice'
    and sub_type = 'Marketnoticedata'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'MarketNotice', 'Marketnoticedata', 1);

insert into MarketNoticeMarketnoticedata1(
file_log_id,
[noticeid],
[effectivedate],
[typeid],
[noticetype],
[lastchanged],
[reason],
[externalreference]
)
select 
(select h.id from @header h),
d.[noticeid],
d.[effectivedate],
d.[typeid],
d.[noticetype],
d.[lastchanged],
d.[reason],
d.[externalreference]
from openjson(@data) with (
[noticeid] decimal(10,0),
[effectivedate] datetime2,
[typeid] varchar(25),
[noticetype] varchar(25),
[lastchanged] datetime2,
[reason] varchar(2000),
[externalreference] varchar(255)
) d
end
go
create or alter procedure InsertP5minBlockedConstraints1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'P5min'
    and sub_type = 'BlockedConstraints'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'P5min'
    and sub_type = 'BlockedConstraints'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'P5min', 'BlockedConstraints', 1);

insert into P5minBlockedConstraints1(
file_log_id,
[run_datetime],
[constraintid]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[constraintid]
from openjson(@data) with (
[run_datetime] datetime2,
[constraintid] varchar(20)
) d
end
go
create or alter procedure InsertP5minCasesolution2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'P5min'
    and sub_type = 'Casesolution'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'P5min'
    and sub_type = 'Casesolution'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'P5min', 'Casesolution', 2);

insert into P5minCasesolution2(
file_log_id,
[run_datetime],
[startinterval_datetime],
[totalobjective],
[nonphysicallosses],
[totalareagenviolation],
[totalinterconnectorviolation],
[totalgenericviolation],
[totalramprateviolation],
[totalunitmwcapacityviolation],
[total5minviolation],
[totalregviolation],
[total6secviolation],
[total60secviolation],
[totalenergyconstrviolation],
[totalenergyofferviolation],
[totalasprofileviolation],
[totalfaststartviolation],
[lastchanged],
[intervention]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[startinterval_datetime],
d.[totalobjective],
d.[nonphysicallosses],
d.[totalareagenviolation],
d.[totalinterconnectorviolation],
d.[totalgenericviolation],
d.[totalramprateviolation],
d.[totalunitmwcapacityviolation],
d.[total5minviolation],
d.[totalregviolation],
d.[total6secviolation],
d.[total60secviolation],
d.[totalenergyconstrviolation],
d.[totalenergyofferviolation],
d.[totalasprofileviolation],
d.[totalfaststartviolation],
d.[lastchanged],
d.[intervention]
from openjson(@data) with (
[run_datetime] datetime2,
[startinterval_datetime] varchar(20),
[totalobjective] decimal(27,10),
[nonphysicallosses] decimal(1,0),
[totalareagenviolation] decimal(15,5),
[totalinterconnectorviolation] decimal(15,5),
[totalgenericviolation] decimal(15,5),
[totalramprateviolation] decimal(15,5),
[totalunitmwcapacityviolation] decimal(15,5),
[total5minviolation] decimal(15,5),
[totalregviolation] decimal(15,5),
[total6secviolation] decimal(15,5),
[total60secviolation] decimal(15,5),
[totalenergyconstrviolation] decimal(15,5),
[totalenergyofferviolation] decimal(15,5),
[totalasprofileviolation] decimal(15,5),
[totalfaststartviolation] decimal(15,5),
[lastchanged] datetime2,
[intervention] decimal(2,0)
) d
end
go
create or alter procedure InsertP5minLocalPrice1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'P5min'
    and sub_type = 'LocalPrice'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'P5min'
    and sub_type = 'LocalPrice'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'P5min', 'LocalPrice', 1);

insert into P5minLocalPrice1(
file_log_id,
[run_datetime],
[interval_datetime],
[duid],
[local_price_adjustment],
[locally_constrained]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[interval_datetime],
d.[duid],
d.[local_price_adjustment],
d.[locally_constrained]
from openjson(@data) with (
[run_datetime] datetime2,
[interval_datetime] datetime2,
[duid] varchar(20),
[local_price_adjustment] decimal(10,2),
[locally_constrained] decimal(1,0)
) d
end
go
create or alter procedure InsertP5minRegionsolution5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'P5min'
    and sub_type = 'Regionsolution'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'P5min'
    and sub_type = 'Regionsolution'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'P5min', 'Regionsolution', 5);

insert into P5minRegionsolution5(
file_log_id,
[run_datetime],
[interval_datetime],
[regionid],
[rrp],
[rop],
[excessgeneration],
[raise6secrrp],
[raise6secrop],
[raise60secrrp],
[raise60secrop],
[raise5minrrp],
[raise5minrop],
[raiseregrrp],
[raiseregrop],
[lower6secrrp],
[lower6secrop],
[lower60secrrp],
[lower60secrop],
[lower5minrrp],
[lower5minrop],
[lowerregrrp],
[lowerregrop],
[totaldemand],
[availablegeneration],
[availableload],
[demandforecast],
[dispatchablegeneration],
[dispatchableload],
[netinterchange],
[lower5mindispatch],
[lower5minimport],
[lower5minlocaldispatch],
[lower5minlocalreq],
[lower5minreq],
[lower60secdispatch],
[lower60secimport],
[lower60seclocaldispatch],
[lower60seclocalreq],
[lower60secreq],
[lower6secdispatch],
[lower6secimport],
[lower6seclocaldispatch],
[lower6seclocalreq],
[lower6secreq],
[raise5mindispatch],
[raise5minimport],
[raise5minlocaldispatch],
[raise5minlocalreq],
[raise5minreq],
[raise60secdispatch],
[raise60secimport],
[raise60seclocaldispatch],
[raise60seclocalreq],
[raise60secreq],
[raise6secdispatch],
[raise6secimport],
[raise6seclocaldispatch],
[raise6seclocalreq],
[raise6secreq],
[aggregatedispatcherror],
[initialsupply],
[clearedsupply],
[lowerregimport],
[lowerregdispatch],
[lowerreglocaldispatch],
[lowerreglocalreq],
[lowerregreq],
[raiseregimport],
[raiseregdispatch],
[raisereglocaldispatch],
[raisereglocalreq],
[raiseregreq],
[raise5minlocalviolation],
[raisereglocalviolation],
[raise60seclocalviolation],
[raise6seclocalviolation],
[lower5minlocalviolation],
[lowerreglocalviolation],
[lower60seclocalviolation],
[lower6seclocalviolation],
[raise5minviolation],
[raiseregviolation],
[raise60secviolation],
[raise6secviolation],
[lower5minviolation],
[lowerregviolation],
[lower60secviolation],
[lower6secviolation],
[lastchanged],
[totalintermittentgeneration],
[demand_and_nonschedgen],
[uigf],
[semischedule_clearedmw],
[semischedule_compliancemw],
[intervention],
[ss_solar_uigf],
[ss_wind_uigf],
[ss_solar_clearedmw],
[ss_wind_clearedmw],
[ss_solar_compliancemw],
[ss_wind_compliancemw]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[interval_datetime],
d.[regionid],
d.[rrp],
d.[rop],
d.[excessgeneration],
d.[raise6secrrp],
d.[raise6secrop],
d.[raise60secrrp],
d.[raise60secrop],
d.[raise5minrrp],
d.[raise5minrop],
d.[raiseregrrp],
d.[raiseregrop],
d.[lower6secrrp],
d.[lower6secrop],
d.[lower60secrrp],
d.[lower60secrop],
d.[lower5minrrp],
d.[lower5minrop],
d.[lowerregrrp],
d.[lowerregrop],
d.[totaldemand],
d.[availablegeneration],
d.[availableload],
d.[demandforecast],
d.[dispatchablegeneration],
d.[dispatchableload],
d.[netinterchange],
d.[lower5mindispatch],
d.[lower5minimport],
d.[lower5minlocaldispatch],
d.[lower5minlocalreq],
d.[lower5minreq],
d.[lower60secdispatch],
d.[lower60secimport],
d.[lower60seclocaldispatch],
d.[lower60seclocalreq],
d.[lower60secreq],
d.[lower6secdispatch],
d.[lower6secimport],
d.[lower6seclocaldispatch],
d.[lower6seclocalreq],
d.[lower6secreq],
d.[raise5mindispatch],
d.[raise5minimport],
d.[raise5minlocaldispatch],
d.[raise5minlocalreq],
d.[raise5minreq],
d.[raise60secdispatch],
d.[raise60secimport],
d.[raise60seclocaldispatch],
d.[raise60seclocalreq],
d.[raise60secreq],
d.[raise6secdispatch],
d.[raise6secimport],
d.[raise6seclocaldispatch],
d.[raise6seclocalreq],
d.[raise6secreq],
d.[aggregatedispatcherror],
d.[initialsupply],
d.[clearedsupply],
d.[lowerregimport],
d.[lowerregdispatch],
d.[lowerreglocaldispatch],
d.[lowerreglocalreq],
d.[lowerregreq],
d.[raiseregimport],
d.[raiseregdispatch],
d.[raisereglocaldispatch],
d.[raisereglocalreq],
d.[raiseregreq],
d.[raise5minlocalviolation],
d.[raisereglocalviolation],
d.[raise60seclocalviolation],
d.[raise6seclocalviolation],
d.[lower5minlocalviolation],
d.[lowerreglocalviolation],
d.[lower60seclocalviolation],
d.[lower6seclocalviolation],
d.[raise5minviolation],
d.[raiseregviolation],
d.[raise60secviolation],
d.[raise6secviolation],
d.[lower5minviolation],
d.[lowerregviolation],
d.[lower60secviolation],
d.[lower6secviolation],
d.[lastchanged],
d.[totalintermittentgeneration],
d.[demand_and_nonschedgen],
d.[uigf],
d.[semischedule_clearedmw],
d.[semischedule_compliancemw],
d.[intervention],
d.[ss_solar_uigf],
d.[ss_wind_uigf],
d.[ss_solar_clearedmw],
d.[ss_wind_clearedmw],
d.[ss_solar_compliancemw],
d.[ss_wind_compliancemw]
from openjson(@data) with (
[run_datetime] datetime2,
[interval_datetime] datetime2,
[regionid] varchar(10),
[rrp] decimal(15,5),
[rop] decimal(15,5),
[excessgeneration] decimal(15,5),
[raise6secrrp] decimal(15,5),
[raise6secrop] decimal(15,5),
[raise60secrrp] decimal(15,5),
[raise60secrop] decimal(15,5),
[raise5minrrp] decimal(15,5),
[raise5minrop] decimal(15,5),
[raiseregrrp] decimal(15,5),
[raiseregrop] decimal(15,5),
[lower6secrrp] decimal(15,5),
[lower6secrop] decimal(15,5),
[lower60secrrp] decimal(15,5),
[lower60secrop] decimal(15,5),
[lower5minrrp] decimal(15,5),
[lower5minrop] decimal(15,5),
[lowerregrrp] decimal(15,5),
[lowerregrop] decimal(15,5),
[totaldemand] decimal(15,5),
[availablegeneration] decimal(15,5),
[availableload] decimal(15,5),
[demandforecast] decimal(15,5),
[dispatchablegeneration] decimal(15,5),
[dispatchableload] decimal(15,5),
[netinterchange] decimal(15,5),
[lower5mindispatch] decimal(15,5),
[lower5minimport] decimal(15,5),
[lower5minlocaldispatch] decimal(15,5),
[lower5minlocalreq] decimal(15,5),
[lower5minreq] decimal(15,5),
[lower60secdispatch] decimal(15,5),
[lower60secimport] decimal(15,5),
[lower60seclocaldispatch] decimal(15,5),
[lower60seclocalreq] decimal(15,5),
[lower60secreq] decimal(15,5),
[lower6secdispatch] decimal(15,5),
[lower6secimport] decimal(15,5),
[lower6seclocaldispatch] decimal(15,5),
[lower6seclocalreq] decimal(15,5),
[lower6secreq] decimal(15,5),
[raise5mindispatch] decimal(15,5),
[raise5minimport] decimal(15,5),
[raise5minlocaldispatch] decimal(15,5),
[raise5minlocalreq] decimal(15,5),
[raise5minreq] decimal(15,5),
[raise60secdispatch] decimal(15,5),
[raise60secimport] decimal(15,5),
[raise60seclocaldispatch] decimal(15,5),
[raise60seclocalreq] decimal(15,5),
[raise60secreq] decimal(15,5),
[raise6secdispatch] decimal(15,5),
[raise6secimport] decimal(15,5),
[raise6seclocaldispatch] decimal(15,5),
[raise6seclocalreq] decimal(15,5),
[raise6secreq] decimal(15,5),
[aggregatedispatcherror] decimal(15,5),
[initialsupply] decimal(15,5),
[clearedsupply] decimal(15,5),
[lowerregimport] decimal(15,5),
[lowerregdispatch] decimal(15,5),
[lowerreglocaldispatch] decimal(15,5),
[lowerreglocalreq] decimal(15,5),
[lowerregreq] decimal(15,5),
[raiseregimport] decimal(15,5),
[raiseregdispatch] decimal(15,5),
[raisereglocaldispatch] decimal(15,5),
[raisereglocalreq] decimal(15,5),
[raiseregreq] decimal(15,5),
[raise5minlocalviolation] decimal(15,5),
[raisereglocalviolation] decimal(15,5),
[raise60seclocalviolation] decimal(15,5),
[raise6seclocalviolation] decimal(15,5),
[lower5minlocalviolation] decimal(15,5),
[lowerreglocalviolation] decimal(15,5),
[lower60seclocalviolation] decimal(15,5),
[lower6seclocalviolation] decimal(15,5),
[raise5minviolation] decimal(15,5),
[raiseregviolation] decimal(15,5),
[raise60secviolation] decimal(15,5),
[raise6secviolation] decimal(15,5),
[lower5minviolation] decimal(15,5),
[lowerregviolation] decimal(15,5),
[lower60secviolation] decimal(15,5),
[lower6secviolation] decimal(15,5),
[lastchanged] datetime2,
[totalintermittentgeneration] decimal(15,5),
[demand_and_nonschedgen] decimal(15,5),
[uigf] decimal(15,5),
[semischedule_clearedmw] decimal(15,5),
[semischedule_compliancemw] decimal(15,5),
[intervention] decimal(2,0),
[ss_solar_uigf] decimal(15,5),
[ss_wind_uigf] decimal(15,5),
[ss_solar_clearedmw] decimal(15,5),
[ss_wind_clearedmw] decimal(15,5),
[ss_solar_compliancemw] decimal(15,5),
[ss_wind_compliancemw] decimal(15,5)
) d
end
go
create or alter procedure InsertP5minUnitsolution3
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'P5min'
    and sub_type = 'Unitsolution'
    and version = '3'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'P5min'
    and sub_type = 'Unitsolution'
    and version = '3'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'P5min', 'Unitsolution', 3);

insert into P5minUnitsolution3(
file_log_id,
[run_datetime],
[interval_datetime],
[duid],
[connectionpointid],
[tradetype],
[agcstatus],
[initialmw],
[totalcleared],
[rampdownrate],
[rampuprate],
[lower5min],
[lower60sec],
[lower6sec],
[raise5min],
[raise60sec],
[raise6sec],
[lowerreg],
[raisereg],
[availability],
[raise6secflags],
[raise60secflags],
[raise5minflags],
[raiseregflags],
[lower6secflags],
[lower60secflags],
[lower5minflags],
[lowerregflags],
[lastchanged],
[semidispatchcap],
[intervention]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[interval_datetime],
d.[duid],
d.[connectionpointid],
d.[tradetype],
d.[agcstatus],
d.[initialmw],
d.[totalcleared],
d.[rampdownrate],
d.[rampuprate],
d.[lower5min],
d.[lower60sec],
d.[lower6sec],
d.[raise5min],
d.[raise60sec],
d.[raise6sec],
d.[lowerreg],
d.[raisereg],
d.[availability],
d.[raise6secflags],
d.[raise60secflags],
d.[raise5minflags],
d.[raiseregflags],
d.[lower6secflags],
d.[lower60secflags],
d.[lower5minflags],
d.[lowerregflags],
d.[lastchanged],
d.[semidispatchcap],
d.[intervention]
from openjson(@data) with (
[run_datetime] datetime2,
[interval_datetime] datetime2,
[duid] varchar(10),
[connectionpointid] varchar(12),
[tradetype] decimal(2,0),
[agcstatus] decimal(2,0),
[initialmw] decimal(15,5),
[totalcleared] decimal(15,5),
[rampdownrate] decimal(15,5),
[rampuprate] decimal(15,5),
[lower5min] decimal(15,5),
[lower60sec] decimal(15,5),
[lower6sec] decimal(15,5),
[raise5min] decimal(15,5),
[raise60sec] decimal(15,5),
[raise6sec] decimal(15,5),
[lowerreg] decimal(15,5),
[raisereg] decimal(15,5),
[availability] decimal(15,5),
[raise6secflags] decimal(3,0),
[raise60secflags] decimal(3,0),
[raise5minflags] decimal(3,0),
[raiseregflags] decimal(3,0),
[lower6secflags] decimal(3,0),
[lower60secflags] decimal(3,0),
[lower5minflags] decimal(3,0),
[lowerregflags] decimal(3,0),
[lastchanged] datetime2,
[semidispatchcap] decimal(3,0),
[intervention] decimal(2,0)
) d
end
go
create or alter procedure InsertP5minConstraintsolution6
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'P5min'
    and sub_type = 'Constraintsolution'
    and version = '6'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'P5min'
    and sub_type = 'Constraintsolution'
    and version = '6'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'P5min', 'Constraintsolution', 6);

insert into P5minConstraintsolution6(
file_log_id,
[run_datetime],
[interval_datetime],
[constraintid],
[rhs],
[marginalvalue],
[violationdegree],
[lastchanged],
[duid],
[genconid_effectivedate],
[genconid_versionno],
[lhs],
[intervention]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[interval_datetime],
d.[constraintid],
d.[rhs],
d.[marginalvalue],
d.[violationdegree],
d.[lastchanged],
d.[duid],
d.[genconid_effectivedate],
d.[genconid_versionno],
d.[lhs],
d.[intervention]
from openjson(@data) with (
[run_datetime] datetime2,
[interval_datetime] datetime2,
[constraintid] varchar(20),
[rhs] decimal(15,5),
[marginalvalue] decimal(15,5),
[violationdegree] decimal(15,5),
[lastchanged] datetime2,
[duid] varchar(20),
[genconid_effectivedate] datetime2,
[genconid_versionno] decimal(22,0),
[lhs] decimal(15,5),
[intervention] decimal(2,0)
) d
end
go
create or alter procedure InsertP5minInterconnectorsoln4
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'P5min'
    and sub_type = 'Interconnectorsoln'
    and version = '4'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'P5min'
    and sub_type = 'Interconnectorsoln'
    and version = '4'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'P5min', 'Interconnectorsoln', 4);

insert into P5minInterconnectorsoln4(
file_log_id,
[run_datetime],
[interconnectorid],
[interval_datetime],
[meteredmwflow],
[mwflow],
[mwlosses],
[marginalvalue],
[violationdegree],
[mnsp],
[exportlimit],
[importlimit],
[marginalloss],
[exportgenconid],
[importgenconid],
[fcasexportlimit],
[fcasimportlimit],
[lastchanged],
[local_price_adjustment_export],
[locally_constrained_export],
[local_price_adjustment_import],
[locally_constrained_import],
[intervention]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[interconnectorid],
d.[interval_datetime],
d.[meteredmwflow],
d.[mwflow],
d.[mwlosses],
d.[marginalvalue],
d.[violationdegree],
d.[mnsp],
d.[exportlimit],
d.[importlimit],
d.[marginalloss],
d.[exportgenconid],
d.[importgenconid],
d.[fcasexportlimit],
d.[fcasimportlimit],
d.[lastchanged],
d.[local_price_adjustment_export],
d.[locally_constrained_export],
d.[local_price_adjustment_import],
d.[locally_constrained_import],
d.[intervention]
from openjson(@data) with (
[run_datetime] datetime2,
[interconnectorid] varchar(10),
[interval_datetime] datetime2,
[meteredmwflow] decimal(15,5),
[mwflow] decimal(15,5),
[mwlosses] decimal(15,5),
[marginalvalue] decimal(15,5),
[violationdegree] decimal(15,5),
[mnsp] decimal(1,0),
[exportlimit] decimal(15,5),
[importlimit] decimal(15,5),
[marginalloss] decimal(15,5),
[exportgenconid] varchar(20),
[importgenconid] varchar(20),
[fcasexportlimit] decimal(15,5),
[fcasimportlimit] decimal(15,5),
[lastchanged] datetime2,
[local_price_adjustment_export] decimal(10,2),
[locally_constrained_export] decimal(1,0),
[local_price_adjustment_import] decimal(10,2),
[locally_constrained_import] decimal(1,0),
[intervention] decimal(2,0)
) d
end
go
create or alter procedure InsertSettlementsLunloadpayment5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Lunloadpayment'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Lunloadpayment'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Lunloadpayment', 5);

insert into SettlementsLunloadpayment5(
file_log_id,
[settlementdate],
[versionno],
[participantid],
[contractid],
[periodid],
[duid],
[regionid],
[tlf],
[ebp],
[rrp],
[enablingprice],
[usageprice],
[ccprice],
[clearedmw],
[unconstrainedmw],
[controlrange],
[enablingpayment],
[usagepayment],
[compensationpayment],
[contractversionno],
[offerdate],
[offerversionno],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[participantid],
d.[contractid],
d.[periodid],
d.[duid],
d.[regionid],
d.[tlf],
d.[ebp],
d.[rrp],
d.[enablingprice],
d.[usageprice],
d.[ccprice],
d.[clearedmw],
d.[unconstrainedmw],
d.[controlrange],
d.[enablingpayment],
d.[usagepayment],
d.[compensationpayment],
d.[contractversionno],
d.[offerdate],
d.[offerversionno],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[contractid] varchar(10),
[periodid] decimal(3,0),
[duid] varchar(10),
[regionid] varchar(10),
[tlf] decimal(7,5),
[ebp] decimal(15,5),
[rrp] decimal(15,5),
[enablingprice] decimal(15,5),
[usageprice] decimal(15,5),
[ccprice] decimal(15,5),
[clearedmw] decimal(15,5),
[unconstrainedmw] decimal(15,5),
[controlrange] decimal(4,0),
[enablingpayment] decimal(15,5),
[usagepayment] decimal(15,5),
[compensationpayment] decimal(15,5),
[contractversionno] decimal(3,0),
[offerdate] datetime2,
[offerversionno] decimal(3,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsFcasRecovery6
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'FcasRecovery'
    and version = '6'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'FcasRecovery'
    and version = '6'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'FcasRecovery', 6);

insert into SettlementsFcasRecovery6(
file_log_id,
[settlementdate],
[versionno],
[participantid],
[regionid],
[periodid],
[lower6sec_recovery],
[raise6sec_recovery],
[lower60sec_recovery],
[raise60sec_recovery],
[lower5min_recovery],
[raise5min_recovery],
[lowerreg_recovery],
[raisereg_recovery],
[lastchanged],
[lower6sec_recovery_gen],
[raise6sec_recovery_gen],
[lower60sec_recovery_gen],
[raise60sec_recovery_gen],
[lower5min_recovery_gen],
[raise5min_recovery_gen],
[lowerreg_recovery_gen],
[raisereg_recovery_gen]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[participantid],
d.[regionid],
d.[periodid],
d.[lower6sec_recovery],
d.[raise6sec_recovery],
d.[lower60sec_recovery],
d.[raise60sec_recovery],
d.[lower5min_recovery],
d.[raise5min_recovery],
d.[lowerreg_recovery],
d.[raisereg_recovery],
d.[lastchanged],
d.[lower6sec_recovery_gen],
d.[raise6sec_recovery_gen],
d.[lower60sec_recovery_gen],
d.[raise60sec_recovery_gen],
d.[lower5min_recovery_gen],
d.[raise5min_recovery_gen],
d.[lowerreg_recovery_gen],
d.[raisereg_recovery_gen]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] varchar(3),
[participantid] varchar(10),
[regionid] varchar(10),
[periodid] decimal(3,0),
[lower6sec_recovery] decimal(18,8),
[raise6sec_recovery] decimal(18,8),
[lower60sec_recovery] decimal(18,8),
[raise60sec_recovery] decimal(18,8),
[lower5min_recovery] decimal(18,8),
[raise5min_recovery] decimal(18,8),
[lowerreg_recovery] decimal(18,8),
[raisereg_recovery] decimal(18,8),
[lastchanged] datetime2,
[lower6sec_recovery_gen] decimal(18,8),
[raise6sec_recovery_gen] decimal(18,8),
[lower60sec_recovery_gen] decimal(18,8),
[raise60sec_recovery_gen] decimal(18,8),
[lower5min_recovery_gen] decimal(18,8),
[raise5min_recovery_gen] decimal(18,8),
[lowerreg_recovery_gen] decimal(18,8),
[raisereg_recovery_gen] decimal(18,8)
) d
end
go
create or alter procedure InsertSettlementsAncillarySummary5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'AncillarySummary'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'AncillarySummary'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'AncillarySummary', 5);

insert into SettlementsAncillarySummary5(
file_log_id,
[settlementdate],
[versionno],
[service],
[paymenttype],
[regionid],
[periodid],
[paymentamount],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[service],
d.[paymenttype],
d.[regionid],
d.[periodid],
d.[paymentamount],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[service] varchar(20),
[paymenttype] varchar(20),
[regionid] varchar(10),
[periodid] decimal(3,0),
[paymentamount] decimal(18,8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsMrRecovery5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'MrRecovery'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'MrRecovery'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'MrRecovery', 5);

insert into SettlementsMrRecovery5(
file_log_id,
[settlementdate],
[versionno],
[regionid],
[participantid],
[duid],
[periodid],
[arodef],
[nta],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[regionid],
d.[participantid],
d.[duid],
d.[periodid],
d.[arodef],
d.[nta],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[regionid] varchar(10),
[participantid] varchar(10),
[duid] varchar(10),
[periodid] decimal(3,0),
[arodef] decimal(16,6),
[nta] decimal(16,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsApcCompensation1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'ApcCompensation'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'ApcCompensation'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'ApcCompensation', 1);

insert into SettlementsApcCompensation1(
file_log_id,
[settlementdate],
[versionno],
[apeventid],
[claimid],
[participantid],
[periodid],
[compensation_amount]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[apeventid],
d.[claimid],
d.[participantid],
d.[periodid],
d.[compensation_amount]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[apeventid] decimal(6,0),
[claimid] decimal(6,0),
[participantid] varchar(20),
[periodid] decimal(3,0),
[compensation_amount] decimal(18,8)
) d
end
go
create or alter procedure InsertSettlementsInterventionrecovery5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Interventionrecovery'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Interventionrecovery'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Interventionrecovery', 5);

insert into SettlementsInterventionrecovery5(
file_log_id,
[settlementdate],
[versionno],
[periodid],
[contractid],
[rcf],
[participantid],
[participantdemand],
[totaldemand],
[interventionpayment],
[interventionamount],
[lastchanged],
[regionid]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[periodid],
d.[contractid],
d.[rcf],
d.[participantid],
d.[participantdemand],
d.[totaldemand],
d.[interventionpayment],
d.[interventionamount],
d.[lastchanged],
d.[regionid]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[periodid] decimal(3,0),
[contractid] varchar(10),
[rcf] char(1),
[participantid] varchar(10),
[participantdemand] decimal(12,5),
[totaldemand] decimal(12,5),
[interventionpayment] decimal(12,5),
[interventionamount] decimal(12,5),
[lastchanged] datetime2,
[regionid] varchar(10)
) d
end
go
create or alter procedure InsertSettlementsIrpartsurplus6
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Irpartsurplus'
    and version = '6'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Irpartsurplus'
    and version = '6'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Irpartsurplus', 6);

insert into SettlementsIrpartsurplus6(
file_log_id,
[settlementdate],
[settlementrunno],
[contractid],
[periodid],
[participantid],
[interconnectorid],
[fromregionid],
[totalsurplus],
[contractallocation],
[surplusvalue],
[lastchanged],
[csp_derogation_amount],
[unadjusted_irsr]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[settlementrunno],
d.[contractid],
d.[periodid],
d.[participantid],
d.[interconnectorid],
d.[fromregionid],
d.[totalsurplus],
d.[contractallocation],
d.[surplusvalue],
d.[lastchanged],
d.[csp_derogation_amount],
d.[unadjusted_irsr]
from openjson(@data) with (
[settlementdate] datetime2,
[settlementrunno] decimal(3,0),
[contractid] varchar(10),
[periodid] decimal(2,0),
[participantid] varchar(10),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[totalsurplus] decimal(15,5),
[contractallocation] decimal(8,5),
[surplusvalue] decimal(15,5),
[lastchanged] datetime2,
[csp_derogation_amount] decimal(18,8),
[unadjusted_irsr] decimal(18,8)
) d
end
go
create or alter procedure InsertSettlementsIrsurplus6
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Irsurplus'
    and version = '6'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Irsurplus'
    and version = '6'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Irsurplus', 6);

insert into SettlementsIrsurplus6(
file_log_id,
[settlementdate],
[settlementrunno],
[periodid],
[interconnectorid],
[regionid],
[mwflow],
[lossfactor],
[surplusvalue],
[lastchanged],
[csp_derogation_amount],
[unadjusted_irsr]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[settlementrunno],
d.[periodid],
d.[interconnectorid],
d.[regionid],
d.[mwflow],
d.[lossfactor],
d.[surplusvalue],
d.[lastchanged],
d.[csp_derogation_amount],
d.[unadjusted_irsr]
from openjson(@data) with (
[settlementdate] datetime2,
[settlementrunno] decimal(3,0),
[periodid] decimal(3,0),
[interconnectorid] varchar(10),
[regionid] varchar(10),
[mwflow] decimal(15,6),
[lossfactor] decimal(15,5),
[surplusvalue] decimal(15,5),
[lastchanged] datetime2,
[csp_derogation_amount] decimal(18,8),
[unadjusted_irsr] decimal(18,8)
) d
end
go
create or alter procedure InsertSettlementsVicenergyflow5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Vicenergyflow'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Vicenergyflow'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Vicenergyflow', 5);

insert into SettlementsVicenergyflow5(
file_log_id,
[settlementdate],
[versionno],
[periodid],
[netflow],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[periodid],
d.[netflow],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[periodid] decimal(3,0),
[netflow] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsLshedrecovery5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Lshedrecovery'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Lshedrecovery'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Lshedrecovery', 5);

insert into SettlementsLshedrecovery5(
file_log_id,
[settlementdate],
[versionno],
[participantid],
[contractid],
[periodid],
[regionid],
[lsepayment],
[ccpayment],
[participantdemand],
[regiondemand],
[lserecovery],
[ccrecovery],
[lastchanged],
[lserecovery_gen],
[ccrecovery_gen],
[participantdemand_gen],
[regiondemand_gen],
[availabilityrecovery],
[availabilityrecovery_gen]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[participantid],
d.[contractid],
d.[periodid],
d.[regionid],
d.[lsepayment],
d.[ccpayment],
d.[participantdemand],
d.[regiondemand],
d.[lserecovery],
d.[ccrecovery],
d.[lastchanged],
d.[lserecovery_gen],
d.[ccrecovery_gen],
d.[participantdemand_gen],
d.[regiondemand_gen],
d.[availabilityrecovery],
d.[availabilityrecovery_gen]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[contractid] varchar(10),
[periodid] decimal(3,0),
[regionid] varchar(10),
[lsepayment] decimal(15,5),
[ccpayment] decimal(15,5),
[participantdemand] decimal(15,5),
[regiondemand] decimal(15,5),
[lserecovery] decimal(15,5),
[ccrecovery] decimal(15,5),
[lastchanged] datetime2,
[lserecovery_gen] decimal(15,5),
[ccrecovery_gen] decimal(15,5),
[participantdemand_gen] decimal(15,5),
[regiondemand_gen] decimal(15,5),
[availabilityrecovery] decimal(16,6),
[availabilityrecovery_gen] decimal(16,6)
) d
end
go
create or alter procedure InsertSettlementsRestartrecovery6
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Restartrecovery'
    and version = '6'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Restartrecovery'
    and version = '6'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Restartrecovery', 6);

insert into SettlementsRestartrecovery6(
file_log_id,
[settlementdate],
[versionno],
[participantid],
[contractid],
[periodid],
[regionid],
[availabilitypayment],
[participantdemand],
[regiondemand],
[availabilityrecovery],
[lastchanged],
[availabilityrecovery_gen],
[participantdemand_gen],
[regiondemand_gen],
[enablingpayment],
[enablingrecovery],
[enablingrecovery_gen]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[participantid],
d.[contractid],
d.[periodid],
d.[regionid],
d.[availabilitypayment],
d.[participantdemand],
d.[regiondemand],
d.[availabilityrecovery],
d.[lastchanged],
d.[availabilityrecovery_gen],
d.[participantdemand_gen],
d.[regiondemand_gen],
d.[enablingpayment],
d.[enablingrecovery],
d.[enablingrecovery_gen]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[contractid] varchar(10),
[periodid] decimal(3,0),
[regionid] varchar(10),
[availabilitypayment] decimal(15,5),
[participantdemand] decimal(15,5),
[regiondemand] decimal(15,5),
[availabilityrecovery] decimal(15,5),
[lastchanged] datetime2,
[availabilityrecovery_gen] decimal(15,5),
[participantdemand_gen] decimal(15,5),
[regiondemand_gen] decimal(15,5),
[enablingpayment] decimal(18,8),
[enablingrecovery] decimal(18,8),
[enablingrecovery_gen] decimal(18,8)
) d
end
go
create or alter procedure InsertSettlementsGendata6
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Gendata'
    and version = '6'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Gendata'
    and version = '6'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Gendata', 6);

insert into SettlementsGendata6(
file_log_id,
[settlementdate],
[versionno],
[periodid],
[participantid],
[stationid],
[duid],
[gensetid],
[regionid],
[genergy],
[aenergy],
[gpower],
[apower],
[rrp],
[eep],
[tlf],
[cprrp],
[cpeep],
[netenergy],
[energycost],
[excessenergycost],
[apc],
[resc],
[resp],
[lastchanged],
[expenergy],
[expenergycost],
[meterrunno],
[mda],
[secondary_tlf]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[periodid],
d.[participantid],
d.[stationid],
d.[duid],
d.[gensetid],
d.[regionid],
d.[genergy],
d.[aenergy],
d.[gpower],
d.[apower],
d.[rrp],
d.[eep],
d.[tlf],
d.[cprrp],
d.[cpeep],
d.[netenergy],
d.[energycost],
d.[excessenergycost],
d.[apc],
d.[resc],
d.[resp],
d.[lastchanged],
d.[expenergy],
d.[expenergycost],
d.[meterrunno],
d.[mda],
d.[secondary_tlf]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(10,0),
[periodid] decimal(10,0),
[participantid] varchar(10),
[stationid] varchar(10),
[duid] varchar(10),
[gensetid] varchar(10),
[regionid] varchar(10),
[genergy] decimal(16,6),
[aenergy] decimal(16,6),
[gpower] decimal(16,6),
[apower] decimal(16,6),
[rrp] decimal(20,5),
[eep] decimal(16,6),
[tlf] decimal(7,5),
[cprrp] decimal(16,6),
[cpeep] decimal(16,6),
[netenergy] decimal(16,6),
[energycost] decimal(16,6),
[excessenergycost] decimal(16,6),
[apc] decimal(16,6),
[resc] decimal(16,6),
[resp] decimal(16,6),
[lastchanged] datetime2,
[expenergy] decimal(15,6),
[expenergycost] decimal(15,6),
[meterrunno] decimal(6,0),
[mda] varchar(10),
[secondary_tlf] decimal(7,5)
) d
end
go
create or alter procedure InsertSettlementsRunParameter5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'RunParameter'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'RunParameter'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'RunParameter', 5);

insert into SettlementsRunParameter5(
file_log_id,
[settlementdate],
[versionno],
[parameterid],
[numvalue],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[parameterid],
d.[numvalue],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[parameterid] varchar(20),
[numvalue] decimal(18,8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsReallocations5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Reallocations'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Reallocations'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Reallocations', 5);

insert into SettlementsReallocations5(
file_log_id,
[settlementdate],
[runno],
[periodid],
[participantid],
[reallocationid],
[reallocationvalue],
[energy],
[rrp],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[periodid],
d.[participantid],
d.[reallocationid],
d.[reallocationvalue],
d.[energy],
d.[rrp],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[periodid] decimal(3,0),
[participantid] varchar(10),
[reallocationid] varchar(20),
[reallocationvalue] decimal(15,5),
[energy] decimal(15,5),
[rrp] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsAgcrecovery5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Agcrecovery'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Agcrecovery'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Agcrecovery', 5);

insert into SettlementsAgcrecovery5(
file_log_id,
[settlementdate],
[versionno],
[participantid],
[contractid],
[periodid],
[regionid],
[enablingpayment],
[participantdemand],
[regiondemand],
[enablingrecovery],
[lastchanged],
[enablingrecovery_gen],
[participantdemand_gen],
[regiondemand_gen]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[participantid],
d.[contractid],
d.[periodid],
d.[regionid],
d.[enablingpayment],
d.[participantdemand],
d.[regiondemand],
d.[enablingrecovery],
d.[lastchanged],
d.[enablingrecovery_gen],
d.[participantdemand_gen],
d.[regiondemand_gen]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[contractid] varchar(10),
[periodid] decimal(3,0),
[regionid] varchar(10),
[enablingpayment] decimal(15,5),
[participantdemand] decimal(15,5),
[regiondemand] decimal(15,5),
[enablingrecovery] decimal(15,5),
[lastchanged] datetime2,
[enablingrecovery_gen] decimal(15,5),
[participantdemand_gen] decimal(15,5),
[regiondemand_gen] decimal(15,5)
) d
end
go
create or alter procedure InsertSettlementsRpowerpayment6
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Rpowerpayment'
    and version = '6'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Rpowerpayment'
    and version = '6'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Rpowerpayment', 6);

insert into SettlementsRpowerpayment6(
file_log_id,
[settlementdate],
[versionno],
[participantid],
[contractid],
[periodid],
[duid],
[regionid],
[tlf],
[ebp],
[rrp],
[mvaraprice],
[mvareprice],
[mvargprice],
[ccprice],
[synccompensation],
[mta],
[mtg],
[blocksize],
[avaflag],
[clearedmw],
[unconstrainedmw],
[availabilitypayment],
[enablingpayment],
[ccpayment],
[contractversionno],
[offerdate],
[offerversionno],
[lastchanged],
[availabilitypayment_rebate]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[participantid],
d.[contractid],
d.[periodid],
d.[duid],
d.[regionid],
d.[tlf],
d.[ebp],
d.[rrp],
d.[mvaraprice],
d.[mvareprice],
d.[mvargprice],
d.[ccprice],
d.[synccompensation],
d.[mta],
d.[mtg],
d.[blocksize],
d.[avaflag],
d.[clearedmw],
d.[unconstrainedmw],
d.[availabilitypayment],
d.[enablingpayment],
d.[ccpayment],
d.[contractversionno],
d.[offerdate],
d.[offerversionno],
d.[lastchanged],
d.[availabilitypayment_rebate]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[contractid] varchar(10),
[periodid] decimal(3,0),
[duid] varchar(10),
[regionid] varchar(10),
[tlf] decimal(7,5),
[ebp] decimal(15,5),
[rrp] decimal(15,5),
[mvaraprice] decimal(15,5),
[mvareprice] decimal(15,5),
[mvargprice] decimal(15,5),
[ccprice] decimal(15,5),
[synccompensation] decimal(1,0),
[mta] decimal(15,5),
[mtg] decimal(15,5),
[blocksize] decimal(4,0),
[avaflag] decimal(1,0),
[clearedmw] decimal(15,5),
[unconstrainedmw] decimal(15,5),
[availabilitypayment] decimal(15,5),
[enablingpayment] decimal(15,5),
[ccpayment] decimal(15,5),
[contractversionno] decimal(3,0),
[offerdate] datetime2,
[offerversionno] decimal(3,0),
[lastchanged] datetime2,
[availabilitypayment_rebate] decimal(18,8)
) d
end
go
create or alter procedure InsertSettlementsVicenergyfigures5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Vicenergyfigures'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Vicenergyfigures'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Vicenergyfigures', 5);

insert into SettlementsVicenergyfigures5(
file_log_id,
[settlementdate],
[versionno],
[periodid],
[totalgenoutput],
[totalpcsd],
[lastchanged],
[tlr],
[mlf]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[periodid],
d.[totalgenoutput],
d.[totalpcsd],
d.[lastchanged],
d.[tlr],
d.[mlf]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[periodid] decimal(3,0),
[totalgenoutput] decimal(15,5),
[totalpcsd] decimal(15,5),
[lastchanged] datetime2,
[tlr] decimal(15,6),
[mlf] decimal(15,6)
) d
end
go
create or alter procedure InsertSettlementsGendataregion5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Gendataregion'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Gendataregion'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Gendataregion', 5);

insert into SettlementsGendataregion5(
file_log_id,
[settlementdate],
[versionno],
[periodid],
[regionid],
[genergy],
[aenergy],
[gpower],
[apower],
[netenergy],
[energycost],
[excessenergycost],
[expenergy],
[expenergycost],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[periodid],
d.[regionid],
d.[genergy],
d.[aenergy],
d.[gpower],
d.[apower],
d.[netenergy],
d.[energycost],
d.[excessenergycost],
d.[expenergy],
d.[expenergycost],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(22,10),
[periodid] decimal(22,10),
[regionid] varchar(10),
[genergy] decimal(22,0),
[aenergy] decimal(22,0),
[gpower] decimal(22,0),
[apower] decimal(22,0),
[netenergy] decimal(27,5),
[energycost] decimal(27,5),
[excessenergycost] decimal(27,5),
[expenergy] decimal(27,6),
[expenergycost] decimal(27,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsNmasRecoveryRbf1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'NmasRecoveryRbf'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'NmasRecoveryRbf'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'NmasRecoveryRbf', 1);

insert into SettlementsNmasRecoveryRbf1(
file_log_id,
[settlementdate],
[versionno],
[periodid],
[service],
[contractid],
[paymenttype],
[regionid],
[rbf],
[payment_amount],
[recovery_amount],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[periodid],
d.[service],
d.[contractid],
d.[paymenttype],
d.[regionid],
d.[rbf],
d.[payment_amount],
d.[recovery_amount],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[periodid] decimal(3,0),
[service] varchar(10),
[contractid] varchar(10),
[paymenttype] varchar(20),
[regionid] varchar(10),
[rbf] decimal(18,8),
[payment_amount] decimal(18,8),
[recovery_amount] decimal(18,8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsMarketfees5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Marketfees'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Marketfees'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Marketfees', 5);

insert into SettlementsMarketfees5(
file_log_id,
[settlementdate],
[runno],
[participantid],
[periodid],
[marketfeeid],
[marketfeevalue],
[energy],
[lastchanged],
[participantcategoryid]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[participantid],
d.[periodid],
d.[marketfeeid],
d.[marketfeevalue],
d.[energy],
d.[lastchanged],
d.[participantcategoryid]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[participantid] varchar(10),
[periodid] decimal(3,0),
[marketfeeid] varchar(10),
[marketfeevalue] decimal(15,5),
[energy] decimal(16,6),
[lastchanged] datetime2,
[participantcategoryid] varchar(10)
) d
end
go
create or alter procedure InsertSettlementsApcRecovery1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'ApcRecovery'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'ApcRecovery'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'ApcRecovery', 1);

insert into SettlementsApcRecovery1(
file_log_id,
[settlementdate],
[versionno],
[apeventid],
[claimid],
[participantid],
[periodid],
[regionid],
[recovery_amount],
[region_recovery_br_amount]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[apeventid],
d.[claimid],
d.[participantid],
d.[periodid],
d.[regionid],
d.[recovery_amount],
d.[region_recovery_br_amount]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[apeventid] decimal(6,0),
[claimid] decimal(6,0),
[participantid] varchar(20),
[periodid] decimal(3,0),
[regionid] varchar(20),
[recovery_amount] decimal(18,8),
[region_recovery_br_amount] decimal(18,8)
) d
end
go
create or alter procedure InsertSettlementsLunloadrecovery5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Lunloadrecovery'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Lunloadrecovery'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Lunloadrecovery', 5);

insert into SettlementsLunloadrecovery5(
file_log_id,
[settlementdate],
[versionno],
[participantid],
[contractid],
[periodid],
[regionid],
[enablingpayment],
[usagepayment],
[compensationpayment],
[participantdemand],
[regiondemand],
[enablingrecovery],
[usagerecovery],
[compensationrecovery],
[lastchanged],
[enablingrecovery_gen],
[usagerecovery_gen],
[compensationrecovery_gen],
[participantdemand_gen],
[regiondemand_gen]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[participantid],
d.[contractid],
d.[periodid],
d.[regionid],
d.[enablingpayment],
d.[usagepayment],
d.[compensationpayment],
d.[participantdemand],
d.[regiondemand],
d.[enablingrecovery],
d.[usagerecovery],
d.[compensationrecovery],
d.[lastchanged],
d.[enablingrecovery_gen],
d.[usagerecovery_gen],
d.[compensationrecovery_gen],
d.[participantdemand_gen],
d.[regiondemand_gen]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[contractid] varchar(10),
[periodid] decimal(3,0),
[regionid] varchar(10),
[enablingpayment] decimal(15,5),
[usagepayment] decimal(15,5),
[compensationpayment] decimal(15,5),
[participantdemand] decimal(15,5),
[regiondemand] decimal(15,5),
[enablingrecovery] decimal(15,5),
[usagerecovery] decimal(15,5),
[compensationrecovery] decimal(15,5),
[lastchanged] datetime2,
[enablingrecovery_gen] decimal(15,5),
[usagerecovery_gen] decimal(15,5),
[compensationrecovery_gen] decimal(15,5),
[participantdemand_gen] decimal(15,5),
[regiondemand_gen] decimal(15,5)
) d
end
go
create or alter procedure InsertSettlementsFcasPayment5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'FcasPayment'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'FcasPayment'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'FcasPayment', 5);

insert into SettlementsFcasPayment5(
file_log_id,
[settlementdate],
[versionno],
[participantid],
[duid],
[regionid],
[periodid],
[lower6sec_payment],
[raise6sec_payment],
[lower60sec_payment],
[raise60sec_payment],
[lower5min_payment],
[raise5min_payment],
[lowerreg_payment],
[raisereg_payment],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[participantid],
d.[duid],
d.[regionid],
d.[periodid],
d.[lower6sec_payment],
d.[raise6sec_payment],
d.[lower60sec_payment],
d.[raise60sec_payment],
d.[lower5min_payment],
d.[raise5min_payment],
d.[lowerreg_payment],
d.[raisereg_payment],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[duid] varchar(10),
[regionid] varchar(10),
[periodid] decimal(3,0),
[lower6sec_payment] decimal(18,8),
[raise6sec_payment] decimal(18,8),
[lower60sec_payment] decimal(18,8),
[raise60sec_payment] decimal(18,8),
[lower5min_payment] decimal(18,8),
[raise5min_payment] decimal(18,8),
[lowerreg_payment] decimal(18,8),
[raisereg_payment] decimal(18,8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsIntraregionresidues5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Intraregionresidues'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Intraregionresidues'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Intraregionresidues', 5);

insert into SettlementsIntraregionresidues5(
file_log_id,
[settlementdate],
[runno],
[periodid],
[regionid],
[ep],
[ec],
[rrp],
[exp],
[irss],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[periodid],
d.[regionid],
d.[ep],
d.[ec],
d.[rrp],
d.[exp],
d.[irss],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[periodid] decimal(3,0),
[regionid] varchar(10),
[ep] decimal(15,5),
[ec] decimal(15,5),
[rrp] decimal(15,5),
[exp] decimal(15,5),
[irss] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsAgcpayment5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Agcpayment'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Agcpayment'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Agcpayment', 5);

insert into SettlementsAgcpayment5(
file_log_id,
[settlementdate],
[versionno],
[participantid],
[contractid],
[periodid],
[duid],
[regionid],
[tlf],
[ebp],
[rrp],
[clearedmw],
[initialmw],
[enablingpayment],
[contractversionno],
[offerdate],
[offerversionno],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[participantid],
d.[contractid],
d.[periodid],
d.[duid],
d.[regionid],
d.[tlf],
d.[ebp],
d.[rrp],
d.[clearedmw],
d.[initialmw],
d.[enablingpayment],
d.[contractversionno],
d.[offerdate],
d.[offerversionno],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[contractid] varchar(10),
[periodid] decimal(3,0),
[duid] varchar(10),
[regionid] varchar(10),
[tlf] decimal(7,5),
[ebp] decimal(15,5),
[rrp] decimal(15,5),
[clearedmw] decimal(15,5),
[initialmw] decimal(15,5),
[enablingpayment] decimal(15,5),
[contractversionno] decimal(3,0),
[offerdate] datetime2,
[offerversionno] decimal(3,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsDaytrack5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Daytrack'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Daytrack'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Daytrack', 5);

insert into SettlementsDaytrack5(
file_log_id,
[settlementdate],
[regionid],
[exanterunstatus],
[exanterunno],
[expostrunstatus],
[expostrunno],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[regionid],
d.[exanterunstatus],
d.[exanterunno],
d.[expostrunstatus],
d.[expostrunno],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[regionid] varchar(10),
[exanterunstatus] varchar(15),
[exanterunno] decimal(3,0),
[expostrunstatus] varchar(15),
[expostrunno] decimal(3,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsIntervention5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Intervention'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Intervention'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Intervention', 5);

insert into SettlementsIntervention5(
file_log_id,
[settlementdate],
[versionno],
[periodid],
[contractid],
[contractversion],
[participantid],
[regionid],
[duid],
[rcf],
[interventionpayment],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[periodid],
d.[contractid],
d.[contractversion],
d.[participantid],
d.[regionid],
d.[duid],
d.[rcf],
d.[interventionpayment],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[periodid] decimal(3,0),
[contractid] varchar(10),
[contractversion] decimal(3,0),
[participantid] varchar(10),
[regionid] varchar(10),
[duid] varchar(10),
[rcf] char(1),
[interventionpayment] decimal(12,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsCpdata5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Cpdata'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Cpdata'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Cpdata', 5);

insert into SettlementsCpdata5(
file_log_id,
[settlementdate],
[versionno],
[periodid],
[participantid],
[tcpid],
[regionid],
[igenergy],
[xgenergy],
[inenergy],
[xnenergy],
[ipower],
[xpower],
[rrp],
[eep],
[tlf],
[cprrp],
[cpeep],
[ta],
[ep],
[apc],
[resc],
[resp],
[meterrunno],
[lastchanged],
[hostdistributor],
[mda]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[periodid],
d.[participantid],
d.[tcpid],
d.[regionid],
d.[igenergy],
d.[xgenergy],
d.[inenergy],
d.[xnenergy],
d.[ipower],
d.[xpower],
d.[rrp],
d.[eep],
d.[tlf],
d.[cprrp],
d.[cpeep],
d.[ta],
d.[ep],
d.[apc],
d.[resc],
d.[resp],
d.[meterrunno],
d.[lastchanged],
d.[hostdistributor],
d.[mda]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(10,0),
[periodid] decimal(10,0),
[participantid] varchar(10),
[tcpid] varchar(10),
[regionid] varchar(10),
[igenergy] decimal(16,6),
[xgenergy] decimal(16,6),
[inenergy] decimal(16,6),
[xnenergy] decimal(16,6),
[ipower] decimal(16,6),
[xpower] decimal(16,6),
[rrp] decimal(20,5),
[eep] decimal(16,6),
[tlf] decimal(7,5),
[cprrp] decimal(16,6),
[cpeep] decimal(16,6),
[ta] decimal(16,6),
[ep] decimal(16,6),
[apc] decimal(16,6),
[resc] decimal(16,6),
[resp] decimal(16,6),
[meterrunno] decimal(10,0),
[lastchanged] datetime2,
[hostdistributor] varchar(10),
[mda] varchar(10)
) d
end
go
create or alter procedure InsertSettlementsVicboundaryenergy5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Vicboundaryenergy'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Vicboundaryenergy'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Vicboundaryenergy', 5);

insert into SettlementsVicboundaryenergy5(
file_log_id,
[settlementdate],
[versionno],
[participantid],
[periodid],
[boundaryenergy],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[participantid],
d.[periodid],
d.[boundaryenergy],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[periodid] decimal(3,0),
[boundaryenergy] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsLuloadrecovery5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Luloadrecovery'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Luloadrecovery'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Luloadrecovery', 5);

insert into SettlementsLuloadrecovery5(
file_log_id,
[settlementdate],
[versionno],
[participantid],
[contractid],
[periodid],
[regionid],
[enablingpayment],
[usagepayment],
[compensationpayment],
[participantdemand],
[regiondemand],
[enablingrecovery],
[usagerecovery],
[compensationrecovery],
[lastchanged],
[enablingrecovery_gen],
[usagerecovery_gen],
[compensationrecovery_gen],
[participantdemand_gen],
[regiondemand_gen]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[participantid],
d.[contractid],
d.[periodid],
d.[regionid],
d.[enablingpayment],
d.[usagepayment],
d.[compensationpayment],
d.[participantdemand],
d.[regiondemand],
d.[enablingrecovery],
d.[usagerecovery],
d.[compensationrecovery],
d.[lastchanged],
d.[enablingrecovery_gen],
d.[usagerecovery_gen],
d.[compensationrecovery_gen],
d.[participantdemand_gen],
d.[regiondemand_gen]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[contractid] varchar(10),
[periodid] decimal(3,0),
[regionid] varchar(10),
[enablingpayment] decimal(15,5),
[usagepayment] decimal(15,5),
[compensationpayment] decimal(15,5),
[participantdemand] decimal(15,5),
[regiondemand] decimal(15,5),
[enablingrecovery] decimal(15,5),
[usagerecovery] decimal(15,5),
[compensationrecovery] decimal(15,5),
[lastchanged] datetime2,
[enablingrecovery_gen] decimal(15,5),
[usagerecovery_gen] decimal(15,5),
[compensationrecovery_gen] decimal(15,5),
[participantdemand_gen] decimal(15,5),
[regiondemand_gen] decimal(15,5)
) d
end
go
create or alter procedure InsertSettlementsNmasRecovery2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'NmasRecovery'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'NmasRecovery'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'NmasRecovery', 2);

insert into SettlementsNmasRecovery2(
file_log_id,
[settlementdate],
[versionno],
[periodid],
[participantid],
[service],
[contractid],
[paymenttype],
[regionid],
[rbf],
[payment_amount],
[participant_energy],
[region_energy],
[recovery_amount],
[lastchanged],
[participant_generation],
[region_generation],
[recovery_amount_customer],
[recovery_amount_generator]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[periodid],
d.[participantid],
d.[service],
d.[contractid],
d.[paymenttype],
d.[regionid],
d.[rbf],
d.[payment_amount],
d.[participant_energy],
d.[region_energy],
d.[recovery_amount],
d.[lastchanged],
d.[participant_generation],
d.[region_generation],
d.[recovery_amount_customer],
d.[recovery_amount_generator]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[periodid] decimal(3,0),
[participantid] varchar(20),
[service] varchar(10),
[contractid] varchar(10),
[paymenttype] varchar(20),
[regionid] varchar(10),
[rbf] decimal(18,8),
[payment_amount] decimal(18,8),
[participant_energy] decimal(18,8),
[region_energy] decimal(18,8),
[recovery_amount] decimal(18,8),
[lastchanged] datetime2,
[participant_generation] decimal(18,8),
[region_generation] decimal(18,8),
[recovery_amount_customer] decimal(18,8),
[recovery_amount_generator] decimal(18,8)
) d
end
go
create or alter procedure InsertSettlementsLshedpayment5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Lshedpayment'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Lshedpayment'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Lshedpayment', 5);

insert into SettlementsLshedpayment5(
file_log_id,
[settlementdate],
[versionno],
[participantid],
[contractid],
[periodid],
[duid],
[regionid],
[tlf],
[rrp],
[lseprice],
[mcpprice],
[lscr],
[lsepayment],
[ccpayment],
[constrainedmw],
[unconstrainedmw],
[als],
[initialdemand],
[finaldemand],
[contractversionno],
[offerdate],
[offerversionno],
[lastchanged],
[availabilitypayment]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[participantid],
d.[contractid],
d.[periodid],
d.[duid],
d.[regionid],
d.[tlf],
d.[rrp],
d.[lseprice],
d.[mcpprice],
d.[lscr],
d.[lsepayment],
d.[ccpayment],
d.[constrainedmw],
d.[unconstrainedmw],
d.[als],
d.[initialdemand],
d.[finaldemand],
d.[contractversionno],
d.[offerdate],
d.[offerversionno],
d.[lastchanged],
d.[availabilitypayment]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[contractid] varchar(10),
[periodid] decimal(3,0),
[duid] varchar(10),
[regionid] varchar(10),
[tlf] decimal(7,5),
[rrp] decimal(15,5),
[lseprice] decimal(15,5),
[mcpprice] decimal(15,5),
[lscr] decimal(4,0),
[lsepayment] decimal(15,5),
[ccpayment] decimal(15,5),
[constrainedmw] decimal(15,5),
[unconstrainedmw] decimal(15,5),
[als] decimal(15,5),
[initialdemand] decimal(15,5),
[finaldemand] decimal(15,5),
[contractversionno] decimal(3,0),
[offerdate] datetime2,
[offerversionno] decimal(3,0),
[lastchanged] datetime2,
[availabilitypayment] decimal(16,6)
) d
end
go
create or alter procedure InsertSettlementsFcascomp5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Fcascomp'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Fcascomp'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Fcascomp', 5);

insert into SettlementsFcascomp5(
file_log_id,
[settlementdate],
[versionno],
[participantid],
[duid],
[regionid],
[periodid],
[ccprice],
[clearedmw],
[unconstrainedmw],
[ebp],
[tlf],
[rrp],
[excessgen],
[fcascomp],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[participantid],
d.[duid],
d.[regionid],
d.[periodid],
d.[ccprice],
d.[clearedmw],
d.[unconstrainedmw],
d.[ebp],
d.[tlf],
d.[rrp],
d.[excessgen],
d.[fcascomp],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[duid] varchar(10),
[regionid] varchar(10),
[periodid] decimal(3,0),
[ccprice] decimal(15,5),
[clearedmw] decimal(15,5),
[unconstrainedmw] decimal(15,5),
[ebp] decimal(15,5),
[tlf] decimal(7,5),
[rrp] decimal(15,5),
[excessgen] decimal(15,5),
[fcascomp] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsFcasregionrecovery5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Fcasregionrecovery'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Fcasregionrecovery'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Fcasregionrecovery', 5);

insert into SettlementsFcasregionrecovery5(
file_log_id,
[settlementdate],
[versionno],
[bidtype],
[regionid],
[periodid],
[generatorregionenergy],
[customerregionenergy],
[regionrecovery],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[bidtype],
d.[regionid],
d.[periodid],
d.[generatorregionenergy],
d.[customerregionenergy],
d.[regionrecovery],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[bidtype] varchar(10),
[regionid] varchar(10),
[periodid] decimal(3,0),
[generatorregionenergy] decimal(16,6),
[customerregionenergy] decimal(16,6),
[regionrecovery] decimal(18,8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsIrfmrecovery5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Irfmrecovery'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Irfmrecovery'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Irfmrecovery', 5);

insert into SettlementsIrfmrecovery5(
file_log_id,
[settlementdate],
[versionno],
[periodid],
[irfmid],
[irmfversion],
[participantid],
[participantdemand],
[totaltcd],
[totaltfd],
[irfmamount],
[irfmpayment],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[periodid],
d.[irfmid],
d.[irmfversion],
d.[participantid],
d.[participantdemand],
d.[totaltcd],
d.[totaltfd],
d.[irfmamount],
d.[irfmpayment],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[periodid] decimal(3,0),
[irfmid] varchar(10),
[irmfversion] decimal(3,0),
[participantid] varchar(10),
[participantdemand] decimal(12,5),
[totaltcd] decimal(12,5),
[totaltfd] decimal(12,5),
[irfmamount] decimal(12,5),
[irfmpayment] decimal(12,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsRestartpayment6
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Restartpayment'
    and version = '6'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Restartpayment'
    and version = '6'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Restartpayment', 6);

insert into SettlementsRestartpayment6(
file_log_id,
[settlementdate],
[versionno],
[participantid],
[contractid],
[periodid],
[regionid],
[restarttype],
[avaflag],
[availabilityprice],
[tcf],
[availabilitypayment],
[contractversionno],
[offerdate],
[offerversionno],
[lastchanged],
[enablingpayment]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[participantid],
d.[contractid],
d.[periodid],
d.[regionid],
d.[restarttype],
d.[avaflag],
d.[availabilityprice],
d.[tcf],
d.[availabilitypayment],
d.[contractversionno],
d.[offerdate],
d.[offerversionno],
d.[lastchanged],
d.[enablingpayment]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[contractid] varchar(10),
[periodid] decimal(3,0),
[regionid] varchar(10),
[restarttype] decimal(1,0),
[avaflag] decimal(1,0),
[availabilityprice] decimal(15,5),
[tcf] decimal(1,0),
[availabilitypayment] decimal(15,5),
[contractversionno] decimal(3,0),
[offerdate] datetime2,
[offerversionno] decimal(3,0),
[lastchanged] datetime2,
[enablingpayment] decimal(18,8)
) d
end
go
create or alter procedure InsertSettlementsMrPayment5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'MrPayment'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'MrPayment'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'MrPayment', 5);

insert into SettlementsMrPayment5(
file_log_id,
[settlementdate],
[versionno],
[regionid],
[participantid],
[duid],
[periodid],
[mr_capacity],
[uncapped_payment],
[capped_payment],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[regionid],
d.[participantid],
d.[duid],
d.[periodid],
d.[mr_capacity],
d.[uncapped_payment],
d.[capped_payment],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[regionid] varchar(10),
[participantid] varchar(10),
[duid] varchar(10),
[periodid] decimal(3,0),
[mr_capacity] decimal(16,6),
[uncapped_payment] decimal(16,6),
[capped_payment] decimal(16,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsIrnspsurplus6
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Irnspsurplus'
    and version = '6'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Irnspsurplus'
    and version = '6'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Irnspsurplus', 6);

insert into SettlementsIrnspsurplus6(
file_log_id,
[settlementdate],
[settlementrunno],
[contractid],
[periodid],
[participantid],
[interconnectorid],
[fromregionid],
[totalsurplus],
[contractallocation],
[surplusvalue],
[lastchanged],
[csp_derogation_amount],
[unadjusted_irsr]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[settlementrunno],
d.[contractid],
d.[periodid],
d.[participantid],
d.[interconnectorid],
d.[fromregionid],
d.[totalsurplus],
d.[contractallocation],
d.[surplusvalue],
d.[lastchanged],
d.[csp_derogation_amount],
d.[unadjusted_irsr]
from openjson(@data) with (
[settlementdate] datetime2,
[settlementrunno] decimal(3,0),
[contractid] varchar(10),
[periodid] decimal(2,0),
[participantid] varchar(10),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[totalsurplus] decimal(15,5),
[contractallocation] decimal(8,5),
[surplusvalue] decimal(15,5),
[lastchanged] datetime2,
[csp_derogation_amount] decimal(18,8),
[unadjusted_irsr] decimal(18,8)
) d
end
go
create or alter procedure InsertSettlementsCpdataregion5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Cpdataregion'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Cpdataregion'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Cpdataregion', 5);

insert into SettlementsCpdataregion5(
file_log_id,
[settlementdate],
[versionno],
[periodid],
[regionid],
[sumigenergy],
[sumxgenergy],
[suminenergy],
[sumxnenergy],
[sumipower],
[sumxpower],
[lastchanged],
[sumep]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[periodid],
d.[regionid],
d.[sumigenergy],
d.[sumxgenergy],
d.[suminenergy],
d.[sumxnenergy],
d.[sumipower],
d.[sumxpower],
d.[lastchanged],
d.[sumep]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(22,10),
[periodid] decimal(22,10),
[regionid] varchar(10),
[sumigenergy] decimal(27,5),
[sumxgenergy] decimal(27,5),
[suminenergy] decimal(27,5),
[sumxnenergy] decimal(27,5),
[sumipower] decimal(22,0),
[sumxpower] decimal(22,0),
[lastchanged] datetime2,
[sumep] decimal(15,5)
) d
end
go
create or alter procedure InsertSettlementsSmallgendata1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Smallgendata'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Smallgendata'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Smallgendata', 1);

insert into SettlementsSmallgendata1(
file_log_id,
[settlementdate],
[versionno],
[connectionpointid],
[periodid],
[participantid],
[regionid],
[importenergy],
[exportenergy],
[rrp],
[tlf],
[impenergycost],
[expenergycost],
[lastchanged],
[raise60secreq],
[raise60secsupplyprice],
[raise6secdispatch],
[raise6secimport],
[raise6seclocaldispatch],
[raise6seclocalprice],
[raise6seclocalreq],
[raise6secprice],
[raise6secreq],
[raise6secsupplyprice],
[lastchanged],
[initialsupply],
[clearedsupply],
[lowerregimport],
[lowerreglocaldispatch],
[lowerreglocalreq],
[lowerregreq],
[raiseregimport],
[raisereglocaldispatch],
[raisereglocalreq],
[raiseregreq],
[raise5minlocalviolation],
[raisereglocalviolation],
[raise60seclocalviolation],
[raise6seclocalviolation],
[lower5minlocalviolation],
[lowerreglocalviolation],
[lower60seclocalviolation],
[lower6seclocalviolation],
[raise5minviolation],
[raiseregviolation],
[raise60secviolation],
[raise6secviolation],
[lower5minviolation],
[lowerregviolation],
[lower60secviolation],
[lower6secviolation],
[totalintermittentgeneration],
[demand_and_nonschedgen],
[uigf]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[connectionpointid],
d.[periodid],
d.[participantid],
d.[regionid],
d.[importenergy],
d.[exportenergy],
d.[rrp],
d.[tlf],
d.[impenergycost],
d.[expenergycost],
d.[lastchanged],
d.[raise60secreq],
d.[raise60secsupplyprice],
d.[raise6secdispatch],
d.[raise6secimport],
d.[raise6seclocaldispatch],
d.[raise6seclocalprice],
d.[raise6seclocalreq],
d.[raise6secprice],
d.[raise6secreq],
d.[raise6secsupplyprice],
d.[lastchanged],
d.[initialsupply],
d.[clearedsupply],
d.[lowerregimport],
d.[lowerreglocaldispatch],
d.[lowerreglocalreq],
d.[lowerregreq],
d.[raiseregimport],
d.[raisereglocaldispatch],
d.[raisereglocalreq],
d.[raiseregreq],
d.[raise5minlocalviolation],
d.[raisereglocalviolation],
d.[raise60seclocalviolation],
d.[raise6seclocalviolation],
d.[lower5minlocalviolation],
d.[lowerreglocalviolation],
d.[lower60seclocalviolation],
d.[lower6seclocalviolation],
d.[raise5minviolation],
d.[raiseregviolation],
d.[raise60secviolation],
d.[raise6secviolation],
d.[lower5minviolation],
d.[lowerregviolation],
d.[lower60secviolation],
d.[lower6secviolation],
d.[totalintermittentgeneration],
d.[demand_and_nonschedgen],
d.[uigf]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[connectionpointid] varchar(20),
[periodid] decimal(3,0),
[participantid] varchar(20),
[regionid] varchar(20),
[importenergy] decimal(18,8),
[exportenergy] decimal(18,8),
[rrp] decimal(18,8),
[tlf] decimal(18,8),
[impenergycost] decimal(18,8),
[expenergycost] decimal(18,8),
[lastchanged] datetime2,
[raise60secreq] decimal(15,5),
[raise60secsupplyprice] decimal(15,5),
[raise6secdispatch] decimal(15,5),
[raise6secimport] decimal(15,5),
[raise6seclocaldispatch] decimal(15,5),
[raise6seclocalprice] decimal(15,5),
[raise6seclocalreq] decimal(15,5),
[raise6secprice] decimal(15,5),
[raise6secreq] decimal(15,5),
[raise6secsupplyprice] decimal(15,5),
[lastchanged] datetime2,
[initialsupply] decimal(15,5),
[clearedsupply] decimal(15,5),
[lowerregimport] decimal(15,5),
[lowerreglocaldispatch] decimal(15,5),
[lowerreglocalreq] decimal(15,5),
[lowerregreq] decimal(15,5),
[raiseregimport] decimal(15,5),
[raisereglocaldispatch] decimal(15,5),
[raisereglocalreq] decimal(15,5),
[raiseregreq] decimal(15,5),
[raise5minlocalviolation] decimal(15,5),
[raisereglocalviolation] decimal(15,5),
[raise60seclocalviolation] decimal(15,5),
[raise6seclocalviolation] decimal(15,5),
[lower5minlocalviolation] decimal(15,5),
[lowerreglocalviolation] decimal(15,5),
[lower60seclocalviolation] decimal(15,5),
[lower6seclocalviolation] decimal(15,5),
[raise5minviolation] decimal(15,5),
[raiseregviolation] decimal(15,5),
[raise60secviolation] decimal(15,5),
[raise6secviolation] decimal(15,5),
[lower5minviolation] decimal(15,5),
[lowerregviolation] decimal(15,5),
[lower60secviolation] decimal(15,5),
[lower6secviolation] decimal(15,5),
[totalintermittentgeneration] decimal(15,5),
[demand_and_nonschedgen] decimal(15,5),
[uigf] decimal(15,5)
) d
end
go
create or alter procedure InsertSettlementsSetFcasRegulationTrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'SetFcasRegulationTrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'SetFcasRegulationTrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'SetFcasRegulationTrk', 1);

insert into SettlementsSetFcasRegulationTrk1(
file_log_id,
[settlementdate],
[versionno],
[interval_datetime],
[constraintid],
[cmpf],
[crmpf],
[recovery_factor_cmpf],
[recovery_factor_crmpf],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[interval_datetime],
d.[constraintid],
d.[cmpf],
d.[crmpf],
d.[recovery_factor_cmpf],
d.[recovery_factor_crmpf],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[interval_datetime] datetime2,
[constraintid] varchar(20),
[cmpf] decimal(18,8),
[crmpf] decimal(18,8),
[recovery_factor_cmpf] decimal(18,8),
[recovery_factor_crmpf] decimal(18,8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementsIraucsurplus6
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Iraucsurplus'
    and version = '6'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Iraucsurplus'
    and version = '6'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Iraucsurplus', 6);

insert into SettlementsIraucsurplus6(
file_log_id,
[settlementdate],
[settlementrunno],
[contractid],
[periodid],
[participantid],
[interconnectorid],
[fromregionid],
[totalsurplus],
[contractallocation],
[surplusvalue],
[lastchanged],
[csp_derogation_amount],
[unadjusted_irsr]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[settlementrunno],
d.[contractid],
d.[periodid],
d.[participantid],
d.[interconnectorid],
d.[fromregionid],
d.[totalsurplus],
d.[contractallocation],
d.[surplusvalue],
d.[lastchanged],
d.[csp_derogation_amount],
d.[unadjusted_irsr]
from openjson(@data) with (
[settlementdate] datetime2,
[settlementrunno] decimal(3,0),
[contractid] varchar(10),
[periodid] decimal(2,0),
[participantid] varchar(10),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[totalsurplus] decimal(15,5),
[contractallocation] decimal(8,5),
[surplusvalue] decimal(15,5),
[lastchanged] datetime2,
[csp_derogation_amount] decimal(18,8),
[unadjusted_irsr] decimal(18,8)
) d
end
go
create or alter procedure InsertSettlementsRpowerrecovery5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Rpowerrecovery'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Settlements'
    and sub_type = 'Rpowerrecovery'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Settlements', 'Rpowerrecovery', 5);

insert into SettlementsRpowerrecovery5(
file_log_id,
[settlementdate],
[versionno],
[participantid],
[contractid],
[periodid],
[regionid],
[availabilitypayment],
[enablingpayment],
[ccpayment],
[participantdemand],
[regiondemand],
[availabilityrecovery],
[enablingrecovery],
[ccrecovery],
[lastchanged],
[availabilityrecovery_gen],
[enablingrecovery_gen],
[ccrecovery_gen],
[participantdemand_gen],
[regiondemand_gen]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[versionno],
d.[participantid],
d.[contractid],
d.[periodid],
d.[regionid],
d.[availabilitypayment],
d.[enablingpayment],
d.[ccpayment],
d.[participantdemand],
d.[regiondemand],
d.[availabilityrecovery],
d.[enablingrecovery],
d.[ccrecovery],
d.[lastchanged],
d.[availabilityrecovery_gen],
d.[enablingrecovery_gen],
d.[ccrecovery_gen],
d.[participantdemand_gen],
d.[regiondemand_gen]
from openjson(@data) with (
[settlementdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[contractid] varchar(10),
[periodid] decimal(3,0),
[regionid] varchar(10),
[availabilitypayment] decimal(15,5),
[enablingpayment] decimal(15,5),
[ccpayment] decimal(15,5),
[participantdemand] decimal(15,5),
[regiondemand] decimal(15,5),
[availabilityrecovery] decimal(15,5),
[enablingrecovery] decimal(15,5),
[ccrecovery] decimal(15,5),
[lastchanged] datetime2,
[availabilityrecovery_gen] decimal(15,5),
[enablingrecovery_gen] decimal(15,5),
[ccrecovery_gen] decimal(15,5),
[participantdemand_gen] decimal(15,5),
[regiondemand_gen] decimal(15,5)
) d
end
go
create or alter procedure InsertIrauctionConfigAuctionRpEstimate1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionConfig'
    and sub_type = 'AuctionRpEstimate'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionConfig'
    and sub_type = 'AuctionRpEstimate'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'IrauctionConfig', 'AuctionRpEstimate', 1);

insert into IrauctionConfigAuctionRpEstimate1(
file_log_id,
[contractyear],
[quarter],
[valuationid],
[versionno],
[interconnectorid],
[fromregionid],
[rpestimate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[quarter],
d.[valuationid],
d.[versionno],
d.[interconnectorid],
d.[fromregionid],
d.[rpestimate],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[quarter] decimal(1,0),
[valuationid] varchar(15),
[versionno] decimal(3,0),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[rpestimate] decimal(17,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionSraFinancialAucpaySum1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraFinancialAucpaySum'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraFinancialAucpaySum'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'SraFinancialAucpaySum', 1);

insert into IrauctionSraFinancialAucpaySum1(
file_log_id,
[sra_year],
[sra_quarter],
[sra_runno],
[participantid],
[gross_proceeds_amount],
[total_gross_proceeds_amount],
[shortfall_amount],
[total_shortfall_amount],
[net_payment_amount],
[lastchanged]
)
select 
(select h.id from @header h),
d.[sra_year],
d.[sra_quarter],
d.[sra_runno],
d.[participantid],
d.[gross_proceeds_amount],
d.[total_gross_proceeds_amount],
d.[shortfall_amount],
d.[total_shortfall_amount],
d.[net_payment_amount],
d.[lastchanged]
from openjson(@data) with (
[sra_year] decimal(4,0),
[sra_quarter] decimal(3,0),
[sra_runno] decimal(3,0),
[participantid] varchar(10),
[gross_proceeds_amount] decimal(18,8),
[total_gross_proceeds_amount] decimal(18,8),
[shortfall_amount] decimal(18,8),
[total_shortfall_amount] decimal(18,8),
[net_payment_amount] decimal(18,8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionBidsFundsBid1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionBids'
    and sub_type = 'FundsBid'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionBids'
    and sub_type = 'FundsBid'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'IrauctionBids', 'FundsBid', 1);

insert into IrauctionBidsFundsBid1(
file_log_id,
[contractid],
[participantid],
[loaddate],
[optionid],
[interconnectorid],
[fromregionid],
[units],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractid],
d.[participantid],
d.[loaddate],
d.[optionid],
d.[interconnectorid],
d.[fromregionid],
d.[units],
d.[lastchanged]
from openjson(@data) with (
[contractid] varchar(30),
[participantid] varchar(10),
[loaddate] datetime2,
[optionid] decimal(3,0),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[units] decimal(5,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionResiduePublicData1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'ResiduePublicData'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'ResiduePublicData'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'ResiduePublicData', 1);

insert into IrauctionResiduePublicData1(
file_log_id,
[contractid],
[versionno],
[interconnectorid],
[fromregionid],
[unitsoffered],
[unitssold],
[clearingprice],
[reserveprice],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractid],
d.[versionno],
d.[interconnectorid],
d.[fromregionid],
d.[unitsoffered],
d.[unitssold],
d.[clearingprice],
d.[reserveprice],
d.[lastchanged]
from openjson(@data) with (
[contractid] varchar(30),
[versionno] decimal(3,0),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[unitsoffered] decimal(5,0),
[unitssold] decimal(16,6),
[clearingprice] decimal(17,5),
[reserveprice] decimal(17,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionSraFinancialAucReceipts1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraFinancialAucReceipts'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraFinancialAucReceipts'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'SraFinancialAucReceipts', 1);

insert into IrauctionSraFinancialAucReceipts1(
file_log_id,
[sra_year],
[sra_quarter],
[sra_runno],
[participantid],
[interconnectorid],
[fromregionid],
[contractid],
[units_purchased],
[clearing_price],
[receipt_amount],
[lastchanged],
[proceeds_amount],
[units_sold]
)
select 
(select h.id from @header h),
d.[sra_year],
d.[sra_quarter],
d.[sra_runno],
d.[participantid],
d.[interconnectorid],
d.[fromregionid],
d.[contractid],
d.[units_purchased],
d.[clearing_price],
d.[receipt_amount],
d.[lastchanged],
d.[proceeds_amount],
d.[units_sold]
from openjson(@data) with (
[sra_year] decimal(4,0),
[sra_quarter] decimal(3,0),
[sra_runno] decimal(3,0),
[participantid] varchar(10),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[contractid] varchar(10),
[units_purchased] decimal(18,8),
[clearing_price] decimal(18,8),
[receipt_amount] decimal(18,8),
[lastchanged] datetime2,
[proceeds_amount] decimal(18,8),
[units_sold] decimal(18,8)
) d
end
go
create or alter procedure InsertIrauctionConfigAuctionTranche1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionConfig'
    and sub_type = 'AuctionTranche'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionConfig'
    and sub_type = 'AuctionTranche'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'IrauctionConfig', 'AuctionTranche', 1);

insert into IrauctionConfigAuctionTranche1(
file_log_id,
[contractyear],
[quarter],
[versionno],
[tranche],
[auctiondate],
[notifydate],
[unitallocation],
[changedate],
[changedby],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[quarter],
d.[versionno],
d.[tranche],
d.[auctiondate],
d.[notifydate],
d.[unitallocation],
d.[changedate],
d.[changedby],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[quarter] decimal(1,0),
[versionno] decimal(3,0),
[tranche] decimal(2,0),
[auctiondate] datetime2,
[notifydate] datetime2,
[unitallocation] decimal(18,8),
[changedate] datetime2,
[changedby] varchar(15),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionSraFinancialAucMargin1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraFinancialAucMargin'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraFinancialAucMargin'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'SraFinancialAucMargin', 1);

insert into IrauctionSraFinancialAucMargin1(
file_log_id,
[sra_year],
[sra_quarter],
[sra_runno],
[participantid],
[total_cash_security],
[required_margin],
[returned_margin],
[returned_margin_interest]
)
select 
(select h.id from @header h),
d.[sra_year],
d.[sra_quarter],
d.[sra_runno],
d.[participantid],
d.[total_cash_security],
d.[required_margin],
d.[returned_margin],
d.[returned_margin_interest]
from openjson(@data) with (
[sra_year] decimal(4,0),
[sra_quarter] decimal(3,0),
[sra_runno] decimal(3,0),
[participantid] varchar(10),
[total_cash_security] decimal(18,8),
[required_margin] decimal(18,8),
[returned_margin] decimal(18,8),
[returned_margin_interest] decimal(18,8)
) d
end
go
create or alter procedure InsertSettlementConfigResiduecontractpayments1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'Residuecontractpayments'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'Residuecontractpayments'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'SettlementConfig', 'Residuecontractpayments', 1);

insert into SettlementConfigResiduecontractpayments1(
file_log_id,
[contractid],
[participantid],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractid],
d.[participantid],
d.[lastchanged]
from openjson(@data) with (
[contractid] varchar(30),
[participantid] varchar(10),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionResidueContracts1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'ResidueContracts'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'ResidueContracts'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'ResidueContracts', 1);

insert into IrauctionResidueContracts1(
file_log_id,
[contractyear],
[quarter],
[tranche],
[contractid],
[startdate],
[enddate],
[notifydate],
[auctiondate],
[calcmethod],
[authoriseddate],
[authorisedby],
[notifypostdate],
[notifyby],
[postdate],
[postedby],
[lastchanged],
[description],
[auctionid]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[quarter],
d.[tranche],
d.[contractid],
d.[startdate],
d.[enddate],
d.[notifydate],
d.[auctiondate],
d.[calcmethod],
d.[authoriseddate],
d.[authorisedby],
d.[notifypostdate],
d.[notifyby],
d.[postdate],
d.[postedby],
d.[lastchanged],
d.[description],
d.[auctionid]
from openjson(@data) with (
[contractyear] decimal(4,0),
[quarter] decimal(1,0),
[tranche] decimal(2,0),
[contractid] varchar(30),
[startdate] datetime2,
[enddate] datetime2,
[notifydate] datetime2,
[auctiondate] datetime2,
[calcmethod] varchar(20),
[authoriseddate] datetime2,
[authorisedby] varchar(15),
[notifypostdate] datetime2,
[notifyby] varchar(15),
[postdate] datetime2,
[postedby] varchar(15),
[lastchanged] datetime2,
[description] varchar(80),
[auctionid] varchar(30)
) d
end
go
create or alter procedure InsertIrauctionValuationid1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'Valuationid'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'Valuationid'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'Valuationid', 1);

insert into IrauctionValuationid1(
file_log_id,
[valuationid],
[description],
[lastchanged]
)
select 
(select h.id from @header h),
d.[valuationid],
d.[description],
d.[lastchanged]
from openjson(@data) with (
[valuationid] varchar(15),
[description] varchar(80),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionConfigAuctionRevenueTrack1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionConfig'
    and sub_type = 'AuctionRevenueTrack'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionConfig'
    and sub_type = 'AuctionRevenueTrack'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'IrauctionConfig', 'AuctionRevenueTrack', 1);

insert into IrauctionConfigAuctionRevenueTrack1(
file_log_id,
[contractyear],
[quarter],
[valuationid],
[versionno],
[effectivedate],
[status],
[documentref],
[authoriseddate],
[authorisedby],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[quarter],
d.[valuationid],
d.[versionno],
d.[effectivedate],
d.[status],
d.[documentref],
d.[authoriseddate],
d.[authorisedby],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[quarter] decimal(1,0),
[valuationid] varchar(15),
[versionno] decimal(3,0),
[effectivedate] datetime2,
[status] varchar(10),
[documentref] varchar(30),
[authoriseddate] datetime2,
[authorisedby] varchar(15),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionResidueConData2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'ResidueConData'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'ResidueConData'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'ResidueConData', 2);

insert into IrauctionResidueConData2(
file_log_id,
[contractid],
[versionno],
[participantid],
[interconnectorid],
[fromregionid],
[unitspurchased],
[linkpayment],
[lastchanged],
[secondary_units_sold]
)
select 
(select h.id from @header h),
d.[contractid],
d.[versionno],
d.[participantid],
d.[interconnectorid],
d.[fromregionid],
d.[unitspurchased],
d.[linkpayment],
d.[lastchanged],
d.[secondary_units_sold]
from openjson(@data) with (
[contractid] varchar(30),
[versionno] decimal(3,0),
[participantid] varchar(10),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[unitspurchased] decimal(17,5),
[linkpayment] decimal(17,5),
[lastchanged] datetime2,
[secondary_units_sold] decimal(18,8)
) d
end
go
create or alter procedure InsertIrauctionBidsFileTrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionBids'
    and sub_type = 'FileTrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionBids'
    and sub_type = 'FileTrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'IrauctionBids', 'FileTrk', 1);

insert into IrauctionBidsFileTrk1(
file_log_id,
[contractid],
[participantid],
[loaddate],
[filename],
[ackfilename],
[status],
[lastchanged],
[auctionid]
)
select 
(select h.id from @header h),
d.[contractid],
d.[participantid],
d.[loaddate],
d.[filename],
d.[ackfilename],
d.[status],
d.[lastchanged],
d.[auctionid]
from openjson(@data) with (
[contractid] varchar(30),
[participantid] varchar(10),
[loaddate] datetime2,
[filename] varchar(40),
[ackfilename] varchar(40),
[status] varchar(10),
[lastchanged] datetime2,
[auctionid] varchar(30)
) d
end
go
create or alter procedure InsertIrauctionSraFinancialAucpayDetail1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraFinancialAucpayDetail'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraFinancialAucpayDetail'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'SraFinancialAucpayDetail', 1);

insert into IrauctionSraFinancialAucpayDetail1(
file_log_id,
[sra_year],
[sra_quarter],
[sra_runno],
[participantid],
[interconnectorid],
[fromregionid],
[contractid],
[maximum_units],
[units_sold],
[shortfall_units],
[reserve_price],
[clearing_price],
[payment_amount],
[shortfall_amount],
[allocation],
[net_payment_amount],
[lastchanged]
)
select 
(select h.id from @header h),
d.[sra_year],
d.[sra_quarter],
d.[sra_runno],
d.[participantid],
d.[interconnectorid],
d.[fromregionid],
d.[contractid],
d.[maximum_units],
d.[units_sold],
d.[shortfall_units],
d.[reserve_price],
d.[clearing_price],
d.[payment_amount],
d.[shortfall_amount],
d.[allocation],
d.[net_payment_amount],
d.[lastchanged]
from openjson(@data) with (
[sra_year] decimal(4,0),
[sra_quarter] decimal(3,0),
[sra_runno] decimal(3,0),
[participantid] varchar(10),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[contractid] varchar(10),
[maximum_units] decimal(18,8),
[units_sold] decimal(18,8),
[shortfall_units] decimal(18,8),
[reserve_price] decimal(18,8),
[clearing_price] decimal(18,8),
[payment_amount] decimal(18,8),
[shortfall_amount] decimal(18,8),
[allocation] decimal(18,8),
[net_payment_amount] decimal(18,8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionConfigAuctionRevenueEstimate1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionConfig'
    and sub_type = 'AuctionRevenueEstimate'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionConfig'
    and sub_type = 'AuctionRevenueEstimate'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'IrauctionConfig', 'AuctionRevenueEstimate', 1);

insert into IrauctionConfigAuctionRevenueEstimate1(
file_log_id,
[contractyear],
[quarter],
[valuationid],
[versionno],
[interconnectorid],
[fromregionid],
[monthno],
[startdate],
[enddate],
[revenue],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[quarter],
d.[valuationid],
d.[versionno],
d.[interconnectorid],
d.[fromregionid],
d.[monthno],
d.[startdate],
d.[enddate],
d.[revenue],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[quarter] decimal(1,0),
[valuationid] varchar(15),
[versionno] decimal(3,0),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[monthno] decimal(1,0),
[startdate] datetime2,
[enddate] datetime2,
[revenue] decimal(17,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionSraFinancialAucMardetail1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraFinancialAucMardetail'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraFinancialAucMardetail'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'SraFinancialAucMardetail', 1);

insert into IrauctionSraFinancialAucMardetail1(
file_log_id,
[sra_year],
[sra_quarter],
[sra_runno],
[participantid],
[cash_security_id],
[returned_amount],
[returned_interest]
)
select 
(select h.id from @header h),
d.[sra_year],
d.[sra_quarter],
d.[sra_runno],
d.[participantid],
d.[cash_security_id],
d.[returned_amount],
d.[returned_interest]
from openjson(@data) with (
[sra_year] decimal(4,0),
[sra_quarter] decimal(3,0),
[sra_runno] decimal(3,0),
[participantid] varchar(10),
[cash_security_id] varchar(36),
[returned_amount] decimal(18,8),
[returned_interest] decimal(18,8)
) d
end
go
create or alter procedure InsertIrauctionSraPrudentialExposure1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraPrudentialExposure'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraPrudentialExposure'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'SraPrudentialExposure', 1);

insert into IrauctionSraPrudentialExposure1(
file_log_id,
[prudential_date],
[prudential_runno],
[participantid],
[sra_year],
[sra_quarter],
[interconnectorid],
[fromregionid],
[max_tranche],
[auctionid],
[offer_submissiontime],
[average_purchase_price],
[average_cancellation_price],
[cancellation_volume],
[trading_position]
)
select 
(select h.id from @header h),
d.[prudential_date],
d.[prudential_runno],
d.[participantid],
d.[sra_year],
d.[sra_quarter],
d.[interconnectorid],
d.[fromregionid],
d.[max_tranche],
d.[auctionid],
d.[offer_submissiontime],
d.[average_purchase_price],
d.[average_cancellation_price],
d.[cancellation_volume],
d.[trading_position]
from openjson(@data) with (
[prudential_date] datetime2,
[prudential_runno] decimal(8,0),
[participantid] varchar(10),
[sra_year] decimal(4,0),
[sra_quarter] decimal(3,0),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[max_tranche] decimal(2,0),
[auctionid] varchar(30),
[offer_submissiontime] datetime2,
[average_purchase_price] decimal(18,8),
[average_cancellation_price] decimal(18,8),
[cancellation_volume] decimal(18,8),
[trading_position] decimal(18,8)
) d
end
go
create or alter procedure InsertIrauctionSraCashSecurity1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraCashSecurity'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraCashSecurity'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'SraCashSecurity', 1);

insert into IrauctionSraCashSecurity1(
file_log_id,
[cash_security_id],
[participantid],
[provision_date],
[cash_amount],
[interest_acct_id],
[authoriseddate],
[finalreturndate],
[cash_security_returned],
[deletiondate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[cash_security_id],
d.[participantid],
d.[provision_date],
d.[cash_amount],
d.[interest_acct_id],
d.[authoriseddate],
d.[finalreturndate],
d.[cash_security_returned],
d.[deletiondate],
d.[lastchanged]
from openjson(@data) with (
[cash_security_id] varchar(36),
[participantid] varchar(10),
[provision_date] datetime2,
[cash_amount] decimal(18,8),
[interest_acct_id] varchar(20),
[authoriseddate] datetime2,
[finalreturndate] datetime2,
[cash_security_returned] decimal(18,8),
[deletiondate] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionSraPrudentialRun1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraPrudentialRun'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraPrudentialRun'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'SraPrudentialRun', 1);

insert into IrauctionSraPrudentialRun1(
file_log_id,
[prudential_date],
[prudential_runno]
)
select 
(select h.id from @header h),
d.[prudential_date],
d.[prudential_runno]
from openjson(@data) with (
[prudential_date] datetime2,
[prudential_runno] decimal(8,0)
) d
end
go
create or alter procedure InsertIrauctionConfigAuction1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionConfig'
    and sub_type = 'Auction'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionConfig'
    and sub_type = 'Auction'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'IrauctionConfig', 'Auction', 1);

insert into IrauctionConfigAuction1(
file_log_id,
[auctionid],
[auctiondate],
[notifydate],
[startdate],
[enddate],
[description],
[authoriseddate],
[authorisedby],
[lastchanged]
)
select 
(select h.id from @header h),
d.[auctionid],
d.[auctiondate],
d.[notifydate],
d.[startdate],
d.[enddate],
d.[description],
d.[authoriseddate],
d.[authorisedby],
d.[lastchanged]
from openjson(@data) with (
[auctionid] varchar(30),
[auctiondate] datetime2,
[notifydate] datetime2,
[startdate] datetime2,
[enddate] datetime2,
[description] varchar(100),
[authoriseddate] datetime2,
[authorisedby] varchar(30),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionSraPrudentialCompPosition1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraPrudentialCompPosition'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraPrudentialCompPosition'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'SraPrudentialCompPosition', 1);

insert into IrauctionSraPrudentialCompPosition1(
file_log_id,
[prudential_date],
[prudential_runno],
[participantid],
[trading_limit],
[prudential_exposure_amount],
[trading_margin]
)
select 
(select h.id from @header h),
d.[prudential_date],
d.[prudential_runno],
d.[participantid],
d.[trading_limit],
d.[prudential_exposure_amount],
d.[trading_margin]
from openjson(@data) with (
[prudential_date] datetime2,
[prudential_runno] decimal(8,0),
[participantid] varchar(10),
[trading_limit] decimal(18,8),
[prudential_exposure_amount] decimal(18,8),
[trading_margin] decimal(18,8)
) d
end
go
create or alter procedure InsertIrauctionConfigAuctionIcAllocations2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionConfig'
    and sub_type = 'AuctionIcAllocations'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionConfig'
    and sub_type = 'AuctionIcAllocations'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'IrauctionConfig', 'AuctionIcAllocations', 2);

insert into IrauctionConfigAuctionIcAllocations2(
file_log_id,
[contractyear],
[quarter],
[versionno],
[interconnectorid],
[fromregionid],
[maximumunits],
[proportion],
[auctionfee],
[changedate],
[changedby],
[lastchanged],
[auctionfee_sales]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[quarter],
d.[versionno],
d.[interconnectorid],
d.[fromregionid],
d.[maximumunits],
d.[proportion],
d.[auctionfee],
d.[changedate],
d.[changedby],
d.[lastchanged],
d.[auctionfee_sales]
from openjson(@data) with (
[contractyear] decimal(4,0),
[quarter] decimal(1,0),
[versionno] decimal(3,0),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[maximumunits] decimal(5,0),
[proportion] decimal(8,5),
[auctionfee] decimal(17,5),
[changedate] datetime2,
[changedby] varchar(15),
[lastchanged] datetime2,
[auctionfee_sales] decimal(18,8)
) d
end
go
create or alter procedure InsertIrauctionResidueBidTrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'ResidueBidTrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'ResidueBidTrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'ResidueBidTrk', 1);

insert into IrauctionResidueBidTrk1(
file_log_id,
[contractid],
[versionno],
[participantid],
[bidloaddate],
[lastchanged],
[auctionid]
)
select 
(select h.id from @header h),
d.[contractid],
d.[versionno],
d.[participantid],
d.[bidloaddate],
d.[lastchanged],
d.[auctionid]
from openjson(@data) with (
[contractid] varchar(30),
[versionno] decimal(3,0),
[participantid] varchar(10),
[bidloaddate] datetime2,
[lastchanged] datetime2,
[auctionid] varchar(30)
) d
end
go
create or alter procedure InsertIrauctionResidueTrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'ResidueTrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'ResidueTrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'ResidueTrk', 1);

insert into IrauctionResidueTrk1(
file_log_id,
[contractid],
[versionno],
[rundate],
[authoriseddate],
[authorisedby],
[postdate],
[postedby],
[lastchanged],
[status],
[auctionid]
)
select 
(select h.id from @header h),
d.[contractid],
d.[versionno],
d.[rundate],
d.[authoriseddate],
d.[authorisedby],
d.[postdate],
d.[postedby],
d.[lastchanged],
d.[status],
d.[auctionid]
from openjson(@data) with (
[contractid] varchar(30),
[versionno] decimal(3,0),
[rundate] datetime2,
[authoriseddate] datetime2,
[authorisedby] varchar(15),
[postdate] datetime2,
[postedby] varchar(15),
[lastchanged] datetime2,
[status] varchar(15),
[auctionid] varchar(30)
) d
end
go
create or alter procedure InsertIrauctionResidueConFunds1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'ResidueConFunds'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'ResidueConFunds'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'ResidueConFunds', 1);

insert into IrauctionResidueConFunds1(
file_log_id,
[contractid],
[interconnectorid],
[fromregionid],
[defaultunits],
[rolloverunits],
[reallocatedunits],
[unitsoffered],
[meanreserveprice],
[scalefactor],
[actualreserveprice],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractid],
d.[interconnectorid],
d.[fromregionid],
d.[defaultunits],
d.[rolloverunits],
d.[reallocatedunits],
d.[unitsoffered],
d.[meanreserveprice],
d.[scalefactor],
d.[actualreserveprice],
d.[lastchanged]
from openjson(@data) with (
[contractid] varchar(30),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[defaultunits] decimal(5,0),
[rolloverunits] decimal(5,0),
[reallocatedunits] decimal(5,0),
[unitsoffered] decimal(5,0),
[meanreserveprice] decimal(9,2),
[scalefactor] decimal(8,5),
[actualreserveprice] decimal(9,2),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionResidueConEstimatesTrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'ResidueConEstimatesTrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'ResidueConEstimatesTrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'ResidueConEstimatesTrk', 1);

insert into IrauctionResidueConEstimatesTrk1(
file_log_id,
[contractid],
[contractyear],
[quarter],
[valuationid],
[versionno],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractid],
d.[contractyear],
d.[quarter],
d.[valuationid],
d.[versionno],
d.[lastchanged]
from openjson(@data) with (
[contractid] varchar(30),
[contractyear] decimal(4,0),
[quarter] decimal(1,0),
[valuationid] varchar(15),
[versionno] decimal(3,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionResiduePriceFundsBid1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'ResiduePriceFundsBid'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'ResiduePriceFundsBid'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'ResiduePriceFundsBid', 1);

insert into IrauctionResiduePriceFundsBid1(
file_log_id,
[contractid],
[interconnectorid],
[fromregionid],
[units],
[bidprice],
[linkedbidflag],
[auctionid],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractid],
d.[interconnectorid],
d.[fromregionid],
d.[units],
d.[bidprice],
d.[linkedbidflag],
d.[auctionid],
d.[lastchanged]
from openjson(@data) with (
[contractid] varchar(30),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[units] decimal(5,0),
[bidprice] decimal(17,5),
[linkedbidflag] decimal(6,0),
[auctionid] varchar(30),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionSraOfferProduct1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraOfferProduct'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraOfferProduct'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'SraOfferProduct', 1);

insert into IrauctionSraOfferProduct1(
file_log_id,
[auctionid],
[participantid],
[loaddate],
[optionid],
[interconnectorid],
[fromregionid],
[offer_quantity],
[offer_price],
[trancheid],
[lastchanged]
)
select 
(select h.id from @header h),
d.[auctionid],
d.[participantid],
d.[loaddate],
d.[optionid],
d.[interconnectorid],
d.[fromregionid],
d.[offer_quantity],
d.[offer_price],
d.[trancheid],
d.[lastchanged]
from openjson(@data) with (
[auctionid] varchar(30),
[participantid] varchar(10),
[loaddate] datetime2,
[optionid] decimal(4,0),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[offer_quantity] decimal(5,0),
[offer_price] decimal(18,8),
[trancheid] varchar(30),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionConfigAuctionCalendar2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionConfig'
    and sub_type = 'AuctionCalendar'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionConfig'
    and sub_type = 'AuctionCalendar'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'IrauctionConfig', 'AuctionCalendar', 2);

insert into IrauctionConfigAuctionCalendar2(
file_log_id,
[contractyear],
[quarter],
[startdate],
[enddate],
[notifydate],
[paymentdate],
[reconciliationdate],
[lastchanged],
[prelimpurchasestmtdate],
[prelimproceedsstmtdate],
[finalpurchasestmtdate],
[finalproceedsstmtdate]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[quarter],
d.[startdate],
d.[enddate],
d.[notifydate],
d.[paymentdate],
d.[reconciliationdate],
d.[lastchanged],
d.[prelimpurchasestmtdate],
d.[prelimproceedsstmtdate],
d.[finalpurchasestmtdate],
d.[finalproceedsstmtdate]
from openjson(@data) with (
[contractyear] decimal(4,0),
[quarter] decimal(1,0),
[startdate] datetime2,
[enddate] datetime2,
[notifydate] datetime2,
[paymentdate] datetime2,
[reconciliationdate] datetime2,
[lastchanged] datetime2,
[prelimpurchasestmtdate] datetime2,
[prelimproceedsstmtdate] datetime2,
[finalpurchasestmtdate] datetime2,
[finalproceedsstmtdate] datetime2
) d
end
go
create or alter procedure InsertIrauctionSraPrudentialCashSecurity1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraPrudentialCashSecurity'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraPrudentialCashSecurity'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'SraPrudentialCashSecurity', 1);

insert into IrauctionSraPrudentialCashSecurity1(
file_log_id,
[prudential_date],
[prudential_runno],
[participantid],
[cash_security_id],
[cash_security_amount]
)
select 
(select h.id from @header h),
d.[prudential_date],
d.[prudential_runno],
d.[participantid],
d.[cash_security_id],
d.[cash_security_amount]
from openjson(@data) with (
[prudential_date] datetime2,
[prudential_runno] decimal(8,0),
[participantid] varchar(10),
[cash_security_id] varchar(36),
[cash_security_amount] decimal(18,8)
) d
end
go
create or alter procedure InsertIrauctionSraOfferProfile1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraOfferProfile'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraOfferProfile'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'SraOfferProfile', 1);

insert into IrauctionSraOfferProfile1(
file_log_id,
[auctionid],
[participantid],
[loaddate],
[filename],
[ackfilename],
[transactionid],
[lastchanged]
)
select 
(select h.id from @header h),
d.[auctionid],
d.[participantid],
d.[loaddate],
d.[filename],
d.[ackfilename],
d.[transactionid],
d.[lastchanged]
from openjson(@data) with (
[auctionid] varchar(30),
[participantid] varchar(10),
[loaddate] datetime2,
[filename] varchar(40),
[ackfilename] varchar(40),
[transactionid] varchar(100),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertIrauctionBidsPriceBid1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionBids'
    and sub_type = 'PriceBid'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'IrauctionBids'
    and sub_type = 'PriceBid'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'IrauctionBids', 'PriceBid', 1);

insert into IrauctionBidsPriceBid1(
file_log_id,
[contractid],
[participantid],
[loaddate],
[optionid],
[bidprice],
[lastchanged],
[auctionid]
)
select 
(select h.id from @header h),
d.[contractid],
d.[participantid],
d.[loaddate],
d.[optionid],
d.[bidprice],
d.[lastchanged],
d.[auctionid]
from openjson(@data) with (
[contractid] varchar(30),
[participantid] varchar(10),
[loaddate] datetime2,
[optionid] decimal(3,0),
[bidprice] decimal(17,5),
[lastchanged] datetime2,
[auctionid] varchar(30)
) d
end
go
create or alter procedure InsertIrauctionSraFinancialRuntrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraFinancialRuntrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Irauction'
    and sub_type = 'SraFinancialRuntrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Irauction', 'SraFinancialRuntrk', 1);

insert into IrauctionSraFinancialRuntrk1(
file_log_id,
[sra_year],
[sra_quarter],
[sra_runno],
[runtype],
[rundate],
[posteddate],
[interest_versionno],
[makeup_versionno],
[lastchanged]
)
select 
(select h.id from @header h),
d.[sra_year],
d.[sra_quarter],
d.[sra_runno],
d.[runtype],
d.[rundate],
d.[posteddate],
d.[interest_versionno],
d.[makeup_versionno],
d.[lastchanged]
from openjson(@data) with (
[sra_year] decimal(4,0),
[sra_quarter] decimal(3,0),
[sra_runno] decimal(3,0),
[runtype] varchar(20),
[rundate] datetime2,
[posteddate] datetime2,
[interest_versionno] decimal(3,0),
[makeup_versionno] decimal(3,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertOfferBidofferfiletrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Offer'
    and sub_type = 'Bidofferfiletrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Offer'
    and sub_type = 'Bidofferfiletrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Offer', 'Bidofferfiletrk', 1);

insert into OfferBidofferfiletrk1(
file_log_id,
[participantid],
[offerdate],
[filename],
[status],
[lastchanged],
[authorisedby],
[authoriseddate]
)
select 
(select h.id from @header h),
d.[participantid],
d.[offerdate],
d.[filename],
d.[status],
d.[lastchanged],
d.[authorisedby],
d.[authoriseddate]
from openjson(@data) with (
[participantid] varchar(10),
[offerdate] datetime2,
[filename] varchar(80),
[status] varchar(10),
[lastchanged] datetime2,
[authorisedby] varchar(20),
[authoriseddate] datetime2
) d
end
go
create or alter procedure InsertBidMnspFiletrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Bid'
    and sub_type = 'MnspFiletrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Bid'
    and sub_type = 'MnspFiletrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Bid', 'MnspFiletrk', 1);

insert into BidMnspFiletrk1(
file_log_id,
[settlementdate],
[offerdate],
[participantid],
[filename],
[status],
[ackfilename],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[offerdate],
d.[participantid],
d.[filename],
d.[status],
d.[ackfilename],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[offerdate] datetime2,
[participantid] varchar(10),
[filename] varchar(40),
[status] varchar(10),
[ackfilename] varchar(40),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBidBidperofferD2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Bid'
    and sub_type = 'BidperofferD'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Bid'
    and sub_type = 'BidperofferD'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Bid', 'BidperofferD', 2);

insert into BidBidperofferD2(
file_log_id,
[settlementdate],
[duid],
[bidtype],
[bidsettlementdate],
[offerdate],
[periodid],
[versionno],
[maxavail],
[fixedload],
[rocup],
[rocdown],
[enablementmin],
[enablementmax],
[lowbreakpoint],
[highbreakpoint],
[bandavail1],
[bandavail2],
[bandavail3],
[bandavail4],
[bandavail5],
[bandavail6],
[bandavail7],
[bandavail8],
[bandavail9],
[bandavail10],
[lastchanged],
[pasaavailability],
[interval_datetime],
[mr_capacity]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[duid],
d.[bidtype],
d.[bidsettlementdate],
d.[offerdate],
d.[periodid],
d.[versionno],
d.[maxavail],
d.[fixedload],
d.[rocup],
d.[rocdown],
d.[enablementmin],
d.[enablementmax],
d.[lowbreakpoint],
d.[highbreakpoint],
d.[bandavail1],
d.[bandavail2],
d.[bandavail3],
d.[bandavail4],
d.[bandavail5],
d.[bandavail6],
d.[bandavail7],
d.[bandavail8],
d.[bandavail9],
d.[bandavail10],
d.[lastchanged],
d.[pasaavailability],
d.[interval_datetime],
d.[mr_capacity]
from openjson(@data) with (
[settlementdate] datetime2,
[duid] varchar(10),
[bidtype] varchar(10),
[bidsettlementdate] datetime2,
[offerdate] datetime2,
[periodid] decimal(22,0),
[versionno] decimal(22,0),
[maxavail] decimal(12,6),
[fixedload] decimal(12,6),
[rocup] decimal(6,0),
[rocdown] decimal(6,0),
[enablementmin] decimal(6,0),
[enablementmax] decimal(6,0),
[lowbreakpoint] decimal(6,0),
[highbreakpoint] decimal(6,0),
[bandavail1] decimal(22,0),
[bandavail2] decimal(22,0),
[bandavail3] decimal(22,0),
[bandavail4] decimal(22,0),
[bandavail5] decimal(22,0),
[bandavail6] decimal(22,0),
[bandavail7] decimal(22,0),
[bandavail8] decimal(22,0),
[bandavail9] decimal(22,0),
[bandavail10] decimal(22,0),
[lastchanged] datetime2,
[pasaavailability] decimal(12,0),
[interval_datetime] datetime2,
[mr_capacity] decimal(6,0)
) d
end
go
create or alter procedure InsertOfferMnspOffertrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Offer'
    and sub_type = 'MnspOffertrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Offer'
    and sub_type = 'MnspOffertrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Offer', 'MnspOffertrk', 1);

insert into OfferMnspOffertrk1(
file_log_id,
[settlementdate],
[offerdate],
[versionno],
[participantid],
[filename],
[authoriseddate],
[authorisedby],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[offerdate],
d.[versionno],
d.[participantid],
d.[filename],
d.[authoriseddate],
d.[authorisedby],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[offerdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[filename] varchar(40),
[authoriseddate] datetime2,
[authorisedby] varchar(15),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBidBiddayofferD2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Bid'
    and sub_type = 'BiddayofferD'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Bid'
    and sub_type = 'BiddayofferD'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Bid', 'BiddayofferD', 2);

insert into BidBiddayofferD2(
file_log_id,
[settlementdate],
[duid],
[bidtype],
[bidsettlementdate],
[offerdate],
[versionno],
[participantid],
[dailyenergyconstraint],
[rebidexplanation],
[priceband1],
[priceband2],
[priceband3],
[priceband4],
[priceband5],
[priceband6],
[priceband7],
[priceband8],
[priceband9],
[priceband10],
[minimumload],
[t1],
[t2],
[t3],
[t4],
[normalstatus],
[lastchanged],
[mr_factor],
[entrytype]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[duid],
d.[bidtype],
d.[bidsettlementdate],
d.[offerdate],
d.[versionno],
d.[participantid],
d.[dailyenergyconstraint],
d.[rebidexplanation],
d.[priceband1],
d.[priceband2],
d.[priceband3],
d.[priceband4],
d.[priceband5],
d.[priceband6],
d.[priceband7],
d.[priceband8],
d.[priceband9],
d.[priceband10],
d.[minimumload],
d.[t1],
d.[t2],
d.[t3],
d.[t4],
d.[normalstatus],
d.[lastchanged],
d.[mr_factor],
d.[entrytype]
from openjson(@data) with (
[settlementdate] datetime2,
[duid] varchar(10),
[bidtype] varchar(10),
[bidsettlementdate] datetime2,
[offerdate] datetime2,
[versionno] decimal(22,0),
[participantid] varchar(10),
[dailyenergyconstraint] decimal(12,6),
[rebidexplanation] varchar(500),
[priceband1] decimal(9,2),
[priceband2] decimal(9,2),
[priceband3] decimal(9,2),
[priceband4] decimal(9,2),
[priceband5] decimal(9,2),
[priceband6] decimal(9,2),
[priceband7] decimal(9,2),
[priceband8] decimal(9,2),
[priceband9] decimal(9,2),
[priceband10] decimal(9,2),
[minimumload] decimal(22,0),
[t1] decimal(22,0),
[t2] decimal(22,0),
[t3] decimal(22,0),
[t4] decimal(22,0),
[normalstatus] varchar(3),
[lastchanged] datetime2,
[mr_factor] decimal(16,6),
[entrytype] varchar(20)
) d
end
go
create or alter procedure InsertOfferMnspDayoffer2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Offer'
    and sub_type = 'MnspDayoffer'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Offer'
    and sub_type = 'MnspDayoffer'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Offer', 'MnspDayoffer', 2);

insert into OfferMnspDayoffer2(
file_log_id,
[settlementdate],
[offerdate],
[versionno],
[participantid],
[linkid],
[entrytype],
[rebidexplanation],
[priceband1],
[priceband2],
[priceband3],
[priceband4],
[priceband5],
[priceband6],
[priceband7],
[priceband8],
[priceband9],
[priceband10],
[lastchanged],
[mr_factor]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[offerdate],
d.[versionno],
d.[participantid],
d.[linkid],
d.[entrytype],
d.[rebidexplanation],
d.[priceband1],
d.[priceband2],
d.[priceband3],
d.[priceband4],
d.[priceband5],
d.[priceband6],
d.[priceband7],
d.[priceband8],
d.[priceband9],
d.[priceband10],
d.[lastchanged],
d.[mr_factor]
from openjson(@data) with (
[settlementdate] datetime2,
[offerdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[linkid] varchar(10),
[entrytype] varchar(20),
[rebidexplanation] varchar(500),
[priceband1] decimal(9,2),
[priceband2] decimal(9,2),
[priceband3] decimal(9,2),
[priceband4] decimal(9,2),
[priceband5] decimal(9,2),
[priceband6] decimal(9,2),
[priceband7] decimal(9,2),
[priceband8] decimal(9,2),
[priceband9] decimal(9,2),
[priceband10] decimal(9,2),
[lastchanged] datetime2,
[mr_factor] decimal(16,6)
) d
end
go
create or alter procedure InsertOfferMnspPeroffer1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Offer'
    and sub_type = 'MnspPeroffer'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Offer'
    and sub_type = 'MnspPeroffer'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Offer', 'MnspPeroffer', 1);

insert into OfferMnspPeroffer1(
file_log_id,
[settlementdate],
[offerdate],
[versionno],
[participantid],
[linkid],
[periodid],
[maxavail],
[bandavail1],
[bandavail2],
[bandavail3],
[bandavail4],
[bandavail5],
[bandavail6],
[bandavail7],
[bandavail8],
[bandavail9],
[bandavail10],
[lastchanged],
[fixedload],
[rampuprate],
[pasaavailability],
[mr_capacity]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[offerdate],
d.[versionno],
d.[participantid],
d.[linkid],
d.[periodid],
d.[maxavail],
d.[bandavail1],
d.[bandavail2],
d.[bandavail3],
d.[bandavail4],
d.[bandavail5],
d.[bandavail6],
d.[bandavail7],
d.[bandavail8],
d.[bandavail9],
d.[bandavail10],
d.[lastchanged],
d.[fixedload],
d.[rampuprate],
d.[pasaavailability],
d.[mr_capacity]
from openjson(@data) with (
[settlementdate] datetime2,
[offerdate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[linkid] varchar(10),
[periodid] decimal(22,0),
[maxavail] decimal(6,0),
[bandavail1] decimal(6,0),
[bandavail2] decimal(6,0),
[bandavail3] decimal(6,0),
[bandavail4] decimal(6,0),
[bandavail5] decimal(6,0),
[bandavail6] decimal(6,0),
[bandavail7] decimal(6,0),
[bandavail8] decimal(6,0),
[bandavail9] decimal(6,0),
[bandavail10] decimal(6,0),
[lastchanged] datetime2,
[fixedload] decimal(12,6),
[rampuprate] decimal(6,0),
[pasaavailability] decimal(12,0),
[mr_capacity] decimal(6,0)
) d
end
go
create or alter procedure InsertOfferBidperoffer1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Offer'
    and sub_type = 'Bidperoffer'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Offer'
    and sub_type = 'Bidperoffer'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Offer', 'Bidperoffer', 1);

insert into OfferBidperoffer1(
file_log_id,
[duid],
[bidtype],
[settlementdate],
[offerdate],
[periodid],
[versionno],
[maxavail],
[fixedload],
[rocup],
[rocdown],
[enablementmin],
[enablementmax],
[lowbreakpoint],
[highbreakpoint],
[bandavail1],
[bandavail2],
[bandavail3],
[bandavail4],
[bandavail5],
[bandavail6],
[bandavail7],
[bandavail8],
[bandavail9],
[bandavail10],
[lastchanged],
[pasaavailability],
[mr_capacity]
)
select 
(select h.id from @header h),
d.[duid],
d.[bidtype],
d.[settlementdate],
d.[offerdate],
d.[periodid],
d.[versionno],
d.[maxavail],
d.[fixedload],
d.[rocup],
d.[rocdown],
d.[enablementmin],
d.[enablementmax],
d.[lowbreakpoint],
d.[highbreakpoint],
d.[bandavail1],
d.[bandavail2],
d.[bandavail3],
d.[bandavail4],
d.[bandavail5],
d.[bandavail6],
d.[bandavail7],
d.[bandavail8],
d.[bandavail9],
d.[bandavail10],
d.[lastchanged],
d.[pasaavailability],
d.[mr_capacity]
from openjson(@data) with (
[duid] varchar(10),
[bidtype] varchar(10),
[settlementdate] datetime2,
[offerdate] datetime2,
[periodid] decimal(22,0),
[versionno] decimal(22,0),
[maxavail] decimal(12,6),
[fixedload] decimal(12,6),
[rocup] decimal(6,0),
[rocdown] decimal(6,0),
[enablementmin] decimal(6,0),
[enablementmax] decimal(6,0),
[lowbreakpoint] decimal(6,0),
[highbreakpoint] decimal(6,0),
[bandavail1] decimal(22,0),
[bandavail2] decimal(22,0),
[bandavail3] decimal(22,0),
[bandavail4] decimal(22,0),
[bandavail5] decimal(22,0),
[bandavail6] decimal(22,0),
[bandavail7] decimal(22,0),
[bandavail8] decimal(22,0),
[bandavail9] decimal(22,0),
[bandavail10] decimal(22,0),
[lastchanged] datetime2,
[pasaavailability] decimal(12,0),
[mr_capacity] decimal(6,0)
) d
end
go
create or alter procedure InsertOfferMtpasaOfferfiletrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Offer'
    and sub_type = 'MtpasaOfferfiletrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Offer'
    and sub_type = 'MtpasaOfferfiletrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Offer', 'MtpasaOfferfiletrk', 1);

insert into OfferMtpasaOfferfiletrk1(
file_log_id,
[participantid],
[offerdatetime],
[filename]
)
select 
(select h.id from @header h),
d.[participantid],
d.[offerdatetime],
d.[filename]
from openjson(@data) with (
[participantid] varchar(20),
[offerdatetime] datetime2,
[filename] varchar(200)
) d
end
go
create or alter procedure InsertOfferBiddayoffer2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Offer'
    and sub_type = 'Biddayoffer'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Offer'
    and sub_type = 'Biddayoffer'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Offer', 'Biddayoffer', 2);

insert into OfferBiddayoffer2(
file_log_id,
[duid],
[bidtype],
[settlementdate],
[offerdate],
[versionno],
[participantid],
[dailyenergyconstraint],
[rebidexplanation],
[priceband1],
[priceband2],
[priceband3],
[priceband4],
[priceband5],
[priceband6],
[priceband7],
[priceband8],
[priceband9],
[priceband10],
[minimumload],
[t1],
[t2],
[t3],
[t4],
[normalstatus],
[lastchanged],
[mr_factor],
[entrytype]
)
select 
(select h.id from @header h),
d.[duid],
d.[bidtype],
d.[settlementdate],
d.[offerdate],
d.[versionno],
d.[participantid],
d.[dailyenergyconstraint],
d.[rebidexplanation],
d.[priceband1],
d.[priceband2],
d.[priceband3],
d.[priceband4],
d.[priceband5],
d.[priceband6],
d.[priceband7],
d.[priceband8],
d.[priceband9],
d.[priceband10],
d.[minimumload],
d.[t1],
d.[t2],
d.[t3],
d.[t4],
d.[normalstatus],
d.[lastchanged],
d.[mr_factor],
d.[entrytype]
from openjson(@data) with (
[duid] varchar(10),
[bidtype] varchar(10),
[settlementdate] datetime2,
[offerdate] datetime2,
[versionno] decimal(22,0),
[participantid] varchar(10),
[dailyenergyconstraint] decimal(12,6),
[rebidexplanation] varchar(500),
[priceband1] decimal(9,2),
[priceband2] decimal(9,2),
[priceband3] decimal(9,2),
[priceband4] decimal(9,2),
[priceband5] decimal(9,2),
[priceband6] decimal(9,2),
[priceband7] decimal(9,2),
[priceband8] decimal(9,2),
[priceband9] decimal(9,2),
[priceband10] decimal(9,2),
[minimumload] decimal(22,0),
[t1] decimal(22,0),
[t2] decimal(22,0),
[t3] decimal(22,0),
[t4] decimal(22,0),
[normalstatus] varchar(3),
[lastchanged] datetime2,
[mr_factor] decimal(16,6),
[entrytype] varchar(20)
) d
end
go
create or alter procedure InsertOfferMtpasaOfferdata1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Offer'
    and sub_type = 'MtpasaOfferdata'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Offer'
    and sub_type = 'MtpasaOfferdata'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Offer', 'MtpasaOfferdata', 1);

insert into OfferMtpasaOfferdata1(
file_log_id,
[participantid],
[offerdatetime],
[unitid],
[effectivedate],
[energy],
[capacity1],
[capacity2],
[capacity3],
[capacity4],
[capacity5],
[capacity6],
[capacity7],
[lastchanged]
)
select 
(select h.id from @header h),
d.[participantid],
d.[offerdatetime],
d.[unitid],
d.[effectivedate],
d.[energy],
d.[capacity1],
d.[capacity2],
d.[capacity3],
d.[capacity4],
d.[capacity5],
d.[capacity6],
d.[capacity7],
d.[lastchanged]
from openjson(@data) with (
[participantid] varchar(20),
[offerdatetime] datetime2,
[unitid] varchar(20),
[effectivedate] datetime2,
[energy] decimal(9,0),
[capacity1] decimal(9,0),
[capacity2] decimal(9,0),
[capacity3] decimal(9,0),
[capacity4] decimal(9,0),
[capacity5] decimal(9,0),
[capacity6] decimal(9,0),
[capacity7] decimal(9,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMeterdataIndividualReads1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Meterdata'
    and sub_type = 'IndividualReads'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Meterdata'
    and sub_type = 'IndividualReads'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Meterdata', 'IndividualReads', 1);

insert into MeterdataIndividualReads1(
file_log_id,
[case_id],
[settlementdate],
[meter_id],
[meter_id_suffix],
[frmp],
[lr],
[periodid],
[connectionpointid],
[meter_type],
[importvalue],
[exportvalue],
[lastchanged]
)
select 
(select h.id from @header h),
d.[case_id],
d.[settlementdate],
d.[meter_id],
d.[meter_id_suffix],
d.[frmp],
d.[lr],
d.[periodid],
d.[connectionpointid],
d.[meter_type],
d.[importvalue],
d.[exportvalue],
d.[lastchanged]
from openjson(@data) with (
[case_id] decimal(15,0),
[settlementdate] datetime2,
[meter_id] varchar(20),
[meter_id_suffix] varchar(20),
[frmp] varchar(20),
[lr] varchar(20),
[periodid] decimal(3,0),
[connectionpointid] varchar(20),
[meter_type] varchar(20),
[importvalue] decimal(18,8),
[exportvalue] decimal(18,8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMeterdataTrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Meterdata'
    and sub_type = 'Trk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Meterdata'
    and sub_type = 'Trk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Meterdata', 'Trk', 1);

insert into MeterdataTrk1(
file_log_id,
[case_id],
[aggregate_reads_load_datetime],
[individual_reads_load_datetime],
[startdate],
[enddate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[case_id],
d.[aggregate_reads_load_datetime],
d.[individual_reads_load_datetime],
d.[startdate],
d.[enddate],
d.[lastchanged]
from openjson(@data) with (
[case_id] decimal(15,0),
[aggregate_reads_load_datetime] datetime2,
[individual_reads_load_datetime] datetime2,
[startdate] datetime2,
[enddate] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMeterdataAggregateReads1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Meterdata'
    and sub_type = 'AggregateReads'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Meterdata'
    and sub_type = 'AggregateReads'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Meterdata', 'AggregateReads', 1);

insert into MeterdataAggregateReads1(
file_log_id,
[case_id],
[settlementdate],
[connectionpointid],
[meter_type],
[frmp],
[lr],
[periodid],
[importvalue],
[exportvalue],
[lastchanged]
)
select 
(select h.id from @header h),
d.[case_id],
d.[settlementdate],
d.[connectionpointid],
d.[meter_type],
d.[frmp],
d.[lr],
d.[periodid],
d.[importvalue],
d.[exportvalue],
d.[lastchanged]
from openjson(@data) with (
[case_id] decimal(15,0),
[settlementdate] datetime2,
[connectionpointid] varchar(20),
[meter_type] varchar(20),
[frmp] varchar(20),
[lr] varchar(20),
[periodid] decimal(3,0),
[importvalue] decimal(18,8),
[exportvalue] decimal(18,8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMeterdataInterconnector1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Meterdata'
    and sub_type = 'Interconnector'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Meterdata'
    and sub_type = 'Interconnector'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Meterdata', 'Interconnector', 1);

insert into MeterdataInterconnector1(
file_log_id,
[case_id],
[settlementdate],
[interconnectorid],
[periodid],
[importvalue],
[exportvalue],
[lastchanged]
)
select 
(select h.id from @header h),
d.[case_id],
d.[settlementdate],
d.[interconnectorid],
d.[periodid],
d.[importvalue],
d.[exportvalue],
d.[lastchanged]
from openjson(@data) with (
[case_id] decimal(15,0),
[settlementdate] datetime2,
[interconnectorid] varchar(20),
[periodid] decimal(3,0),
[importvalue] decimal(18,8),
[exportvalue] decimal(18,8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertPrudentialRuntrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Prudential'
    and sub_type = 'Runtrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Prudential'
    and sub_type = 'Runtrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Prudential', 'Runtrk', 1);

insert into PrudentialRuntrk1(
file_log_id,
[prudential_date],
[runno],
[authorisedby],
[authoriseddate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[prudential_date],
d.[runno],
d.[authorisedby],
d.[authoriseddate],
d.[lastchanged]
from openjson(@data) with (
[prudential_date] datetime2,
[runno] decimal(3,0),
[authorisedby] varchar(15),
[authoriseddate] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertPrudentialCompanyPosition1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Prudential'
    and sub_type = 'CompanyPosition'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Prudential'
    and sub_type = 'CompanyPosition'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Prudential', 'CompanyPosition', 1);

insert into PrudentialCompanyPosition1(
file_log_id,
[prudential_date],
[runno],
[company_id],
[mcl],
[credit_support],
[trading_limit],
[current_amount_balance],
[security_deposit_provision],
[security_deposit_offset],
[security_deposit_balance],
[expost_realloc_balance],
[default_balance],
[outstandings],
[trading_margin],
[typical_accrual],
[prudential_margin],
[early_payment_amount],
[percentage_outstandings],
[lastchanged]
)
select 
(select h.id from @header h),
d.[prudential_date],
d.[runno],
d.[company_id],
d.[mcl],
d.[credit_support],
d.[trading_limit],
d.[current_amount_balance],
d.[security_deposit_provision],
d.[security_deposit_offset],
d.[security_deposit_balance],
d.[expost_realloc_balance],
d.[default_balance],
d.[outstandings],
d.[trading_margin],
d.[typical_accrual],
d.[prudential_margin],
d.[early_payment_amount],
d.[percentage_outstandings],
d.[lastchanged]
from openjson(@data) with (
[prudential_date] datetime2,
[runno] decimal(3,0),
[company_id] varchar(20),
[mcl] decimal(16,6),
[credit_support] decimal(16,6),
[trading_limit] decimal(16,6),
[current_amount_balance] decimal(16,6),
[security_deposit_provision] decimal(16,6),
[security_deposit_offset] decimal(16,6),
[security_deposit_balance] decimal(16,6),
[expost_realloc_balance] decimal(16,6),
[default_balance] decimal(16,6),
[outstandings] decimal(16,6),
[trading_margin] decimal(16,6),
[typical_accrual] decimal(16,6),
[prudential_margin] decimal(16,6),
[early_payment_amount] decimal(18,8),
[percentage_outstandings] decimal(18,8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertTradingInterconnectorres2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Trading'
    and sub_type = 'Interconnectorres'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Trading'
    and sub_type = 'Interconnectorres'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Trading', 'Interconnectorres', 2);

insert into TradingInterconnectorres2(
file_log_id,
[settlementdate],
[runno],
[interconnectorid],
[periodid],
[meteredmwflow],
[mwflow],
[mwlosses],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[interconnectorid],
d.[periodid],
d.[meteredmwflow],
d.[mwflow],
d.[mwlosses],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[interconnectorid] varchar(10),
[periodid] decimal(3,0),
[meteredmwflow] decimal(15,5),
[mwflow] decimal(15,5),
[mwlosses] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertTradingRegionsum4
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Trading'
    and sub_type = 'Regionsum'
    and version = '4'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Trading'
    and sub_type = 'Regionsum'
    and version = '4'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Trading', 'Regionsum', 4);

insert into TradingRegionsum4(
file_log_id,
[settlementdate],
[runno],
[regionid],
[periodid],
[totaldemand],
[availablegeneration],
[availableload],
[demandforecast],
[dispatchablegeneration],
[dispatchableload],
[netinterchange],
[excessgeneration],
[lower5mindispatch],
[lower5minimport],
[lower5minlocaldispatch],
[lower5minlocalprice],
[lower5minlocalreq],
[lower5minprice],
[lower5minreq],
[lower5minsupplyprice],
[lower60secdispatch],
[lower60secimport],
[lower60seclocaldispatch],
[lower60seclocalprice],
[lower60seclocalreq],
[lower60secprice],
[lower60secreq],
[lower60secsupplyprice],
[lower6secdispatch],
[lower6secimport],
[lower6seclocaldispatch],
[lower6seclocalprice],
[lower6seclocalreq],
[lower6secprice],
[lower6secreq],
[lower6secsupplyprice],
[raise5mindispatch],
[raise5minimport],
[raise5minlocaldispatch],
[raise5minlocalprice],
[raise5minlocalreq],
[raise5minprice],
[raise5minreq],
[raise5minsupplyprice],
[raise60secdispatch],
[raise60secimport],
[raise60seclocaldispatch],
[raise60seclocalprice],
[raise60seclocalreq],
[raise60secprice],
[raise60secreq],
[raise60secsupplyprice],
[raise6secdispatch],
[raise6secimport],
[raise6seclocaldispatch],
[raise6seclocalprice],
[raise6seclocalreq],
[raise6secprice],
[raise6secreq],
[raise6secsupplyprice],
[lastchanged],
[initialsupply],
[clearedsupply],
[lowerregimport],
[lowerreglocaldispatch],
[lowerreglocalreq],
[lowerregreq],
[raiseregimport],
[raisereglocaldispatch],
[raisereglocalreq],
[raiseregreq],
[raise5minlocalviolation],
[raisereglocalviolation],
[raise60seclocalviolation],
[raise6seclocalviolation],
[lower5minlocalviolation],
[lowerreglocalviolation],
[lower60seclocalviolation],
[lower6seclocalviolation],
[raise5minviolation],
[raiseregviolation],
[raise60secviolation],
[raise6secviolation],
[lower5minviolation],
[lowerregviolation],
[lower60secviolation],
[lower6secviolation],
[totalintermittentgeneration],
[demand_and_nonschedgen],
[uigf]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[regionid],
d.[periodid],
d.[totaldemand],
d.[availablegeneration],
d.[availableload],
d.[demandforecast],
d.[dispatchablegeneration],
d.[dispatchableload],
d.[netinterchange],
d.[excessgeneration],
d.[lower5mindispatch],
d.[lower5minimport],
d.[lower5minlocaldispatch],
d.[lower5minlocalprice],
d.[lower5minlocalreq],
d.[lower5minprice],
d.[lower5minreq],
d.[lower5minsupplyprice],
d.[lower60secdispatch],
d.[lower60secimport],
d.[lower60seclocaldispatch],
d.[lower60seclocalprice],
d.[lower60seclocalreq],
d.[lower60secprice],
d.[lower60secreq],
d.[lower60secsupplyprice],
d.[lower6secdispatch],
d.[lower6secimport],
d.[lower6seclocaldispatch],
d.[lower6seclocalprice],
d.[lower6seclocalreq],
d.[lower6secprice],
d.[lower6secreq],
d.[lower6secsupplyprice],
d.[raise5mindispatch],
d.[raise5minimport],
d.[raise5minlocaldispatch],
d.[raise5minlocalprice],
d.[raise5minlocalreq],
d.[raise5minprice],
d.[raise5minreq],
d.[raise5minsupplyprice],
d.[raise60secdispatch],
d.[raise60secimport],
d.[raise60seclocaldispatch],
d.[raise60seclocalprice],
d.[raise60seclocalreq],
d.[raise60secprice],
d.[raise60secreq],
d.[raise60secsupplyprice],
d.[raise6secdispatch],
d.[raise6secimport],
d.[raise6seclocaldispatch],
d.[raise6seclocalprice],
d.[raise6seclocalreq],
d.[raise6secprice],
d.[raise6secreq],
d.[raise6secsupplyprice],
d.[lastchanged],
d.[initialsupply],
d.[clearedsupply],
d.[lowerregimport],
d.[lowerreglocaldispatch],
d.[lowerreglocalreq],
d.[lowerregreq],
d.[raiseregimport],
d.[raisereglocaldispatch],
d.[raisereglocalreq],
d.[raiseregreq],
d.[raise5minlocalviolation],
d.[raisereglocalviolation],
d.[raise60seclocalviolation],
d.[raise6seclocalviolation],
d.[lower5minlocalviolation],
d.[lowerreglocalviolation],
d.[lower60seclocalviolation],
d.[lower6seclocalviolation],
d.[raise5minviolation],
d.[raiseregviolation],
d.[raise60secviolation],
d.[raise6secviolation],
d.[lower5minviolation],
d.[lowerregviolation],
d.[lower60secviolation],
d.[lower6secviolation],
d.[totalintermittentgeneration],
d.[demand_and_nonschedgen],
d.[uigf]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[regionid] varchar(10),
[periodid] decimal(3,0),
[totaldemand] decimal(15,5),
[availablegeneration] decimal(15,5),
[availableload] decimal(15,5),
[demandforecast] decimal(15,5),
[dispatchablegeneration] decimal(15,5),
[dispatchableload] decimal(15,5),
[netinterchange] decimal(15,5),
[excessgeneration] decimal(15,5),
[lower5mindispatch] decimal(15,5),
[lower5minimport] decimal(15,5),
[lower5minlocaldispatch] decimal(15,5),
[lower5minlocalprice] decimal(15,5),
[lower5minlocalreq] decimal(15,5),
[lower5minprice] decimal(15,5),
[lower5minreq] decimal(15,5),
[lower5minsupplyprice] decimal(15,5),
[lower60secdispatch] decimal(15,5),
[lower60secimport] decimal(15,5),
[lower60seclocaldispatch] decimal(15,5),
[lower60seclocalprice] decimal(15,5),
[lower60seclocalreq] decimal(15,5),
[lower60secprice] decimal(15,5),
[lower60secreq] decimal(15,5),
[lower60secsupplyprice] decimal(15,5),
[lower6secdispatch] decimal(15,5),
[lower6secimport] decimal(15,5),
[lower6seclocaldispatch] decimal(15,5),
[lower6seclocalprice] decimal(15,5),
[lower6seclocalreq] decimal(15,5),
[lower6secprice] decimal(15,5),
[lower6secreq] decimal(15,5),
[lower6secsupplyprice] decimal(15,5),
[raise5mindispatch] decimal(15,5),
[raise5minimport] decimal(15,5),
[raise5minlocaldispatch] decimal(15,5),
[raise5minlocalprice] decimal(15,5),
[raise5minlocalreq] decimal(15,5),
[raise5minprice] decimal(15,5),
[raise5minreq] decimal(15,5),
[raise5minsupplyprice] decimal(15,5),
[raise60secdispatch] decimal(15,5),
[raise60secimport] decimal(15,5),
[raise60seclocaldispatch] decimal(15,5),
[raise60seclocalprice] decimal(15,5),
[raise60seclocalreq] decimal(15,5),
[raise60secprice] decimal(15,5),
[raise60secreq] decimal(15,5),
[raise60secsupplyprice] decimal(15,5),
[raise6secdispatch] decimal(15,5),
[raise6secimport] decimal(15,5),
[raise6seclocaldispatch] decimal(15,5),
[raise6seclocalprice] decimal(15,5),
[raise6seclocalreq] decimal(15,5),
[raise6secprice] decimal(15,5),
[raise6secreq] decimal(15,5),
[raise6secsupplyprice] decimal(15,5),
[lastchanged] datetime2,
[initialsupply] decimal(15,5),
[clearedsupply] decimal(15,5),
[lowerregimport] decimal(15,5),
[lowerreglocaldispatch] decimal(15,5),
[lowerreglocalreq] decimal(15,5),
[lowerregreq] decimal(15,5),
[raiseregimport] decimal(15,5),
[raisereglocaldispatch] decimal(15,5),
[raisereglocalreq] decimal(15,5),
[raiseregreq] decimal(15,5),
[raise5minlocalviolation] decimal(15,5),
[raisereglocalviolation] decimal(15,5),
[raise60seclocalviolation] decimal(15,5),
[raise6seclocalviolation] decimal(15,5),
[lower5minlocalviolation] decimal(15,5),
[lowerreglocalviolation] decimal(15,5),
[lower60seclocalviolation] decimal(15,5),
[lower6seclocalviolation] decimal(15,5),
[raise5minviolation] decimal(15,5),
[raiseregviolation] decimal(15,5),
[raise60secviolation] decimal(15,5),
[raise6secviolation] decimal(15,5),
[lower5minviolation] decimal(15,5),
[lowerregviolation] decimal(15,5),
[lower60secviolation] decimal(15,5),
[lower6secviolation] decimal(15,5),
[totalintermittentgeneration] decimal(15,5),
[demand_and_nonschedgen] decimal(15,5),
[uigf] decimal(15,5)
) d
end
go
create or alter procedure InsertTradingPrice2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Trading'
    and sub_type = 'Price'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Trading'
    and sub_type = 'Price'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Trading', 'Price', 2);

insert into TradingPrice2(
file_log_id,
[settlementdate],
[runno],
[regionid],
[periodid],
[rrp],
[eep],
[invalidflag],
[lastchanged],
[rop],
[raise6secrrp],
[raise6secrop],
[raise60secrrp],
[raise60secrop],
[raise5minrrp],
[raise5minrop],
[raiseregrrp],
[raiseregrop],
[lower6secrrp],
[lower6secrop],
[lower60secrrp],
[lower60secrop],
[lower5minrrp],
[lower5minrop],
[lowerregrrp],
[lowerregrop],
[price_status]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[regionid],
d.[periodid],
d.[rrp],
d.[eep],
d.[invalidflag],
d.[lastchanged],
d.[rop],
d.[raise6secrrp],
d.[raise6secrop],
d.[raise60secrrp],
d.[raise60secrop],
d.[raise5minrrp],
d.[raise5minrop],
d.[raiseregrrp],
d.[raiseregrop],
d.[lower6secrrp],
d.[lower6secrop],
d.[lower60secrrp],
d.[lower60secrop],
d.[lower5minrrp],
d.[lower5minrop],
d.[lowerregrrp],
d.[lowerregrop],
d.[price_status]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[regionid] varchar(10),
[periodid] decimal(3,0),
[rrp] decimal(15,5),
[eep] decimal(15,5),
[invalidflag] varchar(1),
[lastchanged] datetime2,
[rop] decimal(15,5),
[raise6secrrp] decimal(15,5),
[raise6secrop] decimal(15,5),
[raise60secrrp] decimal(15,5),
[raise60secrop] decimal(15,5),
[raise5minrrp] decimal(15,5),
[raise5minrop] decimal(15,5),
[raiseregrrp] decimal(15,5),
[raiseregrop] decimal(15,5),
[lower6secrrp] decimal(15,5),
[lower6secrop] decimal(15,5),
[lower60secrrp] decimal(15,5),
[lower60secrop] decimal(15,5),
[lower5minrrp] decimal(15,5),
[lower5minrop] decimal(15,5),
[lowerregrrp] decimal(15,5),
[lowerregrop] decimal(15,5),
[price_status] varchar(20)
) d
end
go
create or alter procedure InsertTradingUnitSolution2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Trading'
    and sub_type = 'UnitSolution'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Trading'
    and sub_type = 'UnitSolution'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Trading', 'UnitSolution', 2);

insert into TradingUnitSolution2(
file_log_id,
[settlementdate],
[runno],
[duid],
[tradetype],
[periodid],
[initialmw],
[totalcleared],
[rampdownrate],
[rampuprate],
[lower5min],
[lower60sec],
[lower6sec],
[raise5min],
[raise60sec],
[raise6sec],
[lastchanged],
[lowerreg],
[raisereg],
[availability],
[semidispatchcap]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[duid],
d.[tradetype],
d.[periodid],
d.[initialmw],
d.[totalcleared],
d.[rampdownrate],
d.[rampuprate],
d.[lower5min],
d.[lower60sec],
d.[lower6sec],
d.[raise5min],
d.[raise60sec],
d.[raise6sec],
d.[lastchanged],
d.[lowerreg],
d.[raisereg],
d.[availability],
d.[semidispatchcap]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[duid] varchar(10),
[tradetype] decimal(2,0),
[periodid] decimal(3,0),
[initialmw] decimal(15,5),
[totalcleared] decimal(15,5),
[rampdownrate] decimal(15,5),
[rampuprate] decimal(15,5),
[lower5min] decimal(15,5),
[lower60sec] decimal(15,5),
[lower6sec] decimal(15,5),
[raise5min] decimal(15,5),
[raise60sec] decimal(15,5),
[raise6sec] decimal(15,5),
[lastchanged] datetime2,
[lowerreg] decimal(15,5),
[raisereg] decimal(15,5),
[availability] decimal(15,5),
[semidispatchcap] decimal(3,0)
) d
end
go
create or alter procedure InsertStpasaRegionsolution5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Stpasa'
    and sub_type = 'Regionsolution'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Stpasa'
    and sub_type = 'Regionsolution'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Stpasa', 'Regionsolution', 5);

insert into StpasaRegionsolution5(
file_log_id,
[run_datetime],
[interval_datetime],
[regionid],
[demand10],
[demand50],
[demand90],
[reservereq],
[capacityreq],
[energyreqdemand50],
[unconstrainedcapacity],
[constrainedcapacity],
[netinterchangeunderscarcity],
[surpluscapacity],
[surplusreserve],
[reservecondition],
[maxsurplusreserve],
[maxsparecapacity],
[lorcondition],
[aggregatecapacityavailable],
[aggregatescheduledload],
[lastchanged],
[aggregatepasaavailability],
[runtype],
[energyreqdemand10],
[calculatedlor1level],
[calculatedlor2level],
[msrnetinterchangeunderscarcity],
[lornetinterchangeunderscarcity],
[totalintermittentgeneration],
[demand_and_nonschedgen],
[uigf],
[semi_scheduled_capacity],
[lor_semi_scheduled_capacity],
[lcr],
[lcr2],
[fum],
[ss_solar_uigf],
[ss_wind_uigf],
[ss_solar_capacity],
[ss_wind_capacity],
[ss_solar_cleared],
[ss_wind_cleared]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[interval_datetime],
d.[regionid],
d.[demand10],
d.[demand50],
d.[demand90],
d.[reservereq],
d.[capacityreq],
d.[energyreqdemand50],
d.[unconstrainedcapacity],
d.[constrainedcapacity],
d.[netinterchangeunderscarcity],
d.[surpluscapacity],
d.[surplusreserve],
d.[reservecondition],
d.[maxsurplusreserve],
d.[maxsparecapacity],
d.[lorcondition],
d.[aggregatecapacityavailable],
d.[aggregatescheduledload],
d.[lastchanged],
d.[aggregatepasaavailability],
d.[runtype],
d.[energyreqdemand10],
d.[calculatedlor1level],
d.[calculatedlor2level],
d.[msrnetinterchangeunderscarcity],
d.[lornetinterchangeunderscarcity],
d.[totalintermittentgeneration],
d.[demand_and_nonschedgen],
d.[uigf],
d.[semi_scheduled_capacity],
d.[lor_semi_scheduled_capacity],
d.[lcr],
d.[lcr2],
d.[fum],
d.[ss_solar_uigf],
d.[ss_wind_uigf],
d.[ss_solar_capacity],
d.[ss_wind_capacity],
d.[ss_solar_cleared],
d.[ss_wind_cleared]
from openjson(@data) with (
[run_datetime] datetime2,
[interval_datetime] datetime2,
[regionid] varchar(10),
[demand10] decimal(12,2),
[demand50] decimal(12,2),
[demand90] decimal(12,2),
[reservereq] decimal(12,2),
[capacityreq] decimal(12,2),
[energyreqdemand50] decimal(12,2),
[unconstrainedcapacity] decimal(12,0),
[constrainedcapacity] decimal(12,0),
[netinterchangeunderscarcity] decimal(12,2),
[surpluscapacity] decimal(12,2),
[surplusreserve] decimal(12,2),
[reservecondition] decimal(1,0),
[maxsurplusreserve] decimal(12,2),
[maxsparecapacity] decimal(12,2),
[lorcondition] decimal(1,0),
[aggregatecapacityavailable] decimal(12,2),
[aggregatescheduledload] decimal(12,2),
[lastchanged] datetime2,
[aggregatepasaavailability] decimal(12,0),
[runtype] varchar(20),
[energyreqdemand10] decimal(12,2),
[calculatedlor1level] decimal(16,6),
[calculatedlor2level] decimal(16,6),
[msrnetinterchangeunderscarcity] decimal(12,2),
[lornetinterchangeunderscarcity] decimal(12,2),
[totalintermittentgeneration] decimal(15,5),
[demand_and_nonschedgen] decimal(15,5),
[uigf] decimal(12,2),
[semi_scheduled_capacity] decimal(12,2),
[lor_semi_scheduled_capacity] decimal(12,2),
[lcr] decimal(16,6),
[lcr2] decimal(16,6),
[fum] decimal(16,6),
[ss_solar_uigf] decimal(12,2),
[ss_wind_uigf] decimal(12,2),
[ss_solar_capacity] decimal(12,2),
[ss_wind_capacity] decimal(12,2),
[ss_solar_cleared] decimal(12,2),
[ss_wind_cleared] decimal(12,2)
) d
end
go
create or alter procedure InsertStpasaInterconnectorsoln2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Stpasa'
    and sub_type = 'Interconnectorsoln'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Stpasa'
    and sub_type = 'Interconnectorsoln'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Stpasa', 'Interconnectorsoln', 2);

insert into StpasaInterconnectorsoln2(
file_log_id,
[run_datetime],
[interval_datetime],
[interconnectorid],
[capacitymwflow],
[capacitymarginalvalue],
[capacityviolationdegree],
[calculatedexportlimit],
[calculatedimportlimit],
[lastchanged],
[runtype],
[exportlimitconstraintid],
[importlimitconstraintid]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[interval_datetime],
d.[interconnectorid],
d.[capacitymwflow],
d.[capacitymarginalvalue],
d.[capacityviolationdegree],
d.[calculatedexportlimit],
d.[calculatedimportlimit],
d.[lastchanged],
d.[runtype],
d.[exportlimitconstraintid],
d.[importlimitconstraintid]
from openjson(@data) with (
[run_datetime] datetime2,
[interval_datetime] datetime2,
[interconnectorid] varchar(10),
[capacitymwflow] decimal(12,2),
[capacitymarginalvalue] decimal(12,2),
[capacityviolationdegree] decimal(12,2),
[calculatedexportlimit] decimal(12,2),
[calculatedimportlimit] decimal(12,2),
[lastchanged] datetime2,
[runtype] varchar(20),
[exportlimitconstraintid] varchar(20),
[importlimitconstraintid] varchar(20)
) d
end
go
create or alter procedure InsertStpasaCasesolution3
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Stpasa'
    and sub_type = 'Casesolution'
    and version = '3'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Stpasa'
    and sub_type = 'Casesolution'
    and version = '3'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Stpasa', 'Casesolution', 3);

insert into StpasaCasesolution3(
file_log_id,
[run_datetime],
[pasaversion],
[reservecondition],
[lorcondition],
[capacityobjfunction],
[capacityoption],
[maxsurplusreserveoption],
[maxsparecapacityoption],
[interconnectorflowpenalty],
[lastchanged],
[reliabilitylrcdemandoption],
[outagelrcdemandoption],
[lordemandoption],
[reliabilitylrccapacityoption],
[outagelrccapacityoption],
[lorcapacityoption],
[loruigf_option],
[reliability_lrcuigf_option],
[outage_lrcuigf_option]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[pasaversion],
d.[reservecondition],
d.[lorcondition],
d.[capacityobjfunction],
d.[capacityoption],
d.[maxsurplusreserveoption],
d.[maxsparecapacityoption],
d.[interconnectorflowpenalty],
d.[lastchanged],
d.[reliabilitylrcdemandoption],
d.[outagelrcdemandoption],
d.[lordemandoption],
d.[reliabilitylrccapacityoption],
d.[outagelrccapacityoption],
d.[lorcapacityoption],
d.[loruigf_option],
d.[reliability_lrcuigf_option],
d.[outage_lrcuigf_option]
from openjson(@data) with (
[run_datetime] datetime2,
[pasaversion] varchar(10),
[reservecondition] decimal(1,0),
[lorcondition] decimal(1,0),
[capacityobjfunction] decimal(12,3),
[capacityoption] decimal(12,3),
[maxsurplusreserveoption] decimal(12,3),
[maxsparecapacityoption] decimal(12,3),
[interconnectorflowpenalty] decimal(12,3),
[lastchanged] datetime2,
[reliabilitylrcdemandoption] decimal(12,3),
[outagelrcdemandoption] decimal(12,3),
[lordemandoption] decimal(12,3),
[reliabilitylrccapacityoption] varchar(10),
[outagelrccapacityoption] varchar(10),
[lorcapacityoption] varchar(10),
[loruigf_option] decimal(3,0),
[reliability_lrcuigf_option] decimal(3,0),
[outage_lrcuigf_option] decimal(3,0)
) d
end
go
create or alter procedure InsertStpasaConstraintsolution2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Stpasa'
    and sub_type = 'Constraintsolution'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Stpasa'
    and sub_type = 'Constraintsolution'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Stpasa', 'Constraintsolution', 2);

insert into StpasaConstraintsolution2(
file_log_id,
[run_datetime],
[interval_datetime],
[constraintid],
[capacityrhs],
[capacitymarginalvalue],
[capacityviolationdegree],
[lastchanged],
[runtype]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[interval_datetime],
d.[constraintid],
d.[capacityrhs],
d.[capacitymarginalvalue],
d.[capacityviolationdegree],
d.[lastchanged],
d.[runtype]
from openjson(@data) with (
[run_datetime] datetime2,
[interval_datetime] datetime2,
[constraintid] varchar(20),
[capacityrhs] decimal(12,2),
[capacitymarginalvalue] decimal(12,2),
[capacityviolationdegree] decimal(12,2),
[lastchanged] datetime2,
[runtype] varchar(20)
) d
end
go
create or alter procedure InsertBillingSmelterreduction5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Smelterreduction'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Smelterreduction'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Smelterreduction', 5);

insert into BillingSmelterreduction5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[participantid],
[rate1],
[ra1],
[rate2],
[ra2],
[te],
[pcsd],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[rate1],
d.[ra1],
d.[rate2],
d.[ra2],
d.[te],
d.[pcsd],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(22,0),
[weekno] decimal(22,0),
[billrunno] decimal(22,0),
[participantid] varchar(10),
[rate1] decimal(15,6),
[ra1] decimal(15,6),
[rate2] decimal(15,6),
[ra2] decimal(15,6),
[te] decimal(15,6),
[pcsd] decimal(15,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingIraucsurplus5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Iraucsurplus'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Iraucsurplus'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Iraucsurplus', 5);

insert into BillingIraucsurplus5(
file_log_id,
[contractyear],
[weekno],
[residueyear],
[quarter],
[billrunno],
[contractid],
[participantid],
[interconnectorid],
[fromregionid],
[totalresidues],
[adjustment],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[residueyear],
d.[quarter],
d.[billrunno],
d.[contractid],
d.[participantid],
d.[interconnectorid],
d.[fromregionid],
d.[totalresidues],
d.[adjustment],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(2,0),
[residueyear] decimal(4,0),
[quarter] decimal(2,0),
[billrunno] decimal(3,0),
[contractid] varchar(30),
[participantid] varchar(10),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[totalresidues] decimal(15,5),
[adjustment] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingApcRecovery2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'ApcRecovery'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'ApcRecovery'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'ApcRecovery', 2);

insert into BillingApcRecovery2(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[apeventid],
[claimid],
[participantid],
[regionid],
[recovery_amount],
[eligibility_start_interval],
[eligibility_end_interval],
[participant_demand],
[region_demand],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[apeventid],
d.[claimid],
d.[participantid],
d.[regionid],
d.[recovery_amount],
d.[eligibility_start_interval],
d.[eligibility_end_interval],
d.[participant_demand],
d.[region_demand],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[apeventid] decimal(6,0),
[claimid] decimal(6,0),
[participantid] varchar(20),
[regionid] varchar(20),
[recovery_amount] decimal(18,8),
[eligibility_start_interval] datetime2,
[eligibility_end_interval] datetime2,
[participant_demand] decimal(18,8),
[region_demand] decimal(18,8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingDaytrk5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Daytrk'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Daytrk'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Daytrk', 5);

insert into BillingDaytrk5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[settlementdate],
[runno],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[settlementdate],
d.[runno],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[settlementdate] datetime2,
[runno] decimal(3,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingIntraresidues5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Intraresidues'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Intraresidues'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Intraresidues', 5);

insert into BillingIntraresidues5(
file_log_id,
[allocation],
[totalsurplus],
[contractyear],
[weekno],
[billrunno],
[participantid],
[surplusvalue],
[lastchanged],
[regionid]
)
select 
(select h.id from @header h),
d.[allocation],
d.[totalsurplus],
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[surplusvalue],
d.[lastchanged],
d.[regionid]
from openjson(@data) with (
[allocation] decimal(6,3),
[totalsurplus] decimal(15,5),
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(10),
[surplusvalue] decimal(15,6),
[lastchanged] datetime2,
[regionid] varchar(10)
) d
end
go
create or alter procedure InsertBillingIrpartsurplus5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Irpartsurplus'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Irpartsurplus'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Irpartsurplus', 5);

insert into BillingIrpartsurplus5(
file_log_id,
[contractyear],
[weekno],
[residueyear],
[quarter],
[billrunno],
[contractid],
[participantid],
[interconnectorid],
[fromregionid],
[totalresidues],
[adjustment],
[lastchanged],
[actualpayment]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[residueyear],
d.[quarter],
d.[billrunno],
d.[contractid],
d.[participantid],
d.[interconnectorid],
d.[fromregionid],
d.[totalresidues],
d.[adjustment],
d.[lastchanged],
d.[actualpayment]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(2,0),
[residueyear] decimal(4,0),
[quarter] decimal(2,0),
[billrunno] decimal(3,0),
[contractid] varchar(30),
[participantid] varchar(10),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[totalresidues] decimal(15,5),
[adjustment] decimal(15,5),
[lastchanged] datetime2,
[actualpayment] decimal(15,5)
) d
end
go
create or alter procedure InsertBillingReallocDetail5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'ReallocDetail'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'ReallocDetail'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'ReallocDetail', 5);

insert into BillingReallocDetail5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[participantid],
[counterparty],
[reallocationid],
[value],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[counterparty],
d.[reallocationid],
d.[value],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(10),
[counterparty] varchar(10),
[reallocationid] varchar(20),
[value] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingEftshortfallAmount1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'EftshortfallAmount'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'EftshortfallAmount'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'EftshortfallAmount', 1);

insert into BillingEftshortfallAmount1(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[participantid],
[shortfall_amount],
[shortfall],
[shortfall_company_id],
[company_shortfall_amount],
[participant_net_energy],
[company_net_energy]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[shortfall_amount],
d.[shortfall],
d.[shortfall_company_id],
d.[company_shortfall_amount],
d.[participant_net_energy],
d.[company_net_energy]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(20),
[shortfall_amount] decimal(18,8),
[shortfall] decimal(18,8),
[shortfall_company_id] varchar(20),
[company_shortfall_amount] decimal(18,8),
[participant_net_energy] decimal(18,8),
[company_net_energy] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingIraucsurplussum7
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Iraucsurplussum'
    and version = '7'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Iraucsurplussum'
    and version = '7'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Iraucsurplussum', 7);

insert into BillingIraucsurplussum7(
file_log_id,
[contractyear],
[weekno],
[residueyear],
[quarter],
[billrunno],
[interconnectorid],
[fromregionid],
[participantid],
[totalsurplus],
[auctionfees],
[actualpayment],
[auctionfees_gst],
[lastchanged],
[csp_derogation_amount],
[unadjusted_irsr],
[negative_residues]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[residueyear],
d.[quarter],
d.[billrunno],
d.[interconnectorid],
d.[fromregionid],
d.[participantid],
d.[totalsurplus],
d.[auctionfees],
d.[actualpayment],
d.[auctionfees_gst],
d.[lastchanged],
d.[csp_derogation_amount],
d.[unadjusted_irsr],
d.[negative_residues]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[residueyear] decimal(4,0),
[quarter] decimal(2,0),
[billrunno] decimal(3,0),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[participantid] varchar(10),
[totalsurplus] decimal(15,5),
[auctionfees] decimal(15,5),
[actualpayment] decimal(15,5),
[auctionfees_gst] decimal(15,5),
[lastchanged] datetime2,
[csp_derogation_amount] decimal(18,8),
[unadjusted_irsr] decimal(18,8),
[negative_residues] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingSecdepInterestPay1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'SecdepInterestPay'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'SecdepInterestPay'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'SecdepInterestPay', 1);

insert into BillingSecdepInterestPay1(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[security_deposit_id],
[participantid],
[interest_amount],
[interest_calc_type],
[interest_acct_id],
[interest_rate]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[security_deposit_id],
d.[participantid],
d.[interest_amount],
d.[interest_calc_type],
d.[interest_acct_id],
d.[interest_rate]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[security_deposit_id] varchar(20),
[participantid] varchar(20),
[interest_amount] decimal(18,8),
[interest_calc_type] varchar(20),
[interest_acct_id] varchar(20),
[interest_rate] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingNmasTstRecvryTrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'NmasTstRecvryTrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'NmasTstRecvryTrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'NmasTstRecvryTrk', 1);

insert into BillingNmasTstRecvryTrk1(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[recovery_contractyear],
[recovery_weekno],
[recovery_billrunno]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[recovery_contractyear],
d.[recovery_weekno],
d.[recovery_billrunno]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[recovery_contractyear] decimal(4,0),
[recovery_weekno] decimal(3,0),
[recovery_billrunno] decimal(3,0)
) d
end
go
create or alter procedure InsertBillingIrfm5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Irfm'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Irfm'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Irfm', 5);

insert into BillingIrfm5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[participantid],
[irfmpayment],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[irfmpayment],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(10),
[irfmpayment] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingWhitehole5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Whitehole'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Whitehole'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Whitehole', 5);

insert into BillingWhitehole5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[participantid],
[nl],
[participantdemand],
[regiondemand],
[whiteholepayment],
[lastchanged],
[interconnectorid]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[nl],
d.[participantdemand],
d.[regiondemand],
d.[whiteholepayment],
d.[lastchanged],
d.[interconnectorid]
from openjson(@data) with (
[contractyear] decimal(22,0),
[weekno] decimal(22,0),
[billrunno] decimal(22,0),
[participantid] varchar(10),
[nl] decimal(15,6),
[participantdemand] decimal(15,6),
[regiondemand] decimal(15,6),
[whiteholepayment] decimal(15,6),
[lastchanged] datetime2,
[interconnectorid] varchar(10)
) d
end
go
create or alter procedure InsertBillingNmasTstRecovery1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'NmasTstRecovery'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'NmasTstRecovery'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'NmasTstRecovery', 1);

insert into BillingNmasTstRecovery1(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[participantid],
[service],
[contractid],
[regionid],
[rbf],
[test_payment],
[recovery_start_date],
[recovery_end_date],
[participant_energy],
[region_energy],
[nem_energy],
[customer_proportion],
[generator_proportion],
[participant_generation],
[nem_generation],
[recovery_amount],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[service],
d.[contractid],
d.[regionid],
d.[rbf],
d.[test_payment],
d.[recovery_start_date],
d.[recovery_end_date],
d.[participant_energy],
d.[region_energy],
d.[nem_energy],
d.[customer_proportion],
d.[generator_proportion],
d.[participant_generation],
d.[nem_generation],
d.[recovery_amount],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(20),
[service] varchar(10),
[contractid] varchar(10),
[regionid] varchar(10),
[rbf] decimal(18,8),
[test_payment] decimal(18,8),
[recovery_start_date] datetime2,
[recovery_end_date] datetime2,
[participant_energy] decimal(18,8),
[region_energy] decimal(18,8),
[nem_energy] decimal(18,8),
[customer_proportion] decimal(18,8),
[generator_proportion] decimal(18,8),
[participant_generation] decimal(18,8),
[nem_generation] decimal(18,8),
[recovery_amount] decimal(18,8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingIrnspsurplussum6
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Irnspsurplussum'
    and version = '6'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Irnspsurplussum'
    and version = '6'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Irnspsurplussum', 6);

insert into BillingIrnspsurplussum6(
file_log_id,
[contractyear],
[weekno],
[residueyear],
[quarter],
[billrunno],
[interconnectorid],
[fromregionid],
[participantid],
[totalsurplus],
[auctionfees],
[auctionfees_gst],
[lastchanged],
[csp_derogation_amount],
[unadjusted_irsr]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[residueyear],
d.[quarter],
d.[billrunno],
d.[interconnectorid],
d.[fromregionid],
d.[participantid],
d.[totalsurplus],
d.[auctionfees],
d.[auctionfees_gst],
d.[lastchanged],
d.[csp_derogation_amount],
d.[unadjusted_irsr]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[residueyear] decimal(4,0),
[quarter] decimal(2,0),
[billrunno] decimal(3,0),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[participantid] varchar(10),
[totalsurplus] decimal(15,5),
[auctionfees] decimal(15,5),
[auctionfees_gst] decimal(15,5),
[lastchanged] datetime2,
[csp_derogation_amount] decimal(18,8),
[unadjusted_irsr] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingRuntrk5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Runtrk'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Runtrk'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Runtrk', 5);

insert into BillingRuntrk5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[status],
[adj_cleared],
[authoriseddate],
[authorisedby],
[postdate],
[postby],
[lastchanged],
[receiptpostdate],
[receiptpostby],
[paymentpostdate],
[paymentpostby],
[shortfall],
[makeup]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[status],
d.[adj_cleared],
d.[authoriseddate],
d.[authorisedby],
d.[postdate],
d.[postby],
d.[lastchanged],
d.[receiptpostdate],
d.[receiptpostby],
d.[paymentpostdate],
d.[paymentpostby],
d.[shortfall],
d.[makeup]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[status] varchar(6),
[adj_cleared] varchar(1),
[authoriseddate] datetime2,
[authorisedby] varchar(10),
[postdate] datetime2,
[postby] varchar(10),
[lastchanged] datetime2,
[receiptpostdate] datetime2,
[receiptpostby] varchar(10),
[paymentpostdate] datetime2,
[paymentpostby] varchar(10),
[shortfall] decimal(16,6),
[makeup] decimal(15,5)
) d
end
go
create or alter procedure InsertBillingAspayments6
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Aspayments'
    and version = '6'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Aspayments'
    and version = '6'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Aspayments', 6);

insert into BillingAspayments6(
file_log_id,
[regionid],
[contractyear],
[weekno],
[billrunno],
[participantid],
[connectionpointid],
[raise6sec],
[lower6sec],
[raise60sec],
[lower60sec],
[agc],
[fcascomp],
[loadshed],
[rgul],
[rguu],
[reactivepower],
[systemrestart],
[lastchanged],
[lower5min],
[raise5min],
[lowerreg],
[raisereg],
[availability_reactive],
[availability_reactive_rbt]
)
select 
(select h.id from @header h),
d.[regionid],
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[connectionpointid],
d.[raise6sec],
d.[lower6sec],
d.[raise60sec],
d.[lower60sec],
d.[agc],
d.[fcascomp],
d.[loadshed],
d.[rgul],
d.[rguu],
d.[reactivepower],
d.[systemrestart],
d.[lastchanged],
d.[lower5min],
d.[raise5min],
d.[lowerreg],
d.[raisereg],
d.[availability_reactive],
d.[availability_reactive_rbt]
from openjson(@data) with (
[regionid] varchar(10),
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(10),
[connectionpointid] varchar(10),
[raise6sec] decimal(15,5),
[lower6sec] decimal(15,5),
[raise60sec] decimal(15,5),
[lower60sec] decimal(15,5),
[agc] decimal(15,5),
[fcascomp] decimal(15,5),
[loadshed] decimal(15,5),
[rgul] decimal(15,5),
[rguu] decimal(15,5),
[reactivepower] decimal(15,5),
[systemrestart] decimal(15,5),
[lastchanged] datetime2,
[lower5min] decimal(15,5),
[raise5min] decimal(15,5),
[lowerreg] decimal(15,5),
[raisereg] decimal(15,5),
[availability_reactive] decimal(18,8),
[availability_reactive_rbt] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingRegionfigures5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Regionfigures'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Regionfigures'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Regionfigures', 5);

insert into BillingRegionfigures5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[regionid],
[energyout],
[valueout],
[energypurchased],
[valuepurchased],
[excessgen],
[reservetrading],
[intcompo],
[adminpricecompo],
[settsurplus],
[aspayment],
[poolfees],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[regionid],
d.[energyout],
d.[valueout],
d.[energypurchased],
d.[valuepurchased],
d.[excessgen],
d.[reservetrading],
d.[intcompo],
d.[adminpricecompo],
d.[settsurplus],
d.[aspayment],
d.[poolfees],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[regionid] varchar(10),
[energyout] decimal(16,6),
[valueout] decimal(16,6),
[energypurchased] decimal(16,6),
[valuepurchased] decimal(16,6),
[excessgen] decimal(16,6),
[reservetrading] decimal(16,6),
[intcompo] decimal(16,6),
[adminpricecompo] decimal(16,6),
[settsurplus] decimal(16,6),
[aspayment] decimal(16,6),
[poolfees] decimal(16,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingEftshortfallDetail1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'EftshortfallDetail'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'EftshortfallDetail'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'EftshortfallDetail', 1);

insert into BillingEftshortfallDetail1(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[participantid],
[transaction_type],
[amount]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[transaction_type],
d.[amount]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(20),
[transaction_type] varchar(40),
[amount] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingAsrecovery7
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Asrecovery'
    and version = '7'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Asrecovery'
    and version = '7'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Asrecovery', 7);

insert into BillingAsrecovery7(
file_log_id,
[regionid],
[contractyear],
[weekno],
[billrunno],
[participantid],
[raise6sec],
[lower6sec],
[raise60sec],
[lower60sec],
[agc],
[fcascomp],
[loadshed],
[rgul],
[rguu],
[reactivepower],
[systemrestart],
[lastchanged],
[raise6sec_gen],
[lower6sec_gen],
[raise60sec_gen],
[lower60sec_gen],
[agc_gen],
[fcascomp_gen],
[loadshed_gen],
[rgul_gen],
[rguu_gen],
[reactivepower_gen],
[systemrestart_gen],
[lower5min],
[raise5min],
[lowerreg],
[raisereg],
[lower5min_gen],
[raise5min_gen],
[lowerreg_gen],
[raisereg_gen],
[availability_reactive],
[availability_reactive_rbt],
[availability_reactive_gen],
[availability_reactive_rbt_gen]
)
select 
(select h.id from @header h),
d.[regionid],
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[raise6sec],
d.[lower6sec],
d.[raise60sec],
d.[lower60sec],
d.[agc],
d.[fcascomp],
d.[loadshed],
d.[rgul],
d.[rguu],
d.[reactivepower],
d.[systemrestart],
d.[lastchanged],
d.[raise6sec_gen],
d.[lower6sec_gen],
d.[raise60sec_gen],
d.[lower60sec_gen],
d.[agc_gen],
d.[fcascomp_gen],
d.[loadshed_gen],
d.[rgul_gen],
d.[rguu_gen],
d.[reactivepower_gen],
d.[systemrestart_gen],
d.[lower5min],
d.[raise5min],
d.[lowerreg],
d.[raisereg],
d.[lower5min_gen],
d.[raise5min_gen],
d.[lowerreg_gen],
d.[raisereg_gen],
d.[availability_reactive],
d.[availability_reactive_rbt],
d.[availability_reactive_gen],
d.[availability_reactive_rbt_gen]
from openjson(@data) with (
[regionid] varchar(10),
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(10),
[raise6sec] decimal(15,5),
[lower6sec] decimal(15,5),
[raise60sec] decimal(15,5),
[lower60sec] decimal(15,5),
[agc] decimal(15,5),
[fcascomp] decimal(15,5),
[loadshed] decimal(15,5),
[rgul] decimal(15,5),
[rguu] decimal(15,5),
[reactivepower] decimal(15,5),
[systemrestart] decimal(15,5),
[lastchanged] datetime2,
[raise6sec_gen] decimal(15,5),
[lower6sec_gen] decimal(15,5),
[raise60sec_gen] decimal(15,5),
[lower60sec_gen] decimal(15,5),
[agc_gen] decimal(15,5),
[fcascomp_gen] decimal(15,5),
[loadshed_gen] decimal(15,5),
[rgul_gen] decimal(15,5),
[rguu_gen] decimal(15,5),
[reactivepower_gen] decimal(15,5),
[systemrestart_gen] decimal(15,5),
[lower5min] decimal(15,5),
[raise5min] decimal(15,5),
[lowerreg] decimal(15,5),
[raisereg] decimal(15,5),
[lower5min_gen] decimal(16,6),
[raise5min_gen] decimal(16,6),
[lowerreg_gen] decimal(16,6),
[raisereg_gen] decimal(16,6),
[availability_reactive] decimal(18,8),
[availability_reactive_rbt] decimal(18,8),
[availability_reactive_gen] decimal(18,8),
[availability_reactive_rbt_gen] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingMrSummary5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'MrSummary'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'MrSummary'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'MrSummary', 5);

insert into BillingMrSummary5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[mr_date],
[regionid],
[total_payments],
[total_recovery],
[total_rsa],
[aage],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[mr_date],
d.[regionid],
d.[total_payments],
d.[total_recovery],
d.[total_rsa],
d.[aage],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[mr_date] datetime2,
[regionid] varchar(10),
[total_payments] decimal(16,6),
[total_recovery] decimal(16,6),
[total_rsa] decimal(16,6),
[aage] decimal(16,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingRegionexports5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Regionexports'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Regionexports'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Regionexports', 5);

insert into BillingRegionexports5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[regionid],
[exportto],
[energy],
[value],
[surplusenergy],
[surplusvalue],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[regionid],
d.[exportto],
d.[energy],
d.[value],
d.[surplusenergy],
d.[surplusvalue],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[regionid] varchar(10),
[exportto] varchar(10),
[energy] decimal(16,6),
[value] decimal(15,5),
[surplusenergy] decimal(16,6),
[surplusvalue] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingSecdepInterestRate1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'SecdepInterestRate'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'SecdepInterestRate'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'SecdepInterestRate', 1);

insert into BillingSecdepInterestRate1(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[interest_acct_id],
[effectivedate],
[interest_rate]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[interest_acct_id],
d.[effectivedate],
d.[interest_rate]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[interest_acct_id] varchar(20),
[effectivedate] datetime2,
[interest_rate] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingIrpartsurplussum7
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Irpartsurplussum'
    and version = '7'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Irpartsurplussum'
    and version = '7'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Irpartsurplussum', 7);

insert into BillingIrpartsurplussum7(
file_log_id,
[contractyear],
[weekno],
[residueyear],
[quarter],
[billrunno],
[interconnectorid],
[fromregionid],
[participantid],
[totalsurplus],
[auctionfees],
[actualpayment],
[auctionfees_gst],
[lastchanged],
[csp_derogation_amount],
[unadjusted_irsr],
[auctionfees_totalgross_adj]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[residueyear],
d.[quarter],
d.[billrunno],
d.[interconnectorid],
d.[fromregionid],
d.[participantid],
d.[totalsurplus],
d.[auctionfees],
d.[actualpayment],
d.[auctionfees_gst],
d.[lastchanged],
d.[csp_derogation_amount],
d.[unadjusted_irsr],
d.[auctionfees_totalgross_adj]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[residueyear] decimal(4,0),
[quarter] decimal(2,0),
[billrunno] decimal(3,0),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[participantid] varchar(10),
[totalsurplus] decimal(15,5),
[auctionfees] decimal(15,5),
[actualpayment] decimal(15,5),
[auctionfees_gst] decimal(15,5),
[lastchanged] datetime2,
[csp_derogation_amount] decimal(18,8),
[unadjusted_irsr] decimal(18,8),
[auctionfees_totalgross_adj] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingBillingCo2ePublicationTrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'BillingCo2ePublicationTrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'BillingCo2ePublicationTrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'BillingCo2ePublicationTrk', 1);

insert into BillingBillingCo2ePublicationTrk1(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingGstDetail5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'GstDetail'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'GstDetail'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'GstDetail', 5);

insert into BillingGstDetail5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[participantid],
[bas_class],
[transaction_type],
[gst_exclusive_amount],
[gst_amount],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[bas_class],
d.[transaction_type],
d.[gst_exclusive_amount],
d.[gst_amount],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(10),
[bas_class] varchar(30),
[transaction_type] varchar(30),
[gst_exclusive_amount] decimal(15,5),
[gst_amount] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingDirectionReconciliatn1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'DirectionReconciliatn'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'DirectionReconciliatn'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'DirectionReconciliatn', 1);

insert into BillingDirectionReconciliatn1(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[direction_id],
[direction_desc],
[direction_start_date],
[direction_end_date],
[compensation_amount],
[independent_expert_fee],
[interest_amount],
[cra],
[nem_fee_id],
[nem_fixed_fee_amount],
[mkt_customer_perc],
[generator_perc],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[direction_id],
d.[direction_desc],
d.[direction_start_date],
d.[direction_end_date],
d.[compensation_amount],
d.[independent_expert_fee],
d.[interest_amount],
d.[cra],
d.[nem_fee_id],
d.[nem_fixed_fee_amount],
d.[mkt_customer_perc],
d.[generator_perc],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[direction_id] varchar(20),
[direction_desc] varchar(200),
[direction_start_date] datetime2,
[direction_end_date] datetime2,
[compensation_amount] decimal(16,6),
[independent_expert_fee] decimal(16,6),
[interest_amount] decimal(16,6),
[cra] decimal(16,6),
[nem_fee_id] varchar(20),
[nem_fixed_fee_amount] decimal(16,6),
[mkt_customer_perc] decimal(16,6),
[generator_perc] decimal(16,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingBillingCo2ePublication1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'BillingCo2ePublication'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'BillingCo2ePublication'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'BillingCo2ePublication', 1);

insert into BillingBillingCo2ePublication1(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[settlementdate],
[regionid],
[sentoutenergy],
[generatoremissions],
[intensityindex]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[settlementdate],
d.[regionid],
d.[sentoutenergy],
d.[generatoremissions],
d.[intensityindex]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[settlementdate] datetime2,
[regionid] varchar(20),
[sentoutenergy] decimal(18,8),
[generatoremissions] decimal(18,8),
[intensityindex] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingInterresidues5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Interresidues'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Interresidues'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Interresidues', 5);

insert into BillingInterresidues5(
file_log_id,
[allocation],
[totalsurplus],
[interconnectorid],
[contractyear],
[weekno],
[billrunno],
[participantid],
[surplusvalue],
[lastchanged],
[regionid]
)
select 
(select h.id from @header h),
d.[allocation],
d.[totalsurplus],
d.[interconnectorid],
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[surplusvalue],
d.[lastchanged],
d.[regionid]
from openjson(@data) with (
[allocation] decimal(6,3),
[totalsurplus] decimal(15,5),
[interconnectorid] varchar(10),
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(10),
[surplusvalue] decimal(15,6),
[lastchanged] datetime2,
[regionid] varchar(10)
) d
end
go
create or alter procedure InsertBillingNmasTstPayments1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'NmasTstPayments'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'NmasTstPayments'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'NmasTstPayments', 1);

insert into BillingNmasTstPayments1(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[participantid],
[service],
[contractid],
[payment_amount]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[service],
d.[contractid],
d.[payment_amount]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(20),
[service] varchar(10),
[contractid] varchar(10),
[payment_amount] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingIrnspsurplus5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Irnspsurplus'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Irnspsurplus'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Irnspsurplus', 5);

insert into BillingIrnspsurplus5(
file_log_id,
[contractyear],
[weekno],
[residueyear],
[quarter],
[billrunno],
[contractid],
[participantid],
[interconnectorid],
[fromregionid],
[totalresidues],
[adjustment],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[residueyear],
d.[quarter],
d.[billrunno],
d.[contractid],
d.[participantid],
d.[interconnectorid],
d.[fromregionid],
d.[totalresidues],
d.[adjustment],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(2,0),
[residueyear] decimal(4,0),
[quarter] decimal(2,0),
[billrunno] decimal(3,0),
[contractid] varchar(30),
[participantid] varchar(10),
[interconnectorid] varchar(10),
[fromregionid] varchar(10),
[totalresidues] decimal(15,5),
[adjustment] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingMrShortfall5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'MrShortfall'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'MrShortfall'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'MrShortfall', 5);

insert into BillingMrShortfall5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[mr_date],
[regionid],
[participantid],
[age],
[rsa],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[mr_date],
d.[regionid],
d.[participantid],
d.[age],
d.[rsa],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[mr_date] datetime2,
[regionid] varchar(10),
[participantid] varchar(10),
[age] decimal(16,6),
[rsa] decimal(16,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingApcCompensation2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'ApcCompensation'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'ApcCompensation'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'ApcCompensation', 2);

insert into BillingApcCompensation2(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[apeventid],
[claimid],
[participantid],
[compensation_amount],
[event_type],
[compensation_type],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[apeventid],
d.[claimid],
d.[participantid],
d.[compensation_amount],
d.[event_type],
d.[compensation_type],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[apeventid] decimal(6,0),
[claimid] decimal(6,0),
[participantid] varchar(20),
[compensation_amount] decimal(18,8),
[event_type] varchar(20),
[compensation_type] varchar(20),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingGendata5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Gendata'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Gendata'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Gendata', 5);

insert into BillingGendata5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[participantid],
[connectionpointid],
[stationid],
[duid],
[aggregateenergy],
[sales],
[purchases],
[lastchanged],
[purchasedenergy],
[mda]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[connectionpointid],
d.[stationid],
d.[duid],
d.[aggregateenergy],
d.[sales],
d.[purchases],
d.[lastchanged],
d.[purchasedenergy],
d.[mda]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(10),
[connectionpointid] varchar(10),
[stationid] varchar(10),
[duid] varchar(10),
[aggregateenergy] decimal(16,6),
[sales] decimal(16,6),
[purchases] decimal(16,6),
[lastchanged] datetime2,
[purchasedenergy] decimal(16,6),
[mda] varchar(10)
) d
end
go
create or alter procedure InsertBillingRealloc5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Realloc'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Realloc'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Realloc', 5);

insert into BillingRealloc5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[participantid],
[counterparty],
[value],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[counterparty],
d.[value],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(10),
[counterparty] varchar(10),
[value] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingResTraderPayment1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'ResTraderPayment'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'ResTraderPayment'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'ResTraderPayment', 1);

insert into BillingResTraderPayment1(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[contractid],
[payment_type],
[participantid],
[payment_amount]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[contractid],
d.[payment_type],
d.[participantid],
d.[payment_amount]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[contractid] varchar(20),
[payment_type] varchar(40),
[participantid] varchar(20),
[payment_amount] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingFees5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Fees'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Fees'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Fees', 5);

insert into BillingFees5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[participantid],
[marketfeeid],
[rate],
[energy],
[value],
[lastchanged],
[participantcategoryid]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[marketfeeid],
d.[rate],
d.[energy],
d.[value],
d.[lastchanged],
d.[participantcategoryid]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(10),
[marketfeeid] varchar(10),
[rate] decimal(15,5),
[energy] decimal(16,6),
[value] decimal(15,5),
[lastchanged] datetime2,
[participantcategoryid] varchar(10)
) d
end
go
create or alter procedure InsertBillingRegionimports5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Regionimports'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Regionimports'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Regionimports', 5);

insert into BillingRegionimports5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[regionid],
[importfrom],
[energy],
[value],
[surplusenergy],
[surplusvalue],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[regionid],
d.[importfrom],
d.[energy],
d.[value],
d.[surplusenergy],
d.[surplusvalue],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[regionid] varchar(10),
[importfrom] varchar(10),
[energy] decimal(16,6),
[value] decimal(15,5),
[surplusenergy] decimal(16,6),
[surplusvalue] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingPrioradjustments5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Prioradjustments'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Prioradjustments'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Prioradjustments', 5);

insert into BillingPrioradjustments5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[adjcontractyear],
[adjweekno],
[adjbillrunno],
[participantid],
[prevamount],
[adjamount],
[irn],
[irp],
[interestamount],
[lastchanged],
[irsr_prevamount],
[irsr_adjamount],
[irsr_interestamount]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[adjcontractyear],
d.[adjweekno],
d.[adjbillrunno],
d.[participantid],
d.[prevamount],
d.[adjamount],
d.[irn],
d.[irp],
d.[interestamount],
d.[lastchanged],
d.[irsr_prevamount],
d.[irsr_adjamount],
d.[irsr_interestamount]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[adjcontractyear] decimal(4,0),
[adjweekno] decimal(3,0),
[adjbillrunno] decimal(3,0),
[participantid] varchar(10),
[prevamount] decimal(15,5),
[adjamount] decimal(15,5),
[irn] decimal(15,5),
[irp] decimal(15,5),
[interestamount] decimal(15,5),
[lastchanged] datetime2,
[irsr_prevamount] decimal(15,5),
[irsr_adjamount] decimal(15,5),
[irsr_interestamount] decimal(15,5)
) d
end
go
create or alter procedure InsertBillingMrRecovery5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'MrRecovery'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'MrRecovery'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'MrRecovery', 5);

insert into BillingMrRecovery5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[mr_date],
[regionid],
[participantid],
[duid],
[mr_amount],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[mr_date],
d.[regionid],
d.[participantid],
d.[duid],
d.[mr_amount],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[mr_date] datetime2,
[regionid] varchar(10),
[participantid] varchar(10),
[duid] varchar(10),
[mr_amount] decimal(16,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingDailyEnergySummary1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'DailyEnergySummary'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'DailyEnergySummary'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'DailyEnergySummary', 1);

insert into BillingDailyEnergySummary1(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[settlementdate],
[participantid],
[regionid],
[customer_energy_purchased],
[generator_energy_sold],
[generator_energy_purchased]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[settlementdate],
d.[participantid],
d.[regionid],
d.[customer_energy_purchased],
d.[generator_energy_sold],
d.[generator_energy_purchased]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[settlementdate] datetime2,
[participantid] varchar(20),
[regionid] varchar(20),
[customer_energy_purchased] decimal(18,8),
[generator_energy_sold] decimal(18,8),
[generator_energy_purchased] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingBillingDirectionReconOther1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'BillingDirectionReconOther'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'BillingDirectionReconOther'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'BillingDirectionReconOther', 1);

insert into BillingBillingDirectionReconOther1(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[direction_id],
[regionid],
[direction_desc],
[direction_type_id],
[direction_start_date],
[direction_end_date],
[direction_start_interval],
[direction_end_interval],
[compensation_amount],
[interest_amount],
[independent_expert_fee],
[cra],
[regional_customer_energy],
[regional_generator_energy],
[regional_benefit_factor]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[direction_id],
d.[regionid],
d.[direction_desc],
d.[direction_type_id],
d.[direction_start_date],
d.[direction_end_date],
d.[direction_start_interval],
d.[direction_end_interval],
d.[compensation_amount],
d.[interest_amount],
d.[independent_expert_fee],
d.[cra],
d.[regional_customer_energy],
d.[regional_generator_energy],
d.[regional_benefit_factor]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[direction_id] varchar(20),
[regionid] varchar(20),
[direction_desc] varchar(200),
[direction_type_id] varchar(20),
[direction_start_date] datetime2,
[direction_end_date] datetime2,
[direction_start_interval] datetime2,
[direction_end_interval] datetime2,
[compensation_amount] decimal(18,8),
[interest_amount] decimal(18,8),
[independent_expert_fee] decimal(18,8),
[cra] decimal(18,8),
[regional_customer_energy] decimal(18,8),
[regional_generator_energy] decimal(18,8),
[regional_benefit_factor] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingMrPayment5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'MrPayment'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'MrPayment'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'MrPayment', 5);

insert into BillingMrPayment5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[mr_date],
[regionid],
[participantid],
[duid],
[mr_amount],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[mr_date],
d.[regionid],
d.[participantid],
d.[duid],
d.[mr_amount],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[mr_date] datetime2,
[regionid] varchar(10),
[participantid] varchar(10),
[duid] varchar(10),
[mr_amount] decimal(16,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingResTraderRecovery1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'ResTraderRecovery'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'ResTraderRecovery'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'ResTraderRecovery', 1);

insert into BillingResTraderRecovery1(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[regionid],
[participantid],
[recovery_amount]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[regionid],
d.[participantid],
d.[recovery_amount]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[regionid] varchar(20),
[participantid] varchar(20),
[recovery_amount] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingCpdata5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Cpdata'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Cpdata'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Cpdata', 5);

insert into BillingCpdata5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[participantid],
[connectionpointid],
[aggregateenergy],
[purchases],
[lastchanged],
[mda]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[connectionpointid],
d.[aggregateenergy],
d.[purchases],
d.[lastchanged],
d.[mda]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(10),
[connectionpointid] varchar(10),
[aggregateenergy] decimal(16,6),
[purchases] decimal(16,6),
[lastchanged] datetime2,
[mda] varchar(10)
) d
end
go
create or alter procedure InsertBillingNmasTstRecvryRbf1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'NmasTstRecvryRbf'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'NmasTstRecvryRbf'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'NmasTstRecvryRbf', 1);

insert into BillingNmasTstRecvryRbf1(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[service],
[contractid],
[regionid],
[rbf],
[payment_amount],
[recovery_amount],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[service],
d.[contractid],
d.[regionid],
d.[rbf],
d.[payment_amount],
d.[recovery_amount],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[service] varchar(10),
[contractid] varchar(10),
[regionid] varchar(10),
[rbf] decimal(18,8),
[payment_amount] decimal(18,8),
[recovery_amount] decimal(18,8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingGstSummary5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'GstSummary'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'GstSummary'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'GstSummary', 5);

insert into BillingGstSummary5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[participantid],
[bas_class],
[gst_exclusive_amount],
[gst_amount],
[lastchanged]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[bas_class],
d.[gst_exclusive_amount],
d.[gst_amount],
d.[lastchanged]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(10),
[bas_class] varchar(30),
[gst_exclusive_amount] decimal(15,5),
[gst_amount] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingSecdepositApplication1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'SecdepositApplication'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'SecdepositApplication'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'SecdepositApplication', 1);

insert into BillingSecdepositApplication1(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[participantid],
[application_amount]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[application_amount]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(20),
[application_amount] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingFinancialadjustments5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Financialadjustments'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Billing'
    and sub_type = 'Financialadjustments'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Billing', 'Financialadjustments', 5);

insert into BillingFinancialadjustments5(
file_log_id,
[contractyear],
[weekno],
[billrunno],
[participantid],
[participanttype],
[adjustmentitem],
[amount],
[value],
[lastchanged],
[financialcode],
[bas_class]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[billrunno],
d.[participantid],
d.[participanttype],
d.[adjustmentitem],
d.[amount],
d.[value],
d.[lastchanged],
d.[financialcode],
d.[bas_class]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[billrunno] decimal(3,0),
[participantid] varchar(10),
[participanttype] varchar(10),
[adjustmentitem] varchar(64),
[amount] decimal(15,5),
[value] decimal(15,5),
[lastchanged] datetime2,
[financialcode] decimal(10,0),
[bas_class] varchar(30)
) d
end
go
create or alter procedure InsertParticipantRegistrationStation1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Station'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Station'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Station', 1);

insert into ParticipantRegistrationStation1(
file_log_id,
[stationid],
[stationname],
[address1],
[address2],
[address3],
[address4],
[city],
[state],
[postcode],
[lastchanged],
[connectionpointid],
[semidispatchcap]
)
select 
(select h.id from @header h),
d.[stationid],
d.[stationname],
d.[address1],
d.[address2],
d.[address3],
d.[address4],
d.[city],
d.[state],
d.[postcode],
d.[lastchanged],
d.[connectionpointid],
d.[semidispatchcap]
from openjson(@data) with (
[stationid] varchar(10),
[stationname] varchar(80),
[address1] varchar(80),
[address2] varchar(80),
[address3] varchar(80),
[address4] varchar(80),
[city] varchar(40),
[state] varchar(10),
[postcode] varchar(10),
[lastchanged] datetime2,
[connectionpointid] varchar(10),
[semidispatchcap] decimal(3,0)
) d
end
go
create or alter procedure InsertParticipantRegistrationStadualloc1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Stadualloc'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Stadualloc'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Stadualloc', 1);

insert into ParticipantRegistrationStadualloc1(
file_log_id,
[duid],
[effectivedate],
[stationid],
[versionno],
[lastchanged],
[lastchanged]
)
select 
(select h.id from @header h),
d.[duid],
d.[effectivedate],
d.[stationid],
d.[versionno],
d.[lastchanged],
d.[lastchanged]
from openjson(@data) with (
[duid] varchar(10),
[effectivedate] datetime2,
[stationid] varchar(10),
[versionno] decimal(3,0),
[lastchanged] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertParticipantRegistrationDudetailsummary4
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Dudetailsummary'
    and version = '4'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Dudetailsummary'
    and version = '4'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Dudetailsummary', 4);

insert into ParticipantRegistrationDudetailsummary4(
file_log_id,
[duid],
[start_date],
[end_date],
[dispatchtype],
[connectionpointid],
[regionid],
[stationid],
[participantid],
[lastchanged],
[transmissionlossfactor],
[starttype],
[distributionlossfactor],
[minimum_energy_price],
[maximum_energy_price],
[schedule_type],
[min_ramp_rate_up],
[min_ramp_rate_down],
[max_ramp_rate_up],
[max_ramp_rate_down],
[is_aggregated]
)
select 
(select h.id from @header h),
d.[duid],
d.[start_date],
d.[end_date],
d.[dispatchtype],
d.[connectionpointid],
d.[regionid],
d.[stationid],
d.[participantid],
d.[lastchanged],
d.[transmissionlossfactor],
d.[starttype],
d.[distributionlossfactor],
d.[minimum_energy_price],
d.[maximum_energy_price],
d.[schedule_type],
d.[min_ramp_rate_up],
d.[min_ramp_rate_down],
d.[max_ramp_rate_up],
d.[max_ramp_rate_down],
d.[is_aggregated]
from openjson(@data) with (
[duid] varchar(10),
[start_date] datetime2,
[end_date] datetime2,
[dispatchtype] varchar(10),
[connectionpointid] varchar(10),
[regionid] varchar(10),
[stationid] varchar(10),
[participantid] varchar(10),
[lastchanged] datetime2,
[transmissionlossfactor] decimal(15,5),
[starttype] varchar(20),
[distributionlossfactor] decimal(15,5),
[minimum_energy_price] decimal(9,2),
[maximum_energy_price] decimal(9,2),
[schedule_type] varchar(20),
[min_ramp_rate_up] decimal(6,0),
[min_ramp_rate_down] decimal(6,0),
[max_ramp_rate_up] decimal(6,0),
[max_ramp_rate_down] decimal(6,0),
[is_aggregated] decimal(1,0)
) d
end
go
create or alter procedure InsertParticipantRegistrationStationownertrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Stationownertrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Stationownertrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Stationownertrk', 1);

insert into ParticipantRegistrationStationownertrk1(
file_log_id,
[effectivedate],
[participantid],
[versionno],
[authorisedby],
[authoriseddate],
[lastchanged],
[rrpeep43]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[participantid],
d.[versionno],
d.[authorisedby],
d.[authoriseddate],
d.[lastchanged],
d.[rrpeep43]
from openjson(@data) with (
[effectivedate] datetime2,
[participantid] varchar(10),
[versionno] decimal(3,0),
[authorisedby] varchar(15),
[authoriseddate] datetime2,
[lastchanged] datetime2,
[rrpeep43] decimal(15,5)
) d
end
go
create or alter procedure InsertParticipantRegistrationGenunitsUnit1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'GenunitsUnit'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'GenunitsUnit'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'GenunitsUnit', 1);

insert into ParticipantRegistrationGenunitsUnit1(
file_log_id,
[gensetid],
[effectivedate],
[versionno],
[unit_grouping_label],
[unit_count],
[unit_size],
[unit_max_size],
[aggregation_flag],
[lastchanged]
)
select 
(select h.id from @header h),
d.[gensetid],
d.[effectivedate],
d.[versionno],
d.[unit_grouping_label],
d.[unit_count],
d.[unit_size],
d.[unit_max_size],
d.[aggregation_flag],
d.[lastchanged]
from openjson(@data) with (
[gensetid] varchar(20),
[effectivedate] datetime2,
[versionno] decimal(6,0),
[unit_grouping_label] varchar(20),
[unit_count] decimal(3,0),
[unit_size] decimal(8,3),
[unit_max_size] decimal(8,3),
[aggregation_flag] decimal(1,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertParticipantRegistrationParticipantcategoryalloc1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Participantcategoryalloc'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Participantcategoryalloc'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Participantcategoryalloc', 1);

insert into ParticipantRegistrationParticipantcategoryalloc1(
file_log_id,
[participantcategoryid],
[participantid],
[lastchanged]
)
select 
(select h.id from @header h),
d.[participantcategoryid],
d.[participantid],
d.[lastchanged]
from openjson(@data) with (
[participantcategoryid] varchar(10),
[participantid] varchar(10),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertParticipantRegistrationGenunits2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Genunits'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Genunits'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Genunits', 2);

insert into ParticipantRegistrationGenunits2(
file_log_id,
[gensetid],
[stationid],
[setlossfactor],
[cdindicator],
[agcflag],
[spinningflag],
[voltlevel],
[registeredcapacity],
[dispatchtype],
[starttype],
[mktgeneratorind],
[normalstatus],
[maxcapacity],
[gensettype],
[gensetname],
[lastchanged],
[co2e_emissions_factor],
[co2e_energy_source],
[co2e_data_source]
)
select 
(select h.id from @header h),
d.[gensetid],
d.[stationid],
d.[setlossfactor],
d.[cdindicator],
d.[agcflag],
d.[spinningflag],
d.[voltlevel],
d.[registeredcapacity],
d.[dispatchtype],
d.[starttype],
d.[mktgeneratorind],
d.[normalstatus],
d.[maxcapacity],
d.[gensettype],
d.[gensetname],
d.[lastchanged],
d.[co2e_emissions_factor],
d.[co2e_energy_source],
d.[co2e_data_source]
from openjson(@data) with (
[gensetid] varchar(20),
[stationid] varchar(10),
[setlossfactor] decimal(16,6),
[cdindicator] varchar(10),
[agcflag] varchar(2),
[spinningflag] varchar(2),
[voltlevel] decimal(6,0),
[registeredcapacity] decimal(6,0),
[dispatchtype] varchar(10),
[starttype] varchar(20),
[mktgeneratorind] varchar(10),
[normalstatus] varchar(10),
[maxcapacity] decimal(6,0),
[gensettype] varchar(15),
[gensetname] varchar(40),
[lastchanged] datetime2,
[co2e_emissions_factor] decimal(18,8),
[co2e_energy_source] varchar(100),
[co2e_data_source] varchar(20)
) d
end
go
create or alter procedure InsertParticipantRegistrationParticipantcreditdetail1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Participantcreditdetail'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Participantcreditdetail'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Participantcreditdetail', 1);

insert into ParticipantRegistrationParticipantcreditdetail1(
file_log_id,
[effectivedate],
[participantid],
[creditlimit],
[authorisedby],
[authoriseddate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[participantid],
d.[creditlimit],
d.[authorisedby],
d.[authoriseddate],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[participantid] varchar(10),
[creditlimit] decimal(10,0),
[authorisedby] varchar(15),
[authoriseddate] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertParticipantRegistrationMnspInterconnector2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'MnspInterconnector'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'MnspInterconnector'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'MnspInterconnector', 2);

insert into ParticipantRegistrationMnspInterconnector2(
file_log_id,
[linkid],
[effectivedate],
[versionno],
[interconnectorid],
[fromregion],
[toregion],
[maxcapacity],
[tlf],
[lhsfactor],
[meterflowconstant],
[authoriseddate],
[authorisedby],
[lastchanged],
[from_region_tlf],
[to_region_tlf]
)
select 
(select h.id from @header h),
d.[linkid],
d.[effectivedate],
d.[versionno],
d.[interconnectorid],
d.[fromregion],
d.[toregion],
d.[maxcapacity],
d.[tlf],
d.[lhsfactor],
d.[meterflowconstant],
d.[authoriseddate],
d.[authorisedby],
d.[lastchanged],
d.[from_region_tlf],
d.[to_region_tlf]
from openjson(@data) with (
[linkid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[interconnectorid] varchar(10),
[fromregion] varchar(10),
[toregion] varchar(10),
[maxcapacity] decimal(5,0),
[tlf] decimal(12,7),
[lhsfactor] decimal(12,7),
[meterflowconstant] decimal(12,7),
[authoriseddate] datetime2,
[authorisedby] varchar(15),
[lastchanged] datetime2,
[from_region_tlf] decimal(12,7),
[to_region_tlf] decimal(12,7)
) d
end
go
create or alter procedure InsertParticipantRegistrationParticipantaccount1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Participantaccount'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Participantaccount'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Participantaccount', 1);

insert into ParticipantRegistrationParticipantaccount1(
file_log_id,
[accountname],
[participantid],
[accountnumber],
[bankname],
[banknumber],
[branchname],
[branchnumber],
[bsbnumber],
[nemmcocreditaccountnumber],
[nemmcodebitaccountnumber],
[authorisedby],
[authoriseddate],
[effectivedate],
[lastchanged],
[abn]
)
select 
(select h.id from @header h),
d.[accountname],
d.[participantid],
d.[accountnumber],
d.[bankname],
d.[banknumber],
d.[branchname],
d.[branchnumber],
d.[bsbnumber],
d.[nemmcocreditaccountnumber],
d.[nemmcodebitaccountnumber],
d.[authorisedby],
d.[authoriseddate],
d.[effectivedate],
d.[lastchanged],
d.[abn]
from openjson(@data) with (
[accountname] varchar(80),
[participantid] varchar(10),
[accountnumber] varchar(16),
[bankname] varchar(16),
[banknumber] decimal(10,0),
[branchname] varchar(16),
[branchnumber] decimal(10,0),
[bsbnumber] varchar(20),
[nemmcocreditaccountnumber] decimal(10,0),
[nemmcodebitaccountnumber] decimal(10,0),
[authorisedby] varchar(15),
[authoriseddate] datetime2,
[effectivedate] datetime2,
[lastchanged] datetime2,
[abn] varchar(20)
) d
end
go
create or alter procedure InsertParticipantRegistrationDualloc1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Dualloc'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Dualloc'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Dualloc', 1);

insert into ParticipantRegistrationDualloc1(
file_log_id,
[effectivedate],
[versionno],
[duid],
[gensetid],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[versionno],
d.[duid],
d.[gensetid],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[versionno] decimal(3,0),
[duid] varchar(10),
[gensetid] varchar(20),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertParticipantRegistrationGenmeter1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Genmeter'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Genmeter'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Genmeter', 1);

insert into ParticipantRegistrationGenmeter1(
file_log_id,
[meterid],
[gensetid],
[connectionpointid],
[stationid],
[metertype],
[meterclass],
[voltagelevel],
[applydate],
[versionno],
[authorisedby],
[authoriseddate],
[comdate],
[decomdate],
[enddate],
[startdate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[meterid],
d.[gensetid],
d.[connectionpointid],
d.[stationid],
d.[metertype],
d.[meterclass],
d.[voltagelevel],
d.[applydate],
d.[versionno],
d.[authorisedby],
d.[authoriseddate],
d.[comdate],
d.[decomdate],
d.[enddate],
d.[startdate],
d.[lastchanged]
from openjson(@data) with (
[meterid] varchar(12),
[gensetid] varchar(20),
[connectionpointid] varchar(10),
[stationid] varchar(10),
[metertype] varchar(20),
[meterclass] varchar(10),
[voltagelevel] decimal(6,0),
[applydate] datetime2,
[versionno] decimal(3,0),
[authorisedby] varchar(10),
[authoriseddate] datetime2,
[comdate] datetime2,
[decomdate] datetime2,
[enddate] datetime2,
[startdate] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertParticipantRegistrationParticipantclass1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Participantclass'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Participantclass'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Participantclass', 1);

insert into ParticipantRegistrationParticipantclass1(
file_log_id,
[participantclassid],
[description],
[lastchanged]
)
select 
(select h.id from @header h),
d.[participantclassid],
d.[description],
d.[lastchanged]
from openjson(@data) with (
[participantclassid] varchar(20),
[description] varchar(64),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertParticipantRegistrationMnspParticipant1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'MnspParticipant'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'MnspParticipant'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'MnspParticipant', 1);

insert into ParticipantRegistrationMnspParticipant1(
file_log_id,
[interconnectorid],
[effectivedate],
[versionno],
[participantid],
[lastchanged]
)
select 
(select h.id from @header h),
d.[interconnectorid],
d.[effectivedate],
d.[versionno],
d.[participantid],
d.[lastchanged]
from openjson(@data) with (
[interconnectorid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[participantid] varchar(10),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertParticipantRegistrationParticipant1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Participant'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Participant'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Participant', 1);

insert into ParticipantRegistrationParticipant1(
file_log_id,
[participantid],
[participantclassid],
[name],
[description],
[acn],
[primarybusiness],
[lastchanged]
)
select 
(select h.id from @header h),
d.[participantid],
d.[participantclassid],
d.[name],
d.[description],
d.[acn],
d.[primarybusiness],
d.[lastchanged]
from openjson(@data) with (
[participantid] varchar(10),
[participantclassid] varchar(20),
[name] varchar(80),
[description] varchar(64),
[acn] varchar(9),
[primarybusiness] varchar(40),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertParticipantRegistrationStationoperatingstatus1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Stationoperatingstatus'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Stationoperatingstatus'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Stationoperatingstatus', 1);

insert into ParticipantRegistrationStationoperatingstatus1(
file_log_id,
[effectivedate],
[stationid],
[versionno],
[status],
[authorisedby],
[authoriseddate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[stationid],
d.[versionno],
d.[status],
d.[authorisedby],
d.[authoriseddate],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[stationid] varchar(10),
[versionno] decimal(3,0),
[status] varchar(20),
[authorisedby] varchar(15),
[authoriseddate] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertParticipantRegistrationStationowner1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Stationowner'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Stationowner'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Stationowner', 1);

insert into ParticipantRegistrationStationowner1(
file_log_id,
[effectivedate],
[participantid],
[stationid],
[versionno],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[participantid],
d.[stationid],
d.[versionno],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[participantid] varchar(10),
[stationid] varchar(10),
[versionno] decimal(3,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertParticipantRegistrationDispatchableunit1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Dispatchableunit'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Dispatchableunit'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Dispatchableunit', 1);

insert into ParticipantRegistrationDispatchableunit1(
file_log_id,
[duid],
[duname],
[unittype],
[lastchanged]
)
select 
(select h.id from @header h),
d.[duid],
d.[duname],
d.[unittype],
d.[lastchanged]
from openjson(@data) with (
[duid] varchar(10),
[duname] varchar(20),
[unittype] varchar(20),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertParticipantRegistrationParticipantcategory1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Participantcategory'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Participantcategory'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Participantcategory', 1);

insert into ParticipantRegistrationParticipantcategory1(
file_log_id,
[participantcategoryid],
[description],
[lastchanged]
)
select 
(select h.id from @header h),
d.[participantcategoryid],
d.[description],
d.[lastchanged]
from openjson(@data) with (
[participantcategoryid] varchar(10),
[description] varchar(64),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertParticipantRegistrationBidduiddetails1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Bidduiddetails'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Bidduiddetails'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Bidduiddetails', 1);

insert into ParticipantRegistrationBidduiddetails1(
file_log_id,
[duid],
[effectivedate],
[versionno],
[bidtype],
[maxcapacity],
[minenablementlevel],
[maxenablementlevel],
[maxlowerangle],
[maxupperangle],
[lastchanged]
)
select 
(select h.id from @header h),
d.[duid],
d.[effectivedate],
d.[versionno],
d.[bidtype],
d.[maxcapacity],
d.[minenablementlevel],
d.[maxenablementlevel],
d.[maxlowerangle],
d.[maxupperangle],
d.[lastchanged]
from openjson(@data) with (
[duid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[bidtype] varchar(10),
[maxcapacity] decimal(22,0),
[minenablementlevel] decimal(22,0),
[maxenablementlevel] decimal(22,0),
[maxlowerangle] decimal(3,0),
[maxupperangle] decimal(3,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertParticipantRegistrationBidduiddetailstrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Bidduiddetailstrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Bidduiddetailstrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Bidduiddetailstrk', 1);

insert into ParticipantRegistrationBidduiddetailstrk1(
file_log_id,
[duid],
[effectivedate],
[versionno],
[authoriseddate],
[authorisedby],
[lastchanged]
)
select 
(select h.id from @header h),
d.[duid],
d.[effectivedate],
d.[versionno],
d.[authoriseddate],
d.[authorisedby],
d.[lastchanged]
from openjson(@data) with (
[duid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[authoriseddate] datetime2,
[authorisedby] varchar(15),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertParticipantRegistrationDudetail3
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Dudetail'
    and version = '3'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ParticipantRegistration'
    and sub_type = 'Dudetail'
    and version = '3'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ParticipantRegistration', 'Dudetail', 3);

insert into ParticipantRegistrationDudetail3(
file_log_id,
[effectivedate],
[duid],
[versionno],
[connectionpointid],
[voltlevel],
[registeredcapacity],
[agccapability],
[dispatchtype],
[maxcapacity],
[starttype],
[normallyonflag],
[physicaldetailsflag],
[spinningreserveflag],
[authorisedby],
[authoriseddate],
[lastchanged],
[intermittentflag],
[semi_schedule_flag],
[maxrateofchangeup],
[maxrateofchangedown]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[duid],
d.[versionno],
d.[connectionpointid],
d.[voltlevel],
d.[registeredcapacity],
d.[agccapability],
d.[dispatchtype],
d.[maxcapacity],
d.[starttype],
d.[normallyonflag],
d.[physicaldetailsflag],
d.[spinningreserveflag],
d.[authorisedby],
d.[authoriseddate],
d.[lastchanged],
d.[intermittentflag],
d.[semi_schedule_flag],
d.[maxrateofchangeup],
d.[maxrateofchangedown]
from openjson(@data) with (
[effectivedate] datetime2,
[duid] varchar(10),
[versionno] decimal(3,0),
[connectionpointid] varchar(10),
[voltlevel] varchar(10),
[registeredcapacity] decimal(6,0),
[agccapability] varchar(1),
[dispatchtype] varchar(10),
[maxcapacity] decimal(6,0),
[starttype] varchar(20),
[normallyonflag] varchar(1),
[physicaldetailsflag] varchar(1),
[spinningreserveflag] varchar(1),
[authorisedby] varchar(15),
[authoriseddate] datetime2,
[lastchanged] datetime2,
[intermittentflag] varchar(1),
[semi_schedule_flag] varchar(1),
[maxrateofchangeup] decimal(6,0),
[maxrateofchangedown] decimal(6,0)
) d
end
go
create or alter procedure InsertSettlementConfigMarketFeeCatExclTrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'MarketFeeCatExclTrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'MarketFeeCatExclTrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'SettlementConfig', 'MarketFeeCatExclTrk', 1);

insert into SettlementConfigMarketFeeCatExclTrk1(
file_log_id,
[marketfeeid],
[effectivedate],
[version_datetime],
[lastchanged]
)
select 
(select h.id from @header h),
d.[marketfeeid],
d.[effectivedate],
d.[version_datetime],
d.[lastchanged]
from openjson(@data) with (
[marketfeeid] varchar(20),
[effectivedate] datetime2,
[version_datetime] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementConfigMarketfeetrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'Marketfeetrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'Marketfeetrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'SettlementConfig', 'Marketfeetrk', 1);

insert into SettlementConfigMarketfeetrk1(
file_log_id,
[marketfeeversionno],
[effectivedate],
[authorisedby],
[authoriseddate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[marketfeeversionno],
d.[effectivedate],
d.[authorisedby],
d.[authoriseddate],
d.[lastchanged]
from openjson(@data) with (
[marketfeeversionno] decimal(3,0),
[effectivedate] datetime2,
[authorisedby] varchar(15),
[authoriseddate] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementConfigSetcfgParticipantMpf1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'SetcfgParticipantMpf'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'SetcfgParticipantMpf'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'SettlementConfig', 'SetcfgParticipantMpf', 1);

insert into SettlementConfigSetcfgParticipantMpf1(
file_log_id,
[participantid],
[effectivedate],
[versionno],
[participantcategoryid],
[connectionpointid],
[mpf],
[lastchanged]
)
select 
(select h.id from @header h),
d.[participantid],
d.[effectivedate],
d.[versionno],
d.[participantcategoryid],
d.[connectionpointid],
d.[mpf],
d.[lastchanged]
from openjson(@data) with (
[participantid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[participantcategoryid] varchar(10),
[connectionpointid] varchar(10),
[mpf] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementConfigMarketfee1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'Marketfee'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'Marketfee'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'SettlementConfig', 'Marketfee', 1);

insert into SettlementConfigMarketfee1(
file_log_id,
[marketfeeid],
[marketfeeperiod],
[marketfeetype],
[description],
[lastchanged],
[gl_tcode],
[gl_financialcode],
[fee_class]
)
select 
(select h.id from @header h),
d.[marketfeeid],
d.[marketfeeperiod],
d.[marketfeetype],
d.[description],
d.[lastchanged],
d.[gl_tcode],
d.[gl_financialcode],
d.[fee_class]
from openjson(@data) with (
[marketfeeid] varchar(10),
[marketfeeperiod] varchar(20),
[marketfeetype] varchar(12),
[description] varchar(64),
[lastchanged] datetime2,
[gl_tcode] varchar(15),
[gl_financialcode] varchar(10),
[fee_class] varchar(40)
) d
end
go
create or alter procedure InsertSettlementConfigMarketFeeExclusionTrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'MarketFeeExclusionTrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'MarketFeeExclusionTrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'SettlementConfig', 'MarketFeeExclusionTrk', 1);

insert into SettlementConfigMarketFeeExclusionTrk1(
file_log_id,
[participantid],
[effectivedate],
[versionno],
[authorisedby],
[authoriseddate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[participantid],
d.[effectivedate],
d.[versionno],
d.[authorisedby],
d.[authoriseddate],
d.[lastchanged]
from openjson(@data) with (
[participantid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[authorisedby] varchar(15),
[authoriseddate] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSetcfgReallocationinterval1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Setcfg'
    and sub_type = 'Reallocationinterval'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Setcfg'
    and sub_type = 'Reallocationinterval'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Setcfg', 'Reallocationinterval', 1);

insert into SetcfgReallocationinterval1(
file_log_id,
[reallocationid],
[periodid],
[value],
[lastchanged],
[nrp]
)
select 
(select h.id from @header h),
d.[reallocationid],
d.[periodid],
d.[value],
d.[lastchanged],
d.[nrp]
from openjson(@data) with (
[reallocationid] varchar(20),
[periodid] decimal(3,0),
[value] decimal(15,5),
[lastchanged] datetime2,
[nrp] decimal(15,5)
) d
end
go
create or alter procedure InsertSettlementConfigMarketfeedata1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'Marketfeedata'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'Marketfeedata'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'SettlementConfig', 'Marketfeedata', 1);

insert into SettlementConfigMarketfeedata1(
file_log_id,
[marketfeeid],
[marketfeeversionno],
[effectivedate],
[marketfeevalue],
[lastchanged]
)
select 
(select h.id from @header h),
d.[marketfeeid],
d.[marketfeeversionno],
d.[effectivedate],
d.[marketfeevalue],
d.[lastchanged]
from openjson(@data) with (
[marketfeeid] varchar(10),
[marketfeeversionno] decimal(3,0),
[effectivedate] datetime2,
[marketfeevalue] decimal(22,8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementConfigParticipantBandfeeAlloc1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'ParticipantBandfeeAlloc'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'ParticipantBandfeeAlloc'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'SettlementConfig', 'ParticipantBandfeeAlloc', 1);

insert into SettlementConfigParticipantBandfeeAlloc1(
file_log_id,
[participantid],
[marketfeeid],
[effectivedate],
[versionno],
[participantcategoryid],
[marketfeevalue],
[lastchanged]
)
select 
(select h.id from @header h),
d.[participantid],
d.[marketfeeid],
d.[effectivedate],
d.[versionno],
d.[participantcategoryid],
d.[marketfeevalue],
d.[lastchanged]
from openjson(@data) with (
[participantid] varchar(10),
[marketfeeid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[participantcategoryid] varchar(10),
[marketfeevalue] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementConfigSetcfgParticipantMpftrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'SetcfgParticipantMpftrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'SetcfgParticipantMpftrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'SettlementConfig', 'SetcfgParticipantMpftrk', 1);

insert into SettlementConfigSetcfgParticipantMpftrk1(
file_log_id,
[participantid],
[effectivedate],
[versionno],
[authorisedby],
[authoriseddate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[participantid],
d.[effectivedate],
d.[versionno],
d.[authorisedby],
d.[authoriseddate],
d.[lastchanged]
from openjson(@data) with (
[participantid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[authorisedby] varchar(15),
[authoriseddate] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSettlementConfigMarketFeeCatExcl1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'MarketFeeCatExcl'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'MarketFeeCatExcl'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'SettlementConfig', 'MarketFeeCatExcl', 1);

insert into SettlementConfigMarketFeeCatExcl1(
file_log_id,
[marketfeeid],
[effectivedate],
[version_datetime],
[participant_categoryid]
)
select 
(select h.id from @header h),
d.[marketfeeid],
d.[effectivedate],
d.[version_datetime],
d.[participant_categoryid]
from openjson(@data) with (
[marketfeeid] varchar(20),
[effectivedate] datetime2,
[version_datetime] datetime2,
[participant_categoryid] varchar(20)
) d
end
go
create or alter procedure InsertSettlementConfigMarketFeeExclusion1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'MarketFeeExclusion'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'MarketFeeExclusion'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'SettlementConfig', 'MarketFeeExclusion', 1);

insert into SettlementConfigMarketFeeExclusion1(
file_log_id,
[participantid],
[effectivedate],
[versionno],
[marketfeeid],
[lastchanged]
)
select 
(select h.id from @header h),
d.[participantid],
d.[effectivedate],
d.[versionno],
d.[marketfeeid],
d.[lastchanged]
from openjson(@data) with (
[participantid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[marketfeeid] varchar(10),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertSetcfgReallocation2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Setcfg'
    and sub_type = 'Reallocation'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Setcfg'
    and sub_type = 'Reallocation'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Setcfg', 'Reallocation', 2);

insert into SetcfgReallocation2(
file_log_id,
[reallocationid],
[creditparticipantid],
[debitparticipantid],
[regionid],
[agreementtype],
[creditreference],
[debitreference],
[lastchanged],
[startdate],
[enddate],
[current_stepid],
[daytype],
[reallocation_type],
[calendarid],
[intervallength]
)
select 
(select h.id from @header h),
d.[reallocationid],
d.[creditparticipantid],
d.[debitparticipantid],
d.[regionid],
d.[agreementtype],
d.[creditreference],
d.[debitreference],
d.[lastchanged],
d.[startdate],
d.[enddate],
d.[current_stepid],
d.[daytype],
d.[reallocation_type],
d.[calendarid],
d.[intervallength]
from openjson(@data) with (
[reallocationid] varchar(20),
[creditparticipantid] varchar(10),
[debitparticipantid] varchar(10),
[regionid] varchar(10),
[agreementtype] varchar(10),
[creditreference] varchar(400),
[debitreference] varchar(400),
[lastchanged] datetime2,
[startdate] datetime2,
[enddate] datetime2,
[current_stepid] varchar(20),
[daytype] varchar(20),
[reallocation_type] varchar(1),
[calendarid] varchar(30),
[intervallength] decimal(3,0)
) d
end
go
create or alter procedure InsertSettlementConfigAncillaryRecoverySplit1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'AncillaryRecoverySplit'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'SettlementConfig'
    and sub_type = 'AncillaryRecoverySplit'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'SettlementConfig', 'AncillaryRecoverySplit', 1);

insert into SettlementConfigAncillaryRecoverySplit1(
file_log_id,
[effectivedate],
[versionno],
[service],
[paymenttype],
[customer_portion],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[versionno],
d.[service],
d.[paymenttype],
d.[customer_portion],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[versionno] decimal(3,0),
[service] varchar(10),
[paymenttype] varchar(20),
[customer_portion] decimal(8,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertPdpasaRegionsolution5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Pdpasa'
    and sub_type = 'Regionsolution'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Pdpasa'
    and sub_type = 'Regionsolution'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Pdpasa', 'Regionsolution', 5);

insert into PdpasaRegionsolution5(
file_log_id,
[run_datetime],
[interval_datetime],
[regionid],
[demand10],
[demand50],
[demand90],
[reservereq],
[capacityreq],
[energyreqdemand50],
[unconstrainedcapacity],
[constrainedcapacity],
[netinterchangeunderscarcity],
[surpluscapacity],
[surplusreserve],
[reservecondition],
[maxsurplusreserve],
[maxsparecapacity],
[lorcondition],
[aggregatecapacityavailable],
[aggregatescheduledload],
[lastchanged],
[aggregatepasaavailability],
[runtype],
[energyreqdemand10],
[calculatedlor1level],
[calculatedlor2level],
[msrnetinterchangeunderscarcity],
[lornetinterchangeunderscarcity],
[totalintermittentgeneration],
[demand_and_nonschedgen],
[uigf],
[semi_scheduled_capacity],
[lor_semi_scheduled_capacity],
[lcr],
[lcr2],
[fum],
[ss_solar_uigf],
[ss_wind_uigf],
[ss_solar_capacity],
[ss_wind_capacity],
[ss_solar_cleared],
[ss_wind_cleared]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[interval_datetime],
d.[regionid],
d.[demand10],
d.[demand50],
d.[demand90],
d.[reservereq],
d.[capacityreq],
d.[energyreqdemand50],
d.[unconstrainedcapacity],
d.[constrainedcapacity],
d.[netinterchangeunderscarcity],
d.[surpluscapacity],
d.[surplusreserve],
d.[reservecondition],
d.[maxsurplusreserve],
d.[maxsparecapacity],
d.[lorcondition],
d.[aggregatecapacityavailable],
d.[aggregatescheduledload],
d.[lastchanged],
d.[aggregatepasaavailability],
d.[runtype],
d.[energyreqdemand10],
d.[calculatedlor1level],
d.[calculatedlor2level],
d.[msrnetinterchangeunderscarcity],
d.[lornetinterchangeunderscarcity],
d.[totalintermittentgeneration],
d.[demand_and_nonschedgen],
d.[uigf],
d.[semi_scheduled_capacity],
d.[lor_semi_scheduled_capacity],
d.[lcr],
d.[lcr2],
d.[fum],
d.[ss_solar_uigf],
d.[ss_wind_uigf],
d.[ss_solar_capacity],
d.[ss_wind_capacity],
d.[ss_solar_cleared],
d.[ss_wind_cleared]
from openjson(@data) with (
[run_datetime] datetime2,
[interval_datetime] datetime2,
[regionid] varchar(10),
[demand10] decimal(12,2),
[demand50] decimal(12,2),
[demand90] decimal(12,2),
[reservereq] decimal(12,2),
[capacityreq] decimal(12,2),
[energyreqdemand50] decimal(12,2),
[unconstrainedcapacity] decimal(12,0),
[constrainedcapacity] decimal(12,0),
[netinterchangeunderscarcity] decimal(12,2),
[surpluscapacity] decimal(12,2),
[surplusreserve] decimal(12,2),
[reservecondition] decimal(1,0),
[maxsurplusreserve] decimal(12,2),
[maxsparecapacity] decimal(12,2),
[lorcondition] decimal(1,0),
[aggregatecapacityavailable] decimal(12,2),
[aggregatescheduledload] decimal(12,2),
[lastchanged] datetime2,
[aggregatepasaavailability] decimal(12,0),
[runtype] varchar(20),
[energyreqdemand10] decimal(12,2),
[calculatedlor1level] decimal(16,6),
[calculatedlor2level] decimal(16,6),
[msrnetinterchangeunderscarcity] decimal(12,2),
[lornetinterchangeunderscarcity] decimal(12,2),
[totalintermittentgeneration] decimal(15,5),
[demand_and_nonschedgen] decimal(15,5),
[uigf] decimal(12,2),
[semi_scheduled_capacity] decimal(12,2),
[lor_semi_scheduled_capacity] decimal(12,2),
[lcr] decimal(16,6),
[lcr2] decimal(16,6),
[fum] decimal(16,6),
[ss_solar_uigf] decimal(12,2),
[ss_wind_uigf] decimal(12,2),
[ss_solar_capacity] decimal(12,2),
[ss_wind_capacity] decimal(12,2),
[ss_solar_cleared] decimal(12,2),
[ss_wind_cleared] decimal(12,2)
) d
end
go
create or alter procedure InsertPdpasaCasesolution3
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Pdpasa'
    and sub_type = 'Casesolution'
    and version = '3'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Pdpasa'
    and sub_type = 'Casesolution'
    and version = '3'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Pdpasa', 'Casesolution', 3);

insert into PdpasaCasesolution3(
file_log_id,
[run_datetime],
[pasaversion],
[reservecondition],
[lorcondition],
[capacityobjfunction],
[capacityoption],
[maxsurplusreserveoption],
[maxsparecapacityoption],
[interconnectorflowpenalty],
[lastchanged],
[reliabilitylrcdemandoption],
[outagelrcdemandoption],
[lordemandoption],
[reliabilitylrccapacityoption],
[outagelrccapacityoption],
[lorcapacityoption],
[loruigf_option],
[reliability_lrcuigf_option],
[outage_lrcuigf_option]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[pasaversion],
d.[reservecondition],
d.[lorcondition],
d.[capacityobjfunction],
d.[capacityoption],
d.[maxsurplusreserveoption],
d.[maxsparecapacityoption],
d.[interconnectorflowpenalty],
d.[lastchanged],
d.[reliabilitylrcdemandoption],
d.[outagelrcdemandoption],
d.[lordemandoption],
d.[reliabilitylrccapacityoption],
d.[outagelrccapacityoption],
d.[lorcapacityoption],
d.[loruigf_option],
d.[reliability_lrcuigf_option],
d.[outage_lrcuigf_option]
from openjson(@data) with (
[run_datetime] datetime2,
[pasaversion] varchar(10),
[reservecondition] decimal(1,0),
[lorcondition] decimal(1,0),
[capacityobjfunction] decimal(12,3),
[capacityoption] decimal(12,3),
[maxsurplusreserveoption] decimal(12,3),
[maxsparecapacityoption] decimal(12,3),
[interconnectorflowpenalty] decimal(12,3),
[lastchanged] datetime2,
[reliabilitylrcdemandoption] decimal(12,3),
[outagelrcdemandoption] decimal(12,3),
[lordemandoption] decimal(12,3),
[reliabilitylrccapacityoption] varchar(10),
[outagelrccapacityoption] varchar(10),
[lorcapacityoption] varchar(10),
[loruigf_option] decimal(3,0),
[reliability_lrcuigf_option] decimal(3,0),
[outage_lrcuigf_option] decimal(3,0)
) d
end
go
create or alter procedure InsertPredispatchScenarioDemand1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'ScenarioDemand'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'ScenarioDemand'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Predispatch', 'ScenarioDemand', 1);

insert into PredispatchScenarioDemand1(
file_log_id,
[effectivedate],
[versionno],
[scenario],
[regionid],
[deltamw]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[versionno],
d.[scenario],
d.[regionid],
d.[deltamw]
from openjson(@data) with (
[effectivedate] datetime2,
[versionno] decimal(3,0),
[scenario] decimal(2,0),
[regionid] varchar(20),
[deltamw] decimal(4,0)
) d
end
go
create or alter procedure InsertPredispatchRegionSolution4
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'RegionSolution'
    and version = '4'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'RegionSolution'
    and version = '4'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Predispatch', 'RegionSolution', 4);

insert into PredispatchRegionSolution4(
file_log_id,
[predispatchseqno],
[runno],
[regionid],
[periodid],
[intervention],
[totaldemand],
[availablegeneration],
[availableload],
[demandforecast],
[dispatchablegeneration],
[dispatchableload],
[netinterchange],
[excessgeneration],
[lower5mindispatch],
[lower5minimport],
[lower5minlocaldispatch],
[lower5minlocalprice],
[lower5minlocalreq],
[lower5minprice],
[lower5minreq],
[lower5minsupplyprice],
[lower60secdispatch],
[lower60secimport],
[lower60seclocaldispatch],
[lower60seclocalprice],
[lower60seclocalreq],
[lower60secprice],
[lower60secreq],
[lower60secsupplyprice],
[lower6secdispatch],
[lower6secimport],
[lower6seclocaldispatch],
[lower6seclocalprice],
[lower6seclocalreq],
[lower6secprice],
[lower6secreq],
[lower6secsupplyprice],
[raise5mindispatch],
[raise5minimport],
[raise5minlocaldispatch],
[raise5minlocalprice],
[raise5minlocalreq],
[raise5minprice],
[raise5minreq],
[raise5minsupplyprice],
[raise60secdispatch],
[raise60secimport],
[raise60seclocaldispatch],
[raise60seclocalprice],
[raise60seclocalreq],
[raise60secprice],
[raise60secreq],
[raise60secsupplyprice],
[raise6secdispatch],
[raise6secimport],
[raise6seclocaldispatch],
[raise6seclocalprice],
[raise6seclocalreq],
[raise6secprice],
[raise6secreq],
[raise6secsupplyprice],
[lastchanged],
[datetime],
[initialsupply],
[clearedsupply],
[lowerregimport],
[lowerreglocaldispatch],
[lowerreglocalreq],
[lowerregreq],
[raiseregimport],
[raisereglocaldispatch],
[raisereglocalreq],
[raiseregreq],
[raise5minlocalviolation],
[raisereglocalviolation],
[raise60seclocalviolation],
[raise6seclocalviolation],
[lower5minlocalviolation],
[lowerreglocalviolation],
[lower60seclocalviolation],
[lower6seclocalviolation],
[raise5minviolation],
[raiseregviolation],
[raise60secviolation],
[raise6secviolation],
[lower5minviolation],
[lowerregviolation],
[lower60secviolation],
[lower6secviolation],
[raise6secactualavailability],
[raise60secactualavailability],
[raise5minactualavailability],
[raiseregactualavailability],
[lower6secactualavailability],
[lower60secactualavailability],
[lower5minactualavailability],
[lowerregactualavailability],
[decavailability],
[lorsurplus],
[lrcsurplus],
[totalintermittentgeneration],
[demand_and_nonschedgen],
[uigf],
[semischedule_clearedmw],
[semischedule_compliancemw],
[ss_solar_uigf],
[ss_wind_uigf],
[ss_solar_clearedmw],
[ss_wind_clearedmw],
[ss_solar_compliancemw],
[ss_wind_compliancemw]
)
select 
(select h.id from @header h),
d.[predispatchseqno],
d.[runno],
d.[regionid],
d.[periodid],
d.[intervention],
d.[totaldemand],
d.[availablegeneration],
d.[availableload],
d.[demandforecast],
d.[dispatchablegeneration],
d.[dispatchableload],
d.[netinterchange],
d.[excessgeneration],
d.[lower5mindispatch],
d.[lower5minimport],
d.[lower5minlocaldispatch],
d.[lower5minlocalprice],
d.[lower5minlocalreq],
d.[lower5minprice],
d.[lower5minreq],
d.[lower5minsupplyprice],
d.[lower60secdispatch],
d.[lower60secimport],
d.[lower60seclocaldispatch],
d.[lower60seclocalprice],
d.[lower60seclocalreq],
d.[lower60secprice],
d.[lower60secreq],
d.[lower60secsupplyprice],
d.[lower6secdispatch],
d.[lower6secimport],
d.[lower6seclocaldispatch],
d.[lower6seclocalprice],
d.[lower6seclocalreq],
d.[lower6secprice],
d.[lower6secreq],
d.[lower6secsupplyprice],
d.[raise5mindispatch],
d.[raise5minimport],
d.[raise5minlocaldispatch],
d.[raise5minlocalprice],
d.[raise5minlocalreq],
d.[raise5minprice],
d.[raise5minreq],
d.[raise5minsupplyprice],
d.[raise60secdispatch],
d.[raise60secimport],
d.[raise60seclocaldispatch],
d.[raise60seclocalprice],
d.[raise60seclocalreq],
d.[raise60secprice],
d.[raise60secreq],
d.[raise60secsupplyprice],
d.[raise6secdispatch],
d.[raise6secimport],
d.[raise6seclocaldispatch],
d.[raise6seclocalprice],
d.[raise6seclocalreq],
d.[raise6secprice],
d.[raise6secreq],
d.[raise6secsupplyprice],
d.[lastchanged],
d.[datetime],
d.[initialsupply],
d.[clearedsupply],
d.[lowerregimport],
d.[lowerreglocaldispatch],
d.[lowerreglocalreq],
d.[lowerregreq],
d.[raiseregimport],
d.[raisereglocaldispatch],
d.[raisereglocalreq],
d.[raiseregreq],
d.[raise5minlocalviolation],
d.[raisereglocalviolation],
d.[raise60seclocalviolation],
d.[raise6seclocalviolation],
d.[lower5minlocalviolation],
d.[lowerreglocalviolation],
d.[lower60seclocalviolation],
d.[lower6seclocalviolation],
d.[raise5minviolation],
d.[raiseregviolation],
d.[raise60secviolation],
d.[raise6secviolation],
d.[lower5minviolation],
d.[lowerregviolation],
d.[lower60secviolation],
d.[lower6secviolation],
d.[raise6secactualavailability],
d.[raise60secactualavailability],
d.[raise5minactualavailability],
d.[raiseregactualavailability],
d.[lower6secactualavailability],
d.[lower60secactualavailability],
d.[lower5minactualavailability],
d.[lowerregactualavailability],
d.[decavailability],
d.[lorsurplus],
d.[lrcsurplus],
d.[totalintermittentgeneration],
d.[demand_and_nonschedgen],
d.[uigf],
d.[semischedule_clearedmw],
d.[semischedule_compliancemw],
d.[ss_solar_uigf],
d.[ss_wind_uigf],
d.[ss_solar_clearedmw],
d.[ss_wind_clearedmw],
d.[ss_solar_compliancemw],
d.[ss_wind_compliancemw]
from openjson(@data) with (
[predispatchseqno] varchar(20),
[runno] decimal(3,0),
[regionid] varchar(10),
[periodid] varchar(20),
[intervention] decimal(2,0),
[totaldemand] decimal(15,5),
[availablegeneration] decimal(15,5),
[availableload] decimal(15,5),
[demandforecast] decimal(15,5),
[dispatchablegeneration] decimal(15,5),
[dispatchableload] decimal(15,5),
[netinterchange] decimal(15,5),
[excessgeneration] decimal(15,5),
[lower5mindispatch] decimal(15,5),
[lower5minimport] decimal(15,5),
[lower5minlocaldispatch] decimal(15,5),
[lower5minlocalprice] decimal(15,5),
[lower5minlocalreq] decimal(15,5),
[lower5minprice] decimal(15,5),
[lower5minreq] decimal(15,5),
[lower5minsupplyprice] decimal(15,5),
[lower60secdispatch] decimal(15,5),
[lower60secimport] decimal(15,5),
[lower60seclocaldispatch] decimal(15,5),
[lower60seclocalprice] decimal(15,5),
[lower60seclocalreq] decimal(15,5),
[lower60secprice] decimal(15,5),
[lower60secreq] decimal(15,5),
[lower60secsupplyprice] decimal(15,5),
[lower6secdispatch] decimal(15,5),
[lower6secimport] decimal(15,5),
[lower6seclocaldispatch] decimal(15,5),
[lower6seclocalprice] decimal(15,5),
[lower6seclocalreq] decimal(15,5),
[lower6secprice] decimal(15,5),
[lower6secreq] decimal(15,5),
[lower6secsupplyprice] decimal(15,5),
[raise5mindispatch] decimal(15,5),
[raise5minimport] decimal(15,5),
[raise5minlocaldispatch] decimal(15,5),
[raise5minlocalprice] decimal(15,5),
[raise5minlocalreq] decimal(15,5),
[raise5minprice] decimal(15,5),
[raise5minreq] decimal(15,5),
[raise5minsupplyprice] decimal(15,5),
[raise60secdispatch] decimal(15,5),
[raise60secimport] decimal(15,5),
[raise60seclocaldispatch] decimal(15,5),
[raise60seclocalprice] decimal(15,5),
[raise60seclocalreq] decimal(15,5),
[raise60secprice] decimal(15,5),
[raise60secreq] decimal(15,5),
[raise60secsupplyprice] decimal(15,5),
[raise6secdispatch] decimal(15,5),
[raise6secimport] decimal(15,5),
[raise6seclocaldispatch] decimal(15,5),
[raise6seclocalprice] decimal(15,5),
[raise6seclocalreq] decimal(15,5),
[raise6secprice] decimal(15,5),
[raise6secreq] decimal(15,5),
[raise6secsupplyprice] decimal(15,5),
[lastchanged] datetime2,
[datetime] datetime2,
[initialsupply] decimal(15,5),
[clearedsupply] decimal(15,5),
[lowerregimport] decimal(15,5),
[lowerreglocaldispatch] decimal(15,5),
[lowerreglocalreq] decimal(15,5),
[lowerregreq] decimal(15,5),
[raiseregimport] decimal(15,5),
[raisereglocaldispatch] decimal(15,5),
[raisereglocalreq] decimal(15,5),
[raiseregreq] decimal(15,5),
[raise5minlocalviolation] decimal(15,5),
[raisereglocalviolation] decimal(15,5),
[raise60seclocalviolation] decimal(15,5),
[raise6seclocalviolation] decimal(15,5),
[lower5minlocalviolation] decimal(15,5),
[lowerreglocalviolation] decimal(15,5),
[lower60seclocalviolation] decimal(15,5),
[lower6seclocalviolation] decimal(15,5),
[raise5minviolation] decimal(15,5),
[raiseregviolation] decimal(15,5),
[raise60secviolation] decimal(15,5),
[raise6secviolation] decimal(15,5),
[lower5minviolation] decimal(15,5),
[lowerregviolation] decimal(15,5),
[lower60secviolation] decimal(15,5),
[lower6secviolation] decimal(15,5),
[raise6secactualavailability] decimal(16,6),
[raise60secactualavailability] decimal(16,6),
[raise5minactualavailability] decimal(16,6),
[raiseregactualavailability] decimal(16,6),
[lower6secactualavailability] decimal(16,6),
[lower60secactualavailability] decimal(16,6),
[lower5minactualavailability] decimal(16,6),
[lowerregactualavailability] decimal(16,6),
[decavailability] decimal(16,6),
[lorsurplus] decimal(16,6),
[lrcsurplus] decimal(16,6),
[totalintermittentgeneration] decimal(15,5),
[demand_and_nonschedgen] decimal(15,5),
[uigf] decimal(15,5),
[semischedule_clearedmw] decimal(15,5),
[semischedule_compliancemw] decimal(15,5),
[ss_solar_uigf] decimal(15,5),
[ss_wind_uigf] decimal(15,5),
[ss_solar_clearedmw] decimal(15,5),
[ss_wind_clearedmw] decimal(15,5),
[ss_solar_compliancemw] decimal(15,5),
[ss_wind_compliancemw] decimal(15,5)
) d
end
go
create or alter procedure InsertPredispatchMnspbidtrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'Mnspbidtrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'Mnspbidtrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Predispatch', 'Mnspbidtrk', 1);

insert into PredispatchMnspbidtrk1(
file_log_id,
[predispatchseqno],
[linkid],
[periodid],
[participantid],
[settlementdate],
[offerdate],
[versionno],
[datetime],
[lastchanged]
)
select 
(select h.id from @header h),
d.[predispatchseqno],
d.[linkid],
d.[periodid],
d.[participantid],
d.[settlementdate],
d.[offerdate],
d.[versionno],
d.[datetime],
d.[lastchanged]
from openjson(@data) with (
[predispatchseqno] varchar(20),
[linkid] varchar(10),
[periodid] varchar(20),
[participantid] varchar(10),
[settlementdate] datetime2,
[offerdate] datetime2,
[versionno] decimal(3,0),
[datetime] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertPredispatchConstraintSolution5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'ConstraintSolution'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'ConstraintSolution'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Predispatch', 'ConstraintSolution', 5);

insert into PredispatchConstraintSolution5(
file_log_id,
[predispatchseqno],
[runno],
[constraintid],
[periodid],
[intervention],
[rhs],
[marginalvalue],
[violationdegree],
[lastchanged],
[datetime],
[duid],
[genconid_effectivedate],
[genconid_versionno],
[lhs]
)
select 
(select h.id from @header h),
d.[predispatchseqno],
d.[runno],
d.[constraintid],
d.[periodid],
d.[intervention],
d.[rhs],
d.[marginalvalue],
d.[violationdegree],
d.[lastchanged],
d.[datetime],
d.[duid],
d.[genconid_effectivedate],
d.[genconid_versionno],
d.[lhs]
from openjson(@data) with (
[predispatchseqno] varchar(20),
[runno] decimal(3,0),
[constraintid] varchar(20),
[periodid] varchar(20),
[intervention] decimal(2,0),
[rhs] decimal(15,5),
[marginalvalue] decimal(15,5),
[violationdegree] decimal(15,5),
[lastchanged] datetime2,
[datetime] datetime2,
[duid] varchar(20),
[genconid_effectivedate] datetime2,
[genconid_versionno] decimal(22,0),
[lhs] decimal(15,5)
) d
end
go
create or alter procedure InsertPredispatchOffertrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'Offertrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'Offertrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Predispatch', 'Offertrk', 1);

insert into PredispatchOffertrk1(
file_log_id,
[predispatchseqno],
[duid],
[bidtype],
[periodid],
[bidsettlementdate],
[bidofferdate],
[datetime],
[lastchanged]
)
select 
(select h.id from @header h),
d.[predispatchseqno],
d.[duid],
d.[bidtype],
d.[periodid],
d.[bidsettlementdate],
d.[bidofferdate],
d.[datetime],
d.[lastchanged]
from openjson(@data) with (
[predispatchseqno] varchar(20),
[duid] varchar(10),
[bidtype] varchar(20),
[periodid] varchar(20),
[bidsettlementdate] datetime2,
[bidofferdate] datetime2,
[datetime] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertPredispatchPricesensitivities1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'Pricesensitivities'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'Pricesensitivities'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Predispatch', 'Pricesensitivities', 1);

insert into PredispatchPricesensitivities1(
file_log_id,
[predispatchseqno],
[runno],
[regionid],
[periodid],
[intervention],
[rrpeep1],
[rrpeep2],
[rrpeep3],
[rrpeep4],
[rrpeep5],
[rrpeep6],
[rrpeep7],
[rrpeep8],
[rrpeep9],
[rrpeep10],
[rrpeep11],
[rrpeep12],
[rrpeep13],
[rrpeep14],
[rrpeep15],
[rrpeep16],
[rrpeep17],
[rrpeep18],
[rrpeep19],
[rrpeep20],
[rrpeep21],
[rrpeep22],
[rrpeep23],
[rrpeep24],
[rrpeep25],
[rrpeep26],
[rrpeep27],
[rrpeep28],
[lastchanged],
[datetime],
[rrpeep29],
[rrpeep30],
[rrpeep31],
[rrpeep32],
[rrpeep33],
[rrpeep34],
[rrpeep35],
[intervention_active],
[rrpeep36],
[rrpeep37],
[rrpeep38],
[rrpeep39],
[rrpeep40],
[rrpeep41],
[rrpeep42],
[rrpeep43]
)
select 
(select h.id from @header h),
d.[predispatchseqno],
d.[runno],
d.[regionid],
d.[periodid],
d.[intervention],
d.[rrpeep1],
d.[rrpeep2],
d.[rrpeep3],
d.[rrpeep4],
d.[rrpeep5],
d.[rrpeep6],
d.[rrpeep7],
d.[rrpeep8],
d.[rrpeep9],
d.[rrpeep10],
d.[rrpeep11],
d.[rrpeep12],
d.[rrpeep13],
d.[rrpeep14],
d.[rrpeep15],
d.[rrpeep16],
d.[rrpeep17],
d.[rrpeep18],
d.[rrpeep19],
d.[rrpeep20],
d.[rrpeep21],
d.[rrpeep22],
d.[rrpeep23],
d.[rrpeep24],
d.[rrpeep25],
d.[rrpeep26],
d.[rrpeep27],
d.[rrpeep28],
d.[lastchanged],
d.[datetime],
d.[rrpeep29],
d.[rrpeep30],
d.[rrpeep31],
d.[rrpeep32],
d.[rrpeep33],
d.[rrpeep34],
d.[rrpeep35],
d.[intervention_active],
d.[rrpeep36],
d.[rrpeep37],
d.[rrpeep38],
d.[rrpeep39],
d.[rrpeep40],
d.[rrpeep41],
d.[rrpeep42],
d.[rrpeep43]
from openjson(@data) with (
[predispatchseqno] varchar(20),
[runno] decimal(3,0),
[regionid] varchar(10),
[periodid] varchar(20),
[intervention] decimal(2,0),
[rrpeep1] decimal(15,5),
[rrpeep2] decimal(15,5),
[rrpeep3] decimal(15,5),
[rrpeep4] decimal(15,5),
[rrpeep5] decimal(15,5),
[rrpeep6] decimal(15,5),
[rrpeep7] decimal(15,5),
[rrpeep8] decimal(15,5),
[rrpeep9] decimal(15,5),
[rrpeep10] decimal(15,5),
[rrpeep11] decimal(15,5),
[rrpeep12] decimal(15,5),
[rrpeep13] decimal(15,5),
[rrpeep14] decimal(15,5),
[rrpeep15] decimal(15,5),
[rrpeep16] decimal(15,5),
[rrpeep17] decimal(15,5),
[rrpeep18] decimal(15,5),
[rrpeep19] decimal(15,5),
[rrpeep20] decimal(15,5),
[rrpeep21] decimal(15,5),
[rrpeep22] decimal(15,5),
[rrpeep23] decimal(15,5),
[rrpeep24] decimal(15,5),
[rrpeep25] decimal(15,5),
[rrpeep26] decimal(15,5),
[rrpeep27] decimal(15,5),
[rrpeep28] decimal(15,5),
[lastchanged] datetime2,
[datetime] datetime2,
[rrpeep29] decimal(15,5),
[rrpeep30] decimal(15,5),
[rrpeep31] decimal(15,5),
[rrpeep32] decimal(15,5),
[rrpeep33] decimal(15,5),
[rrpeep34] decimal(15,5),
[rrpeep35] decimal(15,5),
[intervention_active] decimal(1,0),
[rrpeep36] decimal(15,5),
[rrpeep37] decimal(15,5),
[rrpeep38] decimal(15,5),
[rrpeep39] decimal(15,5),
[rrpeep40] decimal(15,5),
[rrpeep41] decimal(15,5),
[rrpeep42] decimal(15,5),
[rrpeep43] decimal(15,5)
) d
end
go
create or alter procedure InsertPredispatchInterconnectorSoln3
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'InterconnectorSoln'
    and version = '3'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'InterconnectorSoln'
    and version = '3'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Predispatch', 'InterconnectorSoln', 3);

insert into PredispatchInterconnectorSoln3(
file_log_id,
[predispatchseqno],
[runno],
[interconnectorid],
[periodid],
[intervention],
[meteredmwflow],
[mwflow],
[mwlosses],
[marginalvalue],
[violationdegree],
[lastchanged],
[datetime],
[exportlimit],
[importlimit],
[marginalloss],
[exportgenconid],
[importgenconid],
[fcasexportlimit],
[fcasimportlimit],
[local_price_adjustment_export],
[locally_constrained_export],
[local_price_adjustment_import],
[locally_constrained_import]
)
select 
(select h.id from @header h),
d.[predispatchseqno],
d.[runno],
d.[interconnectorid],
d.[periodid],
d.[intervention],
d.[meteredmwflow],
d.[mwflow],
d.[mwlosses],
d.[marginalvalue],
d.[violationdegree],
d.[lastchanged],
d.[datetime],
d.[exportlimit],
d.[importlimit],
d.[marginalloss],
d.[exportgenconid],
d.[importgenconid],
d.[fcasexportlimit],
d.[fcasimportlimit],
d.[local_price_adjustment_export],
d.[locally_constrained_export],
d.[local_price_adjustment_import],
d.[locally_constrained_import]
from openjson(@data) with (
[predispatchseqno] varchar(20),
[runno] decimal(3,0),
[interconnectorid] varchar(10),
[periodid] varchar(20),
[intervention] decimal(2,0),
[meteredmwflow] decimal(15,5),
[mwflow] decimal(15,5),
[mwlosses] decimal(15,5),
[marginalvalue] decimal(15,5),
[violationdegree] decimal(15,5),
[lastchanged] datetime2,
[datetime] datetime2,
[exportlimit] decimal(15,5),
[importlimit] decimal(15,5),
[marginalloss] decimal(15,5),
[exportgenconid] varchar(20),
[importgenconid] varchar(20),
[fcasexportlimit] decimal(15,5),
[fcasimportlimit] decimal(15,5),
[local_price_adjustment_export] decimal(10,2),
[locally_constrained_export] decimal(1,0),
[local_price_adjustment_import] decimal(10,2),
[locally_constrained_import] decimal(1,0)
) d
end
go
create or alter procedure InsertPredispatchLocalPrice1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'LocalPrice'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'LocalPrice'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Predispatch', 'LocalPrice', 1);

insert into PredispatchLocalPrice1(
file_log_id,
[predispatchseqno],
[datetime],
[duid],
[periodid],
[local_price_adjustment],
[locally_constrained],
[lastchanged]
)
select 
(select h.id from @header h),
d.[predispatchseqno],
d.[datetime],
d.[duid],
d.[periodid],
d.[local_price_adjustment],
d.[locally_constrained],
d.[lastchanged]
from openjson(@data) with (
[predispatchseqno] varchar(20),
[datetime] datetime2,
[duid] varchar(20),
[periodid] varchar(20),
[local_price_adjustment] decimal(10,2),
[locally_constrained] decimal(1,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertPredispatchRegionfcasrequirement2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'Regionfcasrequirement'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'Regionfcasrequirement'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Predispatch', 'Regionfcasrequirement', 2);

insert into PredispatchRegionfcasrequirement2(
file_log_id,
[predispatchseqno],
[runno],
[intervention],
[periodid],
[genconid],
[regionid],
[bidtype],
[genconeffectivedate],
[genconversionno],
[marginalvalue],
[datetime],
[lastchanged],
[base_cost],
[adjusted_cost],
[estimated_cmpf],
[estimated_crmpf],
[recovery_factor_cmpf],
[recovery_factor_crmpf]
)
select 
(select h.id from @header h),
d.[predispatchseqno],
d.[runno],
d.[intervention],
d.[periodid],
d.[genconid],
d.[regionid],
d.[bidtype],
d.[genconeffectivedate],
d.[genconversionno],
d.[marginalvalue],
d.[datetime],
d.[lastchanged],
d.[base_cost],
d.[adjusted_cost],
d.[estimated_cmpf],
d.[estimated_crmpf],
d.[recovery_factor_cmpf],
d.[recovery_factor_crmpf]
from openjson(@data) with (
[predispatchseqno] varchar(20),
[runno] decimal(3,0),
[intervention] decimal(2,0),
[periodid] varchar(20),
[genconid] varchar(20),
[regionid] varchar(10),
[bidtype] varchar(10),
[genconeffectivedate] datetime2,
[genconversionno] decimal(3,0),
[marginalvalue] decimal(16,6),
[datetime] datetime2,
[lastchanged] datetime2,
[base_cost] decimal(18,8),
[adjusted_cost] decimal(18,8),
[estimated_cmpf] decimal(18,8),
[estimated_crmpf] decimal(18,8),
[recovery_factor_cmpf] decimal(18,8),
[recovery_factor_crmpf] decimal(18,8)
) d
end
go
create or alter procedure InsertPredispatchRegionPrices1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'RegionPrices'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'RegionPrices'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Predispatch', 'RegionPrices', 1);

insert into PredispatchRegionPrices1(
file_log_id,
[predispatchseqno],
[runno],
[regionid],
[periodid],
[intervention],
[rrp],
[eep],
[rrp1],
[eep1],
[rrp2],
[eep2],
[rrp3],
[eep3],
[rrp4],
[eep4],
[rrp5],
[eep5],
[rrp6],
[eep6],
[rrp7],
[eep7],
[rrp8],
[eep8],
[lastchanged],
[datetime],
[raise6secrrp],
[raise60secrrp],
[raise5minrrp],
[raiseregrrp],
[lower6secrrp],
[lower60secrrp],
[lower5minrrp],
[lowerregrrp]
)
select 
(select h.id from @header h),
d.[predispatchseqno],
d.[runno],
d.[regionid],
d.[periodid],
d.[intervention],
d.[rrp],
d.[eep],
d.[rrp1],
d.[eep1],
d.[rrp2],
d.[eep2],
d.[rrp3],
d.[eep3],
d.[rrp4],
d.[eep4],
d.[rrp5],
d.[eep5],
d.[rrp6],
d.[eep6],
d.[rrp7],
d.[eep7],
d.[rrp8],
d.[eep8],
d.[lastchanged],
d.[datetime],
d.[raise6secrrp],
d.[raise60secrrp],
d.[raise5minrrp],
d.[raiseregrrp],
d.[lower6secrrp],
d.[lower60secrrp],
d.[lower5minrrp],
d.[lowerregrrp]
from openjson(@data) with (
[predispatchseqno] varchar(20),
[runno] decimal(3,0),
[regionid] varchar(10),
[periodid] varchar(20),
[intervention] decimal(2,0),
[rrp] decimal(15,5),
[eep] decimal(15,5),
[rrp1] decimal(15,5),
[eep1] decimal(15,5),
[rrp2] decimal(15,5),
[eep2] decimal(15,5),
[rrp3] decimal(15,5),
[eep3] decimal(15,5),
[rrp4] decimal(15,5),
[eep4] decimal(15,5),
[rrp5] decimal(15,5),
[eep5] decimal(15,5),
[rrp6] decimal(15,5),
[eep6] decimal(15,5),
[rrp7] decimal(15,5),
[eep7] decimal(15,5),
[rrp8] decimal(15,5),
[eep8] decimal(15,5),
[lastchanged] datetime2,
[datetime] datetime2,
[raise6secrrp] decimal(15,5),
[raise60secrrp] decimal(15,5),
[raise5minrrp] decimal(15,5),
[raiseregrrp] decimal(15,5),
[lower6secrrp] decimal(15,5),
[lower60secrrp] decimal(15,5),
[lower5minrrp] decimal(15,5),
[lowerregrrp] decimal(15,5)
) d
end
go
create or alter procedure InsertPredispatchUnitSolution2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'UnitSolution'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'UnitSolution'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Predispatch', 'UnitSolution', 2);

insert into PredispatchUnitSolution2(
file_log_id,
[predispatchseqno],
[runno],
[duid],
[tradetype],
[periodid],
[intervention],
[connectionpointid],
[agcstatus],
[dispatchmode],
[initialmw],
[totalcleared],
[lower5min],
[lower60sec],
[lower6sec],
[raise5min],
[raise60sec],
[raise6sec],
[rampdownrate],
[rampuprate],
[downepf],
[upepf],
[marginal5minvalue],
[marginal60secvalue],
[marginal6secvalue],
[marginalvalue],
[violation5mindegree],
[violation60secdegree],
[violation6secdegree],
[violationdegree],
[lastchanged],
[datetime],
[lowerreg],
[raisereg],
[availability],
[raise6secflags],
[raise60secflags],
[raise5minflags],
[raiseregflags],
[lower6secflags],
[lower60secflags],
[lower5minflags],
[lowerregflags],
[raise6secactualavailability],
[raise60secactualavailability],
[raise5minactualavailability],
[raiseregactualavailability],
[lower6secactualavailability],
[lower60secactualavailability],
[lower5minactualavailability],
[lowerregactualavailability],
[semidispatchcap]
)
select 
(select h.id from @header h),
d.[predispatchseqno],
d.[runno],
d.[duid],
d.[tradetype],
d.[periodid],
d.[intervention],
d.[connectionpointid],
d.[agcstatus],
d.[dispatchmode],
d.[initialmw],
d.[totalcleared],
d.[lower5min],
d.[lower60sec],
d.[lower6sec],
d.[raise5min],
d.[raise60sec],
d.[raise6sec],
d.[rampdownrate],
d.[rampuprate],
d.[downepf],
d.[upepf],
d.[marginal5minvalue],
d.[marginal60secvalue],
d.[marginal6secvalue],
d.[marginalvalue],
d.[violation5mindegree],
d.[violation60secdegree],
d.[violation6secdegree],
d.[violationdegree],
d.[lastchanged],
d.[datetime],
d.[lowerreg],
d.[raisereg],
d.[availability],
d.[raise6secflags],
d.[raise60secflags],
d.[raise5minflags],
d.[raiseregflags],
d.[lower6secflags],
d.[lower60secflags],
d.[lower5minflags],
d.[lowerregflags],
d.[raise6secactualavailability],
d.[raise60secactualavailability],
d.[raise5minactualavailability],
d.[raiseregactualavailability],
d.[lower6secactualavailability],
d.[lower60secactualavailability],
d.[lower5minactualavailability],
d.[lowerregactualavailability],
d.[semidispatchcap]
from openjson(@data) with (
[predispatchseqno] varchar(20),
[runno] decimal(3,0),
[duid] varchar(10),
[tradetype] decimal(2,0),
[periodid] varchar(20),
[intervention] decimal(2,0),
[connectionpointid] varchar(12),
[agcstatus] decimal(2,0),
[dispatchmode] decimal(2,0),
[initialmw] decimal(15,5),
[totalcleared] decimal(15,5),
[lower5min] decimal(15,5),
[lower60sec] decimal(15,5),
[lower6sec] decimal(15,5),
[raise5min] decimal(15,5),
[raise60sec] decimal(15,5),
[raise6sec] decimal(15,5),
[rampdownrate] decimal(15,5),
[rampuprate] decimal(15,5),
[downepf] decimal(15,5),
[upepf] decimal(15,5),
[marginal5minvalue] decimal(15,5),
[marginal60secvalue] decimal(15,5),
[marginal6secvalue] decimal(15,5),
[marginalvalue] decimal(15,5),
[violation5mindegree] decimal(15,5),
[violation60secdegree] decimal(15,5),
[violation6secdegree] decimal(15,5),
[violationdegree] decimal(15,5),
[lastchanged] datetime2,
[datetime] datetime2,
[lowerreg] decimal(15,5),
[raisereg] decimal(15,5),
[availability] decimal(15,5),
[raise6secflags] decimal(3,0),
[raise60secflags] decimal(3,0),
[raise5minflags] decimal(3,0),
[raiseregflags] decimal(3,0),
[lower6secflags] decimal(3,0),
[lower60secflags] decimal(3,0),
[lower5minflags] decimal(3,0),
[lowerregflags] decimal(3,0),
[raise6secactualavailability] decimal(16,6),
[raise60secactualavailability] decimal(16,6),
[raise5minactualavailability] decimal(16,6),
[raiseregactualavailability] decimal(16,6),
[lower6secactualavailability] decimal(16,6),
[lower60secactualavailability] decimal(16,6),
[lower5minactualavailability] decimal(16,6),
[lowerregactualavailability] decimal(16,6),
[semidispatchcap] decimal(3,0)
) d
end
go
create or alter procedure InsertPredispatchScenarioDemandTrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'ScenarioDemandTrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'ScenarioDemandTrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Predispatch', 'ScenarioDemandTrk', 1);

insert into PredispatchScenarioDemandTrk1(
file_log_id,
[effectivedate],
[versionno],
[authorisedby],
[authoriseddate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[versionno],
d.[authorisedby],
d.[authoriseddate],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[versionno] decimal(3,0),
[authorisedby] varchar(15),
[authoriseddate] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertPredispatchCaseSolution1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'CaseSolution'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'CaseSolution'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Predispatch', 'CaseSolution', 1);

insert into PredispatchCaseSolution1(
file_log_id,
[predispatchseqno],
[runno],
[solutionstatus],
[spdversion],
[nonphysicallosses],
[totalobjective],
[totalareagenviolation],
[totalinterconnectorviolation],
[totalgenericviolation],
[totalramprateviolation],
[totalunitmwcapacityviolation],
[total5minviolation],
[totalregviolation],
[total6secviolation],
[total60secviolation],
[totalasprofileviolation],
[totalenergyconstrviolation],
[totalenergyofferviolation],
[lastchanged],
[intervention]
)
select 
(select h.id from @header h),
d.[predispatchseqno],
d.[runno],
d.[solutionstatus],
d.[spdversion],
d.[nonphysicallosses],
d.[totalobjective],
d.[totalareagenviolation],
d.[totalinterconnectorviolation],
d.[totalgenericviolation],
d.[totalramprateviolation],
d.[totalunitmwcapacityviolation],
d.[total5minviolation],
d.[totalregviolation],
d.[total6secviolation],
d.[total60secviolation],
d.[totalasprofileviolation],
d.[totalenergyconstrviolation],
d.[totalenergyofferviolation],
d.[lastchanged],
d.[intervention]
from openjson(@data) with (
[predispatchseqno] varchar(20),
[runno] decimal(3,0),
[solutionstatus] decimal(2,0),
[spdversion] varchar(20),
[nonphysicallosses] decimal(1,0),
[totalobjective] decimal(27,10),
[totalareagenviolation] decimal(15,5),
[totalinterconnectorviolation] decimal(15,5),
[totalgenericviolation] decimal(15,5),
[totalramprateviolation] decimal(15,5),
[totalunitmwcapacityviolation] decimal(15,5),
[total5minviolation] decimal(15,5),
[totalregviolation] decimal(15,5),
[total6secviolation] decimal(15,5),
[total60secviolation] decimal(15,5),
[totalasprofileviolation] decimal(15,5),
[totalenergyconstrviolation] decimal(15,5),
[totalenergyofferviolation] decimal(15,5),
[lastchanged] datetime2,
[intervention] decimal(2,0)
) d
end
go
create or alter procedure InsertPredispatchBlockedConstraints1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'BlockedConstraints'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'BlockedConstraints'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Predispatch', 'BlockedConstraints', 1);

insert into PredispatchBlockedConstraints1(
file_log_id,
[predispatchseqno],
[constraintid]
)
select 
(select h.id from @header h),
d.[predispatchseqno],
d.[constraintid]
from openjson(@data) with (
[predispatchseqno] varchar(20),
[constraintid] varchar(20)
) d
end
go
create or alter procedure InsertPredispatchInterconnectrSens1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'InterconnectrSens'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Predispatch'
    and sub_type = 'InterconnectrSens'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Predispatch', 'InterconnectrSens', 1);

insert into PredispatchInterconnectrSens1(
file_log_id,
[predispatchseqno],
[runno],
[interconnectorid],
[periodid],
[intervention],
[datetime],
[intervention_active],
[mwflow1],
[mwflow2],
[mwflow3],
[mwflow4],
[mwflow5],
[mwflow6],
[mwflow7],
[mwflow8],
[mwflow9],
[mwflow10],
[mwflow11],
[mwflow12],
[mwflow13],
[mwflow14],
[mwflow15],
[mwflow16],
[mwflow17],
[mwflow18],
[mwflow19],
[mwflow20],
[mwflow21],
[mwflow22],
[mwflow23],
[mwflow24],
[mwflow25],
[mwflow26],
[mwflow27],
[mwflow28],
[mwflow29],
[mwflow30],
[mwflow31],
[mwflow32],
[mwflow33],
[mwflow34],
[mwflow35],
[mwflow36],
[mwflow37],
[mwflow38],
[mwflow39],
[mwflow40],
[mwflow41],
[mwflow42],
[mwflow43],
[lastchanged]
)
select 
(select h.id from @header h),
d.[predispatchseqno],
d.[runno],
d.[interconnectorid],
d.[periodid],
d.[intervention],
d.[datetime],
d.[intervention_active],
d.[mwflow1],
d.[mwflow2],
d.[mwflow3],
d.[mwflow4],
d.[mwflow5],
d.[mwflow6],
d.[mwflow7],
d.[mwflow8],
d.[mwflow9],
d.[mwflow10],
d.[mwflow11],
d.[mwflow12],
d.[mwflow13],
d.[mwflow14],
d.[mwflow15],
d.[mwflow16],
d.[mwflow17],
d.[mwflow18],
d.[mwflow19],
d.[mwflow20],
d.[mwflow21],
d.[mwflow22],
d.[mwflow23],
d.[mwflow24],
d.[mwflow25],
d.[mwflow26],
d.[mwflow27],
d.[mwflow28],
d.[mwflow29],
d.[mwflow30],
d.[mwflow31],
d.[mwflow32],
d.[mwflow33],
d.[mwflow34],
d.[mwflow35],
d.[mwflow36],
d.[mwflow37],
d.[mwflow38],
d.[mwflow39],
d.[mwflow40],
d.[mwflow41],
d.[mwflow42],
d.[mwflow43],
d.[lastchanged]
from openjson(@data) with (
[predispatchseqno] varchar(20),
[runno] decimal(3,0),
[interconnectorid] varchar(10),
[periodid] varchar(20),
[intervention] decimal(2,0),
[datetime] datetime2,
[intervention_active] decimal(1,0),
[mwflow1] decimal(15,5),
[mwflow2] decimal(15,5),
[mwflow3] decimal(15,5),
[mwflow4] decimal(15,5),
[mwflow5] decimal(15,5),
[mwflow6] decimal(15,5),
[mwflow7] decimal(15,5),
[mwflow8] decimal(15,5),
[mwflow9] decimal(15,5),
[mwflow10] decimal(15,5),
[mwflow11] decimal(15,5),
[mwflow12] decimal(15,5),
[mwflow13] decimal(15,5),
[mwflow14] decimal(15,5),
[mwflow15] decimal(15,5),
[mwflow16] decimal(15,5),
[mwflow17] decimal(15,5),
[mwflow18] decimal(15,5),
[mwflow19] decimal(15,5),
[mwflow20] decimal(15,5),
[mwflow21] decimal(15,5),
[mwflow22] decimal(15,5),
[mwflow23] decimal(15,5),
[mwflow24] decimal(15,5),
[mwflow25] decimal(15,5),
[mwflow26] decimal(15,5),
[mwflow27] decimal(15,5),
[mwflow28] decimal(15,5),
[mwflow29] decimal(15,5),
[mwflow30] decimal(15,5),
[mwflow31] decimal(15,5),
[mwflow32] decimal(15,5),
[mwflow33] decimal(15,5),
[mwflow34] decimal(15,5),
[mwflow35] decimal(15,5),
[mwflow36] decimal(15,5),
[mwflow37] decimal(15,5),
[mwflow38] decimal(15,5),
[mwflow39] decimal(15,5),
[mwflow40] decimal(15,5),
[mwflow41] decimal(15,5),
[mwflow42] decimal(15,5),
[mwflow43] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertVoltageInstructionTrack2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'VoltageInstruction'
    and sub_type = 'Track'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'VoltageInstruction'
    and sub_type = 'Track'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'VoltageInstruction', 'Track', 2);

insert into VoltageInstructionTrack2(
file_log_id,
[run_datetime],
[file_type],
[version_datetime],
[se_datetime],
[solution_category],
[solution_status],
[operating_mode],
[operating_status],
[est_expiry],
[est_next_instruction]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[file_type],
d.[version_datetime],
d.[se_datetime],
d.[solution_category],
d.[solution_status],
d.[operating_mode],
d.[operating_status],
d.[est_expiry],
d.[est_next_instruction]
from openjson(@data) with (
[run_datetime] datetime2,
[file_type] varchar(20),
[version_datetime] datetime2,
[se_datetime] datetime2,
[solution_category] varchar(60),
[solution_status] varchar(60),
[operating_mode] varchar(60),
[operating_status] varchar(100),
[est_expiry] datetime2,
[est_next_instruction] datetime2
) d
end
go
create or alter procedure InsertVoltageInstructionInstruction2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'VoltageInstruction'
    and sub_type = 'Instruction'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'VoltageInstruction'
    and sub_type = 'Instruction'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'VoltageInstruction', 'Instruction', 2);

insert into VoltageInstructionInstruction2(
file_log_id,
[run_datetime],
[ems_id],
[participantid],
[station_id],
[device_id],
[device_type],
[control_type],
[target],
[conforming],
[instruction_summary],
[version_datetime],
[instruction_sequence],
[additional_notes]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[ems_id],
d.[participantid],
d.[station_id],
d.[device_id],
d.[device_type],
d.[control_type],
d.[target],
d.[conforming],
d.[instruction_summary],
d.[version_datetime],
d.[instruction_sequence],
d.[additional_notes]
from openjson(@data) with (
[run_datetime] datetime2,
[ems_id] varchar(60),
[participantid] varchar(20),
[station_id] varchar(60),
[device_id] varchar(60),
[device_type] varchar(20),
[control_type] varchar(20),
[target] decimal(15,0),
[conforming] decimal(1,0),
[instruction_summary] varchar(400),
[version_datetime] datetime2,
[instruction_sequence] decimal(4,0),
[additional_notes] varchar(60)
) d
end
go
create or alter procedure InsertBillingConfigBillingcalendar2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'BillingConfig'
    and sub_type = 'Billingcalendar'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'BillingConfig'
    and sub_type = 'Billingcalendar'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'BillingConfig', 'Billingcalendar', 2);

insert into BillingConfigBillingcalendar2(
file_log_id,
[contractyear],
[weekno],
[startdate],
[enddate],
[preliminarystatementdate],
[finalstatementdate],
[paymentdate],
[lastchanged],
[revision1_statementdate],
[revision2_statementdate]
)
select 
(select h.id from @header h),
d.[contractyear],
d.[weekno],
d.[startdate],
d.[enddate],
d.[preliminarystatementdate],
d.[finalstatementdate],
d.[paymentdate],
d.[lastchanged],
d.[revision1_statementdate],
d.[revision2_statementdate]
from openjson(@data) with (
[contractyear] decimal(4,0),
[weekno] decimal(3,0),
[startdate] datetime2,
[enddate] datetime2,
[preliminarystatementdate] datetime2,
[finalstatementdate] datetime2,
[paymentdate] datetime2,
[lastchanged] datetime2,
[revision1_statementdate] datetime2,
[revision2_statementdate] datetime2
) d
end
go
create or alter procedure InsertBillingConfigGstTransactionClass1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'BillingConfig'
    and sub_type = 'GstTransactionClass'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'BillingConfig'
    and sub_type = 'GstTransactionClass'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'BillingConfig', 'GstTransactionClass', 1);

insert into BillingConfigGstTransactionClass1(
file_log_id,
[effectivedate],
[versionno],
[transaction_type],
[bas_class],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[versionno],
d.[transaction_type],
d.[bas_class],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[versionno] decimal(3,0),
[transaction_type] varchar(30),
[bas_class] varchar(30),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingConfigGstBasClass1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'BillingConfig'
    and sub_type = 'GstBasClass'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'BillingConfig'
    and sub_type = 'GstBasClass'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'BillingConfig', 'GstBasClass', 1);

insert into BillingConfigGstBasClass1(
file_log_id,
[bas_class],
[description],
[lastchanged]
)
select 
(select h.id from @header h),
d.[bas_class],
d.[description],
d.[lastchanged]
from openjson(@data) with (
[bas_class] varchar(30),
[description] varchar(100),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingConfigSecdepositProvision1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'BillingConfig'
    and sub_type = 'SecdepositProvision'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'BillingConfig'
    and sub_type = 'SecdepositProvision'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'BillingConfig', 'SecdepositProvision', 1);

insert into BillingConfigSecdepositProvision1(
file_log_id,
[security_deposit_id],
[participantid],
[transaction_date],
[maturity_contractyear],
[maturity_weekno],
[amount],
[interest_rate],
[interest_calc_type],
[interest_acct_id]
)
select 
(select h.id from @header h),
d.[security_deposit_id],
d.[participantid],
d.[transaction_date],
d.[maturity_contractyear],
d.[maturity_weekno],
d.[amount],
d.[interest_rate],
d.[interest_calc_type],
d.[interest_acct_id]
from openjson(@data) with (
[security_deposit_id] varchar(20),
[participantid] varchar(20),
[transaction_date] datetime2,
[maturity_contractyear] decimal(4,0),
[maturity_weekno] decimal(3,0),
[amount] decimal(18,8),
[interest_rate] decimal(18,8),
[interest_calc_type] varchar(20),
[interest_acct_id] varchar(20)
) d
end
go
create or alter procedure InsertBillingConfigGstTransactionType1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'BillingConfig'
    and sub_type = 'GstTransactionType'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'BillingConfig'
    and sub_type = 'GstTransactionType'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'BillingConfig', 'GstTransactionType', 1);

insert into BillingConfigGstTransactionType1(
file_log_id,
[transaction_type],
[description],
[gl_financialcode],
[gl_tcode],
[lastchanged]
)
select 
(select h.id from @header h),
d.[transaction_type],
d.[description],
d.[gl_financialcode],
d.[gl_tcode],
d.[lastchanged]
from openjson(@data) with (
[transaction_type] varchar(30),
[description] varchar(100),
[gl_financialcode] varchar(10),
[gl_tcode] varchar(15),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertBillingConfigSecdepositInterestRate1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'BillingConfig'
    and sub_type = 'SecdepositInterestRate'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'BillingConfig'
    and sub_type = 'SecdepositInterestRate'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'BillingConfig', 'SecdepositInterestRate', 1);

insert into BillingConfigSecdepositInterestRate1(
file_log_id,
[interest_acct_id],
[effectivedate],
[version_datetime],
[interest_rate]
)
select 
(select h.id from @header h),
d.[interest_acct_id],
d.[effectivedate],
d.[version_datetime],
d.[interest_rate]
from openjson(@data) with (
[interest_acct_id] varchar(20),
[effectivedate] datetime2,
[version_datetime] datetime2,
[interest_rate] decimal(18,8)
) d
end
go
create or alter procedure InsertBillingConfigGstRate1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'BillingConfig'
    and sub_type = 'GstRate'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'BillingConfig'
    and sub_type = 'GstRate'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'BillingConfig', 'GstRate', 1);

insert into BillingConfigGstRate1(
file_log_id,
[effectivedate],
[versionno],
[bas_class],
[gst_rate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[versionno],
d.[bas_class],
d.[gst_rate],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[versionno] decimal(3,0),
[bas_class] varchar(30),
[gst_rate] decimal(8,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertForceMajeureOverriderrp1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ForceMajeure'
    and sub_type = 'Overriderrp'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ForceMajeure'
    and sub_type = 'Overriderrp'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ForceMajeure', 'Overriderrp', 1);

insert into ForceMajeureOverriderrp1(
file_log_id,
[regionid],
[startdate],
[startperiod],
[enddate],
[endperiod],
[rrp],
[description],
[authorisestart],
[authoriseend],
[lastchanged]
)
select 
(select h.id from @header h),
d.[regionid],
d.[startdate],
d.[startperiod],
d.[enddate],
d.[endperiod],
d.[rrp],
d.[description],
d.[authorisestart],
d.[authoriseend],
d.[lastchanged]
from openjson(@data) with (
[regionid] varchar(10),
[startdate] datetime2,
[startperiod] decimal(3,0),
[enddate] datetime2,
[endperiod] decimal(3,0),
[rrp] decimal(15,0),
[description] varchar(128),
[authorisestart] varchar(15),
[authoriseend] varchar(15),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertForceMajeureIrfmevents1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ForceMajeure'
    and sub_type = 'Irfmevents'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ForceMajeure'
    and sub_type = 'Irfmevents'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ForceMajeure', 'Irfmevents', 1);

insert into ForceMajeureIrfmevents1(
file_log_id,
[irfmid],
[startdate],
[startperiod],
[enddate],
[endperiod],
[lastchanged]
)
select 
(select h.id from @header h),
d.[irfmid],
d.[startdate],
d.[startperiod],
d.[enddate],
d.[endperiod],
d.[lastchanged]
from openjson(@data) with (
[irfmid] varchar(10),
[startdate] datetime2,
[startperiod] decimal(3,0),
[enddate] datetime2,
[endperiod] decimal(3,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertForceMajeureMarketSuspendRegionSum1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ForceMajeure'
    and sub_type = 'MarketSuspendRegionSum'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ForceMajeure'
    and sub_type = 'MarketSuspendRegionSum'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ForceMajeure', 'MarketSuspendRegionSum', 1);

insert into ForceMajeureMarketSuspendRegionSum1(
file_log_id,
[suspension_id],
[regionid],
[initial_interval],
[end_region_interval],
[end_suspension_interval],
[lastchanged]
)
select 
(select h.id from @header h),
d.[suspension_id],
d.[regionid],
d.[initial_interval],
d.[end_region_interval],
d.[end_suspension_interval],
d.[lastchanged]
from openjson(@data) with (
[suspension_id] varchar(20),
[regionid] varchar(20),
[initial_interval] datetime2,
[end_region_interval] datetime2,
[end_suspension_interval] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertForceMajeureMarketSuspendSchedule1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ForceMajeure'
    and sub_type = 'MarketSuspendSchedule'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ForceMajeure'
    and sub_type = 'MarketSuspendSchedule'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ForceMajeure', 'MarketSuspendSchedule', 1);

insert into ForceMajeureMarketSuspendSchedule1(
file_log_id,
[effectivedate],
[day_type],
[regionid],
[periodid],
[energy_rrp],
[r6_rrp],
[r60_rrp],
[r5_rrp],
[rreg_rrp],
[l6_rrp],
[l60_rrp],
[l5_rrp],
[lreg_rrp],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[day_type],
d.[regionid],
d.[periodid],
d.[energy_rrp],
d.[r6_rrp],
d.[r60_rrp],
d.[r5_rrp],
d.[rreg_rrp],
d.[l6_rrp],
d.[l60_rrp],
d.[l5_rrp],
d.[lreg_rrp],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[day_type] varchar(20),
[regionid] varchar(20),
[periodid] decimal(3,0),
[energy_rrp] decimal(15,5),
[r6_rrp] decimal(15,5),
[r60_rrp] decimal(15,5),
[r5_rrp] decimal(15,5),
[rreg_rrp] decimal(15,5),
[l6_rrp] decimal(15,5),
[l60_rrp] decimal(15,5),
[l5_rrp] decimal(15,5),
[lreg_rrp] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertApApevent1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Ap'
    and sub_type = 'Apevent'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Ap'
    and sub_type = 'Apevent'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Ap', 'Apevent', 1);

insert into ApApevent1(
file_log_id,
[apeventid],
[effectivefrominterval],
[effectivetointerval],
[reason],
[startauthorisedby],
[startauthoriseddate],
[endauthorisedby],
[endauthoriseddate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[apeventid],
d.[effectivefrominterval],
d.[effectivetointerval],
d.[reason],
d.[startauthorisedby],
d.[startauthoriseddate],
d.[endauthorisedby],
d.[endauthoriseddate],
d.[lastchanged]
from openjson(@data) with (
[apeventid] decimal(22,0),
[effectivefrominterval] datetime2,
[effectivetointerval] datetime2,
[reason] varchar(2000),
[startauthorisedby] varchar(15),
[startauthoriseddate] datetime2,
[endauthorisedby] varchar(15),
[endauthoriseddate] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertForceMajeureIrfmamount1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ForceMajeure'
    and sub_type = 'Irfmamount'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ForceMajeure'
    and sub_type = 'Irfmamount'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ForceMajeure', 'Irfmamount', 1);

insert into ForceMajeureIrfmamount1(
file_log_id,
[irfmid],
[effectivedate],
[versionno],
[periodid],
[amount],
[authorisedby],
[authoriseddate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[irfmid],
d.[effectivedate],
d.[versionno],
d.[periodid],
d.[amount],
d.[authorisedby],
d.[authoriseddate],
d.[lastchanged]
from openjson(@data) with (
[irfmid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[periodid] decimal(4,0),
[amount] decimal(15,5),
[authorisedby] varchar(15),
[authoriseddate] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertApApeventregion1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Ap'
    and sub_type = 'Apeventregion'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Ap'
    and sub_type = 'Apeventregion'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Ap', 'Apeventregion', 1);

insert into ApApeventregion1(
file_log_id,
[apeventid],
[regionid],
[lastchanged],
[energyapflag],
[raise6secapflag],
[raise60secapflag],
[raise5minapflag],
[raiseregapflag],
[lower6secapflag],
[lower60secapflag],
[lower5minapflag],
[lowerregapflag]
)
select 
(select h.id from @header h),
d.[apeventid],
d.[regionid],
d.[lastchanged],
d.[energyapflag],
d.[raise6secapflag],
d.[raise60secapflag],
d.[raise5minapflag],
d.[raiseregapflag],
d.[lower6secapflag],
d.[lower60secapflag],
d.[lower5minapflag],
d.[lowerregapflag]
from openjson(@data) with (
[apeventid] decimal(22,0),
[regionid] varchar(10),
[lastchanged] datetime2,
[energyapflag] decimal(1,0),
[raise6secapflag] decimal(1,0),
[raise60secapflag] decimal(1,0),
[raise5minapflag] decimal(1,0),
[raiseregapflag] decimal(1,0),
[lower6secapflag] decimal(1,0),
[lower60secapflag] decimal(1,0),
[lower5minapflag] decimal(1,0),
[lowerregapflag] decimal(1,0)
) d
end
go
create or alter procedure InsertForceMajeureMarketSuspendRegimeSum1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ForceMajeure'
    and sub_type = 'MarketSuspendRegimeSum'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ForceMajeure'
    and sub_type = 'MarketSuspendRegimeSum'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ForceMajeure', 'MarketSuspendRegimeSum', 1);

insert into ForceMajeureMarketSuspendRegimeSum1(
file_log_id,
[suspension_id],
[regionid],
[start_interval],
[end_interval],
[pricing_regime],
[lastchanged]
)
select 
(select h.id from @header h),
d.[suspension_id],
d.[regionid],
d.[start_interval],
d.[end_interval],
d.[pricing_regime],
d.[lastchanged]
from openjson(@data) with (
[suspension_id] varchar(20),
[regionid] varchar(20),
[start_interval] datetime2,
[end_interval] datetime2,
[pricing_regime] varchar(20),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertApRegionapcintervals1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Ap'
    and sub_type = 'Regionapcintervals'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Ap'
    and sub_type = 'Regionapcintervals'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Ap', 'Regionapcintervals', 1);

insert into ApRegionapcintervals1(
file_log_id,
[regionid],
[effectivedate],
[versionno],
[periodid],
[apcvalue],
[lastchanged],
[apctype],
[fcasapcvalue],
[apfvalue]
)
select 
(select h.id from @header h),
d.[regionid],
d.[effectivedate],
d.[versionno],
d.[periodid],
d.[apcvalue],
d.[lastchanged],
d.[apctype],
d.[fcasapcvalue],
d.[apfvalue]
from openjson(@data) with (
[regionid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[periodid] decimal(3,0),
[apcvalue] decimal(16,6),
[lastchanged] datetime2,
[apctype] decimal(3,0),
[fcasapcvalue] decimal(16,6),
[apfvalue] decimal(16,6)
) d
end
go
create or alter procedure InsertForceMajeureMarketSuspendScheduleTrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ForceMajeure'
    and sub_type = 'MarketSuspendScheduleTrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'ForceMajeure'
    and sub_type = 'MarketSuspendScheduleTrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'ForceMajeure', 'MarketSuspendScheduleTrk', 1);

insert into ForceMajeureMarketSuspendScheduleTrk1(
file_log_id,
[effectivedate],
[source_start_date],
[source_end_date],
[comments],
[authoriseddate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[source_start_date],
d.[source_end_date],
d.[comments],
d.[authoriseddate],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[source_start_date] datetime2,
[source_end_date] datetime2,
[comments] varchar(1000),
[authoriseddate] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertApRegionapc1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Ap'
    and sub_type = 'Regionapc'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Ap'
    and sub_type = 'Regionapc'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Ap', 'Regionapc', 1);

insert into ApRegionapc1(
file_log_id,
[regionid],
[effectivedate],
[versionno],
[authoriseddate],
[authorisedby],
[lastchanged]
)
select 
(select h.id from @header h),
d.[regionid],
d.[effectivedate],
d.[versionno],
d.[authoriseddate],
d.[authorisedby],
d.[lastchanged]
from openjson(@data) with (
[regionid] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[authoriseddate] datetime2,
[authorisedby] varchar(10),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMccCasesolution1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mcc'
    and sub_type = 'Casesolution'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mcc'
    and sub_type = 'Casesolution'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Mcc', 'Casesolution', 1);

insert into MccCasesolution1(
file_log_id,
[run_datetime]
)
select 
(select h.id from @header h),
d.[run_datetime]
from openjson(@data) with (
[run_datetime] datetime2
) d
end
go
create or alter procedure InsertMccConstraintsolution1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mcc'
    and sub_type = 'Constraintsolution'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Mcc'
    and sub_type = 'Constraintsolution'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Mcc', 'Constraintsolution', 1);

insert into MccConstraintsolution1(
file_log_id,
[run_datetime],
[constraintid],
[rhs],
[marginalvalue]
)
select 
(select h.id from @header h),
d.[run_datetime],
d.[constraintid],
d.[rhs],
d.[marginalvalue]
from openjson(@data) with (
[run_datetime] datetime2,
[constraintid] varchar(20),
[rhs] decimal(15,5),
[marginalvalue] decimal(15,5)
) d
end
go
create or alter procedure InsertNetworkOutagestatuscode1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Network'
    and sub_type = 'Outagestatuscode'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Network'
    and sub_type = 'Outagestatuscode'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Network', 'Outagestatuscode', 1);

insert into NetworkOutagestatuscode1(
file_log_id,
[outagestatuscode],
[description],
[lastchanged]
)
select 
(select h.id from @header h),
d.[outagestatuscode],
d.[description],
d.[lastchanged]
from openjson(@data) with (
[outagestatuscode] varchar(10),
[description] varchar(100),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertNetworkStaticrating1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Network'
    and sub_type = 'Staticrating'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Network'
    and sub_type = 'Staticrating'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Network', 'Staticrating', 1);

insert into NetworkStaticrating1(
file_log_id,
[substationid],
[equipmenttype],
[equipmentid],
[ratinglevel],
[applicationid],
[validfrom],
[validto],
[ratingvalue],
[lastchanged]
)
select 
(select h.id from @header h),
d.[substationid],
d.[equipmenttype],
d.[equipmentid],
d.[ratinglevel],
d.[applicationid],
d.[validfrom],
d.[validto],
d.[ratingvalue],
d.[lastchanged]
from openjson(@data) with (
[substationid] varchar(30),
[equipmenttype] varchar(10),
[equipmentid] varchar(30),
[ratinglevel] varchar(10),
[applicationid] varchar(20),
[validfrom] datetime2,
[validto] datetime2,
[ratingvalue] decimal(16,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertNetworkEquipmentdetail1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Network'
    and sub_type = 'Equipmentdetail'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Network'
    and sub_type = 'Equipmentdetail'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Network', 'Equipmentdetail', 1);

insert into NetworkEquipmentdetail1(
file_log_id,
[substationid],
[equipmenttype],
[equipmentid],
[validfrom],
[validto],
[voltage],
[description],
[lastchanged]
)
select 
(select h.id from @header h),
d.[substationid],
d.[equipmenttype],
d.[equipmentid],
d.[validfrom],
d.[validto],
d.[voltage],
d.[description],
d.[lastchanged]
from openjson(@data) with (
[substationid] varchar(30),
[equipmenttype] varchar(10),
[equipmentid] varchar(30),
[validfrom] datetime2,
[validto] datetime2,
[voltage] varchar(20),
[description] varchar(100),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertNetworkRating1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Network'
    and sub_type = 'Rating'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Network'
    and sub_type = 'Rating'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Network', 'Rating', 1);

insert into NetworkRating1(
file_log_id,
[spd_id],
[validfrom],
[validto],
[regionid],
[substationid],
[equipmenttype],
[equipmentid],
[ratinglevel],
[isdynamic],
[lastchanged]
)
select 
(select h.id from @header h),
d.[spd_id],
d.[validfrom],
d.[validto],
d.[regionid],
d.[substationid],
d.[equipmenttype],
d.[equipmentid],
d.[ratinglevel],
d.[isdynamic],
d.[lastchanged]
from openjson(@data) with (
[spd_id] varchar(21),
[validfrom] datetime2,
[validto] datetime2,
[regionid] varchar(10),
[substationid] varchar(30),
[equipmenttype] varchar(10),
[equipmentid] varchar(30),
[ratinglevel] varchar(10),
[isdynamic] decimal(1,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertNetworkOutageconstraintset1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Network'
    and sub_type = 'Outageconstraintset'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Network'
    and sub_type = 'Outageconstraintset'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Network', 'Outageconstraintset', 1);

insert into NetworkOutageconstraintset1(
file_log_id,
[outageid],
[genconsetid],
[startinterval],
[endinterval]
)
select 
(select h.id from @header h),
d.[outageid],
d.[genconsetid],
d.[startinterval],
d.[endinterval]
from openjson(@data) with (
[outageid] decimal(15,0),
[genconsetid] varchar(50),
[startinterval] datetime2,
[endinterval] datetime2
) d
end
go
create or alter procedure InsertNetworkSubstationdetail1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Network'
    and sub_type = 'Substationdetail'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Network'
    and sub_type = 'Substationdetail'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Network', 'Substationdetail', 1);

insert into NetworkSubstationdetail1(
file_log_id,
[substationid],
[validfrom],
[validto],
[description],
[regionid],
[ownerid],
[lastchanged]
)
select 
(select h.id from @header h),
d.[substationid],
d.[validfrom],
d.[validto],
d.[description],
d.[regionid],
d.[ownerid],
d.[lastchanged]
from openjson(@data) with (
[substationid] varchar(30),
[validfrom] datetime2,
[validto] datetime2,
[description] varchar(100),
[regionid] varchar(10),
[ownerid] varchar(30),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertNetworkRealtimerating1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Network'
    and sub_type = 'Realtimerating'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Network'
    and sub_type = 'Realtimerating'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Network', 'Realtimerating', 1);

insert into NetworkRealtimerating1(
file_log_id,
[settlementdate],
[spd_id],
[ratingvalue]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[spd_id],
d.[ratingvalue]
from openjson(@data) with (
[settlementdate] datetime2,
[spd_id] varchar(21),
[ratingvalue] decimal(16,6)
) d
end
go
create or alter procedure InsertNetworkOutagedetail3
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Network'
    and sub_type = 'Outagedetail'
    and version = '3'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Network'
    and sub_type = 'Outagedetail'
    and version = '3'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Network', 'Outagedetail', 3);

insert into NetworkOutagedetail3(
file_log_id,
[outageid],
[substationid],
[equipmenttype],
[equipmentid],
[starttime],
[endtime],
[submitteddate],
[outagestatuscode],
[resubmitreason],
[resubmitoutageid],
[recalltimeday],
[recalltimenight],
[lastchanged],
[reason],
[issecondary],
[actual_starttime],
[actual_endtime],
[companyrefcode]
)
select 
(select h.id from @header h),
d.[outageid],
d.[substationid],
d.[equipmenttype],
d.[equipmentid],
d.[starttime],
d.[endtime],
d.[submitteddate],
d.[outagestatuscode],
d.[resubmitreason],
d.[resubmitoutageid],
d.[recalltimeday],
d.[recalltimenight],
d.[lastchanged],
d.[reason],
d.[issecondary],
d.[actual_starttime],
d.[actual_endtime],
d.[companyrefcode]
from openjson(@data) with (
[outageid] decimal(15,0),
[substationid] varchar(30),
[equipmenttype] varchar(10),
[equipmentid] varchar(30),
[starttime] datetime2,
[endtime] datetime2,
[submitteddate] datetime2,
[outagestatuscode] varchar(10),
[resubmitreason] varchar(50),
[resubmitoutageid] decimal(15,0),
[recalltimeday] decimal(10,0),
[recalltimenight] decimal(10,0),
[lastchanged] datetime2,
[reason] varchar(100),
[issecondary] decimal(1,0),
[actual_starttime] datetime2,
[actual_endtime] datetime2,
[companyrefcode] varchar(20)
) d
end
go
create or alter procedure InsertMarketConfigTransmissionlossfactor2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Transmissionlossfactor'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Transmissionlossfactor'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'MarketConfig', 'Transmissionlossfactor', 2);

insert into MarketConfigTransmissionlossfactor2(
file_log_id,
[transmissionlossfactor],
[effectivedate],
[versionno],
[connectionpointid],
[regionid],
[lastchanged],
[secondary_tlf]
)
select 
(select h.id from @header h),
d.[transmissionlossfactor],
d.[effectivedate],
d.[versionno],
d.[connectionpointid],
d.[regionid],
d.[lastchanged],
d.[secondary_tlf]
from openjson(@data) with (
[transmissionlossfactor] decimal(15,5),
[effectivedate] datetime2,
[versionno] decimal(22,0),
[connectionpointid] varchar(10),
[regionid] varchar(10),
[lastchanged] datetime2,
[secondary_tlf] decimal(18,8)
) d
end
go
create or alter procedure InsertMarketConfigLossfactormodel1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Lossfactormodel'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Lossfactormodel'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'MarketConfig', 'Lossfactormodel', 1);

insert into MarketConfigLossfactormodel1(
file_log_id,
[effectivedate],
[versionno],
[interconnectorid],
[regionid],
[demandcoefficient],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[versionno],
d.[interconnectorid],
d.[regionid],
d.[demandcoefficient],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[versionno] decimal(3,0),
[interconnectorid] varchar(10),
[regionid] varchar(10),
[demandcoefficient] decimal(27,17),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMarketConfigRegionstandingdata1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Regionstandingdata'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Regionstandingdata'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'MarketConfig', 'Regionstandingdata', 1);

insert into MarketConfigRegionstandingdata1(
file_log_id,
[effectivedate],
[versionno],
[regionid],
[rsoid],
[regionalreferencepointid],
[peaktradingperiod],
[authoriseddate],
[authorisedby],
[scalingfactor],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[versionno],
d.[regionid],
d.[rsoid],
d.[regionalreferencepointid],
d.[peaktradingperiod],
d.[authoriseddate],
d.[authorisedby],
d.[scalingfactor],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[versionno] decimal(3,0),
[regionid] varchar(10),
[rsoid] varchar(10),
[regionalreferencepointid] varchar(10),
[peaktradingperiod] decimal(3,0),
[authoriseddate] datetime2,
[authorisedby] varchar(15),
[scalingfactor] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMarketConfigMarketPriceThresholds1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'MarketPriceThresholds'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'MarketPriceThresholds'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'MarketConfig', 'MarketPriceThresholds', 1);

insert into MarketConfigMarketPriceThresholds1(
file_log_id,
[effectivedate],
[versionno],
[voll],
[marketpricefloor],
[administered_price_threshold],
[authoriseddate],
[authorisedby],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[versionno],
d.[voll],
d.[marketpricefloor],
d.[administered_price_threshold],
d.[authoriseddate],
d.[authorisedby],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[versionno] decimal(4,0),
[voll] decimal(15,5),
[marketpricefloor] decimal(15,5),
[administered_price_threshold] decimal(15,5),
[authoriseddate] datetime2,
[authorisedby] varchar(15),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMarketConfigBidtypes1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Bidtypes'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Bidtypes'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'MarketConfig', 'Bidtypes', 1);

insert into MarketConfigBidtypes1(
file_log_id,
[bidtype],
[effectivedate],
[versionno],
[description],
[numberofbands],
[numdaysaheadpricelocked],
[validationrule],
[lastchanged],
[spdalias]
)
select 
(select h.id from @header h),
d.[bidtype],
d.[effectivedate],
d.[versionno],
d.[description],
d.[numberofbands],
d.[numdaysaheadpricelocked],
d.[validationrule],
d.[lastchanged],
d.[spdalias]
from openjson(@data) with (
[bidtype] varchar(10),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[description] varchar(64),
[numberofbands] decimal(3,0),
[numdaysaheadpricelocked] decimal(2,0),
[validationrule] varchar(10),
[lastchanged] datetime2,
[spdalias] varchar(10)
) d
end
go
create or alter procedure InsertMarketConfigInterconnectorconstraint1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Interconnectorconstraint'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Interconnectorconstraint'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'MarketConfig', 'Interconnectorconstraint', 1);

insert into MarketConfigInterconnectorconstraint1(
file_log_id,
[reserveoverallloadfactor],
[fromregionlossshare],
[effectivedate],
[versionno],
[interconnectorid],
[maxmwin],
[maxmwout],
[lossconstant],
[lossflowcoefficient],
[emsmeasurand],
[authorisedby],
[authoriseddate],
[dynamicrhs],
[importlimit],
[exportlimit],
[outagederationfactor],
[nonphysicallossfactor],
[overloadfactor60sec],
[overloadfactor6sec],
[lastchanged],
[fcassupportunavailable],
[ictype]
)
select 
(select h.id from @header h),
d.[reserveoverallloadfactor],
d.[fromregionlossshare],
d.[effectivedate],
d.[versionno],
d.[interconnectorid],
d.[maxmwin],
d.[maxmwout],
d.[lossconstant],
d.[lossflowcoefficient],
d.[emsmeasurand],
d.[authorisedby],
d.[authoriseddate],
d.[dynamicrhs],
d.[importlimit],
d.[exportlimit],
d.[outagederationfactor],
d.[nonphysicallossfactor],
d.[overloadfactor60sec],
d.[overloadfactor6sec],
d.[lastchanged],
d.[fcassupportunavailable],
d.[ictype]
from openjson(@data) with (
[reserveoverallloadfactor] decimal(5,2),
[fromregionlossshare] decimal(5,2),
[effectivedate] datetime2,
[versionno] decimal(3,0),
[interconnectorid] varchar(10),
[maxmwin] decimal(15,5),
[maxmwout] decimal(15,5),
[lossconstant] decimal(15,6),
[lossflowcoefficient] decimal(27,17),
[emsmeasurand] varchar(40),
[authorisedby] varchar(15),
[authoriseddate] datetime2,
[dynamicrhs] varchar(1),
[importlimit] decimal(6,0),
[exportlimit] decimal(6,0),
[outagederationfactor] decimal(15,5),
[nonphysicallossfactor] decimal(15,5),
[overloadfactor60sec] decimal(15,5),
[overloadfactor6sec] decimal(15,5),
[lastchanged] datetime2,
[fcassupportunavailable] decimal(1,0),
[ictype] varchar(10)
) d
end
go
create or alter procedure InsertMarketConfigIntraregionalloc1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Intraregionalloc'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Intraregionalloc'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'MarketConfig', 'Intraregionalloc', 1);

insert into MarketConfigIntraregionalloc1(
file_log_id,
[effectivedate],
[versionno],
[regionid],
[participantid],
[allocation],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[versionno],
d.[regionid],
d.[participantid],
d.[allocation],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[versionno] decimal(5,0),
[regionid] varchar(10),
[participantid] varchar(10),
[allocation] decimal(12,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMarketConfigRegion1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Region'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Region'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'MarketConfig', 'Region', 1);

insert into MarketConfigRegion1(
file_log_id,
[regionid],
[description],
[regionstatus],
[lastchanged]
)
select 
(select h.id from @header h),
d.[regionid],
d.[description],
d.[regionstatus],
d.[lastchanged]
from openjson(@data) with (
[regionid] varchar(10),
[description] varchar(64),
[regionstatus] varchar(8),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMarketConfigLossmodel1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Lossmodel'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Lossmodel'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'MarketConfig', 'Lossmodel', 1);

insert into MarketConfigLossmodel1(
file_log_id,
[effectivedate],
[versionno],
[interconnectorid],
[periodid],
[losssegment],
[mwbreakpoint],
[lossfactor],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[versionno],
d.[interconnectorid],
d.[periodid],
d.[losssegment],
d.[mwbreakpoint],
d.[lossfactor],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[versionno] decimal(3,0),
[interconnectorid] varchar(10),
[periodid] varchar(20),
[losssegment] decimal(6,0),
[mwbreakpoint] decimal(6,0),
[lossfactor] decimal(16,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMarketConfigBidtypestrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Bidtypestrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Bidtypestrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'MarketConfig', 'Bidtypestrk', 1);

insert into MarketConfigBidtypestrk1(
file_log_id,
[effectivedate],
[versionno],
[authoriseddate],
[authorisedby],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[versionno],
d.[authoriseddate],
d.[authorisedby],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[versionno] decimal(3,0),
[authoriseddate] datetime2,
[authorisedby] varchar(15),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMarketConfigInterconnector1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Interconnector'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Interconnector'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'MarketConfig', 'Interconnector', 1);

insert into MarketConfigInterconnector1(
file_log_id,
[interconnectorid],
[regionfrom],
[rsoid],
[regionto],
[description],
[lastchanged]
)
select 
(select h.id from @header h),
d.[interconnectorid],
d.[regionfrom],
d.[rsoid],
d.[regionto],
d.[description],
d.[lastchanged]
from openjson(@data) with (
[interconnectorid] varchar(10),
[regionfrom] varchar(10),
[rsoid] varchar(10),
[regionto] varchar(10),
[description] varchar(64),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertMarketConfigInterconnectoralloc1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Interconnectoralloc'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'MarketConfig'
    and sub_type = 'Interconnectoralloc'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'MarketConfig', 'Interconnectoralloc', 1);

insert into MarketConfigInterconnectoralloc1(
file_log_id,
[effectivedate],
[versionno],
[interconnectorid],
[regionid],
[participantid],
[allocation],
[lastchanged]
)
select 
(select h.id from @header h),
d.[effectivedate],
d.[versionno],
d.[interconnectorid],
d.[regionid],
d.[participantid],
d.[allocation],
d.[lastchanged]
from openjson(@data) with (
[effectivedate] datetime2,
[versionno] decimal(5,0),
[interconnectorid] varchar(10),
[regionid] varchar(10),
[participantid] varchar(10),
[allocation] decimal(12,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertDispatchIntermittentForecastTrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'IntermittentForecastTrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'IntermittentForecastTrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'IntermittentForecastTrk', 1);

insert into DispatchIntermittentForecastTrk1(
file_log_id,
[settlementdate],
[duid],
[origin],
[forecast_priority],
[offerdatetime]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[duid],
d.[origin],
d.[forecast_priority],
d.[offerdatetime]
from openjson(@data) with (
[settlementdate] datetime2,
[duid] varchar(20),
[origin] varchar(20),
[forecast_priority] decimal(10,0),
[offerdatetime] datetime2
) d
end
go
create or alter procedure InsertDispatchMrScheduleTrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'MrScheduleTrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'MrScheduleTrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'MrScheduleTrk', 1);

insert into DispatchMrScheduleTrk1(
file_log_id,
[settlementdate],
[regionid],
[mr_date],
[version_datetime],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[regionid],
d.[mr_date],
d.[version_datetime],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[regionid] varchar(10),
[mr_date] datetime2,
[version_datetime] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertDispatchUnitSolution2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'UnitSolution'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'UnitSolution'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'UnitSolution', 2);

insert into DispatchUnitSolution2(
file_log_id,
[settlementdate],
[runno],
[duid],
[tradetype],
[dispatchinterval],
[intervention],
[connectionpointid],
[dispatchmode],
[agcstatus],
[initialmw],
[totalcleared],
[rampdownrate],
[rampuprate],
[lower5min],
[lower60sec],
[lower6sec],
[raise5min],
[raise60sec],
[raise6sec],
[downepf],
[upepf],
[marginal5minvalue],
[marginal60secvalue],
[marginal6secvalue],
[marginalvalue],
[violation5mindegree],
[violation60secdegree],
[violation6secdegree],
[violationdegree],
[lastchanged],
[lowerreg],
[raisereg],
[availability],
[raise6secflags],
[raise60secflags],
[raise5minflags],
[raiseregflags],
[lower6secflags],
[lower60secflags],
[lower5minflags],
[lowerregflags],
[raiseregavailability],
[raiseregenablementmax],
[raiseregenablementmin],
[lowerregavailability],
[lowerregenablementmax],
[lowerregenablementmin],
[raise6secactualavailability],
[raise60secactualavailability],
[raise5minactualavailability],
[raiseregactualavailability],
[lower6secactualavailability],
[lower60secactualavailability],
[lower5minactualavailability],
[lowerregactualavailability],
[semidispatchcap]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[duid],
d.[tradetype],
d.[dispatchinterval],
d.[intervention],
d.[connectionpointid],
d.[dispatchmode],
d.[agcstatus],
d.[initialmw],
d.[totalcleared],
d.[rampdownrate],
d.[rampuprate],
d.[lower5min],
d.[lower60sec],
d.[lower6sec],
d.[raise5min],
d.[raise60sec],
d.[raise6sec],
d.[downepf],
d.[upepf],
d.[marginal5minvalue],
d.[marginal60secvalue],
d.[marginal6secvalue],
d.[marginalvalue],
d.[violation5mindegree],
d.[violation60secdegree],
d.[violation6secdegree],
d.[violationdegree],
d.[lastchanged],
d.[lowerreg],
d.[raisereg],
d.[availability],
d.[raise6secflags],
d.[raise60secflags],
d.[raise5minflags],
d.[raiseregflags],
d.[lower6secflags],
d.[lower60secflags],
d.[lower5minflags],
d.[lowerregflags],
d.[raiseregavailability],
d.[raiseregenablementmax],
d.[raiseregenablementmin],
d.[lowerregavailability],
d.[lowerregenablementmax],
d.[lowerregenablementmin],
d.[raise6secactualavailability],
d.[raise60secactualavailability],
d.[raise5minactualavailability],
d.[raiseregactualavailability],
d.[lower6secactualavailability],
d.[lower60secactualavailability],
d.[lower5minactualavailability],
d.[lowerregactualavailability],
d.[semidispatchcap]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[duid] varchar(10),
[tradetype] decimal(2,0),
[dispatchinterval] decimal(22,0),
[intervention] decimal(2,0),
[connectionpointid] varchar(12),
[dispatchmode] decimal(2,0),
[agcstatus] decimal(2,0),
[initialmw] decimal(15,5),
[totalcleared] decimal(15,5),
[rampdownrate] decimal(15,5),
[rampuprate] decimal(15,5),
[lower5min] decimal(15,5),
[lower60sec] decimal(15,5),
[lower6sec] decimal(15,5),
[raise5min] decimal(15,5),
[raise60sec] decimal(15,5),
[raise6sec] decimal(15,5),
[downepf] decimal(15,5),
[upepf] decimal(15,5),
[marginal5minvalue] decimal(15,5),
[marginal60secvalue] decimal(15,5),
[marginal6secvalue] decimal(15,5),
[marginalvalue] decimal(15,5),
[violation5mindegree] decimal(15,5),
[violation60secdegree] decimal(15,5),
[violation6secdegree] decimal(15,5),
[violationdegree] decimal(15,5),
[lastchanged] datetime2,
[lowerreg] decimal(15,5),
[raisereg] decimal(15,5),
[availability] decimal(15,5),
[raise6secflags] decimal(3,0),
[raise60secflags] decimal(3,0),
[raise5minflags] decimal(3,0),
[raiseregflags] decimal(3,0),
[lower6secflags] decimal(3,0),
[lower60secflags] decimal(3,0),
[lower5minflags] decimal(3,0),
[lowerregflags] decimal(3,0),
[raiseregavailability] decimal(15,5),
[raiseregenablementmax] decimal(15,5),
[raiseregenablementmin] decimal(15,5),
[lowerregavailability] decimal(15,5),
[lowerregenablementmax] decimal(15,5),
[lowerregenablementmin] decimal(15,5),
[raise6secactualavailability] decimal(16,6),
[raise60secactualavailability] decimal(16,6),
[raise5minactualavailability] decimal(16,6),
[raiseregactualavailability] decimal(16,6),
[lower6secactualavailability] decimal(16,6),
[lower60secactualavailability] decimal(16,6),
[lower5minactualavailability] decimal(16,6),
[lowerregactualavailability] decimal(16,6),
[semidispatchcap] decimal(3,0)
) d
end
go
create or alter procedure InsertPriceloadPriceRevision1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Priceload'
    and sub_type = 'PriceRevision'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Priceload'
    and sub_type = 'PriceRevision'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Priceload', 'PriceRevision', 1);

insert into PriceloadPriceRevision1(
file_log_id,
[settlementdate],
[runno],
[intervention],
[regionid],
[bidtype],
[versionno],
[rrp_new],
[rrp_old],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[intervention],
d.[regionid],
d.[bidtype],
d.[versionno],
d.[rrp_new],
d.[rrp_old],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[intervention] decimal(2,0),
[regionid] varchar(10),
[bidtype] varchar(10),
[versionno] decimal(3,0),
[rrp_new] decimal(15,5),
[rrp_old] decimal(15,5),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertDispatchPrice4
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'Price'
    and version = '4'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'Price'
    and version = '4'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'Price', 4);

insert into DispatchPrice4(
file_log_id,
[settlementdate],
[runno],
[regionid],
[dispatchinterval],
[intervention],
[rrp],
[eep],
[rop],
[apcflag],
[marketsuspendedflag],
[lastchanged],
[raise6secrrp],
[raise6secrop],
[raise6secapcflag],
[raise60secrrp],
[raise60secrop],
[raise60secapcflag],
[raise5minrrp],
[raise5minrop],
[raise5minapcflag],
[raiseregrrp],
[raiseregrop],
[raiseregapcflag],
[lower6secrrp],
[lower6secrop],
[lower6secapcflag],
[lower60secrrp],
[lower60secrop],
[lower60secapcflag],
[lower5minrrp],
[lower5minrop],
[lower5minapcflag],
[lowerregrrp],
[lowerregrop],
[lowerregapcflag],
[price_status],
[pre_ap_energy_price],
[pre_ap_raise6_price],
[pre_ap_raise60_price],
[pre_ap_raise5min_price],
[pre_ap_raisereg_price],
[pre_ap_lower6_price],
[pre_ap_lower60_price],
[pre_ap_lower5min_price],
[pre_ap_lowerreg_price],
[cumul_pre_ap_energy_price],
[cumul_pre_ap_raise6_price],
[cumul_pre_ap_raise60_price],
[cumul_pre_ap_raise5min_price],
[cumul_pre_ap_raisereg_price],
[cumul_pre_ap_lower6_price],
[cumul_pre_ap_lower60_price],
[cumul_pre_ap_lower5min_price],
[cumul_pre_ap_lowerreg_price],
[ocd_status],
[mii_status]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[regionid],
d.[dispatchinterval],
d.[intervention],
d.[rrp],
d.[eep],
d.[rop],
d.[apcflag],
d.[marketsuspendedflag],
d.[lastchanged],
d.[raise6secrrp],
d.[raise6secrop],
d.[raise6secapcflag],
d.[raise60secrrp],
d.[raise60secrop],
d.[raise60secapcflag],
d.[raise5minrrp],
d.[raise5minrop],
d.[raise5minapcflag],
d.[raiseregrrp],
d.[raiseregrop],
d.[raiseregapcflag],
d.[lower6secrrp],
d.[lower6secrop],
d.[lower6secapcflag],
d.[lower60secrrp],
d.[lower60secrop],
d.[lower60secapcflag],
d.[lower5minrrp],
d.[lower5minrop],
d.[lower5minapcflag],
d.[lowerregrrp],
d.[lowerregrop],
d.[lowerregapcflag],
d.[price_status],
d.[pre_ap_energy_price],
d.[pre_ap_raise6_price],
d.[pre_ap_raise60_price],
d.[pre_ap_raise5min_price],
d.[pre_ap_raisereg_price],
d.[pre_ap_lower6_price],
d.[pre_ap_lower60_price],
d.[pre_ap_lower5min_price],
d.[pre_ap_lowerreg_price],
d.[cumul_pre_ap_energy_price],
d.[cumul_pre_ap_raise6_price],
d.[cumul_pre_ap_raise60_price],
d.[cumul_pre_ap_raise5min_price],
d.[cumul_pre_ap_raisereg_price],
d.[cumul_pre_ap_lower6_price],
d.[cumul_pre_ap_lower60_price],
d.[cumul_pre_ap_lower5min_price],
d.[cumul_pre_ap_lowerreg_price],
d.[ocd_status],
d.[mii_status]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[regionid] varchar(10),
[dispatchinterval] varchar(22),
[intervention] decimal(2,0),
[rrp] decimal(15,5),
[eep] decimal(15,5),
[rop] decimal(15,5),
[apcflag] decimal(3,0),
[marketsuspendedflag] decimal(3,0),
[lastchanged] datetime2,
[raise6secrrp] decimal(15,5),
[raise6secrop] decimal(15,5),
[raise6secapcflag] decimal(3,0),
[raise60secrrp] decimal(15,5),
[raise60secrop] decimal(15,5),
[raise60secapcflag] decimal(3,0),
[raise5minrrp] decimal(15,5),
[raise5minrop] decimal(15,5),
[raise5minapcflag] decimal(3,0),
[raiseregrrp] decimal(15,5),
[raiseregrop] decimal(15,5),
[raiseregapcflag] decimal(3,0),
[lower6secrrp] decimal(15,5),
[lower6secrop] decimal(15,5),
[lower6secapcflag] decimal(3,0),
[lower60secrrp] decimal(15,5),
[lower60secrop] decimal(15,5),
[lower60secapcflag] decimal(3,0),
[lower5minrrp] decimal(15,5),
[lower5minrop] decimal(15,5),
[lower5minapcflag] decimal(3,0),
[lowerregrrp] decimal(15,5),
[lowerregrop] decimal(15,5),
[lowerregapcflag] decimal(3,0),
[price_status] varchar(20),
[pre_ap_energy_price] decimal(15,5),
[pre_ap_raise6_price] decimal(15,5),
[pre_ap_raise60_price] decimal(15,5),
[pre_ap_raise5min_price] decimal(15,5),
[pre_ap_raisereg_price] decimal(15,5),
[pre_ap_lower6_price] decimal(15,5),
[pre_ap_lower60_price] decimal(15,5),
[pre_ap_lower5min_price] decimal(15,5),
[pre_ap_lowerreg_price] decimal(15,5),
[cumul_pre_ap_energy_price] decimal(15,5),
[cumul_pre_ap_raise6_price] decimal(15,5),
[cumul_pre_ap_raise60_price] decimal(15,5),
[cumul_pre_ap_raise5min_price] decimal(15,5),
[cumul_pre_ap_raisereg_price] decimal(15,5),
[cumul_pre_ap_lower6_price] decimal(15,5),
[cumul_pre_ap_lower60_price] decimal(15,5),
[cumul_pre_ap_lower5min_price] decimal(15,5),
[cumul_pre_ap_lowerreg_price] decimal(15,5),
[ocd_status] varchar(14),
[mii_status] varchar(21)
) d
end
go
create or alter procedure InsertDispatchUnitConformance1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'UnitConformance'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'UnitConformance'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'UnitConformance', 1);

insert into DispatchUnitConformance1(
file_log_id,
[interval_datetime],
[duid],
[totalcleared],
[actualmw],
[roc],
[availability],
[lowerreg],
[raisereg],
[striglm],
[ltriglm],
[mwerror],
[max_mwerror],
[lecount],
[secount],
[status],
[participant_status_action],
[operating_mode],
[lastchanged]
)
select 
(select h.id from @header h),
d.[interval_datetime],
d.[duid],
d.[totalcleared],
d.[actualmw],
d.[roc],
d.[availability],
d.[lowerreg],
d.[raisereg],
d.[striglm],
d.[ltriglm],
d.[mwerror],
d.[max_mwerror],
d.[lecount],
d.[secount],
d.[status],
d.[participant_status_action],
d.[operating_mode],
d.[lastchanged]
from openjson(@data) with (
[interval_datetime] datetime2,
[duid] varchar(20),
[totalcleared] decimal(16,6),
[actualmw] decimal(16,6),
[roc] decimal(16,6),
[availability] decimal(16,6),
[lowerreg] decimal(16,6),
[raisereg] decimal(16,6),
[striglm] decimal(16,6),
[ltriglm] decimal(16,6),
[mwerror] decimal(16,6),
[max_mwerror] decimal(16,6),
[lecount] decimal(6,0),
[secount] decimal(6,0),
[status] varchar(20),
[participant_status_action] varchar(100),
[operating_mode] varchar(20),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertDispatchBlockedConstraints1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'BlockedConstraints'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'BlockedConstraints'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'BlockedConstraints', 1);

insert into DispatchBlockedConstraints1(
file_log_id,
[settlementdate],
[runno],
[constraintid]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[constraintid]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[constraintid] varchar(20)
) d
end
go
create or alter procedure InsertDispatchFcasReq2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'FcasReq'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'FcasReq'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'FcasReq', 2);

insert into DispatchFcasReq2(
file_log_id,
[settlementdate],
[runno],
[intervention],
[genconid],
[regionid],
[bidtype],
[genconeffectivedate],
[genconversionno],
[marginalvalue],
[lastchanged],
[base_cost],
[adjusted_cost],
[estimated_cmpf],
[estimated_crmpf],
[recovery_factor_cmpf],
[recovery_factor_crmpf]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[intervention],
d.[genconid],
d.[regionid],
d.[bidtype],
d.[genconeffectivedate],
d.[genconversionno],
d.[marginalvalue],
d.[lastchanged],
d.[base_cost],
d.[adjusted_cost],
d.[estimated_cmpf],
d.[estimated_crmpf],
d.[recovery_factor_cmpf],
d.[recovery_factor_crmpf]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[intervention] decimal(2,0),
[genconid] varchar(20),
[regionid] varchar(10),
[bidtype] varchar(10),
[genconeffectivedate] datetime2,
[genconversionno] decimal(3,0),
[marginalvalue] decimal(16,6),
[lastchanged] datetime2,
[base_cost] decimal(18,8),
[adjusted_cost] decimal(18,8),
[estimated_cmpf] decimal(18,8),
[estimated_crmpf] decimal(18,8),
[recovery_factor_cmpf] decimal(18,8),
[recovery_factor_crmpf] decimal(18,8)
) d
end
go
create or alter procedure InsertDispatchLocalPrice1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'LocalPrice'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'LocalPrice'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'LocalPrice', 1);

insert into DispatchLocalPrice1(
file_log_id,
[settlementdate],
[duid],
[local_price_adjustment],
[locally_constrained]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[duid],
d.[local_price_adjustment],
d.[locally_constrained]
from openjson(@data) with (
[settlementdate] datetime2,
[duid] varchar(20),
[local_price_adjustment] decimal(10,2),
[locally_constrained] decimal(1,0)
) d
end
go
create or alter procedure InsertDispatchInterconnection1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'Interconnection'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'Interconnection'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'Interconnection', 1);

insert into DispatchInterconnection1(
file_log_id,
[settlementdate],
[runno],
[intervention],
[from_regionid],
[to_regionid],
[dispatchinterval],
[irlf],
[mwflow],
[meteredmwflow],
[from_region_mw_losses],
[to_region_mw_losses],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[intervention],
d.[from_regionid],
d.[to_regionid],
d.[dispatchinterval],
d.[irlf],
d.[mwflow],
d.[meteredmwflow],
d.[from_region_mw_losses],
d.[to_region_mw_losses],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[intervention] decimal(2,0),
[from_regionid] varchar(20),
[to_regionid] varchar(20),
[dispatchinterval] decimal(22,0),
[irlf] decimal(15,5),
[mwflow] decimal(16,6),
[meteredmwflow] decimal(16,6),
[from_region_mw_losses] decimal(16,6),
[to_region_mw_losses] decimal(16,6),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertDispatchNegativeResidue1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'NegativeResidue'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'NegativeResidue'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'NegativeResidue', 1);

insert into DispatchNegativeResidue1(
file_log_id,
[settlementdate],
[nrm_datetime],
[directional_interconnectorid],
[nrm_activated_flag],
[cumul_negresidue_amount],
[cumul_negresidue_prev_ti],
[negresidue_current_ti],
[negresidue_pd_next_ti],
[price_revision],
[predispatchseqno],
[event_activated_di],
[event_deactivated_di],
[di_notbinding_count],
[di_violated_count],
[nrmconstraint_blocked_flag]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[nrm_datetime],
d.[directional_interconnectorid],
d.[nrm_activated_flag],
d.[cumul_negresidue_amount],
d.[cumul_negresidue_prev_ti],
d.[negresidue_current_ti],
d.[negresidue_pd_next_ti],
d.[price_revision],
d.[predispatchseqno],
d.[event_activated_di],
d.[event_deactivated_di],
d.[di_notbinding_count],
d.[di_violated_count],
d.[nrmconstraint_blocked_flag]
from openjson(@data) with (
[settlementdate] datetime2,
[nrm_datetime] datetime2,
[directional_interconnectorid] varchar(30),
[nrm_activated_flag] decimal(1,0),
[cumul_negresidue_amount] decimal(15,5),
[cumul_negresidue_prev_ti] decimal(15,5),
[negresidue_current_ti] decimal(15,5),
[negresidue_pd_next_ti] decimal(15,5),
[price_revision] varchar(30),
[predispatchseqno] varchar(20),
[event_activated_di] datetime2,
[event_deactivated_di] datetime2,
[di_notbinding_count] decimal(2,0),
[di_violated_count] decimal(2,0),
[nrmconstraint_blocked_flag] decimal(1,0)
) d
end
go
create or alter procedure InsertDispatchRegionsum4
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'Regionsum'
    and version = '4'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'Regionsum'
    and version = '4'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'Regionsum', 4);

insert into DispatchRegionsum4(
file_log_id,
[settlementdate],
[runno],
[regionid],
[dispatchinterval],
[intervention],
[totaldemand],
[availablegeneration],
[availableload],
[demandforecast],
[dispatchablegeneration],
[dispatchableload],
[netinterchange],
[excessgeneration],
[lower5mindispatch],
[lower5minimport],
[lower5minlocaldispatch],
[lower5minlocalprice],
[lower5minlocalreq],
[lower5minprice],
[lower5minreq],
[lower5minsupplyprice],
[lower60secdispatch],
[lower60secimport],
[lower60seclocaldispatch],
[lower60seclocalprice],
[lower60seclocalreq],
[lower60secprice],
[lower60secreq],
[lower60secsupplyprice],
[lower6secdispatch],
[lower6secimport],
[lower6seclocaldispatch],
[lower6seclocalprice],
[lower6seclocalreq],
[lower6secprice],
[lower6secreq],
[lower6secsupplyprice],
[raise5mindispatch],
[raise5minimport],
[raise5minlocaldispatch],
[raise5minlocalprice],
[raise5minlocalreq],
[raise5minprice],
[raise5minreq],
[raise5minsupplyprice],
[raise60secdispatch],
[raise60secimport],
[raise60seclocaldispatch],
[raise60seclocalprice],
[raise60seclocalreq],
[raise60secprice],
[raise60secreq],
[raise60secsupplyprice],
[raise6secdispatch],
[raise6secimport],
[raise6seclocaldispatch],
[raise6seclocalprice],
[raise6seclocalreq],
[raise6secprice],
[raise6secreq],
[raise6secsupplyprice],
[aggegatedispatcherror],
[aggregatedispatcherror],
[lastchanged],
[initialsupply],
[clearedsupply],
[lowerregimport],
[lowerreglocaldispatch],
[lowerreglocalreq],
[lowerregreq],
[raiseregimport],
[raisereglocaldispatch],
[raisereglocalreq],
[raiseregreq],
[raise5minlocalviolation],
[raisereglocalviolation],
[raise60seclocalviolation],
[raise6seclocalviolation],
[lower5minlocalviolation],
[lowerreglocalviolation],
[lower60seclocalviolation],
[lower6seclocalviolation],
[raise5minviolation],
[raiseregviolation],
[raise60secviolation],
[raise6secviolation],
[lower5minviolation],
[lowerregviolation],
[lower60secviolation],
[lower6secviolation],
[raise6secactualavailability],
[raise60secactualavailability],
[raise5minactualavailability],
[raiseregactualavailability],
[lower6secactualavailability],
[lower60secactualavailability],
[lower5minactualavailability],
[lowerregactualavailability],
[lorsurplus],
[lrcsurplus],
[totalintermittentgeneration],
[demand_and_nonschedgen],
[uigf],
[semischedule_clearedmw],
[semischedule_compliancemw],
[ss_solar_uigf],
[ss_wind_uigf],
[ss_solar_clearedmw],
[ss_wind_clearedmw],
[ss_solar_compliancemw],
[ss_wind_compliancemw]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[regionid],
d.[dispatchinterval],
d.[intervention],
d.[totaldemand],
d.[availablegeneration],
d.[availableload],
d.[demandforecast],
d.[dispatchablegeneration],
d.[dispatchableload],
d.[netinterchange],
d.[excessgeneration],
d.[lower5mindispatch],
d.[lower5minimport],
d.[lower5minlocaldispatch],
d.[lower5minlocalprice],
d.[lower5minlocalreq],
d.[lower5minprice],
d.[lower5minreq],
d.[lower5minsupplyprice],
d.[lower60secdispatch],
d.[lower60secimport],
d.[lower60seclocaldispatch],
d.[lower60seclocalprice],
d.[lower60seclocalreq],
d.[lower60secprice],
d.[lower60secreq],
d.[lower60secsupplyprice],
d.[lower6secdispatch],
d.[lower6secimport],
d.[lower6seclocaldispatch],
d.[lower6seclocalprice],
d.[lower6seclocalreq],
d.[lower6secprice],
d.[lower6secreq],
d.[lower6secsupplyprice],
d.[raise5mindispatch],
d.[raise5minimport],
d.[raise5minlocaldispatch],
d.[raise5minlocalprice],
d.[raise5minlocalreq],
d.[raise5minprice],
d.[raise5minreq],
d.[raise5minsupplyprice],
d.[raise60secdispatch],
d.[raise60secimport],
d.[raise60seclocaldispatch],
d.[raise60seclocalprice],
d.[raise60seclocalreq],
d.[raise60secprice],
d.[raise60secreq],
d.[raise60secsupplyprice],
d.[raise6secdispatch],
d.[raise6secimport],
d.[raise6seclocaldispatch],
d.[raise6seclocalprice],
d.[raise6seclocalreq],
d.[raise6secprice],
d.[raise6secreq],
d.[raise6secsupplyprice],
d.[aggegatedispatcherror],
d.[aggregatedispatcherror],
d.[lastchanged],
d.[initialsupply],
d.[clearedsupply],
d.[lowerregimport],
d.[lowerreglocaldispatch],
d.[lowerreglocalreq],
d.[lowerregreq],
d.[raiseregimport],
d.[raisereglocaldispatch],
d.[raisereglocalreq],
d.[raiseregreq],
d.[raise5minlocalviolation],
d.[raisereglocalviolation],
d.[raise60seclocalviolation],
d.[raise6seclocalviolation],
d.[lower5minlocalviolation],
d.[lowerreglocalviolation],
d.[lower60seclocalviolation],
d.[lower6seclocalviolation],
d.[raise5minviolation],
d.[raiseregviolation],
d.[raise60secviolation],
d.[raise6secviolation],
d.[lower5minviolation],
d.[lowerregviolation],
d.[lower60secviolation],
d.[lower6secviolation],
d.[raise6secactualavailability],
d.[raise60secactualavailability],
d.[raise5minactualavailability],
d.[raiseregactualavailability],
d.[lower6secactualavailability],
d.[lower60secactualavailability],
d.[lower5minactualavailability],
d.[lowerregactualavailability],
d.[lorsurplus],
d.[lrcsurplus],
d.[totalintermittentgeneration],
d.[demand_and_nonschedgen],
d.[uigf],
d.[semischedule_clearedmw],
d.[semischedule_compliancemw],
d.[ss_solar_uigf],
d.[ss_wind_uigf],
d.[ss_solar_clearedmw],
d.[ss_wind_clearedmw],
d.[ss_solar_compliancemw],
d.[ss_wind_compliancemw]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[regionid] varchar(10),
[dispatchinterval] decimal(22,0),
[intervention] decimal(2,0),
[totaldemand] decimal(15,5),
[availablegeneration] decimal(15,5),
[availableload] decimal(15,5),
[demandforecast] decimal(15,5),
[dispatchablegeneration] decimal(15,5),
[dispatchableload] decimal(15,5),
[netinterchange] decimal(15,5),
[excessgeneration] decimal(15,5),
[lower5mindispatch] decimal(15,5),
[lower5minimport] decimal(15,5),
[lower5minlocaldispatch] decimal(15,5),
[lower5minlocalprice] decimal(15,5),
[lower5minlocalreq] decimal(15,5),
[lower5minprice] decimal(15,5),
[lower5minreq] decimal(15,5),
[lower5minsupplyprice] decimal(15,5),
[lower60secdispatch] decimal(15,5),
[lower60secimport] decimal(15,5),
[lower60seclocaldispatch] decimal(15,5),
[lower60seclocalprice] decimal(15,5),
[lower60seclocalreq] decimal(15,5),
[lower60secprice] decimal(15,5),
[lower60secreq] decimal(15,5),
[lower60secsupplyprice] decimal(15,5),
[lower6secdispatch] decimal(15,5),
[lower6secimport] decimal(15,5),
[lower6seclocaldispatch] decimal(15,5),
[lower6seclocalprice] decimal(15,5),
[lower6seclocalreq] decimal(15,5),
[lower6secprice] decimal(15,5),
[lower6secreq] decimal(15,5),
[lower6secsupplyprice] decimal(15,5),
[raise5mindispatch] decimal(15,5),
[raise5minimport] decimal(15,5),
[raise5minlocaldispatch] decimal(15,5),
[raise5minlocalprice] decimal(15,5),
[raise5minlocalreq] decimal(15,5),
[raise5minprice] decimal(15,5),
[raise5minreq] decimal(15,5),
[raise5minsupplyprice] decimal(15,5),
[raise60secdispatch] decimal(15,5),
[raise60secimport] decimal(15,5),
[raise60seclocaldispatch] decimal(15,5),
[raise60seclocalprice] decimal(15,5),
[raise60seclocalreq] decimal(15,5),
[raise60secprice] decimal(15,5),
[raise60secreq] decimal(15,5),
[raise60secsupplyprice] decimal(15,5),
[raise6secdispatch] decimal(15,5),
[raise6secimport] decimal(15,5),
[raise6seclocaldispatch] decimal(15,5),
[raise6seclocalprice] decimal(15,5),
[raise6seclocalreq] decimal(15,5),
[raise6secprice] decimal(15,5),
[raise6secreq] decimal(15,5),
[raise6secsupplyprice] decimal(15,5),
[aggegatedispatcherror] decimal(15,5),
[aggregatedispatcherror] decimal(15,5),
[lastchanged] datetime2,
[initialsupply] decimal(15,5),
[clearedsupply] decimal(15,5),
[lowerregimport] decimal(15,5),
[lowerreglocaldispatch] decimal(15,5),
[lowerreglocalreq] decimal(15,5),
[lowerregreq] decimal(15,5),
[raiseregimport] decimal(15,5),
[raisereglocaldispatch] decimal(15,5),
[raisereglocalreq] decimal(15,5),
[raiseregreq] decimal(15,5),
[raise5minlocalviolation] decimal(15,5),
[raisereglocalviolation] decimal(15,5),
[raise60seclocalviolation] decimal(15,5),
[raise6seclocalviolation] decimal(15,5),
[lower5minlocalviolation] decimal(15,5),
[lowerreglocalviolation] decimal(15,5),
[lower60seclocalviolation] decimal(15,5),
[lower6seclocalviolation] decimal(15,5),
[raise5minviolation] decimal(15,5),
[raiseregviolation] decimal(15,5),
[raise60secviolation] decimal(15,5),
[raise6secviolation] decimal(15,5),
[lower5minviolation] decimal(15,5),
[lowerregviolation] decimal(15,5),
[lower60secviolation] decimal(15,5),
[lower6secviolation] decimal(15,5),
[raise6secactualavailability] decimal(16,6),
[raise60secactualavailability] decimal(16,6),
[raise5minactualavailability] decimal(16,6),
[raiseregactualavailability] decimal(16,6),
[lower6secactualavailability] decimal(16,6),
[lower60secactualavailability] decimal(16,6),
[lower5minactualavailability] decimal(16,6),
[lowerregactualavailability] decimal(16,6),
[lorsurplus] decimal(16,6),
[lrcsurplus] decimal(16,6),
[totalintermittentgeneration] decimal(15,5),
[demand_and_nonschedgen] decimal(15,5),
[uigf] decimal(15,5),
[semischedule_clearedmw] decimal(15,5),
[semischedule_compliancemw] decimal(15,5),
[ss_solar_uigf] decimal(15,5),
[ss_wind_uigf] decimal(15,5),
[ss_solar_clearedmw] decimal(15,5),
[ss_wind_clearedmw] decimal(15,5),
[ss_solar_compliancemw] decimal(15,5),
[ss_wind_compliancemw] decimal(15,5)
) d
end
go
create or alter procedure InsertDispatchOffertrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'Offertrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'Offertrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'Offertrk', 1);

insert into DispatchOffertrk1(
file_log_id,
[settlementdate],
[duid],
[bidtype],
[bidsettlementdate],
[bidofferdate],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[duid],
d.[bidtype],
d.[bidsettlementdate],
d.[bidofferdate],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[duid] varchar(10),
[bidtype] varchar(10),
[bidsettlementdate] datetime2,
[bidofferdate] datetime2,
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertDispatchMnspbidtrk1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'Mnspbidtrk'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'Mnspbidtrk'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'Mnspbidtrk', 1);

insert into DispatchMnspbidtrk1(
file_log_id,
[settlementdate],
[runno],
[participantid],
[linkid],
[offersettlementdate],
[offereffectivedate],
[offerversionno],
[lastchanged]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[participantid],
d.[linkid],
d.[offersettlementdate],
d.[offereffectivedate],
d.[offerversionno],
d.[lastchanged]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[participantid] varchar(10),
[linkid] varchar(10),
[offersettlementdate] datetime2,
[offereffectivedate] datetime2,
[offerversionno] decimal(3,0),
[lastchanged] datetime2
) d
end
go
create or alter procedure InsertDispatchInterconnectorres3
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'Interconnectorres'
    and version = '3'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'Interconnectorres'
    and version = '3'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'Interconnectorres', 3);

insert into DispatchInterconnectorres3(
file_log_id,
[settlementdate],
[runno],
[interconnectorid],
[dispatchinterval],
[intervention],
[meteredmwflow],
[mwflow],
[mwlosses],
[marginalvalue],
[violationdegree],
[lastchanged],
[exportlimit],
[importlimit],
[marginalloss],
[exportgenconid],
[importgenconid],
[fcasexportlimit],
[fcasimportlimit],
[local_price_adjustment_export],
[locally_constrained_export],
[local_price_adjustment_import],
[locally_constrained_import]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[interconnectorid],
d.[dispatchinterval],
d.[intervention],
d.[meteredmwflow],
d.[mwflow],
d.[mwlosses],
d.[marginalvalue],
d.[violationdegree],
d.[lastchanged],
d.[exportlimit],
d.[importlimit],
d.[marginalloss],
d.[exportgenconid],
d.[importgenconid],
d.[fcasexportlimit],
d.[fcasimportlimit],
d.[local_price_adjustment_export],
d.[locally_constrained_export],
d.[local_price_adjustment_import],
d.[locally_constrained_import]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[interconnectorid] varchar(10),
[dispatchinterval] decimal(22,0),
[intervention] decimal(2,0),
[meteredmwflow] decimal(15,5),
[mwflow] decimal(15,5),
[mwlosses] decimal(15,5),
[marginalvalue] decimal(15,5),
[violationdegree] decimal(15,5),
[lastchanged] datetime2,
[exportlimit] decimal(15,5),
[importlimit] decimal(15,5),
[marginalloss] decimal(15,5),
[exportgenconid] varchar(20),
[importgenconid] varchar(20),
[fcasexportlimit] decimal(15,5),
[fcasimportlimit] decimal(15,5),
[local_price_adjustment_export] decimal(10,2),
[locally_constrained_export] decimal(1,0),
[local_price_adjustment_import] decimal(10,2),
[locally_constrained_import] decimal(1,0)
) d
end
go
create or alter procedure InsertDispatchConstraint5
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'Constraint'
    and version = '5'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'Constraint'
    and version = '5'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'Constraint', 5);

insert into DispatchConstraint5(
file_log_id,
[settlementdate],
[runno],
[constraintid],
[dispatchinterval],
[intervention],
[rhs],
[marginalvalue],
[violationdegree],
[lastchanged],
[duid],
[genconid_effectivedate],
[genconid_versionno],
[lhs]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[constraintid],
d.[dispatchinterval],
d.[intervention],
d.[rhs],
d.[marginalvalue],
d.[violationdegree],
d.[lastchanged],
d.[duid],
d.[genconid_effectivedate],
d.[genconid_versionno],
d.[lhs]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[constraintid] varchar(20),
[dispatchinterval] decimal(22,0),
[intervention] decimal(2,0),
[rhs] decimal(15,5),
[marginalvalue] decimal(15,5),
[violationdegree] decimal(15,5),
[lastchanged] datetime2,
[duid] varchar(20),
[genconid_effectivedate] datetime2,
[genconid_versionno] decimal(22,0),
[lhs] decimal(15,5)
) d
end
go
create or alter procedure InsertPriceloadConstraintFcasOcd1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Priceload'
    and sub_type = 'ConstraintFcasOcd'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Priceload'
    and sub_type = 'ConstraintFcasOcd'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Priceload', 'ConstraintFcasOcd', 1);

insert into PriceloadConstraintFcasOcd1(
file_log_id,
[settlementdate],
[runno],
[intervention],
[constraintid],
[versionno],
[lastchanged],
[rhs],
[marginalvalue],
[violationdegree]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[intervention],
d.[constraintid],
d.[versionno],
d.[lastchanged],
d.[rhs],
d.[marginalvalue],
d.[violationdegree]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[intervention] decimal(2,0),
[constraintid] varchar(20),
[versionno] decimal(3,0),
[lastchanged] datetime2,
[rhs] decimal(15,5),
[marginalvalue] decimal(15,5),
[violationdegree] decimal(15,5)
) d
end
go
create or alter procedure InsertPriceloadConstraintrelaxation1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Priceload'
    and sub_type = 'Constraintrelaxation'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Priceload'
    and sub_type = 'Constraintrelaxation'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Priceload', 'Constraintrelaxation', 1);

insert into PriceloadConstraintrelaxation1(
file_log_id,
[settlementdate],
[runno],
[constraintid],
[rhs],
[lastchanged],
[versionno]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[constraintid],
d.[rhs],
d.[lastchanged],
d.[versionno]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[constraintid] varchar(20),
[rhs] decimal(16,6),
[lastchanged] datetime2,
[versionno] decimal(3,0)
) d
end
go
create or alter procedure InsertDispatchUnitScada1
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'UnitScada'
    and version = '1'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'UnitScada'
    and version = '1'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'UnitScada', 1);

insert into DispatchUnitScada1(
file_log_id,
[settlementdate],
[duid],
[scadavalue]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[duid],
d.[scadavalue]
from openjson(@data) with (
[settlementdate] datetime2,
[duid] varchar(20),
[scadavalue] decimal(16,6)
) d
end
go
create or alter procedure InsertDispatchCaseSolution2
    @file_name varchar(255),
    @data nvarchar(max)
as begin
declare @header table (id bigint not null);
if exists (
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'CaseSolution'
    and version = '2'
    )
    insert into @header (id)
    select id from FileLog 
    where file_name = @file_name
    and data_set = 'Dispatch'
    and sub_type = 'CaseSolution'
    and version = '2'
else
    insert into FileLog(file_name, data_set, sub_type, version)
    output inserted.id into @header
    values (@file_name, 'Dispatch', 'CaseSolution', 2);

insert into DispatchCaseSolution2(
file_log_id,
[settlementdate],
[runno],
[intervention],
[casesubtype],
[solutionstatus],
[spdversion],
[nonphysicallosses],
[totalobjective],
[totalareagenviolation],
[totalinterconnectorviolation],
[totalgenericviolation],
[totalramprateviolation],
[totalunitmwcapacityviolation],
[total5minviolation],
[totalregviolation],
[total6secviolation],
[total60secviolation],
[totalasprofileviolation],
[totalfaststartviolation],
[totalenergyofferviolation],
[lastchanged],
[switchruninitialstatus],
[switchrunbeststatus],
[switchrunbeststatus_int]
)
select 
(select h.id from @header h),
d.[settlementdate],
d.[runno],
d.[intervention],
d.[casesubtype],
d.[solutionstatus],
d.[spdversion],
d.[nonphysicallosses],
d.[totalobjective],
d.[totalareagenviolation],
d.[totalinterconnectorviolation],
d.[totalgenericviolation],
d.[totalramprateviolation],
d.[totalunitmwcapacityviolation],
d.[total5minviolation],
d.[totalregviolation],
d.[total6secviolation],
d.[total60secviolation],
d.[totalasprofileviolation],
d.[totalfaststartviolation],
d.[totalenergyofferviolation],
d.[lastchanged],
d.[switchruninitialstatus],
d.[switchrunbeststatus],
d.[switchrunbeststatus_int]
from openjson(@data) with (
[settlementdate] datetime2,
[runno] decimal(3,0),
[intervention] decimal(2,0),
[casesubtype] varchar(3),
[solutionstatus] decimal(2,0),
[spdversion] varchar(20),
[nonphysicallosses] decimal(1,0),
[totalobjective] decimal(27,10),
[totalareagenviolation] decimal(15,5),
[totalinterconnectorviolation] decimal(15,5),
[totalgenericviolation] decimal(15,5),
[totalramprateviolation] decimal(15,5),
[totalunitmwcapacityviolation] decimal(15,5),
[total5minviolation] decimal(15,5),
[totalregviolation] decimal(15,5),
[total6secviolation] decimal(15,5),
[total60secviolation] decimal(15,5),
[totalasprofileviolation] decimal(15,5),
[totalfaststartviolation] decimal(15,5),
[totalenergyofferviolation] decimal(15,5),
[lastchanged] datetime2,
[switchruninitialstatus] decimal(1,0),
[switchrunbeststatus] decimal(1,0),
[switchrunbeststatus_int] decimal(1,0)
) d
end
go